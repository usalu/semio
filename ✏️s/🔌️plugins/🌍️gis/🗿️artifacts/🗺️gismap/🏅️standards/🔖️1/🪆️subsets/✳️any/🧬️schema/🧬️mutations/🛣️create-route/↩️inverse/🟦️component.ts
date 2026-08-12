/** ↩️ Inverse reconstruction for `create-route` — undo is deleting the created feature. */
import type { CreateRoute } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `CreateRoute` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: CreateRoute, base: unknown): unknown[];
