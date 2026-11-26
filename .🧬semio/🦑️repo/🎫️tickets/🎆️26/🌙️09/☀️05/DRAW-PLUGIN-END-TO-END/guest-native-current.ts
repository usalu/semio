/** 🖍️ Executes an existing Draw native lifecycle binary against one exact new publication. */
import { createHash } from "node:crypto";
import { copyFile, lstat, mkdir, mkdtemp, realpath, rm } from "node:fs/promises";
import { homedir } from "node:os";
import { join, resolve } from "node:path";
import { parseArgs } from "node:util";

const root = process.cwd();
const generated = join(import.meta.dir, "🗑️generated");
const framework = join(root, "🧰️framework");
const plugin = join(framework, "🛍️products/💻️os/🔨️modules/🔌️plugin");
const draw = "🌎️hub/🧩️compositions/🖍️draw";
const oldSha256 = "fcc8e11c2267be890c236bf25f06d9e378b87b4becc78bebb74d95a6bcfb10e2";
const testName = "quick::wasmtime_runtime_instance_open_settles_against_the_draw_component";
const MiB = 1 << 20;
const [action, ...args] = Bun.argv.slice(2);
const { values } = parseArgs({ args, options: {
  sha256: { type: "string" }, executable: { type: "string" }, model: { type: "string" },
  home: { type: "string" }, mode: { type: "string", default: "prototype" }, cold: { type: "boolean" },
}});
if (action !== "plan" && action !== "run" && action !== "execute") throw Error("Expected plan or run");
if (!values.sha256 || !/^[a-f0-9]{64}$/u.test(values.sha256) || values.sha256 === oldSha256) throw Error("The expected SHA-256 of the new Draw component is required; fcc8 is the previous publication");
if (action !== "plan" && !values.executable) throw Error("An existing dedicated Draw guest test executable is required");
const executable = values.executable ? resolve(values.executable) : undefined;
const home = await realpath(resolve(values.home ?? process.env.SEMIO_HOME ?? homedir()));
const { Blake3Hasher, blake3Hex } = await import(join(framework, "🔨️modules/🔏️hash/🟦️.ts"));
const { componentModelLayoutV1 } = await import(join(plugin, "📦️model/📋️contract/🟦️.ts"));
const { COMPONENT_RECEIPT_MAX_BYTES, parseComponentReceiptV1, componentReceiptBindsV1, componentProfileV1 } = await import(join(plugin, "🖨️describe/🏷️component-receipt/🧬️schema/🟦️.ts"));
const { CATALOG_DESCRIPTOR_MAX_BYTES } = await import(join(plugin, "📇️registry/✅️catalog-verification/🟦️.ts"));
const { verifySemanticDescriptorPairBytesV1 } = await import(join(plugin, "📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts"));
const { fileDigest, readArtifactBytesV1, readArtifactPrefixV1 } = await import(join(plugin, "🌐️browser-bundle/📦️distribution/📋️inventory/🟦️.ts"));
const { validateDrawGuestScenarios } = await import(join(root, draw, "🧪️tests/🌐️guest/🟦️.ts"));
const mode = componentProfileV1(values.mode);
const model = resolve(values.model ?? componentModelLayoutV1(join(root, ".cache/b/p/m"), { version: 1, mode, pluginId: "draw", cratePath: draw + "/🧩️component/📦️packages/🦀️rust" }).path);
const files = ["📦️component.wasm", "🔣️.json", "🛂️.descriptor.semio", "🧾️receipt.json"] as const;

async function inspect(directory: string) {
  const [component, descriptor, pack, receiptFile] = files.map(name => join(directory, name)) as [string, string, string, string];
  const receipt = parseComponentReceiptV1(new TextDecoder("utf-8", { fatal: true }).decode(await readArtifactBytesV1(receiptFile, COMPONENT_RECEIPT_MAX_BYTES)));
  if (receipt.wasmSha256 !== values.sha256) throw Error("The selected model receipt names a different Draw component");
  if ((await lstat(component)).size !== receipt.wasmBytes || !Buffer.from(await readArtifactPrefixV1(component, 8, receipt.wasmBytes)).equals(Buffer.from([0, 97, 115, 109, 13, 0, 1, 0]))) throw Error("The published file does not have the receipt's Wasm component format and length");
  const pair = verifySemanticDescriptorPairBytesV1(await readArtifactBytesV1(descriptor, CATALOG_DESCRIPTOR_MAX_BYTES), await readArtifactBytesV1(pack, CATALOG_DESCRIPTOR_MAX_BYTES));
  const current = await fileDigest(join(root, draw, "🧩️component/🛂️.descriptor.semio"));
  const wasm = new Blake3Hasher(), receiptHash = createHash("sha256");
  let scanned = 0;
  for await (const chunk of Bun.file(component).stream()) {
    scanned += chunk.byteLength;
    if (scanned > receipt.wasmBytes) throw Error("The Draw component changed length while it was being read");
    wasm.update(chunk);
    receiptHash.update(chunk);
  }
  if (scanned !== receipt.wasmBytes) throw Error("The Draw component is shorter than its receipt");
  const sha256 = receiptHash.digest("hex");
  if (pair.pluginId !== "draw" || pair.packageId !== "semio:draw" || pair.semanticSha256 !== current || !componentReceiptBindsV1(receipt, { profile: mode, pluginId: "draw", packageId: "semio:draw", wasmSha256: sha256, semanticDescriptorSha256: pair.semanticSha256 })) throw Error("Draw component, published descriptor, receipt, and current semantic source do not agree");
  await validateDrawGuestScenarios(root, descriptor);
  const packageHash = Buffer.from(wasm.digest()).toString("hex");
  const cache = [true, false].map(pooling => {
    const namespace = blake3Hex(new TextEncoder().encode("wasmtime=47.0.3;component_model=1;fuel=1;epoch=1;pooling=" + pooling + ";instances=4096;max_memory=536870912;keep_resident=2097152"));
    const path = join(home, ".semio", "cache", "wasmtime", namespace, packageHash + ".cwasm");
    if (path.length > 256) throw Error("The Wasmtime cache path exceeds 256 characters");
    return path;
  });
  return { component, descriptor, receipt, sha256, packageHash, cache };
}

async function cacheEntries(paths: string[]) {
  const rows = await Promise.all(paths.map(async path => {
    const info = await lstat(path).catch(error => {
      if ((error as NodeJS.ErrnoException).code === "ENOENT") return undefined;
      throw error;
    });
    if (info && (!info.isFile() || info.size === 0)) throw Error("The Wasmtime cache entry is not a regular nonempty file: " + path);
    return info ? { path, bytes: info.size } : undefined;
  }));
  return rows.filter(row => row !== undefined);
}

const selected = await inspect(model);
const existing = await cacheEntries(selected.cache);
const warm = existing.length > 0;
const coldEstimateMiB = Math.ceil(256 + 48 * selected.receipt.wasmBytes / MiB);
const measuredColdMiB = Math.ceil(5.1 * 1024);
const reservedMiB = warm ? 1536 : Math.max(coldEstimateMiB, Math.ceil(measuredColdMiB * 1.25)) + 512;
console.log("[DEBUG] " + JSON.stringify({ action, model, componentSha256: selected.sha256, semanticDescriptorSha256: selected.receipt.semanticDescriptorSha256, sourceTreeDigest: selected.receipt.sourceTreeDigest, packageHash: selected.packageHash, componentBytes: selected.receipt.wasmBytes, cache: selected.cache, existing, warm, coldEstimateMiB, measuredColdMiB, reservedMiB }));
if (action === "plan") process.exit(0);
if (!warm && !values.cold) throw Error("This exact Draw publication has no Wasmtime cache; rerun explicitly with --cold to request the reported compiler reservation");
if (!(await lstat(executable!)).isFile()) throw Error("The Draw native test executable must be a regular existing file");

if (action === "run") {
  const command = ["repo", "capacity", "run", "--memory=" + reservedMiB, "--", "repo", "tools", "exec", "--light", "bun", "--", import.meta.path, "execute", "--sha256", selected.sha256, "--model", model, "--mode", mode, "--home", home, "--executable", executable!, ...(values.cold ? ["--cold"] : [])];
  const child = Bun.spawn(command, { stdout: "inherit", stderr: "inherit" });
  const interrupt = () => child.kill("SIGINT"), terminate = () => child.kill("SIGTERM");
  process.once("SIGINT", interrupt);
  process.once("SIGTERM", terminate);
  process.exitCode = await child.exited.finally(() => {
    process.off("SIGINT", interrupt);
    process.off("SIGTERM", terminate);
  });
} else {
  const { parentCompilerMemoryAllowanceBytesV1 } = await import(join(framework, "🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts"));
  const actualGrant = parentCompilerMemoryAllowanceBytesV1(process.env);
  if (actualGrant === undefined || actualGrant < reservedMiB * MiB) {
    throw Error("The Draw executable requires an active " + reservedMiB + " MiB reservation. Actual grant: " + (actualGrant === undefined ? "missing" : Math.floor(actualGrant / MiB) + " MiB"));
  }
  await mkdir(generated, { recursive: true });
  const pinned = await mkdtemp(join(generated, "guest-"));
  try {
    await Promise.all(files.map(name => copyFile(join(model, name), join(pinned, name))));
    const current = await inspect(pinned);
    const binaryHash = createHash("sha256");
    for await (const chunk of Bun.file(executable!).stream()) binaryHash.update(chunk);
    console.log("[DEBUG] Existing guest executable SHA-256 " + binaryHash.digest("hex") + " at " + executable);
    const env = { ...process.env, SEMIO_HOME: home, SEMIO_DRAW_GUEST_COMPONENT: current.component, SEMIO_DRAW_GUEST_DESCRIPTOR: current.descriptor, SEMIO_DRAW_GUEST_CACHE_ONLY: warm ? "1" : undefined, RUST_MIN_STACK: process.env.RUST_MIN_STACK ?? "134217728" };
    const listing = Bun.spawn([executable!, "--list", "--exact", testName], { stdout: "pipe", stderr: "inherit", env });
    const found = await new Response(listing.stdout).text();
    if (await listing.exited !== 0 || !found.split("\n").some(line => line.trim() === testName + ": test")) throw Error("The provided executable does not contain the exact dedicated Draw lifecycle test");
    const started = performance.now();
    const child = Bun.spawn([executable!, "--exact", testName, "--test-threads=1", "--nocapture"], { env, stdout: "pipe", stderr: "pipe" });
    const interrupt = () => child.kill("SIGINT"), terminate = () => child.kill("SIGTERM");
    process.once("SIGINT", interrupt);
    process.once("SIGTERM", terminate);
    const expected = ["test result: ok. 1 passed", "Native Draw editor opened, closed and retired", "Native Draw viewer opened, closed and retired"];
    const observed = new Set<string>();
    const observe = async (stream: ReadableStream<Uint8Array>, write: (chunk: string) => unknown) => {
      let tail = "";
      for await (const chunk of stream.pipeThrough(new TextDecoderStream())) {
        write(chunk);
        tail = tail.slice(-256) + chunk;
        for (const marker of expected) if (tail.includes(marker)) observed.add(marker);
      }
    };
    const [exit] = await Promise.all([child.exited, observe(child.stdout, chunk => process.stdout.write(chunk)), observe(child.stderr, chunk => process.stderr.write(chunk))]).finally(() => {
      process.off("SIGINT", interrupt);
      process.off("SIGTERM", terminate);
    });
    const after = await cacheEntries(current.cache);
    console.log("[DEBUG] " + JSON.stringify({ exit, activeGrantMiB: actualGrant / MiB, elapsedMs: Math.round(performance.now() - started), maximumResidentBytes: child.resourceUsage()?.maxRSS ?? null, cache: after, observed: [...observed] }));
    if (exit || observed.size !== expected.length || after.length === 0) throw Error("The exact editor and viewer lifecycle did not pass with its component cache");
  } finally {
    await rm(pinned, { recursive: true, force: true });
  }
}
