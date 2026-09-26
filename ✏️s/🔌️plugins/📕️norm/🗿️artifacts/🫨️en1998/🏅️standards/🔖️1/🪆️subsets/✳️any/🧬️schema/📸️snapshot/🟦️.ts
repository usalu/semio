/** @parity 🔣️.json En1998Snapshot — SI camelCase nested subject. */
export type DeSeismicZone = "zone0" | "zone1" | "zone2" | "zone3";
export type DeGroundCombo = "A-R" | "B-R" | "C-R" | "B-T" | "C-T" | "C-S";
export type En1998LimitState = "nc" | "sd" | "dl";

export interface En1998Site {
  seismicZone: DeSeismicZone;
  aGr: number;
  deGroundCombo: DeGroundCombo;
  enGroundType: string;
  enSpectrumType: string;
  importanceClass: string;
}

export interface En1998System {
  id: string;
  direction: string;
  systemType: string;
  material: string;
  ductilityClass: string;
  q0: number;
  alphaUOverAlpha1: number;
  kW: number;
  baseShearResistanceN: number;
}

export interface En1998VariableAction {
  id: string;
  category: string;
  qkN: number;
}

export interface En1998Storey {
  id: string;
  heightM: number;
  permanentGkN: number;
  correlatedOccupancy: boolean;
  variables: En1998VariableAction[];
  stiffnessX: number;
  stiffnessY: number;
  centreOfMassXM: number;
  centreOfMassYM: number;
  centreOfStiffnessXM: number;
  centreOfStiffnessYM: number;
  driftXM: number;
  driftYM: number;
  shearResistanceN: number;
}

export interface En1998Member {
  id: string;
  material: string;
  role: string;
  detailingCompatibleWithQ: boolean;
  minDimensionM: number;
  rho: number;
  rhoPrime: number;
  omegaWd: number;
  steelSectionClass: number;
}

export interface En1998Building {
  id: string;
  name: string;
  planWidthM: number;
  planLengthM: number;
  systems: En1998System[];
  storeys: En1998Storey[];
  members: En1998Member[];
  planRegular: boolean;
  elevationRegular: boolean;
  t1Method: string;
  t1GivenS: number;
  ct: number;
  driftLimitClass: string;
  nu: number;
  multipleResistingSystems: boolean;
  claimsSimpleMasonry: boolean;
  masonryWallAreaRatio: number;
  accidentalEccentricityRatio: number;
}

export interface En1998Bridge {
  id: string;
  periodRatio: number;
  fundamentalPeriodS: number;
  vRdN: number;
  bearingDRdM: number;
  permanentGkN: number;
  correlatedOccupancy: boolean;
  variables: En1998VariableAction[];
}

export interface En1998Assessment {
  id: string;
  knowledgeLevel: string;
  limitState: string;
  supportedBuildingId: string;
  rKN: number;
  gammaEl: number;
}

export interface En1998Silo {
  id: string;
  heightM: number;
  radiusM: number;
  permanentGkN: number;
  contentQkN: number;
  contentCategory: string;
  fillingRatio: number;
  nRdN: number;
  vRdN: number;
  qNominal: number;
}

export interface En1998Tank {
  id: string;
  heightM: number;
  radiusM: number;
  permanentGkN: number;
  contentQkN: number;
  contentCategory: string;
  fillingRatio: number;
  vRdN: number;
}

export interface En1998Foundation {
  id: string;
  supportedBuildingId: string;
  areaM2: number;
  pRdPa: number;
  hRdN: number;
  kFoundation: number;
  kSoil: number;
}

export interface En1998RetainingWall {
  id: string;
  heightM: number;
  phiDeg: number;
  soilGamma: number;
  r: number;
  hRdNPerM: number;
}

export interface En1998Tower {
  id: string;
  heightM: number;
  mRdNm: number;
  isChimney: boolean;
  qNominal: number;
  permanentGkN: number;
  correlatedOccupancy: boolean;
  variables: En1998VariableAction[];
}

export interface En1998Snapshot {
  annex: "de" | "en";
  site: En1998Site;
  buildings: En1998Building[];
  bridges: En1998Bridge[];
  assessments: En1998Assessment[];
  silos: En1998Silo[];
  tanks: En1998Tank[];
  foundations: En1998Foundation[];
  retainingWalls: En1998RetainingWall[];
  towers: En1998Tower[];
}
