/** 🚚️ `move-block-to-step` payload — mirrors Rust `MoveBlockToStep` (`../🦀️.rs:16`).
 * Its `#[value(rename_all = "camelCase")]` spells all four fields camelCase, like the enum-level tag. */
export interface MoveBlockToStep {
  stepId: string;
  blockId: string;
  toStepId: string;
  index: number;
}
