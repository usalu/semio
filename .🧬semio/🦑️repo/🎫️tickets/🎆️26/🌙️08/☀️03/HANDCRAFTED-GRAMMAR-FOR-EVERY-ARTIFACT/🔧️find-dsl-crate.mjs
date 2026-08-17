import { readFileSync, readdirSync, statSync, existsSync } from "fs";
import { join } from "path";
function walk(dir, depth = 0, out = []) {
  if (depth > 8) return out;
  let entries;
  try { entries = readdirSync(dir); } catch { return out; }
  for (const e of entries) {
    if (e === "target" || e === "node_modules" || e === ".git") continue;
    const p = join(dir, e);
    let st;
    try { st = statSync(p); } catch { continue; }
    if (st.isFile() && e === "Cargo.toml") {
      const t = readFileSync(p, "utf8");
      if (t.includes("semio-framework-os-kernel-dsl") || t.includes('name = "dsl"')) out.push({ p, snippet: t.slice(0, 400) });
    } else if (st.isDirectory()) walk(p, depth + 1, out);
  }
  return out;
}
const hits = walk(".");
console.log("hits", hits.length);
for (const h of hits.slice(0, 30)) {
  console.log("---", h.p);
  console.log(h.snippet);
}
