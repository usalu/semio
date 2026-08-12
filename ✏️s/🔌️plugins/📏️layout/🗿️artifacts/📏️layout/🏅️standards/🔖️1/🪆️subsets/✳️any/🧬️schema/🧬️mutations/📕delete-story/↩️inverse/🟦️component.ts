/** ↩ Inverse constructor for `delete-story` — captures the removed story's full payload and position. */
import type { DeleteStory } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `DeleteStory` from `base` (pre-state) — mirrors the Rust `inverse_delete_story`. */
export declare function inverseDeleteStory(payload: DeleteStory, base: unknown): unknown[];
