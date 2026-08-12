/** 🔺️ Sparse diff construction for `delete-tile`. */
import type { DeleteTile } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `DeleteTile` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: DeleteTile, base: unknown): unknown;
