#!/usr/bin/env bun
/** @emoji 🧩 Syncs callable `surface.construct` and `curve.construct` interactions under spatial.shape. */
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { buildConstructCurveInteractionSpec, buildConstructSurfaceInteractionSpec } from "./shape-construct-codegen.ts";

const repoRoot = join(import.meta.dir, "../..");
const interactionDir = join(repoRoot, "assets/modelDefinition/spatial.shape/interaction");

function writeJson(path: string, value: unknown): void {
	mkdirSync(dirname(path), { recursive: true });
	writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`, "utf8");
}

writeJson(join(interactionDir, "constructSurface.json"), buildConstructSurfaceInteractionSpec());
writeJson(join(interactionDir, "constructCurve.json"), buildConstructCurveInteractionSpec());
console.log("[sync] spatial.shape → surface.construct, curve.construct");
