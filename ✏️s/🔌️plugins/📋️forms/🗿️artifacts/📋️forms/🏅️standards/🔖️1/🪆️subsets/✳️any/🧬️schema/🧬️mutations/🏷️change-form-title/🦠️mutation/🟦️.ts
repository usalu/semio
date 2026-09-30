/** 🖋️ `change-form-title` payload — mirrors Rust `ChangeFormTitle` (`../🦀️.rs:13`).
 * Its `#[value(rename_all = "camelCase")]` spells `newTitle` camelCase, like the enum-level tag.
 * `Option<String>` with no `skip_serializing_if` — the key stays required, its value nullable. */
export interface ChangeFormTitle {
  newTitle: string | null;
}
