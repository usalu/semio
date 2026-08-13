/** 🔺️ `change-coefficient` — computed from `(payload, base)`, never apply-then-capture. */
import type { ChangeCoefficient } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ChangeCoefficient` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ChangeCoefficient, base: unknown): unknown;
