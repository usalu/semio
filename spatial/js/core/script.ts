#!/usr/bin/env bun
/** @emoji 🧭 `@spatial/js-core` task router: `bun ./script.ts test`. */
import { spawnSync } from "node:child_process";

const cwd = import.meta.dir;
const segs = process.argv.slice(2);
const command = segs[0];
const extra = segs.slice(1);

if (command === "test") {
	const r = spawnSync("bunx", ["vitest", "run", "--config", "vitest.config.ts", ...extra], {
		cwd,
		stdio: "inherit",
		shell: true,
		env: process.env,
	});
	process.exit(r.status ?? 1);
} else if (command === "sync-typology-construct") {
	const r = spawnSync("bun", ["./sync-typology-construct.ts"], {
		cwd,
		stdio: "inherit",
		shell: true,
		env: process.env,
	});
	process.exit(r.status ?? 1);
} else {
	console.error("usage: bun ./script.ts test|sync-typology-construct [args…]");
	process.exit(1);
}
