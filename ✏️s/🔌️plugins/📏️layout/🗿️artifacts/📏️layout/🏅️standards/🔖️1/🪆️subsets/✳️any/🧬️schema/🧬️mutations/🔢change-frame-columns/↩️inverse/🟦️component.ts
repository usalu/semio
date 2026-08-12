/** ↩ Inverse constructor for `change-frame-columns` — reconstructed from captured BASE state. A non-text frame has nothing to undo (the field-patch was a no-op). */
import type { ChangeFrameColumns } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `ChangeFrameColumns` from `base` (pre-state) — mirrors the Rust `inverse_change_frame_columns`. */
export declare function inverseChangeFrameColumns(payload: ChangeFrameColumns, base: unknown): unknown[];
