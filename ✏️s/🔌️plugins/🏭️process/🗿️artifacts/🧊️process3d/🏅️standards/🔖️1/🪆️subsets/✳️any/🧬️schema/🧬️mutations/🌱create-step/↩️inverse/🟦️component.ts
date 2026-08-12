/** ↩️ Inverse reconstruction for `create-step` — reads the BASE state, never the diff. */
import type { CreateStep } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `CreateStep` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: CreateStep, base: unknown): unknown[];
