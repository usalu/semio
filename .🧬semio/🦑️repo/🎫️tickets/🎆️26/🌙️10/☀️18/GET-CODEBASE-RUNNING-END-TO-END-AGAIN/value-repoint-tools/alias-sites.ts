import { readFileSync } from "node:fs";
import { lex, lineOf } from "./lex.ts";
import { ROOT } from "./resolve-owner.ts";
const G = `${import.meta.dir}/../🗑️generated/value-repoint`;
const moved = JSON.parse(readFileSync(`${import.meta.dir}/moved.json`, "utf8"));
const names = new Set([...Object.keys(moved.modules), ...Object.keys(moved.items), ...Object.keys(moved.functions)]);
const files = readFileSync(`${G}/rs.nul`, "utf8").split("\0").filter(Boolean).filter((f) => !f.includes("/.🧬semio/") && !f.includes("🗑️generated"));
for (const rel of files) {
  let src: string;
  try { src = readFileSync(`${ROOT}/${rel}`, "utf8"); } catch { continue; }
  if (!/\b(__)?value\s*::/.test(src)) continue;
  const toks = lex(src).filter((t) => t.k !== "w" && t.k !== "c");
  for (let i = 0; i < toks.length; i++) {
    const t = toks[i];
    if (t.k !== "id" || (t.t !== "value" && t.t !== "__value") || toks[i + 1]?.t !== "::") continue;
    if (toks[i - 1]?.t === "#" || toks[i - 1]?.t === ".") continue;
    let j = i + 2, hit = false;
    if (toks[j]?.t === "{") { let d = 0; for (; j < toks.length; j++) { if (toks[j].t === "{") d++; else if (toks[j].t === "}" && --d === 0) break; else if (toks[j].k === "id" && names.has(toks[j].t) && (toks[j - 1].t === "{" || toks[j - 1].t === ",")) hit = true; } }
    else hit = toks[j]?.k === "id" && names.has(toks[j].t);
    if (hit) console.log(`${rel}:${lineOf(src, t.s)}: ${src.slice(Math.max(0, t.s - 40), t.s + 80).replace(/\n/g, " ")}`);
  }
}
