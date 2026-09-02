/** 🖋️ `change-form-title` payload — mirrors Rust `ChangeFormTitle` (`../🦀️.rs:13`). No
 * `#[serde(rename_all)]` on the struct itself, so `new_title` stays snake_case (confirmed by the
 * committed `per-verb 🧪️tests 🦠️mutation/🔣️.json` fixture) despite the enum-level camelCase tag.
 * `Option<String>` with no `skip_serializing_if` — the key stays required, its value nullable. */
export interface ChangeFormTitle {
  new_title: string | null;
}
