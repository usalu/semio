/** 🔺️ Sparse diff construction for `delete-tiles`. */
import type { DeleteTiles } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `DeleteTiles` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: DeleteTiles, base: unknown): unknown;
