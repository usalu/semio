/** 🔺️ Sparse diff construction for `change-node-kind`. */
import type { ChangeNodeKind } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ChangeNodeKind` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ChangeNodeKind, base: unknown): unknown;
