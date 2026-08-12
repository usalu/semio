/** 🔺️ Sparse diff construction for `delete-route`. */
import type { DeleteRoute } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `DeleteRoute` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: DeleteRoute, base: unknown): unknown;
