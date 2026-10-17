// 🔬️ Cheap closure check for a re-gated owner: type-checks the lib root with and without the feature (non-test) and prints errors that exist only with the feature on. Usage: bun ctc-oracle.ts PACKAGE FEATURE
import {readFileSync} from "node:fs";
import {dirname, resolve, join} from "node:path";
import {execFileSync, spawnSync} from "node:child_process";
const [pkg, feature] = process.argv.slice(2);
const ROOT = "/Users/ueli/Documents/semio/";
const map = new Map(readFileSync(import.meta.dir + "/../🗑️generated/ctc-manifests.tsv", "utf8").split("\n").filter(Boolean).map(l => l.split("\t") as [string, string]));
const manifest = map.get(pkg!);
if (!manifest) throw new Error("unknown package " + pkg);
const t = Bun.TOML.parse(readFileSync(ROOT + manifest, "utf8")) as any;
const dir = dirname(ROOT + manifest);
const lib = resolve(dir, t.lib?.path ?? "src/lib.rs");
const name = t.lib?.name ?? pkg!.replaceAll("-", "_");
const run = (cfgs: string[]) => {
  const r = spawnSync("rustc", ["+nightly-2026-07-20", "--edition", "2021", "--crate-type", "lib", "--crate-name", name, "--emit=metadata", "-o", "/dev/null", ...cfgs.flatMap(c => ["--cfg", c]), lib], {encoding: "utf8", maxBuffer: 1 << 29});
  const lines = (r.stderr ?? "").split("\n");
  const out = new Set<string>();
  for (let i = 0; i < lines.length; i++) {
    if (/^error(\[E\d+\])?:/.test(lines[i]!)) {
      const code = lines[i]!;
      const loc = lines.slice(i + 1, i + 4).find(l => l.includes("-->")) ?? "";
      out.add(code + " " + loc.replace(/:\d+:\d+$/, "").replace(ROOT, "").trim());
    }
  }
  return out;
};
const off = run([]);
const on = run([`feature="${feature}"`]);
const only = [...on].filter(e => !off.has(e) && !/unresolved import|can't find crate|use of undeclared crate|failed to resolve: use of undeclared|cannot find attribute|cannot find derive|cannot find macro|aborting due to/.test(e));
const normal = new Set<string>();
for (const tbl of [t.dependencies, ...Object.values(t.target ?? {}).map((x: any) => x.dependencies)]) for (const [k, v] of Object.entries(tbl ?? {}) as [string, any][]) { normal.add(k.replaceAll("-", "_")); if (v && typeof v === "object" && v.package) normal.add(String(v.package).replaceAll("-", "_")); }
const wsDeps = (() => { try { let d = dir; while (d !== "/") { const m = d + "/Cargo.toml"; try { const w = Bun.TOML.parse(readFileSync(m, "utf8")) as any; if (w.workspace) return w.workspace.dependencies ?? {}; } catch {} d = dirname(d); } } catch {} return {}; })();
void wsDeps;
const crateRefs = [...on].filter(e => !off.has(e)).map(e => /`([A-Za-z0-9_]+)`/.exec(e)?.[1]).filter((x): x is string => !!x && /undeclared crate|cannot find module or crate/.test([...on].find(e => e.includes("`" + x + "`")) ?? ""));
const devOnly = [...new Set(crateRefs)].filter(c => !normal.has(c) && !["std", "core", "alloc", "crate", "self", "super"].includes(c));
if (devOnly.length) console.log("  NEW CRATE REFS NOT IN [dependencies]: " + devOnly.join(", "));
console.log(`${pkg}: errors off=${off.size} on=${on.size}; only-with-feature (filtered)=${only.length}`);
for (const e of only.slice(0, 20)) console.log("  " + e);
