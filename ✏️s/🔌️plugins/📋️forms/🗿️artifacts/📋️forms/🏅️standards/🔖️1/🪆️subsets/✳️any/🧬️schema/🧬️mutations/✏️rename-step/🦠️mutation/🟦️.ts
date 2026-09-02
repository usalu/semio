/** 🏷️ `rename-step` payload — mirrors Rust `RenameStep` (`../🦀️.rs:13`). No
 * `#[serde(rename_all)]` on the struct itself, so `new_title` stays snake_case (confirmed by the
 * committed `per-verb 🧪️tests 🦠️mutation/🔣️.json` fixture) despite the enum-level camelCase tag. */
export interface RenameStep {
  id: string;
  new_title: string;
}
