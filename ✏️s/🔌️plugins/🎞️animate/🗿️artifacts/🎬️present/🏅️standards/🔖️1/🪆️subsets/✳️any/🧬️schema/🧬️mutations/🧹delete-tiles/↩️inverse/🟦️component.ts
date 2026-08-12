/** ↩️ Inverse reconstruction for `delete-tiles` — reads the BASE state, never the diff. */
import type { DeleteTiles } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `DeleteTiles` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: DeleteTiles, base: unknown): unknown[];
