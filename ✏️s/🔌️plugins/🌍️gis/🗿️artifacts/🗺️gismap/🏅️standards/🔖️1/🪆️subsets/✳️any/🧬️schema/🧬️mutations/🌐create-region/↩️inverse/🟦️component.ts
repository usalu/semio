/** ↩️ Inverse reconstruction for `create-region` — undo is deleting the created feature. */
import type { CreateRegion } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `CreateRegion` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: CreateRegion, base: unknown): unknown[];
