#!/usr/bin/env bun
/** @emoji 🧾 One-off: rewrite nakagin 2d kind-catalog + instance handle angles from compose connector ring `t`. */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import {
  puzzle2dHandleAngleFromRingT,
  puzzle2dNodeKindHandlesFromKitConnectors,
  puzzle2dNormalizeRingT,
  type KitConnectorCadRow,
} from "../../../../../../puzzle/2d/react/index.tsx";

const repoRoot = join(import.meta.dir, "..", "..", "..", "..", "..", "..");
const kitPath = join(repoRoot, "compose/fixtures/nakagin-capsule-tower.filtered.kit.compose.json");
const fixturePath = join(repoRoot, "puzzle/2d/fixture/nakagin-capsule-tower.2d.json");

type PortRow = { readonly id: string; readonly name: string };
type ConnectorRow = {
  readonly name?: string;
  readonly t?: number;
  readonly port?: { readonly id?: string };
};
type TypeRow = { readonly id: string; readonly name: string; readonly connectors?: readonly ConnectorRow[] };
type KitJson = {
  readonly families?: readonly { readonly ports?: readonly PortRow[] }[];
  readonly typologies?: { readonly items?: readonly { readonly types?: { readonly items?: readonly TypeRow[] } } }[];
};

type CatalogHandle = { handleKind: string; angle: number; radius?: number };
type CatalogNode = { id: string; name: string; handles?: CatalogHandle[]; shape?: "circle" | "rectangle" };
type FixtureHandle = { id: string; handleKind: string; angle: number; radius?: number };
type FixtureNode = {
  readonly id: string;
  readonly nodeKind?: string;
  readonly shape?: "circle" | "rectangle";
  readonly radius?: number;
  readonly width?: number;
  readonly height?: number;
  handles?: FixtureHandle[];
};

type FixtureJson = {
  nodes: FixtureNode[];
  meta?: { kindCatalogs?: { nodes?: CatalogNode[] } };
};

const BRUSH = 40;

function collectTypes(kit: KitJson): TypeRow[] {
  const out: TypeRow[] = [];
  for (const topo of kit.typologies?.items ?? []) {
    for (const type of topo.types?.items ?? []) {
      out.push(type);
    }
  }
  return out;
}

function portNameById(kit: KitJson): Map<string, string> {
  const map = new Map<string, string>();
  for (const family of kit.families ?? []) {
    for (const port of family.ports ?? []) {
      map.set(port.id, port.name);
    }
  }
  return map;
}

function kitConnectorsForType(type: TypeRow, portNames: Map<string, string>): KitConnectorCadRow[] {
  return (type.connectors ?? []).map((connector) => ({
    ...(typeof connector.t === "number" ? { t: connector.t } : {}),
    port: { handleKind: portNames.get(connector.port?.id ?? "") ?? "" },
  }));
}

function prototypeForCatalogNode(row: CatalogNode | undefined) {
  const shape = row?.shape ?? "circle";
  return shape === "rectangle" ? { shape: "rectangle" as const, width: BRUSH, height: BRUSH } : { shape: "circle" as const, radius: BRUSH / 2 };
}

function prototypeForFixtureNode(node: FixtureNode) {
  if (node.shape === "rectangle") {
    return { shape: "rectangle" as const, width: node.width ?? BRUSH, height: node.height ?? BRUSH };
  }
  return { shape: "circle" as const, radius: node.radius ?? BRUSH / 2 };
}

function connectorByName(type: TypeRow): Map<string, ConnectorRow> {
  const map = new Map<string, ConnectorRow>();
  for (const connector of type.connectors ?? []) {
    const name = connector.name?.trim();
    if (name) {
      map.set(name, connector);
    }
  }
  return map;
}

async function main(): Promise<void> {
  const kit = JSON.parse(await Bun.file(kitPath).text()) as KitJson;
  const fixture = JSON.parse(await Bun.file(fixturePath).text()) as FixtureJson;
  const portNames = portNameById(kit);
  const typesByName = new Map(collectTypes(kit).map((type) => [type.name, type]));

  let catalogAngles = 0;
  for (const row of fixture.meta?.kindCatalogs?.nodes ?? []) {
    const type = typesByName.get(row.name) ?? typesByName.get(row.id);
    if (!type) {
      continue;
    }
    const templates = puzzle2dNodeKindHandlesFromKitConnectors(kitConnectorsForType(type, portNames), {
      prototype: prototypeForCatalogNode(row),
    });
    if (templates.length) {
      row.handles = templates;
      catalogAngles += templates.length;
    }
  }

  let instanceAngles = 0;
  let instanceMiss = 0;
  for (const node of fixture.nodes) {
    const kindName = node.nodeKind?.trim();
    if (!kindName || !node.handles?.length) {
      continue;
    }
    const type = typesByName.get(kindName);
    if (!type) {
      instanceMiss += node.handles.length;
      continue;
    }
    const byName = connectorByName(type);
    const proto = prototypeForFixtureNode(node);
    const ringNode = { height: proto.shape === "rectangle" ? proto.height : 0, radius: proto.shape === "circle" ? proto.radius : 0, shape: proto.shape, width: proto.shape === "rectangle" ? proto.width : 0, x: 0, y: 0 };
    for (const handle of node.handles) {
      const suffix = handle.id.includes(":") ? (handle.id.split(":").pop() ?? "") : "";
      const connector = (suffix ? byName.get(suffix) : undefined) ?? (type.connectors ?? []).find((c) => portNames.get(c.port?.id ?? "") === handle.handleKind);
      if (!connector || typeof connector.t !== "number" || !Number.isFinite(connector.t)) {
        instanceMiss += 1;
        continue;
      }
      handle.angle = puzzle2dHandleAngleFromRingT(ringNode, puzzle2dNormalizeRingT(connector.t));
      instanceAngles += 1;
    }
  }

  writeFileSync(fixturePath, `${JSON.stringify(fixture, null, 2)}\n`);
  console.log(
    `[DEBUG] migrate-nakagin-2d-handle-angles wrote ${fixturePath} (${catalogAngles} catalog templates, ${instanceAngles} instance handles, ${instanceMiss} misses)`,
  );
}

await main();
