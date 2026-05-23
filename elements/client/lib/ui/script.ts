#!/usr/bin/env bun
/** 🧭 `@elements/ui-shell` task router — `bun ./script.ts test` is a no-op until shell-only vitest is wired here. */
const command = process.argv[2];
if (command === "test") {
	console.log("[DEBUG] @elements/ui-shell: tests run via @elements/ui consumer vitest");
	process.exit(0);
}
console.error("usage: bun ./script.ts test");
process.exit(1);
