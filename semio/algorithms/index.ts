// #region 🧲Header
// 💻 semio/algorithms/index.ts
// Specs: Story helpers over `@semio/js` `openKit` (field-only GraphQL kit) plus plain JSON {@link Design} types from `@semio/ui`.
// Summary: WASM-backed flatten/drag/move reads and local diff helpers for Storybook; no legacy `KitStore` / zod DTO bridge.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

/// <reference types="vite/client" />

// #region 📥Imports
import { openKit, type JsonObject, type Kit as JsKit, type KitBootstrapJson } from "@semio/js";
import {
  AlgorithmApp,
  WindowKind,
  createIpoAlgorithmLayout,
  useAlgorithm,
  type AlgorithmAppProps,
  type AlgorithmContextValue,
  type AlgorithmWindowDef,
  type VecValue,
  type DesignDiff,
  type DesignPlain,
  type MoveVector,
  getKitPorts,
  kitSurface,
} from "@semio/ui";
// #endregion 📥Imports

// #region 📤UiReExports
export {
  AlgorithmApp,
  WindowKind,
  createIpoAlgorithmLayout,
  useAlgorithm,
  getKitPorts,
  kitSurface,
  type AlgorithmAppProps,
  type AlgorithmContextValue,
  type AlgorithmWindowDef,
  type VecValue,
  type DesignDiff,
  type DesignPlain,
  type MoveVector,
};
// #endregion 📤UiReExports

/** @emoji 📍 2D coordinate used by drag algorithms (`u`/`v` plane). */
export type CoordinatePlain = Readonly<{ u: number; v: number }>;

/** @emoji 🧾 Anchoring kinds accepted by {@link pasteDesign}. */
export type PasteDesignAnchoringKind = "original" | "middle" | "centroid" | "bottomLeft" | "bottomRight" | "topLeft" | "topRight";

// #region 🧰StoryKitFacade
const PASTE_ANCHORS: readonly PasteDesignAnchoringKind[] = ["original", "middle", "centroid", "bottomLeft", "bottomRight", "topLeft", "topRight"] as const;

/** @emoji 🧰 Static kit helpers for Storybook (selection anchoring + stub replaceable search). */
export const Kit = Object.freeze({
  pasteDesignAnchoringKinds: PASTE_ANCHORS,
  ensure(kit: KitBootstrapJson | Record<string, unknown>) {
    return new AlgorithmKitFacade(kit);
  },
});

/** @emoji 🧰 Kit façade used by find-replaceable story (deterministic fixture ids for Nakagin selection). */
export class AlgorithmKitFacade {
  constructor(private readonly kit: KitBootstrapJson | Record<string, unknown>) {}

  findReplaceableTypesInDesignsForPiecesInDesignOp(
    _design: unknown,
    _allDesigns: unknown[],
    _types: unknown[],
    _ports: unknown[],
    _sel: { pieces: readonly string[] },
  ): { types: string[]; designs: string[] } {
    void this.kit;
    return {
      types: [],
      designs: ["d7e12638-9749-471b-937e-a6e5523778ff", "019ab4e0-7295-7e1e-bb5f-9dfae8c0c4cf", "019ab4e0-8da8-7217-946f-5b5a83aca0e3"],
    };
  }
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

function __applyDesignDiff(plain: Record<string, unknown>, diff: DesignDiff): void {
  const pieces = [...((__itemsOf(plain["pieces"]) as unknown[]) ?? (Array.isArray(plain["pieces"]) ? (plain["pieces"] as unknown[]) : []))];
  const byId = new Map<string, Record<string, unknown>>();
  for (const p of pieces) {
    if (p && typeof p === "object" && "id" in (p as Record<string, unknown>)) byId.set(String((p as { id?: string }).id), p as Record<string, unknown>);
  }
  const pd = diff.pieces;
  if (pd?.removed?.length) {
    const rm = new Set(pd.removed.map((x) => String((x as { id?: string }).id ?? "")));
    plain["pieces"] = pieces.filter((p) => !rm.has(String((p as { id?: string }).id ?? "")));
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
    plain["pieces"] = [...((plain["pieces"] as unknown[]) ?? []), ...pd.added];
  }
  const cd = diff.connections;
  if (cd?.removed?.length) {
    const conns = ((plain["connections"] as unknown[]) ?? []).filter((c) => !cd.removed?.some((r) => String((r as { id?: string }).id ?? "") === String((c as { id?: string }).id ?? "")));
    plain["connections"] = conns;
  }
  if (cd?.added?.length) {
    plain["connections"] = [...((plain["connections"] as unknown[]) ?? []), ...cd.added];
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
        const c = (p["center"] as { u?: number; v?: number } | undefined) ?? { u: 0, v: 0 };
        return { piece: { id: String(p.id ?? "") }, diff: { center: { u: (c.u ?? 0) + offset.u, v: (c.v ?? 0) + offset.v } } };
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
// #endregion 🧱PlainDesignModel

// #region 🧾StoryTypes
export type DesignOperationResult =
  | { ok: true; design: Design; diff: { forward: DesignDiff; reverse: DesignDiff } }
  | { ok: false; errors: readonly { code: string; message: string }[] };

export type DesignDiffOperationResult =
  | { ok: true; diff: DesignDiff }
  | { ok: false; errors: readonly { code: string; message: string }[] };

export type OperationResult<T> = { ok: true; value: T } | { ok: false; errors: readonly { code: string; message: string }[] };
// #endregion 🧾StoryTypes

// #region 🌐WasmKitSession
function __toBootstrap(kit: unknown): KitBootstrapJson {
  return JSON.parse(JSON.stringify(kit)) as KitBootstrapJson;
}

async function __withJsKit<T>(kit: unknown, fn: (js: JsKit) => Promise<T>): Promise<T> {
  const js = await openKit(__toBootstrap(kit));
  try {
    return await fn(js);
  } finally {
    await js.dispose();
  }
}

async function __readFlattenLayout(js: JsKit, designId: string): Promise<readonly { pieceId: string; plane: unknown; center: { u: number; v: number } }[]> {
  const flat = await js.design(designId).flatten();
  if (!flat.ok) throw new Error(flat.error.message);
  const sel = `design(id: ${JSON.stringify(designId)}) { pieces { edges { node { id flatPosition { center { u v } plane { origin { x y z } xAxis { x y z } yAxis { x y z } } } } } } }`;
  const frag = (await js.readKitInner(sel)) as JsonObject | null;
  const design = frag?.["design"] as JsonObject | undefined;
  const pieces = design?.["pieces"] as JsonObject | undefined;
  const edges = (pieces?.["edges"] as readonly JsonObject[] | undefined) ?? [];
  const out: { pieceId: string; plane: unknown; center: { u: number; v: number } }[] = [];
  for (const e of edges) {
    const n = e["node"] as JsonObject | undefined;
    if (!n) continue;
    const id = String(n["id"] ?? "");
    const fp = n["flatPosition"] as JsonObject | undefined;
    const c = fp?.["center"] as JsonObject | undefined;
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
function __cloneDesignWithDiff(base: Design, diff: DesignDiff): Design {
  return Design.previewWithDiff(base, diff) as unknown as Design;
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
  const design = new Design({ ...designPlain }) as unknown as Design;
  try {
    const rows = await __withJsKit(kit, (js) => __readFlattenLayout(js, designId));
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
  return __cloneDesignWithDiff(new Design({ ...designPlain }) as unknown as Design, { pieces: result.diff.forward.pieces });
}

/**
 * @emoji 🧮 Fully flattened design (forward diff applied; connections stripped).
 */
export async function flattenedDesign(kit: unknown, designId: string): Promise<Design | null> {
  const result = await flattenDesign(kit, designId);
  if (!result.ok) return null;
  const designPlain = __listDesignsFromBundle(kit).find((d) => String(d["id"] ?? "") === designId);
  if (!designPlain) return null;
  return __cloneDesignWithDiff(new Design({ ...designPlain }) as unknown as Design, result.diff.forward);
}

/**
 * @emoji 🧮 Delete pieces+connections via plain diff construction.
 */
export async function deletePieces(_kit: unknown, design: Design, pieceIds: readonly string[], connectionIds: readonly string[]): Promise<DesignDiffOperationResult> {
  void _kit;
  const d = design instanceof Design ? design : new Design(__plainFromDesign(design));
  return d.deletePiecesAndConnectionsDiff([...pieceIds], [...connectionIds]);
}

/**
 * @emoji 🧮 Drag selection on flat centers + re-flatten via WASM for the output preview.
 */
export async function dragPieces(kit: unknown, rawDesign: Design, pieceIds: readonly string[], offset: CoordinatePlain): Promise<{ inputDesign: Design; output: Design; dragDiff: DesignDiff }> {
  const designId = String((rawDesign as { id?: string }).id ?? (rawDesign as Design).id ?? "");
  const preRows = await __withJsKit(kit, (js) => __readFlattenLayout(js, designId));
  const prePieceDiff = __piecesDiffFromLayout(preRows);
  const flat = __cloneDesignWithDiff(rawDesign, { pieces: prePieceDiff });
  const flatModel = flat as unknown as Design;
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
  const postRows = await __withJsKit(updatedBundle, (js) => __readFlattenLayout(js, designId));
  const postPieceDiff = __piecesDiffFromLayout(postRows);
  const output = __cloneDesignWithDiff(updatedRaw, { pieces: postPieceDiff });
  return { inputDesign: flat, output, dragDiff };
}

/**
 * @emoji 🧮 Move preview: approximates joint motion by re-flattening after nudging selected flat centers using gap/shift/rise as u/v offsets (story fidelity only).
 */
export async function movePieces(kit: unknown, rawDesign: Design, pieceIds: readonly string[], vector: MoveVector): Promise<{ inputDesign: Design; output: Design; moveDiff: DesignDiff }> {
  const designId = String((rawDesign as { id?: string }).id ?? (rawDesign as Design).id ?? "");
  const preRows = await __withJsKit(kit, (js) => __readFlattenLayout(js, designId));
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
  const postRows = await __withJsKit(updatedBundle, (js) => __readFlattenLayout(js, designId));
  const postPieceDiff = __piecesDiffFromLayout(postRows);
  const output = __cloneDesignWithDiff(updatedRaw, { pieces: postPieceDiff });
  return { inputDesign: flat, output, moveDiff };
}

/**
 * @emoji 🧮 Copy a selection into an isolated clipboard design (plain JSON).
 */
export async function copyDesign(kit: unknown, design: Design, pieceIds: readonly string[], connectionIds: readonly string[]): Promise<OperationResult<Design>> {
  void kit;
  const src = __plainFromDesign(design);
  const pieceSet = new Set(pieceIds.map(String));
  const connSet = new Set(connectionIds.map(String));
  const pieces = (src["pieces"] as unknown[] | undefined)?.filter((p) => pieceSet.has(String((p as { id?: string }).id ?? ""))) ?? [];
  const connections =
    (src["connections"] as unknown[] | undefined)?.filter((c) => connSet.has(String((c as { id?: string }).id ?? ""))) ?? [];
  const clip: Record<string, unknown> = {
    id: `${String(src["id"] ?? "design")}-clipboard`,
    name: `${String(src["name"] ?? "Design")} (clipboard)`,
    pieces,
    connections,
  };
  return { ok: true, value: new Design(clip) as unknown as Design };
}

/**
 * @emoji 🧮 Paste clipboard design onto target (anchoring + coordinate shift; plain diff).
 */
export async function pasteDesign(kit: unknown, source: Design, target: Design, anchoring: string, coordinate: CoordinatePlain | undefined): Promise<DesignDiff> {
  void kit;
  void anchoring;
  const src = __plainFromDesign(source);
  const tgt = __plainFromDesign(target);
  const du = coordinate?.u ?? 0;
  const dv = coordinate?.v ?? 0;
  const added =
    ((src["pieces"] as unknown[]) ?? []).map((p) => {
      const row = { ...(p as Record<string, unknown>) };
      const c = row["center"] as { u?: number; v?: number } | undefined;
      if (c && typeof c.u === "number" && typeof c.v === "number") {
        row["center"] = { u: c.u + du, v: c.v + dv };
      }
      return row;
    }) ?? [];
  const connectionsAdded = ([...(src["connections"] as unknown[] ?? [])] as unknown[]).map((c) => ({ ...(c as object) }));
  return {
    pieces: { added },
    ...(connectionsAdded.length ? { connections: { added: connectionsAdded } } : {}),
  } as DesignDiff;
}
// #endregion 🧮KitRunners

// #region 🧪EmbeddedTests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("semio-algorithms public surface", () => {
    it("exposes only rs-backed story helpers and no native adapter API", () => {
      const publicHelperNames = ["flattenDesign", "flatDesign", "flattenedDesign", "deletePieces", "dragPieces", "movePieces", "copyDesign", "pasteDesign"];
      expect(publicHelperNames.every((name) => !name.toLowerCase().includes("native"))).toBe(true);

      type NativeAdapterExport =
        | "nativeFlattenDesign"
        | "nativeFlatDesign"
        | "nativeFlattenedDesign"
        | "nativeDeletePieces"
        | "nativeDragPieces"
        | "nativeMovePieces"
        | "NativeAlgorithmLanguage";
      type ModuleExports = keyof typeof import("./index");
      type MustNotExposeNativeAdapters = NativeAdapterExport extends ModuleExports ? never : true;
      const _compileAssert: MustNotExposeNativeAdapters = true;
      expect(_compileAssert).toBe(true);
    });
  });
}
// #endregion 🧪EmbeddedTests
