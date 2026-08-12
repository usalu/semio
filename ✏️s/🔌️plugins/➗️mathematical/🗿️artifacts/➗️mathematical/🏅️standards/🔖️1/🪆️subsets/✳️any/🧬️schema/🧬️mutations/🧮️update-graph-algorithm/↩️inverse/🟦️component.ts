/** ↩️ `update-graph-algorithm` — undo reconstructed from BASE state. */
import type { UpdateGraphAlgorithm } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `UpdateGraphAlgorithm` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: UpdateGraphAlgorithm, base: unknown): unknown[];
