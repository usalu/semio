/** ↩️ Inverse reconstruction for `change-cursor` — reads the BASE state, never the diff. */
import type { ChangeCursor } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `ChangeCursor` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: ChangeCursor, base: unknown): unknown[];
