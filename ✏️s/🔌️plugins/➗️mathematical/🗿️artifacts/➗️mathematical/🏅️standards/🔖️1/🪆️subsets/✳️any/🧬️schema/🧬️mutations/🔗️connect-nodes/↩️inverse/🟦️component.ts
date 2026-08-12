/** ↩️ `connect-nodes` — undo is `disconnect-nodes`, unless `base` already had this edge id (then `connect` was a no-op, matching `create-node`'s duplicate-id handling). */
import type { ConnectNodes } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `ConnectNodes` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: ConnectNodes, base: unknown): unknown[];
