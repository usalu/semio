/** 🧬️ En1992 document mutations — discriminated union mirroring `En1992Mutation` (WASM wiring). */

export interface ChangeAnnex {
  newAnnex: "En" | "De";
}

export interface ChangeMEdKnm {
  newMEdKnm: number;
}

export interface ChangeVEdKn {
  newVEdKn: number;
}

export interface ChangeFCk {
  newFCk: number;
}

export interface ChangeBMm {
  newBMm: number;
}

export interface ChangeDMm {
  newDMm: number;
}

export interface ChangeASMm2 {
  newASMm2: number;
}

export interface ChangeFYk {
  newFYk: number;
}

export interface ChangeRhoL {
  newRhoL: number;
}

export interface ChangeNEdKn {
  newNEdKn: number;
}

export interface ChangePKn {
  newPKn: number;
}

export interface ChangeACMm2 {
  newACMm2: number;
}

export interface ChangeUseFem {
  newUseFem: boolean;
}

export interface ChangeSpanM {
  newSpanM: number;
}

export interface ChangeUdlKnM {
  newUdlKnM: number;
}

export interface ChangeFireRating {
  newFireRating: "R30" | "R60" | "R90" | "R120";
}

export interface ChangeProvidedAxisDistanceMm {
  newProvidedAxisDistanceMm: number;
}

export interface ChangeBridgeSigmaCMpa {
  newBridgeSigmaCMpa: number;
}

export interface ChangeBridgeDeltaSigmaSMpa {
  newBridgeDeltaSigmaSMpa: number;
}

export interface ChangeTightnessClass {
  newTightnessClass: "Tc0" | "Tc1" | "Tc2";
}

export interface ChangeHdOverH {
  newHdOverH: number;
}

export interface ChangeLiquidSigmaSMpa {
  newLiquidSigmaSMpa: number;
}

export interface ChangeLiquidRhoPEff {
  newLiquidRhoPEff: number;
}

export interface ChangeLiquidFCtEffMpa {
  newLiquidFCtEffMpa: number;
}

export interface ChangeLiquidESMpa {
  newLiquidESMpa: number;
}

export interface ChangeLiquidSRMaxMm {
  newLiquidSRMaxMm: number;
}

export interface ChangeAnchorHEfMm {
  newAnchorHEfMm: number;
}

export interface ChangeAnchorCracked {
  newAnchorCracked: boolean;
}

export interface ChangeAnchorFUkMpa {
  newAnchorFUkMpa: number;
}

export interface ChangeAnchorFYkMpa {
  newAnchorFYkMpa: number;
}

export interface ChangeAnchorASMm2 {
  newAnchorASMm2: number;
}

export interface ChangeAnchorDMm {
  newAnchorDMm: number;
}

export interface ChangeAnchorC1Mm {
  newAnchorC1Mm: number;
}

export interface ChangeAnchorNEdKn {
  newAnchorNEdKn: number;
}

export interface ChangeAnchorVEdKn {
  newAnchorVEdKn: number;
}

export type En1992Mutation =
  | { ChangeAnnex: ChangeAnnex }
  | { ChangeMEdKnm: ChangeMEdKnm }
  | { ChangeVEdKn: ChangeVEdKn }
  | { ChangeFCk: ChangeFCk }
  | { ChangeBMm: ChangeBMm }
  | { ChangeDMm: ChangeDMm }
  | { ChangeASMm2: ChangeASMm2 }
  | { ChangeFYk: ChangeFYk }
  | { ChangeRhoL: ChangeRhoL }
  | { ChangeNEdKn: ChangeNEdKn }
  | { ChangePKn: ChangePKn }
  | { ChangeACMm2: ChangeACMm2 }
  | { ChangeUseFem: ChangeUseFem }
  | { ChangeSpanM: ChangeSpanM }
  | { ChangeUdlKnM: ChangeUdlKnM }
  | { ChangeFireRating: ChangeFireRating }
  | { ChangeProvidedAxisDistanceMm: ChangeProvidedAxisDistanceMm }
  | { ChangeBridgeSigmaCMpa: ChangeBridgeSigmaCMpa }
  | { ChangeBridgeDeltaSigmaSMpa: ChangeBridgeDeltaSigmaSMpa }
  | { ChangeTightnessClass: ChangeTightnessClass }
  | { ChangeHdOverH: ChangeHdOverH }
  | { ChangeLiquidSigmaSMpa: ChangeLiquidSigmaSMpa }
  | { ChangeLiquidRhoPEff: ChangeLiquidRhoPEff }
  | { ChangeLiquidFCtEffMpa: ChangeLiquidFCtEffMpa }
  | { ChangeLiquidESMpa: ChangeLiquidESMpa }
  | { ChangeLiquidSRMaxMm: ChangeLiquidSRMaxMm }
  | { ChangeAnchorHEfMm: ChangeAnchorHEfMm }
  | { ChangeAnchorCracked: ChangeAnchorCracked }
  | { ChangeAnchorFUkMpa: ChangeAnchorFUkMpa }
  | { ChangeAnchorFYkMpa: ChangeAnchorFYkMpa }
  | { ChangeAnchorASMm2: ChangeAnchorASMm2 }
  | { ChangeAnchorDMm: ChangeAnchorDMm }
  | { ChangeAnchorC1Mm: ChangeAnchorC1Mm }
  | { ChangeAnchorNEdKn: ChangeAnchorNEdKn }
  | { ChangeAnchorVEdKn: ChangeAnchorVEdKn };
