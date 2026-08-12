/** 🔺️ Sparse diff construction for `move-node`. */
import type { MoveNode } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `MoveNode` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: MoveNode, base: unknown): unknown;
