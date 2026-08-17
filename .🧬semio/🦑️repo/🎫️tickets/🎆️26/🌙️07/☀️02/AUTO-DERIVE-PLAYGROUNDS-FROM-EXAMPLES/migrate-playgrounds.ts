#!/usr/bin/env bun
/** @emoji 🛝️ Bulk rename: fixture dirs → example, playground symbol renames in core files. */

import { existsSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const repoRoot = resolve(import.meta.dir, "../../../../../../");

const APP_CORE_DIRS = [
  "draw/core",
  "note/core",
  "writer/core",
  "forms/core",
  "s/core",
  "layout/core",
  "shooting/core",
  "procedural/2d/core",
  "procedural/3d/core",
  "gis/2d/core",
  "raster/core",
  "mathematical/graph/port/directed/dag/core",
  "reasoning/mindmap/wires/core",
  "puzzle/2d/core",
  "puzzle/3d/core",
  "puzzle/5d/core",
  "framework/product/presentation/core",
  "trinity/rewrite/core",
  "trinity/jack/host-core",
  "cad/js/renderer/core",
  "flow/core",
  "imperative/core",
  "sequence/core",
  "lowpoly/core",
  "vcs/core",
] as const;

const EXAMPLE_DIR_PARENTS = [
  "draw",
  "note",
  "writer",
  "forms",
  "s",
  "layout",
  "shooting",
  "procedural/2d",
  "procedural/3d",
  "gis/2d",
  "raster",
  "mathematical/graph/port/directed/dag",
  "reasoning/mindmap/wires",
  "puzzle/2d",
  "puzzle/3d",
  "puzzle/5d",
  "trinity",
] as const;

const REPLACEMENTS: readonly [string, string][] = [
  ["PlaygroundFixtureHost", "PlaygroundExampleHost"],
  ["PlaygroundFixtureCatalog", "PlaygroundExampleCatalog"],
  ["PlaygroundFixtureOption", "PlaygroundExampleOption"],
  ["getFixtureCatalog", "getExampleCatalog"],
  ["activeFixtureId", "activeExampleId"],
  ["setActiveFixture", "setActiveExample"],
  ["PlayFixtureHostConfig", "PlayExampleHostConfig"],
  ["fixtureHost", "exampleHost"],
  ["eagerPlayFixtureGlob", "eagerPlayExampleGlob"],
  ["playgroundResolvedFixtureId", "playgroundResolvedExampleId"],
  ["isPlaygroundFixtureLocked", "isPlaygroundExampleLocked"],
  ["isPlaygroundNoFixtureId", "isPlaygroundNoExampleId"],
  ["PLAYGROUND_NO_FIXTURE_ID", "PLAYGROUND_NO_EXAMPLE_ID"],
  ["PLAYGROUND_NO_FIXTURE_OPTION", "PLAYGROUND_NO_EXAMPLE_OPTION"],
  ["playgroundFixtureCatalogWithNoOption", "playgroundExampleCatalogWithNoOption"],
  ["resolvePlaygroundFixtureCatalog", "resolvePlaygroundExampleCatalog"],
  ["loadPlaygroundFixtureCatalog", "loadPlaygroundExampleCatalog"],
  ["PLAY_FIXTURE_", "PLAY_EXAMPLE_"],
  ["_PLAY_FIXTURE_", "_PLAY_EXAMPLE_"],
  ["_PLAY_FILE_FIXTURE_", "_PLAY_FILE_EXAMPLE_"],
  ["fixture-slugs.ts", "example-slugs.ts"],
  ["../fixture/", "../example/"],
  ["../../draw/fixture/", "../../draw/example/"],
  ["../../writer/fixture/", "../../writer/example/"],
  ["../../note/fixture/", "../../note/example/"],
  ["../../fixture/", "../../example/"],
  ["args.fixtureId", "args.exampleId"],
  ["{ fixtureId }", "{ exampleId }"],
  ["{ fixtureId:", "{ exampleId:"],
  ['"./playground"', '"."'],
];

function applyReplacements(content: string): string {
  let out = content;
  for (const [from, to] of REPLACEMENTS) out = out.split(from).join(to);
  return out;
}

for (const parent of EXAMPLE_DIR_PARENTS) {
  const fixtureDir = join(repoRoot, parent, "fixture");
  const exampleDir = join(repoRoot, parent, "example");
  if (existsSync(fixtureDir) && !existsSync(exampleDir)) renameSync(fixtureDir, exampleDir);
}

for (const coreDir of APP_CORE_DIRS) {
  const slugPath = join(repoRoot, coreDir, "fixture-slugs.ts");
  const examplePath = join(repoRoot, coreDir, "example-slugs.ts");
  if (existsSync(slugPath) && !existsSync(examplePath)) renameSync(slugPath, examplePath);
}

for (const coreDir of APP_CORE_DIRS) {
  for (const file of ["index.ts", "playground.ts", "internal.ts", "example-slugs.ts"]) {
    const filePath = join(repoRoot, coreDir, file);
    if (!existsSync(filePath)) continue;
    const raw = readFileSync(filePath, "utf8");
    const next = applyReplacements(raw);
    if (next !== raw) writeFileSync(filePath, next);
  }
  const pkgPath = join(repoRoot, coreDir, "package.json");
  if (existsSync(pkgPath)) {
    const raw = readFileSync(pkgPath, "utf8");
    const next = raw.replace(/\s*,?\s*"\.\/playground":\s*"\.\/playground\.ts"/g, "");
    if (next !== raw) writeFileSync(pkgPath, next);
  }
}

console.log("[DEBUG] rename pass complete");
