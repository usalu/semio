/** ↩️ Inverse reconstruction for `delete-region` — reads the BASE item, never the diff. */
import type { DeleteRegion } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `DeleteRegion` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: DeleteRegion, base: unknown): unknown[];
