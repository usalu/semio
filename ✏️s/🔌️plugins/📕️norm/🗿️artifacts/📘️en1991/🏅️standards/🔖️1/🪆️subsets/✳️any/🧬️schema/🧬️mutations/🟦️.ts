/** 🧬️ En1991 document mutations — discriminated union mirroring `En1991Mutation` (WASM wiring). */

export interface ChangeAreaM2 {
  newAreaM2: number;
}

export interface ChangeCategory {
  newCategory: "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H";
}

export interface ChangeAnnex {
  newAnnex: "En" | "De";
}

export interface ChangeSelfWeightMaterial {
  newSelfWeightMaterial: string;
}

export interface ChangeSelfWeightThicknessM {
  newSelfWeightThicknessM: number;
}

export interface ChangeAssumedGKKnM2 {
  newAssumedGKKnM2: number;
}

export interface ChangeFireCurve {
  newFireCurve: "Standard" | "External" | "Hydrocarbon";
}

export interface ChangeFireResistanceMin {
  newFireResistanceMin: number;
}

export interface ChangeFireMemberCapacityC {
  newFireMemberCapacityC: number;
}

export interface ChangeSnowZone {
  newSnowZone: number;
}

export interface ChangeSnowAltitudeM {
  newSnowAltitudeM: number;
}

export interface ChangeEnSKKnM2 {
  newEnSKKnM2: number;
}

export interface ChangeWindZone {
  newWindZone: number;
}

export interface ChangeEnVBMS {
  newEnVBMS: number;
}

export interface ChangeDeltaTK {
  newDeltaTK: number;
}

export interface ChangeConstructionActivity {
  newConstructionActivity: string;
}

export interface ChangeAccidentalMassT {
  newAccidentalMassT: number;
}

export interface ChangeAccidentalSpeedKmH {
  newAccidentalSpeedKmH: number;
}

export interface ChangeBridgeLane {
  newBridgeLane: number;
}

export interface ChangeBridgeSpanM {
  newBridgeSpanM: number;
}

export interface ChangeBridgeLaneWidthM {
  newBridgeLaneWidthM: number;
}

export interface ChangeBridgeMomentResistanceKnm {
  newBridgeMomentResistanceKnm: number;
}

export interface ChangeCraneClass {
  newCraneClass: string;
}

export interface ChangeHoistClass {
  newHoistClass: string;
}

export interface ChangeHoistingSpeedMS {
  newHoistingSpeedMS: number;
}

export interface ChangeSiloBulkDensityKnM3 {
  newSiloBulkDensityKnM3: number;
}

export interface ChangeSiloHeightM {
  newSiloHeightM: number;
}

export interface ChangeSiloHydraulicRadiusM {
  newSiloHydraulicRadiusM: number;
}

export interface ChangeSiloMu {
  newSiloMu: number;
}

export interface ChangeSiloK {
  newSiloK: number;
}

export interface ChangeCS {
  newCS: number;
}

export interface ChangeCD {
  newCD: number;
}

export type En1991Mutation =
  | { ChangeAreaM2: ChangeAreaM2 }
  | { ChangeCategory: ChangeCategory }
  | { ChangeAnnex: ChangeAnnex }
  | { ChangeSelfWeightMaterial: ChangeSelfWeightMaterial }
  | { ChangeSelfWeightThicknessM: ChangeSelfWeightThicknessM }
  | { ChangeAssumedGKKnM2: ChangeAssumedGKKnM2 }
  | { ChangeFireCurve: ChangeFireCurve }
  | { ChangeFireResistanceMin: ChangeFireResistanceMin }
  | { ChangeFireMemberCapacityC: ChangeFireMemberCapacityC }
  | { ChangeSnowZone: ChangeSnowZone }
  | { ChangeSnowAltitudeM: ChangeSnowAltitudeM }
  | { ChangeEnSKKnM2: ChangeEnSKKnM2 }
  | { ChangeWindZone: ChangeWindZone }
  | { ChangeEnVBMS: ChangeEnVBMS }
  | { ChangeDeltaTK: ChangeDeltaTK }
  | { ChangeConstructionActivity: ChangeConstructionActivity }
  | { ChangeAccidentalMassT: ChangeAccidentalMassT }
  | { ChangeAccidentalSpeedKmH: ChangeAccidentalSpeedKmH }
  | { ChangeBridgeLane: ChangeBridgeLane }
  | { ChangeBridgeSpanM: ChangeBridgeSpanM }
  | { ChangeBridgeLaneWidthM: ChangeBridgeLaneWidthM }
  | { ChangeBridgeMomentResistanceKnm: ChangeBridgeMomentResistanceKnm }
  | { ChangeCraneClass: ChangeCraneClass }
  | { ChangeHoistClass: ChangeHoistClass }
  | { ChangeHoistingSpeedMS: ChangeHoistingSpeedMS }
  | { ChangeSiloBulkDensityKnM3: ChangeSiloBulkDensityKnM3 }
  | { ChangeSiloHeightM: ChangeSiloHeightM }
  | { ChangeSiloHydraulicRadiusM: ChangeSiloHydraulicRadiusM }
  | { ChangeSiloMu: ChangeSiloMu }
  | { ChangeSiloK: ChangeSiloK }
  | { ChangeCS: ChangeCS }
  | { ChangeCD: ChangeCD };
