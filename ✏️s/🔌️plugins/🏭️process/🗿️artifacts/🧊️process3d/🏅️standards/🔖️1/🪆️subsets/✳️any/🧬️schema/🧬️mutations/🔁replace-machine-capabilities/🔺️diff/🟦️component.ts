/** 🔺️ Sparse diff construction for `replace-machine-capabilities`. */
import type { ReplaceMachineCapabilities } from "../🦠️mutation/🟦️component.ts";

/** Builds the sparse artifact diff for `ReplaceMachineCapabilities` directly from `(payload, base)` — mirrors the Rust `diff`. */
export declare function diff(payload: ReplaceMachineCapabilities, base: unknown): unknown;
