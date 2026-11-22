import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, readFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";

const root = process.cwd();
const [stem] = process.argv.slice(2);
assert.ok(stem, "Pass the Cargo log stem");
const capture = resolve(stem);
const wasm = join(capture, "stdio-office.wasm");
const receipt = join(capture, "stdio-office-cargo-artifact.json");
const web = join(capture, "web");
const descriptor = join(capture, "descriptor");
const tool = join(root, ".cache/b/p/m/browser/prototype/tools/descriptor-emitter", process.platform,
  process.platform === "win32" ? "semio-plugin-describe.exe" : "semio-plugin-describe");
const scripts = import.meta.dir;

async function run(args: string[]) {
  const command = ["repo", ...args];
  const proc = Bun.spawn(command, { cwd: root, stdout: "inherit", stderr: "inherit" });
  assert.equal(await proc.exited, 0, command.join(" "));
}

while (!existsSync(receipt)) {
  if (existsSync(stem + ".exit")) {
    const status = readFileSync(stem + ".exit", "utf8").trim();
    if (status !== "0") throw Error("Fresh Office compiler failed before a component was captured");
    if (Date.now() - Bun.file(stem + ".exit").lastModified > 60_000) throw Error("The successful compiler log yielded no captured Office component");
  }
  await Bun.sleep(1000);
}
const captured = JSON.parse(readFileSync(receipt, "utf8"));
assert.ok(captured.cargo.package_id.includes("#semio-hub-stdio-office-component@"));
assert.notEqual(captured.sha256, "2ae0845ec0987eaab2d1d584fed4fb1f7745d8d5de819f5db88145b90f163a7b");
const productRoot = dirname(tool);
const product = JSON.parse(readFileSync(join(productRoot, ".semio-artifact.json"), "utf8"));
const index = JSON.parse(readFileSync(join(productRoot, "compilation", "index.json"), "utf8"));
const receiptBytes = readFileSync(join(productRoot, "compilation", "receipt.json"));
const sha256 = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex");
assert.ok(product.files.includes(basename(tool)));
assert.equal(index.receipt.sha256, sha256(receiptBytes));
const { decodeCargoProvenanceV1 } = await import(join(root, "🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts"));
const build = decodeCargoProvenanceV1(receiptBytes);
assert.equal(build.status, 0);
assert.equal(build.cancelled, false);
const toolDigest = sha256(readFileSync(tool));
assert.equal(build.units.filter(unit => unit.message.package_id.includes("#semio-plugin-describe@")
  && unit.message.target.name === "semio-plugin-describe"
  && unit.artifacts.some(artifact => artifact.sha256 === toolDigest || artifact.stagedSha256 === toolDigest)).length, 1);
console.log(`[DEBUG] Fresh Office ${captured.sha256} uses the repository-published descriptor emitter ${toolDigest}`);
mkdirSync(descriptor);

const bundle = await Promise.allSettled([
  run(["capacity", "run", "--memory=2048", "--", tool, "describe", wasm, "--out", descriptor]),
  run(["capacity", "run", "--memory=1536", "--", "repo", "tools", "exec", "--light", "bun", "--",
    join(scripts, "prove-office-browser-component.ts"), "transpile", capture, web]),
]);
const faults = bundle.flatMap(result => result.status === "rejected" ? [result.reason] : []);
if (faults.length) throw new AggregateError(faults, "Fresh Office descriptor generation or browser transpilation failed");
assert.equal(sha256(readFileSync(tool)), toolDigest, "Descriptor emitter changed during verification");

await run(["tools", "exec", "--light", "bun", "--", join(scripts, "verify-office-idml.ts"), descriptor, wasm, "verify"]);
await run(["capacity", "run", "--memory=1024", "--", "repo", "tools", "exec", "--light", "bun", "--",
  join(scripts, "prove-office-browser-codec.ts"), "--fresh", web, descriptor]);
