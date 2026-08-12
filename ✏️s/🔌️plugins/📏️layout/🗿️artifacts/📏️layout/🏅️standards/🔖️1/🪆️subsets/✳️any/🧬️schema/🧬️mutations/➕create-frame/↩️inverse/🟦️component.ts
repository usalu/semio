/** ↩ Inverse constructor for `create-frame` — always undoes to `delete-frame` (matches the pre-migration `AddFrame`'s inverse, which never inspected `base`). */
import type { CreateFrame } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `CreateFrame` from `base` (pre-state) — mirrors the Rust `inverse_create_frame`. */
export declare function inverseCreateFrame(payload: CreateFrame, base: unknown): unknown[];
