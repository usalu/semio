/** 📸️ EN 1996 snapshot TypeScript facet — SI masonry building subject. */
export type En1996Snapshot = {
  annex: string;
  masonryClass: string;
  designSituation: string;
  storeys: number;
  walls: MasonryWall[];
};

export type MasonryWall = {
  id: string;
  labelEn: string;
  labelDe: string;
  wallType: string;
  thicknessM: number;
  heightM: number;
  lengthM: number;
  supportSides: number;
  openings: WallOpening[];
  slabBearingDepthM: number;
  eccentricityTopM: number;
  eccentricityBottomM: number;
  unitGroup: string;
  unitMaterial: string;
  fBPa: number;
  unitLengthM: number;
  unitWidthM: number;
  unitHeightM: number;
  mortarType: string;
  mortarClass: string;
  mortarStrengthPa: number;
  bedJointThicknessM: number;
  reinforced: boolean;
  asVerticalM2: number;
  asHorizontalM2: number;
  fYdPa: number;
  fireReiMin: number;
  exposure: string;
  mu: number;
  densityKgM3: number;
  phiInfinity: number;
  isBasement: boolean;
  loadCases: WallLoadCase[];
};

export type WallOpening = { id: string; widthM: number; heightM: number; sillHeightM: number };

export type WallLoadCase = {
  id: string;
  designSituation: string;
  imposedCategory: string;
  gKSlabN: number;
  qKImposedPa: number;
  tributaryAreaM2: number;
  slabSpanM: number;
  qKSnowPa: number;
  qPWindPa: number;
  cPe: number;
  hKEarthN: number;
  concentrated: ConcentratedLoad[];
};

export type ConcentratedLoad = { id: string; forceN: number; bearingAreaM2: number; bearingLengthM: number };
