/** 🧬️ En1997 document mutations — discriminated union mirroring `En1997Mutation` (WASM wiring). */

export interface ChangeVEdKn {
  newVEdKn: number;
}

export interface ChangeHEdKn {
  newHEdKn: number;
}

export interface ChangeFootingAreaM2 {
  newFootingAreaM2: number;
}

export interface ChangePhiDeg {
  newPhiDeg: number;
}

export interface ChangeCKpa {
  newCKpa: number;
}

export interface ChangeGammaKnM3 {
  newGammaKnM3: number;
}

export interface ChangeBM {
  newBM: number;
}

export interface ChangeDFM {
  newDFM: number;
}

export interface ChangeESMpa {
  newESMpa: number;
}

export interface ChangeNu {
  newNu: number;
}

export interface ChangeDesignApproach {
  newDesignApproach: string;
}

export interface ChangeAnnex {
  newAnnex: "En" | "De";
}

export interface ChangeSettlementLimitMm {
  newSettlementLimitMm: number;
}

export interface ChangeNPileEdKn {
  newNPileEdKn: number;
}

export interface ChangeAlphaS {
  newAlphaS: number;
}

export interface ChangePileDM {
  newPileDM: number;
}

export interface ChangeQSKpa {
  newQSKpa: number;
}

export interface ChangePileLM {
  newPileLM: number;
}

export interface ChangeQBKpa {
  newQBKpa: number;
}

export interface ChangePileBaseAreaM2 {
  newPileBaseAreaM2: number;
}

export interface ChangePileNProfiles {
  newPileNProfiles: number;
}

export interface ChangeZInvestigatedM {
  newZInvestigatedM: number;
}

export type En1997Mutation =
  | ({ mutation: "changeVEdKn" } & ChangeVEdKn)
  | ({ mutation: "changeHEdKn" } & ChangeHEdKn)
  | ({ mutation: "changeFootingAreaM2" } & ChangeFootingAreaM2)
  | ({ mutation: "changePhiDeg" } & ChangePhiDeg)
  | ({ mutation: "changeCKpa" } & ChangeCKpa)
  | ({ mutation: "changeGammaKnM3" } & ChangeGammaKnM3)
  | ({ mutation: "changeBM" } & ChangeBM)
  | ({ mutation: "changeDFM" } & ChangeDFM)
  | ({ mutation: "changeESMpa" } & ChangeESMpa)
  | ({ mutation: "changeNu" } & ChangeNu)
  | ({ mutation: "changeDesignApproach" } & ChangeDesignApproach)
  | ({ mutation: "changeAnnex" } & ChangeAnnex)
  | ({ mutation: "changeSettlementLimitMm" } & ChangeSettlementLimitMm)
  | ({ mutation: "changeNPileEdKn" } & ChangeNPileEdKn)
  | ({ mutation: "changeAlphaS" } & ChangeAlphaS)
  | ({ mutation: "changePileDM" } & ChangePileDM)
  | ({ mutation: "changeQSKpa" } & ChangeQSKpa)
  | ({ mutation: "changePileLM" } & ChangePileLM)
  | ({ mutation: "changeQBKpa" } & ChangeQBKpa)
  | ({ mutation: "changePileBaseAreaM2" } & ChangePileBaseAreaM2)
  | ({ mutation: "changePileNProfiles" } & ChangePileNProfiles)
  | ({ mutation: "changeZInvestigatedM" } & ChangeZInvestigatedM);
