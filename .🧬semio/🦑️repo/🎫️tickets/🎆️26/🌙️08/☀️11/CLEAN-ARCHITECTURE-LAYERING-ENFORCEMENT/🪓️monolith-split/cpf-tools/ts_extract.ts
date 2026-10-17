import { comments, diagnostics } from "./ts_lib.ts";
import { readFileSync, writeFileSync } from "node:fs";
const R = "/Users/ueli/Documents/semio/";
const G = process.env.G!;
const EX = /^(\/\/|\/\*)\s*(#?(end)?region\b|@ts-|@vitest|@jsx|eslint|prettier|biome|istanbul|c8 |v8 |nolint|#\s*sourceMappingURL|\/ *<reference|SAFETY\b)/i;
const paths = (readFileSync(process.argv[2], "utf8")).split("\n").filter(Boolean);
const out: any[] = [];
let files = 0, bad = 0;
for (const p of paths) {
  let text: string;
  try { text = readFileSync(R + p, "utf8"); } catch { continue; }
  if (!text.includes("//") && !text.includes("/*")) continue;
  files++;
  if (diagnostics(p, text)) { bad++; out.push({ path: p, parseError: true }); continue; }
  const cs = comments(p, text).filter(c => !c.doc && !c.jsx && (c.fn || c.de) && !EX.test(c.text.trim()));
  if (cs.length) out.push({ path: p, cs });
}
writeFileSync(G + "/cpf-ts-extract.json", JSON.stringify(out));
console.log("files", files, "withcands", out.filter(o => o.cs).length, "parseErr", bad);
