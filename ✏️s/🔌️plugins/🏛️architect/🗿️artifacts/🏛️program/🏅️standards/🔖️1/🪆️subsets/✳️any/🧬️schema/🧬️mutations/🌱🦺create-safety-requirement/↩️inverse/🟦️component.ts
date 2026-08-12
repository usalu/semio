/** ↩️ Mirrors `inverse(payload, base)` → ProgramMutation[] (see sibling 🦀️component.rs for the
 *  real handcrafted logic — this is a type-level mirror only). */
export type InverseCreateSafetyRequirement = (payload: CreateSafetyRequirement, base: ProgramSnapshot) => ProgramMutation[];
