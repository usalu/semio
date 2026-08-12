/** ↩️ Inverse reconstruction for `delete-step` — reads the BASE state, never the diff. */
import type { DeleteStep } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `DeleteStep` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: DeleteStep, base: unknown): unknown[];
