#!/usr/bin/env bun
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const repoRoot = resolve(import.meta.dir, "../../../../../../");
const coreDirs = [
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
  "puzzle/2d/core",
  "puzzle/5d/core",
  "trinity/rewrite/core",
  "trinity/jack/host-core",
  "cad/js/renderer/core",
  "flow/core",
  "imperative/core",
  "sequence/core",
  "lowpoly/core",
  "vcs/core",
  "framework/product/presentation/core",
  "reasoning/mindmap/wires/core",
  "puzzle/3d/core",
];

function symbolDeclaredBefore(before: string, name: string): boolean {
  return new RegExp(`\\b${name}\\b`).test(before);
}

for (const coreDir of coreDirs) {
  const path = join(repoRoot, coreDir, "index.ts");
  if (!existsSync(path)) continue;
  let content = readFileSync(path, "utf8");
  const playStart = content.indexOf("//#region 🔖Play");
  if (playStart < 0) continue;
  const before = content.slice(0, playStart);
  let play = content.slice(playStart);
  play = play.replace(/^export const ([A-Z0-9_]+)[^;]*;\n/gm, (match, name) => (symbolDeclaredBefore(before, name) ? "" : match));
  play = play.replace(/^export function ([A-Za-z0-9_]+)\(/gm, (match, name) => (symbolDeclaredBefore(before, name) ? "" : match));
  play = play.replace(/^const ([A-Z0-9_]+)[^;]*;\n/gm, (match, name) => (symbolDeclaredBefore(before, name) ? "" : match));
  play = play.replace(/^function ([A-Za-z0-9_]+)\(/gm, (match, name) => (symbolDeclaredBefore(before, name) ? "" : match));
  play = play.replace(/^let ([A-Za-z0-9_]+)[^;]*;\n/gm, (match, name) => (symbolDeclaredBefore(before, name) ? "" : match));
  content = before + play;
  writeFileSync(path, content);
}
console.log("[DEBUG] dedupe play region complete");
