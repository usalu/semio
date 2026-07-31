/** @emoji 🧳 Migrate geometry*.json fixtures to spatial.model/v1. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const fixtures = "c:/git/compose/spatial/fixtures";

function migrate(raw: Record<string, unknown>): Record<string, unknown> {
  const cells = Array.isArray(raw.cells) ? (raw.cells as { id: string }[]) : [];
  const objects = cells.map((c) => ({
    id: `object-${c.id}`,
    typologyId: "builtin.primitive.box",
    geometryRef: c.id,
  }));
  const geometry: Record<string, unknown> = {};
  for (const k of ["anchors", "vertices", "edges", "wires", "faces", "shells", "cells", "cellComplexes", "clusters"] as const) {
    geometry[k] = Array.isArray(raw[k]) ? raw[k] : [];
  }
  return {
    schema: "spatial.model/v1",
    revision: typeof raw.revision === "number" ? raw.revision : 0,
    objects,
    geometry,
  };
}

for (const name of ["geometry.json", "geometry-loom.json", "geometry-routes.json"]) {
  const path = join(fixtures, name);
  const raw = JSON.parse(readFileSync(path, "utf8")) as Record<string, unknown>;
  if (raw.schema === "spatial.topology/v1") writeFileSync(path, JSON.stringify(migrate(raw), null, 2) + "\n");
}
