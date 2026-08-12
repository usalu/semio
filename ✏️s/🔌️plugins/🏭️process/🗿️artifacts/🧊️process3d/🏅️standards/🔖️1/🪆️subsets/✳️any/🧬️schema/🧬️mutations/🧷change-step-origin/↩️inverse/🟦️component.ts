/** ↩️ Inverse reconstruction for `change-step-origin` — reads the BASE state, never the diff. */
import type { ChangeStepOrigin } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `ChangeStepOrigin` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: ChangeStepOrigin, base: unknown): unknown[];
