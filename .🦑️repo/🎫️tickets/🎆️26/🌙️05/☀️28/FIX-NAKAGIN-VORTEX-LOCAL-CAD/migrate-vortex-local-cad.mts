#!/usr/bin/env bun
/** @emoji 🧾️ One-off: rewrite nakagin scene vortex positions as type-local CAD connector points (kit port → connector.point). */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "..", "..", "..", "..", "..", "..");
const kitPath = join(repoRoot, "compose/fixtures/stores/metabolism/wip/initialKit/kit.compose.json");
const boardPath = join(repoRoot, "elements/lib/board/play/fixtures/nakagin-capsule-tower.board.json");
const scenePath = join(repoRoot, "elements/lib/react/scene/play/fixtures/nakagin-capsule-tower.scene.json");

type Vec3 = [number, number, number];
type ConnectorRow = { readonly id: string; readonly point: Vec3; readonly direction?: Vec3; readonly port?: { readonly id?: string } };
type TypeRow = { readonly id: string; readonly connectors?: { readonly items?: readonly ConnectorRow[] } };
type BoardHandle = { readonly id: string; readonly handleKind?: string; readonly radius?: number };
type BoardNode = { readonly id: string; readonly nodeKind?: string; readonly handles?: readonly BoardHandle[] };
type SceneVortex = { readonly id: string; readonly vortexKind?: string; readonly position: Vec3; readonly direction?: Vec3; readonly radius?: number };
type SceneObject = {
  readonly id: string;
  readonly vortices?: SceneVortex[];
  readonly [k: string]: unknown;
};

function suffixAfterLastDot(kind: string): string {
  const i = kind.lastIndexOf(".");
  return i >= 0 ? kind.slice(i + 1) : kind;
}

function cadPoint(row: { x: number; y: number; z: number }): Vec3 {
  return [row.x, row.y, row.z];
}

function connectorOnTypeByPortId(type: TypeRow, portId: string): ConnectorRow | undefined {
  return (type.connectors?.items ?? []).find((c) => c.port?.id === portId);
}

function buildBoardById(nodes: readonly BoardNode[]): Map<string, BoardNode> {
  return new Map(nodes.map((n) => [n.id, n]));
}

function sceneRadiusFromBoard(boardRadius: number | undefined, existing: number | undefined): number | undefined {
  if (typeof existing === "number") return existing;
  if (typeof boardRadius === "number") return boardRadius * 0.12;
  return undefined;
}

function main(): void {
  const kit = JSON.parse(readFileSync(kitPath, "utf8")) as { types: { items: TypeRow[] } };
  const board = JSON.parse(readFileSync(boardPath, "utf8")) as { nodes: BoardNode[] };
  const scene = JSON.parse(readFileSync(scenePath, "utf8")) as { objects: SceneObject[] };

  const boardById = buildBoardById(board.nodes);

  let migrated = 0;
  let missing = 0;

  const objects = scene.objects.map((obj) => {
    const node = boardById.get(obj.id);
    if (!node?.handles?.length) return obj;
    const typeId = node.nodeKind ? suffixAfterLastDot(node.nodeKind) : "";
    const type = kit.types.items.find((t) => t.id === typeId);
    if (!type) {
      missing += node.handles.length;
      return obj;
    }

    const existingById = new Map((obj.vortices ?? []).map((v) => [v.id, v]));
    const vortices: SceneVortex[] = [];

    for (const h of node.handles) {
      const portId = h.handleKind ? suffixAfterLastDot(h.handleKind) : "";
      const connector = connectorOnTypeByPortId(type, portId);
      if (!connector?.point) {
        missing += 1;
        const prev = existingById.get(h.id);
        if (prev) vortices.push(prev);
        continue;
      }
      const prev = existingById.get(h.id);
      const position = cadPoint(connector.point as { x: number; y: number; z: number });
      const direction = connector.direction != null ? cadPoint(connector.direction as { x: number; y: number; z: number }) : prev?.direction;
      const radius = sceneRadiusFromBoard(h.radius, prev?.radius);
      vortices.push({
        id: h.id,
        ...(h.handleKind ? { vortexKind: h.handleKind } : {}),
        position,
        ...(direction ? { direction } : {}),
        ...(radius !== undefined ? { radius } : {}),
      });
      migrated += 1;
    }

    return { ...obj, vortices };
  });

  writeFileSync(scenePath, `${JSON.stringify({ ...scene, objects }, null, 2)}\n`, "utf8");
  console.log(`[migrate-vortex-local-cad] wrote ${scenePath} (${migrated} vortices, ${missing} unresolved)`);
}

main();
