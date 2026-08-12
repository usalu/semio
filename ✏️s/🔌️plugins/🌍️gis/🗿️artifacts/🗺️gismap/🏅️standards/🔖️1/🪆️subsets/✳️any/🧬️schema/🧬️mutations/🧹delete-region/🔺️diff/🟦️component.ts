/** 🔺️ Sparse diff construction for `delete-region`. */
import type { DeleteRegion } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `DeleteRegion` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: DeleteRegion, base: unknown): unknown;
