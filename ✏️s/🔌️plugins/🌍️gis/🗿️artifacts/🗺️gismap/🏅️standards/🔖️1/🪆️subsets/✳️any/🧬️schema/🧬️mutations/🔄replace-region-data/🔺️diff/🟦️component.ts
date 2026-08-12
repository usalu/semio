/** 🔺️ Sparse diff construction for `replace-region-data`. */
import type { ReplaceRegionData } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ReplaceRegionData` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ReplaceRegionData, base: unknown): unknown;
