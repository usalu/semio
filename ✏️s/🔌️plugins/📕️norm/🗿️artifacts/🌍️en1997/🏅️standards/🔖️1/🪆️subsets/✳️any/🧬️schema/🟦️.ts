/** 🗿️ En1997Artifact — hierarchical geotechnical subject (SI: N, m, Pa). */

export type AnnexChoice = "de" | "en";

export interface SoilLayer {
  id: string;
  soilType: string;
  depthTop: number;
  depthBottom: number;
  gamma: number;
  gammaPrime: number;
  phiPrimeDeg: number;
  cohesionEffective: number;
  cohesionUndrained: number;
  oedometricModulus: number;
  poissonRatio: number;
  cptQc: number;
  sptN: number;
}

export interface FoundationLoadCase {
  id: string;
  designSituation: string;
  verticalPermanent: number;
  verticalVariable: number;
  horizontalPermanent: number;
  horizontalVariable: number;
  momentPermanent: number;
  momentVariable: number;
}

export interface SpreadFoundation {
  id: string;
  width: number;
  length: number;
  embedment: number;
  baseInclinationDeg: number;
  settlementLimit: number;
  loadCases: FoundationLoadCase[];
}

export interface PileTestProfile {
  id: string;
  shaftResistance: number;
  baseResistance: number;
}

export interface Pile {
  id: string;
  pileType: string;
  diameter: number;
  length: number;
  count: number;
  alphaS: number;
  unitShaftResistance: number;
  unitBaseResistance: number;
  compressionPermanent: number;
  compressionVariable: number;
  tensionPermanent: number;
  tensionVariable: number;
  testProfiles: PileTestProfile[];
}

export interface RetainingWall {
  id: string;
  height: number;
  embedment: number;
  baseWidth: number;
  stemThickness: number;
  backfillPhiDeg: number;
  backfillGamma: number;
  wallFrictionDeg: number;
  earthPressureMode: string;
  wallMovement: string;
  ocr: number;
  concreteGamma: number;
  surcharge: number;
  verticalPermanent: number;
  horizontalPermanent: number;
}

export interface Slope {
  id: string;
  angleDeg: number;
  height: number;
  length: number;
  governingLayerId: string;
}

export interface UpliftCase {
  id: string;
  permanentStabilizing: number;
  permanentDestabilizing: number;
  variableDestabilizing: number;
  porePressure: number;
  totalStress: number;
}

export interface En1997Artifact {
  structureId: string;
  geotechnicalCategory: number;
  designSituation: string;
  designApproach: string;
  annex: AnnexChoice;
  groundwaterLevel: number;
  investigationDepth: number;
  layers: SoilLayer[];
  footings: SpreadFoundation[];
  piles: Pile[];
  retainingWalls: RetainingWall[];
  slopes: Slope[];
  upliftCases: UpliftCase[];
}

export type En1997Snapshot = En1997Artifact;

export function parseEn1997Artifact(value: unknown, _at = "$"): En1997Artifact {
  return value as En1997Artifact;
}

export function parseEn1997Fields(value: unknown, _partial: boolean, at = "$"): Partial<En1997Artifact> {
  return parseEn1997Artifact(value, at);
}
