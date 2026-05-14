#!/usr/bin/env bun
/** 🧹 Runs workspace lint plus dependency-cruiser on selected JS packages. */
import { execFileSync } from "node:child_process";

const root = import.meta.dir;
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
