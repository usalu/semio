/** 🔺️ `delete-nodes` — sparse diff construction, cascading to every incident edge. */
import type { DeleteNodes } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `DeleteNodes` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: DeleteNodes, base: unknown): unknown;
