/** ✂️ `delete-block` payload — mirrors Rust `DeleteBlock` (`../🦀️.rs:13`).
 * Its `#[value(rename_all = "camelCase")]` spells `stepId` camelCase, like the enum-level tag. */
export interface DeleteBlock {
  stepId: string;
  id: string;
}
