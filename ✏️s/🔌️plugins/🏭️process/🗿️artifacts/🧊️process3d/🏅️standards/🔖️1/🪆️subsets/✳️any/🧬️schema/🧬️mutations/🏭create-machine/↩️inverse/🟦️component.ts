/** ↩️ Inverse reconstruction for `create-machine` — reads the BASE state, never the diff. */
import type { CreateMachine } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `CreateMachine` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: CreateMachine, base: unknown): unknown[];
