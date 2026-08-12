/** 🔺 Diff constructor for `delete-story`. */
import type { DeleteStory } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `DeleteStory` directly from `(payload, base)` — mirrors the Rust `diff_delete_story`. */
export declare function diffDeleteStory(payload: DeleteStory, base: unknown): unknown;
