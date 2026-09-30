/** 🔀️ `reorder-step` payload — mirrors Rust `ReorderStep` (`../🦀️.rs:13`).
 * Its `#[value(rename_all = "camelCase")]` spells `toIndex` camelCase, like the enum-level tag. */
export interface ReorderStep {
  id: string;
  toIndex: number;
}
