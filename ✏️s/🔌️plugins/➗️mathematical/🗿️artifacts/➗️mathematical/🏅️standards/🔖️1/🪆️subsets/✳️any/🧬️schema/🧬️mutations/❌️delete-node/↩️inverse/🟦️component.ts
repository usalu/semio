/** ↩️ `delete-node` — undo re-creates the node and re-`connect`s every edge it severed, both captured from BASE state (pre-deletion); missing id ⇒ `Vec::new()`. */
import type { DeleteNode } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `DeleteNode` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: DeleteNode, base: unknown): unknown[];
