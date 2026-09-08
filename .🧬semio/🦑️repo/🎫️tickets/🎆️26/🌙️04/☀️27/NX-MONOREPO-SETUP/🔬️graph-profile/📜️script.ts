import { readdirSync, readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
const root = process.cwd(), ticket = dirname(dirname(fileURLToPath(import.meta.url)));
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const policy = JSON.parse(readFileSync(join(root, library, "⚡️caching/🔣️policy.json"), "utf8"));
const files: string[] = [];
const walk = (path: string): void => {
  for (const entry of readdirSync(join(root, path), { withFileTypes: true })) {
    const file = path ? path + "/" + entry.name : entry.name;
    if (entry.isSymbolicLink() || ["compose", "temp/compose"].includes(file) || entry.name === ".🧬semio" || policy.generatedDirectories.includes(entry.name)) continue;
    if (entry.isDirectory()) walk(file);
    else if (["📋️project.json", "Cargo.toml"].includes(entry.name)) files.push(file);
  }
};
walk("");
let modulePath = join(root, library, "🟨️.mjs");
if (process.env.SEMIO_GRAPH_PROFILE_HELPERS === "1") {
  let source = readFileSync(modulePath, "utf8").replace("const LIBRARY_ROOT = dirname(fileURLToPath(import.meta.url));", "const LIBRARY_ROOT = " + JSON.stringify(join(root, library)) + ";");
  source += "\nconst helperTimings = {};\n";
  for (const name of ["nativeCommandInputs", "cargoSourceInputs", "relativeScriptInputs", "nativeDependencyRoots", "projectInputs", "projectWithDefaults"]) {
    source = source.replace("function " + name + "(", "function profiled_" + name + "(");
    source += `function ${name}(...args) { const started=performance.now(); try { return profiled_${name}(...args); } finally { const row=helperTimings[${JSON.stringify(name)}] ??= {calls:0, milliseconds:0}; row.calls++; row.milliseconds+=performance.now()-started; } }\n`;
  }
  source += "export { helperTimings };\n";
  modulePath = join(ticket, "🗑️generated/graph-helpers/📜️script.ts"); mkdirSync(dirname(modulePath), {recursive:true}); writeFileSync(modulePath,source);
}
const module = await import(pathToFileURL(modulePath).href), plugin = module.default;
const timings: number[] = [];
let projects = 0;
for (let index = 0; index < 2; index++) {
  const start = performance.now();
  const results = await plugin.createNodesV2[1](files, {}, { workspaceRoot: root });
  projects = results.reduce((total: number, [, result]: any) => total + Object.keys(result.projects).length, 0);
  timings.push(performance.now() - start);
  console.log("[DEBUG] Native input discovery sample", index, timings.at(-1), "ms", projects, "projects");
}
if (module.helperTimings) console.log("[DEBUG] Helper timings", JSON.stringify(module.helperTimings));
writeFileSync(join(ticket, "🗑️generated", process.env.SEMIO_GRAPH_PROFILE_OUTPUT ?? "graph-discovery-timings.json"), JSON.stringify({ timings, projects, memory: process.memoryUsage() }, null, 2));
