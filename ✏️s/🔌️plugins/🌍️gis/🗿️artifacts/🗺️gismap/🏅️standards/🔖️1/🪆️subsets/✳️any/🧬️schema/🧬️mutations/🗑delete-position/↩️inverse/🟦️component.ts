/** ↩️ Inverse reconstruction for `delete-position` — reads the BASE item, never the diff. */
import type { DeletePosition } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `DeletePosition` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: DeletePosition, base: unknown): unknown[];
