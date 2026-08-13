/** 🔺️ AssemblyDiff schema — real facet mirror of the Rust `🦀️component.rs` sibling: a sparse,
 * id-keyed structural delta, never a whole-snapshot capture. */
import type { AssemblyModuleWeight, AssemblyRule, AssemblySlot, AssemblySlotEdge } from "../📸️snapshot/🟦️component";

export interface AssemblyDiff {
  /** @state persistent */ schema?: string;
  /** @state persistent */ seed?: number;
  /** @state persistent */ slotsRemoved: string[];
  /** @state persistent */ slotsUpserted: [number, AssemblySlot][];
  /** @state persistent */ edgesRemoved: string[];
  /** @state persistent */ edgesUpserted: [number, AssemblySlotEdge][];
  /** @state persistent */ weightsRemoved: string[];
  /** @state persistent */ weightsUpserted: AssemblyModuleWeight[];
  /** @state persistent */ rulesRemoved: string[];
  /** @state persistent */ rulesUpserted: [number, AssemblyRule][];
}
