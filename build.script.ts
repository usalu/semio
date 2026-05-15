#!/usr/bin/env bun
/** 🏗️ Nx `build`: full workspace, or a named slice (`3dm`, `assets`, `desktop`, …). */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = import.meta.dir;
const slice = process.argv[2];

const single: Record<string, string> = {
  "3dm": "@semio/3dm-ui:build",
  assets: "@semio/assets:build",
  desktop: "@semio/desktop:build",
  engine: "@semio/engine:build",
  "coda-desktop": "@coda/desktop:build",
  "repo-cli": "@repo/client:build",
  "repo-server": "@repo/coordinator:build",
  "repo-vscode": "repo:build-vsix",
};

if (!slice) {
  execFileSync("bun", ["nx", "run-many", "-t", "build", "--all", "--exclude", "workspace"], {
    cwd: root,
    stdio: "inherit",
  });
  process.exit(0);
}

if (slice === "sites") {
  execFileSync("bun", ["nx", "run-many", "-t", "build", "-p", "@semio/play", "@semio/docs"], {
    cwd: root,
    stdio: "inherit",
  });
  process.exit(0);
}

const target = single[slice];
if (!target) {
  console.error(
    `[build] unknown slice ${JSON.stringify(slice)}. Expected one of: ${[...Object.keys(single), "sites"].join(", ")}`,
  );
  process.exit(1);
}

execFileSync("bun", ["nx", "run", target], { cwd: root, stdio: "inherit" });
