/** ↩️ `insert-point` — undo is `remove-point` at the same (now FINAL-state) index, per the index-keyed addressing law. */
import type { InsertPoint } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `InsertPoint` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: InsertPoint, base: unknown): unknown[];
