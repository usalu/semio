// 💡️ WFC 2D inferences — the TypeScript twin of `🦀️.rs`'s commit shape. The SOLVE itself runs in
// Rust (the engine is Rust-only); this leaf states the wire contract a client decodes, plus the
// prior-entropy math, which is pure and therefore twinnable.

import type { Wfc2dSlot, Wfc2dSnapshot } from "../📸️snapshot/🟦️.ts";

export const WFC_2D_INFERENCE_TOOL_ID = "s.wfc.wfc2d.solve";
export const WFC_2D_INFERENCE_PAYLOAD_SCHEMA = "s.wfc.wfc2d.inference.request.v1";
export const WFC_2D_INFERENCE_JOB_KIND = "semio.infer";

/** 💡️ The request the `semio.infer` cold job takes. */
export type Wfc2dInferenceRequest = { readonly snapshot: Wfc2dSnapshot; readonly checkpoint?: readonly number[] };

/** 🏁 What `s.wfc.wfc2d.solve` commits — never persisted on the snapshot. */
export type Wfc2dInferenceCommit = {
  readonly assignments: Readonly<Record<string, string>>;
  readonly contradiction: boolean;
  readonly entropy: Readonly<Record<string, number>>;
};

/** 🎲 One slot's pre-propagation Shannon entropy over the tile weights — `0` for a pinned slot. */
export function slotEntropy(snapshot: Wfc2dSnapshot, slot: Wfc2dSlot): number {
  if (slot.pinnedTileId !== undefined) return 0;
  const weights = snapshot.tiles.map((tile) => (Number.isFinite(tile.weight) && tile.weight > 0 ? tile.weight : 1));
  const total = weights.reduce((sum, weight) => sum + weight, 0);
  if (weights.length === 0 || total <= 0) return 0;
  return -weights
    .map((weight) => weight / total)
    .filter((share) => share > 0)
    .reduce((sum, share) => sum + share * Math.log(share), 0);
}

/** 🔗 The relation universe a solve compiles, ascending — the Rust `solve_relations` twin. */
export function solveRelations(snapshot: Wfc2dSnapshot): readonly string[] {
  const names = [...new Set(snapshot.edges.map((edge) => edge.relation))].sort();
  return names.length === 0 ? ["adjacent"] : names;
}
