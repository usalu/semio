/** 🔺 Diff constructor for `change-data-fields`. */
import type { ChangeDataFields } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ChangeDataFields` directly from `(payload, base)` — mirrors the Rust `diff_change_data_fields`. */
export declare function diffChangeDataFields(payload: ChangeDataFields, base: unknown): unknown;
