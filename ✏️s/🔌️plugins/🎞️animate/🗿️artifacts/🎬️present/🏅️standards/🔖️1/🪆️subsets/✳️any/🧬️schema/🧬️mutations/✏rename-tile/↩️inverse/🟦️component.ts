/** ↩️ Inverse reconstruction for `rename-tile` — reads the BASE state, never the diff. */
import type { RenameTile } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `RenameTile` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: RenameTile, base: unknown): unknown[];
