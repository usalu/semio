/** ↩️ Inverse reconstruction for `change-step-enabled` — reads the BASE state, never the diff. */
import type { ChangeStepEnabled } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `ChangeStepEnabled` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: ChangeStepEnabled, base: unknown): unknown[];
