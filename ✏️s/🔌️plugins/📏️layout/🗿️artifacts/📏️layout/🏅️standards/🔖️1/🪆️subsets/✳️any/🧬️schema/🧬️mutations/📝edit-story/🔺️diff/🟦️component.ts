/** 🔺 Diff constructor for `edit-story`. */
import type { EditStory } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `EditStory` directly from `(payload, base)` — mirrors the Rust `diff_edit_story`. */
export declare function diffEditStory(payload: EditStory, base: unknown): unknown;
