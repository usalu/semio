/** 🧮️ Classifies the cells of the sample matrices: `bun classify.ts <matrix-dir> <engine.ts>` prints status totals, failing cells per code × scale kind × scale key × viewpoint × mode × category, per asset rows, one example per code and asset, and the extrapolation to the catalog universe. */
import { readdirSync, readFileSync, existsSync } from "node:fs";
import { join, resolve } from "node:path";

type Failures = { count: number; first: { subject: string; message: string }[]; worst?: { deltaMm: number; subject: string; pair: string } };
type Cell = { asset: string; category: string; scale: string; scaleKind: string; requested: number | null; view: string; mode: string; status: string; checked: number; failures: Record<string, Failures>; rangeEnd?: Record<string, Failures>; groupDrift?: Record<string, Failures> };

const dir = resolve(process.argv[2]!);
const engine = await import(resolve(process.argv[3]!));
const cells: Cell[] = readdirSync(dir, { withFileTypes: true })
  .filter((entry) => entry.isDirectory())
  .flatMap((entry) =>
    existsSync(join(dir, entry.name, "matrix.json"))
      ? (JSON.parse(readFileSync(join(dir, entry.name, "matrix.json"), "utf8")).cells as Cell[])
      : readdirSync(join(dir, entry.name))
          .filter((name) => /^shard-\d+\.json$/u.test(name))
          .flatMap((name) => JSON.parse(readFileSync(join(dir, entry.name, name), "utf8")) as Cell[]),
  );

const bump = (map: Map<string, number>, key: string, by = 1): void => void map.set(key, (map.get(key) ?? 0) + by);
const table = (title: string, map: Map<string, number>): void => {
  console.log(`\n## ${title}`);
  for (const [key, value] of [...map].sort((a, b) => a[0].localeCompare(b[0]))) console.log(`${key}\t${value}`);
};
const repOf = (cell: Cell): string => (cell.scaleKind === "alternative" ? `${cell.scale.split(">")[0]}>${cell.scale.split(":")[1]}` : cell.scale.replace(/^\d+:/u, (m) => m));

const status = new Map<string, number>();
const statusByCategory = new Map<string, number>();
const perAsset = new Map<string, { category: string; cells: number; fail: number; codes: Map<string, number> }>();
const byCode = new Map<string, number>();
const dims: Record<string, Map<string, number>> = { kind: new Map(), scale: new Map(), view: new Map(), mode: new Map(), category: new Map(), asset: new Map() };
const combo = new Map<string, number>();
const examples = new Map<string, string>();
const soft = new Map<string, number>();
for (const cell of cells) {
  bump(status, cell.status);
  bump(statusByCategory, `${cell.category}\t${cell.status}`);
  const row = perAsset.get(cell.asset) ?? perAsset.set(cell.asset, { category: cell.category, cells: 0, fail: 0, codes: new Map() }).get(cell.asset)!;
  row.cells++;
  if (cell.status === "fail") row.fail++;
  for (const code of Object.keys(cell.rangeEnd ?? {})) bump(soft, `${code}\t${cell.category}`);
  for (const code of Object.keys(cell.groupDrift ?? {})) bump(soft, `${code}\t${cell.category}`);
  const codes = Object.keys(cell.failures).sort();
  if (codes.length > 0) bump(combo, codes.join("+"));
  for (const code of codes) {
    bump(byCode, code);
    bump(row.codes, code);
    bump(dims.kind!, `${code}\t${cell.scaleKind}`);
    bump(dims.scale!, `${code}\t${cell.scaleKind === "alternative" ? `${cell.requested} alternative` : cell.scale}`);
    bump(dims.view!, `${code}\t${cell.view}`);
    bump(dims.mode!, `${code}\t${cell.mode}`);
    bump(dims.category!, `${code}\t${cell.category}`);
    bump(dims.asset!, `${code}\t${cell.asset}`);
    const key = `${code}\t${cell.asset}`;
    const f = cell.failures[code]!;
    if (!examples.has(key)) examples.set(key, `${cell.scale} ${cell.view} ${cell.mode} | n=${f.count} | ${f.first[0]?.subject}: ${f.first[0]?.message}${f.worst ? ` | worst ${f.worst.deltaMm.toFixed(1)} mm ${f.worst.pair} @ ${f.worst.subject}` : ""}`);
  }
}
void repOf;
console.log(`cells ${cells.length} assets ${perAsset.size}`);
table("status", status);
table("status by category", statusByCategory);
table("failing cells by code", byCode);
table("code combinations per failing cell", combo);
for (const [name, map] of Object.entries(dims)) table(`code x ${name}`, map);
table("soft (range-end, group-drift) by category", soft);
console.log("\n## per asset");
for (const [id, row] of [...perAsset].sort((a, b) => a[1].category.localeCompare(b[1].category) || a[0].localeCompare(b[0]))) console.log(`${row.category}\t${id}\t${row.cells}\t${row.fail}\t${[...row.codes].map(([code, n]) => `${code}:${n}`).join(" ")}`);
console.log("\n## examples");
for (const [key, text] of [...examples].sort((a, b) => a[0].localeCompare(b[0]))) console.log(`${key}\t${text}`);

const catalog = engine.loadCatalog() as { id: string; category: string }[];
const universeByCategory = new Map<string, number>();
const assetsByCategory = new Map<string, number>();
for (const asset of catalog) {
  bump(universeByCategory, asset.category, engine.universe([asset]).length);
  bump(assetsByCategory, asset.category);
}
console.log("\n## catalog universe");
let all = 0;
for (const [category, n] of [...universeByCategory].sort()) {
  all += n;
  console.log(`${category}\t${assetsByCategory.get(category)}\t${n}`);
}
console.log(`total\t${catalog.length}\t${all}`);
const sampleByCategory = new Map<string, number>();
const sampleFailByCategory = new Map<string, number>();
for (const cell of cells) {
  bump(sampleByCategory, cell.category);
  if (cell.status === "fail") bump(sampleFailByCategory, cell.category);
}
console.log("\n## extrapolation (category rate x category universe)");
const estimate = new Map<string, number>();
for (const [key, n] of dims.category!) {
  const [code, category] = key.split("\t") as [string, string];
  bump(estimate, code, (n / sampleByCategory.get(category)!) * universeByCategory.get(category)!);
}
for (const [code, n] of [...estimate].sort((a, b) => b[1] - a[1])) console.log(`${code}\t${Math.round(n)}`);
let failEstimate = 0;
for (const [category, n] of sampleFailByCategory) failEstimate += (n / sampleByCategory.get(category)!) * universeByCategory.get(category)!;
console.log(`failing cells (any code)\t${Math.round(failEstimate)}`);
