/** 🧬️ En1993 document mutations — discriminated union mirroring `En1993Mutation` (WASM wiring). */

export interface ChangeAnnex {
  newAnnex: "En" | "De";
}

export interface UpdateMemberProperties {
  newNEdKn: number;
  newMEdKnm: number;
  newVEdKn: number;
  newAMm2: number;
  newAVMm2: number;
  newWPlMm3: number;
  newFYMpa: number;
  newFUMpa: number;
  newChi: number;
  newANetMm2: number;
  newTensionNEdKn: number;
}

export interface UpdateFireInputs {
  newFireThicknessMm: number;
  newFireRating: string;
  newFireMassivity: number;
  newFireMu0: number;
  newFireDesignTemperatureC: number;
}

export interface UpdateColdFormedInputs {
  newCfBBarMm: number;
  newCfTMm: number;
  newCfKSigma: number;
  newCfPsi: number;
  newCfNEdKn: number;
  newCfGrossResistanceKn: number;
}

export interface UpdateStainlessInputs {
  newStainlessMEdKnm: number;
  newStainlessWPlMm3: number;
  newStainlessFYMpa: number;
}

export interface UpdatePlatedInputs {
  newPlatedLambdaP: number;
  newPlatedSigmaEdMpa: number;
}

export interface UpdateSiloShellInputs {
  newSiloTMm: number;
  newSiloRMm: number;
  newShellSigmaXEdMpa: number;
  newSiloK: number;
  newSiloGammaKnM3: number;
  newSiloDepthM: number;
}

export interface UpdateBoltInputs {
  newBoltFEdKn: number;
  newBoltNBolts: number;
  newBoltASMm2: number;
  newBoltE1Mm: number;
  newBoltE2Mm: number;
  newBoltD0Mm: number;
  newBoltDMm: number;
  newBoltTMm: number;
  newBoltFUMpa: number;
  newBoltFUbMpa: number;
}

export interface UpdateWeldInputs {
  newWeldAMm: number;
  newWeldLMm: number;
  newWeldFUMpa: number;
  newWeldSteelGrade: string;
  newWeldFEdKn: number;
}

export interface UpdateFatigueInputs {
  newDeltaSigmaMpa: number;
  newFatigueCategory: number;
  newFatigueMethod: string;
}

export interface UpdateThroughThicknessInputs {
  newT10SteelSubgrade: string;
  newT10ActualThicknessMm: number;
  newT10TEdC: number;
}

export interface UpdateTensionComponentInputs {
  newTensionComponentFUkKn: number;
  newTensionComponentFKKn: number;
  newTensionComponentNEdKn: number;
}

export interface UpdateHssInputs {
  newHssWElMm3: number;
  newHssFYMpa: number;
  newHssSectionClass: number;
  newHssMEdKnm: number;
}

export interface UpdateBridgeInputs {
  newBridgeLambda: number;
  newBridgePhi2: number;
  newBridgeDeltaSigmaPMpa: number;
}

export interface UpdateTowerInputs {
  newTowerWindFactor: number;
  newTowerNEdKn: number;
}

export interface UpdatePileInputs {
  newPileSigmaMpa: number;
  newPileKRed: number;
  newPileNEdKn: number;
}

export interface UpdateCraneInputs {
  newCraneFZEdKn: number;
  newCraneWheelContactLengthMm: number;
  newCraneDispersionMm: number;
  newCraneTWMm: number;
}

export type En1993Mutation =
  | { ChangeAnnex: ChangeAnnex }
  | { UpdateMemberProperties: UpdateMemberProperties }
  | { UpdateFireInputs: UpdateFireInputs }
  | { UpdateColdFormedInputs: UpdateColdFormedInputs }
  | { UpdateStainlessInputs: UpdateStainlessInputs }
  | { UpdatePlatedInputs: UpdatePlatedInputs }
  | { UpdateSiloShellInputs: UpdateSiloShellInputs }
  | { UpdateBoltInputs: UpdateBoltInputs }
  | { UpdateWeldInputs: UpdateWeldInputs }
  | { UpdateFatigueInputs: UpdateFatigueInputs }
  | { UpdateThroughThicknessInputs: UpdateThroughThicknessInputs }
  | { UpdateTensionComponentInputs: UpdateTensionComponentInputs }
  | { UpdateHssInputs: UpdateHssInputs }
  | { UpdateBridgeInputs: UpdateBridgeInputs }
  | { UpdateTowerInputs: UpdateTowerInputs }
  | { UpdatePileInputs: UpdatePileInputs }
  | { UpdateCraneInputs: UpdateCraneInputs };
