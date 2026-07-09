#!/usr/bin/env bun
/** @emoji 🧾 One-off: board fixture + 2D layout → scene fixture JSON (no `@elements/scene` import). */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "..", "..", "..", "..", "..", "..");

const boardPath = join(repoRoot, ".storybook/fixtures/nakagin-capsule-tower.board.json");
const outPath = join(repoRoot, "elements/client/lib/scene/fixtures/nakagin-capsule-tower.scene.json");

function iconKindToMeshUrl(icon: string): string {
  return `/meshes/${icon}.glb`;
}

function bodyRadius(node: Record<string, unknown>): number {
  if (typeof node.radius === "number") return node.radius;
  const w = typeof node.width === "number" ? node.width : 40;
  const h = typeof node.height === "number" ? node.height : 40;
  return Math.max(w, h) * 0.5;
}

function main() {
  const board = JSON.parse(readFileSync(boardPath, "utf8")) as Record<string, unknown>;
  const nodes = board.nodes as Record<string, unknown>[];
  const edges = board.edges as Record<string, unknown>[];
  const meta = board.meta;

  const objects = nodes.map((node) => {
    const id = String(node.id);
    const x = Number(node.x);
    const y = Number(node.y);
    const origin: [number, number, number] = [x, 0, -y];
    const orientation: [number, number, number, number] = [0, 0, 0, 1];
    const iconKind = String(node.iconKind ?? "placeholder");
    const br = bodyRadius(node);
    const handles = (node.handles as Record<string, unknown>[] | undefined) ?? [];
    const vortices = handles.map((h) => {
      const angle = Number(h.angle ?? 0);
      const lx = Math.cos(angle) * br;
      const lz = Math.sin(angle) * br;
      return {
        id: String(h.id),
        vortexKind: typeof h.handleKind === "string" ? h.handleKind : undefined,
        position: [lx, 0.4, lz] as [number, number, number],
        ...(typeof h.radius === "number" ? { radius: h.radius * 0.12 } : {}),
      };
    });
    return {
      id,
      label: typeof node.label === "string" ? node.label : id,
      objectKind: typeof node.nodeKind === "string" ? node.nodeKind : undefined,
      meshUrl: iconKindToMeshUrl(iconKind),
      origin,
      orientation,
      vortices,
    };
  });

  const ties = edges.map((e) => ({
    id: String(e.id),
    source: String(e.source),
    target: String(e.target),
  }));

  const scene = {
    schema: "elements.scene.fixture/v1",
    camera: {
      position: [420, 320, 420],
      target: [0, 40, 0],
      zoom: 1,
    },
    ...(meta && typeof meta === "object" ? { meta } : {}),
    ties,
    objects,
  };

  writeFileSync(outPath, JSON.stringify(scene, null, 2));
  console.log(`[bake] wrote ${outPath} (${objects.length} objects, ${ties.length} ties)`);
}

main();
