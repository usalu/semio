/** 🔺️ Sparse diff construction for `replace-position-data`. */
import type { ReplacePositionData } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ReplacePositionData` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ReplacePositionData, base: unknown): unknown;
