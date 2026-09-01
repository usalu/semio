/** 🔀️ `reorder-step` payload — mirrors Rust `ReorderStep` (`../🦀️component.rs:13`). No
 * `#[serde(rename_all)]` on the struct itself, so `to_index` stays snake_case (confirmed by the
 * committed `per-verb 🧪️tests 🦠️mutation/🔣️component.json` fixture) despite the enum-level camelCase tag. */
export interface ReorderStep {
  id: string;
  to_index: number;
}
