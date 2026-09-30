/** 🌱️ `create-step` payload — mirrors Rust `CreateStep` (`../🦀️.rs:14`).
 * Its `#[value(rename_all = "camelCase")]` spells its fields camelCase, like `FormMutation`'s enum tag. */
import type { FormStep } from "../../🟦️.ts";

export interface CreateStep {
  step: FormStep;
  index: number | null;
}
