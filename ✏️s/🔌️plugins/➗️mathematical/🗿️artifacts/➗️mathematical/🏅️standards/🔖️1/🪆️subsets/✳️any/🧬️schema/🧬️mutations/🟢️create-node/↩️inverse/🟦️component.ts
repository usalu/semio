/** ↩️ `create-node` — undo is `delete-node`, unless `base` already had this id (then `create` was a no-op and there's nothing to undo). */
import type { CreateNode } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `CreateNode` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: CreateNode, base: unknown): unknown[];
