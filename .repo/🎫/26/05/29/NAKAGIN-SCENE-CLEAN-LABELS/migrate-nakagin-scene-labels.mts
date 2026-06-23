#!/usr/bin/env bun
/** @emoji 🧾 One-off: bake human-readable labels and clean kind ids into nakagin scene fixture (no runtime compose links). */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "..", "..", "..", "..", "..", "..");
const boardPath = join(repoRoot, "elements/lib/board/play/fixtures/nakagin-capsule-tower.board.json");
const scenePath = join(repoRoot, "elements/lib/react/scene/play/fixtures/nakagin-capsule-tower.scene.json");

type CatalogRow = { id: string; label?: string; name?: string; [k: string]: unknown };
type SceneVortex = { id: string; vortexKind?: string; label?: string; [k: string]: unknown };
type SceneObject = {
	id: string;
	label?: string;
	objectKind?: string;
	vortices?: SceneVortex[];
	[k: string]: unknown;
};

function catalogText(row: CatalogRow): string {
	const label = typeof row.label === "string" ? row.label.trim() : "";
	const name = typeof row.name === "string" ? row.name.trim() : "";
	if (label && !label.startsWith("compose.")) return label;
	if (name && !name.startsWith("compose.")) return name;
	return "";
}

function cleanKindId(row: CatalogRow): string {
	return catalogText(row) || row.id;
}

function remapCatalog(rows: CatalogRow[] | undefined): { rows: CatalogRow[]; idMap: Map<string, string> } {
	const idMap = new Map<string, string>();
	if (!rows?.length) return { rows: [], idMap };
	const rowsOut = rows.map((row) => {
		const nextId = cleanKindId(row);
		idMap.set(row.id, nextId);
		const text = catalogText(row) || nextId;
		return { ...row, id: nextId, label: text, name: text };
	});
	return { rows: rowsOut, idMap };
}

function remapKindRef(kind: string | undefined, idMap: Map<string, string>): string | undefined {
	if (!kind) return undefined;
	return idMap.get(kind) ?? kind;
}

function vortexPortSuffix(vortexId: string, objectId: string): string {
	const prefix = `${objectId}:`;
	if (!vortexId.startsWith(prefix)) {
		const i = vortexId.indexOf(":");
		return i >= 0 ? vortexId.slice(i + 1) : vortexId;
	}
	return vortexId.slice(prefix.length);
}

function vortexDisplayLabel(
	vortex: SceneVortex,
	objectId: string,
	handleLabelByKind: Map<string, string>,
): string {
	const kind = vortex.vortexKind?.trim() ?? "";
	const handleLabel = kind ? (handleLabelByKind.get(kind) ?? kind) : "";
	const port = vortexPortSuffix(vortex.id, objectId);
	if (port && port !== "link") {
		return handleLabel ? `${handleLabel} (${port})` : port;
	}
	return handleLabel || "link";
}

function main(): void {
	const board = JSON.parse(readFileSync(boardPath, "utf8")) as { nodes: { id: string; text?: string }[] };
	const scene = JSON.parse(readFileSync(scenePath, "utf8")) as {
		meta?: {
			kindCatalogs?: { handles?: CatalogRow[]; nodes?: CatalogRow[]; wires?: CatalogRow[]; edges?: CatalogRow[] };
			kindCompatibility?: { source: string; target: string; [k: string]: unknown }[];
		};
		objects: SceneObject[];
	};

	const boardTextById = new Map(board.nodes.map((n) => [n.id, typeof n.text === "string" ? n.text.trim() : ""]));

	const handles = remapCatalog(scene.meta?.kindCatalogs?.handles);
	const nodes = remapCatalog(scene.meta?.kindCatalogs?.nodes);
	const wires = remapCatalog(scene.meta?.kindCatalogs?.wires);
	const edges = remapCatalog(scene.meta?.kindCatalogs?.edges);
	const idMap = new Map<string, string>([...handles.idMap, ...nodes.idMap, ...wires.idMap, ...edges.idMap]);

	const handleLabelByKind = new Map(handles.rows.map((h) => [h.id, catalogText(h) || h.id]));
	const nodeLabelByKind = new Map(nodes.rows.map((n) => [n.id, catalogText(n) || n.id]));

	let vortexLabels = 0;
	let objectLabels = 0;

	const objects = scene.objects.map((obj) => {
		const boardText = boardTextById.get(obj.id) ?? "";
		const typeLabel = obj.objectKind ? (nodeLabelByKind.get(remapKindRef(obj.objectKind, idMap) ?? "") ?? "") : "";
		const pieceLabel = boardText || obj.label?.trim() || "";
		const label =
			pieceLabel && typeLabel && pieceLabel !== typeLabel ? `${typeLabel} · ${pieceLabel}` : pieceLabel || typeLabel || obj.id;

		if (label !== obj.id) objectLabels += 1;

		const vortices = (obj.vortices ?? []).map((v) => {
			const vortexKind = remapKindRef(v.vortexKind, idMap);
			const vLabel = vortexDisplayLabel({ ...v, vortexKind }, obj.id, handleLabelByKind);
			vortexLabels += 1;
			return {
				...v,
				...(vortexKind ? { vortexKind } : {}),
				label: vLabel,
			};
		});

		return {
			...obj,
			label,
			...(obj.objectKind ? { objectKind: remapKindRef(obj.objectKind, idMap) } : {}),
			vortices,
		};
	});

	const kindCompatibility = (scene.meta?.kindCompatibility ?? []).map((rule) => ({
		...rule,
		source: remapKindRef(rule.source, idMap) ?? rule.source,
		target: remapKindRef(rule.target, idMap) ?? rule.target,
	}));

	const next = {
		...scene,
		meta: {
			...scene.meta,
			kindCatalogs: {
				handles: handles.rows,
				nodes: nodes.rows,
				...(wires.rows.length ? { wires: wires.rows } : {}),
				...(edges.rows.length ? { edges: edges.rows } : {}),
			},
			kindCompatibility,
		},
		objects,
	};

	writeFileSync(scenePath, `${JSON.stringify(next, null, 2)}\n`, "utf8");
	console.log(
		`[migrate-nakagin-scene-labels] wrote ${scenePath} (${objectLabels} object labels, ${vortexLabels} vortex labels, ${idMap.size} kind ids cleaned)`,
	);
}

main();
