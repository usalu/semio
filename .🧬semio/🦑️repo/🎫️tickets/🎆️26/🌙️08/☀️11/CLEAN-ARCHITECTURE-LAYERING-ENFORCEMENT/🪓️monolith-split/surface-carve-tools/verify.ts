// 🧾️ Static registration checks for the surface crates. usage: bun verify.ts <crate-dir-relative-to-repo>...
import {readFileSync, existsSync} from "node:fs";
import {join, resolve, dirname} from "node:path";
const R = "/Users/ueli/Documents/semio";
const dirs = process.argv.slice(2);
const read = (p: string) => readFileSync(p, "utf8");
const strip = (s: string) => s.replace(/^\s*\/\/.*$/gm, "");
const launch = [read(R + "/.vscode/launch.json"), read(R + "/.vscode/🧩️launch.seed.jsonc")];
const launchJson = JSON.parse(launch[0]);
JSON.parse(strip(launch[1]).replace(/,(\s*[}\]])/g, "$1"));
const owners = JSON.parse(read(R + "/📋️build-owners.json")).owners as {name: string; root: string; manifest: string; packages: string[]}[];
const fwToml = read(R + "/🧰️framework/Cargo.toml");
const names = new Set<string>(); let bad = 0;
const fail = (m: string) => { bad++; console.log("FAIL", m); };
for (const d of dirs) {
  const pkg = join(R, d), mp = join(pkg, "Cargo.toml"), m: any = Bun.TOML.parse(read(mp));
  const name = m.package.name; names.add(name);
  if (resolve(pkg, m.package.workspace) !== R + "/🧰️framework") fail(`${name} workspace`);
  if (!existsSync(resolve(pkg, m.lib.path))) fail(`${name} lib path`);
  const b = m.package.metadata?.semio?.["rust-source-boundary"];
  if (JSON.stringify(Object.keys(b ?? {}).sort()) !== JSON.stringify(["maxActiveLines", "maxSourceBytes", "maxSourceFiles"]) || b.maxActiveLines !== 4000 || b.maxSourceFiles !== 32 || b.maxSourceBytes !== 262144) fail(`${name} boundary`);
  const groups = [m.dependencies, m["dev-dependencies"], ...Object.values(m.target ?? {}).map((t: any) => t.dependencies)];
  for (const g of groups) for (const [k, v] of Object.entries<any>(g ?? {})) if (v && typeof v === "object" && v.path) {
    const dm: any = Bun.TOML.parse(read(join(resolve(pkg, v.path), "Cargo.toml")));
    if (dm.package.name !== (v.package ?? k)) fail(`${name} dep ${k} resolves to ${dm.package.name}`);
  }
  for (const t of m.test ?? []) if (!existsSync(resolve(pkg, t.path))) fail(`${name} test path ${t.path}`);
  const member = d.replace("🧰️framework/", "");
  if (!fwToml.includes(`"${member}"`)) fail(`${name} not a framework workspace member`);
  const proj = JSON.parse(read(join(pkg, "📋️project.json"))); const owner = owners.filter(o => o.name === proj.name);
  if (owner.length !== 1 || owner[0]!.root !== d || owner[0]!.manifest !== d + "/📋️project.json") fail(`${name} owner row`);
  for (const t of ["build", "check", "test"]) for (const c of ["prototype", "release"]) {
    const cmd = `bun repo run ${proj.name}:${t} --configuration=${c} --parallel=1 -- --lib --offline`;
    if (launchJson.configurations.filter((x: any) => x.command === cmd).length !== 1) fail(`${name} launch.json ${t} ${c}`);
    if (launch[1].split(`"command": "${cmd}"`).length !== 2) fail(`${name} seed ${t} ${c}`);
  }
  console.log("ok", name, Buffer.byteLength(d + "/📋️project.json"));
}
const memberRows = [...fwToml.matchAll(/^\s*"([^"]+)",?$/gm)].map(x => x[1]!);
for (const d of dirs) if (memberRows.filter(x => x === d.replace("🧰️framework/", "")).length > 1) fail(`dup member ${d}`);
console.log(bad ? `${bad} FAILURES` : "ALL STATIC CHECKS PASSED");
process.exit(bad ? 1 : 0);
