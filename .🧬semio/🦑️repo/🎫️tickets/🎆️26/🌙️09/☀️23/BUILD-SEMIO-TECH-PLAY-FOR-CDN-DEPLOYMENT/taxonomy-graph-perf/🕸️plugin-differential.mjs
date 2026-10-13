/** 🕸️ Differential oracle for the Nx inference plugins: runs the pre-change snapshot's and the current createNodesV2/createDependencies over the
 * same Nx-globbed config files, then the current one again (inference-cache hit), and reports whether all results are byte-identical with timings.
 * argv: <workspaceRoot> <oldModulesRoot> <workspaceDataDirectory>; run under node with NODE_PATH at the Nx tooling node_modules. */
import { createRequire } from "node:module";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

const [root, oldRoot, workspaceData] = process.argv.slice(2);
process.env.NX_WORKSPACE_DATA_DIRECTORY = workspaceData;
const require = createRequire(join(process.env.NODE_PATH, "nx", "package.json"));
const { globWithWorkspaceContext } = require("nx/src/utils/workspace-context");
const plugins = [
  { name: "emoji-project-json", module: "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs", glob: "**/{📋️project.json,Cargo.toml,bun.lock,*.patch}", options: { analyzeLockfile: true } },
  { name: "test-cases", module: "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs", glob: "**/*.feature", options: {} },
];
const timed = async (label, run) => { const start = performance.now(), value = await run(); console.log(`[DEBUG] ${label} ${((performance.now() - start) / 1000).toFixed(2)}s`); return value; };
for (const plugin of plugins) {
  const files = await globWithWorkspaceContext(root, [plugin.glob]), context = { workspaceRoot: root, nxJsonConfiguration: {} };
  const current = await import(pathToFileURL(join(root, plugin.module)).href), previous = await import(pathToFileURL(join(oldRoot, plugin.module)).href);
  const before = JSON.stringify(await timed(`${plugin.name} old`, () => previous.default.createNodesV2[1](files, plugin.options, context)));
  const miss = JSON.stringify(await timed(`${plugin.name} new (compute)`, () => current.default.createNodesV2[1](files, plugin.options, context)));
  const hit = JSON.stringify(await timed(`${plugin.name} new (cache)`, () => current.default.createNodesV2[1](files, plugin.options, context)));
  console.log(`[DEBUG] ${plugin.name}: files=${files.length} bytes=${before.length} old==compute ${before === miss} compute==cache ${miss === hit}`);
}
