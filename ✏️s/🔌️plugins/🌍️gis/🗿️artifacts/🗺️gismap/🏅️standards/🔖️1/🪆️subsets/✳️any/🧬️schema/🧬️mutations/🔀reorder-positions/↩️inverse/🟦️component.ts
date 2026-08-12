/** ↩️ Inverse reconstruction for `reorder-positions` — reads the BASE position, never the diff. */
import type { ReorderPositions } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `ReorderPositions` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: ReorderPositions, base: unknown): unknown[];
