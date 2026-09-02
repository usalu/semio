/** 🧬️ En1999 document mutations — discriminated union mirroring `En1999Mutation` (WASM wiring). */

export interface ChangeNEdKn {
  newNEdKn: number;
}

export interface ChangeMEdKnm {
  newMEdKnm: number;
}

export interface ChangeAMm2 {
  newAMm2: number;
}

export interface ChangeWElMm3 {
  newWElMm3: number;
}

export interface ChangeAlloy {
  newAlloy: string;
}

export interface ChangeChi {
  newChi: number;
}

export interface ChangeITMm4 {
  newITMm4: number;
}

export interface ChangeLCrMm {
  newLCrMm: number;
}

export interface ChangeThetaC {
  newThetaC: number;
}

export interface ChangeDeltaSigmaEd {
  newDeltaSigmaEd: number;
}

export interface ChangeDeltaSigmaC {
  newDeltaSigmaC: number;
}

export interface ChangeFatigueM {
  newFatigueM: number;
}

export interface ChangeNCycles {
  newNCycles: number;
}

export interface ChangeVWeldEdKn {
  newVWeldEdKn: number;
}

export interface ChangeWeldThroatMm {
  newWeldThroatMm: number;
}

export interface ChangeWeldLengthMm {
  newWeldLengthMm: number;
}

export interface ChangeBetaW {
  newBetaW: number;
}

export interface ChangeSheetBMm {
  newSheetBMm: number;
}

export interface ChangeSheetTMm {
  newSheetTMm: number;
}

export interface ChangeSheetKSigma {
  newSheetKSigma: number;
}

export interface ChangeSheetWElMm3 {
  newSheetWElMm3: number;
}

export interface ChangeSheetMEdKnm {
  newSheetMEdKnm: number;
}

export interface ChangeShellTMm {
  newShellTMm: number;
}

export interface ChangeShellRMm {
  newShellRMm: number;
}

export interface ChangeSigmaEdShellMpa {
  newSigmaEdShellMpa: number;
}

export interface ChangeAnnex {
  newAnnex: "En" | "De";
}

export type En1999Mutation =
  | ({ mutation: "changeNEdKn" } & ChangeNEdKn)
  | ({ mutation: "changeMEdKnm" } & ChangeMEdKnm)
  | ({ mutation: "changeAMm2" } & ChangeAMm2)
  | ({ mutation: "changeWElMm3" } & ChangeWElMm3)
  | ({ mutation: "changeAlloy" } & ChangeAlloy)
  | ({ mutation: "changeChi" } & ChangeChi)
  | ({ mutation: "changeITMm4" } & ChangeITMm4)
  | ({ mutation: "changeLCrMm" } & ChangeLCrMm)
  | ({ mutation: "changeThetaC" } & ChangeThetaC)
  | ({ mutation: "changeDeltaSigmaEd" } & ChangeDeltaSigmaEd)
  | ({ mutation: "changeDeltaSigmaC" } & ChangeDeltaSigmaC)
  | ({ mutation: "changeFatigueM" } & ChangeFatigueM)
  | ({ mutation: "changeNCycles" } & ChangeNCycles)
  | ({ mutation: "changeVWeldEdKn" } & ChangeVWeldEdKn)
  | ({ mutation: "changeWeldThroatMm" } & ChangeWeldThroatMm)
  | ({ mutation: "changeWeldLengthMm" } & ChangeWeldLengthMm)
  | ({ mutation: "changeBetaW" } & ChangeBetaW)
  | ({ mutation: "changeSheetBMm" } & ChangeSheetBMm)
  | ({ mutation: "changeSheetTMm" } & ChangeSheetTMm)
  | ({ mutation: "changeSheetKSigma" } & ChangeSheetKSigma)
  | ({ mutation: "changeSheetWElMm3" } & ChangeSheetWElMm3)
  | ({ mutation: "changeSheetMEdKnm" } & ChangeSheetMEdKnm)
  | ({ mutation: "changeShellTMm" } & ChangeShellTMm)
  | ({ mutation: "changeShellRMm" } & ChangeShellRMm)
  | ({ mutation: "changeSigmaEdShellMpa" } & ChangeSigmaEdShellMpa)
  | ({ mutation: "changeAnnex" } & ChangeAnnex);
