/** ↩️ Inverse reconstruction for `rename-machine` — reads the BASE state, never the diff. */
import type { RenameMachine } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `RenameMachine` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: RenameMachine, base: unknown): unknown[];
