/** 🔺️ Sparse diff construction for `create-route`. */
import type { CreateRoute } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `CreateRoute` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: CreateRoute, base: unknown): unknown;
