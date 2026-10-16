// 📋️ TOML parse of the seven WP2 manifests (from OUT or the repository), path dependencies resolving to manifests of the declared package, [lib] path resolving to the crate root, both launch files parsing.
import {readFileSync, existsSync} from "node:fs";
import {resolve, dirname} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const PLUG = ROOT + "/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin";
const crates: Record<string, {pkg: string; root: string; src: string; id: string; rank: number; deps: string[]}> = JSON.parse(process.env.CRATES!);
const rankOf: Record<string, number> = JSON.parse(process.env.RANKS!);
const rootToml: any = Bun.TOML.parse(readFileSync(ROOT + "/Cargo.toml", "utf8"));
const owners: any = JSON.parse(readFileSync(ROOT + "/📋️build-owners.json", "utf8"));
const added: string[] = [];
let bad = 0, deps = 0;
for (const [name, c] of Object.entries(crates)) {
  const path = c.src;
  const parsed: any = Bun.TOML.parse(readFileSync(path, "utf8"));
  if (parsed.package.name !== name) { bad++; console.log("NAME", name, parsed.package.name); }
  const pkgDir = PLUG + "/" + c.pkg;
  const lib = resolve(pkgDir, parsed.lib.path);
  if (lib !== PLUG + "/" + c.root || !existsSync(lib)) { bad++; console.log("LIB", name, parsed.lib.path); }
  const meta = parsed.package.metadata.semio;
  const b = meta["rust-source-boundary"];
  if (meta.role !== "framework" || meta.id !== c.id) { bad++; console.log("META", name); }
  if (JSON.stringify(Object.keys(b).sort()) !== JSON.stringify(["maxActiveLines", "maxSourceBytes", "maxSourceFiles"]) || b.maxSourceBytes !== 262144 || b.maxSourceFiles !== 32 || b.maxActiveLines !== 4000) { bad++; console.log("BOUNDARY", name); }
  if (resolve(pkgDir, parsed.package.workspace) !== ROOT || parsed.lints?.workspace !== true || parsed.package["rust-version"]?.workspace !== true) { bad++; console.log("WORKSPACE", name); }
  if (!rootToml.workspace.members.includes((PLUG + "/" + c.pkg).slice(ROOT.length + 1))) { bad++; console.log("MEMBER", name); }
  if (!owners.owners.some((o: any) => o.name === "@semio-tech/" + c.id + "-rs" && o.root === (PLUG + "/" + c.pkg).slice(ROOT.length + 1))) { bad++; console.log("OWNER", name); }
  for (const dep of Object.keys(parsed.dependencies ?? {})) {
    if (dep.startsWith("semio-framework-plugin-") && rankOf[dep] !== undefined) {
      if (rankOf[dep] >= c.rank) { bad++; console.log("UPWARD", name, dep); }
      if (!c.deps.includes(dep)) added.push(name + " -> " + dep);
    }
  }
  const tables = [parsed.dependencies, parsed["dev-dependencies"], ...Object.values(parsed.target ?? {}).flatMap((t: any) => [t.dependencies, t["dev-dependencies"]])];
  for (const table of tables) for (const [key, spec] of Object.entries(table ?? {}) as [string, any][]) {
    if (spec && typeof spec === "object" && typeof spec.path === "string") {
      deps++;
      const target = resolve(pkgDir, spec.path, "Cargo.toml");
      if (!existsSync(target)) { bad++; console.log("MISSING", name, key, spec.path); continue; }
      const dep: any = Bun.TOML.parse(readFileSync(target, "utf8"));
      const want = spec.package ?? key;
      if (dep.package?.name !== want) { bad++; console.log("PACKAGE", name, key, dep.package?.name, want); }
    }
  }
}
JSON.parse(readFileSync(ROOT + "/.vscode/launch.json", "utf8").replace(/^﻿/, ""));
const seed = readFileSync(ROOT + "/.vscode/🧩️launch.seed.jsonc", "utf8").replace(/^﻿/, "");
const stripped = seed.replace(/("(?:[^"\\]|\\.)*")|\/\/[^\n]*|\/\*[\s\S]*?\*\//g, (m, s) => s ?? "").replace(/,(\s*[}\]])/g, "$1");
JSON.parse(stripped);
console.log("manifests", Object.keys(crates).length, "path deps", deps, "problems", bad);
console.log("first-party edges added to the frozen Appendix H set (all downward):", added.length);
for (const a of added) console.log("   ", a);
