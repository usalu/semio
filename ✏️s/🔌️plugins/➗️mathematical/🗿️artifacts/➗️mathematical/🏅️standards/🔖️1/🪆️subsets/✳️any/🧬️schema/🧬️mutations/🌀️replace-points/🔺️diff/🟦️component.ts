/** 🔺️ `replace-points` — sparse diff construction. */
import type { ReplacePoints } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ReplacePoints` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ReplacePoints, base: unknown): unknown;
