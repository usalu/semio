/** ↩️ `change-coefficient` — undo reconstructed from BASE's own value at `label`; missing/non-numeric target ⇒ empty array. */
import type { ChangeCoefficient } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `ChangeCoefficient` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: ChangeCoefficient, base: unknown): unknown[];
