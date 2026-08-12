/** ↩️ Inverse reconstruction for `create-position` — undo is deleting the created feature. */
import type { CreatePosition } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `CreatePosition` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: CreatePosition, base: unknown): unknown[];
