/** 🏷️ `rename-step` payload — mirrors Rust `RenameStep` (`../🦀️.rs:13`).
 * Its `#[value(rename_all = "camelCase")]` spells `newTitle` camelCase, like the enum-level tag. */
export interface RenameStep {
  id: string;
  newTitle: string;
}
