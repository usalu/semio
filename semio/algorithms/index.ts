// #region 🧲Header
// 💻 semio/algorithms/index.ts
// Specs: Re-exports from @semio/js, @semio/react, @semio/ui (all backed by semio/rs as single source of truth).
// Summary: Algorithms package is a re-export layer over rs WASM (`KitStore`) plus tiny story helpers; no cross-language adapters.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

/// <reference types="vite/client" />

// #region 📥Imports
import { KitStore, type KitFullDto as WasmKitFullDto } from "@semio/js";
import type {
  CoordinatePlain as Coordinate,
  Design,
  DesignDiff,
  DesignDiffOperationResult,
  DesignOperationResult,
  DesignPlain,
  Kit,
  MoveVector,
  OperationResult,
} from "@semio/react";
import { Design as DesignEntity, Kit as KitEntity, asKitInstance } from "@semio/react";
// #endregion 📥Imports

// #region 📤UiReExports
export { AlgorithmApp, WindowKind, createIpoAlgorithmLayout, useAlgorithm, type AlgorithmAppProps, type AlgorithmContextValue, type AlgorithmWindowDef, type VecValue } from "@semio/ui";
// #endregion 📤UiReExports

// #region 🧮KitStoreRunners
// @emoji 🧮 Story-only async helpers. All paths route through `semio/rs` WASM via `@semio/js` `KitStore`; entity helpers (`Design`, `Kit`) live in `@semio/react`.

function cloneDesignWithDiff(base: Design, diff: DesignDiff): Design {
  const plain = (base as DesignEntity).toPlain?.() ?? (JSON.parse(JSON.stringify(base)) as DesignPlain);
  const d = new DesignEntity(plain);
  d.applyDiff(diff);
  return d;
}

async function openWasmKit(kit: Kit): Promise<KitStore> {
  const dto = JSON.parse(JSON.stringify(asKitInstance(kit).toJSON())) as WasmKitFullDto;
  return KitStore.open(dto);
}

type FlattenRow = { pieceId: string; plane: unknown; center: unknown };

async function readFlattenRows(ks: KitStore, designId: string): Promise<readonly FlattenRow[]> {
  return (await ks.readDesignFlattenMap(designId)) as FlattenRow[];
}

function piecesDiffFromFlattenRows(rows: readonly FlattenRow[]): NonNullable<DesignDiff["pieces"]> {
  return {
    updated: rows.map((r) => ({
      piece: { id: r.pieceId },
      diff: { plane: r.plane as never, center: r.center as never },
    })),
  };
}

/**
 * @emoji 🧮 Forward+empty-reverse flatten diff produced from `semio/rs` `flatten_map`.
 */
export async function flattenDesign(kit: Kit, designId: string): Promise<DesignOperationResult> {
  const design = asKitInstance(kit).designs?.find((d) => d.id === designId);
  if (!design) {
    return { ok: false, errors: [{ code: "flatten.design-not-found", message: `flattenDesign: design ${designId} not found in kit` }] };
  }
  let ks: KitStore | undefined;
  try {
    ks = await openWasmKit(kit);
    const rows = await readFlattenRows(ks, designId);
    const conns = design.connections ?? [];
    const forward: DesignDiff = {
      pieces: piecesDiffFromFlattenRows(rows),
      connections: conns.length ? { removed: conns.map((c) => ({ id: c.id })) } : undefined,
    };
    return { ok: true, design, diff: { forward, reverse: {} } };
  } catch (e) {
    return { ok: false, errors: [{ code: "flatten.wasm", message: String(e) }] };
  } finally {
    if (ks) await ks.dispose();
  }
}

/**
 * @emoji 🧮 Flat design used as the display base for input + diff windows (connections preserved).
 */
export async function flatDesign(kit: Kit, designId: string): Promise<Design | null> {
  const result = await flattenDesign(kit, designId);
  if (!result.ok) return null;
  const design = (kit.designs ?? []).find((d) => d.id === designId);
  if (!design) return null;
  return cloneDesignWithDiff(design, { pieces: result.diff.forward.pieces });
}

/**
 * @emoji 🧮 Fully flattened design (forward diff applied; connections stripped).
 */
export async function flattenedDesign(kit: Kit, designId: string): Promise<Design | null> {
  const result = await flattenDesign(kit, designId);
  if (!result.ok) return null;
  const design = (kit.designs ?? []).find((d) => d.id === designId);
  if (!design) return null;
  return cloneDesignWithDiff(design, result.diff.forward);
}

/**
 * @emoji 🧮 Delete pieces+connections via `Design.deletePiecesAndConnectionsDiff`.
 */
export async function deletePieces(_kit: Kit, design: Design, pieceIds: readonly string[], connectionIds: readonly string[]): Promise<DesignDiffOperationResult> {
  const d = design instanceof DesignEntity ? design : new DesignEntity(design as DesignPlain);
  return d.deletePiecesAndConnectionsDiff([...pieceIds], [...connectionIds]);
}

/**
 * @emoji 🧮 Drag selection on flat centers + re-flatten via WASM for the output preview.
 */
export async function dragPieces(kit: Kit, rawDesign: Design, pieceIds: readonly string[], offset: Coordinate): Promise<{ inputDesign: Design; output: Design; dragDiff: DesignDiff }> {
  const designId = rawDesign.id;
  let ks0: KitStore | undefined;
  let ks1: KitStore | undefined;
  try {
    ks0 = await openWasmKit(kit);
    const preRows = await readFlattenRows(ks0, designId);
    await ks0.dispose();
    ks0 = undefined;
    const prePieceDiff = piecesDiffFromFlattenRows(preRows);
    const flat = cloneDesignWithDiff(rawDesign, { pieces: prePieceDiff });
    const piecesDesign: Design = { id: flat.id, name: flat.name, pieces: (flat.pieces ?? []).filter((p) => pieceIds.includes(p.id)) } as Design;
    const dragDiff = DesignEntity.prototype.dragBySelection.call(flat, piecesDesign, offset);
    const updatedRaw = cloneDesignWithDiff(rawDesign, dragDiff);
    const updatedKit: Kit = { ...asKitInstance(kit).toJSON(), designs: (kit.designs ?? []).map((d) => (d.id === designId ? updatedRaw : d)) } as Kit;
    ks1 = await openWasmKit(updatedKit);
    const postRows = await readFlattenRows(ks1, designId);
    const postPieceDiff = piecesDiffFromFlattenRows(postRows);
    const output = cloneDesignWithDiff(updatedRaw, { pieces: postPieceDiff });
    return { inputDesign: flat, output, dragDiff };
  } finally {
    if (ks0) await ks0.dispose();
    if (ks1) await ks1.dispose();
  }
}

/**
 * @emoji 🧮 Move via `KitStore.movePieces` (rs mutation) plus before/after flatten maps for diff display.
 */
export async function movePieces(kit: Kit, rawDesign: Design, pieceIds: readonly string[], vector: MoveVector): Promise<{ inputDesign: Design; output: Design; moveDiff: DesignDiff }> {
  const designId = rawDesign.id;
  let ks: KitStore | undefined;
  try {
    ks = await openWasmKit(kit);
    const preRows = await readFlattenRows(ks, designId);
    const prePieceDiff = piecesDiffFromFlattenRows(preRows);
    const flat = cloneDesignWithDiff(rawDesign, { pieces: prePieceDiff });
    const r = await ks.movePieces(designId, [...pieceIds], vector.gap, vector.shift, vector.rise);
    if (!r.ok) {
      throw new Error(String((r as { ok: false; error?: { message?: string } }).error?.message ?? "movePieces failed"));
    }
    const postRows = await readFlattenRows(ks, designId);
    const moveDiff: DesignDiff = {
      pieces: {
        updated: pieceIds
          .map((id) => {
            const a = preRows.find((x) => x.pieceId === id)?.center as { u?: number; v?: number } | undefined;
            const b = postRows.find((x) => x.pieceId === id)?.center as { u?: number; v?: number } | undefined;
            if (a == null || b == null) return null;
            return { piece: { id }, diff: { center: { u: (b.u ?? 0) - (a.u ?? 0), v: (b.v ?? 0) - (a.v ?? 0) } } };
          })
          .filter((x): x is NonNullable<typeof x> => x != null),
      },
    };
    const updatedRaw = cloneDesignWithDiff(rawDesign, moveDiff);
    const postPieceDiff = piecesDiffFromFlattenRows(postRows);
    const output = cloneDesignWithDiff(updatedRaw, { pieces: postPieceDiff });
    return { inputDesign: flat, output, moveDiff };
  } finally {
    if (ks) await ks.dispose();
  }
}

/**
 * @emoji 🧮 Copy a selection through `KitEntity.copyDesignOp`.
 */
export async function copyDesign(kit: Kit, design: Design, pieceIds: readonly string[], connectionIds: readonly string[]): Promise<OperationResult<Design>> {
  return KitEntity.ensure(kit).copyDesignOp(design, [...pieceIds], [...connectionIds]);
}

/**
 * @emoji 🧮 Paste a selection through `KitEntity.pasteDesignOp`.
 */
export async function pasteDesign(kit: Kit, source: Design, target: Design, anchoring: string, coordinate: Coordinate | undefined): Promise<DesignDiff> {
  return KitEntity.ensure(kit).pasteDesignOp(source, target, anchoring, coordinate);
}
// #endregion 🧮KitStoreRunners

// #region 🧪EmbeddedTests
if (process.env["SEMIO_ALGORITHMS_RUN_EMBEDDED_TESTS"] === "1") {
  const { describe, expect, it } = await import("vitest");

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
