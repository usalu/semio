/** 🔺️ Sparse diff construction for `create-region`. */
import type { CreateRegion } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `CreateRegion` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: CreateRegion, base: unknown): unknown;
