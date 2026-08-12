/** 🔺️ Sparse diff construction for `change-cursor`. */
import type { ChangeCursor } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ChangeCursor` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ChangeCursor, base: unknown): unknown;
