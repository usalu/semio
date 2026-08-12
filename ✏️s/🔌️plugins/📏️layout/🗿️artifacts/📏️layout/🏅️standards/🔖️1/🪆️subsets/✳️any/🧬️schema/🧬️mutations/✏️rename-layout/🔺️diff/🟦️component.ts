/** 🔺 Diff constructor for `rename-layout` — builds `LayoutDiff` sparsely from the payload. */
import type { RenameLayout } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `RenameLayout` directly from `(payload, base)` — mirrors the Rust `diff_rename_layout`. */
export declare function diffRenameLayout(payload: RenameLayout, base: unknown): unknown;
