export type AnnexChoice = "en" | "de";

export interface CharacteristicAction {
  id: string;
  kind: string;
  category: string;
  stage: string;
  qAreaPa: number;
  fKN: number;
  deltaSigmaKPa: number;
  deltaTauKPa: number;
}

export interface ColumnAction {
  id: string;
  kind: string;
  category: string;
  stage: string;
  nKN: number;
  mKNm: number;
}

export interface SteelSection {
  designation: string;
  heightM: number;
  widthM: number;
  twM: number;
  tfM: number;
  aM2: number;
  wPlYM3: number;
  iYM4: number;
  aVM2: number;
}

export interface ProfiledSheeting {
  profile: string;
  heightM: number;
  ribWidthM: number;
  thicknessM: number;
  ribsParallelToBeam: boolean;
}

export interface HeadedStuds {
  diameterM: number;
  heightM: number;
  fUPa: number;
  countPerRib: number;
  spacingM: number;
  totalCount: number;
}

export interface CompositeBeam {
  id: string;
  spanM: number;
  spacingM: number;
  support: string;
  construction: string;
  steel: SteelSection;
  slabThicknessM: number;
  concreteFCkPa: number;
  concreteECmPa: number;
  sheeting: ProfiledSheeting;
  studs: HeadedStuds;
  transverseAsM2PerM: number;
  ltbLengthM: number;
  asHoggingM2PerM: number;
  barSpacingM: number;
  wkLimitM: number;
  nCycles: number;
  actions: CharacteristicAction[];
}

export interface CompositeColumn {
  id: string;
  kind: string;
  lengthM: number;
  outerSizeM: number;
  wallThicknessM: number;
  steelAM2: number;
  steelFYPa: number;
  concreteAM2: number;
  concreteFCkPa: number;
  reinforcementAsM2: number;
  reinforcementFYkPa: number;
  iM4: number;
  bucklingCurve: string;
  actions: ColumnAction[];
}

export interface CompositeSlab {
  id: string;
  spanM: number;
  support: string;
  sheeting: ProfiledSheeting;
  concreteThicknessM: number;
  fCkPa: number;
  mFactor: number;
  kFactor: number;
  asM2PerM: number;
  actions: CharacteristicAction[];
}

export interface En1994Snapshot {
  annex: AnnexChoice | string;
  structureKind: string;
  steelFYPa: number;
  beams: CompositeBeam[];
  columns: CompositeColumn[];
  slabs: CompositeSlab[];
  fireRating: string;
  insulationThicknessM: number;
  fatigueDetail: string;
}

/** @deprecated Use En1994Snapshot */
export type En1994Artifact = En1994Snapshot;
