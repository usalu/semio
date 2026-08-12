/** 🔺️ Sparse diff construction for `rename-tile`. */
import type { RenameTile } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `RenameTile` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: RenameTile, base: unknown): unknown;
