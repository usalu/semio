/** ↩️ Inverse reconstruction for `edit-node-text` — reads the BASE state, never the diff. */
import type { EditNodeText } from "../🦠️mutation/🟦️component.ts";

/** Builds the inverse mutation(s) for `EditNodeText` from `(payload, base)` — mirrors the Rust `inverse`. */
export declare function inverse(payload: EditNodeText, base: unknown): unknown[];
