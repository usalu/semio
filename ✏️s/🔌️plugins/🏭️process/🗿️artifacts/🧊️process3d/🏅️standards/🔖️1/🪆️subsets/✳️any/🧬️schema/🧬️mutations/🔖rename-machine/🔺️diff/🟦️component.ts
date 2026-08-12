/** 🔺️ Sparse diff construction for `rename-machine`. */
import type { RenameMachine } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `RenameMachine` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: RenameMachine, base: unknown): unknown;
