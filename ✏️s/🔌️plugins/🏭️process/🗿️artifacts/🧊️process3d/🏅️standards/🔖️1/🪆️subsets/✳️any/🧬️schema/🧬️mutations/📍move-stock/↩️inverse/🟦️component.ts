/** ↩️ Inverse reconstruction for `move-stock` — reads the BASE state, never the diff. */
import type { MoveStock } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `MoveStock` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: MoveStock, base: unknown): unknown[];
