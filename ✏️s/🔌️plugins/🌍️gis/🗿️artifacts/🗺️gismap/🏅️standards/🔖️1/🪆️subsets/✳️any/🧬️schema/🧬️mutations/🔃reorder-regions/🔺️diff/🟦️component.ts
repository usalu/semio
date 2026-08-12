/** 🔺️ Sparse diff construction for `reorder-regions`. */
import type { ReorderRegions } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ReorderRegions` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ReorderRegions, base: unknown): unknown;
