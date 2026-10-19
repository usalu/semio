import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { C } from "./lib.ts";
import { loadCrates } from "./rs.ts";
const map = JSON.parse(readFileSync(C + "/rmod-tools/crate-map.json", "utf8")).crates as Record<string, string>;
const fams = JSON.parse(readFileSync(C + "/🗑️generated/rmod-families.json", "utf8")) as Record<string, string[]>;
const crates = loadCrates();
const dependents = new Map<string, Set<string>>();
for (const old of Object.keys(map)) dependents.set(old, new Set([old]));
for (const c of crates) {
  const t = readFileSync(c.manifest, "utf8");
  for (const old of Object.keys(map)) if (t.includes(old) && c.name !== old) dependents.get(old)!.add(c.name);
}
const groups: Record<string, string[]> = {
  brep: [...fams["core->sweep_machinery"]!, ...fams["shared->intersection_numerics"]!],
  value: [...fams["shared->immutable_alias"]!, ...fams["shared->issuance_clone"]!],
  norm: [...fams["helpers->part_libraries"]!, ...fams["helpers->check_evidence"]!],
  meshsess: [...fams["helpers->operator_registration"]!, ...fams["helpers->mesh_operator_io"]!],
  editor: fams["base->editor_vocabulary"]!,
  arc: fams["shared->arc_custody"]!,
};
for (const [old] of Object.entries(map)) groups["crate-" + old.replace(/^semio-(framework-|s-)?/, "").slice(0, 40)] = [...dependents.get(old)!];
mkdirSync(C + "/rmod-tools/families", { recursive: true });
for (const [n, cs] of Object.entries(groups)) {
  const uniq = [...new Set(cs)].sort();
  const pre = { name: n, crates: uniq };
  const post = { name: n, crates: uniq.map((x) => map[x] ?? x) };
  writeFileSync(`${C}/rmod-tools/families/${n}.pre.json`, JSON.stringify(pre));
  writeFileSync(`${C}/rmod-tools/families/${n}.post.json`, JSON.stringify(post));
  console.log(n, uniq.length);
}
