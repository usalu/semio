/** 🛎️ Verifies the newly staged Note guest and runs the browser scenarios against its served bytes. */
import assert from "node:assert/strict";
import { join } from "node:path";

const baseline = process.env.SEMIO_NOTE_WATCH_FROM;
assert.ok(baseline, "Provide the previous Note component receipt");
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6080/?plugin=note&role=editor";
const core = "semio_hub_note_component.core.wasm";
const leaf = ".cache/b/p/m/browser/prototype/modules/🗒️note";
const coreUrl = new URL("/🔌️plugin-modules/🗒️note/" + core, url);
const receipt = Bun.file(join(leaf, "🧾️receipt.json"));
const expires = Date.now() + Number(process.env.SEMIO_NOTE_WATCH_SECONDS ?? "3600") * 1000;
const output = join(import.meta.dir, "🗑️generated");
const probe = join(import.meta.dir, "🐍️note-acceptance.mjs");
const math = join(output, "note-document-archive-test.semio-archive");
const mixed = join(output, "note-picker-mixed-export-without-camera", "note-acceptance.semio-archive");
const prefix = process.env.SEMIO_NOTE_WATCH_PREFIX ?? "note-final57";

const hash = async (stream) => {
  const digest = new Bun.CryptoHasher("sha256");
  let bytes = 0;
  for await (const chunk of stream) {
    digest.update(chunk);
    bytes += chunk.byteLength;
  }
  return { bytes, sha: digest.digest("hex") };
};

let observed;
while (Date.now() < expires) {
  const current = await receipt.json().catch(() => undefined);
  if (current?.wasmSha256 && current.wasmSha256 !== baseline) {
    const local = Bun.file(join(leaf, core));
    const head = await fetch(coreUrl, { method: "HEAD", signal: AbortSignal.timeout(3000) }).catch(() => undefined);
    if (head?.ok && Number(head.headers.get("content-length")) === local.size && local.size > 0) {
      const served = await fetch(coreUrl, { headers: { "Accept-Encoding": "identity" }, signal: AbortSignal.timeout(60000) }).catch(() => undefined);
      if (served?.ok && served.body) {
        const [expected, actual] = await Promise.all([hash(local.stream()), hash(served.body)]);
        if (expected.bytes === actual.bytes && expected.sha === actual.sha &&
            (await receipt.json().catch(() => undefined))?.wasmSha256 === current.wasmSha256) {
          observed = current;
          console.log("[DEBUG] Note staged and served provenance", current.wasmSha256, current.sourceTreeDigest, actual.bytes, actual.sha);
          break;
        }
      }
    }
  }
  await Bun.sleep(3500);
}
assert.ok(observed, "A fresh Note guest with matching served bytes did not become available");

const base = Object.fromEntries(Object.entries(process.env).filter(([key]) => !key.startsWith("SEMIO_PROBE_")));
const scenarios = [
  [["history", { SEMIO_PROBE_HISTORY_ROUND_TRIP: "1", SEMIO_PROBE_ARCHIVE_TRACE: "1" }],
   ["editor", { SEMIO_PROBE_EXTENDED: "1" }]],
  [["mixed", { SEMIO_PROBE_IMPORT: mixed, SEMIO_PROBE_IMPORT_MIXED: "1", SEMIO_PROBE_ARCHIVE_TRACE: "1" }],
   ["cancel", { SEMIO_PROBE_IMPORT: mixed, SEMIO_PROBE_IMPORT_CANCEL: "1" }]],
  [["viewer", { SEMIO_PROBE_VIEWER_ONLY: "1" }],
   ["math", { SEMIO_PROBE_IMPORT: math, SEMIO_PROBE_ARCHIVE_TRACE: "1" }]],
];
const failures = await Promise.all(scenarios.map(async (group) => {
  const rejected = [];
  for (const [name, variables] of group) {
    const run = prefix + "-" + name;
    const log = Bun.file(join(output, run + ".log"));
    const child = Bun.spawn(["repo", "tools", "exec", "--light", "bun", "--", probe], {
      env: { ...base, ...variables, SEMIO_PROBE_OUT: run, SEMIO_PROBE_URL: url, SEMIO_PROBE_SETTLE: "150", SEMIO_PROBE_IMPORT_REPLACE_EMPTY_FIXTURE: "1" },
      stdout: log,
      stderr: log,
    });
    const code = await child.exited;
    console.log("[DEBUG] Note browser scenario", run, code);
    if (code !== 0) rejected.push(name);
  }
  return rejected;
}));
assert.deepEqual(failures.flat(), [], "One or more Note browser scenarios failed");
