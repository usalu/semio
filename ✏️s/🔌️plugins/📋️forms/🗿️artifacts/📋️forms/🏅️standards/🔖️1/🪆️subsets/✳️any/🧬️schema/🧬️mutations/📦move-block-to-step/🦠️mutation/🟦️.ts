/** 🚚️ `move-block-to-step` payload — mirrors Rust `MoveBlockToStep` (`../🦀️.rs:16`). No
 * `#[serde(rename_all)]` on the struct itself, so all four fields stay snake_case (confirmed by the
 * committed `per-verb 🧪️tests 🦠️mutation/🔣️.json` fixture) despite the enum-level camelCase tag. */
export interface MoveBlockToStep {
  step_id: string;
  block_id: string;
  to_step_id: string;
  index: number;
}
