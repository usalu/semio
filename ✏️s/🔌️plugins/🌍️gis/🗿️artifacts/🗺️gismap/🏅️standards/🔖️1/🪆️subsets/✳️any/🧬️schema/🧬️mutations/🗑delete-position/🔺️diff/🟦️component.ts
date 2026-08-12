/** 🔺️ Sparse diff construction for `delete-position`. */
import type { DeletePosition } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `DeletePosition` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: DeletePosition, base: unknown): unknown;
