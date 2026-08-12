/** 🔺️ Sparse diff construction for `replace-source`. */
import type { ReplaceSource } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ReplaceSource` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ReplaceSource, base: unknown): unknown;
