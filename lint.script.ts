#!/usr/bin/env bun
/**
 * 🧹 Workspace lint: Prettier write, Nx `lint` (or `@repo/*` only with `repo`), dependency-cruiser on selected JS packages.
 * Subcommands: `repo` — lint only `@repo/*`; `format` — Prettier write only.
 */
import { execFileSync } from "node:child_process";

const root = import.meta.dir;
const sub = process.argv[2];

if (sub === "format") {
  execFileSync("bunx", ["prettier", "-w", "."], { cwd: root, stdio: "inherit", shell: true });
  process.exit(0);
}

if (sub === "repo") {
  execFileSync("bun", ["nx", "run-many", "-t", "lint", "-p", "@repo/*"], {
    cwd: root,
    stdio: "inherit",
  });
  process.exit(0);
}

execFileSync("bunx", ["prettier", "-w", "."], { cwd: root, stdio: "inherit", shell: true });

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
