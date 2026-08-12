/** ↩️ Inverse reconstruction for `reorder-routes` — reads the BASE position, never the diff. */
import type { ReorderRoutes } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `ReorderRoutes` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: ReorderRoutes, base: unknown): unknown[];
