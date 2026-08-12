/** ↩️ Inverse reconstruction for `replace-stock-solid` — reads the BASE state, never the diff. */
import type { ReplaceStockSolid } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `ReplaceStockSolid` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: ReplaceStockSolid, base: unknown): unknown[];
