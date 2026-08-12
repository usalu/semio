/** ↩️ `change-graph-directed` — undo reconstructed from BASE state. */
import type { ChangeGraphDirected } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `ChangeGraphDirected` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: ChangeGraphDirected, base: unknown): unknown[];
