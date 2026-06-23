#!/usr/bin/env bun
import { readdirSync, readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const root = "c:/git/compose/spatial/assets/modelDefinition";
const norm = (p: string) => p.replace(/\\/g, "/");

const actionPaths = new Map<string, string>();
const commitActions = new Set<string>();
const typologyFiles: string[] = [];

function walk(dir: string, onFile: (path: string) => void): void {
	for (const entry of readdirSync(dir, { withFileTypes: true })) {
		const p = join(dir, entry.name);
		if (entry.isDirectory()) walk(p, onFile);
		else onFile(p);
	}
}

walk(root, (p) => {
	const n = norm(p);
	if (n.endsWith(".json") && n.includes("/action/")) {
		const j = JSON.parse(readFileSync(p, "utf8")) as { id?: string };
		if (j.id) actionPaths.set(j.id, p);
	}
	if (n.endsWith(".json") && n.includes("/interaction/")) {
		const j = JSON.parse(readFileSync(p, "utf8")) as { commit?: { operation?: { action?: string } } };
		const id = j.commit?.operation?.action;
		if (id) commitActions.add(id);
	}
	if (entryBasename(p) === "typology.json") typologyFiles.push(p);
});

function entryBasename(p: string): string {
	const parts = norm(p).split("/");
	return parts[parts.length - 1] ?? "";
}

const capabilityDoc = (id: string, label: string) =>
	`${JSON.stringify(
		{
			schema: "spatial.action/v1",
			id,
			version: "1.0.0",
			label,
			steps: [
				{ op: "kernel.call", function: "spatial.action.capability", assignTo: "result" },
				{ op: "return", result: { kind: "var", name: "result" } },
			],
		},
		null,
		2,
	)}\n`;

let created = 0;
for (const id of commitActions) {
	if (actionPaths.has(id)) continue;
	const dir = join(root, "_generated", "action");
	mkdirSync(dir, { recursive: true });
	const file = join(dir, `${id.replace(/\./g, "-")}.json`);
	writeFileSync(file, capabilityDoc(id, id.split(".").pop() ?? id));
	actionPaths.set(id, file);
	created++;
}

let typologyFixed = 0;
for (const file of typologyFiles) {
	const t = JSON.parse(readFileSync(file, "utf8")) as {
		schema?: string;
		id?: string;
		actions?: string[];
		interactions?: string[];
	};
	if (t.schema !== "spatial.typology/v1" || !t.interactions?.length) continue;
	const interactionPath = norm(file).replace("/typology.json", `/interaction/${t.interactions[0]!.split(".").pop()}.json`);
	const alt = norm(file).replace("/typology.json", `/interaction/${t.interactions[0]}.json`);
	let commitId: string | undefined;
	for (const candidate of [interactionPath, alt]) {
		try {
			const raw = readFileSync(candidate.replace(/\//g, "\\"), "utf8");
			commitId = (JSON.parse(raw) as { commit?: { operation?: { action?: string } } }).commit?.operation?.action;
			if (commitId) break;
		} catch {
			/* try glob sibling */
		}
	}
	if (!commitId) {
		const folder = norm(file).replace("/typology.json", "/interaction");
		try {
			for (const name of readdirSync(folder.replace(/\//g, "\\"))) {
				if (!name.endsWith(".json")) continue;
				const raw = readFileSync(join(folder.replace(/\//g, "\\"), name), "utf8");
				const j = JSON.parse(raw) as { id?: string; commit?: { operation?: { action?: string } } };
				if (j.id === t.interactions![0]) {
					commitId = j.commit?.operation?.action;
					break;
				}
			}
		} catch {
			continue;
		}
	}
	if (!commitId) continue;
	const next = [...new Set([commitId, ...(t.actions ?? []).filter((a) => actionPaths.has(a))])];
	if (JSON.stringify(t.actions) === JSON.stringify(next)) continue;
	t.actions = next;
	writeFileSync(file, `${JSON.stringify(t, null, 2)}\n`);
	typologyFixed++;
}

const missing = [...commitActions].filter((id) => !actionPaths.has(id));
console.log(`[DEBUG] commitActions=${commitActions.size} catalog=${actionPaths.size} created=${created} typologyFixed=${typologyFixed} missing=${missing.length}`);
if (missing.length) {
	console.log(missing.join("\n"));
	process.exit(1);
}
