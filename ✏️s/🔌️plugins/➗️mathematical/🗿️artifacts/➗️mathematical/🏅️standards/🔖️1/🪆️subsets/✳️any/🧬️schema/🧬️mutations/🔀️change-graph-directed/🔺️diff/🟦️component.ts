/** 🔺️ `change-graph-directed` — sparse diff construction. */
import type { ChangeGraphDirected } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ChangeGraphDirected` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ChangeGraphDirected, base: unknown): unknown;
