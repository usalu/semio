/** 🧱️ Re-derives the declared production dependencies of the ui crate-boundaries corpus from the manifests and reports (or applies with --write) rows whose declared edges drifted. */
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
const ui = resolve(process.cwd(), "🧰️framework/🔨️modules/🖱️ui");
const path = resolve(ui, "🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json");
const text = readFileSync(path, "utf8");
const corpus = JSON.parse(text) as { crates: { package: string; manifest: string; dependencies: string[]; forbiddenDependencies: string[] }[] };
let changed = 0;
for (const row of corpus.crates) {
  const own = Bun.TOML.parse(readFileSync(resolve(ui, row.manifest), "utf8")) as any;
  const declared = Object.assign({}, own.dependencies, ...Object.values(own.target ?? {}).map((target: any) => target.dependencies ?? {}));
  const actual = Object.entries(declared).map(([alias, edge]: [string, any]) => edge.package ?? alias).sort();
  if (JSON.stringify(actual) === JSON.stringify([...row.dependencies].sort())) continue;
  const forbidden = actual.filter((name) => row.forbiddenDependencies.includes(name));
  console.log(row.package, "added", actual.filter((n) => !row.dependencies.includes(n)), "removed", row.dependencies.filter((n) => !actual.includes(n)), "forbidden", forbidden);
  if (!forbidden.length) { row.dependencies = actual; changed++; }
}
if (process.argv.includes("--write")) {
  if (readFileSync(path, "utf8") !== text) throw new Error("fixture changed meanwhile");
  writeFileSync(path, JSON.stringify(corpus, null, 2) + "\n");
}
console.log("rows changed:", changed);
