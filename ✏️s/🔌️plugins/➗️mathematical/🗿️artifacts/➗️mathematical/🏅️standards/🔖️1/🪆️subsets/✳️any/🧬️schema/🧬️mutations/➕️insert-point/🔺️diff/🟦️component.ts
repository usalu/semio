/** 🔺️ `insert-point` — sparse diff construction. */
import type { InsertPoint } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `InsertPoint` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: InsertPoint, base: unknown): unknown;
