/** 🔺️ Sparse diff construction for `reorder-steps`. */
import type { ReorderSteps } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ReorderSteps` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ReorderSteps, base: unknown): unknown;
