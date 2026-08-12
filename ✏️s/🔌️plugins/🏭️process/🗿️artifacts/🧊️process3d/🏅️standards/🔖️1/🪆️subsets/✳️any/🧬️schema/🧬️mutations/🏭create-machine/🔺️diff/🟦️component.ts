/** 🔺️ Sparse diff construction for `create-machine`. */
import type { CreateMachine } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `CreateMachine` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: CreateMachine, base: unknown): unknown;
