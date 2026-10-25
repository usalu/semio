import { existsSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { loadDependencyDirectionPolicy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🚀️bootstrap/🟦️.ts";
import { dependencyDirectionEdges, dependencyDirectionSourceInventory, dependencyDirectionStrictRules, type DependencyDirectionGraphScope } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🟦️.ts";
const repoRoot = "/Users/ueli/Documents/semio", graphPath = process.argv[2]!, out = process.argv[3]!;
const snapshot = loadDependencyDirectionPolicy(repoRoot);
const { policy, taxonomy, workspacePackages } = snapshot, rules = dependencyDirectionStrictRules(snapshot);
const areas = Object.keys(taxonomy.areaLayers).filter((area) => existsSync(join(repoRoot, area)));
const rootSources = readdirSync(repoRoot, { withFileTypes: true }).filter((entry) => (entry.isFile() || entry.isSymbolicLink()) && /\.(?:[cm]?[jt]s|[jt]sx)$/u.test(entry.name)).map((entry) => entry.name);
const scope: DependencyDirectionGraphScope = { workspaceRoots: Object.keys(taxonomy.areaLayers), workspacePackages, excludedPaths: policy.options.exclude.path, nonFollowedPaths: [policy.options.doNotFollow.path], expectedSources: [] };
const expectedSources = dependencyDirectionSourceInventory(repoRoot, [...areas, ...rootSources], scope);
const graph = JSON.parse(readFileSync(graphPath, "utf8"));
const graphSources = new Set((graph.modules as { source: string }[]).map((module) => module.source));
const inventoried = expectedSources.filter((source: any) => graphSources.has(typeof source === "string" ? source : source.path ?? source.source));
try {
  const edges = dependencyDirectionEdges(graph, rules, { ...scope, expectedSources: inventoried });
  writeFileSync(out, JSON.stringify({ ok: true, edges }, null, 1));
  console.log("edges", edges.length);
} catch (error) {
  writeFileSync(out, String((error as Error).message));
  console.log("threw", String((error as Error).message).split("\n").length, "lines");
}
