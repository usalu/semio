/** 🔺️ Triangles per solid of the scenes of an asset document, largest first, and the geometry findings: tri.ts <asset document | id> <scene prefix> [count] */
import { readFileSync } from "node:fs";
import { assetFile, mod } from "../../🧰️linux/s4/lib.ts";
const [target, prefix, count = "14"] = process.argv.slice(2) as [string, string, string?];
const asset = JSON.parse(readFileSync(target.endsWith(".json") ? target : assetFile(target), "utf8")).asset;
const { auditGeometry } = await mod("🧪️tests/🧊️model-catalog/🔍️oracles/🟦️.ts");
const g = await auditGeometry(asset);
console.log(JSON.stringify(g.facts.scenes.map((s: any) => [s.scene, s.solids, s.glyphs, s.triangles])));
for (const s of [...g.solids].filter((s: any) => s.id.startsWith(prefix)).sort((a: any, b: any) => b.triangles - a.triangles).slice(0, Number(count))) console.log(s.id, s.kind, s.triangles, s.glyph ? "glyph" : "");
for (const f of g.findings) console.log("FINDING", f.check, f.item, f.message);
