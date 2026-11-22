/** 🧪 Runs the mounted Open, edit, and Save gate only after the new signed Browser session publishes. */
import assert from "node:assert/strict";
import { closeSync, openSync } from "node:fs";
import { join } from "node:path";

const [candidate, previousSha, run, ...cases] = process.argv.slice(2);
assert(/^[a-z0-9-]+$/u.test(candidate) && /^[a-z0-9-]+$/u.test(run) && /^[a-f0-9]{64}$/u.test(previousSha));
const generated = join(import.meta.dir, "🗑️generated");
const pipeline = join(generated, "pdf-browser-" + candidate + "-pipeline.log");
const receiptPath = ".cache/b/p/m/browser/prototype/modules/📘️stdio-pdf/🧾️receipt.json";
const deadline = Date.now() + 18 * 60 * 1000;
while (true) {
  assert(Date.now() < deadline, "Browser publication did not finish within the observed staging window");
  const output = await Bun.file(pipeline).text();
  if (output.includes("[DEBUG] assets finished with 0")) break;
  const failures = [...output.matchAll(/\[DEBUG\] (?:components-pdf|session|assets) finished with (\d+)/gu)];
  assert(failures.every(match => ["0", "75"].includes(match[1])), "Browser publication failed");
  await Bun.sleep(3000);
}
const receipt = await Bun.file(receiptPath).json();
assert.equal(receipt.pluginId, "stdio-pdf");
assert(/^[a-f0-9]{64}$/u.test(receipt.wasmSha256) && receipt.wasmSha256 !== previousSha, "Publication retained the prior PDF core");
console.log("[DEBUG] Starting mounted Browser acceptance", receipt.wasmSha256, receipt.wasmBytes);
const log = openSync(join(generated, "pdf-browser-" + run + "-mounted-suite.log"), "wx");
const child = Bun.spawn(["repo","tools","exec","--light","bun","--",join(import.meta.dir,"📜️pdf-browser-mounted-suite.ts"),run,...cases], {
  env: {...process.env, SEMIO_PDF_TRACE_OPEN:"1"},
  stdout: log, stderr: log,
});
closeSync(log);
const interrupt = () => child.kill("SIGINT");
process.once("SIGINT", interrupt);
process.once("SIGTERM", interrupt);
process.exitCode = await child.exited;
console.log("[DEBUG] Mounted Browser acceptance completed", process.exitCode);
