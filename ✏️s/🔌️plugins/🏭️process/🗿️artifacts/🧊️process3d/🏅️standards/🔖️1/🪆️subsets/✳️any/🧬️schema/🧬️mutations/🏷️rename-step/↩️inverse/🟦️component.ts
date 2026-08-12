/** ↩️ Inverse reconstruction for `rename-step` — reads the BASE state, never the diff. */
import type { RenameStep } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `RenameStep` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: RenameStep, base: unknown): unknown[];
