/**
 * Reads workspace package.json scripts, writes nx project.json targets, strips scripts.
 * Run: node migrate-workspaces-to-nx.mjs
 */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT = join(__dirname, "..", "..", "..", "..", "..", "..");
const pkgPath = join(ROOT, "package.json");
const rootPkg = JSON.parse(readFileSync(pkgPath, "utf8"));

function schemaFromProjectDir(projectDir) {
  const rel = relative(ROOT, projectDir);
  const depth = rel.split(/[/\\]/).filter(Boolean).length;
  const prefix = depth === 0 ? "." : Array(depth).fill("..").join("/");
  return `${prefix}/node_modules/nx/schemas/project-schema.json`;
}

function scriptToTargetName(name) {
  return name.replace(/:/g, "-");
}

for (const ws of rootPkg.workspaces) {
  const dir = join(ROOT, ws);
  const p = join(dir, "package.json");
  if (!existsSync(p)) continue;
  const pkg = JSON.parse(readFileSync(p, "utf8"));
  const scripts = pkg.scripts;
  if (!scripts || Object.keys(scripts).length === 0) continue;

  const targets = {};
  for (const [name, command] of Object.entries(scripts)) {
    targets[scriptToTargetName(name)] = {
      executor: "nx:run-commands",
      options: {
        cwd: ws.replace(/\\/g, "/"),
        command,
        forwardAllArgs: true,
      },
    };
  }

  const projPath = join(dir, "project.json");
  const existing = existsSync(projPath)
    ? JSON.parse(readFileSync(projPath, "utf8"))
    : {};
  const projName = existing.name || pkg.name;
  if (!projName) {
    console.warn("[skip] no project name", ws);
    continue;
  }

  const oldTargets = existing.targets || {};
  const merged = {
    ...existing,
    name: projName,
    $schema: schemaFromProjectDir(dir),
    targets: { ...oldTargets, ...targets },
  };

  writeFileSync(projPath, JSON.stringify(merged, null, 2) + "\n");

  const { scripts: _removed, ...rest } = pkg;
  writeFileSync(p, JSON.stringify(rest, null, 2) + "\n");
  console.log("migrated", ws, projName);
}

console.log("done");
