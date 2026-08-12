/** 🔺 Diff constructor for `delete-frame` — a `PagePatch.frame_removed` fragment. */
import type { DeleteFrame } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `DeleteFrame` directly from `(payload, base)` — mirrors the Rust `diff_delete_frame`. */
export declare function diffDeleteFrame(payload: DeleteFrame, base: unknown): unknown;
