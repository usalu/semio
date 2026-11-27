import { createRequire } from "node:module";
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const factory = require("assimpjs");
const mode = process.argv[3] ?? "fresh";
const files = (d) => readdirSync(d, { recursive: true }).filter((f) => f.endsWith(".fbx")).map((f) => join(d, f));
let shared;
for (const f of files(process.argv[2])) {
  const bytes = new Uint8Array(readFileSync(f));
  const m = mode === "fresh" ? await factory() : (shared ??= await factory());
  let msg;
  try {
    const l = new m.FileList(); l.AddFile("model.fbx", bytes);
    const r = m.ConvertFileList(l, "assjson");
    msg = r.IsSuccess() ? "ok" : r.GetErrorCode();
    r.delete(); l.delete();
  } catch (e) { msg = "THROW " + (e?.message ?? e); }
  console.log((bytes.length / 1048576).toFixed(1) + "MB", msg, f.split("/").slice(-1)[0]);
}
