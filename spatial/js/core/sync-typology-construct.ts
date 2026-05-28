#!/usr/bin/env bun
/** @emoji 🏗️ Writes construct create actions + interactions for typologies missing them. */
import { mkdirSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import {
	buildTypologyConstructInteractionSpec,
	capabilityActionSpecJson,
	typologyConstructAssetIds,
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

const WALL_EXTRA_ACTIONS = [
	"building.wall.constructVerticalConcreteWall",
	"building.wall.constructWallFromBottomAndTop",
	"building.wall.constructWallFromHorizontalPathAndProfile",
	"building.wall.constructWallFromHorizontalPathAndProfiles",
] as const;

const COLUMN_EXTRA_ACTIONS = [
	"building.mushroomcolumn.constructMushroomColumn",
	"building.mushroomcolumn.constructExtrudedMushroomColumn",
	"building.mushroomcolumn.constructFullyQuadraticMushroomColumn",
	"building.mushroomcolumn.constructRectangularMushroomColumnWithQuadraticSlab",
] as const;

function writeJson(path: string, value: unknown): void {
	mkdirSync(dirname(path), { recursive: true });
	writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`, "utf8");
}

function extraActionsForTypology(typology: string): readonly string[] {
	if (typology.includes("wall")) return WALL_EXTRA_ACTIONS;
	if (typology.includes("column")) return COLUMN_EXTRA_ACTIONS;
	return [];
}

let updated = 0;
for (const typologyPath of walkTypologyJsonFiles(assetsRoot)) {
	const raw = JSON.parse(readFileSync(typologyPath, "utf8")) as unknown;
	const spec = parseTypologySpec(raw);
	if (!spec) continue;
	if (spec.actions.length > 0 && spec.interactions.length > 0) continue;
	const rel = typologyPath.slice(assetsRoot.length + 1).replace(/\\/g, "/");
	const ownerFolder = rel.split("/typology/")[0];
	if (!ownerFolder) continue;
	const modelDefinitionRoot = join(assetsRoot, ownerFolder);
	const ids = typologyConstructAssetIds(spec.id, spec.label);
	const extras = extraActionsForTypology(spec.id);
	const actionDir = join(modelDefinitionRoot, "action");
	const interactionDir = join(modelDefinitionRoot, "interaction");
	writeJson(
		join(actionDir, `${ids.createFrom2PointsAndHeight.split(".").pop()}.json`),
		capabilityActionSpecJson(ids.createFrom2PointsAndHeight, `Create ${spec.label} From 2 Points And Height`),
	);
	writeJson(
		join(actionDir, `${ids.createFromCurveAndHeight.split(".").pop()}.json`),
		capabilityActionSpecJson(ids.createFromCurveAndHeight, `Create ${spec.label} From Curve And Height`),
	);
	writeJson(
		join(actionDir, `${ids.createFromSurface.split(".").pop()}.json`),
		capabilityActionSpecJson(ids.createFromSurface, `Create ${spec.label} From Surface`),
	);
	writeJson(
		join(actionDir, `${ids.construct.split(".").pop()}.json`),
		capabilityActionSpecJson(ids.construct, `Construct ${spec.label}`),
	);
	writeJson(
		join(interactionDir, `${ids.construct.split(".").pop()}.json`),
		buildTypologyConstructInteractionSpec(spec.id, spec.label, ids.construct),
	);
	const nextTypology = {
		...(raw as Record<string, unknown>),
		actions: [...extras, ids.createFrom2PointsAndHeight, ids.createFromCurveAndHeight, ids.createFromSurface, ids.construct],
		interactions: [ids.construct],
	};
	writeJson(typologyPath.replace(/\\/g, "/"), nextTypology);
	console.log(`[sync] ${spec.id} (${ownerFolder})`);
	updated += 1;
}
console.log(`[sync] updated ${updated} typolog${updated === 1 ? "y" : "ies"}`);
