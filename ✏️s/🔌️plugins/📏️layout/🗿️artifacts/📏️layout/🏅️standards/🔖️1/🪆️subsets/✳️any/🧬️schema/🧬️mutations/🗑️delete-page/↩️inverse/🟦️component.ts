/** ↩ Inverse constructor for `delete-page` — captures the removed page's full payload and position. */
import type { DeletePage } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `DeletePage` from `base` (pre-state) — mirrors the Rust `inverse_delete_page`. */
export declare function inverseDeletePage(payload: DeletePage, base: unknown): unknown[];
