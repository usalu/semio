// 🧾 Static manifest check of companion crates (Bun TOML): workspace resolves, [lib] path and path dependencies exist, boundary keys exact, member present, owner row and project name agree, path length.
import {existsSync, readFileSync} from "node:fs";
import {dirname, join, normalize, relative} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const owners = new Map<string, any>((JSON.parse(readFileSync(join(ROOT, "📋️build-owners.json"), "utf8")).owners as any[]).map(o => [o.name, o]));
let ok = true;
for (const man of process.argv.slice(2)) {
  const dir = dirname(join(ROOT, man)), t = Bun.TOML.parse(readFileSync(join(ROOT, man), "utf8")) as any, errs: string[] = [];
  const ws = normalize(join(dir, t.package.workspace)), wsMan = join(ws, "Cargo.toml");
  if (!existsSync(wsMan)) errs.push(`workspace ${ws} has no Cargo.toml`);
  const lib = normalize(join(dir, t.lib?.path ?? "src/lib.rs"));
  if (!existsSync(lib)) errs.push(`lib path missing ${lib}`);
  const b = t.package.metadata?.semio?.["rust-source-boundary"] ?? {};
  if (JSON.stringify(Object.keys(b).sort()) !== JSON.stringify(["maxActiveLines", "maxSourceBytes", "maxSourceFiles"]) || b.maxSourceBytes !== 262144 || b.maxSourceFiles !== 32 || b.maxActiveLines !== 4000) errs.push(`boundary ${JSON.stringify(b)}`);
  const deps = (tab: any) => { for (const [k, v] of Object.entries(tab ?? {}) as [string, any][]) if (v && typeof v === "object" && v.path && !existsSync(join(normalize(join(dir, v.path)), "Cargo.toml"))) errs.push(`dep ${k} path missing ${v.path}`); };
  for (const s of ["dependencies", "dev-dependencies", "build-dependencies"]) deps(t[s]);
  for (const tg of Object.values(t.target ?? {}) as any[]) for (const s of ["dependencies", "dev-dependencies"]) deps(tg[s]);
  const pj = JSON.parse(readFileSync(join(dir, "📋️project.json"), "utf8")), row = owners.get(pj.name);
  if (!row) errs.push(`owner row missing for ${pj.name}`); else if (join(ROOT, row.root) !== dir || row.manifest !== row.root + "/📋️project.json") errs.push(`owner row root/manifest mismatch ${row.root}`);
  const member = relative(ws, dir);
  if (!existsSync(wsMan) || !readFileSync(wsMan, "utf8").includes(`"${member}"`)) errs.push(`not a member of ${wsMan}: ${member}`);
  const bytes = Buffer.byteLength(join(ROOT, man));
  if (bytes > 240) errs.push(`path bytes ${bytes}`);
  console.log((errs.length ? "FAIL " : "OK   ") + t.package.name + " " + errs.join("; "));
  ok &&= !errs.length;
}
process.exit(ok ? 0 : 1);
