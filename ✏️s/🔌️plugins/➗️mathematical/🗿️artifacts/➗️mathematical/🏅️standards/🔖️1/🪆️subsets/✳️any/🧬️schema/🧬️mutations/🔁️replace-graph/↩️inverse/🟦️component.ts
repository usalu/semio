/** ↩️ `replace-graph` — undo reconstructed from BASE state (the whole prior graph). */
import type { ReplaceGraph } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `ReplaceGraph` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: ReplaceGraph, base: unknown): unknown[];
