/** 🔍️ Diagnostic: per development-dependencies row, the difference between Cargo dev-dependencies and the fixture references. */
import * as toml from "@iarna/toml";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { packagesForOwner } from "@semio-tech/repo-test/dependencies";
import contract from "/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🧫️fixtures/🧩️composition/🔣️.json";
import development from "/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🧫️fixtures/🧪️development-dependencies/🔣️.json";
const root = "/Users/ueli/Documents/semio";
const all = [...(contract as any).families ?? [], ...(contract as any).providers ?? []];
const byPackage = new Map<string, any>();
for (const row of all) byPackage.set(row.package.package, row);
const out: any[] = [];
for (const row of development.owners as any[]) {
  const owner = byPackage.get(row.package);
  if (!owner) { out.push({ package: row.package, missing: true }); continue; }
  const cargo: any = toml.parse(readFileSync(resolve(root, owner.package.path, "Cargo.toml"), "utf8"));
  const have = Object.keys(cargo["dev-dependencies"] ?? {}).sort(), want = [...row.references].sort();
  if (JSON.stringify(have) !== JSON.stringify(want)) out.push({ package: row.package, extra: have.filter((x) => !want.includes(x)), lacking: want.filter((x) => !have.includes(x)) });
}
console.log(JSON.stringify(out, null, 1), development.owners.length);
