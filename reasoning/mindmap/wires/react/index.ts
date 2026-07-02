// #region 🧲Header
/** @emoji 🔗 `@semio-tech/reasoning-mindmap-wires-react` — WIRES fixture parsing and puzzle 2d board adapter. */
// #endregion 🧲Header

import metabolismWiresJson from "../example/metabolism.wires.json";
import { mergeKindCatalogBundleByRowId, type CameraState, type KindCatalogBundle, type Puzzle2dFixtureEdge, type Puzzle2dFixtureNode, type Puzzle2dFixture } from "@semio-tech/puzzle-2d-react";
import { mergeManifestCatalogBundles, type WiresEdgeKindId, wiresManifestCatalogBundle } from "@semio-tech/graph-manifest";
import type { MindmapFixtureEdge, MindmapFixtureNode, MindmapFixture } from "@semio-tech/reasoning-mindmap-react";

/** @emoji 🧩 Board slice embedded in {@link WiresFixture}. */
export type WiresFixtureBoard = MindmapFixture;

export type { MindmapFixtureEdge, MindmapFixtureNode, MindmapFixture };

// #region 🔖RelationshipKind
export type RelationshipKind = "owns" | "is" | "references" | "has";

export type { WiresEdgeKindId };

const RELATIONSHIP_KINDS: readonly RelationshipKind[] = ["owns", "is", "references", "has"];

export function isRelationshipKind(value: string): value is RelationshipKind {
  return (RELATIONSHIP_KINDS as readonly string[]).includes(value);
}

export function relationshipKindToEdgeKindId(kind: RelationshipKind): string {
  return `wires.${kind}`;
}

export function edgeKindIdToRelationshipKind(edgeKindId: string): RelationshipKind | undefined {
  const trimmed = edgeKindId.trim();
  if (!trimmed.startsWith("wires.")) {
    return undefined;
  }
  const slug = trimmed.slice("wires.".length);
  return isRelationshipKind(slug) ? slug : undefined;
}

export function relationshipKindDisplayName(kind: RelationshipKind): string {
  switch (kind) {
    case "owns":
      return "Owns";
    case "is":
      return "Is";
    case "references":
      return "References";
    case "has":
      return "Has";
  }
}

export interface RelationshipKindTips {
  readonly sourceTip?: string;
  readonly targetTip?: string;
}

export function relationshipKindTips(kind: RelationshipKind): RelationshipKindTips {
  switch (kind) {
    case "is":
      return { targetTip: "filled-arrow" };
    case "has":
      return { targetTip: "open-diamond" };
    case "owns":
      return { targetTip: "filled-diamond" };
    case "references":
      return { targetTip: "fine-arrow" };
  }
}
// #endregion 🔖RelationshipKind

// #region 🔖IdentityKindCatalog
export interface WiresIdentityKindCatalogRow {
  readonly id: string;
  readonly name: string;
  readonly color?: string;
  readonly shape?: "circle" | "rectangle";
  readonly icon?: string;
  readonly stroke?: string;
}

export interface WiresRelationshipKindCatalogRow {
  readonly id: string;
  readonly name: string;
  readonly color?: string;
  readonly stroke?: string;
  readonly pattern?: "solid" | "dashed" | "dotted";
  readonly sourceTip?: string;
  readonly targetTip?: string;
  readonly directed?: boolean;
}

export interface WiresFixtureKindCatalogs {
  readonly identityKinds?: readonly WiresIdentityKindCatalogRow[];
  readonly relationshipKinds?: readonly WiresRelationshipKindCatalogRow[];
}

export function wiresFixtureKindCatalogsToPuzzle2d(catalogs: WiresFixtureKindCatalogs | undefined): KindCatalogBundle {
  const base = wiresManifestCatalogBundle();
  if (catalogs == null) {
    return base;
  }
  const patch: KindCatalogBundle = {};
  if (catalogs.identityKinds) {
    patch.nodes = catalogs.identityKinds.map((row) => ({
      id: row.id,
      name: row.name,
      ...(row.color !== undefined ? { color: row.color } : {}),
      ...(row.shape !== undefined ? { shape: row.shape } : {}),
      ...(row.icon !== undefined ? { icon: row.icon } : {}),
      ...(row.stroke !== undefined ? { stroke: row.stroke } : {}),
    }));
  }
  if (catalogs.relationshipKinds) {
    patch.edges = catalogs.relationshipKinds.map((row) => ({
      id: row.id,
      name: row.name,
      directed: row.directed ?? false,
      ...(row.color !== undefined ? { color: row.color } : {}),
      ...(row.stroke !== undefined ? { stroke: row.stroke } : {}),
      ...(row.pattern !== undefined ? { pattern: row.pattern } : {}),
      ...(row.sourceTip !== undefined ? { sourceTip: row.sourceTip } : {}),
      ...(row.targetTip !== undefined ? { targetTip: row.targetTip } : {}),
    }));
  }
  return mergeKindCatalogBundleByRowId(mergeManifestCatalogBundles(base), patch);
}

function parseWiresFixtureKindCatalogs(meta: Record<string, unknown> | undefined): WiresFixtureKindCatalogs | undefined {
  if (meta == null) {
    return undefined;
  }
  const raw = meta.kindCatalogs;
  if (raw == null || typeof raw !== "object") {
    return undefined;
  }
  const catalogs = raw as Record<string, unknown>;
  const identityKinds = parseIdentityKindCatalogRows(catalogs.identityKinds);
  const relationshipKinds = parseRelationshipKindCatalogRows(catalogs.relationshipKinds);
  if (identityKinds == null && relationshipKinds == null) {
    return undefined;
  }
  return {
    ...(identityKinds != null ? { identityKinds } : {}),
    ...(relationshipKinds != null ? { relationshipKinds } : {}),
  };
}

function parseIdentityKindCatalogRows(value: unknown): WiresIdentityKindCatalogRow[] | null {
  if (!Array.isArray(value)) {
    return null;
  }
  const out: WiresIdentityKindCatalogRow[] = [];
  for (const row of value) {
    if (row == null || typeof row !== "object") {
      return null;
    }
    const kind = row as Record<string, unknown>;
    if (typeof kind.id !== "string" || typeof kind.name !== "string") {
      return null;
    }
    out.push({
      id: kind.id,
      name: kind.name,
      ...(typeof kind.color === "string" ? { color: kind.color } : {}),
      ...(kind.shape === "circle" || kind.shape === "rectangle" ? { shape: kind.shape } : {}),
      ...(typeof kind.icon === "string" ? { icon: kind.icon } : {}),
      ...(typeof kind.stroke === "string" ? { stroke: kind.stroke } : {}),
    });
  }
  return out;
}

function parseRelationshipKindCatalogRows(value: unknown): WiresRelationshipKindCatalogRow[] | null {
  if (!Array.isArray(value)) {
    return null;
  }
  const out: WiresRelationshipKindCatalogRow[] = [];
  for (const row of value) {
    if (row == null || typeof row !== "object") {
      return null;
    }
    const kind = row as Record<string, unknown>;
    if (typeof kind.id !== "string" || typeof kind.name !== "string") {
      return null;
    }
    const sourceTip = typeof kind.sourceTip === "string" ? kind.sourceTip : typeof kind.source_tip === "string" ? kind.source_tip : undefined;
    const targetTip =
      typeof kind.targetTip === "string"
        ? kind.targetTip
        : typeof kind.target_tip === "string"
          ? kind.target_tip
          : typeof kind.marker === "string"
            ? kind.marker
            : undefined;
    out.push({
      id: kind.id,
      name: kind.name,
      ...(typeof kind.color === "string" ? { color: kind.color } : {}),
      ...(typeof kind.stroke === "string" ? { stroke: kind.stroke } : {}),
      ...(kind.pattern === "solid" || kind.pattern === "dashed" || kind.pattern === "dotted" ? { pattern: kind.pattern } : {}),
      ...(sourceTip !== undefined ? { sourceTip } : {}),
      ...(targetTip !== undefined ? { targetTip } : {}),
      ...(typeof kind.directed === "boolean" ? { directed: kind.directed } : { directed: false }),
    });
  }
  return out;
}

function identityKindCatalogShape(catalogs: WiresFixtureKindCatalogs | undefined, identityKindId: string): "circle" | "rectangle" | undefined {
  return catalogs?.identityKinds?.find((row) => row.id === identityKindId)?.shape;
}
// #endregion 🔖IdentityKindCatalog

// #region 🔖WiresFixture
export interface WiresFixtureIdentity {
  readonly identityId: number;
  readonly label: string;
  readonly nodeId: string;
  readonly identityKind?: string;
}

export interface WiresFixtureRelationship {
  readonly relationshipId: number;
  readonly sourceIdentityId: number;
  readonly targetIdentityId: number;
  readonly kind: RelationshipKind;
  readonly edgeId: string;
}

export interface WiresFixtureSource {
  readonly kitPath: string;
  readonly kitId: string;
  readonly kitName: string;
}

export interface WiresFixture {
  readonly schema: "reasoning.wires.fixture";
  readonly source: WiresFixtureSource;
  readonly identities: readonly WiresFixtureIdentity[];
  readonly relationships: readonly WiresFixtureRelationship[];
  readonly board: WiresFixtureBoard;
  readonly kindCatalogs?: WiresFixtureKindCatalogs;
}

export function parseWiresFixture(value: unknown): WiresFixture | null {
  if (value == null || typeof value !== "object") {
    return null;
  }
  const root = value as Record<string, unknown>;
  if (root.schema !== "reasoning.wires.fixture") {
    return null;
  }
  const sourceRaw = root.source;
  if (sourceRaw == null || typeof sourceRaw !== "object") {
    return null;
  }
  const source = sourceRaw as Record<string, unknown>;
  if (typeof source.kitPath !== "string" || typeof source.kitId !== "string" || typeof source.kitName !== "string") {
    return null;
  }
  const identities = parseWiresFixtureIdentities(root.identities);
  if (!identities) {
    return null;
  }
  const relationships = parseWiresFixtureRelationships(root.relationships);
  if (!relationships) {
    return null;
  }
  const board = parseWiresFixtureBoard(root.board);
  if (!board) {
    return null;
  }
  const kindCatalogs = parseWiresFixtureKindCatalogs(board.meta);
  return {
    schema: "reasoning.wires.fixture",
    source: { kitPath: source.kitPath, kitId: source.kitId, kitName: source.kitName },
    identities,
    relationships,
    board,
    ...(kindCatalogs !== undefined ? { kindCatalogs } : {}),
  };
}

function parseWiresFixtureIdentities(value: unknown): WiresFixtureIdentity[] | null {
  if (!Array.isArray(value)) {
    return null;
  }
  const out: WiresFixtureIdentity[] = [];
  for (const row of value) {
    if (row == null || typeof row !== "object") {
      return null;
    }
    const identity = row as Record<string, unknown>;
    if (typeof identity.identityId !== "number" || typeof identity.label !== "string" || typeof identity.nodeId !== "string") {
      return null;
    }
    out.push({
      identityId: identity.identityId,
      label: identity.label,
      nodeId: identity.nodeId,
      identityKind: typeof identity.identityKind === "string" ? identity.identityKind : undefined,
    });
  }
  return out;
}

function parseWiresFixtureRelationships(value: unknown): WiresFixtureRelationship[] | null {
  if (!Array.isArray(value)) {
    return null;
  }
  const out: WiresFixtureRelationship[] = [];
  for (const row of value) {
    if (row == null || typeof row !== "object") {
      return null;
    }
    const rel = row as Record<string, unknown>;
    if (
      typeof rel.relationshipId !== "number" ||
      typeof rel.sourceIdentityId !== "number" ||
      typeof rel.targetIdentityId !== "number" ||
      typeof rel.edgeId !== "string" ||
      typeof rel.kind !== "string" ||
      !isRelationshipKind(rel.kind)
    ) {
      return null;
    }
    out.push({
      relationshipId: rel.relationshipId,
      sourceIdentityId: rel.sourceIdentityId,
      targetIdentityId: rel.targetIdentityId,
      kind: rel.kind,
      edgeId: rel.edgeId,
    });
  }
  return out;
}

function parseWiresFixtureBoard(value: unknown): WiresFixtureBoard | null {
  if (value == null || typeof value !== "object") {
    return null;
  }
  const board = value as Record<string, unknown>;
  if (board.schema !== "reasoning.mindmap.fixture") {
    return null;
  }
  const camera = board.camera;
  if (camera == null || typeof camera !== "object") {
    return null;
  }
  const cam = camera as Record<string, unknown>;
  if (typeof cam.x !== "number" || typeof cam.y !== "number" || typeof cam.zoom !== "number") {
    return null;
  }
  if (!Array.isArray(board.nodes) || !Array.isArray(board.edges)) {
    return null;
  }
  for (const row of board.nodes) {
    if (row == null || typeof row !== "object") {
      return null;
    }
    const node = row as Record<string, unknown>;
    if (typeof node.id !== "string" || typeof node.x !== "number" || typeof node.y !== "number") {
      return null;
    }
    if ("handles" in node) {
      return null;
    }
  }
  for (const row of board.edges) {
    if (row == null || typeof row !== "object") {
      return null;
    }
    const edge = row as Record<string, unknown>;
    if (typeof edge.id !== "string" || typeof edge.source !== "string" || typeof edge.target !== "string") {
      return null;
    }
  }
  return {
    schema: "reasoning.mindmap.fixture",
    camera: { x: cam.x, y: cam.y, zoom: cam.zoom },
    nodes: board.nodes as MindmapFixtureNode[],
    edges: board.edges as MindmapFixtureEdge[],
    meta: board.meta != null && typeof board.meta === "object" ? (board.meta as Record<string, unknown>) : undefined,
  };
}

function identityKindByNodeId(fixture: WiresFixture, nodeId: string): string | undefined {
  return fixture.identities.find((identity) => identity.nodeId === nodeId)?.identityKind;
}

function applyIdentityKindsToBoardNodes(fixture: WiresFixture): MindmapFixtureNode[] {
  const catalogs = fixture.kindCatalogs;
  return fixture.board.nodes.map((node) => {
    const identityKind = identityKindByNodeId(fixture, node.id) ?? node.nodeKind;
    const catalogShape = identityKind != null ? identityKindCatalogShape(catalogs, identityKind) : undefined;
    const shape = node.shape ?? catalogShape ?? "rectangle";
    return {
      ...node,
      ...(identityKind !== undefined ? { nodeKind: identityKind } : {}),
      shape,
    };
  });
}

function applyRelationshipKindsToBoardEdges(fixture: WiresFixture): MindmapFixtureEdge[] {
  const byEdgeId = new Map(fixture.relationships.map((rel) => [rel.edgeId, rel.kind]));
  return fixture.board.edges.map((edge) => {
    const kind = byEdgeId.get(edge.id);
    const edgeKind = kind != null ? relationshipKindToEdgeKindId(kind) : edge.edgeKind;
    return edgeKind !== undefined ? { ...edge, edgeKind } : edge;
  });
}

export function mindmapBoardToPuzzle2dFixture(board: MindmapFixture, kindCatalogs?: WiresFixtureKindCatalogs): Puzzle2dFixture {
  const nodes: Puzzle2dFixtureNode[] = board.nodes.map((node) => {
    const shared = {
      id: node.id,
      x: node.x,
      y: node.y,
      handles: [] as const,
      ...(node.text !== undefined ? { text: node.text } : {}),
      ...(node.iconKind !== undefined ? { iconKind: node.iconKind } : {}),
      ...(node.nodeKind !== undefined ? { nodeKind: node.nodeKind } : {}),
      ...(node.root === true ? { root: true as const } : {}),
    };
    if (node.shape === "rectangle") {
      return { ...shared, shape: "rectangle" as const, width: node.width ?? 40, height: node.height ?? 40 };
    }
    return { ...shared, radius: node.radius ?? 40, shape: "circle" as const };
  });
  const edges: Puzzle2dFixtureEdge[] = board.edges.map((edge) => ({
    id: edge.id,
    source: edge.source,
    target: edge.target,
    ...(edge.edgeKind !== undefined ? { edgeKind: edge.edgeKind } : {}),
  }));
  const puzzleCatalogs = wiresFixtureKindCatalogsToPuzzle2d(kindCatalogs);
  return {
    schema: "puzzle.2d.fixture",
    camera: board.camera as CameraState,
    nodes,
    edges,
    ...(kindCatalogs !== undefined || board.meta !== undefined
      ? { meta: { ...(board.meta ?? {}), kindCatalogs: puzzleCatalogs } }
      : {}),
  };
}

export function wiresFixtureBoard(fixture: WiresFixture): Puzzle2dFixture {
  const board: MindmapFixture = {
    ...fixture.board,
    nodes: applyIdentityKindsToBoardNodes(fixture),
    edges: applyRelationshipKindsToBoardEdges(fixture),
  };
  return mindmapBoardToPuzzle2dFixture(board, fixture.kindCatalogs);
}

export function wiresRelationshipKindForEdgeId(fixture: WiresFixture, edgeId: string): RelationshipKind | undefined {
  return fixture.relationships.find((rel) => rel.edgeId === edgeId)?.kind;
}

export function wiresIdentityLabelForNodeId(fixture: WiresFixture, nodeId: string): string | undefined {
  return fixture.identities.find((identity) => identity.nodeId === nodeId)?.label;
}

export function wiresRelationshipDisplayLabelForEdgeId(fixture: WiresFixture, edgeId: string): string | undefined {
  const kind = wiresRelationshipKindForEdgeId(fixture, edgeId);
  return kind != null ? relationshipKindDisplayName(kind) : undefined;
}

export function wiresRelationshipHierarchyLabel(fixture: WiresFixture, edgeId: string): string | undefined {
  const edge = fixture.board.edges.find((row) => row.id === edgeId);
  const kind = wiresRelationshipKindForEdgeId(fixture, edgeId);
  if (edge == null || kind == null) {
    return undefined;
  }
  const source = wiresIdentityLabelForNodeId(fixture, edge.source) ?? edge.source;
  const target = wiresIdentityLabelForNodeId(fixture, edge.target) ?? edge.target;
  return `${relationshipKindDisplayName(kind)}: ${source} → ${target}`;
}

export const METABOLISM_WIRES_FIXTURE: WiresFixture =
  parseWiresFixture(metabolismWiresJson as unknown) ?? (metabolismWiresJson as WiresFixture);

/** @emoji 🧩 FiveD instance id suffix for kit WIRES surfaces (`{kitId}:kit:wires`). */
export const WIRES_KIT_INSTANCE_SUFFIX = ":kit:wires";

/** @emoji 🧩 Stable FiveD instance id for a kit WIRES graph surface. */
export function wiresKitInstanceId(kitId: string): string {
  return `${kitId}${WIRES_KIT_INSTANCE_SUFFIX}`;
}

/** @emoji 🕸️ True when a FiveD flat instance should run the live force-graph layout loop. */
export function isWiresLiveForceGraphInstanceId(instanceId: string): boolean {
  return instanceId.endsWith(WIRES_KIT_INSTANCE_SUFFIX);
}
// #endregion 🔖WiresFixture

// #region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("parseWiresFixture", () => {
    it("parses metabolism wires fixture with normal graph board", () => {
      const fixture = parseWiresFixture(metabolismWiresJson as unknown);
      expect(fixture?.schema).toBe("reasoning.wires.fixture");
      expect(fixture?.source.kitName).toBe("Metabolism");
      expect(fixture?.identities.length).toBe(7);
      expect(fixture?.relationships.length).toBe(9);
      expect(fixture?.board.schema).toBe("reasoning.mindmap.fixture");
      expect(fixture?.board.nodes.length).toBe(7);
      expect(fixture?.board.edges.length).toBe(9);
      expect(fixture?.board.nodes.every((node) => !("handles" in node))).toBe(true);
      expect(fixture?.board.edges[0]?.source).toBe("f042c2a4-3ba5-44b0-b22c-0ae8f568aacc");
      const board = wiresFixtureBoard(fixture!);
      expect(board.schema).toBe("puzzle.2d.fixture");
      expect(board.nodes.every((node) => node.handles.length === 0)).toBe(true);
      expect(board.edges[0]?.source).toBe("f042c2a4-3ba5-44b0-b22c-0ae8f568aacc");
      expect(board.edges[0]?.edgeKind).toBe("wires.owns");
    });

    it("maps edge ids to relationship kinds", () => {
      expect(wiresRelationshipKindForEdgeId(METABOLISM_WIRES_FIXTURE, "wires-rel-is-capital")).toBe("is");
      expect(wiresRelationshipKindForEdgeId(METABOLISM_WIRES_FIXTURE, "wires-rel-has-capsule")).toBe("has");
    });

    it("relationship kind mapping helpers round-trip edge kind ids", () => {
      expect(relationshipKindToEdgeKindId("owns")).toBe("wires.owns");
      expect(edgeKindIdToRelationshipKind("wires.references")).toBe("references");
      expect(relationshipKindDisplayName("has")).toBe("Has");
      expect(relationshipKindTips("is")).toEqual({ targetTip: "filled-arrow" });
      expect(relationshipKindTips("has")).toEqual({ targetTip: "open-diamond" });
      expect(relationshipKindTips("owns")).toEqual({ targetTip: "filled-diamond" });
      expect(relationshipKindTips("references")).toEqual({ targetTip: "fine-arrow" });
    });

    it("builds relationship hierarchy labels with kind prefix", () => {
      expect(wiresRelationshipHierarchyLabel(METABOLISM_WIRES_FIXTURE, "wires-rel-is-capital")).toBe("Is: Bridge → Capital");
    });

    it("wiresKitInstanceId and isWiresLiveForceGraphInstanceId match kit wires surfaces", () => {
      expect(wiresKitInstanceId("kit-a")).toBe("kit-a:kit:wires");
      expect(isWiresLiveForceGraphInstanceId("kit-a:kit:wires")).toBe(true);
      expect(isWiresLiveForceGraphInstanceId("kit-a:design:diagram")).toBe(false);
    });

    it("propagates node text and iconKind avatars onto the puzzle 2d board", () => {
      const board = mindmapBoardToPuzzle2dFixture({
        schema: "reasoning.mindmap.fixture",
        camera: { x: 0, y: 0, zoom: 1 },
        nodes: [
          { id: "a", x: 0, y: 0, shape: "rectangle", width: 40, height: 40, text: "Alpha", iconKind: "<svg></svg>" } as MindmapFixtureNode,
          { id: "b", x: 10, y: 0, shape: "circle", radius: 20, text: "Beta" } as MindmapFixtureNode,
        ],
        edges: [],
      });
      const a = board.nodes.find((node) => node.id === "a") as { text?: string; iconKind?: string };
      const b = board.nodes.find((node) => node.id === "b") as { text?: string; iconKind?: string };
      expect(a.text).toBe("Alpha");
      expect(a.iconKind).toBe("<svg></svg>");
      expect(b.text).toBe("Beta");
      expect(b.iconKind).toBeUndefined();
    });
  });
}
// #endregion 🧪Tests
