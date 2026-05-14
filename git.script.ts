#!/usr/bin/env bun
/**
 * 🔧 Git workspace helpers: `setup` enables symlink policy, AGENTS.md alias links, and repo client `configure`.
 */
import { execFileSync } from "node:child_process";
import { existsSync, linkSync, rmSync, symlinkSync } from "node:fs";
import { join } from "node:path";

const root = import.meta.dir;
const sub = process.argv[2];

if (sub !== "setup") {
  console.error("usage: bun ./git.script.ts setup");
  process.exit(1);
}

//#region 🔖GitSetup
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
  execFileSync("go", ["run", "./repo/client/mcp", "configure"], {
    stdio: "inherit",
    cwd: root,
    env: { ...process.env, GOWORK: join(root, "go.work") },
  });
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
//#endregion
