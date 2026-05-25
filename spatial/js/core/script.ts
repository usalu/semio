#!/usr/bin/env bun
/** @emoji 🧭 `@spatial/js-core` task router: `bun ./script.ts test` | `bun ./script.ts migrate-topology-fixtures`. */
import { spawnSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const cwd = import.meta.dir;
const segs = process.argv.slice(2);
const command = segs[0];
const extra = segs.slice(1);

// #region 🧱MigrateTopologyFixtures
/** @emoji 🧱 Rewrites legacy id-keyed topology buckets to sorted arrays (`spatial.topology/v1`). */
function bucketToArray(v: unknown): unknown[] {
	if (Array.isArray(v)) return v;
	if (!v || typeof v !== "object") return [];
	const o = v as Record<string, unknown>;
	return Object.keys(o)
		.sort()
		.map((k) => o[k]);
}

function migrateTopologyFixture(obj: Record<string, unknown>): Record<string, unknown> {
	const next = { ...obj };
	for (const k of ["vertices", "edges", "wires", "faces", "shells", "cells", "cellComplexes", "clusters"]) {
		if (k in next) next[k] = bucketToArray(next[k]);
	}
	return next;
}
// #endregion 🧱MigrateTopologyFixtures

// #region 🏢ConvertBuildingFixtures
const convertBuildingsScript = join(cwd, "..", "..", "..", ".repo", "🎫", "26", "05", "25", "FIX-BUILDING-CELL-COMPLEX-PLAY", "convert-buildings.ts");
// #endregion 🏢ConvertBuildingFixtures

if (command === "test") {
	const r = spawnSync("bunx", ["vitest", "run", "--config", "vitest.config.ts", ...extra], {
		cwd,
		stdio: "inherit",
		shell: true,
		env: process.env,
	});
	process.exit(r.status ?? 1);
} else if (command === "migrate-topology-fixtures") {
	const root = join(cwd, "..", "..", "fixtures");
	const names = [
		"geometry.json",
		"geometry-routes.json",
		"geometry-loom.json",
		"small-building.topology.json",
		"tall-building.topology.json",
		"large-building.topology.json",
	];
	for (const n of names) {
		const p = join(root, n);
		const raw = JSON.parse(readFileSync(p, "utf8")) as Record<string, unknown>;
		const next = migrateTopologyFixture(raw);
		writeFileSync(p, JSON.stringify(next, null, 2) + "\n");
		console.log("[DEBUG] migrated", p);
	}
	process.exit(0);
} else if (command === "convert-building-fixtures") {
	const r = spawnSync("bun", [convertBuildingsScript], { cwd, stdio: "inherit", shell: true, env: process.env });
	process.exit(r.status ?? 1);
} else {
	console.error(
		"usage: bun ./script.ts test [args…] | bun ./script.ts migrate-topology-fixtures | bun ./script.ts convert-building-fixtures",
	);
	process.exit(1);
}
