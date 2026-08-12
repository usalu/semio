/** 🔺️ Sparse diff construction for `replace-tiles`. */
import type { ReplaceTiles } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ReplaceTiles` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ReplaceTiles, base: unknown): unknown;
