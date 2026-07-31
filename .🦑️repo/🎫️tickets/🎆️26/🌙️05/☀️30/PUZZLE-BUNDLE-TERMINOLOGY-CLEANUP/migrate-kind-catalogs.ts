#!/usr/bin/env bun
/** One-off migration: puzzle fixture kind catalog keys and builtin ids. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "../../../../../..");

function replaceAll(text: string, pairs: Array<[string, string]>): string {
  let out = text;
  for (const [from, to] of pairs) out = out.split(from).join(to);
  return out;
}

function migrateText(path: string): void {
  const full = join(root, path);
  try {
    readFileSync(full);
  } catch {
    return;
  }
  let text = readFileSync(full, "utf8");
  text = replaceAll(text, [
    ["board.wire.link", "wire.link"],
    ["board.edge.link", "edge.link"],
    ["board.cable.link", "cable.link"],
    ['"board.port"', '"port"'],
    ["board.port", "port"],
  ]);
  writeFileSync(full, text);
  console.log(`[DEBUG] migrated ${path}`);
}

function migrateTopologyKindCatalogs(path: string): void {
  const full = join(root, path);
  const doc = JSON.parse(readFileSync(full, "utf8")) as Record<string, unknown>;
  const kc = doc.kindCatalogs;
  if (kc && typeof kc === "object" && !Array.isArray(kc)) {
    const box = kc as Record<string, unknown>;
    if (Array.isArray(box.handles) && !box.grips) {
      box.grips = (box.handles as Array<Record<string, unknown>>).map((row) => {
        const { defaultWireKind, ...rest } = row;
        return {
          ...rest,
          ...(defaultWireKind !== undefined ? { defaultRopeKind: defaultWireKind } : {}),
        };
      });
      delete box.handles;
    }
    if (Array.isArray(box.nodes) && !box.parts) {
      box.parts = box.nodes;
      delete box.nodes;
    }
    if (Array.isArray(box.wires) && !box.ropes) {
      box.ropes = (box.wires as Array<Record<string, unknown>>).map((row) => {
        const { defaultEdgeKind, ...rest } = row;
        return {
          ...rest,
          ...(defaultEdgeKind !== undefined ? { defaultFastenerKind: defaultEdgeKind } : {}),
        };
      });
      delete box.wires;
    }
    if (Array.isArray(box.edges) && !box.fasteners) {
      box.fasteners = box.edges;
      delete box.edges;
    }
  }
  writeFileSync(full, `${JSON.stringify(doc, null, 2)}\n`);
  console.log(`[DEBUG] migrated topology kindCatalogs in ${path}`);
}

for (const path of [
  "puzzle/assets/nakagin-capsule-tower.board.json",
  "puzzle/assets/nakagin-capsule-tower.scene.json",
  "puzzle/assets/nakagin-capsule-tower.topology.json",
  "puzzle/2d/play/fixtures/nakagin-capsule-tower.board.json",
  "puzzle/3d/play/fixtures/nakagin-capsule-tower.scene.json",
  "puzzle/5d/play/fixtures/nakagin-capsule-tower.topology.json",
]) {
  migrateText(path);
}

migrateTopologyKindCatalogs("puzzle/assets/nakagin-capsule-tower.topology.json");
migrateTopologyKindCatalogs("puzzle/5d/play/fixtures/nakagin-capsule-tower.topology.json");
