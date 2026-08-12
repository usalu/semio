/** ↩️ `disconnect-nodes` — undo re-`connect`s the exact edge captured from BASE state; missing edge ⇒ `Vec::new()`. */
import type { DisconnectNodes } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `DisconnectNodes` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: DisconnectNodes, base: unknown): unknown[];
