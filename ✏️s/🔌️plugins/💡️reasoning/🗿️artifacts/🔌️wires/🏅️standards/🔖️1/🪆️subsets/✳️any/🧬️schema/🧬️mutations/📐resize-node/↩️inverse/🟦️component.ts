/** ↩️ Inverse reconstruction for `resize-node` — reads the BASE state, never the diff. */
import type { ResizeNode } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `ResizeNode` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: ResizeNode, base: unknown): unknown[];
