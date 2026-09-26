/** En1996Mutation discriminated union — every mounted leaf. */
export type En1996Mutation =
  | { changeAnnex: { newAnnex: string } } |
  | { changeMasonryClass: { newMasonryClass: string } } |
  | { changeDesignSituation: { newDesignSituation: string } } |
  | { changeStoreys: { newStoreys: number } } |
  | { insertWall: { index: number; wall: string } } |
  | { removeWall: { index: number } } |
  | { changeWallThickness: { index: number; newThicknessM: number } } |
  | { changeWallHeight: { index: number; newHeightM: number } } |
  | { changeWallLength: { index: number; newLengthM: number } } |
  | { changeWallType: { index: number; newWallType: string } } |
  | { changeSupportSides: { index: number; newSupportSides: number } } |
  | { changeSlabBearingDepth: { index: number; newSlabBearingDepthM: number } } |
  | { changeEccentricityTop: { index: number; newEccentricityTopM: number } } |
  | { changeEccentricityBottom: { index: number; newEccentricityBottomM: number } } |
  | { changeUnitGroup: { index: number; newUnitGroup: string } } |
  | { changeUnitMaterial: { index: number; newUnitMaterial: string } } |
  | { changeUnitFb: { index: number; newFBPa: number } } |
  | { changeUnitLength: { index: number; newUnitLengthM: number } } |
  | { changeUnitWidth: { index: number; newUnitWidthM: number } } |
  | { changeUnitHeight: { index: number; newUnitHeightM: number } } |
  | { changeMortarType: { index: number; newMortarType: string } } |
  | { changeMortarClass: { index: number; newMortarClass: string } } |
  | { changeFm: { index: number; newFMPa: number } } |
  | { changeBedJointThickness: { index: number; newBedJointThicknessM: number } } |
  | { changeReinforced: { index: number; newReinforced: boolean } } |
  | { changeAsVertical: { index: number; newAsVerticalM2: number } } |
  | { changeAsHorizontal: { index: number; newAsHorizontalM2: number } } |
  | { changeFYd: { index: number; newFYdPa: number } } |
  | { changeFireRei: { index: number; newFireReiMin: number } } |
  | { changeExposure: { index: number; newExposure: string } } |
  | { changeMu: { index: number; newMu: number } } |
  | { changeLoadCaseSituation: { wallIndex: number; loadCaseIndex: number; newDesignSituation: string } } |
  | { changeWallLabelDe: { index: number; newLabelDe: string } } |
  | { changeWallLabelEn: { index: number; newLabelEn: string } } |
  | { insertOpening: { wallIndex: number; index: number; opening: string } } |
  | { removeOpening: { wallIndex: number; index: number } } |
  | { changeOpeningWidth: { wallIndex: number; index: number; newWidthM: number } } |
  | { changeOpeningHeight: { wallIndex: number; index: number; newHeightM: number } } |
  | { changeOpeningSill: { wallIndex: number; index: number; newSillHeightM: number } } |
  | { insertLoadCase: { wallIndex: number; index: number; loadCase: string } } |
  | { removeLoadCase: { wallIndex: number; index: number } } |
  | { changeNEdTop: { wallIndex: number; index: number; newNEdTopN: number } } |
  | { changeNEdMid: { wallIndex: number; index: number; newNEdMidN: number } } |
  | { changeNEdBottom: { wallIndex: number; index: number; newNEdBottomN: number } } |
  | { changeVEd: { wallIndex: number; index: number; newVEdN: number } } |
  | { changeWEd: { wallIndex: number; index: number; newWEdPa: number } } |
  | { insertConcentrated: { wallIndex: number; loadCaseIndex: number; index: number; load: string } } |
  | { removeConcentrated: { wallIndex: number; loadCaseIndex: number; index: number } } |
  | { changeConcentratedForce: { wallIndex: number; loadCaseIndex: number; index: number; newForceN: number } } |
  | { changeConcentratedBearingArea: { wallIndex: number; loadCaseIndex: number; index: number; newBearingAreaM2: number } } |
  | { changeConcentratedBearingLength: { wallIndex: number; loadCaseIndex: number; index: number; newBearingLengthM: number } };
