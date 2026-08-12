/** 🔺️ `remove-point` — sparse diff construction. */
import type { RemovePoint } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `RemovePoint` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: RemovePoint, base: unknown): unknown;
