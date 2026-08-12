/** 🔺️ Sparse diff construction for `change-exaggeration`. */
import type { ChangeExaggeration } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ChangeExaggeration` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ChangeExaggeration, base: unknown): unknown;
