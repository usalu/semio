/** 🔺️ Sparse diff construction for `change-step-origin`. */
import type { ChangeStepOrigin } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ChangeStepOrigin` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ChangeStepOrigin, base: unknown): unknown;
