/** 🔺️ Sparse diff construction for `reorder-routes`. */
import type { ReorderRoutes } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ReorderRoutes` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ReorderRoutes, base: unknown): unknown;
