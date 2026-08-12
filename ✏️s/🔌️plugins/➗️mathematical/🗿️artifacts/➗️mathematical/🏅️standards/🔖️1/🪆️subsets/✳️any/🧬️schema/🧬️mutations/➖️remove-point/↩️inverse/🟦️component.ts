/** ↩️ `remove-point` — undo re-`insert`s the exact point captured from BASE state; out-of-range index ⇒ `Vec::new()`. */
import type { RemovePoint } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `RemovePoint` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: RemovePoint, base: unknown): unknown[];
