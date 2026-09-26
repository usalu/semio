/** 🧬️ En1990Mutation — mirrors Rust enum (camelCase tags). */
export type En1990Mutation =
  | { mutation: "changeAnnex"; newAnnex: "En" | "De" }
  | { mutation: "changeProjectId"; newProjectId: string }
  | { mutation: "changeConsequenceClass"; newConsequenceClass: number }
  | { mutation: "changeReliabilityClass"; newReliabilityClass: number }
  | { mutation: "changeDesignWorkingLifeCategory"; newDesignWorkingLifeCategory: number }
  | { mutation: "changeDesignWorkingLifeYears"; newDesignWorkingLifeYears: number }
  | { mutation: "changeReferencePeriodYears"; newReferencePeriodYears: number }
  | { mutation: "changeSupervisionLevel"; newSupervisionLevel: string }
  | { mutation: "changeInspectionLevel"; newInspectionLevel: string }
  | { mutation: "changeBetaComputed"; newBetaComputed: number }
  | { mutation: "changePermanents"; newPermanents: unknown[] }
  | { mutation: "changeVariables"; newVariables: unknown[] }
  | { mutation: "changeAccidentals"; newAccidentals: unknown[] }
  | { mutation: "changeSeismics"; newSeismics: unknown[] }
  | { mutation: "changeMembers"; newMembers: unknown[] }
  | { mutation: "changeEffects"; newEffects: unknown[] }
  | { mutation: "insertPermanent"; index: number; item: unknown }
  | { mutation: "removePermanent"; index: number }
  | { mutation: "insertVariable"; index: number; item: unknown }
  | { mutation: "removeVariable"; index: number }
  | { mutation: "insertAccidental"; index: number; item: unknown }
  | { mutation: "removeAccidental"; index: number }
  | { mutation: "insertSeismic"; index: number; item: unknown }
  | { mutation: "removeSeismic"; index: number }
  | { mutation: "insertMember"; index: number; item: unknown }
  | { mutation: "removeMember"; index: number }
  | { mutation: "insertEffect"; index: number; item: unknown }
  | { mutation: "removeEffect"; index: number };
