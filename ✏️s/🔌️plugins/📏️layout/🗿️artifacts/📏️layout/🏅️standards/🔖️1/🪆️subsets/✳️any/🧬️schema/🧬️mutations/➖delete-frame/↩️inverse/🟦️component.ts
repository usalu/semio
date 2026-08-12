/** ↩ Inverse constructor for `delete-frame` — captures the removed frame's full payload, its position within the page, and which layer (if any) referenced it. */
import type { DeleteFrame } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `DeleteFrame` from `base` (pre-state) — mirrors the Rust `inverse_delete_frame`. */
export declare function inverseDeleteFrame(payload: DeleteFrame, base: unknown): unknown[];
