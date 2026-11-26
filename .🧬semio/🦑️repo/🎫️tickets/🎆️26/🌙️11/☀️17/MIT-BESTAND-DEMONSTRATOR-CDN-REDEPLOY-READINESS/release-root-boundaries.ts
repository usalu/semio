import { readFileSync, statSync, writeFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createRustSourceAnalysisV1, readRustSourceClosureV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
import { cargoSourceBoundaryExcessV1, effectiveCargoSourceBoundaryV1, mergeCargoCfgFeaturesV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/🟦️.ts";

type Unit = {
  pkg_id: string;
  target: { name: string; src_path: string; kind: string[] };
  platform: string | null;
  features: string[];
  dependencies: { index: number }[];
};
const root = resolve(import.meta.dirname, "../../../../../../..");
const generated = join(import.meta.dirname, "🗑️generated");
const readJson = (path: string) => JSON.parse(readFileSync(path, "utf8"));
const graph = readJson(join(generated, process.argv[2] ?? "all-canonical-roots-units.json")) as { units: Unit[]; roots: number[] };
const policy = readJson(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json")).storage.sourceBoundary;
const cfgByPlatform = new Map(["host", "proc-macro", "wasm32-wasip2"].map(target => [target, readFileSync(join(generated, "boundary-" + target + ".cfg"), "utf8").trim().split("\n")]));
const analysis = createRustSourceAnalysisV1();
const namesByIndex = new Map<number, Set<string>>();
for (const start of graph.roots) {
  const pending = [start], seen = new Set<number>(), label = graph.units[start]!.target.name;
  while (pending.length) {
    const index = pending.pop()!;
    if (seen.has(index)) continue;
    seen.add(index);
    if (!namesByIndex.has(index)) namesByIndex.set(index, new Set());
    namesByIndex.get(index)!.add(label);
    pending.push(...graph.units[index]!.dependencies.map(edge => edge.index));
  }
}
const outcomes = new Map<string, { roots: Set<string>; report: string; level: number }>();
let observed = 0;
for (const [index, unit] of graph.units.entries()) {
  if (!unit.pkg_id.startsWith("path+file:") || relative(root, unit.target.src_path).startsWith("..")) continue;
  const source = unit.pkg_id.slice(5);
  const manifestDirectory = fileURLToPath(source.split("#")[0]!);
  if (!statSync(join(manifestDirectory, "Cargo.toml"), { throwIfNoEntry: false })?.isFile()) continue;
  const manifest = readFileSync(join(manifestDirectory, "Cargo.toml"), "utf8");
  const packageName = /^name\s*=\s*"([^"]+)"/m.exec(manifest)?.[1] ?? unit.target.name;
  const heading = /^\[package\.metadata\.semio\.rust-source-boundary\]\s*\n([^\[]*)/m.exec(manifest)?.[1];
  const keys = ["maxSourceBytes", "maxSourceFiles", "maxActiveLines"] as const;
  const overrides = Object.fromEntries(keys.map(key => [key, Number(new RegExp("^" + key + "\\s*=\\s*([\\d_]+)", "m").exec(heading ?? "")?.[1]?.replaceAll("_", "") ?? policy[key])]));
  const limits = effectiveCargoSourceBoundaryV1(policy, overrides as typeof policy);
  const cfg = mergeCargoCfgFeaturesV1(cfgByPlatform.get(unit.target.kind.includes("proc-macro") ? "proc-macro" : (unit.platform ?? "host"))!, unit.features);
  const key = JSON.stringify([unit.target.src_path, cfg]);
  const previous = outcomes.get(key);
  const names = namesByIndex.get(index) ?? [];
  if (previous) {
    for (const name of names) previous.roots.add(name);
    continue;
  }
  const prefix = [packageName, unit.platform ?? "host", unit.features.join(", ") || "—"];
  let report: string, level: number;
  try {
    const sources = await readRustSourceClosureV1([unit.target.src_path], root, cfg, async path => {
      try { return readFileSync(path); } catch (error) {
        if (["ENOENT", "ENOTDIR", "EISDIR"].includes((error as NodeJS.ErrnoException).code ?? "")) return null;
        throw error;
      }
    }, { manifestDirectory, generated: () => undefined }, analysis);
    const actual = {
      files: sources.length,
      bytes: sources.reduce((sum, item) => sum + item.bytes, 0),
      lines: sources.reduce((sum, item) => sum + item.activeLines, 0),
    };
    const exceeded = cargoSourceBoundaryExcessV1(limits, actual);
    level = exceeded.length ? 3 : Math.max(actual.files / limits.maxSourceFiles, actual.bytes / limits.maxSourceBytes, actual.lines / limits.maxActiveLines) >= 0.98 ? 1 : 0;
    report = [...prefix, `${actual.lines}/${limits.maxActiveLines}`, `${actual.files}/${limits.maxSourceFiles}`, `${actual.bytes}/${limits.maxSourceBytes}`, exceeded.join(", ") || "—"].join(" | ");
  } catch (error) {
    level = 2;
    report = [...prefix, "—", "—", "—", String(error).replaceAll("|", "\\|").replaceAll("\n", " ")].join(" | ");
  }
  outcomes.set(key, { roots: new Set(names), report, level });
  observed++;
}
const rows = [...outcomes.values()].filter(row => row.level).sort((left, right) => right.level - left.level || left.report.localeCompare(right.report));
const headings = [
  "# Rust source boundaries for the release roots",
  "",
  `Checked ${observed} distinct exact target/feature closures from ${graph.roots.length} selected Hub release roots. The scanner uses the pinned compiler's platform cfg and the production Rust source resolver and limits. Rows show over-limit, unresolved and within 2% of an applicable limit. Admission still rechecks the actual per-build compiler configuration and source epoch.`,
  "",
  "| Unit | Platform | Features | Active lines | Files | Bytes | Limit or unresolved edge | Release roots |",
  "| --- | --- | --- | ---: | ---: | ---: | --- | --- |",
  ...rows.map(row => "| " + row.report + " | " + [...row.roots].map(name => name.replace(/^semio_hub_/, "").replace(/_component$/, "")).sort().join(", ") + " |"),
  "",
  "The same compiled-entry census before the two repairs found BREP Engine at 4,006 active lines and Solid Transaction at 4,074 active lines and 33 files. Both exceeded the original limits. FEM/GIS has no dependency on either repaired closure.",
  "",
];
writeFileSync(join(import.meta.dirname, process.argv[3] ?? "🔬️release-root-boundaries-1125.md"), headings.join("\n"));
console.log(`${observed} closures, ${rows.filter(row => row.level === 3).length} over limit, ${rows.filter(row => row.level === 2).length} unresolved, ${rows.filter(row => row.level === 1).length} near.`);
