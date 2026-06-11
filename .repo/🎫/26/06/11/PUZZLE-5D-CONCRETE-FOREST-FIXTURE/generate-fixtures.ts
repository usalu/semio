#!/usr/bin/env bun
/** 🏗 Generates puzzle 2d + 5d concrete forest fixtures from the existing 3d fixture. */
import { writeFile } from "node:fs/promises";
import { join } from "node:path";
import concreteForest3d from "../../../../../../puzzle/3d/fixture/concrete-forest.3d.json";
import { compose5d, flatHandleConnectorAngle } from "../../../../../../puzzle/5d/react/index.tsx";
import { parseFixtureV1, type FixtureV1 } from "../../../../../../puzzle/3d/react/index.tsx";
import { parsePuzzle2dFixtureV1, type Puzzle2dFixtureV1 } from "../../../../../../puzzle/2d/react/index.tsx";

const fixture3d = parseFixtureV1(concreteForest3d as unknown);
if (!fixture3d) throw new Error("concrete-forest.3d.json invalid");

function meta2dFromMeta3d(meta: FixtureV1["meta"]): Puzzle2dFixtureV1["meta"] {
  const kc = meta?.kindCatalogs;
  if (!kc) return { kindCompatibility: meta?.kindCompatibility };
  return {
    kindCatalogs: {
      handles: (kc.vortices ?? []).map((row) => ({
        id: row.id,
        label: row.label ?? row.id,
        name: row.name ?? row.id,
        color: row.color,
        defaultWireKind: row.defaultCableKind ?? "wire.link",
      })),
      nodes: (kc.objects ?? []).map((row) => ({
        id: row.id,
        label: row.label ?? row.id,
        name: row.name ?? row.id,
        meshUrl: row.meshUrl,
        handles: (row.vortices ?? []).map((vortex, index, all) => ({
          handleKind: vortex.vortexKind ?? "port",
          angle: flatHandleConnectorAngle(index, all.length),
          radius: vortex.radius ?? 0.36,
        })),
      })),
      wires: (kc.cables ?? []).map((row) => ({
        id: row.id,
        label: row.label ?? row.id,
        name: row.name ?? row.id,
        defaultEdgeKind: row.defaultAttractionKind ?? "puzzle2d.attraction.link",
      })),
      edges: (kc.attractions ?? []).map((row) => ({
        id: row.id,
        label: row.label ?? row.id,
        name: row.name ?? row.id,
      })),
    },
    kindCompatibility: meta?.kindCompatibility,
  };
}

const seed = fixture3d.objects[0];
if (!seed) throw new Error("concrete-forest.3d.json needs a seed object");

const scale = 40;
const cx = seed.vortices.reduce((sum, v) => sum + v.position[0], 0) / Math.max(seed.vortices.length, 1);
const cy = seed.vortices.reduce((sum, v) => sum + v.position[1], 0) / Math.max(seed.vortices.length, 1);

const fixture2d: Puzzle2dFixtureV1 = {
  schema: "puzzle.2d.fixture/v1",
  camera: { x: -cx * scale, y: -cy * scale, zoom: 1.2 },
  nodes: [
    {
      id: seed.id,
      nodeKind: seed.objectKind,
      shape: "rectangle",
      x: cx * scale,
      y: cy * scale,
      width: 10.8 * scale,
      height: 6.2 * scale,
      text: seed.label ?? seed.objectKind,
      handles: seed.vortices.map((vortex, index) => ({
        id: vortex.id.includes(":") ? vortex.id : `${seed.id}:${vortex.id}`,
        handleKind: vortex.vortexKind ?? "port",
        angle: flatHandleConnectorAngle(index, seed.vortices.length),
        radius: 3,
      })),
    },
  ],
  edges: [],
  meta: meta2dFromMeta3d(fixture3d.meta),
};

if (!parsePuzzle2dFixtureV1(fixture2d)) throw new Error("generated 2d fixture invalid");

const model5d = {
  ...compose5d(fixture2d, fixture3d),
  label: "Concrete Forest",
  meta: {
    description: "Unified puzzle 5d source for Concrete Forest play; 2d and 3d views project from this model.",
  },
};

const root = join(import.meta.dir, "../../../../../../");
await writeFile(join(root, "puzzle/2d/fixture/concrete-forest.2d.json"), `${JSON.stringify(fixture2d, null, 2)}\n`);
await writeFile(join(root, "puzzle/5d/fixture/concrete-forest.5d.json"), `${JSON.stringify(model5d, null, 2)}\n`);
console.log(`[generate-fixtures] wrote concrete-forest 2d (${fixture2d.nodes.length} nodes) and 5d (${model5d.parts.length} parts)`);
