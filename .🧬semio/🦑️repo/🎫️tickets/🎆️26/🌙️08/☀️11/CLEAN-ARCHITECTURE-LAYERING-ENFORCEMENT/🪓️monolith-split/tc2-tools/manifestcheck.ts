// tc2: parses every companion / edited manifest with Bun.TOML, checks workspace deps exist, path deps resolve, features forward to declared deps, members listed once.
import {readFileSync, existsSync, readdirSync} from "node:fs";
import {dirname, join, resolve} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const here = import.meta.dir;
const specs = readdirSync(join(here, "specs")).filter(f => f.endsWith(".json")).map(f => JSON.parse(readFileSync(join(here, "specs", f), "utf8")));
const extra: string[] = JSON.parse(readFileSync(join(here, "touched-manifests.json"), "utf8"));
let bad = 0;
const fail = (m: string) => { bad++; console.log("FAIL", m); };
const toml = (p: string) => Bun.TOML.parse(readFileSync(p, "utf8")) as any;
const manifests = [...specs.map(s => join(ROOT, s.dir, "📦️packages/🦀️rust/Cargo.toml")), ...extra.map(p => join(ROOT, p))];
let checked = 0;
for (const mp of manifests) {
  if (!existsSync(mp)) { fail("missing " + mp); continue; }
  const t = toml(mp);
  checked++;
  const wsDir = resolve(dirname(mp), t.package.workspace ?? ".");
  const ws = toml(join(wsDir, "Cargo.toml"));
  const wdeps = ws.workspace?.dependencies ?? {};
  const tables = [t.dependencies, t["dev-dependencies"], t["build-dependencies"]];
  for (const tab of tables) for (const [k, v] of Object.entries(tab ?? {}) as [string, any][]) {
    if (v && v.workspace === true && !(k in wdeps)) fail(`${mp.slice(ROOT.length)}: workspace dep ${k} not in workspace.dependencies`);
    if (v && typeof v.path === "string" && !existsSync(join(resolve(dirname(mp), v.path), "Cargo.toml"))) fail(`${mp.slice(ROOT.length)}: path dep ${k} -> ${v.path} missing`);
  }
  const depKeys = new Set([...Object.keys(t.dependencies ?? {})]);
  const optional = new Set(Object.entries(t.dependencies ?? {}).filter(([, v]: any) => v?.optional).map(([k]) => k));
  for (const [f, items] of Object.entries(t.features ?? {}) as [string, string[]][]) for (const it of items) {
    if (it.startsWith("dep:")) { if (!optional.has(it.slice(4))) fail(`${mp.slice(ROOT.length)}: feature ${f} uses ${it} but it is not an optional dependency`); continue; }
    const slash = it.indexOf("/");
    if (slash > 0) { const dep = it.slice(0, slash).replace(/\?$/, ""); if (!depKeys.has(dep)) fail(`${mp.slice(ROOT.length)}: feature ${f} forwards to ${dep} which is not a normal dependency`); }
    else if (!(it in (t.features ?? {})) && !optional.has(it)) fail(`${mp.slice(ROOT.length)}: feature ${f} names unknown ${it}`);
  }
  const members: string[] = ws.workspace?.members ?? [];
  const rel = dirname(mp).slice(wsDir.length + 1);
  if (t.package.name && specs.some(s => s.name === t.package.name)) {
    const n = members.filter(m => m === rel).length;
    if (n !== 1) fail(`${t.package.name}: listed ${n} times in members of ${wsDir.slice(ROOT.length)}`);
  }
}
console.log(`checked ${checked} manifests, failures ${bad}`);
