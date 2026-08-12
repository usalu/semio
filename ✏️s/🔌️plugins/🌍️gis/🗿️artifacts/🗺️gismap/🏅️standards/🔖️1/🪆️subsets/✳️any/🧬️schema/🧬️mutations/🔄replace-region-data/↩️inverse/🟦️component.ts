/** ↩️ Inverse reconstruction for `replace-region-data` — reads the BASE payload, never the diff. */
import type { ReplaceRegionData } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `ReplaceRegionData` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: ReplaceRegionData, base: unknown): unknown[];
