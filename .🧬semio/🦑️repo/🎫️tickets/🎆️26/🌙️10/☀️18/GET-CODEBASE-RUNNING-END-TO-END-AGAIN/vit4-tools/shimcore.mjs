import { readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
const ROOT = "/Users/ueli/Documents/semio";
const map = new Map();
for (const w of JSON.parse(readFileSync(join(ROOT, "package.json"), "utf8")).workspaces) {
  try {
    const j = JSON.parse(readFileSync(join(ROOT, w, "package.json"), "utf8"));
    if (!existsSync(join(ROOT, "node_modules", j.name))) map.set(j.name, { dir: join(ROOT, w), exports: j.exports, main: j.main });
  } catch {}
}
function target(t, conds) {
  if (typeof t === "string") return t;
  if (Array.isArray(t)) { for (const x of t) { const r = target(x, conds); if (r) return r; } return null; }
  if (t && typeof t === "object") { for (const k of Object.keys(t)) if (conds.includes(k) || k === "default") { const r = target(t[k], conds); if (r) return r; } }
  return null;
}
export function shimResolve(spec, conds = ["import", "node", "default"]) {
  const m = /^(@[^/]+\/[^/]+)(\/.*)?$/.exec(spec);
  if (!m) return null;
  const pkg = map.get(m[1]);
  if (!pkg) return null;
  const sub = m[2] ? "." + m[2] : ".";
  const ex = pkg.exports;
  let t = null;
  if (ex && typeof ex === "object" && !Array.isArray(ex) && Object.keys(ex).some((k) => k.startsWith("."))) {
    if (ex[sub] !== undefined) t = target(ex[sub], conds);
    else for (const k of Object.keys(ex)) { const i = k.indexOf("*"); if (i >= 0 && sub.startsWith(k.slice(0, i)) && sub.endsWith(k.slice(i + 1))) { const r = target(ex[k], conds); if (r) { t = r.replace("*", sub.slice(i, sub.length - (k.length - i - 1))); break; } } }
  } else if (sub === ".") t = target(ex, conds) ?? pkg.main ?? "./index.js";
  return t ? join(pkg.dir, t) : null;
}
export const shimNames = () => [...map.keys()];
export const shimUrl = (spec) => { const p = shimResolve(spec); return p ? pathToFileURL(p).href : null; };
