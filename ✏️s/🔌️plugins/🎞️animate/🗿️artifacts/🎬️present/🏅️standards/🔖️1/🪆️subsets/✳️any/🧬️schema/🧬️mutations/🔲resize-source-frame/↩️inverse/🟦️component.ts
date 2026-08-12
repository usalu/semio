/** ↩️ Inverse reconstruction for `resize-source-frame` — reads the BASE state, never the diff. */
import type { ResizeSourceFrame } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `ResizeSourceFrame` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: ResizeSourceFrame, base: unknown): unknown[];
