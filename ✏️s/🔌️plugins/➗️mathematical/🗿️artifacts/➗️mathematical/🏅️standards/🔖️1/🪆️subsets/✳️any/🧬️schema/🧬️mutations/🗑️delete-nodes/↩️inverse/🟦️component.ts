/** ↩️ `delete-nodes` — re-creates every deleted node then re-`connect`s every severed edge, both captured from BASE state. */
import type { DeleteNodes } from "../🦠️mutation/🟦️component.ts";

/** Reconstructs the inverse mutation list for `DeleteNodes` from `base` (pre-state) — mirrors the Rust `inverse`. */
export declare function inverse(payload: DeleteNodes, base: unknown): unknown[];
