/** ↩️ Inverse reconstruction for `replace-route-data` — reads the BASE payload, never the diff. */
import type { ReplaceRouteData } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `ReplaceRouteData` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: ReplaceRouteData, base: unknown): unknown[];
