/** 🔺️ Sparse diff construction for `rename-step`. */
import type { RenameStep } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `RenameStep` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: RenameStep, base: unknown): unknown;
