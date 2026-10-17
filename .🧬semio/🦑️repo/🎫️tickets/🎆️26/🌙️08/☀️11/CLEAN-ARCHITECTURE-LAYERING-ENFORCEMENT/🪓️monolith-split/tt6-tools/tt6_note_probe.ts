/** 🔎 Finds which committed note mutation fixtures the production parsers refuse. */
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
const S = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets";
const io = await import(S + "/✳️any/🚪️io/📝️text/📸️snapshot/🔣️json/🟦️.ts");
let ok = 0, bad = 0;
for (const subset of readdirSync(S)) {
  const root = join(S, subset, "🧪️tests/🧫️fixtures/🧬️mutations");
  let paths: string[] = [];
  try { paths = readdirSync(root, { recursive: true }).map(String); } catch { continue; }
  for (const p of paths) {
    const snap = p.endsWith("/📸️snapshot/⬅️before/🔣️.json") || p.endsWith("/📸️snapshot/➡️after/🔣️.json");
    const diff = p.endsWith("/🔺️diff/🔣️.json");
    if (!snap && !diff) continue;
    const input = JSON.parse(readFileSync(join(root, p), "utf8"));
    try { (snap ? io.parseNoteSnapshotJson : io.parseNoteDiffJson)(input); ok++; } catch (e) { bad++; console.log("REFUSED", subset, p, String(e).slice(0, 100)); }
  }
}
console.log({ ok, bad });
