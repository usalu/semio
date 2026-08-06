import { readdirSync } from "node:fs";
import { join, relative } from "node:path";

const workspaceRoot = process.cwd();
const pluginDir = process.argv[2];

const SKIP_DIRS = new Set(["node_modules", ".git", "target", "dist", ".nx", "pkg", ".venv"]);

/** @param {string} dir @param {string[]} out */
function walk(dir, out) {
  let entries;
  try {
    entries = readdirSync(dir, { withFileTypes: true });
  } catch {
    return out;
  }
  for (const e of entries) {
    const full = join(dir, e.name);
    if (e.isDirectory()) {
      if (SKIP_DIRS.has(e.name) || e.name === ".🦑️repo") continue;
      walk(full, out);
    } else if (e.isFile()) {
      out.push(relative(workspaceRoot, full).split("\\").join("/"));
    }
  }
  return out;
}

const allFiles = walk(workspaceRoot, []);

const plugins = [
  ["🟨️nx-plugin.mjs", "📜️script.ts"],
  ["🟨️nx-emoji-project-plugin.mjs", "📋️project.json"],
];

const result = [];
for (const [file, basename] of plugins) {
  const mod = (await import(join(pluginDir, file))).default;
  const fn = mod.createNodesV2[1];
  const matches = allFiles.filter((f) => f.endsWith("/" + basename) || f === basename);
  const out = await fn(matches, {}, { workspaceRoot });
  for (const entry of out) {
    if (!entry) continue;
    const [, res] = entry;
    for (const [name, proj] of Object.entries(res.projects ?? {})) {
      result.push({ plugin: mod.name, name, root: proj.root, project: proj });
    }
  }
}

result.sort((a, b) => (a.name + a.root).localeCompare(b.name + b.root));
console.log(JSON.stringify(result, null, 2));
console.error(`[harness] plugins=${plugins.length} projects=${result.length}`);
