/** 🔺️ Sparse diff construction for `replace-step-measure`. */
import type { ReplaceStepMeasure } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ReplaceStepMeasure` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ReplaceStepMeasure, base: unknown): unknown;
