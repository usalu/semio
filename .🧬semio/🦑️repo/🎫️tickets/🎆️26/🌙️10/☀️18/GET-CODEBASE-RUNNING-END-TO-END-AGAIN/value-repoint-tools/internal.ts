import { readFileSync, writeFileSync } from "node:fs";
import { relative } from "node:path";
import { initParser, findWasm } from "../rs-sweep-tools/parse.ts";
import { loadOwners, ROOT } from "./resolve-owner.ts";
import { transform, Moved } from "./transform.ts";

const apply = process.argv.includes("--apply");
const G = `${import.meta.dir}/../🗑️generated/value-repoint`;
const moved: Moved = JSON.parse(readFileSync(`${import.meta.dir}/moved.json`, "utf8"));
await initParser(findWasm(ROOT)!);
const owners = loadOwners([...new Set<string>([...Object.values(moved.modules), ...Object.values(moved.functions)])]);
const fileMap: Record<string, string[]> = JSON.parse(readFileSync(`${G}/filemap.json`, "utf8"));
const rows: string[] = [];
for (const [f, dirs] of Object.entries(fileMap)) {
  if (!dirs.includes(owners.facade.dir)) continue;
  const src = readFileSync(f, "utf8");
  const r = transform(src, { moved, facade: "crate", verify: owners.verify });
  if (!r.rewrites && !r.notes.length) continue;
  rows.push(`${relative(ROOT, f)}\t${r.rewrites}\t${r.notes.map((n) => n.kind + ":" + n.line).join(",")}\t${[...r.libs].join(",")}`);
  if (apply && r.out !== src) writeFileSync(f, r.out);
}
writeFileSync(`${G}/facade-internal.tsv`, rows.join("\n") + "\n");
console.log(rows.length, "facade-crate files use moved names through crate::");
console.log(rows.slice(0, 20).join("\n"));
