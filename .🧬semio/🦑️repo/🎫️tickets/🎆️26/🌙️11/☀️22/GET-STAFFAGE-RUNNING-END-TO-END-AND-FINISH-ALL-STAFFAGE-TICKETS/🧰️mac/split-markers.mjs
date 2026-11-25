// 🧊️ The assertions of `model-split` ("builds the viewer and the encoders into lazy chunks and keeps them out of the entry") read from an existing release build instead of building one again. Usage: node split-markers.mjs <dist directory>
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
const out = process.argv[2];
const html = readFileSync(join(out, "index.html"), "utf8");
const entry = /src="\/assets\/([^"]+\.js)"/u.exec(html)[1];
const chunks = Object.fromEntries(readdirSync(join(out, "assets")).filter((name) => name.endsWith(".js") && !name.startsWith("🔣️")).map((name) => [name, readFileSync(join(out, "assets", name), "utf8")]));
let failed = 0;
for (const marker of ["#version 300 es", "KHR_materials_variants", "FBXHeaderExtension", "semio-stage", "staffage volume/v2", "Removing hidden surfaces", "semio.model/v1"]) {
  const where = Object.entries(chunks).filter(([, text]) => text.includes(marker)).map(([name]) => name);
  const ok = where.length > 0 && !where.includes(entry);
  if (!ok) failed++;
  console.log(ok ? "ok  " : "FAIL", JSON.stringify(marker), "in", where.length, "chunk(s), entry", entry, where.includes(entry) ? "CONTAINS IT" : "does not");
}
process.exitCode = failed === 0 ? 0 : 1;
