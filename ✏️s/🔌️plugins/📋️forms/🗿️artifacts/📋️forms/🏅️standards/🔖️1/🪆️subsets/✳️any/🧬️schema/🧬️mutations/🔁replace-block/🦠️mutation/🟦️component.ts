/** 🔁️ `replace-block` payload — mirrors Rust `ReplaceBlock` (`../🦀️component.rs:14`). No
 * `#[serde(rename_all)]` on the struct itself, so `step_id` stays snake_case (confirmed by the
 * committed `per-verb 🧪️tests 🦠️mutation/🔣️component.json` fixture) despite the enum-level camelCase tag. */
import type { FormQuestion } from "../../🟦️component.ts";

export interface ReplaceBlock {
  step_id: string;
  block: FormQuestion;
}
