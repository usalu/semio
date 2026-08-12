/** 🔺️ `create-node` — sparse diff construction. */
import type { CreateNode } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `CreateNode` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: CreateNode, base: unknown): unknown;
