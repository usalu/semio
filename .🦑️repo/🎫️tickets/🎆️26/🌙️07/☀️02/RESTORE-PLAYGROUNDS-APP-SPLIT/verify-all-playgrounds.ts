#!/usr/bin/env bun
/** 🧪️ Loads every playground definition and optionally probes createRuntime. */

const APPS: ReadonlyArray<{ readonly entry: string; readonly pkg: string; readonly exportName: string }> = [
  { entry: "2d", pkg: "@semio-tech/puzzle-2d-core", exportName: "puzzle2dPlayAppDefinition" },
  { entry: "3d", pkg: "@semio-tech/puzzle-3d-core", exportName: "puzzle3dPlayAppDefinition" },
  { entry: "5d", pkg: "@semio-tech/puzzle-5d-core", exportName: "puzzle5dPlayAppDefinition" },
  { entry: "flow", pkg: "@semio-tech/flow-core", exportName: "flowPlayAppDefinition" },
  { entry: "dag", pkg: "@semio-tech/dag-host-core", exportName: "dagPlayAppDefinition" },
  { entry: "imperative", pkg: "@semio-tech/imperative-core", exportName: "imperativePlayAppDefinition" },
  { entry: "sequence", pkg: "@semio-tech/sequence-core", exportName: "sequencePlayAppDefinition" },
  { entry: "layout", pkg: "@semio-tech/layout-core", exportName: "layoutPlayAppDefinition" },
  { entry: "lowpoly", pkg: "@semio-tech/lowpoly-core", exportName: "lowpolyPlayAppDefinition" },
  { entry: "procedural-2d", pkg: "@semio-tech/procedural-2d-core", exportName: "procedural2dPlayAppDefinition" },
  { entry: "procedural-3d", pkg: "@semio-tech/procedural-3d-core", exportName: "procedural3dPlayAppDefinition" },
  { entry: "shooting", pkg: "@semio-tech/shooting-core", exportName: "shootingPlayAppDefinition" },
  { entry: "forms", pkg: "@semio-tech/forms-core", exportName: "formsPlayAppDefinition" },
  { entry: "raster", pkg: "@semio-tech/raster-core", exportName: "rasterPlayAppDefinition" },
  { entry: "draw", pkg: "@semio-tech/draw-core", exportName: "drawPlayAppDefinition" },
  { entry: "writer", pkg: "@semio-tech/writer-core", exportName: "writerPlayAppDefinition" },
  { entry: "s", pkg: "@semio-tech/s-core", exportName: "sPlayAppDefinition" },
  { entry: "vcs", pkg: "@semio-tech/vcs-core", exportName: "vcsPlayAppDefinition" },
  { entry: "gis-2d", pkg: "@semio-tech/gis-2d-core", exportName: "gis2dPlayAppDefinition" },
  { entry: "wires", pkg: "@semio-tech/reasoning-mindmap-wires-core", exportName: "wiresPlayAppDefinition" },
  { entry: "trinity-jack", pkg: "@semio-tech/trinity-jack-host-core", exportName: "trinityJackPlayAppDefinition" },
  { entry: "trinity-rewrite", pkg: "@semio-tech/trinity-rewrite-core", exportName: "trinityRewritePlayAppDefinition" },
  { entry: "presentation", pkg: "@semio-tech/framework-presentation-core", exportName: "presentationPlayAppDefinition" },
  { entry: "cad", pkg: "@semio-tech/cad-js-renderer-core", exportName: "cadPlayAppDefinition" },
];

const probeRuntime = process.argv.includes("--runtime");

let failed = 0;
for (const row of APPS) {
  try {
    const mod = (await import(row.pkg)) as Record<string, unknown>;
    const app = mod[row.exportName] as { id: string; devHost?: { playEntryKind?: string }; createPlayground: () => { runtime: unknown } };
    if (!app?.id) throw new Error(`missing ${row.exportName}`);
    if (app.devHost?.playEntryKind !== row.entry) {
      throw new Error(`playEntryKind ${app.devHost?.playEntryKind} !== ${row.entry}`);
    }
    let runtimeNote = "";
    if (probeRuntime) {
      try {
        const pg = app.createPlayground();
        const apps = (pg.runtime as { apps?: unknown[] })?.apps?.length;
        runtimeNote = ` runtime.apps=${apps ?? "?"}`;
      } catch (runtimeError) {
        runtimeNote = ` runtime: ${runtimeError instanceof Error ? runtimeError.message : String(runtimeError)}`;
      }
    }
    console.log(`OK ${row.entry} id=${app.id}${runtimeNote}`);
  } catch (error) {
    failed += 1;
    const message = error instanceof Error ? error.message : String(error);
    console.log(`FAIL ${row.entry}: ${message}`);
  }
}
process.exit(failed > 0 ? 1 : 0);
