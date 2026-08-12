/** ↩️ Inverse reconstruction for `resize-tile-crop` — reads the BASE state, never the diff. */
import type { ResizeTileCrop } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `ResizeTileCrop` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: ResizeTileCrop, base: unknown): unknown[];
