#!/usr/bin/env bun
/** 🧹 Workspace lint: Nx `lint` (optional `repo` via `lint.repo.script.ts`) + dependency-cruiser on selected JS packages. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = import.meta.dir;
const sub = process.argv[2];

if (sub === "repo") {
  execFileSync("bun", [join(root, "lint.repo.script.ts")], { cwd: root, stdio: "inherit" });
  process.exit(0);
}

execFileSync("bun", ["nx", "run-many", "-t", "lint", "--all", "--exclude", "workspace"], {
  cwd: root,
  stdio: "inherit",
});
execFileSync(
  "bunx",
  [
    "dependency-cruiser@16",
    "semio/client/lib/js",
    "semio/client/lib/react",
    "semio/client/lib/sketchpad",
    "--config",
    ".dependency-cruiser.cjs",
    "--output-type",
    "err",
  ],
  { cwd: root, stdio: "inherit", shell: true },
);
