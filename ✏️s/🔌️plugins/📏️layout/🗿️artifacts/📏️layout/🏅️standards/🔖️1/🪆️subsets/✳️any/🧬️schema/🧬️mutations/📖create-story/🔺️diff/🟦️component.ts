/** 🔺 Diff constructor for `create-story`. */
import type { CreateStory } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `CreateStory` directly from `(payload, base)` — mirrors the Rust `diff_create_story`. */
export declare function diffCreateStory(payload: CreateStory, base: unknown): unknown;
