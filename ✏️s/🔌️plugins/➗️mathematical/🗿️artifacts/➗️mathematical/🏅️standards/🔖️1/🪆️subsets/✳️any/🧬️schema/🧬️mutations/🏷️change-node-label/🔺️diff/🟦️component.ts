/** 🔺️ `change-node-label` — sparse diff construction. */
import type { ChangeNodeLabel } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ChangeNodeLabel` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ChangeNodeLabel, base: unknown): unknown;
