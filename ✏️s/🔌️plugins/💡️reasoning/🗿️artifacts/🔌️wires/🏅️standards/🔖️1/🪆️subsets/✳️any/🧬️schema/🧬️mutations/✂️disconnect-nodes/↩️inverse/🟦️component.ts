/** ↩️ Inverse reconstruction for `disconnect-nodes` — reads the BASE state, never the diff. */
import type { DisconnectNodes } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `DisconnectNodes` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: DisconnectNodes, base: unknown): unknown[];
