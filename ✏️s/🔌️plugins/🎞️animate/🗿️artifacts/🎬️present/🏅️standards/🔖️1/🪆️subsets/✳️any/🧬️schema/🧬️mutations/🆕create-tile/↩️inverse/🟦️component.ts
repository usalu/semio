/** ↩️ Inverse reconstruction for `create-tile` — reads the BASE state, never the diff. */
import type { CreateTile } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `CreateTile` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: CreateTile, base: unknown): unknown[];
