import { lstatSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, renameSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { msiDestination } from "./msi-paths.ts";

const generated = resolve(process.env.SEMIO_TICKET_DIR!, "🗑️generated"), source = join(generated, "sdk-flat-2356413337");
const plan = JSON.parse(readFileSync(join(generated, "sdk-file-plan.json"), "utf8"));
if (plan.length !== 4453 || readdirSync(source).length !== 4453) throw Error("SDK staging inventory differs");
const data = Array.from({ length: 8 }, (_, i) => JSON.parse(readFileSync(join(generated, `sdk-files-${i}.json`), "utf8")));
const destinations = new Set<string>();
for (const row of plan) {
  if (!Number.isInteger(row.index) || row.index < 0 || row.index >= 8 || !/^[a-z0-9_]+$/iu.test(row.id)) throw Error("Invalid staged file identity");
  const table = data[row.index], sourceRow = table.files.find((item: string[]) => item[0] === row.id), component = sourceRow && table.components.find((item: string[]) => item[0] === sourceRow[1]);
  if (!sourceRow || !component || msiDestination(table.directories, component[1], sourceRow[2]) !== row.path || Number(sourceRow[3]) !== row.size || !row.path.startsWith("Windows Kits/10/")) throw Error(`SDK file plan mismatch: ${row.id}`);
  const relative = row.path.slice("Windows Kits/10/".length), key = relative.toLowerCase();
  if (destinations.has(key)) throw Error("Duplicate SDK destination");
  destinations.add(key);
  const entry = lstatSync(join(source, row.id));
  if (!entry.isFile() || entry.isSymbolicLink() || entry.size !== row.size) throw Error(`SDK staged file mismatch: ${row.id}`);
}
const dest = mkdtempSync(join(generated, "k-"));
for (const row of plan) {
  const relative = row.path.slice("Windows Kits/10/".length), target = join(dest, ...relative.split("/"));
  if (target.length > 256) throw Error("SDK destination exceeds path limit");
  mkdirSync(dirname(target), { recursive: true });
  renameSync(join(source, row.id), target);
}
writeFileSync(join(dest, ".complete.json"), JSON.stringify({ files: plan.length, manifest: "a8e854fa9f3fbaeac27d7304e297f1660d7137401389deb57cf7983f33c16fef", version: "10.0.26100.0" }), { flag: "wx" });
console.log(`Materialized ${plan.length} SDK files in ${dest}`);
