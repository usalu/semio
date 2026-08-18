#!/usr/bin/env bun
/** [DEBUG] D0-descriptor-plumbing scratch tool (ticket-folder only, not part of the repo): adds the
 * `describe` command + project.json target to every top-level plugin crate's own `📜️script.ts`,
 * mirroring the hand-written `🗒️note` registration exactly. Idempotent (skips a file that already
 * imports `describePluginComponent`). Run once, then delete. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const pluginsDir = join(repoRoot, "✏️s/🔌️plugins");
const { readdirSync } = await import("node:fs");

const names = readdirSync(pluginsDir, { withFileTypes: true })
  .filter((e) => e.isDirectory())
  .map((e) => e.name)
  .filter((name) => {
    try {
      readFileSync(join(pluginsDir, name, "📦️packages/🦀️rust/📜️script.ts"), "utf8");
      return true;
    } catch {
      return false;
    }
  });

console.log(`[DEBUG] found ${names.length} plugin crates with a rust 📜️script.ts`);

const IMPORT_LINE = `import { describePluginComponent } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/📜️script.ts";`;

let changed = 0;
let skipped = 0;

for (const name of names) {
  const scriptPath = join(pluginsDir, name, "📦️packages/🦀️rust/📜️script.ts");
  const projectPath = join(pluginsDir, name, "📦️packages/🦀️rust/📋️project.json");
  let src = readFileSync(scriptPath, "utf8");

  if (src.includes("describePluginComponent")) {
    skipped++;
    continue;
  }

  const crateMatch = src.match(/runCargoTestBudgeted\(\s*\[\s*"([^"]+)"/);
  if (!crateMatch) {
    console.log(`[DEBUG] SKIP ${name}: no runCargoTestBudgeted([...]) crate name found`);
    skipped++;
    continue;
  }
  const crateName = crateMatch[1];

  // 1) node:path import, right after the doc-comment block.
  const docCommentEnd = src.indexOf("*/");
  if (docCommentEnd === -1) {
    console.log(`[DEBUG] SKIP ${name}: no doc-comment block found`);
    skipped++;
    continue;
  }
  const afterDoc = docCommentEnd + 2;
  src = `${src.slice(0, afterDoc)}\nimport { join } from "node:path";${src.slice(afterDoc)}`;

  // 2) describePluginComponent import, right after the existing library import line.
  const libImportMatch = src.match(/^import \{[^}]*\} from "\.\.\/\.\.\/\.\.\/\.\.\/\.\.\/🧰️framework\/🛍️products\/🦑️repo\/🔨️modules\/📚️library\/📦️packages\/🟦️typescript\/📦️index\.ts";$/m);
  if (!libImportMatch) {
    console.log(`[DEBUG] SKIP ${name}: no shared-library import line found`);
    skipped++;
    continue;
  }
  const libImportEnd = libImportMatch.index! + libImportMatch[0].length;
  src = `${src.slice(0, libImportEnd)}\n${IMPORT_LINE}${src.slice(libImportEnd)}`;

  // 3) DescribeScript class, right before `const router = `.
  const routerDeclMatch = src.match(/^const router = /m);
  if (!routerDeclMatch) {
    console.log(`[DEBUG] SKIP ${name}: no 'const router = ' line found`);
    skipped++;
    continue;
  }
  const describeClass = `/** @emoji 🛂️ Builds this crate's \`wasm32-wasip2\` component and re-emits \`🛂️descriptor.semio\` +\n * \`🔣️descriptor.json\` at this plugin's own owner root (D0-descriptor-plumbing) — the command\n * \`📇️registry:check\`'s own descriptor-gate warning tells a developer to run. */\nclass DescribeScript extends BundleScript {\n  run(): void {\n    process.exit(describePluginComponent(this.repoRoot, "${crateName}", join(this.root, "..", "..")));\n  }\n}\n\n`;
  src = `${src.slice(0, routerDeclMatch.index)}${describeClass}${src.slice(routerDeclMatch.index)}`;

  // 4) register("describe", DescribeScript) on the router chain.
  const before = src;
  src = src.replace(/(new ScriptRouter\(import\.meta\.dir\)(?:\.register\("[^"]+",\s*\w+\))+)(;|\n)/, (_all, chain, tail) => `${chain}.register("describe", DescribeScript)${tail}`);
  if (src === before) {
    console.log(`[DEBUG] SKIP ${name}: router chain regex did not match`);
    skipped++;
    continue;
  }

  writeFileSync(scriptPath, src);

  // 5) project.json "describe" target.
  const project = JSON.parse(readFileSync(projectPath, "utf8"));
  if (!project.targets.describe) {
    const testTarget = project.targets.test;
    project.targets.describe = {
      executor: "nx:run-commands",
      options: {
        cwd: testTarget.options.cwd,
        command: "bun ./📜️script.ts describe",
        forwardAllArgs: true,
      },
    };
    writeFileSync(projectPath, `${JSON.stringify(project, null, 2)}\n`);
  }

  changed++;
  console.log(`[DEBUG] OK ${name} (${crateName})`);
}

console.log(`[DEBUG] done: ${changed} changed, ${skipped} skipped`);
