/** 🔺️ Sparse diff construction for `delete-step`. */
import type { DeleteStep } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `DeleteStep` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: DeleteStep, base: unknown): unknown;
