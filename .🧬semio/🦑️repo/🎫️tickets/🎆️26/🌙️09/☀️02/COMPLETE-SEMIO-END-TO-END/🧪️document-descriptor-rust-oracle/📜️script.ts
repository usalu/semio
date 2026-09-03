#!/usr/bin/env bun

const command = process.argv[2];
if (command !== "test" && command !== "check") throw new Error("expected test or check");
const args = command === "test"
  ? ["cargo", "test", "--manifest-path", "Cargo.toml", "--lib", "--", "document_descriptor"]
  : ["cargo", "check", "--manifest-path", "Cargo.toml", "--all-features"];
const result = Bun.spawnSync(args, { cwd: import.meta.dir, env: process.env, stdout: "inherit", stderr: "inherit" });
process.exit(result.exitCode);
