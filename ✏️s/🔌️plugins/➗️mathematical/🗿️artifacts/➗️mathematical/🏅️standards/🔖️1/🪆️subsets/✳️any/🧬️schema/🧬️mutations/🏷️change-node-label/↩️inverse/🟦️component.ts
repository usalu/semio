/** ↩️ `change-node-label` — undo reconstructed from BASE state; missing node ⇒ `Vec::new()`. */
import type { ChangeNodeLabel } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `ChangeNodeLabel` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: ChangeNodeLabel, base: unknown): unknown[];
