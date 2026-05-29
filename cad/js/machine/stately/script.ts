#!/usr/bin/env bun
/** @emoji 🧭 `@cad/js-machine-stately` task router; `generate` catalogs model-definition interactions via core. */
import { join, resolve } from "node:path";
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
} else if (command === "generate") {
	const { SHAPE_MODEL_DEFINITION_ID } = await import("@cad/js-core");
	const { buildSpatialStatelyMachineCatalogView } = await import("./index.ts");
	let outPath = join(cwd, "machine.json");
	let modelDefinitionId = SHAPE_MODEL_DEFINITION_ID;
	const interactionIds: string[] = [];
	for (let i = 0; i < extra.length; i++) {
		const a = extra[i]!;
		if (a === "--out" && extra[i + 1]) {
			outPath = resolve(cwd, extra[i + 1]!);
			i++;
			continue;
		}
		if (a === "--model-definition" && extra[i + 1]) {
			modelDefinitionId = extra[i + 1]!;
			i++;
			continue;
		}
		if (!a.startsWith("-")) interactionIds.push(a);
	}
	const doc = buildSpatialStatelyMachineCatalogView({
		modelDefinitionId,
		interactionIds: interactionIds.length > 0 ? interactionIds : undefined,
	});
	await Bun.write(outPath, `${JSON.stringify(doc, null, 2)}\n`);
	console.error(`wrote ${outPath} (${doc.machines.length} machine(s))`);
	process.exit(0);
} else {
	console.error("usage: bun ./script.ts test [args…]");
	console.error("       bun ./script.ts generate [--out path] [--model-definition id] [interactionId …]");
	process.exit(1);
}
