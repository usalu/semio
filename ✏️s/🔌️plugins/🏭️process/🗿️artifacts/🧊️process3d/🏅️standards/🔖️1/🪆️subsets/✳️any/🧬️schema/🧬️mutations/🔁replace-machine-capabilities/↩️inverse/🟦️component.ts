/** ↩️ Inverse reconstruction for `replace-machine-capabilities` — reads the BASE state, never the diff. */
import type { ReplaceMachineCapabilities } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `ReplaceMachineCapabilities` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: ReplaceMachineCapabilities, base: unknown): unknown[];
