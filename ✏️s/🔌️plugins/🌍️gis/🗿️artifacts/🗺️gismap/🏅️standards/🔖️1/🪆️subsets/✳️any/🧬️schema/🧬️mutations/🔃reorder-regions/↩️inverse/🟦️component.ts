/** ↩️ Inverse reconstruction for `reorder-regions` — reads the BASE position, never the diff. */
import type { ReorderRegions } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `ReorderRegions` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: ReorderRegions, base: unknown): unknown[];
