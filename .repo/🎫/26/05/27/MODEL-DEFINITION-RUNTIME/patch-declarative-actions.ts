#!/usr/bin/env bun
/** One-off: rewrite selection + capability-stub action JSON to declarative kernel.call steps. */
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = "c:/git/compose/spatial/assets/modelDefinition";

const SELECTION: Record<string, { operation: string; kinds?: string[] }> = {
	"selection.selectAll": { operation: "selectAll" },
	"selection.deselectAll": { operation: "deselectAll" },
	"selection.invert": { operation: "invert" },
	"selection.selectAnchors": { operation: "selectKinds", kinds: ["anchor"] },
	"selection.selectVertices": { operation: "selectKinds", kinds: ["vertex"] },
	"selection.selectEdges": { operation: "selectKinds", kinds: ["edge"] },
	"selection.selectWires": { operation: "selectKinds", kinds: ["wire"] },
	"selection.selectFaces": { operation: "selectKinds", kinds: ["face"] },
	"selection.selectSolids": { operation: "selectKinds", kinds: ["solid"] },
	"selection.selectGeometries": { operation: "selectKinds", kinds: ["geometry"] },
	"selection.selectObjects": { operation: "selectKinds", kinds: ["object"] },
};

const seedPath = {
	kind: "path",
	root: "params",
	segments: [{ kind: "field", name: "seedTargets" }],
};

function selectionDoc(id: string, label: string, meta: { operation: string; kinds?: string[] }) {
	const args: Record<string, unknown> = {
		operation: { kind: "const", value: meta.operation },
		seedTargets: seedPath,
	};
	if (meta.kinds) args.kinds = { kind: "const", value: meta.kinds };
	return {
		schema: "spatial.action/v1",
		id,
		version: "1.0.0",
		label,
		steps: [
			{ op: "kernel.call", function: "spatial.selection.apply", args, assignTo: "result" },
			{ op: "return", result: { kind: "var", name: "result" } },
		],
	};
}

function applyDoc(id: string, label: string) {
	return {
		schema: "spatial.action/v1",
		id,
		version: "1.0.0",
		label,
		steps: [
			{
				op: "kernel.call",
				function: "spatial.selection.apply",
				args: {
					operation: { kind: "path", root: "params", segments: [{ kind: "field", name: "operation" }] },
					seedTargets: seedPath,
					kinds: { kind: "path", root: "params", segments: [{ kind: "field", name: "kinds" }] },
				},
				assignTo: "result",
			},
			{ op: "return", result: { kind: "var", name: "result" } },
		],
	};
}

function capabilityDoc(id: string, label: string) {
	return {
		schema: "spatial.action/v1",
		id,
		version: "1.0.0",
		label,
		steps: [
			{ op: "kernel.call", function: "spatial.action.capability", assignTo: "result" },
			{ op: "return", result: { kind: "var", name: "result" } },
		],
	};
}

function walk(dir: string): string[] {
	const out: string[] = [];
	for (const name of readdirSync(dir, { withFileTypes: true })) {
		const p = join(dir, name.name);
		if (name.isDirectory()) out.push(...walk(p));
		else if (name.name.endsWith(".json") && p.includes(`${join("", "action", "")}`)) out.push(p);
	}
	return out;
}

let n = 0;
for (const file of walk(root)) {
	if (!file.includes("/action/") && !file.includes("\\action\\")) continue;
	const raw = JSON.parse(readFileSync(file, "utf8")) as Record<string, unknown>;
	if (raw.schema !== "spatial.action/v1" || typeof raw.id !== "string") continue;
	const id = raw.id;
	let next: Record<string, unknown> | null = null;
	if (id === "selection.apply") next = applyDoc(id, String(raw.label ?? "SelectionApply"));
	else if (SELECTION[id]) next = selectionDoc(id, String(raw.label ?? id), SELECTION[id]!);
	else if (Array.isArray(raw.steps) && raw.steps.some((s) => (s as { function?: string }).function === "spatial.action.execute"))
		next = capabilityDoc(id, String(raw.label ?? id));
	if (!next) continue;
	writeFileSync(file, `${JSON.stringify(next, null, 2)}\n`);
	n++;
}
console.log(`[DEBUG] patched ${n} action JSON files`);
