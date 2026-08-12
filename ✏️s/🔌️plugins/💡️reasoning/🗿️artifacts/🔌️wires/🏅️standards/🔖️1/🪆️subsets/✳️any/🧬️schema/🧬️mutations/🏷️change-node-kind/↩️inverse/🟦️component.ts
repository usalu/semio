/** ↩️ Inverse reconstruction for `change-node-kind` — reads the BASE state, never the diff. */
import type { ChangeNodeKind } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `ChangeNodeKind` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: ChangeNodeKind, base: unknown): unknown[];
