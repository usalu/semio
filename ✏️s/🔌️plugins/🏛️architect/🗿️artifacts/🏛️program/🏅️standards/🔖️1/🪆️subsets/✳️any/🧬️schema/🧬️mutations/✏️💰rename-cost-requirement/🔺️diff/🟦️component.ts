/** 🔺️ Mirrors `diff(payload, base)` → ProgramDiff (see sibling 🦀️component.rs for the real
 *  handcrafted logic — this is a type-level mirror only). */
export type DiffRenameCostRequirement = (payload: RenameCostRequirement, base: ProgramSnapshot) => ProgramDiff;
