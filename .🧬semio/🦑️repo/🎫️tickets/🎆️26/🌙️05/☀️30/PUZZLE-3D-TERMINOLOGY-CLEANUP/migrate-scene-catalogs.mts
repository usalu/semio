#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const repoRoot = resolve(import.meta.dir, "../../../../../..");
const scenePath = resolve(repoRoot, "puzzle/assets/nakagin-capsule-tower.scene.json");
const scene = JSON.parse(readFileSync(scenePath, "utf8")) as {
  meta?: {
    kindCatalogs?: Record<string, unknown>;
    kindCompatibility?: Array<{ specificity?: string }>;
  };
};

const kc = scene.meta?.kindCatalogs;
if (kc && typeof kc === "object") {
  if (Array.isArray(kc.handles) && !kc.vortices) {
    kc.vortices = kc.handles;
    delete kc.handles;
  }
  if (Array.isArray(kc.nodes) && !kc.objects) {
    kc.objects = kc.nodes;
    delete kc.nodes;
  }
  delete kc.wires;
  delete kc.edges;
  for (const row of (kc.vortices ?? []) as Array<Record<string, unknown>>) {
    const wire = typeof row.defaultWireKind === "string" ? row.defaultWireKind : undefined;
    if (wire) {
      row.defaultCableKind = wire.replace("board.wire.", "board.cable.");
      delete row.defaultWireKind;
    }
  }
  kc.cables = [
    {
      id: "board.cable.link",
      label: "Link",
      name: "Link",
      defaultAttractionKind: "board.attraction.link",
    },
  ];
  kc.attractions = [{ id: "board.attraction.link", label: "Link", name: "Link" }];
}

for (const rule of scene.meta?.kindCompatibility ?? []) {
  if (rule.specificity === "handle") {
    rule.specificity = "vortex";
  }
}

writeFileSync(scenePath, `${JSON.stringify(scene, null, 2)}\n`);
