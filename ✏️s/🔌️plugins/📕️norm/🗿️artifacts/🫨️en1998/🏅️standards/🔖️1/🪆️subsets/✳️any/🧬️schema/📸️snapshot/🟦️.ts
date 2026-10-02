/** 📸️ `En1998Snapshot` wire twin: the persisted snapshot and every record it holds, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1998Snapshot {
  annex: string;
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

export interface En1998Site {
  seismicZone: DeSeismicZone;
  aGr: number;
  deGroundCombo: DeGroundCombo;
  enGroundType?: string;
  enSpectrumType?: string;
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

export interface En1998VariableAction {
  id: string;
  category: string;
  qkN: number;
}

export type DeSeismicZone = "zone0" | "zone1" | "zone2" | "zone3";

export type DeGroundCombo = "A-R" | "B-R" | "C-R" | "B-T" | "C-T" | "C-S";

export const parseEn1998Snapshot: NormWireReader<En1998Snapshot> = normWireObject<En1998Snapshot>({ annex: normWireRequired(normWireString), site: normWireRequired(normWireRef(() => parseEn1998Site)), buildings: normWireRequired(normWireArray(normWireRef(() => parseEn1998Building))), bridges: normWireRequired(normWireArray(normWireRef(() => parseEn1998Bridge))), assessments: normWireRequired(normWireArray(normWireRef(() => parseEn1998Assessment))), silos: normWireRequired(normWireArray(normWireRef(() => parseEn1998Silo))), tanks: normWireRequired(normWireArray(normWireRef(() => parseEn1998Tank))), foundations: normWireRequired(normWireArray(normWireRef(() => parseEn1998Foundation))), retainingWalls: normWireRequired(normWireArray(normWireRef(() => parseEn1998RetainingWall))), towers: normWireRequired(normWireArray(normWireRef(() => parseEn1998Tower))) });
export const parseEn1998Site: NormWireReader<En1998Site> = normWireObject<En1998Site>({ seismicZone: normWireRequired(normWireRef(() => parseDeSeismicZone)), aGr: normWireRequired(normWireNumber), deGroundCombo: normWireRequired(normWireRef(() => parseDeGroundCombo)), enGroundType: normWireOptional(normWireString), enSpectrumType: normWireOptional(normWireString), importanceClass: normWireRequired(normWireString) });
export const parseEn1998System: NormWireReader<En1998System> = normWireObject<En1998System>({ id: normWireRequired(normWireString), direction: normWireRequired(normWireString), systemType: normWireRequired(normWireString), material: normWireRequired(normWireString), ductilityClass: normWireRequired(normWireString), q0: normWireRequired(normWireNumber), alphaUOverAlpha1: normWireRequired(normWireNumber), kW: normWireRequired(normWireNumber), baseShearResistanceN: normWireRequired(normWireNumber) });
export const parseEn1998Storey: NormWireReader<En1998Storey> = normWireObject<En1998Storey>({ id: normWireRequired(normWireString), heightM: normWireRequired(normWireNumber), permanentGkN: normWireRequired(normWireNumber), correlatedOccupancy: normWireRequired(normWireBoolean), variables: normWireRequired(normWireArray(normWireRef(() => parseEn1998VariableAction))), stiffnessX: normWireRequired(normWireNumber), stiffnessY: normWireRequired(normWireNumber), centreOfMassXM: normWireRequired(normWireNumber), centreOfMassYM: normWireRequired(normWireNumber), centreOfStiffnessXM: normWireRequired(normWireNumber), centreOfStiffnessYM: normWireRequired(normWireNumber), driftXM: normWireRequired(normWireNumber), driftYM: normWireRequired(normWireNumber), shearResistanceN: normWireRequired(normWireNumber) });
export const parseEn1998Member: NormWireReader<En1998Member> = normWireObject<En1998Member>({ id: normWireRequired(normWireString), material: normWireRequired(normWireString), role: normWireRequired(normWireString), detailingCompatibleWithQ: normWireRequired(normWireBoolean), minDimensionM: normWireRequired(normWireNumber), rho: normWireRequired(normWireNumber), rhoPrime: normWireRequired(normWireNumber), omegaWd: normWireRequired(normWireNumber), steelSectionClass: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})) });
export const parseEn1998Building: NormWireReader<En1998Building> = normWireObject<En1998Building>({ id: normWireRequired(normWireString), name: normWireRequired(normWireString), planWidthM: normWireRequired(normWireNumber), planLengthM: normWireRequired(normWireNumber), systems: normWireRequired(normWireArray(normWireRef(() => parseEn1998System))), storeys: normWireRequired(normWireArray(normWireRef(() => parseEn1998Storey))), members: normWireRequired(normWireArray(normWireRef(() => parseEn1998Member))), planRegular: normWireRequired(normWireBoolean), elevationRegular: normWireRequired(normWireBoolean), t1Method: normWireRequired(normWireString), t1GivenS: normWireRequired(normWireNumber), ct: normWireRequired(normWireNumber), driftLimitClass: normWireRequired(normWireString), nu: normWireRequired(normWireNumber), multipleResistingSystems: normWireRequired(normWireBoolean), claimsSimpleMasonry: normWireRequired(normWireBoolean), masonryWallAreaRatio: normWireRequired(normWireNumber), accidentalEccentricityRatio: normWireRequired(normWireNumber) });
export const parseEn1998Bridge: NormWireReader<En1998Bridge> = normWireObject<En1998Bridge>({ id: normWireRequired(normWireString), periodRatio: normWireRequired(normWireNumber), fundamentalPeriodS: normWireRequired(normWireNumber), vRdN: normWireRequired(normWireNumber), bearingDRdM: normWireRequired(normWireNumber), permanentGkN: normWireRequired(normWireNumber), correlatedOccupancy: normWireRequired(normWireBoolean), variables: normWireRequired(normWireArray(normWireRef(() => parseEn1998VariableAction))) });
export const parseEn1998Assessment: NormWireReader<En1998Assessment> = normWireObject<En1998Assessment>({ id: normWireRequired(normWireString), knowledgeLevel: normWireRequired(normWireString), limitState: normWireRequired(normWireString), supportedBuildingId: normWireRequired(normWireString), rKN: normWireRequired(normWireNumber), gammaEl: normWireRequired(normWireNumber) });
export const parseEn1998Silo: NormWireReader<En1998Silo> = normWireObject<En1998Silo>({ id: normWireRequired(normWireString), heightM: normWireRequired(normWireNumber), radiusM: normWireRequired(normWireNumber), permanentGkN: normWireRequired(normWireNumber), contentQkN: normWireRequired(normWireNumber), contentCategory: normWireRequired(normWireString), fillingRatio: normWireRequired(normWireNumber), nRdN: normWireRequired(normWireNumber), vRdN: normWireRequired(normWireNumber), qNominal: normWireRequired(normWireNumber) });
export const parseEn1998Tank: NormWireReader<En1998Tank> = normWireObject<En1998Tank>({ id: normWireRequired(normWireString), heightM: normWireRequired(normWireNumber), radiusM: normWireRequired(normWireNumber), permanentGkN: normWireRequired(normWireNumber), contentQkN: normWireRequired(normWireNumber), contentCategory: normWireRequired(normWireString), fillingRatio: normWireRequired(normWireNumber), vRdN: normWireRequired(normWireNumber) });
export const parseEn1998Foundation: NormWireReader<En1998Foundation> = normWireObject<En1998Foundation>({ id: normWireRequired(normWireString), supportedBuildingId: normWireRequired(normWireString), areaM2: normWireRequired(normWireNumber), pRdPa: normWireRequired(normWireNumber), hRdN: normWireRequired(normWireNumber), kFoundation: normWireRequired(normWireNumber), kSoil: normWireRequired(normWireNumber) });
export const parseEn1998RetainingWall: NormWireReader<En1998RetainingWall> = normWireObject<En1998RetainingWall>({ id: normWireRequired(normWireString), heightM: normWireRequired(normWireNumber), phiDeg: normWireRequired(normWireNumber), soilGamma: normWireRequired(normWireNumber), r: normWireRequired(normWireNumber), hRdNPerM: normWireRequired(normWireNumber) });
export const parseEn1998Tower: NormWireReader<En1998Tower> = normWireObject<En1998Tower>({ id: normWireRequired(normWireString), heightM: normWireRequired(normWireNumber), mRdNm: normWireRequired(normWireNumber), isChimney: normWireRequired(normWireBoolean), qNominal: normWireRequired(normWireNumber), permanentGkN: normWireRequired(normWireNumber), correlatedOccupancy: normWireRequired(normWireBoolean), variables: normWireRequired(normWireArray(normWireRef(() => parseEn1998VariableAction))) });
export const parseEn1998VariableAction: NormWireReader<En1998VariableAction> = normWireObject<En1998VariableAction>({ id: normWireRequired(normWireString), category: normWireRequired(normWireString), qkN: normWireRequired(normWireNumber) });
export const parseDeSeismicZone: NormWireReader<DeSeismicZone> = normWireLiteral("zone0", "zone1", "zone2", "zone3");
export const parseDeGroundCombo: NormWireReader<DeGroundCombo> = normWireLiteral("A-R", "B-R", "C-R", "B-T", "C-T", "C-S");

export * from "./🪶️sqlite/🟦️.ts";
