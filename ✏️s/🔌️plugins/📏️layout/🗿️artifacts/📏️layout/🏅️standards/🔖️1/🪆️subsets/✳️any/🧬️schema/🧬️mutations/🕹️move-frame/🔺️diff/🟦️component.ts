/** 🔺 Diff constructor for `move-frame` — a `PagePatch.frame_patched` fragment carrying a bounds-only `FramePatch`. */
import type { MoveFrame } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `MoveFrame` directly from `(payload, base)` — mirrors the Rust `diff_move_frame`. */
export declare function diffMoveFrame(payload: MoveFrame, base: unknown): unknown;
