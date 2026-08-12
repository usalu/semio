/** ↩️ Inverse reconstruction for `connect-nodes` — reads the BASE state, never the diff. */
import type { ConnectNodes } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `ConnectNodes` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: ConnectNodes, base: unknown): unknown[];
