/** 🔺️ Sparse diff construction for `disconnect-nodes`. */
import type { DisconnectNodes } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `DisconnectNodes` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: DisconnectNodes, base: unknown): unknown;
