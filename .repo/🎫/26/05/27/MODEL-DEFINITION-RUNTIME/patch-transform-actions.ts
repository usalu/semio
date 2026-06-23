#!/usr/bin/env bun
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const base = "c:/git/compose/spatial/assets/modelDefinition/geometry/typology/transform";
const transforms = [
	{ dir: "move", id: "transform.move", label: "Move", typologyId: "builtin.transform.move" },
	{ dir: "copy", id: "transform.copy", label: "Copy", typologyId: "builtin.transform.copy" },
	{ dir: "rotate", id: "transform.rotate", label: "Rotate", typologyId: "builtin.transform.rotate" },
	{ dir: "mirror", id: "transform.mirror", label: "Mirror", typologyId: "builtin.transform.mirror" },
	{ dir: "scale1d", id: "transform.scale1d", label: "Scale1d", typologyId: "builtin.transform.scale1d" },
	{ dir: "scale3d", id: "transform.scale3d", label: "Scale3d", typologyId: "builtin.transform.scale3d" },
];

const capabilityDoc = (id: string, label: string) =>
	JSON.stringify(
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
	) + "\n";

for (const row of transforms) {
	const actionDir = join(base, row.dir, "action");
	mkdirSync(actionDir, { recursive: true });
	const fileName = row.id.split(".").pop()! + ".json";
	writeFileSync(join(actionDir, fileName), capabilityDoc(row.id, row.label));
	const typologyPath = join(base, row.dir, "typology.json");
	const typology = JSON.parse(readFileSync(typologyPath, "utf8")) as Record<string, unknown>;
	typology.actions = [row.id];
	writeFileSync(typologyPath, `${JSON.stringify(typology, null, 2)}\n`);
}
console.log(`[DEBUG] wrote ${transforms.length} transform action JSON files`);
