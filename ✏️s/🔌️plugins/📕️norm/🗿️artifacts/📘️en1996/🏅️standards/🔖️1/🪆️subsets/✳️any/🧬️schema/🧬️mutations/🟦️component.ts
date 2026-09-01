/** 🧬️ En1996 document mutations — discriminated union mirroring `En1996Mutation` (WASM wiring). */

export interface ChangeMEdKnm {
  newMEdKnm: number;
}

export interface ChangeNEdKn {
  newNEdKn: number;
}

export interface ChangeVEdKn {
  newVEdKn: number;
}

export interface ChangeHEdKn {
  newHEdKn: number;
}

export interface ChangeZMm3 {
  newZMm3: number;
}

export interface ChangeAreaMm2 {
  newAreaMm2: number;
}

export interface ChangeShearAreaMm2 {
  newShearAreaMm2: number;
}

export interface ChangeFKMpa {
  newFKMpa: number;
}

export interface ChangeFVkMpa {
  newFVkMpa: number;
}

export interface ChangeAnnex {
  newAnnex: "En" | "De";
}

export interface ChangeMasonryClass {
  newMasonryClass: "Class1" | "Class2" | "Class3" | "Class4" | "Class5";
}

export interface ChangeDesignSituation {
  newDesignSituation: "Persistent" | "Transient" | "Accidental" | "Seismic";
}

export interface ChangeMu {
  newMu: number;
}

export interface ChangeWallThicknessMm {
  newWallThicknessMm: number;
}

export interface ChangeFireResistanceMin {
  newFireResistanceMin: number;
}

export interface ChangeUnit {
  newUnit: string;
}

export interface ChangeExposure {
  newExposure: "Mx1" | "Mx2" | "Mx3" | "Mx4" | "Mx5";
}

export interface ChangeMortar {
  newMortar: "M1" | "M2_5" | "M5" | "M10" | "M20";
}

export interface ChangeBedJointThicknessMm {
  newBedJointThicknessMm: number;
}

export interface ChangeStoreys {
  newStoreys: number;
}

export interface ChangeHEfMm {
  newHEfMm: number;
}

export interface ChangeTEfMm {
  newTEfMm: number;
}

export type En1996Mutation =
  | ({ mutation: "changeMEdKnm" } & ChangeMEdKnm)
  | ({ mutation: "changeNEdKn" } & ChangeNEdKn)
  | ({ mutation: "changeVEdKn" } & ChangeVEdKn)
  | ({ mutation: "changeHEdKn" } & ChangeHEdKn)
  | ({ mutation: "changeZMm3" } & ChangeZMm3)
  | ({ mutation: "changeAreaMm2" } & ChangeAreaMm2)
  | ({ mutation: "changeShearAreaMm2" } & ChangeShearAreaMm2)
  | ({ mutation: "changeFKMpa" } & ChangeFKMpa)
  | ({ mutation: "changeFVkMpa" } & ChangeFVkMpa)
  | ({ mutation: "changeAnnex" } & ChangeAnnex)
  | ({ mutation: "changeMasonryClass" } & ChangeMasonryClass)
  | ({ mutation: "changeDesignSituation" } & ChangeDesignSituation)
  | ({ mutation: "changeMu" } & ChangeMu)
  | ({ mutation: "changeWallThicknessMm" } & ChangeWallThicknessMm)
  | ({ mutation: "changeFireResistanceMin" } & ChangeFireResistanceMin)
  | ({ mutation: "changeUnit" } & ChangeUnit)
  | ({ mutation: "changeExposure" } & ChangeExposure)
  | ({ mutation: "changeMortar" } & ChangeMortar)
  | ({ mutation: "changeBedJointThicknessMm" } & ChangeBedJointThicknessMm)
  | ({ mutation: "changeStoreys" } & ChangeStoreys)
  | ({ mutation: "changeHEfMm" } & ChangeHEfMm)
  | ({ mutation: "changeTEfMm" } & ChangeTEfMm);
