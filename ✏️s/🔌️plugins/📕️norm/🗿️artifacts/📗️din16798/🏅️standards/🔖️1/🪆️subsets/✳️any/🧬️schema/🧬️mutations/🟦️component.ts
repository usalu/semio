/** 🧬️ Din16798 document mutations — discriminated union mirroring `Din16798Mutation` (WASM wiring). */

export interface ChangeAnnex {
  newAnnex: "En" | "De";
}

export interface ChangeOccupancy {
  newOccupancy: string;
}

export interface ChangeComfortCategory {
  newComfortCategory: string;
}

export interface ChangeTOpC {
  newTOpC: number;
}

export interface ChangeRhPercent {
  newRhPercent: number;
}

export interface ChangeAirSpeedMS {
  newAirSpeedMS: number;
}

export interface ChangeThetaRmC {
  newThetaRmC: number;
}

export interface ChangeCo2Ppm {
  newCo2Ppm: number;
}

export interface ChangeDfPercent {
  newDfPercent: number;
}

export interface ChangeLAeqDb {
  newLAeqDb: number;
}

export interface ChangePersons {
  newPersons: number;
}

export interface ChangeIdaClass {
  newIdaClass: string;
}

export interface ChangeVentilationM3H {
  newVentilationM3H: number;
}

export interface ChangeFloorAreaM2 {
  newFloorAreaM2: number;
}

export interface ChangeBedrooms {
  newBedrooms: number;
}

export interface ChangeDwellingVentilationM3H {
  newDwellingVentilationM3H: number;
}

export interface ChangeOccupants {
  newOccupants: number;
}

export interface ChangeResidentialVentilationM3H {
  newResidentialVentilationM3H: number;
}

export interface ChangeSfpWM3S {
  newSfpWM3S: number;
}

export interface ChangeSfpRequiredClass {
  newSfpRequiredClass: number;
}

export interface ChangeHeatRecoveryEta {
  newHeatRecoveryEta: number;
}

export interface ChangeHeatRecoveryEtaMin {
  newHeatRecoveryEtaMin: number;
}

export interface ChangeSystemType {
  newSystemType: string;
}

export interface ChangeYearsSinceInspection {
  newYearsSinceInspection: number;
}

export interface ChangeHumidificationRequiredKgH {
  newHumidificationRequiredKgH: number;
}

export interface ChangeHumidificationProvidedKgH {
  newHumidificationProvidedKgH: number;
}

export interface ChangeFanQVM3S {
  newFanQVM3S: number;
}

export interface ChangeFanTRunH {
  newFanTRunH: number;
}

export interface ChangeFanEnergyReferenceKwh {
  newFanEnergyReferenceKwh: number;
}

export interface ChangeNightSetbackK {
  newNightSetbackK: number;
}

export interface ChangeHrMDotKgS {
  newHrMDotKgS: number;
}

export interface ChangeHrCpJKgk {
  newHrCpJKgk: number;
}

export interface ChangeHrDeltaTC {
  newHrDeltaTC: number;
}

export interface ChangeHrTH {
  newHrTH: number;
}

export interface ChangeHrSavingsReferenceKwh {
  newHrSavingsReferenceKwh: number;
}

export interface ChangeN50HInv {
  newN50HInv: number;
}

export interface ChangeVolumeM3 {
  newVolumeM3: number;
}

export interface ChangeInfiltrationAllowanceM3H {
  newInfiltrationAllowanceM3H: number;
}

export interface ChangeCellarAreaM2 {
  newCellarAreaM2: number;
}

export interface ChangeCellarVentilationM3H {
  newCellarVentilationM3H: number;
}

export interface ChangeHTrWK {
  newHTrWK: number;
}

export interface ChangeHVeWK {
  newHVeWK: number;
}

export interface ChangeThetaEC {
  newThetaEC: number;
}

export interface ChangeThetaSetC {
  newThetaSetC: number;
}

export interface ChangeCoolingDeltaTH {
  newCoolingDeltaTH: number;
}

export interface ChangeCoolingGainsKwh {
  newCoolingGainsKwh: number;
}

export interface ChangeCoolingUtilizationFactor {
  newCoolingUtilizationFactor: number;
}

export interface ChangeCoolingReferenceKwh {
  newCoolingReferenceKwh: number;
}

export interface ChangeChillerType {
  newChillerType: string;
}

export interface ChangeEerActual {
  newEerActual: number;
}

export interface ChangeQCKwh {
  newQCKwh: number;
}

export interface ChangeGenerationReferenceKwh {
  newGenerationReferenceKwh: number;
}

export interface ChangeDataCenterSupplyC {
  newDataCenterSupplyC: number;
}

export interface ChangeHStWK {
  newHStWK: number;
}

export interface ChangeThetaStC {
  newThetaStC: number;
}

export interface ChangeThetaAmbC {
  newThetaAmbC: number;
}

export interface ChangeStorageTH {
  newStorageTH: number;
}

export interface ChangeStorageAllowanceKwh {
  newStorageAllowanceKwh: number;
}

export interface ChangeDhwDeliveryC {
  newDhwDeliveryC: number;
}

export interface ChangeDuctClass {
  newDuctClass: string;
}

export interface ChangeDuctTestPressurePa {
  newDuctTestPressurePa: number;
}

export interface ChangeDuctLeakageM3SM2 {
  newDuctLeakageM3SM2: number;
}

export type Din16798Mutation =
  | ({ mutation: "changeAnnex" } & ChangeAnnex)
  | ({ mutation: "changeOccupancy" } & ChangeOccupancy)
  | ({ mutation: "changeComfortCategory" } & ChangeComfortCategory)
  | ({ mutation: "changeTOpC" } & ChangeTOpC)
  | ({ mutation: "changeRhPercent" } & ChangeRhPercent)
  | ({ mutation: "changeAirSpeedMS" } & ChangeAirSpeedMS)
  | ({ mutation: "changeThetaRmC" } & ChangeThetaRmC)
  | ({ mutation: "changeCo2Ppm" } & ChangeCo2Ppm)
  | ({ mutation: "changeDfPercent" } & ChangeDfPercent)
  | ({ mutation: "changeLAeqDb" } & ChangeLAeqDb)
  | ({ mutation: "changePersons" } & ChangePersons)
  | ({ mutation: "changeIdaClass" } & ChangeIdaClass)
  | ({ mutation: "changeVentilationM3H" } & ChangeVentilationM3H)
  | ({ mutation: "changeFloorAreaM2" } & ChangeFloorAreaM2)
  | ({ mutation: "changeBedrooms" } & ChangeBedrooms)
  | ({ mutation: "changeDwellingVentilationM3H" } & ChangeDwellingVentilationM3H)
  | ({ mutation: "changeOccupants" } & ChangeOccupants)
  | ({ mutation: "changeResidentialVentilationM3H" } & ChangeResidentialVentilationM3H)
  | ({ mutation: "changeSfpWM3S" } & ChangeSfpWM3S)
  | ({ mutation: "changeSfpRequiredClass" } & ChangeSfpRequiredClass)
  | ({ mutation: "changeHeatRecoveryEta" } & ChangeHeatRecoveryEta)
  | ({ mutation: "changeHeatRecoveryEtaMin" } & ChangeHeatRecoveryEtaMin)
  | ({ mutation: "changeSystemType" } & ChangeSystemType)
  | ({ mutation: "changeYearsSinceInspection" } & ChangeYearsSinceInspection)
  | ({ mutation: "changeHumidificationRequiredKgH" } & ChangeHumidificationRequiredKgH)
  | ({ mutation: "changeHumidificationProvidedKgH" } & ChangeHumidificationProvidedKgH)
  | ({ mutation: "changeFanQVM3S" } & ChangeFanQVM3S)
  | ({ mutation: "changeFanTRunH" } & ChangeFanTRunH)
  | ({ mutation: "changeFanEnergyReferenceKwh" } & ChangeFanEnergyReferenceKwh)
  | ({ mutation: "changeNightSetbackK" } & ChangeNightSetbackK)
  | ({ mutation: "changeHrMDotKgS" } & ChangeHrMDotKgS)
  | ({ mutation: "changeHrCpJKgk" } & ChangeHrCpJKgk)
  | ({ mutation: "changeHrDeltaTC" } & ChangeHrDeltaTC)
  | ({ mutation: "changeHrTH" } & ChangeHrTH)
  | ({ mutation: "changeHrSavingsReferenceKwh" } & ChangeHrSavingsReferenceKwh)
  | ({ mutation: "changeN50HInv" } & ChangeN50HInv)
  | ({ mutation: "changeVolumeM3" } & ChangeVolumeM3)
  | ({ mutation: "changeInfiltrationAllowanceM3H" } & ChangeInfiltrationAllowanceM3H)
  | ({ mutation: "changeCellarAreaM2" } & ChangeCellarAreaM2)
  | ({ mutation: "changeCellarVentilationM3H" } & ChangeCellarVentilationM3H)
  | ({ mutation: "changeHTrWK" } & ChangeHTrWK)
  | ({ mutation: "changeHVeWK" } & ChangeHVeWK)
  | ({ mutation: "changeThetaEC" } & ChangeThetaEC)
  | ({ mutation: "changeThetaSetC" } & ChangeThetaSetC)
  | ({ mutation: "changeCoolingDeltaTH" } & ChangeCoolingDeltaTH)
  | ({ mutation: "changeCoolingGainsKwh" } & ChangeCoolingGainsKwh)
  | ({ mutation: "changeCoolingUtilizationFactor" } & ChangeCoolingUtilizationFactor)
  | ({ mutation: "changeCoolingReferenceKwh" } & ChangeCoolingReferenceKwh)
  | ({ mutation: "changeChillerType" } & ChangeChillerType)
  | ({ mutation: "changeEerActual" } & ChangeEerActual)
  | ({ mutation: "changeQCKwh" } & ChangeQCKwh)
  | ({ mutation: "changeGenerationReferenceKwh" } & ChangeGenerationReferenceKwh)
  | ({ mutation: "changeDataCenterSupplyC" } & ChangeDataCenterSupplyC)
  | ({ mutation: "changeHStWK" } & ChangeHStWK)
  | ({ mutation: "changeThetaStC" } & ChangeThetaStC)
  | ({ mutation: "changeThetaAmbC" } & ChangeThetaAmbC)
  | ({ mutation: "changeStorageTH" } & ChangeStorageTH)
  | ({ mutation: "changeStorageAllowanceKwh" } & ChangeStorageAllowanceKwh)
  | ({ mutation: "changeDhwDeliveryC" } & ChangeDhwDeliveryC)
  | ({ mutation: "changeDuctClass" } & ChangeDuctClass)
  | ({ mutation: "changeDuctTestPressurePa" } & ChangeDuctTestPressurePa)
  | ({ mutation: "changeDuctLeakageM3SM2" } & ChangeDuctLeakageM3SM2);
