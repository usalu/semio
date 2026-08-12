/** 🔺️ `replace-graph` — sparse diff construction. */
import type { ReplaceGraph } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ReplaceGraph` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ReplaceGraph, base: unknown): unknown;
