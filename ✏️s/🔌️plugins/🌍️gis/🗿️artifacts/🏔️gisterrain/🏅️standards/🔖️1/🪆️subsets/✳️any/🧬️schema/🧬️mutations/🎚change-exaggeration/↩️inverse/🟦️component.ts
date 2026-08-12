/** ↩️ Inverse reconstruction for `change-exaggeration` — reads the BASE value, never the diff. */
import type { ChangeExaggeration } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `ChangeExaggeration` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: ChangeExaggeration, base: unknown): unknown[];
