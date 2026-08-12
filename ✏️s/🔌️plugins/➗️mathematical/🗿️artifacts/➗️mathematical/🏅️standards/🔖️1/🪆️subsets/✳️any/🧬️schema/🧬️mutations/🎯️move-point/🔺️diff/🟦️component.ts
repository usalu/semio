/** 🔺️ `move-point` — sparse diff construction. */
import type { MovePoint } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `MovePoint` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: MovePoint, base: unknown): unknown;
