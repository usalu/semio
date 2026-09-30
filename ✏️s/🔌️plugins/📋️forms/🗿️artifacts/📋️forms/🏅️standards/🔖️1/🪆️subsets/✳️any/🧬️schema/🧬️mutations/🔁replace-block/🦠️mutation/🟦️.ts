/** 🔁️ `replace-block` payload — mirrors Rust `ReplaceBlock` (`../🦀️.rs:14`).
 * Its `#[value(rename_all = "camelCase")]` spells `stepId` camelCase, like the enum-level tag. */
import type { FormQuestion } from "../../🟦️.ts";

export interface ReplaceBlock {
  stepId: string;
  block: FormQuestion;
}
