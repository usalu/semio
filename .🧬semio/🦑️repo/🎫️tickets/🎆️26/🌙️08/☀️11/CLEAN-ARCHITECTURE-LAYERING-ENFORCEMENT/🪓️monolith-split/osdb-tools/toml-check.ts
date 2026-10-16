// 📋️ Parses every staged manifest with Bun.TOML and @iarna/toml, checks names, lib paths and path dependencies.
import {readFileSync, existsSync, readdirSync, statSync} from "node:fs";
import {join, resolve, dirname} from "node:path";
import TOML from "/Users/ueli/Documents/semio/node_modules/@iarna/toml";
const STAGE = process.argv[2]!;
const names = new Map<string, string>();
const manifests: string[] = [];
const walk = (d: string) => { for (const e of readdirSync(d)) { const p = join(d, e); if (statSync(p).isDirectory()) walk(p); else if (e === "Cargo.toml") manifests.push(p); } };
walk(STAGE);
let bad = 0;
for (const m of manifests) { const t = Bun.TOML.parse(readFileSync(m, "utf8")) as any; names.set(t.package.name, m); }
for (const m of manifests) {
  const src = readFileSync(m, "utf8");
  const own = Bun.TOML.parse(src) as any, oracle = TOML.parse(src) as any;
  if (JSON.stringify(own) !== JSON.stringify(oracle)) { console.log("TOML parsers disagree", m); bad++; }
  const lib = resolve(dirname(m), own.lib.path);
  if (!existsSync(lib)) { console.log("lib path missing", m, own.lib.path); bad++; }
  const b = own.package.metadata?.semio?.["rust-source-boundary"] ?? {};
  if (JSON.stringify(Object.keys(b).sort()) !== JSON.stringify(["maxActiveLines", "maxSourceBytes", "maxSourceFiles"]) || b.maxActiveLines !== 4000 || b.maxSourceBytes !== 262144 || b.maxSourceFiles !== 32) { console.log("boundary keys", m); bad++; }
  if (!existsSync(resolve(dirname(m), own.package.workspace, "Cargo.toml"))) { console.log("workspace unresolved", m); bad++; }
  const tables = [own.dependencies, own["dev-dependencies"], ...Object.values(own.target ?? {}).map((t: any) => t.dependencies)];
  for (const tab of tables) for (const [k, v] of Object.entries(tab ?? {}) as [string, any][]) {
    if (typeof v === "object" && typeof v.path === "string") {
      const dep = resolve(dirname(m), v.path, "Cargo.toml");
      if (!manifests.includes(dep) && !existsSync(dep)) { console.log("dep path missing", m, k, v.path); bad++; }
    }
  }
}
console.log("manifests", manifests.length, "problems", bad);
