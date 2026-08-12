/** ↩️ Inverse reconstruction for `delete-machine` — reads the BASE state, never the diff. */
import type { DeleteMachine } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `DeleteMachine` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: DeleteMachine, base: unknown): unknown[];
