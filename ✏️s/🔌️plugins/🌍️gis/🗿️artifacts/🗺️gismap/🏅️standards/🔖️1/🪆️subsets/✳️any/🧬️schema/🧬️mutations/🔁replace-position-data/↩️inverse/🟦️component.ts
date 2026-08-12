/** ↩️ Inverse reconstruction for `replace-position-data` — reads the BASE payload, never the diff. */
import type { ReplacePositionData } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `ReplacePositionData` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: ReplacePositionData, base: unknown): unknown[];
