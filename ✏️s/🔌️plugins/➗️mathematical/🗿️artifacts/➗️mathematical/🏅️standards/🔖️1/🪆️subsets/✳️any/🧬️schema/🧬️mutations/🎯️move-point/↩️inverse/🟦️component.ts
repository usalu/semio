/** ↩️ `move-point` — undo reconstructed from BASE state; out-of-range index ⇒ `Vec::new()`. */
import type { MovePoint } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `MovePoint` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: MovePoint, base: unknown): unknown[];
