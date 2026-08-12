/** ↩️ `move-node` — undo reconstructed from BASE state; missing node ⇒ `Vec::new()`. */
import type { MoveNode } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `MoveNode` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: MoveNode, base: unknown): unknown[];
