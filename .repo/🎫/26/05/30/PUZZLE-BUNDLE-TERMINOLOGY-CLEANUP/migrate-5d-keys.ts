#!/usr/bin/env bun
/** One-off: rename flat/volume/board/scene keys in puzzle 5d fixture JSON. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "../../../../../..");

function migrate5dJson(path: string): void {
  const full = join(root, path);
  const doc = JSON.parse(readFileSync(full, "utf8")) as Record<string, unknown>;
  if (doc.schema === "puzzle.5d.topology/v1") doc.schema = "puzzle.5d/v1";
  if ("flatCamera" in doc) {
    doc.camera2d = doc.flatCamera;
    delete doc.flatCamera;
  }
  if ("volumeCamera" in doc) {
    doc.camera3d = doc.volumeCamera;
    delete doc.volumeCamera;
  }
  const parts = doc.parts;
  if (Array.isArray(parts)) {
    for (const part of parts) {
      if (!part || typeof part !== "object") continue;
      const p = part as Record<string, unknown>;
      if ("flat" in p) {
        p.puzzle2d = p.flat;
        delete p.flat;
      }
      if ("volume" in p) {
        p.puzzle3d = p.volume;
        delete p.volume;
      }
      const anchors = p.anchors;
      if (Array.isArray(anchors)) {
        for (const anchor of anchors) {
          if (!anchor || typeof anchor !== "object") continue;
          const a = anchor as Record<string, unknown>;
          if ("flat" in a) {
            a.puzzle2d = a.flat;
            delete a.flat;
          }
          if ("volume" in a) {
            a.puzzle3d = a.volume;
            delete a.volume;
          }
        }
      }
    }
  }
  writeFileSync(full, `${JSON.stringify(doc, null, 2)}\n`);
  console.log(`[DEBUG] migrated ${path}`);
}

migrate5dJson("puzzle/assets/nakagin-capsule-tower.5d.json");
const fixture5d = join(root, "puzzle/5d/fixture/nakagin-capsule-tower.5d.json");
try {
  readFileSync(fixture5d);
  migrate5dJson("puzzle/5d/fixture/nakagin-capsule-tower.5d.json");
} catch {
  /* optional */
}
