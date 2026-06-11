#!/usr/bin/env bun
/** One-off: drop version suffixes from puzzle 5d model types and schema. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "../../../../../..");

function migrateFile(rel: string, replacer: (s: string) => string): void {
  const path = join(root, rel);
  writeFileSync(path, replacer(readFileSync(path, "utf8")));
  console.log(`[DEBUG] migrated ${rel}`);
}

migrateFile("puzzle/5d/react/index.tsx", (s) => {
  const pairs: [string, string][] = [
    ["FastenerV1", "Fastener"],
    ["GripV1", "Grip"],
    ["PartV1", "Part"],
    ["parseV1", "parseModel"],
    ['readonly schema: "puzzle.5d/v1"', 'readonly schema: "puzzle.5d"'],
    ['r.schema !== "puzzle.5d/v1"', 'r.schema !== "puzzle.5d"'],
    ['schema: "puzzle.5d/v1"', 'schema: "puzzle.5d"'],
    ['describe("parseV1"', 'describe("parseModel"'],
    ["export interface V1", "export interface Model"],
    ["compose5d(fixture2d: Puzzle2dFixtureV1, fixture3d: Puzzle3dFixtureV1): V1", "compose5d(fixture2d: Puzzle2dFixtureV1, fixture3d: Puzzle3dFixtureV1): Model"],
    ["project2d(model: V1)", "project2d(model: Model)"],
    ["project3d(model: V1)", "project3d(model: Model)"],
    ["createStore(model: V1)", "createStore(model: Model)"],
    ["cloneModel(model: V1): V1", "cloneModel(model: Model): Model"],
    ["partById(model: V1", "partById(model: Model"],
    ["peerPartForKind(model: V1", "peerPartForKind(model: Model"],
    ["volumeTemplatesForPartKind(model: V1", "volumeTemplatesForPartKind(model: Model"],
    ["synthesizeVolumeAspectFromFlat(\n  model: V1", "synthesizeVolumeAspectFromFlat(\n  model: Model"],
    ["synthesizeVolumeAspectFromBrushPayload(\n  model: V1", "synthesizeVolumeAspectFromBrushPayload(\n  model: Model"],
    ["synthesizeFlatAspectFromVolume(model: V1", "synthesizeFlatAspectFromVolume(model: Model"],
    ["applyBrushPlacementToModel(model: V1", "applyBrushPlacementToModel(model: Model"],
    ["applyFillPlacementsToModel(base: V1", "applyFillPlacementsToModel(base: Model)"],
    ["partFromPaletteNodeDrop(model: V1", "partFromPaletteNodeDrop(model: Model"],
    ["partFromPaletteObjectDrop(model: V1", "partFromPaletteObjectDrop(model: Model"],
    ["removePartFromModel(model: V1", "removePartFromModel(model: Model"],
    ["removeGripFromModel(model: V1", "removeGripFromModel(model: Model"],
    ["removeFastenerFromModel(model: V1", "removeFastenerFromModel(model: Model"],
    ["applyStructuralDelete2dToModel(model: V1", "applyStructuralDelete2dToModel(model: Model"],
    ["puzzle5dModelStructureEpoch(model: V1)", "puzzle5dModelStructureEpoch(model: Model)"],
    ["readonly baseModel: V1", "readonly baseModel: Model"],
    ["readonly model: V1", "readonly model: Model"],
    ["| { readonly kind: \"placed\"; readonly model: V1", "| { readonly kind: \"placed\"; readonly model: Model"],
    [": V1 | null", ": Model | null"],
    [": V1):", ": Model):"],
    ["JSON.parse(JSON.stringify(model)) as V1", "JSON.parse(JSON.stringify(model)) as Model"],
    ["pruneSelectionAfterModelEdit(selection: SelectionSnapshot, _prevModel: V1, nextModel: V1)", "pruneSelectionAfterModelEdit(selection: SelectionSnapshot, _prevModel: Model, nextModel: Model)"],
    ["read(): V1", "read(): Model"],
    ["replaceModel(model: V1)", "replaceModel(model: Model)"],
    ["{@link V1}", "{@link Model}"],
  ];
  for (const [from, to] of pairs) s = s.split(from).join(to);
  return s;
});

for (const rel of ["puzzle/5d/fixture/nakagin-capsule-tower.5d.json", "puzzle/5d/fixture/concrete-forest.5d.json"]) {
  migrateFile(rel, (s) => s.replace('"schema": "puzzle.5d/v1"', '"schema": "puzzle.5d"'));
}
