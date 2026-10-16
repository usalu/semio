// 📋️ TOML parse of every carved crate manifest, the top manifest and both launch files (JSON / JSONC), plus path dependencies resolving to manifests of the declared package.
import {readFileSync, existsSync} from "node:fs";
import {resolve, dirname} from "node:path";
const ROOT = "/Users/ueli/Documents/semio";
const T = ROOT + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu";
const crates: Record<string, string> = JSON.parse(process.env.CRATES!);
let bad = 0, deps = 0;
for (const [name, folder] of Object.entries(crates)) {
  const path = T + "/" + folder + "/📦️packages/🦀️rust/Cargo.toml";
  const parsed: any = Bun.TOML.parse(readFileSync(path, "utf8"));
  if (parsed.package.name !== name) { bad++; console.log("NAME", name, parsed.package.name); }
  const tables = [parsed.dependencies, parsed["dev-dependencies"], ...Object.values(parsed.target ?? {}).flatMap((t: any) => [t.dependencies, t["dev-dependencies"]])];
  for (const table of tables) for (const [key, spec] of Object.entries(table ?? {}) as [string, any][]) {
    if (spec && typeof spec === "object" && typeof spec.path === "string") {
      deps++;
      const target = resolve(dirname(path), spec.path, "Cargo.toml");
      if (!existsSync(target)) { bad++; console.log("MISSING", name, key, spec.path); continue; }
      const dep: any = Bun.TOML.parse(readFileSync(target, "utf8"));
      const want = spec.package ?? key;
      if (dep.package?.name !== want) { bad++; console.log("PACKAGE", name, key, dep.package?.name, want); }
    }
  }
}
JSON.parse(readFileSync(ROOT + "/.vscode/launch.json", "utf8").replace(/^﻿/, ""));
console.log("manifests", Object.keys(crates).length, "path deps", deps, "problems", bad);
