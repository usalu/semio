/** ↩ Inverse constructor for `change-frame-stroke` — reconstructed from captured BASE state. A non-rect frame has nothing to undo (the field-patch was a no-op). */
import type { ChangeFrameStroke } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `ChangeFrameStroke` from `base` (pre-state) — mirrors the Rust `inverse_change_frame_stroke`. */
export declare function inverseChangeFrameStroke(payload: ChangeFrameStroke, base: unknown): unknown[];
