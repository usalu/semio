/** 🧬️ En1994 document mutations — discriminated union mirroring `En1994Mutation` (WASM wiring). */

export interface ChangeAnnex {
  newAnnex: "En" | "De";
}

export interface ChangeMEdKnm {
  newMEdKnm: number;
}

export interface ChangeVEdKn {
  newVEdKn: number;
}

export interface ChangeMPla {
  newMPla: number;
}

export interface ChangeMPlRd {
  newMPlRd: number;
}

export interface ChangeEta {
  newEta: number;
}

export interface ChangeVLRd {
  newVLRd: number;
}

export interface ChangeInsulationThicknessMm {
  newInsulationThicknessMm: number;
}

export interface ChangeFireRating {
  newFireRating: string;
}

export interface ChangeDeckType {
  newDeckType: string;
}

export interface ChangeDeltaSigmaMpa {
  newDeltaSigmaMpa: number;
}

export interface ChangeFatigueDetail {
  newFatigueDetail: string;
}

export interface ChangeDMm {
  newDMm: number;
}

export interface ChangeHScMm {
  newHScMm: number;
}

export interface ChangeFCkMpa {
  newFCkMpa: number;
}

export interface ChangeFUMpa {
  newFUMpa: number;
}

export interface ChangeECmMpa {
  newECmMpa: number;
}

export interface ChangeVEdPerStudKn {
  newVEdPerStudKn: number;
}

export interface ChangeSpanM {
  newSpanM: number;
}

export interface ChangeFYMpa {
  newFYMpa: number;
}

export interface ChangeNCyclesStud {
  newNCyclesStud: number;
}

export interface ChangeDeltaTauStudMpa {
  newDeltaTauStudMpa: number;
}

export type En1994Mutation =
  | { ChangeAnnex: ChangeAnnex }
  | { ChangeMEdKnm: ChangeMEdKnm }
  | { ChangeVEdKn: ChangeVEdKn }
  | { ChangeMPla: ChangeMPla }
  | { ChangeMPlRd: ChangeMPlRd }
  | { ChangeEta: ChangeEta }
  | { ChangeVLRd: ChangeVLRd }
  | { ChangeInsulationThicknessMm: ChangeInsulationThicknessMm }
  | { ChangeFireRating: ChangeFireRating }
  | { ChangeDeckType: ChangeDeckType }
  | { ChangeDeltaSigmaMpa: ChangeDeltaSigmaMpa }
  | { ChangeFatigueDetail: ChangeFatigueDetail }
  | { ChangeDMm: ChangeDMm }
  | { ChangeHScMm: ChangeHScMm }
  | { ChangeFCkMpa: ChangeFCkMpa }
  | { ChangeFUMpa: ChangeFUMpa }
  | { ChangeECmMpa: ChangeECmMpa }
  | { ChangeVEdPerStudKn: ChangeVEdPerStudKn }
  | { ChangeSpanM: ChangeSpanM }
  | { ChangeFYMpa: ChangeFYMpa }
  | { ChangeNCyclesStud: ChangeNCyclesStud }
  | { ChangeDeltaTauStudMpa: ChangeDeltaTauStudMpa };
