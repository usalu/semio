/** ↩ Inverse constructor for `create-story` — always undoes to `delete-story`. */
import type { CreateStory } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `CreateStory` from `base` (pre-state) — mirrors the Rust `inverse_create_story`. */
export declare function inverseCreateStory(payload: CreateStory, base: unknown): unknown[];
