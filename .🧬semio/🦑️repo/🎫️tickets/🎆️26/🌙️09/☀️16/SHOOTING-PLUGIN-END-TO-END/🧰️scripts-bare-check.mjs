import {readdirSync, readFileSync, statSync, existsSync} from "node:fs";
import {join} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const pk = new Map();
function walk(d) {
  let ents; try { ents = readdirSync(d); } catch { return; }
  for (const e of ents) {
    if (e === "node_modules" || e === ".git" || e === "target" || e === "dist") continue;
    const p = join(d, e);
    let st; try { st = statSync(p); } catch { continue; }
    if (st.isDirectory()) walk(p);
    else if (e === "package.json") { try { const j = JSON.parse(readFileSync(p, "utf8")); if (j.name && j.name.startsWith("@semio-tech/")) pk.set(j.name, {dir: d, exports: j.exports, main: j.main}); } catch {} }
  }
}
for (const top of ["🧰️framework", "✏️s"]) walk(join(ROOT, top));
const specs = JSON.parse(readFileSync("🗑️generated/imports.json", "utf8")).findings.filter(f => f.kind === "bare-package");
const uniq = [...new Set(specs.map(s => s.spec))];
const out = [];
for (const spec of uniq) {
  const parts = spec.split("/"); const name = spec.startsWith("@") ? parts.slice(0, 2).join("/") : parts[0];
  const sub = "./" + parts.slice(name.split("/").length).join("/");
  const p = pk.get(name);
  if (!p) { out.push({spec, status: "NO-PACKAGE", name}); continue; }
  const ex = p.exports;
  if (!ex) { out.push({spec, status: "no-exports-map", dir: p.dir}); continue; }
  const key = sub === "./" ? "." : sub;
  if (!(key in ex)) { out.push({spec, status: "SUBPATH-NOT-EXPORTED", key, pkg: name}); continue; }
  let target = ex[key]; if (typeof target === "object") target = target.default ?? target.import ?? Object.values(target)[0];
  const file = join(p.dir, target);
  out.push({spec, status: existsSync(file) ? "ok" : "EXPORT-TARGET-MISSING", file: file.slice(ROOT.length + 1)});
}
console.log(out.filter(o => o.status !== "ok").map(o => JSON.stringify(o)).join("\n"));
console.log("total unique", uniq.length, "ok", out.filter(o => o.status === "ok").length, "packages found", pk.size);
