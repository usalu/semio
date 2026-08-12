/** 🔺️ Sparse diff construction for `set-node-root`. */
import type { SetNodeRoot } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `SetNodeRoot` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: SetNodeRoot, base: unknown): unknown;
