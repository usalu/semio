#!/usr/bin/env bun
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { join, dirname } from "node:path";

const REPO = "/Users/ueli/Documents/semio";

const TARGETS: Array<{ path: string; pkg: string; bootFns: string[] }> = [
  { path: "puzzle/2d/react/index.tsx", pkg: "@semio-tech/puzzle-2d-react", bootFns: ["boot2dPlay", "bootWiresPlay"] },
  { path: "puzzle/3d/react/index.tsx", pkg: "@semio-tech/puzzle-3d-react", bootFns: ["bootPuzzle3dPlay"] },
  { path: "puzzle/5d/react/index.tsx", pkg: "@semio-tech/puzzle-5d-react", bootFns: ["boot5dPlay"] },
  { path: "gis/2d/react/index.tsx", pkg: "@semio-tech/gis-2d-react", bootFns: ["bootMapPlay"] },
  { path: "flow/react/index.tsx", pkg: "@semio-tech/flow-react", bootFns: ["bootFlowPlay"] },
  { path: "mathematical/graph/port/directed/dag/react/index.tsx", pkg: "@semio-tech/dag-react", bootFns: ["bootDagPlay"] },
  { path: "imperative/react/index.tsx", pkg: "@semio-tech/imperative-react", bootFns: ["bootImperativePlay"] },
  { path: "sequence/react/index.tsx", pkg: "@semio-tech/sequence-react", bootFns: ["bootSequencePlay"] },
  { path: "layout/react/index.tsx", pkg: "@semio-tech/layout-react", bootFns: ["bootLayoutPlay"] },
  { path: "lowpoly/react/index.tsx", pkg: "@semio-tech/lowpoly-react", bootFns: ["bootLowpolyPlay"] },
  { path: "trinity/react/index.tsx", pkg: "@semio-tech/trinity-react", bootFns: ["bootTrinityRewritePlay", "bootTrinityJackPlay"] },
  { path: "procedural/3d/react/index.tsx", pkg: "@semio-tech/procedural-3d-react", bootFns: ["bootProceduralPlay"] },
  { path: "procedural/2d/react/index.tsx", pkg: "@semio-tech/procedural-2d-react", bootFns: ["bootProcedural2dPlay"] },
  { path: "shooting/react/index.tsx", pkg: "@semio-tech/shooting-react", bootFns: ["bootShootingPlay"] },
  { path: "forms/react/index.tsx", pkg: "@semio-tech/forms-react", bootFns: ["bootFormsPlay"] },
  { path: "raster/react/index.tsx", pkg: "@semio-tech/raster-react", bootFns: ["bootRasterPlay"] },
  { path: "draw/react/index.tsx", pkg: "@semio-tech/draw-react", bootFns: ["bootDrawPlay"] },
  { path: "note/react/index.tsx", pkg: "@semio-tech/note-react", bootFns: ["bootNotePlay"] },
  { path: "cad/renderer/react/index.tsx", pkg: "@semio-tech/cad-js-renderer-react", bootFns: ["bootCadPlay"] },
  { path: "vcs/react/index.tsx", pkg: "@semio-tech/vcs-react", bootFns: ["bootVcsPlay"] },
  { path: "writer/react/index.tsx", pkg: "@semio-tech/writer-react", bootFns: ["bootWriterPlay"] },
  { path: "framework/product/presentation/renderer/react/index.tsx", pkg: "@semio-tech/framework-presentation-renderer-react", bootFns: ["bootPresentationPlay"] },
  { path: "s/react/index.tsx", pkg: "@semio-tech/s-react", bootFns: ["bootSPlay"] },
];

const CORE_BOOT_UPDATES: Array<{ corePath: string; reactPkg: string; bootFn: string }> = [
  { corePath: "forms/core/js/index.ts", reactPkg: "@semio-tech/forms-react", bootFn: "bootFormsPlay" },
  { corePath: "lowpoly/core/js/index.ts", reactPkg: "@semio-tech/lowpoly-react", bootFn: "bootLowpolyPlay" },
  { corePath: "flow/core/js/index.ts", reactPkg: "@semio-tech/flow-react", bootFn: "bootFlowPlay" },
  { corePath: "s/core/js/index.ts", reactPkg: "@semio-tech/s-react", bootFn: "bootSPlay" },
  { corePath: "draw/core/js/index.ts", reactPkg: "@semio-tech/draw-react", bootFn: "bootDrawPlay" },
  { corePath: "note/core/js/index.ts", reactPkg: "@semio-tech/note-react", bootFn: "bootNotePlay" },
  { corePath: "puzzle/5d/core/js/index.ts", reactPkg: "@semio-tech/puzzle-5d-react", bootFn: "boot5dPlay" },
  { corePath: "puzzle/2d/core/js/index.ts", reactPkg: "@semio-tech/puzzle-2d-react", bootFn: "boot2dPlay" },
  { corePath: "writer/core/js/index.ts", reactPkg: "@semio-tech/writer-react", bootFn: "bootWriterPlay" },
  { corePath: "procedural/3d/core/js/index.ts", reactPkg: "@semio-tech/procedural-3d-react", bootFn: "bootProceduralPlay" },
  { corePath: "procedural/2d/core/js/index.ts", reactPkg: "@semio-tech/procedural-2d-react", bootFn: "bootProcedural2dPlay" },
  { corePath: "mathematical/graph/port/directed/dag/core/js/index.ts", reactPkg: "@semio-tech/dag-react", bootFn: "bootDagPlay" },
  { corePath: "sequence/core/js/index.ts", reactPkg: "@semio-tech/sequence-react", bootFn: "bootSequencePlay" },
  { corePath: "layout/core/js/index.ts", reactPkg: "@semio-tech/layout-react", bootFn: "bootLayoutPlay" },
  { corePath: "cad/renderer/core/js/index.ts", reactPkg: "@semio-tech/cad-js-renderer-react", bootFn: "bootCadPlay" },
  { corePath: "reasoning/mindmap/wires/core/js/index.ts", reactPkg: "@semio-tech/puzzle-2d-react", bootFn: "bootWiresPlay" },
  { corePath: "puzzle/3d/core/js/index.ts", reactPkg: "@semio-tech/puzzle-3d-react", bootFn: "bootPuzzle3dPlay" },
  { corePath: "gis/2d/core/js/index.ts", reactPkg: "@semio-tech/gis-2d-react", bootFn: "bootMapPlay" },
  { corePath: "shooting/core/js/index.ts", reactPkg: "@semio-tech/shooting-react", bootFn: "bootShootingPlay" },
  { corePath: "raster/core/js/index.ts", reactPkg: "@semio-tech/raster-react", bootFn: "bootRasterPlay" },
  { corePath: "vcs/core/js/index.ts", reactPkg: "@semio-tech/vcs-react", bootFn: "bootVcsPlay" },
  { corePath: "framework/product/presentation/core/js/index.ts", reactPkg: "@semio-tech/framework-presentation-renderer-react", bootFn: "bootPresentationPlay" },
  { corePath: "trinity/rewrite/core/js/index.ts", reactPkg: "@semio-tech/trinity-react", bootFn: "bootTrinityRewritePlay" },
  { corePath: "trinity/jack/host-core/js/index.ts", reactPkg: "@semio-tech/trinity-react", bootFn: "bootTrinityJackPlay" },
  { corePath: "imperative/core/js/index.ts", reactPkg: "@semio-tech/imperative-react", bootFn: "bootImperativePlay" },
];

function extractPlayHostRegion(content: string): { before: string; region: string; after: string; regionName: string } | null {
  const marker = content.match(/\/\/#region 🔖️\w+PlayHost/);
  if (!marker) return null;
  const startIdx = content.lastIndexOf(marker[0]);
  const regionName = marker[0].replace("//#region 🔖️", "");
  const endMarker = `//#endregion 🔖️${regionName}`;
  const endIdx = content.indexOf(endMarker, startIdx);
  if (endIdx < 0) return null;
  return {
    before: content.slice(0, startIdx).replace(/\n{3,}$/, "\n\n"),
    region: content.slice(startIdx, endIdx + endMarker.length),
    after: content.slice(endIdx + endMarker.length).replace(/^\n+/, "\n"),
    regionName,
  };
}

function playHostHeader(regionName: string): string {
  return `// #region 🧲️Header
/** @emoji 🛝️ Playground play host for ${regionName.replace(/PlayHost$/, "")} — loaded only via \`./play\` subpath. */
// #endregion 🧲️Header

`;
}

for (const { path, pkg } of TARGETS) {
  const fullPath = join(REPO, path);
  const content = readFileSync(fullPath, "utf8");
  const parts = extractPlayHostRegion(content);
  if (!parts) {
    console.log(`SKIP no PlayHost: ${path}`);
    continue;
  }
  const playHostPath = join(dirname(fullPath), "play-host.tsx");
  const playHostContent = playHostHeader(parts.regionName) + parts.region.replace(/^\/\/#region 🔖️\w+PlayHost\n/, "");
  writeFileSync(playHostPath, playHostContent);
  writeFileSync(fullPath, parts.before.trimEnd() + "\n" + parts.after.trimStart());
  const pkgPath = join(dirname(fullPath), "package.json");
  if (existsSync(pkgPath)) {
    const pkgJson = JSON.parse(readFileSync(pkgPath, "utf8"));
    pkgJson.exports ??= {};
    pkgJson.exports["./play"] = "./play-host.tsx";
    writeFileSync(pkgPath, JSON.stringify(pkgJson, null, 2) + "\n");
  }
  console.log(`Split ${path} -> play-host.tsx`);
}

for (const { corePath, reactPkg, bootFn } of CORE_BOOT_UPDATES) {
  const fullPath = join(REPO, corePath);
  if (!existsSync(fullPath)) continue;
  let content = readFileSync(fullPath, "utf8");
  const oldImport = `await import("${reactPkg}")`;
  const newImport = `await import("${reactPkg}/play")`;
  if (content.includes(oldImport)) {
    content = content.replaceAll(oldImport, newImport);
    writeFileSync(fullPath, content);
    console.log(`Updated ${corePath} bootRenderer -> ${reactPkg}/play`);
  }
}

console.log("DONE split-play-host");
