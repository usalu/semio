/** ↩️ Mirrors `inverse(payload, base)` → ProgramMutation[] (see sibling 🦀️component.rs for the
 *  real handcrafted logic — this is a type-level mirror only). */
export type InverseReplaceFlexibilityRequirement = (payload: ReplaceFlexibilityRequirement, base: ProgramSnapshot) => ProgramMutation[];
