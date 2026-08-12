/** 🔺️ Sparse diff construction for `create-tile`. */
import type { CreateTile } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `CreateTile` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: CreateTile, base: unknown): unknown;
