#!/usr/bin/env bun
/**
 * Reads workspace package.json scripts, writes nx project.json targets (nx:run-commands), strips scripts.
 */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";

const ROOT = join(import.meta.dir, "..", "..", "..", "..", "..", "..");
const pkgPath = join(ROOT, "package.json");
const rootPkg = JSON.parse(readFileSync(pkgPath, "utf8")) as {
  workspaces: string[];
};

function schemaFromProjectDir(projectDir: string): string {
  const rel = relative(ROOT, projectDir);
  const depth = rel.split(/[/\\]/).filter(Boolean).length;
  const prefix = depth === 0 ? "." : Array(depth).fill("..").join("/");
  return `${prefix}/node_modules/nx/schemas/project-schema.json`;
}

function scriptToTargetName(name: string): string {
  return name.replace(/:/g, "-");
}

for (const ws of rootPkg.workspaces) {
  const dir = join(ROOT, ws);
  const p = join(dir, "package.json");
  if (!existsSync(p)) continue;
  const pkg = JSON.parse(readFileSync(p, "utf8")) as {
    name?: string;
    scripts?: Record<string, string>;
    targets?: Record<string, unknown>;
    [k: string]: unknown;
  };
  const scripts = pkg.scripts;
  if (!scripts || Object.keys(scripts).length === 0) continue;

  const targets: Record<string, unknown> = {};
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
    ? (JSON.parse(readFileSync(projPath, "utf8")) as Record<string, unknown>)
    : {};
  const projName = (existing.name as string) || pkg.name;
  if (!projName) {
    console.warn("[skip] no project name", ws);
    continue;
  }

  const oldTargets =
    (existing.targets as Record<string, unknown> | undefined) || {};
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
