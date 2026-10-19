import { readFileSync, writeFileSync } from "node:fs";

type Cycle = { name: string }[];
type Violation = { type: string; from: string; to: string; rule: { name: string; severity: string }; cycle?: Cycle };
const [, , reportPath, outPath] = process.argv;
const report = JSON.parse(readFileSync(reportPath!, "utf8"));
const violations: Violation[] = report.summary.violations;
const family = (name: string): string => name.replace(/^(no-cross-package-relative|no-cross-technology|no-cross-plugin|plugin-no-extension-or-artifact)-.+$/u, "$1");
const strip = (path: string): string => path.replace(/[^\x00-\x7f]/gu, "");
const count = <T>(rows: T[], key: (row: T) => string): Record<string, number> => {
  const out: Record<string, number> = {};
  for (const row of rows) out[key(row)] = (out[key(row)] ?? 0) + 1;
  return Object.fromEntries(Object.entries(out).sort((a, b) => b[1] - a[1]));
};
const pluginOf = (path: string): string | undefined => /^✏️s\/🔌️plugins\/([^/]+)\//u.exec(path)?.[1];
const area = (path: string): string => strip(path.split("/").slice(0, 3).join("/"));

const perRule = count(violations, (row) => family(row.rule.name));
const severities = count(violations, (row) => row.rule.severity);

const noCore = violations.filter((row) => row.rule.name === "no-core-path");
const segmentOf = (path: string): string => {
  const hit = path.split("/").map((segment) => ({ segment, stem: strip(segment).replace(/\.(rs|ts|tsx|js|mjs|cjs|go|py|cs)$/u, "").toLowerCase() })).find(({ stem }) => ["core", "common", "util", "utils", "helper", "helpers", "misc", "shared", "base", "lib", "impl"].includes(stem));
  return hit ? hit.stem : "(compound)";
};
const noCoreByStem = count(noCore, (row) => segmentOf(row.to));
const noCoreByArea = count(noCore, (row) => area(row.from));

const circular = violations.filter((row) => row.rule.name === "no-circular");
const cyclesByArea = count(circular, (row) => area(row.from));
const pluginCycles = circular.filter((row) => pluginOf(row.from) !== undefined);
const cycleMembers = (row: Violation): string[] => [row.from, ...(row.cycle ?? []).map((step) => step.name)];
const cyclePlugins = (row: Violation): string[] => [...new Set(cycleMembers(row).map(pluginOf).filter((id): id is string => id !== undefined))].sort();
const cross = pluginCycles.filter((row) => cyclePlugins(row).length > 1);
const intraByPlugin = count(pluginCycles.filter((row) => cyclePlugins(row).length === 1), (row) => cyclePlugins(row)[0]!);
const mixedExternal = pluginCycles.filter((row) => cycleMembers(row).some((path) => pluginOf(path) === undefined));
const cycleLength = count(pluginCycles, (row) => String(cycleMembers(row).length));
const pluginCycleRows = pluginCycles.map((row) => ({ plugins: cyclePlugins(row), length: cycleMembers(row).length, members: cycleMembers(row) }));

const result = {
  totalViolations: violations.length,
  totalCruised: report.summary.totalCruised,
  perRule,
  severities,
  noCore: { total: noCore.length, byStem: noCoreByStem, byFromArea: noCoreByArea },
  circular: { total: circular.length, byArea: cyclesByArea, pluginTotal: pluginCycles.length, pluginCrossPlugin: cross.length, pluginIntraByPlugin: intraByPlugin, pluginMixedWithNonPlugin: mixedExternal.length, pluginCycleLengths: cycleLength },
  pluginCycleRows,
  rendererTargets: count(violations.filter((row) => row.rule.name === "renderer-hosts-only-ui"), (row) => (/\.cache\//u.test(row.to) ? ".cache:" + (/node_modules\/\.bun\/([^/]+)/u.exec(row.to)?.[1] ?? row.to) : row.to.startsWith("node_modules/") ? "node_modules:" + row.to.split("/")[1] : strip(row.to.split("/").slice(0, 3).join("/")))),
  noCoreSamples: noCore.slice(0, 40).map((row) => ({ from: row.from, to: row.to })),
};
writeFileSync(outPath!, JSON.stringify(result, null, 1));
console.log(JSON.stringify({ totalViolations: result.totalViolations, perRule, noCore: result.noCore, circular: { total: circular.length, byArea: cyclesByArea, pluginTotal: pluginCycles.length, cross: cross.length } }, null, 1));
