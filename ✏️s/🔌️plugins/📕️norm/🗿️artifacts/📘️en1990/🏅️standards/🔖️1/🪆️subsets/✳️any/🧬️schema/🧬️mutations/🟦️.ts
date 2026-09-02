/** 🧬️ En1990 document mutations — discriminated union mirroring `En1990Mutation` (WASM wiring). */

export interface ChangeAnnex {
  newAnnex: "En" | "De";
}

export interface ChangePermanentAction {
  newGK: number;
}

export interface ChangeResistance {
  newResistanceKn: number;
}

export interface ChangeConsequenceClass {
  newConsequenceClass: number;
}

export interface ChangeSeismicAction {
  newSeismicAEdKn: number;
}

export interface InsertVariableAction {
  index: number;
  category: string;
  value: number;
}

export interface RemoveVariableAction {
  index: number;
}

export interface ChangeVariableActionCategory {
  index: number;
  newCategory: string;
}

export interface ChangeVariableActionValue {
  index: number;
  newValue: number;
}

export interface ReorderVariableActions {
  from: number;
  to: number;
}

export type En1990Mutation =
  | ({ mutation: "changeAnnex" } & ChangeAnnex)
  | ({ mutation: "changePermanentAction" } & ChangePermanentAction)
  | ({ mutation: "changeResistance" } & ChangeResistance)
  | ({ mutation: "changeConsequenceClass" } & ChangeConsequenceClass)
  | ({ mutation: "changeSeismicAction" } & ChangeSeismicAction)
  | ({ mutation: "insertVariableAction" } & InsertVariableAction)
  | ({ mutation: "removeVariableAction" } & RemoveVariableAction)
  | ({ mutation: "changeVariableActionCategory" } & ChangeVariableActionCategory)
  | ({ mutation: "changeVariableActionValue" } & ChangeVariableActionValue)
  | ({ mutation: "reorderVariableActions" } & ReorderVariableActions);
