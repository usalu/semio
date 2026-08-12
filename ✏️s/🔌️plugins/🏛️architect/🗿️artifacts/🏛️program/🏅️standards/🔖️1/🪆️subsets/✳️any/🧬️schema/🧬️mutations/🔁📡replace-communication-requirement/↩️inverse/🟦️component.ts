/** ↩️ Mirrors `inverse(payload, base)` → ProgramMutation[] (see sibling 🦀️component.rs for the
 *  real handcrafted logic — this is a type-level mirror only). */
export type InverseReplaceCommunicationRequirement = (payload: ReplaceCommunicationRequirement, base: ProgramSnapshot) => ProgramMutation[];
