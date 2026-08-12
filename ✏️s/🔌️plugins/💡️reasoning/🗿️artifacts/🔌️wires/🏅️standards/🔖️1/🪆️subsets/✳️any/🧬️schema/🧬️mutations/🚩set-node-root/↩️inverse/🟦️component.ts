/** ↩️ Inverse reconstruction for `set-node-root` — reads the BASE state, never the diff. */
import type { SetNodeRoot } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `SetNodeRoot` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: SetNodeRoot, base: unknown): unknown[];
