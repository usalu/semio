/** 🔺️ Sparse diff construction for `connect-nodes`. */
import type { ConnectNodes } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ConnectNodes` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ConnectNodes, base: unknown): unknown;
