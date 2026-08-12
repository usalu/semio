/** ↩️ Inverse reconstruction for `replace-source` — reads the BASE state, never the diff. */
import type { ReplaceSource } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `ReplaceSource` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: ReplaceSource, base: unknown): unknown[];
