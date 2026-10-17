// 🔎 FB: parses the touched manifests with Bun's TOML, checks workspace paths, lib paths, dependency paths and byte-sorted workspace members.
import {readFileSync, existsSync} from "node:fs";
import {resolve, dirname} from "node:path";
const R = "/Users/ueli/Documents/semio";
const E = "🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine";
const dirs = ["📦️packages", "🧬️kernel-contract/📦️packages", "📦️mesh-io/📦️packages", "🪅️shape-value/📦️packages", "🧪️tests/📦️packages"].map(d => `${E}/${d}/🦀️rust`);
let bad = 0;
const ws = (Bun.TOML.parse(readFileSync(`${R}/🧰️framework/Cargo.toml`, "utf8")) as any);
for (const d of dirs) {
  const m = resolve(R, d, "Cargo.toml");
  const t = Bun.TOML.parse(readFileSync(m, "utf8")) as any;
  const wsRoot = resolve(R, d, t.package.workspace);
  const okWs = existsSync(resolve(wsRoot, "Cargo.toml")) && wsRoot === resolve(R, "🧰️framework");
  const lib = resolve(R, d, t.lib.path);
  const okLib = existsSync(lib);
  const miss: string[] = [];
  for (const sect of ["dependencies", "dev-dependencies"]) for (const [k, v] of Object.entries(t[sect] ?? {}) as [string, any][]) {
    if (v.path && !existsSync(resolve(R, d, v.path, "Cargo.toml"))) miss.push(`${k}:${v.path}`);
    if (v.workspace === true && !ws.workspace.dependencies[k]) miss.push(`${k}:workspace-missing`);
  }
  const bytes = Buffer.byteLength(m.replace(R + "/", ""));
  if (!okWs || !okLib || miss.length || bytes > 240) bad++;
  console.log(t.package.name, "workspace", okWs, "lib", okLib, "missing", miss, "pathBytes", bytes);
}
const members: string[] = ws.workspace.members;
const sorted = [...members].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
console.log("members", members.length, "byte-sorted", JSON.stringify(sorted) === JSON.stringify(members));
for (const m of members) if (!existsSync(`${R}/🧰️framework/${m}/Cargo.toml`)) { console.log("MISSING member", m); bad++; }
for (const need of dirs.map(d => d.replace("🧰️framework/", ""))) if (!members.includes(need)) { console.log("NOT A MEMBER", need); bad++; }
console.log("bad", bad);
