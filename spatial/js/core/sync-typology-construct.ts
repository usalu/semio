#!/usr/bin/env bun
/** @emoji 🏗️ Syncs strict typology construct kits: mode `construct*` actions + one construct interaction. */
import { existsSync, mkdirSync, readdirSync, readFileSync, statSync, unlinkSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import {
	buildTypologyConstructInteractionSpec,
	capabilityActionSpecJson,
	legacyTypologyConstructActionBasenames,
	typologyConstructAssetIds,
	typologyConstructIsSurfacePrimary,
	typologyConstructModeActionIds,
} from "./typology-construct-codegen.ts";

function parseTypologySpec(raw: unknown): { id: string; label: string; actions: string[]; interactions: string[] } | null {
	if (!raw || typeof raw !== "object") return null;
	const r = raw as Record<string, unknown>;
	if (r.schema !== "spatial.typology/v1" || typeof r.id !== "string" || typeof r.label !== "string") return null;
	if (!Array.isArray(r.actions) || !Array.isArray(r.interactions)) return null;
	return { id: r.id, label: r.label, actions: r.actions as string[], interactions: r.interactions as string[] };
}

const repoRoot = join(import.meta.dir, "../..");
const assetsRoot = join(repoRoot, "assets/modelDefinition");

function walkTypologyJsonFiles(dir: string, out: string[] = []): string[] {
	for (const name of readdirSync(dir)) {
		const path = join(dir, name);
		if (statSync(path).isDirectory()) walkTypologyJsonFiles(path, out);
		else if (name === "typology.json") out.push(path);
	}
	return out;
}

function writeJson(path: string, value: unknown): void {
	mkdirSync(dirname(path), { recursive: true });
	writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`, "utf8");
}

function removeLegacyActions(actionDir: string, label: string, interactionId: string): void {
	for (const name of legacyTypologyConstructActionBasenames(label, interactionId)) {
		const path = join(actionDir, name);
		if (existsSync(path)) unlinkSync(path);
	}
}

function removeModeActionIfPresent(actionDir: string, actionId: string): void {
	const path = join(actionDir, `${actionId.split(".").pop()}.json`);
	if (existsSync(path)) unlinkSync(path);
}

function writeModeActions(actionDir: string, typologyId: string, label: string): void {
	const ids = typologyConstructAssetIds(typologyId, label);
	const surfacePrimary = typologyConstructIsSurfacePrimary(typologyId);
	const modeActions: { id: string; title: string }[] = surfacePrimary
		? [{ id: ids.constructFromSurface, title: `Construct ${label} From Surface` }]
		: [
				{ id: ids.constructFrom2PointsAndHeight, title: `Construct ${label} From 2 Points And Height` },
				{ id: ids.constructFromCurveAndHeight, title: `Construct ${label} From Curve And Height` },
				{ id: ids.constructFromSurface, title: `Construct ${label} From Surface` },
			];
	if (surfacePrimary) {
		removeModeActionIfPresent(actionDir, ids.constructFrom2PointsAndHeight);
		removeModeActionIfPresent(actionDir, ids.constructFromCurveAndHeight);
	}
	for (const row of modeActions) {
		writeJson(join(actionDir, `${row.id.split(".").pop()}.json`), capabilityActionSpecJson(row.id, row.title));
	}
}

let synced = 0;
for (const typologyPath of walkTypologyJsonFiles(assetsRoot)) {
	const raw = JSON.parse(readFileSync(typologyPath, "utf8")) as unknown;
	const spec = parseTypologySpec(raw);
	if (!spec) continue;
	const rel = typologyPath.slice(assetsRoot.length + 1).replace(/\\/g, "/");
	const ownerFolder = rel.split("/typology/")[0];
	if (!ownerFolder) continue;
	const ids = typologyConstructAssetIds(spec.id, spec.label);
	const hasKit = spec.interactions.includes(ids.interaction);
	if (!hasKit && spec.actions.length > 0 && spec.interactions.length > 0) continue;

	const modelDefinitionRoot = join(assetsRoot, ownerFolder);
	const actionDir = join(modelDefinitionRoot, "action");
	const interactionDir = join(modelDefinitionRoot, "interaction");
	const interactionPath = join(interactionDir, `${ids.interaction.split(".").pop()}.json`);

	removeLegacyActions(actionDir, spec.label, ids.interaction);
	writeModeActions(actionDir, spec.id, spec.label);
	writeJson(interactionPath, buildTypologyConstructInteractionSpec(spec.id, spec.label, ids.interaction));

	const modeActions = typologyConstructModeActionIds(spec.id, spec.label);
	const nextTypology = {
		...(raw as Record<string, unknown>),
		actions: modeActions,
		interactions: [ids.interaction],
	};
	writeJson(typologyPath.replace(/\\/g, "/"), nextTypology);
	console.log(`[sync] ${spec.id} (${ownerFolder}) actions=[${modeActions.join(", ")}]`);
	synced += 1;
}
console.log(`[sync] strict construct kit on ${synced} typolog${synced === 1 ? "y" : "ies"}`);
