/** ↩ Inverse constructor for `create-link` — always undoes to `delete-link`. */
import type { CreateLink } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `CreateLink` from `base` (pre-state) — mirrors the Rust `inverse_create_link`. */
export declare function inverseCreateLink(payload: CreateLink, base: unknown): unknown[];
