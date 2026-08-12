/** ↩️ `replace-points` — undo reconstructed from BASE state (the whole prior point cloud). */
import type { ReplacePoints } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `ReplacePoints` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: ReplacePoints, base: unknown): unknown[];
