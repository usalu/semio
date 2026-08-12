/** 🔺️ `delete-node` — sparse diff construction, cascading to incident edges. */
import type { DeleteNode } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `DeleteNode` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: DeleteNode, base: unknown): unknown;
