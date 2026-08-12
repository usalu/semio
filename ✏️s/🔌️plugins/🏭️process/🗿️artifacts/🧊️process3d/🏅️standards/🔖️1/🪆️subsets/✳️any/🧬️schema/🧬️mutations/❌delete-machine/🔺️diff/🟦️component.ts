/** 🔺️ Sparse diff construction for `delete-machine`. */
import type { DeleteMachine } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `DeleteMachine` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: DeleteMachine, base: unknown): unknown;
