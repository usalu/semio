#!/usr/bin/env bun
/** 🏛️ `@semio/architect` — `bun script.ts <build|test|wasm>`. */
import { execFileSync } from "node:child_process";
import { join, resolve } from "node:path";

const root = resolve(import.meta.dir);
const bun = process.execPath;

function run(cmd: string, args: string[]): void {
  execFileSync(cmd, args, { stdio: "inherit", cwd: root });
}

const sub = process.argv[2] ?? "test";
if (sub === "build") {
  run(bun, [join(root, "scripts/build-wasm.script.mjs")]);
  run("cargo", ["build", "--release"]);
} else if (sub === "wasm") {
  run(bun, [join(root, "scripts/build-wasm.script.mjs")]);
} else if (sub === "test") {
  run("cargo", ["test", ...process.argv.slice(3)]);
} else {
  console.error(`usage: bun script.ts <build|test|wasm>`);
  process.exit(1);
}
