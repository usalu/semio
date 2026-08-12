/** 🔺️ Sparse diff construction for `create-position`. */
import type { CreatePosition } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `CreatePosition` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: CreatePosition, base: unknown): unknown;
