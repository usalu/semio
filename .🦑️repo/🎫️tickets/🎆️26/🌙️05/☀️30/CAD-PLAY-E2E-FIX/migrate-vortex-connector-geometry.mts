#!/usr/bin/env bun
/** @emoji 🧾️ One-off: bake nakagin vortex position + direction from metabolism kit type connector point/direction (port match via handle catalog). */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "..", "..", "..", "..", "..", "..");
const kitPath = join(repoRoot, "assets/compose/metabolism/wip/initialKit/kit.compose.json");
const boardPath = join(repoRoot, "puzzle/assets/nakagin-capsule-tower.board.json");
const scenePath = join(repoRoot, "puzzle/assets/nakagin-capsule-tower.scene.json");
const topologyPath = join(repoRoot, "puzzle/assets/nakagin-capsule-tower.topology.json");

type Vec3 = [number, number, number];
type CatalogRow = { readonly id: string; readonly name?: string };
type ConnectorRow = { readonly point: { readonly x: number; readonly y: number; readonly z: number }; readonly direction?: { readonly x: number; readonly y: number; readonly z: number }; readonly port?: { readonly id?: string } };
type TypeRow = { readonly id: string; readonly name: string; readonly connectors?: { readonly items?: readonly ConnectorRow[] } };
type BoardHandle = { readonly id: string; readonly handleKind?: string; readonly radius?: number };
type BoardNode = { readonly id: string; readonly nodeKind?: string; readonly handles?: readonly BoardHandle[] };
type SceneVortex = { readonly id: string; readonly vortexKind?: string; readonly position: Vec3; readonly direction?: Vec3; readonly radius?: number; readonly label?: string };
type SceneObject = { readonly id: string; readonly objectKind?: string; readonly vortices?: SceneVortex[]; readonly [k: string]: unknown };
type TopologyAnchor = { readonly id: string; readonly anchorKind?: string; readonly volume?: { readonly position?: Vec3; readonly direction?: Vec3; readonly radius?: number; readonly label?: string } };
type TopologyPart = { readonly id: string; readonly anchors?: TopologyAnchor[] };

function suffixAfterLastDot(kind: string): string {
  const i = kind.lastIndexOf(".");
  return i >= 0 ? kind.slice(i + 1) : kind;
}

function cadPoint(row: { x: number; y: number; z: number }): Vec3 {
  return [row.x, row.y, row.z];
}

function buildHandlePortByKind(handles: readonly CatalogRow[] | undefined): Map<string, string> {
  const map = new Map<string, string>();
  for (const row of handles ?? []) {
    const portId = suffixAfterLastDot(row.id);
    map.set(row.id, portId);
    if (row.name) map.set(row.name, portId);
  }
  return map;
}

function connectorOnTypeByPortId(type: TypeRow, portId: string): ConnectorRow | undefined {
  return (type.connectors?.items ?? []).find((c) => c.port?.id === portId);
}

function sceneRadiusFromBoard(boardRadius: number | undefined, existing: number | undefined): number | undefined {
  if (typeof existing === "number") return existing;
  if (typeof boardRadius === "number") return boardRadius * 0.12;
  return undefined;
}

function main(): void {
  const kit = JSON.parse(readFileSync(kitPath, "utf8")) as { types: { items: TypeRow[] } };
  const board = JSON.parse(readFileSync(boardPath, "utf8")) as { nodes: BoardNode[]; meta?: { kindCatalogs?: { handles?: CatalogRow[] } } };
  const scene = JSON.parse(readFileSync(scenePath, "utf8")) as { objects: SceneObject[] };
  const topology = JSON.parse(readFileSync(topologyPath, "utf8")) as { parts: TopologyPart[] };

  const typesByUuid = new Map(kit.types.items.map((t) => [t.id, t]));
  const typesByName = new Map(kit.types.items.map((t) => [t.name, t]));
  const boardById = new Map(board.nodes.map((n) => [n.id, n]));
  const handlePortByKind = buildHandlePortByKind(board.meta?.kindCatalogs?.handles);

  let migrated = 0;
  let missing = 0;

  const objects = scene.objects.map((obj) => {
    const node = boardById.get(obj.id);
    if (!node?.handles?.length) return obj;
    const typeUuid = node.nodeKind ? suffixAfterLastDot(node.nodeKind) : "";
    const type = typesByUuid.get(typeUuid) ?? (obj.objectKind ? typesByName.get(obj.objectKind) : undefined);
    if (!type) {
      missing += node.handles.length;
      return obj;
    }

    const existingById = new Map((obj.vortices ?? []).map((v) => [v.id, v]));
    const vortices: SceneVortex[] = [];

    for (const h of node.handles) {
      const portId = (h.handleKind ? handlePortByKind.get(h.handleKind) : undefined) ?? (h.handleKind ? suffixAfterLastDot(h.handleKind) : "");
      const connector = connectorOnTypeByPortId(type, portId);
      if (!connector?.point) {
        missing += 1;
        const prev = existingById.get(h.id);
        if (prev) vortices.push(prev);
        continue;
      }
      const prev = existingById.get(h.id);
      const position = cadPoint(connector.point);
      const direction = connector.direction != null ? cadPoint(connector.direction) : prev?.direction;
      const radius = sceneRadiusFromBoard(h.radius, prev?.radius);
      vortices.push({
        id: h.id,
        ...(h.handleKind ? { vortexKind: prev?.vortexKind ?? h.handleKind } : {}),
        ...(prev?.label ? { label: prev.label } : {}),
        position,
        ...(direction ? { direction } : {}),
        ...(radius !== undefined ? { radius } : {}),
      });
      migrated += 1;
    }

    return { ...obj, vortices };
  });

  const sceneByObjectId = new Map(objects.map((o) => [o.id, o]));
  const parts = topology.parts.map((part) => {
    const obj = sceneByObjectId.get(part.id);
    if (!obj?.vortices?.length) return part;
    const vortexByAnchorId = new Map(obj.vortices.map((v) => [suffixAfterLastDot(v.id), v]));
    const anchors = (part.anchors ?? []).map((anchor) => {
      const v = vortexByAnchorId.get(anchor.id) ?? vortexByAnchorId.get("link");
      if (!v?.position) return anchor;
      return {
        ...anchor,
        volume: {
          ...(anchor.volume ?? {}),
          position: v.position,
          ...(v.direction ? { direction: v.direction } : {}),
          ...(v.radius !== undefined ? { radius: v.radius } : {}),
          ...(v.label ? { label: v.label } : {}),
        },
      };
    });
    return { ...part, anchors };
  });

  writeFileSync(scenePath, `${JSON.stringify({ ...scene, objects }, null, 2)}\n`, "utf8");
  writeFileSync(topologyPath, `${JSON.stringify({ ...topology, parts }, null, 2)}\n`, "utf8");
  console.log(`[migrate-vortex-connector-geometry] scene ${migrated} vortices (${missing} unresolved) → ${scenePath}`);
  console.log(`[migrate-vortex-connector-geometry] topology anchors synced → ${topologyPath}`);
}

main();
