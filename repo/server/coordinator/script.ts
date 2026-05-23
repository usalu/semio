#!/usr/bin/env bun
/** 🧭 Coordinator package router: `bun ./script.ts build`. */
import { execFileSync } from "node:child_process";

const segs = process.argv.slice(2);
const verb = segs[0] ?? "build";

if (verb !== "build") {
  console.error("usage: bun ./script.ts build");
  process.exit(1);
}

const coordinatorRoot = import.meta.dir;
const ext = process.platform === "win32" ? ".exe" : "";
execFileSync("go", ["build", "-o", `server${ext}`, "."], {
  cwd: coordinatorRoot,
  stdio: "inherit",
});
