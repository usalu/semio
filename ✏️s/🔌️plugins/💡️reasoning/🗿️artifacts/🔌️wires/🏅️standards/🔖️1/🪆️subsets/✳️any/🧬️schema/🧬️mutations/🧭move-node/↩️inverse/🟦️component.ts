/** ↩️ Inverse reconstruction for `move-node` — reads the BASE state, never the diff. */
import type { MoveNode } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `MoveNode` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: MoveNode, base: unknown): unknown[];
