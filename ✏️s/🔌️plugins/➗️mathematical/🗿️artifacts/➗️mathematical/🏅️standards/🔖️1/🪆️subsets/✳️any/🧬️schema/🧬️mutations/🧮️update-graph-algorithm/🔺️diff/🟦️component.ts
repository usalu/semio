/** 🔺️ `update-graph-algorithm` — sparse diff construction. */
import type { UpdateGraphAlgorithm } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `UpdateGraphAlgorithm` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: UpdateGraphAlgorithm, base: unknown): unknown;
