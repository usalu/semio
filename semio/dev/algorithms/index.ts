// #region 🧲Header
// 💻 semio/algorithms/index.ts
// Specs: Story helpers over `@semio/js` `openSessionInMemory` + `installProjection` plus plain JSON {@link Design} types from `@semio/ui`.
// Summary: WASM-backed flatten/drag/move reads and local diff helpers for Storybook; no snapshot store or schema bridge.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

/// <reference types="vite/client" />

// #region 📥Imports
import {
  AlgorithmApp,
  WindowKind,
  createIpoAlgorithmLayout,
  getKitPorts,
  kitSurface,
  useAlgorithm,
  type AlgorithmAppProps,
  type AlgorithmContextValue,
  type AlgorithmWindowDef,
  type DesignDiff,
  type DesignPlain,
  type MoveVector,
  type VecValue,
} from "@semio/ui";
import * as React from "react";

import { NakaginCapsuleTowerCopySelection, NakaginCapsuleTowerPasteDesign } from "@semio/assets";

import { openSessionInMemory, type Store as JsStore } from "@semio/js";
// #endregion 📥Imports

// #region 🧾GqlWire
/** @emoji 🧾 Local GraphQL response fragment object (replaces re-exported @semio/js wire types). */
type GqlWireObject = { readonly [k: string]: unknown };
// #endregion 🧾GqlWire

// #region 📤UiReExports
export {
  AlgorithmApp,
  WindowKind,
  createIpoAlgorithmLayout,
  getKitPorts,
  kitSurface,
  useAlgorithm,
  type AlgorithmAppProps,
  type AlgorithmContextValue,
  type AlgorithmWindowDef,
  type DesignDiff,
  type DesignPlain,
  type MoveVector,
  type VecValue,
};
// #endregion 📤UiReExports

/** @emoji 📍 2D coordinate used by drag algorithms (`u`/`v` plane). */
export type CoordinatePlain = Readonly<{ u: number; v: number }>;

/** @emoji 🧾 Anchoring kinds accepted by {@link pasteDesign}. */
export type PasteDesignAnchoringKind = "original" | "middle" | "centroid" | "bottomLeft" | "bottomRight" | "topLeft" | "topRight";

/** @emoji 🎯 Nakagin tower design id in metabolism kit fixtures. */
export const NAKAGIN_CAPSULE_TOWER_DESIGN_ID = "9a890dd4-0a9c-48ac-920a-9e62666465ef";

/** @emoji 🧾 Diagram selection wire for Storybook fixture JSON. */
export type DiagramSelectionWire = Readonly<{
  pieces?: readonly { id?: string }[];
  connections?: readonly { id?: string }[];
}>;

/** @emoji 📋 Paste options for {@link pasteDesign}. */
export type PasteDesignOptions = Readonly<{
  coordinate?: CoordinatePlain;
  anchoring?: PasteDesignAnchoringKind;
}>;

// #region 🧰StoryKitFacade
const PASTE_ANCHORS: readonly PasteDesignAnchoringKind[] = ["original", "middle", "centroid", "bottomLeft", "bottomRight", "topLeft", "topRight"] as const;

const PASTE_ANCHOR_LABELS: Record<PasteDesignAnchoringKind, string> = {
  original: "Original",
  middle: "Middle (bbox)",
  centroid: "Centroid",
  bottomLeft: "Bottom left",
  bottomRight: "Bottom right",
  topLeft: "Top left",
  topRight: "Top right",
};

/** @emoji 🧰 Static kit helpers for Storybook (selection anchoring + stub replaceable search). */
export const Kit = Object.freeze({
  pasteDesignAnchoringKinds: PASTE_ANCHORS,
  pasteAnchoringOptions: PASTE_ANCHORS.map((anchoringKind) => ({ anchoringKind, label: PASTE_ANCHOR_LABELS[anchoringKind] })),
});

/** @emoji 🔍 Stub find-replaceable result for Nakagin story (deterministic fixture design ids). */
export function findReplaceableTypesForSelection(_sel: { pieces: readonly string[] }): { types: string[]; designs: string[] } {
  return {
    types: [],
    designs: ["d7e12638-9749-471b-937e-a6e5523778ff", "019ab4e0-7295-7e1e-bb5f-9dfae8c0c4cf", "019ab4e0-8da8-7217-946f-5b5a83aca0e3"],
  };
}
// #endregion 🧰StoryKitFacade

// #region 🧱PlainDesignModel
function __itemsOf<T>(node: unknown): readonly T[] {
  if (Array.isArray(node)) return node as readonly T[];
  if (node && typeof node === "object" && "items" in node && Array.isArray((node as { items: unknown }).items)) return (node as { items: T[] }).items;
  return [];
}

function __listDesignsFromBundle(kit: unknown): Record<string, unknown>[] {
  const d = kitSurface(kit)["designs"];
  return [...__itemsOf<Record<string, unknown>>(d)];
}

function __plainFromDesign(d: unknown): Record<string, unknown> {
  const anyD = d as { toPlain?: () => DesignPlain };
  if (typeof anyD.toPlain === "function") return { ...(anyD.toPlain() as Record<string, unknown>) };
  return JSON.parse(JSON.stringify(d ?? {})) as Record<string, unknown>;
}

function __clonePlainDesign(d: unknown): Record<string, unknown> {
  return JSON.parse(JSON.stringify(__plainFromDesign(d))) as Record<string, unknown>;
}

function __ensureStoryDesign(d: unknown): Design {
  return d instanceof Design ? d : new Design(__plainFromDesign(d));
}

function __designId(d: unknown): string {
  return __ensureStoryDesign(d).id;
}

function __pieceCenterPlain(piece: Record<string, unknown>): { u: number; v: number } | undefined {
  const top = piece["center"] as { u?: number; v?: number } | undefined;
  if (top && typeof top.u === "number" && typeof top.v === "number") return { u: top.u, v: top.v };
  const pose = piece["pose"] as { center?: { u?: number; v?: number } } | undefined;
  const pc = pose?.center;
  if (pc && typeof pc.u === "number" && typeof pc.v === "number") return { u: pc.u, v: pc.v };
  return undefined;
}

function __centroidOfPieces(pieces: readonly Record<string, unknown>[]): { u: number; v: number } | undefined {
  const centers = pieces.map((p) => __pieceCenterPlain(p)).filter((c): c is { u: number; v: number } => c !== undefined);
  if (centers.length === 0) return undefined;
  const u = centers.reduce((s, c) => s + c.u, 0) / centers.length;
  const v = centers.reduce((s, c) => s + c.v, 0) / centers.length;
  return { u, v };
}

function __connectionId(c: Record<string, unknown>): string {
  return String(c["id"] ?? "");
}

function __connectionEndpoints(c: Record<string, unknown>): { parentId?: string; childId?: string } {
  const parentId = (c["parent"] as { piece?: { id?: string } } | undefined)?.piece?.id;
  const childId = (c["child"] as { piece?: { id?: string } } | undefined)?.piece?.id;
  return { parentId: parentId ? String(parentId) : undefined, childId: childId ? String(childId) : undefined };
}

function __piecesOfPlain(plain: Record<string, unknown>): Record<string, unknown>[] {
  return [...__itemsOf<Record<string, unknown>>(plain["pieces"])];
}

function __connectionsOfPlain(plain: Record<string, unknown>): Record<string, unknown>[] {
  return [...__itemsOf<Record<string, unknown>>(plain["connections"])];
}

function __applyDesignDiff(plain: Record<string, unknown>, diff: DesignDiff): void {
  const pieces = __piecesOfPlain(plain);
  const byId = new Map<string, Record<string, unknown>>();
  for (const p of pieces) {
    const id = String(p["id"] ?? "");
    if (id) byId.set(id, p);
  }
  const pd = diff.pieces;
  if (pd?.removed?.length) {
    const rm = new Set(pd.removed.map((x) => String((x as { id?: string }).id ?? "")));
    plain["pieces"] = pieces.filter((p) => !rm.has(String(p["id"] ?? "")));
  }
  if (pd?.updated?.length) {
    for (const u of pd.updated) {
      const row = u as { piece?: { id?: string }; diff?: Record<string, unknown> };
      const id = String(row.piece?.id ?? "");
      if (!id) continue;
      const cur = byId.get(id) ?? { id };
      byId.set(id, { ...cur, ...(row.diff ?? {}) });
    }
    plain["pieces"] = Array.from(byId.values());
  }
  if (pd?.added?.length) {
    plain["pieces"] = [...__piecesOfPlain(plain), ...pd.added.map((p) => ({ ...(p as Record<string, unknown>) }))];
  }
  const cd = diff.connections;
  if (cd?.removed?.length) {
    const rm = new Set(cd.removed.map((r) => String((r as { id?: string }).id ?? "")));
    plain["connections"] = __connectionsOfPlain(plain).filter((c) => !rm.has(__connectionId(c)));
  }
  if (cd?.added?.length) {
    plain["connections"] = [...__connectionsOfPlain(plain), ...cd.added.map((c) => ({ ...(c as Record<string, unknown>) }))];
  }
}

function __previewWithDiff(design: unknown, diff: DesignDiff): Record<string, unknown> {
  const merged = __clonePlainDesign(design);
  __applyDesignDiff(merged, diff);
  return merged;
}

class Design {
  constructor(private plain: Record<string, unknown>) {}
  get id(): string {
    return String(this.plain["id"] ?? "");
  }
  get name(): string | undefined {
    return this.plain["name"] as string | undefined;
  }
  get pieces(): readonly Record<string, unknown>[] {
    const p = this.plain["pieces"];
    if (Array.isArray(p)) return p as readonly Record<string, unknown>[];
    return __itemsOf(p) as readonly Record<string, unknown>[];
  }
  get connections(): readonly Record<string, unknown>[] {
    const c = this.plain["connections"];
    if (Array.isArray(c)) return c as readonly Record<string, unknown>[];
    return __itemsOf(c) as readonly Record<string, unknown>[];
  }
  get parent(): { id: string } | undefined {
    const p = this.plain["parent"] as { id?: string } | undefined;
    return p?.id ? { id: String(p.id) } : undefined;
  }
  toPlain(): DesignPlain {
    return this.plain as DesignPlain;
  }
  applyDiff(diff: DesignDiff): void {
    __applyDesignDiff(this.plain, diff);
  }
  static previewWithDiff(design: unknown, diff: DesignDiff): Design {
    return new Design(__previewWithDiff(design, diff));
  }
  dragBySelection(piecesDesign: Design, offset: CoordinatePlain): DesignDiff {
    const ids = new Set(piecesDesign.pieces.map((p) => String(p.id ?? "")));
    const updated = this.pieces
      .filter((p) => ids.has(String(p.id ?? "")))
      .map((p) => {
        const c = __pieceCenterPlain(p) ?? { u: 0, v: 0 };
        return { piece: { id: String(p.id ?? "") }, diff: { center: { u: c.u + offset.u, v: c.v + offset.v } } };
      });
    return { pieces: { updated } };
  }
  deletePiecesAndConnectionsDiff(pieceIds: string[], connectionIds: string[]): DesignDiffOperationResult {
    return {
      ok: true,
      diff: {
        pieces: { removed: pieceIds.map((id) => ({ id })) },
        connections: { removed: connectionIds.map((id) => ({ id })) },
      },
    };
  }
}

export { Design };

/** @emoji 🧾 Story {@link Design} handle (plain JSON model used by algorithm runners). */
export type StoryDesign = Design;

/** @emoji 📋 Builds unique piece/connection id lists from diagram selection fixture JSON. */
export function selectionIdsFromWire(
  wire: DiagramSelectionWire,
  options?: Readonly<{
    omitPieceIds?: readonly string[];
    omitConnectionIds?: readonly string[];
    extraPieceIds?: readonly string[];
    extraConnectionIds?: readonly string[];
  }>,
): { pieceIds: string[]; connectionIds: string[] } {
  const omitP = new Set(options?.omitPieceIds ?? []);
  const omitC = new Set(options?.omitConnectionIds ?? []);
  const pieceIds = Array.from(
    new Set([...(wire.pieces ?? []).map((p) => String(p.id ?? "")).filter(Boolean), ...(options?.extraPieceIds ?? [])].filter((id) => !omitP.has(id))),
  );
  const connectionIds = Array.from(
    new Set([...(wire.connections ?? []).map((c) => String(c.id ?? "")).filter(Boolean), ...(options?.extraConnectionIds ?? [])].filter((id) => !omitC.has(id))),
  );
  return { pieceIds, connectionIds };
}

/** @emoji 🏯 Nakagin copy/paste story selection (omits external-stub piece + link). */
export function nakaginStoryCopySelection(): { pieceIds: string[]; connectionIds: string[] } {
  return selectionIdsFromWire(NakaginCapsuleTowerCopySelection, {
    omitPieceIds: ["31be08e1-e75c-4024-86b4-c3c6d3939fbb"],
    omitConnectionIds: ["b1ecc6c5-722a-4814-9047-a87222bbaa4d"],
    extraPieceIds: ["9c1ec7a2-13c2-4d23-b7bd-1efe2663d0a9", "5feebbf8-33d9-41ad-a13a-24c271a1860b"],
    extraConnectionIds: ["eb8ce9ce-091c-4495-a651-fa703748dfef", "4d5ff333-d70a-43e1-8b7a-8849c8c91405"],
  });
}

/** @emoji 🏯 Nakagin paste-target design row for copy/paste stories. */
export function nakaginPasteTargetDesign(): Design {
  return new Design(NakaginCapsuleTowerPasteDesign as Record<string, unknown>);
}

/** @emoji 📄 Plain design row for {@link mergeKitDesigns}. */
export function storyDesignPlain(design: Design): Record<string, unknown> {
  return design.toPlain() as Record<string, unknown>;
}

/** @emoji 🎯 Piece ids from a pieces-only diagram wire. */
export function pieceIdsFromWire(wire: { pieces?: readonly { id?: string }[] }): string[] {
  return (wire.pieces ?? []).map((p) => String(p.id ?? "")).filter(Boolean);
}
// #endregion 🧱PlainDesignModel

// #region 🧾StoryTypes
export type DesignOperationResult = { ok: true; design: Design; diff: { forward: DesignDiff; reverse: DesignDiff } } | { ok: false; errors: readonly { code: string; message: string }[] };

export type DesignDiffOperationResult = { ok: true; diff: DesignDiff } | { ok: false; errors: readonly { code: string; message: string }[] };

export type OperationResult<T> = { ok: true; value: T } | { ok: false; errors: readonly { code: string; message: string }[] };
// #endregion 🧾StoryTypes

// #region 🌐WasmKitSession
function __toBootstrap(kit: unknown): GqlWireObject {
  return JSON.parse(JSON.stringify(kit)) as GqlWireObject;
}

async function __withJsStore<T>(kit: unknown, fn: (store: JsStore) => Promise<T>): Promise<T> {
	const session = await openSessionInMemory();
	try {
		const stores = await session.stores();
		const store = stores[0];
		if (!store) throw new Error("__withJsStore: session has no stores");
		const installed = await store.installProjection(JSON.stringify(__toBootstrap(kit)));
		if (!installed.ok) {
			throw new Error(`__withJsStore: installProjection failed: ${installed.error?.message ?? "unknown"}`);
		}
		return await fn(store);
	} finally {
		await session.dispose();
	}
}

async function __readFlattenLayout(store: JsStore, designId: string): Promise<readonly { pieceId: string; plane: unknown; center: { u: number; v: number } }[]> {
  const flat = await store.design(designId).flatten();
  if (!flat.ok) throw new Error(flat.error.message);
  const sel = `design(id: ${JSON.stringify(designId)}) { pieces { edges { node { id flatPosition { center { u v } plane { origin { x y z } xAxis { x y z } yAxis { x y z } } } } } } }`;
  const frag = (await store.readKitInner(sel)) as GqlWireObject | null;
  const design = frag?.["design"] as GqlWireObject | undefined;
  const pieces = design?.["pieces"] as GqlWireObject | undefined;
  const edges = (pieces?.["edges"] as readonly GqlWireObject[] | undefined) ?? [];
  const out: { pieceId: string; plane: unknown; center: { u: number; v: number } }[] = [];
  for (const e of edges) {
    const n = e["node"] as GqlWireObject | undefined;
    if (!n) continue;
    const id = String(n["id"] ?? "");
    const fp = n["flatPosition"] as GqlWireObject | undefined;
    const c = fp?.["center"] as GqlWireObject | undefined;
    const plane = fp?.["plane"];
    out.push({ pieceId: id, plane, center: { u: Number(c?.["u"] ?? 0), v: Number(c?.["v"] ?? 0) } });
  }
  return out;
}

function __piecesDiffFromLayout(rows: readonly { pieceId: string; plane: unknown; center: { u: number; v: number } }[]): NonNullable<DesignDiff["pieces"]> {
  return {
    updated: rows.map((r) => ({
      piece: { id: r.pieceId },
      diff: { plane: r.plane as never, center: r.center as never },
    })),
  };
}
// #endregion 🌐WasmKitSession

// #region 🧮KitRunners
function __cloneDesignWithDiff(base: unknown, diff: DesignDiff): Design {
  return Design.previewWithDiff(base, diff);
}

function __pasteCoordinateOffset(srcPieces: readonly Record<string, unknown>[], targetPlain: Record<string, unknown>, options: PasteDesignOptions): CoordinatePlain {
  const coordinate = options.coordinate ?? { u: 0, v: 0 };
  const anchoring = options.anchoring ?? "original";
  if (anchoring === "original") return coordinate;
  const srcCentroid = __centroidOfPieces(srcPieces);
  const tgtPieces = __itemsOf<Record<string, unknown>>(targetPlain["pieces"]);
  const tgtCentroid = __centroidOfPieces(tgtPieces);
  if (!srcCentroid || !tgtCentroid) return coordinate;
  return { u: tgtCentroid.u - srcCentroid.u + coordinate.u, v: tgtCentroid.v - srcCentroid.v + coordinate.v };
}

/**
 * @emoji 🧮 Forward+empty-reverse flatten diff produced from rs-backed `flatPosition` reads.
 */
export async function flattenDesign(kit: unknown, designId: string): Promise<DesignOperationResult> {
  const designs = __listDesignsFromBundle(kit);
  const designPlain = designs.find((d) => String(d["id"] ?? "") === designId);
  if (!designPlain) {
    return { ok: false, errors: [{ code: "flatten.design-not-found", message: `flattenDesign: design ${designId} not found in kit` }] };
  }
  const design = new Design({ ...designPlain });
  try {
    const rows = await __withJsStore(kit, (js) => __readFlattenLayout(js, designId));
    const conns = new Design(designPlain).connections;
    const forward: DesignDiff = {
      pieces: __piecesDiffFromLayout(rows),
      connections: conns.length ? { removed: conns.map((c) => ({ id: String(c["id"] ?? "") })) } : undefined,
    };
    return { ok: true, design, diff: { forward, reverse: {} } };
  } catch (e) {
    return { ok: false, errors: [{ code: "flatten.wasm", message: String(e) }] };
  }
}

/**
 * @emoji 🧮 Flat design used as the display base for input + diff windows (connections preserved).
 */
export async function flatDesign(kit: unknown, designId: string): Promise<Design | null> {
  const result = await flattenDesign(kit, designId);
  if (!result.ok) return null;
  const designPlain = __listDesignsFromBundle(kit).find((d) => String(d["id"] ?? "") === designId);
  if (!designPlain) return null;
  return __cloneDesignWithDiff(new Design({ ...designPlain }), { pieces: result.diff.forward.pieces });
}

/**
 * @emoji 🧮 Fully flattened design (forward diff applied; connections stripped).
 */
export async function flattenedDesign(kit: unknown, designId: string): Promise<Design | null> {
  const result = await flattenDesign(kit, designId);
  if (!result.ok) return null;
  const designPlain = __listDesignsFromBundle(kit).find((d) => String(d["id"] ?? "") === designId);
  if (!designPlain) return null;
  return __cloneDesignWithDiff(new Design({ ...designPlain }), result.diff.forward);
}

/**
 * @emoji 🧮 Delete pieces+connections via plain diff construction.
 */
export async function deletePieces(design: Design, pieceIds: readonly string[], connectionIds: readonly string[]): Promise<DesignDiffOperationResult> {
  return __ensureStoryDesign(design).deletePiecesAndConnectionsDiff([...pieceIds], [...connectionIds]);
}

/**
 * @emoji 🧮 Drag selection on flat centers + re-flatten via WASM for the output preview.
 */
export async function dragPieces(kit: unknown, rawDesign: Design, pieceIds: readonly string[], offset: CoordinatePlain): Promise<{ inputDesign: Design; output: Design; dragDiff: DesignDiff }> {
  const designId = __designId(rawDesign);
  const preRows = await __withJsStore(kit, (js) => __readFlattenLayout(js, designId));
  const prePieceDiff = __piecesDiffFromLayout(preRows);
  const flat = __cloneDesignWithDiff(rawDesign, { pieces: prePieceDiff });
  const flatModel = __ensureStoryDesign(flat);
  const piecesSubset = new Design({
    id: flatModel.id,
    name: flatModel.name,
    pieces: flatModel.pieces.filter((p) => pieceIds.includes(String(p.id ?? ""))),
  });
  const dragDiff = flatModel.dragBySelection(piecesSubset, offset);
  const updatedRaw = __cloneDesignWithDiff(rawDesign, dragDiff);
  const kitPlain = __toBootstrap(kit) as Record<string, unknown>;
  const surface = kitSurface(kitPlain);
  const designs = __itemsOf<Record<string, unknown>>(surface["designs"]).map((d) => (String(d["id"] ?? "") === designId ? __plainFromDesign(updatedRaw) : d));
  surface["designs"] = designs;
  const updatedBundle = { ...kitPlain, wip: { ...(kitPlain["wip"] as object), initialKit: surface } };
  const postRows = await __withJsStore(updatedBundle, (js) => __readFlattenLayout(js, designId));
  const postPieceDiff = __piecesDiffFromLayout(postRows);
  const output = __cloneDesignWithDiff(updatedRaw, { pieces: postPieceDiff });
  return { inputDesign: flat, output, dragDiff };
}

/**
 * @emoji 🧮 Move preview: approximates joint motion by re-flattening after nudging selected flat centers using gap/shift/rise as u/v offsets (story fidelity only).
 */
export async function movePieces(kit: unknown, rawDesign: Design, pieceIds: readonly string[], vector: MoveVector): Promise<{ inputDesign: Design; output: Design; moveDiff: DesignDiff }> {
  const designId = __designId(rawDesign);
  const preRows = await __withJsStore(kit, (js) => __readFlattenLayout(js, designId));
  const prePieceDiff = __piecesDiffFromLayout(preRows);
  const flat = __cloneDesignWithDiff(rawDesign, { pieces: prePieceDiff });
  const preById = new Map(preRows.map((r) => [r.pieceId, r] as const));
  const moveDiff: DesignDiff = {
    pieces: {
      updated: pieceIds
        .map((id) => {
          const a = preById.get(id)?.center;
          if (!a) return null;
          return { piece: { id }, diff: { center: { u: a.u + vector.shift, v: a.v + vector.gap } } };
        })
        .filter((x): x is NonNullable<typeof x> => x != null),
    },
  };
  const updatedRaw = __cloneDesignWithDiff(rawDesign, moveDiff);
  const kitPlain = __toBootstrap(kit) as Record<string, unknown>;
  const surface = kitSurface(kitPlain);
  const designs = __itemsOf<Record<string, unknown>>(surface["designs"]).map((d) => (String(d["id"] ?? "") === designId ? __plainFromDesign(updatedRaw) : d));
  surface["designs"] = designs;
  const updatedBundle = { ...kitPlain, wip: { ...(kitPlain["wip"] as object), initialKit: surface } };
  const postRows = await __withJsStore(updatedBundle, (js) => __readFlattenLayout(js, designId));
  const postPieceDiff = __piecesDiffFromLayout(postRows);
  const output = __cloneDesignWithDiff(updatedRaw, { pieces: postPieceDiff });
  return { inputDesign: flat, output, moveDiff };
}

/**
 * @emoji 🧮 Copy a selection into an isolated clipboard design (plain JSON).
 */
export async function copyDesign(design: Design, pieceIds: readonly string[], connectionIds: readonly string[]): Promise<OperationResult<Design>> {
  const src = __plainFromDesign(design);
  const pieceSet = new Set(pieceIds.map(String));
  const connSet = new Set(connectionIds.map(String));
  const allPieces = __itemsOf<Record<string, unknown>>(src["pieces"]);
  const allConnections = __itemsOf<Record<string, unknown>>(src["connections"]);
  for (const c of allConnections) {
    if (!connSet.has(__connectionId(c))) continue;
    const { parentId, childId } = __connectionEndpoints(c);
    if (parentId) pieceSet.add(parentId);
    if (childId) pieceSet.add(childId);
  }
  if (pieceSet.size > 0) {
    for (const c of allConnections) {
      const { parentId, childId } = __connectionEndpoints(c);
      if (parentId && childId && pieceSet.has(parentId) && pieceSet.has(childId)) connSet.add(__connectionId(c));
    }
  }
  return {
    ok: true,
    value: new Design({
      id: `${String(src["id"] ?? "design")}-clipboard`,
      name: `${String(src["name"] ?? "Design")} (clipboard)`,
      pieces: allPieces.filter((p) => pieceSet.has(String(p["id"] ?? ""))),
      connections: allConnections.filter((c) => connSet.has(__connectionId(c))),
    }),
  };
}

/**
 * @emoji 🧮 Paste clipboard design onto target (anchoring + coordinate shift; plain diff).
 */
export async function pasteDesign(source: Design, target: Design, options: PasteDesignOptions = {}): Promise<DesignDiff> {
  const src = __plainFromDesign(source);
  const tgt = __plainFromDesign(target);
  const srcPieces = __itemsOf<Record<string, unknown>>(src["pieces"]);
  const offset = __pasteCoordinateOffset(srcPieces, tgt, options);
  const added = srcPieces.map((p) => {
    const row = { ...p };
    const c = __pieceCenterPlain(row);
    if (!c) return row;
    const shifted = { u: c.u + offset.u, v: c.v + offset.v };
    row["center"] = shifted;
    const pose = row["pose"] as { center?: { u: number; v: number } } | undefined;
    if (pose?.center) row["pose"] = { ...pose, center: shifted };
    return row;
  });
  const connectionsAdded = __itemsOf<Record<string, unknown>>(src["connections"]).map((c) => ({ ...c }));
  return {
    pieces: { added },
    ...(connectionsAdded.length ? { connections: { added: connectionsAdded } } : {}),
  } as DesignDiff;
}
// #endregion 🧮KitRunners

// #region 🪝StoryProxies
/** @emoji 🔎 Finds a design row by id inside a kit bundle surface. */
export function designFromKit(kit: unknown, designId: string): Record<string, unknown> | undefined {
  return __listDesignsFromBundle(kit).find((d) => String(d["id"] ?? "") === designId);
}

/** @emoji 📚 Lists type rows from a kit bundle surface. */
export function typesFromKit(kit: unknown): readonly { id: string; name?: string }[] {
  return __itemsOf<Record<string, unknown>>(kitSurface(kit)["types"]).map((t) => ({
    id: String(t["id"] ?? ""),
    name: t["name"] as string | undefined,
  }));
}

/** @emoji 📚 Lists design rows from a kit bundle surface. */
export function designsFromKit(kit: unknown): readonly Record<string, unknown>[] {
  return __listDesignsFromBundle(kit);
}

/** @emoji 🧩 Merges extra design rows into a kit bundle for Storybook presets. */
export function mergeKitDesigns(kit: unknown, ...extraDesigns: Record<string, unknown>[]): unknown {
  const root = kit as { wip?: { initialKit?: Record<string, unknown> } };
  const surface = { ...kitSurface(kit) };
  const designs = [...__itemsOf<Record<string, unknown>>(surface["designs"]), ...extraDesigns];
  surface["designs"] = designs;
  return { ...(kit as object), wip: { ...root.wip, initialKit: surface } };
}

/** @emoji 📐 WASM flatten layout diff for {@link AlgorithmContextValue.diagramLayoutDiff}. */
export async function flattenDiagramLayoutDiff(kit: unknown, designId: string): Promise<DesignDiff | undefined> {
  const flattenRes = await flattenDesign(kit, designId);
  return flattenRes.ok && flattenRes.diff.forward.pieces ? { pieces: flattenRes.diff.forward.pieces } : undefined;
}

/** @emoji 📐 Loads flat design preview plus optional diagram layout diff from WASM flatten. */
export async function loadFlatDesignBundle(kit: unknown, designId: string): Promise<{ flat: Design | null; diagramLayoutDiff: DesignDiff | undefined }> {
  const [flat, diagramLayoutDiff] = await Promise.all([flatDesign(kit, designId), flattenDiagramLayoutDiff(kit, designId)]);
  return { flat, diagramLayoutDiff };
}

/** @emoji 🔀 Applies a {@link DesignDiff} onto a design row; returns fallback when diff is absent. */
export function previewDesignWithAppliedDiff(design: unknown, diff: DesignDiff | undefined, fallback: unknown): Design {
  if (!diff) return __ensureStoryDesign(design ?? fallback);
  return Design.previewWithDiff(design ?? fallback, diff);
}

/** @emoji 🎯 Keeps piece selection ids that still exist on the preset design. */
export function reconcilePieceSelectionIds(rawDesign: { pieces?: readonly { id?: string }[] }, prev: readonly string[], fallbackIds: readonly string[]): string[] {
  const pieceIds = new Set((rawDesign.pieces ?? []).map((p) => String(p.id ?? "")).filter(Boolean));
  const filtered = prev.filter((g) => pieceIds.has(g));
  return filtered.length > 0 ? [...filtered] : [...fallbackIds];
}

function __useCancelledEffect(effect: (isCancelled: () => boolean) => void | Promise<void>, deps: React.DependencyList): void {
  React.useEffect(() => {
    let cancelled = false;
    void effect(() => cancelled);
    return () => {
      cancelled = true;
    };
  }, deps);
}

/** @emoji 📐 Story hook: WASM flat design + diagram layout diff for algorithm input boards. */
export function useFlatDesignPreview(kit: unknown, designId: string) {
  const [flatInputDesign, setFlatInputDesign] = React.useState<Design | null>(null);
  const [diagramLayoutDiff, setDiagramLayoutDiff] = React.useState<DesignDiff | undefined>(undefined);
  const [loading, setLoading] = React.useState(true);

  __useCancelledEffect(async (isCancelled) => {
    setLoading(true);
    setFlatInputDesign(null);
    setDiagramLayoutDiff(undefined);
    const bundle = await loadFlatDesignBundle(kit, designId);
    if (isCancelled()) return;
    setFlatInputDesign(bundle.flat);
    setDiagramLayoutDiff(bundle.diagramLayoutDiff);
    setLoading(bundle.flat === null);
  }, [kit, designId]);

  return { flatInputDesign, diagramLayoutDiff, loading, ready: flatInputDesign !== null };
}

/** @emoji 📐 Story hook: flatten IPO trio (flat base, flattened output, forward diff). */
export function useFlattenPreview(kit: unknown, designId: string) {
  const [flatPreview, setFlatPreview] = React.useState<Design | null>(null);
  const [flattenedPreview, setFlattenedPreview] = React.useState<Design | null>(null);
  const [flattenDiff, setFlattenDiff] = React.useState<DesignDiff | undefined>(undefined);
  const [loading, setLoading] = React.useState(true);

  __useCancelledEffect(async (isCancelled) => {
    setLoading(true);
    setFlatPreview(null);
    setFlattenedPreview(null);
    setFlattenDiff(undefined);
    const [flatResult, flattenedResult, flattenResult] = await Promise.all([flatDesign(kit, designId), flattenedDesign(kit, designId), flattenDesign(kit, designId)]);
    if (isCancelled()) return;
    setFlatPreview(flatResult);
    setFlattenedPreview(flattenedResult);
    setFlattenDiff(flattenResult.ok ? flattenResult.diff.forward : undefined);
    setLoading(!flatResult || !flattenedResult || !flattenResult.ok);
  }, [kit, designId]);

  return { flatPreview, flattenedPreview, flattenDiff, loading, ready: flatPreview !== null && flattenedPreview !== null && flattenDiff !== undefined };
}

/** @emoji 🎯 Story hook: piece ids with reconcile after flat preset load. */
export function useAlgorithmPieceSelection(initialPieceIds: readonly string[], rawDesign?: { pieces?: readonly { id?: string }[] }, fallbackPieceIds?: readonly string[]) {
  const [selectedPieceIds, setSelectedPieceIds] = React.useState<string[]>([...initialPieceIds]);
  const reconcile = React.useCallback((prev: string[]) => reconcilePieceSelectionIds(rawDesign ?? {}, prev, fallbackPieceIds ?? initialPieceIds), [fallbackPieceIds, initialPieceIds, rawDesign]);
  return { selectedPieceIds, setSelectedPieceIds, reconcile };
}

/** @emoji 🎯 Reconciles piece selection once flat WASM preview is ready. */
export function useReconciledPieceSelection(initialPieceIds: readonly string[], rawDesign: { pieces?: readonly { id?: string }[] }, fallbackPieceIds: readonly string[], ready: boolean) {
  const { selectedPieceIds, setSelectedPieceIds, reconcile } = useAlgorithmPieceSelection(initialPieceIds, rawDesign, fallbackPieceIds);
  React.useEffect(() => {
    if (!ready) return;
    setSelectedPieceIds((prev) => reconcile(prev));
  }, [ready, reconcile, setSelectedPieceIds]);
  return { selectedPieceIds, setSelectedPieceIds };
}

/** @emoji 🧩 Kit bundle with one extra design row from a {@link Design} instance. */
export function mergeKitWithStoryDesign(kit: unknown, design: Design): unknown {
  return mergeKitDesigns(kit, storyDesignPlain(design));
}

/** @emoji ⚡ Story hook: runs an async algorithm op when deps change; surfaces loading and errors. */
export function useAlgorithmAsyncRun<T>(enabled: boolean, run: () => Promise<T>, deps: React.DependencyList): { result: T | undefined; loading: boolean; error: string | undefined } {
  const [result, setResult] = React.useState<T | undefined>(undefined);
  const [loading, setLoading] = React.useState(false);
  const [error, setError] = React.useState<string | undefined>(undefined);
  const runRef = React.useRef(run);
  runRef.current = run;

  __useCancelledEffect(
    async (isCancelled) => {
      if (!enabled) {
        setResult(undefined);
        setLoading(false);
        setError(undefined);
        return;
      }
      setResult(undefined);
      setLoading(true);
      setError(undefined);
      try {
        const value = await runRef.current();
        if (!isCancelled()) {
          setResult(value);
          setLoading(false);
        }
      } catch (e) {
        if (!isCancelled()) {
          setResult(undefined);
          setLoading(false);
          setError(e instanceof Error ? e.message : String(e));
        }
      }
    },
    [enabled, ...deps],
  );

  return { result, loading, error };
}

/** @emoji 📋 Story hook: copy/paste pipeline for IPO copy & paste boards. */
export function useCopyPastePreview(params: {
  kit: unknown;
  kitWithTarget: unknown;
  sourceDesignId: string;
  targetDesignId: string;
  pasteTarget: Design;
  selectedPieceIds: readonly string[];
  selectedConnectionIds: readonly string[];
  mode: "with" | "without";
  vec: VecValue;
  pasteAnchoring: PasteDesignAnchoringKind;
}) {
  const source = useFlatDesignPreview(params.kit, params.sourceDesignId);
  const target = useFlatDesignPreview(params.kitWithTarget, params.targetDesignId);
  const ready = source.ready && target.ready;
  const hasSelection = params.selectedPieceIds.length > 0 || params.selectedConnectionIds.length > 0;
  const { result: designDiff, loading: runLoading, error } = useAlgorithmAsyncRun(
    ready && hasSelection,
    async () => {
      if (!source.flatInputDesign) return undefined;
      const copyRes = await copyDesign(source.flatInputDesign, params.selectedPieceIds, params.selectedConnectionIds);
      if (!copyRes.ok) return undefined;
      const coordinate = params.mode === "with" ? { u: params.vec.u, v: params.vec.v } : undefined;
      return pasteDesign(copyRes.value, params.pasteTarget, { anchoring: params.pasteAnchoring, coordinate });
    },
    [source.flatInputDesign, params.selectedPieceIds, params.selectedConnectionIds, params.pasteAnchoring, params.mode, params.mode === "with" ? params.vec.u : 0, params.mode === "with" ? params.vec.v : 0],
  );
  const outputDesign = previewDesignWithAppliedDiff(target.flatInputDesign, designDiff, params.pasteTarget);
  return {
    source,
    target,
    designDiff,
    outputDesign,
    loading: source.loading || target.loading,
    runLoading,
    error,
    ready,
    hasSelection,
  };
}
// #endregion 🪝StoryProxies

// #region 🧪EmbeddedTests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("semio-algorithms public surface", () => {
    it("exports async kit runners used by semio algorithm stories", () => {
      expect([flattenDesign, flatDesign, flattenedDesign, deletePieces, dragPieces, movePieces, copyDesign, pasteDesign].every((f) => typeof f === "function")).toBe(true);
    });
    it("exposes only rs-backed story helpers and no native adapter API", () => {
      const publicHelperNames = ["flattenDesign", "flatDesign", "flattenedDesign", "deletePieces", "dragPieces", "movePieces", "copyDesign", "pasteDesign"];
      expect(publicHelperNames.every((name) => !name.toLowerCase().includes("native"))).toBe(true);

      type NativeAdapterExport = "nativeFlattenDesign" | "nativeFlatDesign" | "nativeFlattenedDesign" | "nativeDeletePieces" | "nativeDragPieces" | "nativeMovePieces" | "NativeAlgorithmLanguage";
      type ModuleExports = keyof typeof import("./index");
      type MustNotExposeNativeAdapters = NativeAdapterExport extends ModuleExports ? never : true;
      const _compileAssert: MustNotExposeNativeAdapters = true;
      expect(_compileAssert).toBe(true);
    });

    it("copyDesign returns clipboard design on value and pasteDesign emits added rows", async () => {
      const source = new Design({
        id: "src",
        pieces: [
          { id: "p1", center: { u: 1, v: 2 } },
          { id: "p2", center: { u: 3, v: 4 } },
        ],
        connections: [{ id: "c1", parent: { piece: { id: "p1" } }, child: { piece: { id: "p2" } } }],
      });
      const target = new Design({ id: "tgt", pieces: [{ id: "p0", center: { u: 0, v: 0 } }], connections: [] });
      const copyRes = await copyDesign(source, ["p1"], ["c1"]);
      expect(copyRes.ok).toBe(true);
      if (!copyRes.ok) return;
      expect("diff" in copyRes).toBe(false);
      expect(copyRes.value.pieces.map((p) => String(p.id))).toEqual(["p1", "p2"]);
      expect(copyRes.value.connections.map((c) => String(c.id))).toEqual(["c1"]);
      const pasteDiff = await pasteDesign(copyRes.value, target, { anchoring: "original", coordinate: { u: 5, v: 6 } });
      expect(pasteDiff.pieces?.added?.length).toBe(2);
      expect(pasteDiff.connections?.added?.length).toBe(1);
    });

    it("previewDesignWithAppliedDiff applies diff onto base design", () => {
      const base = new Design({ id: "b", pieces: [{ id: "p1", center: { u: 0, v: 0 } }], connections: [] });
      const diff: DesignDiff = { pieces: { added: [{ id: "p2", center: { u: 1, v: 1 } }] } };
      const out = previewDesignWithAppliedDiff(base, diff, base);
      expect(out.pieces.map((p) => String(p.id))).toEqual(["p1", "p2"]);
    });

    it("copyDesign includes internal links when only pieces are selected", async () => {
      const source = new Design({
        id: "src",
        pieces: [
          { id: "p1", center: { u: 0, v: 0 } },
          { id: "p2", center: { u: 1, v: 1 } },
        ],
        connections: [{ id: "c1", parent: { piece: { id: "p1" } }, child: { piece: { id: "p2" } } }],
      });
      const copyRes = await copyDesign(source, ["p1", "p2"], []);
      expect(copyRes.ok).toBe(true);
      if (!copyRes.ok) return;
      expect(copyRes.value.connections.length).toBe(1);
    });
  });
}
// #endregion 🧪EmbeddedTests
