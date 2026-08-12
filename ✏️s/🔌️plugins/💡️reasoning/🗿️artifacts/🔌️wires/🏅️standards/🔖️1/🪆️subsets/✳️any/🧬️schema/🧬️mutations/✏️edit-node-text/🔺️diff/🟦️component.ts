/** 🔺️ Sparse diff construction for `edit-node-text`. */
import type { EditNodeText } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `EditNodeText` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: EditNodeText, base: unknown): unknown;
