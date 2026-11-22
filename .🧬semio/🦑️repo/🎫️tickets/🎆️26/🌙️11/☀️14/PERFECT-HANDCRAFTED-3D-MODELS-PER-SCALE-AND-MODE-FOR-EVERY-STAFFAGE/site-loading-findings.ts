/** 🩺️ Runs `validateAsset` natively on asset documents and prints, per document, the wall and CPU time and a digest of the findings; `--dump <file>` writes the findings themselves so two runs can be compared byte for byte.
 * Usage: `bun site-loading-findings.ts [--portable <previous portable module>] [--dump out.json] <--all | category/slug | path/to/asset.json>...` (`--all`: every asset of the live catalog)
 * A slug selects the catalog asset (the category is only a label); a `.json` path reads that document instead, e.g. a frozen copy outside the live worktree.
 * `--portable` loads the given file in place of the framework's portable math module (`git show HEAD:<portable path>` written to a generated file), so the same validator sources and the same documents
 * run once with the previous and once with the current `hypot`: the only difference between the two runs is that one function. */
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const args = process.argv.slice(2);
const option = (name: string): string | null => (args[0] === name ? args.splice(0, 2)[1]! : null);
const portable = option("--portable");
const dump = option("--dump");
if (portable !== null) {
  const contents = readFileSync(resolve(portable), "utf8");
  Bun.plugin({ name: "previous-portable", setup: (build) => void build.onLoad({ filter: /🧮️math\/🎯️portable\/🟦️\.ts$/u }, () => ({ contents, loader: "ts" })) });
}
const { assetPathsFromFiles, readStreamedAsset } = await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts");
const { validateAsset } = await import("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts");
const math = await import("../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts");
console.log(`portable hypot: ${String(math.hypot).includes("inputs.map(Number)") ? "previous" : "current"}`);
const entries = await assetPathsFromFiles();
const all: Record<string, unknown> = {};
for (const name of args[0] === "--all" ? entries.map((entry) => `${entry.categoryFolder}/${entry.folder}`) : args) {
  const frozen = name.endsWith(".json");
  const slug = frozen ? name.split("/").at(-1)!.slice(0, -5) : name.split("/").at(-1)!;
  const entry = entries.find((candidate) => candidate.folder === slug);
  if (!frozen && entry === undefined) throw new Error(`no asset ${slug}`);
  const asset = frozen ? (JSON.parse(readFileSync(name, "utf8")) as { asset: unknown }).asset : (await readStreamedAsset(entry!)).asset;
  const cpu = process.cpuUsage();
  const start = performance.now();
  const issues = validateAsset(asset as never);
  const wall = performance.now() - start;
  const used = process.cpuUsage(cpu);
  const label = name.split("/").slice(-2).join("/");
  all[label] = issues;
  console.log(JSON.stringify({ document: label, wallMs: Math.round(wall), cpuMs: Math.round((used.user + used.system) / 1000), findings: issues.length, sha256: createHash("sha256").update(JSON.stringify(issues)).digest("hex") }));
}
if (dump !== null) writeFileSync(dump, JSON.stringify(all, null, 1));
