/** 🔺 Diff constructor for `create-frame` — a `PagePatch.frame_added` fragment nested under the target page's `LayoutPagesDelta` patch entry (never apply-then-capture). */
import type { CreateFrame } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `CreateFrame` directly from `(payload, base)` — mirrors the Rust `diff_create_frame`. */
export declare function diffCreateFrame(payload: CreateFrame, base: unknown): unknown;
