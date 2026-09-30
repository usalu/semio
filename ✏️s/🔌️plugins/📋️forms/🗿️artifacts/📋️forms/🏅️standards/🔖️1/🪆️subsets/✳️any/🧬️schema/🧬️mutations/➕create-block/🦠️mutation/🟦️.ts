/** 🧩️ `create-block` payload — mirrors Rust `CreateBlock` (`../🦀️.rs:14`).
 * Its `#[value(rename_all = "camelCase")]` spells `stepId` camelCase, like the enum-level tag. */
import type { FormQuestion } from "../../🟦️.ts";

export interface CreateBlock {
  stepId: string;
  block: FormQuestion;
  index: number | null;
}
