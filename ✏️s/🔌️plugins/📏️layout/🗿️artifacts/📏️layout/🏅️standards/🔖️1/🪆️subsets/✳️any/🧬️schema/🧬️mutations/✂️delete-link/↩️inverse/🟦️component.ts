/** ↩ Inverse constructor for `delete-link` — captures the removed link's full payload and position. */
import type { DeleteLink } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `DeleteLink` from `base` (pre-state) — mirrors the Rust `inverse_delete_link`. */
export declare function inverseDeleteLink(payload: DeleteLink, base: unknown): unknown[];
