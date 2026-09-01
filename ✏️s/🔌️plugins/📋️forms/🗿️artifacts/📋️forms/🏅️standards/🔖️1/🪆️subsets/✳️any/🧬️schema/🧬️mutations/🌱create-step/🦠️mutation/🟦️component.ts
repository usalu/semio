/** 🌱️ `create-step` payload — mirrors Rust `CreateStep` (`../🦀️component.rs:14`). No
 * `#[serde(rename_all)]` on the struct itself, so its own two fields stay snake_case (confirmed by
 * the committed `per-verb 🧪️tests 🦠️mutation/🔣️component.json` fixtures) even though `FormMutation`'s
 * enum tag is camelCase. */
import type { FormStep } from "../../🟦️component.ts";

export interface CreateStep {
  step: FormStep;
  index: number | null;
}
