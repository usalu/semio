/** 🧬️ Assembly artifact schema — the persisted WFC problem spec. */

export { type AssemblySnapshot, type AssemblySlot, type AssemblySlotEdge, type AssemblyModuleWeight, type AssemblyRule } from "./📸️snapshot/🟦️";

export interface AssemblyArtifact {
  /** @state artifact */
  snapshot: import("./📸️snapshot/🟦️").AssemblySnapshot;
}
