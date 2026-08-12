/** 🔺️ Sparse diff construction for `resize-node`. */
import type { ResizeNode } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ResizeNode` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ResizeNode, base: unknown): unknown;
