/** ✂️ `delete-block` payload — mirrors Rust `DeleteBlock` (`../🦀️.rs:13`). No
 * `#[serde(rename_all)]` on the struct itself, so `step_id` stays snake_case (confirmed by the
 * committed `per-verb 🧪️tests 🦠️mutation/🔣️.json` fixture) despite the enum-level camelCase tag. */
export interface DeleteBlock {
  step_id: string;
  id: string;
}
