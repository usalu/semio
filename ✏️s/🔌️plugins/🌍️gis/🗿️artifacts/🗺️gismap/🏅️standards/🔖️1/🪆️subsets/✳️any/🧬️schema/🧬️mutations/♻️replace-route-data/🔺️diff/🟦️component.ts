/** 🔺️ Sparse diff construction for `replace-route-data`. */
import type { ReplaceRouteData } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ReplaceRouteData` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ReplaceRouteData, base: unknown): unknown;
