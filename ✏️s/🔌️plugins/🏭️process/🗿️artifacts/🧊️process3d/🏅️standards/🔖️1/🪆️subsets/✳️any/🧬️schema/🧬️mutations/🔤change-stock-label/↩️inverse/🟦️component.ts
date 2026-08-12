/** ↩️ Inverse reconstruction for `change-stock-label` — reads the BASE state, never the diff. */
import type { ChangeStockLabel } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `ChangeStockLabel` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: ChangeStockLabel, base: unknown): unknown[];
