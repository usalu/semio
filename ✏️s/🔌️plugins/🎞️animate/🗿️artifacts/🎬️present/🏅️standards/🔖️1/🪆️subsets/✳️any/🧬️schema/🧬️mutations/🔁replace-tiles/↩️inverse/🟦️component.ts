/** ↩️ Inverse reconstruction for `replace-tiles` — reads the BASE state, never the diff. */
import type { ReplaceTiles } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `ReplaceTiles` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: ReplaceTiles, base: unknown): unknown[];
