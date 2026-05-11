#!/usr/bin/env bun
/** Git symlinks + repo client configure (AGENTS aliases). */
import { execFileSync } from "node:child_process";
import { existsSync, linkSync, rmSync, symlinkSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "..");
const source = "AGENTS.md";
const aliases = ["CLAUDE.md", "GEMINI.md"];

function run(cmd: string, args: string[]) {
  execFileSync(cmd, args, { stdio: "inherit", cwd: root });
}

run("git", ["config", "--local", "core.symlinks", "true"]);

const repoClientCandidates = [
  join(root, "repo", "client", "client.exe"),
  join(root, "repo", "client", "client"),
];
const repoClientPath = repoClientCandidates.find((p) => existsSync(p));
if (repoClientPath) {
  run(repoClientPath, ["configure"]);
} else {
  run("go", ["run", "./repo/mcp", "configure"]);
}

for (const alias of aliases) {
  const aliasPath = join(root, alias);
  if (existsSync(aliasPath)) rmSync(aliasPath, { force: true });
  try {
    symlinkSync(source, aliasPath, "file");
  } catch (error) {
    if (process.platform !== "win32") throw error;
    linkSync(join(root, source), aliasPath);
  }
}
