/** ↩️ Inverse reconstruction for `delete-route` — reads the BASE item, never the diff. */
import type { DeleteRoute } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `DeleteRoute` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: DeleteRoute, base: unknown): unknown[];
