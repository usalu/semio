/** 🔺️ Sparse diff construction for `change-imported-features`. */
import type { ChangeImportedFeatures } from "../🟦️.ts";

/** Builds the sparse artifact diff for `ChangeImportedFeatures` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ChangeImportedFeatures, base: unknown): unknown;
