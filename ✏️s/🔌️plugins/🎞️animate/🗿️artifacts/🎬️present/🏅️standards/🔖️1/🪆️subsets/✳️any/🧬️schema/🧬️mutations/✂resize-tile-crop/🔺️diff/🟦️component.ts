/** 🔺️ Sparse diff construction for `resize-tile-crop`. */
import type { ResizeTileCrop } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ResizeTileCrop` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ResizeTileCrop, base: unknown): unknown;
