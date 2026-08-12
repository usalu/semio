/** ↩️ Inverse reconstruction for `create-node` — reads the BASE state, never the diff. */
import type { CreateNode } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `CreateNode` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: CreateNode, base: unknown): unknown[];
