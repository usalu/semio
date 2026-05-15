#!/usr/bin/env bun
/** 🧭 `@repo/lib` router: `bun ./script.ts lint` runs the package lint target via Nx from the monorepo root. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const cwd = import.meta.dir;
const root = join(cwd, "..", "..", "..");
const segs = process.argv.slice(2);

if (segs[0] === "lint") {
  execFileSync("bun", ["nx", "run", "@repo/lib:lint"], { cwd: root, stdio: "inherit" });
} else {
  console.error("usage: bun ./script.ts lint");
  process.exit(1);
}
