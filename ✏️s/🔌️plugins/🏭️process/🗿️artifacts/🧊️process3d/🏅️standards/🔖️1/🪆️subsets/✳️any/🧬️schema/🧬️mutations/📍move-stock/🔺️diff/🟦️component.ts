/** 🔺️ Sparse diff construction for `move-stock`. */
import type { MoveStock } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `MoveStock` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: MoveStock, base: unknown): unknown;
