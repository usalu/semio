/** 🔺️ Sparse diff construction for `change-node-shape`. */
import type { ChangeNodeShape } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ChangeNodeShape` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ChangeNodeShape, base: unknown): unknown;
