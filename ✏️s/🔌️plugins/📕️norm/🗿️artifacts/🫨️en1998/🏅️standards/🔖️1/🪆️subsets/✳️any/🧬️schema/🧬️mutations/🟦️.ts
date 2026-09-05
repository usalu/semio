/** 🧬️ En1998 document mutations — discriminated union mirroring `En1998Mutation` (WASM wiring). */

export interface ChangeSeismicZone {
  newSeismicZone: number;
}

export interface ChangeGroundType {
  newGroundType: string;
}

export interface ChangeImportanceClass {
  newImportanceClass: string;
}

export interface ChangeStructuralSystem {
  newStructuralSystem: string;
}

export interface ChangeT1S {
  newT1S: number;
}

export interface ChangeMassT {
  newMassT: number;
}

export interface ChangeVRdKn {
  newVRdKn: number;
}

export interface ChangeDriftMm {
  newDriftMm: number;
}

export interface ChangeHeightM {
  newHeightM: number;
}

export interface ChangeMultipleResistingSystems {
  newMultipleResistingSystems: boolean;
}

export interface ChangeAnnex {
  newAnnex: string;
}

export interface ChangeEnAGr {
  newEnAGr: number;
}

export interface ChangeEnGroundType {
  newEnGroundType: string;
}

export interface ChangeEnSpectrumType {
  newEnSpectrumType: string;
}

export interface ChangePeriodRatio {
  newPeriodRatio: number;
}

export interface ChangeBridgeVRdKn {
  newBridgeVRdKn: number;
}

export interface ChangeBearingDEdMm {
  newBearingDEdMm: number;
}

export interface ChangeBearingDRdMm {
  newBearingDRdMm: number;
}

export interface ChangeRetrofitKnowledgeLevel {
  newRetrofitKnowledgeLevel: string;
}

export interface ChangeRetrofitLimitState {
  newRetrofitLimitState: string;
}

export interface ChangeRetrofitEDKn {
  newRetrofitEDKn: number;
}

export interface ChangeRetrofitRKKn {
  newRetrofitRKKn: number;
}

export interface ChangeRetrofitGammaEl {
  newRetrofitGammaEl: number;
}

export interface ChangeSiloHeightM {
  newSiloHeightM: number;
}

export interface ChangeSiloRadiusM {
  newSiloRadiusM: number;
}

export interface ChangeSiloNRdKn {
  newSiloNRdKn: number;
}

export interface ChangeSiloVEdKn {
  newSiloVEdKn: number;
}

export interface ChangeSiloVRdKn {
  newSiloVRdKn: number;
}

export interface ChangeSiloQNominal {
  newSiloQNominal: number;
}

export interface ChangeTankHeightM {
  newTankHeightM: number;
}

export interface ChangeTankRadiusM {
  newTankRadiusM: number;
}

export interface ChangeTankMassT {
  newTankMassT: number;
}

export interface ChangeTankVRdKn {
  newTankVRdKn: number;
}

export interface ChangeTowerMEdKnm {
  newTowerMEdKnm: number;
}

export interface ChangeTowerMRdKnm {
  newTowerMRdKnm: number;
}

export interface ChangeTowerIsChimney {
  newTowerIsChimney: boolean;
}

export interface ChangeTowerQNominal {
  newTowerQNominal: number;
}

export interface ChangeTowerMassT {
  newTowerMassT: number;
}

export interface ChangeFoundationAreaM2 {
  newFoundationAreaM2: number;
}

export interface ChangeFoundationPRdKpa {
  newFoundationPRdKpa: number;
}

export interface ChangeFoundationHEdKn {
  newFoundationHEdKn: number;
}

export interface ChangeFoundationHRdKn {
  newFoundationHRdKn: number;
}

export interface ChangeKFoundation {
  newKFoundation: number;
}

export interface ChangeKSoil {
  newKSoil: number;
}

export interface ChangeWallHeightM {
  newWallHeightM: number;
}

export interface ChangeWallPhiDeg {
  newWallPhiDeg: number;
}

export interface ChangeWallSoilGammaKnM3 {
  newWallSoilGammaKnM3: number;
}

export interface ChangeWallR {
  newWallR: number;
}

export interface ChangeWallHRdKn {
  newWallHRdKn: number;
}

export type En1998Mutation =
  | ({ mutation: "changeSeismicZone" } & ChangeSeismicZone)
  | ({ mutation: "changeGroundType" } & ChangeGroundType)
  | ({ mutation: "changeImportanceClass" } & ChangeImportanceClass)
  | ({ mutation: "changeStructuralSystem" } & ChangeStructuralSystem)
  | ({ mutation: "changeT1S" } & ChangeT1S)
  | ({ mutation: "changeMassT" } & ChangeMassT)
  | ({ mutation: "changeVRdKn" } & ChangeVRdKn)
  | ({ mutation: "changeDriftMm" } & ChangeDriftMm)
  | ({ mutation: "changeHeightM" } & ChangeHeightM)
  | ({ mutation: "changeMultipleResistingSystems" } & ChangeMultipleResistingSystems)
  | ({ mutation: "changeAnnex" } & ChangeAnnex)
  | ({ mutation: "changeEnAGr" } & ChangeEnAGr)
  | ({ mutation: "changeEnGroundType" } & ChangeEnGroundType)
  | ({ mutation: "changeEnSpectrumType" } & ChangeEnSpectrumType)
  | ({ mutation: "changePeriodRatio" } & ChangePeriodRatio)
  | ({ mutation: "changeBridgeVRdKn" } & ChangeBridgeVRdKn)
  | ({ mutation: "changeBearingDEdMm" } & ChangeBearingDEdMm)
  | ({ mutation: "changeBearingDRdMm" } & ChangeBearingDRdMm)
  | ({ mutation: "changeRetrofitKnowledgeLevel" } & ChangeRetrofitKnowledgeLevel)
  | ({ mutation: "changeRetrofitLimitState" } & ChangeRetrofitLimitState)
  | ({ mutation: "changeRetrofitEDKn" } & ChangeRetrofitEDKn)
  | ({ mutation: "changeRetrofitRKKn" } & ChangeRetrofitRKKn)
  | ({ mutation: "changeRetrofitGammaEl" } & ChangeRetrofitGammaEl)
  | ({ mutation: "changeSiloHeightM" } & ChangeSiloHeightM)
  | ({ mutation: "changeSiloRadiusM" } & ChangeSiloRadiusM)
  | ({ mutation: "changeSiloNRdKn" } & ChangeSiloNRdKn)
  | ({ mutation: "changeSiloVEdKn" } & ChangeSiloVEdKn)
  | ({ mutation: "changeSiloVRdKn" } & ChangeSiloVRdKn)
  | ({ mutation: "changeSiloQNominal" } & ChangeSiloQNominal)
  | ({ mutation: "changeTankHeightM" } & ChangeTankHeightM)
  | ({ mutation: "changeTankRadiusM" } & ChangeTankRadiusM)
  | ({ mutation: "changeTankMassT" } & ChangeTankMassT)
  | ({ mutation: "changeTankVRdKn" } & ChangeTankVRdKn)
  | ({ mutation: "changeTowerMEdKnm" } & ChangeTowerMEdKnm)
  | ({ mutation: "changeTowerMRdKnm" } & ChangeTowerMRdKnm)
  | ({ mutation: "changeTowerIsChimney" } & ChangeTowerIsChimney)
  | ({ mutation: "changeTowerQNominal" } & ChangeTowerQNominal)
  | ({ mutation: "changeTowerMassT" } & ChangeTowerMassT)
  | ({ mutation: "changeFoundationAreaM2" } & ChangeFoundationAreaM2)
  | ({ mutation: "changeFoundationPRdKpa" } & ChangeFoundationPRdKpa)
  | ({ mutation: "changeFoundationHEdKn" } & ChangeFoundationHEdKn)
  | ({ mutation: "changeFoundationHRdKn" } & ChangeFoundationHRdKn)
  | ({ mutation: "changeKFoundation" } & ChangeKFoundation)
  | ({ mutation: "changeKSoil" } & ChangeKSoil)
  | ({ mutation: "changeWallHeightM" } & ChangeWallHeightM)
  | ({ mutation: "changeWallPhiDeg" } & ChangeWallPhiDeg)
  | ({ mutation: "changeWallSoilGammaKnM3" } & ChangeWallSoilGammaKnM3)
  | ({ mutation: "changeWallR" } & ChangeWallR)
  | ({ mutation: "changeWallHRdKn" } & ChangeWallHRdKn);
