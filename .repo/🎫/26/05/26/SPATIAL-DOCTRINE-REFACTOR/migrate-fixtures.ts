/** @emoji 🧳 Migrate spatial.topology/v1 fixtures to spatial.model/v1. */
import { readFileSync, writeFileSync, renameSync, existsSync } from "node:fs";
import { join } from "node:path";

const fixtures = "c:/git/compose/spatial/fixtures";

function migrateTopologyBlock(raw: Record<string, unknown>): Record<string, unknown> {
	const cells = Array.isArray(raw.cells) ? (raw.cells as { id: string }[]) : [];
	const objects = cells.map((c) => ({
		id: `object-${c.id}`,
		typologyId: "builtin.primitive.box",
		geometryRef: c.id,
	}));
	const geometry: Record<string, unknown> = {};
	for (const k of ["anchors", "vertices", "edges", "wires", "faces", "shells", "cells", "cellComplexes", "clusters"] as const) {
		geometry[k] = Array.isArray(raw[k]) ? raw[k] : [];
	}
	return {
		schema: "spatial.model/v1",
		revision: typeof raw.revision === "number" ? raw.revision : 0,
		objects,
		geometry,
	};
}

function migrateFile(path: string): void {
	const text = readFileSync(path, "utf8");
	const doc = JSON.parse(text) as Record<string, unknown>;
	if (doc.schema === "spatial.topology/v1") {
		writeFileSync(path, JSON.stringify(migrateTopologyBlock(doc), null, 2) + "\n");
		return;
	}
	if (doc.raw && typeof doc.raw === "object") {
		const raw = doc.raw as Record<string, unknown>;
		if (raw.schema === "spatial.topology/v1") doc.raw = migrateTopologyBlock(raw);
	}
	if (doc.analytic && typeof doc.analytic === "object") {
		const a = doc.analytic as Record<string, unknown>;
		if (a.schema === "spatial.topology/v1") doc.analytic = migrateTopologyBlock(a);
	}
	writeFileSync(path, JSON.stringify(doc, null, 2) + "\n");
}

for (const name of ["small-building", "tall-building", "large-building"]) {
	const oldPath = join(fixtures, `${name}.topology.json`);
	const newPath = join(fixtures, `${name}.model.json`);
	if (existsSync(oldPath)) {
		migrateFile(oldPath);
		renameSync(oldPath, newPath);
	}
}

for (const name of ["simple.spatial.json", "spatial.spatial.json"]) {
	migrateFile(join(fixtures, name));
}
