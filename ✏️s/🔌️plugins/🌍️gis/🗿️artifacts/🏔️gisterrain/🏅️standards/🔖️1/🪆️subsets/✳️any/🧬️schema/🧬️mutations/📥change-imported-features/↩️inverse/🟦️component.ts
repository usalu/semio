/** ↩️ Inverse reconstruction for `change-imported-features` — reads the BASE value, never the diff. */
import type { ChangeImportedFeatures } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `ChangeImportedFeatures` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: ChangeImportedFeatures, base: unknown): unknown[];
