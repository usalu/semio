/** ↩️ Inverse reconstruction for `change-node-shape` — reads the BASE state, never the diff. */
import type { ChangeNodeShape } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `ChangeNodeShape` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: ChangeNodeShape, base: unknown): unknown[];
