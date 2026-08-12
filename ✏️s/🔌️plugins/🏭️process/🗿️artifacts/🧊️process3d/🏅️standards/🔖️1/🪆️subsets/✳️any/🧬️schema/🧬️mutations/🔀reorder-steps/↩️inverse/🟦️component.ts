/** ↩️ Inverse reconstruction for `reorder-steps` — reads the BASE state, never the diff. */
import type { ReorderSteps } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `ReorderSteps` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: ReorderSteps, base: unknown): unknown[];
