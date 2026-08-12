/** ↩ Inverse constructor for `create-page` — always undoes to `delete-page`. */
import type { CreatePage } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `CreatePage` from `base` (pre-state) — mirrors the Rust `inverse_create_page`. */
export declare function inverseCreatePage(payload: CreatePage, base: unknown): unknown[];
