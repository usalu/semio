/** 🔺️ Sparse diff construction for `create-step`. */
import type { CreateStep } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `CreateStep` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: CreateStep, base: unknown): unknown;
