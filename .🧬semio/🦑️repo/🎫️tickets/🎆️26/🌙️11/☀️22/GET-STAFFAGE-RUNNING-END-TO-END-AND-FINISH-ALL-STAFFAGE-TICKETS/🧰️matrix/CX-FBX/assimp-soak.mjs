import { createRequire } from "node:module";
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
const factory = createRequire("/Users/ueli/Documents/semio/package.json")("assimpjs");
const dir = process.argv[2];
const files = readdirSync(dir, { recursive: true }).filter((f) => f.endsWith(".fbx")).map((f) => new Uint8Array(readFileSync(join(dir, f))));
const m = await factory();
let n = 0;
for (let round = 0; round < 40; round++) for (const bytes of files) {
  let msg;
  try { const l = new m.FileList(); l.AddFile("model.fbx", bytes); const r = m.ConvertFileList(l, "assjson"); msg = r.IsSuccess() ? "ok" : r.GetErrorCode(); r.delete(); l.delete(); } catch (e) { msg = "THROW " + (e?.message ?? e); }
  n++;
  if (msg !== "ok") { console.log("first failure at conversion", n, "round", round, msg, "wasm heap MB", (m.HEAP8?.length ?? 0) / 1048576); process.exit(0); }
}
console.log("no failure in", n);
