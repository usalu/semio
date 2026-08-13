/** 🔺️ AssemblyDiff schema — real facet mirror of the Rust `🦀️component.rs` sibling: a sparse,
 * id-keyed structural delta, never a whole-snapshot capture. */
import type { AssemblyModuleWeight, AssemblyRule, AssemblySlot, AssemblySlotEdge } from "../📸️snapshot/🟦️component";

export interface AssemblyDiff {
  /** @state artifact */ schema?: string;
  /** @state artifact */ seed?: number;
  /** @state artifact */ slotsRemoved: string[];
  /** @state artifact */ slotsUpserted: [number, AssemblySlot][];
  /** @state artifact */ edgesRemoved: string[];
  /** @state artifact */ edgesUpserted: [number, AssemblySlotEdge][];
  /** @state artifact */ weightsRemoved: string[];
  /** @state artifact */ weightsUpserted: AssemblyModuleWeight[];
  /** @state artifact */ rulesRemoved: string[];
  /** @state artifact */ rulesUpserted: [number, AssemblyRule][];
}
