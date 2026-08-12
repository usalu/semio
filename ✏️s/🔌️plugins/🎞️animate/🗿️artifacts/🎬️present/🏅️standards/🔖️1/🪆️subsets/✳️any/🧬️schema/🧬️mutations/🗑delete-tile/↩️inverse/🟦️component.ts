/** ↩️ Inverse reconstruction for `delete-tile` — reads the BASE state, never the diff. */
import type { DeleteTile } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `DeleteTile` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: DeleteTile, base: unknown): unknown[];
