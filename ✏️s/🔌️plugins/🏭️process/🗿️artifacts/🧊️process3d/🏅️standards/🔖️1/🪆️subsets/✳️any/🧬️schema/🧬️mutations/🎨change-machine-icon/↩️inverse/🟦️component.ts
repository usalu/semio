/** ↩️ Inverse reconstruction for `change-machine-icon` — reads the BASE state, never the diff. */
import type { ChangeMachineIcon } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `ChangeMachineIcon` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: ChangeMachineIcon, base: unknown): unknown[];
