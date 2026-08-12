/** 🔺️ Sparse diff construction for `reorder-tiles`. */
import type { ReorderTiles } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ReorderTiles` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ReorderTiles, base: unknown): unknown;
