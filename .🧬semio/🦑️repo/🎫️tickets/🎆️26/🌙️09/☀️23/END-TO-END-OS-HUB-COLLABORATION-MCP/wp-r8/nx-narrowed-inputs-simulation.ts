/** 🧮️ R8 S12-7 proof-by-planning: applies the narrowed-inputs design IN MEMORY to the real project graph and re-plans every plugin's
 * describe/component-dev/materialize-dev with Nx's own HashPlanInspector (nothing runs, nothing is written to the repo).
 * Design: (1) a named input never carries a workspace-rooted negation without the positives it subtracts from — `production` inlines
 * `default` instead of referencing it (Nx evaluates a referenced input's negations on their own: measured, a lone
 * `!{workspaceRoot}/…` fileset expands to the whole repository); (2) `describe` hashes the component's native closure, like
 * component-*. Experiments: (a) edit one plugin → which OTHER plugins' tasks would miss; (b) edit a shared crate → exactly its
 * Cargo dependents miss. Usage: bun nx-narrowed-inputs-simulation.ts <out.json> [--baseline] */
import { writeFileSync } from "node:fs";
import { createProjectGraphAsync } from "nx/src/project-graph/project-graph";
import { HashPlanInspector } from "nx/src/hasher/hash-plan-inspector";

process.chdir("/Users/ueli/Documents/semio");
const out = process.argv[2]!, baseline = process.argv.includes("--baseline");
const graph = await createProjectGraphAsync({ exitOnError: true });
const ROOT_TOOLCHAIN = new Set(["Cargo.toml", "Cargo.lock", ".cargo/config.toml", "rust-toolchain.toml", "rustfmt.toml", "package.json", "bun.lock", "bunfig.toml", "tsconfig.json"].map((path) => `{workspaceRoot}/${path}`));
if (!baseline) {
  for (const node of Object.values(graph.nodes)) {
    const named = node.data.namedInputs;
    if (named?.production?.includes("default") && named.default) named.production = [...named.default, ...named.production.filter((input: unknown) => input !== "default" && input !== "!{workspaceRoot}/**/🧫️fixtures/**/*")];
    else if (!named?.production && graph.nodes) node.data.namedInputs = { ...named, production: ["default", "!{projectRoot}/**/🧪️tests/**/*", "!{projectRoot}/**/🧫️fixtures/**/*"] };
    const generatorOnly = (named?.default ?? []).filter((input: unknown) => typeof input === "string" && input.startsWith("{workspaceRoot}/") && !input.startsWith(`{workspaceRoot}/${node.data.root.split("/📦️packages/")[0]}/`) && !ROOT_TOOLCHAIN.has(input));
    if (generatorOnly.length && node.name === "@semio-tech/framework-graph") {
      node.data.namedInputs = { ...node.data.namedInputs, generatorSources: [...(named!.default as unknown[])], default: (named!.default as unknown[]).filter((input) => !generatorOnly.includes(input)), production: (node.data.namedInputs!.production as unknown[]).filter((input) => !generatorOnly.includes(input)) };
      for (const name of ["generate", "preview-generated"]) if (node.data.targets[name]) node.data.targets[name]!.inputs = ["generatorSources"];
    }
    const describe = node.data.targets?.describe;
    const component = node.data.targets?.["component-dev"];
    if (describe && component) describe.inputs = [...(component.inputs ?? []).filter((input: unknown) => input === "nativeSources" || (typeof input === "object" && input !== null && "input" in input)), ...(describe.inputs ?? []).filter((input: unknown) => typeof input === "string" && input.startsWith("commandSources"))];
  }
}
const plugins = Object.values(graph.nodes).filter((node) => node.data.targets?.["component-dev"]).map((node) => node.name).sort();
const inspector = new HashPlanInspector(graph);
await inspector.init();
const plan = inspector.inspectHashPlan(plugins, ["describe", "component-dev", "materialize-dev"], undefined, {}, {}, true);
const files = Object.fromEntries(Object.entries(plan).map(([task, items]) => [task, new Set(items.filter((item) => item.startsWith("file:")).map((item) => item.slice(5)))]));
const pluginDir = (path: string) => path.startsWith("✏️s/🔌️plugins/") ? path.split("/")[2] : undefined;
const rootOf = (name: string) => graph.nodes[name]!.data.root;
const counts: Record<string, number[]> = {};
for (const [task, set] of Object.entries(files)) (counts[task.slice(task.lastIndexOf(":") + 1)] ??= []).push(set.size);
const median = (values: number[]) => values.sort((a, b) => a - b)[Math.floor(values.length / 2)]!;
const summary = Object.fromEntries(Object.entries(counts).map(([target, values]) => [target, { min: Math.min(...values), median: median(values), max: Math.max(...values) }]));
const reverse = new Map<string, Set<string>>();
for (const [source, edges] of Object.entries(graph.dependencies)) for (const edge of edges) (reverse.get(edge.target) ?? reverse.set(edge.target, new Set()).get(edge.target)!).add(source);
const dependents = (name: string) => { const seen = new Set<string>(); const stack = [name]; while (stack.length) { const next = stack.pop()!; for (const up of reverse.get(next) ?? []) if (!seen.has(up)) { seen.add(up); stack.push(up); } } return seen; };
const experimentA = plugins.map((plugin) => {
  const dir = pluginDir(rootOf(plugin));
  const missing = Object.entries(files).filter(([task, set]) => !task.startsWith(`${plugin}:`) && [...set].some((path) => pluginDir(path) === dir)).map(([task]) => task.slice(0, task.lastIndexOf(":")));
  const allowed = new Set([...dependents(plugin)].filter((name) => plugins.includes(name)));
  const pluginsSameDir = plugins.filter((name) => name !== plugin && pluginDir(rootOf(name)) === dir);
  const unexpected = [...new Set(missing)].filter((name) => !allowed.has(name) && !pluginsSameDir.includes(name));
  return { plugin, dir, invalidated: [...new Set(missing)].length, unexpected };
});
const shared = ["@semio-tech/framework-replication-rs", "@semio-tech/ui-contract-rs", "semio-framework-mesh-engine", "@semio-tech/stdio-dwg-rs", "@semio-tech/stdio-png-rs", "semio-framework-geometry"].filter((name) => graph.nodes[name]);
const experimentB = shared.map((crate) => {
  const root = rootOf(crate).split("/📦️packages/")[0];
  const sample = [...new Set(Object.values(files).flatMap((set) => [...set]))].find((path) => path.startsWith(`${root}/`) && path.endsWith(".rs"));
  const missing = new Set(Object.entries(files).filter(([, set]) => sample && set.has(sample)).map(([task]) => task.slice(0, task.lastIndexOf(":"))));
  const expected = new Set([...dependents(crate)].filter((name) => plugins.includes(name)));
  return { crate, sample, missingPlugins: missing.size, cargoDependentPlugins: expected.size, extra: [...missing].filter((name) => !expected.has(name)), notMissing: [...expected].filter((name) => !missing.has(name)) };
});
const explainDirs = ["🖍️draw", "🌊️flow", "📏️layout"];
const explain = Object.fromEntries(Object.entries(files).flatMap(([task, set]) => {
  const own = pluginDir(rootOf(task.slice(0, task.lastIndexOf(":"))));
  const leaked = [...set].filter((path) => explainDirs.includes(pluginDir(path) ?? "") && pluginDir(path) !== own);
  return leaked.length ? [[task, { count: leaked.length, sample: leaked.slice(0, 5) }]] : [];
}));
writeFileSync(out, JSON.stringify({ baseline, summary, experimentA, experimentB, explain }, null, 1));
console.log(JSON.stringify({ baseline, summary, pluginsWithUnexpectedInvalidation: experimentA.filter((row) => row.unexpected.length).length, experimentB: experimentB.map(({ crate, missingPlugins, cargoDependentPlugins, extra, notMissing }) => ({ crate, missingPlugins, cargoDependentPlugins, extra: extra.length, notMissing: notMissing.length })) }));
