import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { createHash } from "node:crypto";
import { once } from "node:events";
import { closeSync, constants, copyFileSync, createReadStream, existsSync, mkdirSync, openSync, writeFileSync, writeSync } from "node:fs";
import { basename, join, resolve } from "node:path";
import { StringDecoder } from "node:string_decoder";

const [label, scope = "history"] = process.argv.slice(2);
assert.ok(/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(label ?? ""));
assert.ok(scope === "history" || scope === "all");
const generated = join(import.meta.dir, "🗑️generated");
const stem = join(generated, label);
const captures = join(stem, "tests");
const cwd = resolve("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx");
const profiles = scope === "history" ? ["base"] : ["base", "strict", "transitional", "strict-mutations"];
const packages = profiles.flatMap(profile => ["-p", `semio-s-artifact-stdio-xlsx-${profile}`]);
const selection = scope === "history" ? ["history_edits_end_to_end"] : ["--no-fail-fast"];
const args = [
  "cargo", "--store=shared", "--configuration=prototype", "--parallelism=1", "--",
  "test", ...packages, "--lib", "--message-format=json-render-diagnostics", ...selection,
  "--", "--test-threads=2", "--nocapture",
];

mkdirSync(captures, { recursive: true });
const log = openSync(stem + ".log", "wx");
const cargo = spawn("repo", args, {
  cwd,
  env: { ...process.env, SEMIO_REPO_SEAL: "live", SEMIO_LANGUAGE: "en", RUST_MIN_STACK: "67108864", RUST_BACKTRACE: "1", SEMIO_XLSX_ARCHIVE_PROGRESS: "1" },
  stdio: ["ignore", "pipe", log],
});
assert.ok(cargo.pid);
writeFileSync(stem + ".pid", String(cargo.pid) + "\n", { flag: "wx" });
console.log(`Native XLSX ${scope} replay PID ${cargo.pid}`);

const decoder = new StringDecoder("utf8");
const captured = new Map<string, { path: string; packageId: string; symbol: string }>();
let frame = "";
let captureNumber = 0;

function capture(line: string) {
  if (!line.startsWith('{"reason":"compiler-artifact"') || !line.includes("#semio-s-artifact-stdio-xlsx-")) return;
  const event = JSON.parse(line);
  if (!event.profile.test || typeof event.executable !== "string") return;
  const match = profiles.find(profile => event.package_id.includes(`#semio-s-artifact-stdio-xlsx-${profile}@`));
  if (!match || !event.target.kind.includes("lib")) return;
  const executable = join(captures, `${++captureNumber}-${basename(event.executable)}`);
  assert.ok(existsSync(event.executable), "Cargo's selected test executable is no longer present");
  copyFileSync(event.executable, executable, constants.COPYFILE_EXCL | constants.COPYFILE_FICLONE);
  captured.set(match, { path: executable, packageId: event.package_id, symbol: event.target.name });
  console.log(`[DEBUG] Captured managed XLSX ${match} test executable`);
}

let captureError: unknown;
cargo.stdout.on("data", (chunk: Buffer) => {
  writeSync(log, chunk);
  const lines = (frame + decoder.write(chunk)).split("\n");
  frame = lines.pop()!;
  try {
    for (const line of lines) capture(line);
  } catch (error) {
    captureError ??= error;
  }
});
const [code, signal] = await once(cargo, "close");
closeSync(log);
const files = [];
for (const [profile, row] of captured) {
  const hash = createHash("sha256");
  for await (const chunk of createReadStream(row.path)) hash.update(chunk);
  files.push({ profile, ...row, sha256: hash.digest("hex") });
}
writeFileSync(join(stem, "capture.json"), JSON.stringify({ scope, args, files }, null, 2) + "\n", { flag: "wx" });
writeFileSync(stem + ".exit", JSON.stringify({ code, signal }) + "\n", { flag: "wx" });
if (captureError) throw captureError;
if (code === 0) assert.equal(captured.size, profiles.length, "Each completed profile must have a captured test executable");
process.exitCode = code ?? 1;
