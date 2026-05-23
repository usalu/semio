#!/usr/bin/env bun
/** Sets each typology `actions` to action JSON ids under the same typology folder. */
import { readdirSync, readFileSync, writeFileSync, existsSync } from "node:fs";
import { join, dirname } from "node:path";

const root = "c:/git/semio/spatial/assets/modelDefinition";

function walk(dir: string, onFile: (path: string) => void): void {
	for (const entry of readdirSync(dir, { withFileTypes: true })) {
		const p = join(dir, entry.name);
		if (entry.isDirectory()) walk(p, onFile);
		else onFile(p);
	}
}

let updated = 0;
walk(root, (file) => {
	if (!file.endsWith("typology.json")) return;
	const typology = JSON.parse(readFileSync(file, "utf8")) as {
		schema?: string;
		actions?: string[];
		interactions?: string[];
	};
	if (typology.schema !== "spatial.typology/v1") return;
	const folder = dirname(file);
	const actionDir = join(folder, "action");
	const ids: string[] = [];
	if (existsSync(actionDir)) {
		for (const name of readdirSync(actionDir)) {
			if (!name.endsWith(".json")) continue;
			const j = JSON.parse(readFileSync(join(actionDir, name), "utf8")) as { id?: string };
			if (j.id) ids.push(j.id);
		}
	}
	const interactionDir = join(folder, "interaction");
	if (existsSync(interactionDir)) {
		for (const name of readdirSync(interactionDir)) {
			if (!name.endsWith(".json")) continue;
			const j = JSON.parse(readFileSync(join(interactionDir, name), "utf8")) as {
				commit?: { operation?: { action?: string } };
			};
			const commitId = j.commit?.operation?.action;
			if (commitId && !ids.includes(commitId)) ids.push(commitId);
		}
	}
	ids.sort((a, b) => a.localeCompare(b));
	if (JSON.stringify(typology.actions ?? []) === JSON.stringify(ids)) return;
	typology.actions = ids;
	writeFileSync(file, `${JSON.stringify(typology, null, 2)}\n`);
	updated++;
});

console.log(`[DEBUG] synced ${updated} typology action lists`);
