/** ↩ Inverse constructor for `reorder-pages` — reorders back to the captured BASE-state position. */
import type { ReorderPages } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `ReorderPages` from `base` (pre-state) — mirrors the Rust `inverse_reorder_pages`. */
export declare function inverseReorderPages(payload: ReorderPages, base: unknown): unknown[];
