/** ↩️ Inverse reconstruction for `replace-step-measure` — reads the BASE state, never the diff. */
import type { ReplaceStepMeasure } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `ReplaceStepMeasure` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: ReplaceStepMeasure, base: unknown): unknown[];
