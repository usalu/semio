/** 🧬️ Assembly artifact mutation dispatch — one discriminated-union member per
 * `🧬️mutations/<slug>/` triad's payload shape. Real facet mirror of the Rust `🦀️component.rs`
 * sibling's `AssemblyMutation` enum. */
import type { AssemblySlot, AssemblySlotEdge, AssemblyRule } from "../📸️snapshot/🟦️component";

export type AssemblyMutation =
  | { kind: "create-slot"; index: number; slot: AssemblySlot }
  | { kind: "delete-slot"; id: string }
  | { kind: "create-rule"; index: number; rule: AssemblyRule }
  | { kind: "delete-rule"; id: string }
  | { kind: "change-weight"; moduleId: string; weight: number }
  | { kind: "remove-weight"; moduleId: string }
  | { kind: "connect-slots"; index: number; edge: AssemblySlotEdge }
  | { kind: "disconnect-slots"; id: string }
  | { kind: "change-seed"; seed: number };
