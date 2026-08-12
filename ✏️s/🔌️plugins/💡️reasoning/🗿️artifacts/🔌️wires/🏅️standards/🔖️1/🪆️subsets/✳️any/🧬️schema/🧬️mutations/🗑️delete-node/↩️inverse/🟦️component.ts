/** ↩️ Inverse reconstruction for `delete-node` — reads the BASE state, never the diff. */
import type { DeleteNode } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `DeleteNode` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: DeleteNode, base: unknown): unknown[];
