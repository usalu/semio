import { spawnSync } from "node:child_process";
import { readFileSync, writeFileSync, readdirSync, statSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
const spec = process.argv[2];
const apply = process.argv[3] === "apply";
const probe = 'import(process.argv[1]).then(m=>console.log("OK",Object.keys(m).length),e=>console.log("ERR "+e.message))';
const importRe = /((?:\bfrom\s*|\bimport\s*)(["']))([^"'\n]+\.json)\2(?!\s*with)/g;
const changed = [];
for (let round = 0; round < 60; round++) {
  const out = spawnSync("node", ["--experimental-strip-types", "-e", probe, spec], { encoding: "utf8", cwd: process.cwd() }).stdout.trim();
  if (out.startsWith("OK")) { console.log("[DEBUG] loads", out, "fixed", changed.length); break; }
  const m = /Module "(file:[^"]+)" needs an import attribute/.exec(out);
  if (!m) { console.log("[DEBUG] other error:", out.replace(/%[0-9A-F]{2}/g, "").slice(0, 500)); break; }
  const json = fileURLToPath(m[1]);
  const cands = new Set();
  const scan = (d, depth) => { let names; try { names = readdirSync(d); } catch { return; } for (const n of names) { const p = join(d, n); let st; try { st = statSync(p); } catch { continue; } if (st.isFile() && /\.[cm]?ts$/.test(n)) { const s = readFileSync(p, "utf8"); for (const mm of s.matchAll(importRe)) if (resolve(dirname(p), mm[3]) === json) cands.add(p); } else if (st.isDirectory() && depth > 0 && n !== "node_modules" && !n.startsWith(".")) scan(p, depth - 1); } };
  let up = dirname(json); for (let i = 0; i < 4; i++) { scan(up, 3); up = dirname(up); }
  console.log("[DEBUG] round", round, json.split("/").slice(-5).join("/"), "importers:", [...cands].map((c) => c.split("/").slice(-4).join("/")));
  if (!apply || cands.size === 0) break;
  for (const c of cands) { let s = readFileSync(c, "utf8"); s = s.replace(importRe, (all, a, q, spec2) => resolve(dirname(c), spec2) === json ? `${a}${spec2}${q} with { type: "json" }` : all); writeFileSync(c, s); changed.push(c); }
}
console.log("[DEBUG] changed files:\n" + changed.join("\n"));
