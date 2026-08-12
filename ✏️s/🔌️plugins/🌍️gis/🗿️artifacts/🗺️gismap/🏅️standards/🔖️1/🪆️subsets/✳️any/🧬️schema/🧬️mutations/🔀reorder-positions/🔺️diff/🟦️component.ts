/** 🔺️ Sparse diff construction for `reorder-positions`. */
import type { ReorderPositions } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ReorderPositions` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ReorderPositions, base: unknown): unknown;
