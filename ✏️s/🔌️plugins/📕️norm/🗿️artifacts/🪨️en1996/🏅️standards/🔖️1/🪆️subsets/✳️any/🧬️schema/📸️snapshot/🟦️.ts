/** 📸️ `En1996Snapshot` wire twin: the persisted snapshot and every record it holds, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1996Snapshot {
  annex: "En" | "De";
  masonryClass: "Class1" | "Class2" | "Class3" | "Class4" | "Class5";
  designSituation: "Persistent" | "Transient" | "Accidental" | "Seismic";
  storeys: number;
  walls: ({ id: string; labelEn: string; labelDe: string; wallType: "LoadBearing" | "Shear" | "NonLoadBearing"; thicknessM: number; heightM: number; lengthM: number; supportSides: number; openings: { id: string; widthM: number; heightM: number; sillHeightM: number; }[]; slabBearingDepthM: number; eccentricityTopM: number; eccentricityBottomM: number; unitGroup: "Group1" | "Group2" | "Group3" | "Group4"; unitMaterial: "Clay" | "CalciumSilicate" | "Aerated" | "Concrete"; fBPa: number; unitLengthM: number; unitWidthM: number; unitHeightM: number; mortarType: "GeneralPurpose" | "ThinLayer" | "Lightweight"; mortarClass: "M1" | "M2_5" | "M5" | "M10" | "M15" | "M20"; mortarStrengthPa: number; bedJointThicknessM: number; reinforced: boolean; asVerticalM2: number; asHorizontalM2: number; fYdPa: number; fireReiMin: number; exposure: "Mx1" | "Mx2" | "Mx3" | "Mx4" | "Mx5"; mu: number; densityKgM3: number; phiInfinity: number; isBasement: boolean; loadCases: { id: string; designSituation: string; imposedCategory: string; gKSlabN: number; qKImposedPa: number; tributaryAreaM2: number; slabSpanM: number; qKSnowPa: number; qPWindPa: number; cPe: number; hKEarthN: number; concentrated: { id: string; forceN: number; bearingAreaM2: number; bearingLengthM: number; }[]; }[]; })[];
}

export interface ConcentratedLoad {
  id: string;
  forceN: number;
  bearingAreaM2: number;
  bearingLengthM: number;
}

export interface WallLoadCase {
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
}

export interface WallOpening {
  id: string;
  widthM: number;
  heightM: number;
  sillHeightM: number;
}

export interface MasonryWall {
  id: string;
  labelEn: string;
  labelDe: string;
  wallType: En1996WallType;
  thicknessM: number;
  heightM: number;
  lengthM: number;
  supportSides: number;
  openings: WallOpening[];
  slabBearingDepthM: number;
  eccentricityTopM: number;
  eccentricityBottomM: number;
  unitGroup: En1996UnitGroup;
  unitMaterial: En1996UnitMaterial;
  fBPa: number;
  unitLengthM: number;
  unitWidthM: number;
  unitHeightM: number;
  mortarType: En1996MortarType;
  mortarClass: En1996MortarClass;
  mortarStrengthPa: number;
  bedJointThicknessM: number;
  reinforced: boolean;
  asVerticalM2: number;
  asHorizontalM2: number;
  fYdPa: number;
  fireReiMin: number;
  exposure: En1996ExposureClass;
  mu: number;
  densityKgM3: number;
  phiInfinity: number;
  isBasement: boolean;
  loadCases: WallLoadCase[];
}

export type En1996AnnexChoice = "En" | "De";

export type En1996DesignSituation = "Persistent" | "Transient" | "Accidental" | "Seismic";

export type En1996MasonryClass = "Class1" | "Class2" | "Class3" | "Class4" | "Class5";

export type En1996ExposureClass = "Mx1" | "Mx2" | "Mx3" | "Mx4" | "Mx5";

export type En1996MortarClass = "M1" | "M2_5" | "M5" | "M10" | "M15" | "M20";

export type En1996MortarType = "GeneralPurpose" | "ThinLayer" | "Lightweight";

export type En1996UnitGroup = "Group1" | "Group2" | "Group3" | "Group4";

export type En1996UnitMaterial = "Clay" | "CalciumSilicate" | "Aerated" | "Concrete";

export type En1996WallType = "LoadBearing" | "Shear" | "NonLoadBearing";

export const parseEn1996Snapshot: NormWireReader<En1996Snapshot> = normWireObject<En1996Snapshot>({ annex: normWireRequired(normWireLiteral("En", "De")), masonryClass: normWireRequired(normWireLiteral("Class1", "Class2", "Class3", "Class4", "Class5")), designSituation: normWireRequired(normWireLiteral("Persistent", "Transient", "Accidental", "Seismic")), storeys: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), walls: normWireRequired(normWireArray(normWireObject<{ id: string; labelEn: string; labelDe: string; wallType: "LoadBearing" | "Shear" | "NonLoadBearing"; thicknessM: number; heightM: number; lengthM: number; supportSides: number; openings: { id: string; widthM: number; heightM: number; sillHeightM: number; }[]; slabBearingDepthM: number; eccentricityTopM: number; eccentricityBottomM: number; unitGroup: "Group1" | "Group2" | "Group3" | "Group4"; unitMaterial: "Clay" | "CalciumSilicate" | "Aerated" | "Concrete"; fBPa: number; unitLengthM: number; unitWidthM: number; unitHeightM: number; mortarType: "GeneralPurpose" | "ThinLayer" | "Lightweight"; mortarClass: "M1" | "M2_5" | "M5" | "M10" | "M15" | "M20"; mortarStrengthPa: number; bedJointThicknessM: number; reinforced: boolean; asVerticalM2: number; asHorizontalM2: number; fYdPa: number; fireReiMin: number; exposure: "Mx1" | "Mx2" | "Mx3" | "Mx4" | "Mx5"; mu: number; densityKgM3: number; phiInfinity: number; isBasement: boolean; loadCases: { id: string; designSituation: string; imposedCategory: string; gKSlabN: number; qKImposedPa: number; tributaryAreaM2: number; slabSpanM: number; qKSnowPa: number; qPWindPa: number; cPe: number; hKEarthN: number; concentrated: { id: string; forceN: number; bearingAreaM2: number; bearingLengthM: number; }[]; }[]; }>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), wallType: normWireRequired(normWireLiteral("LoadBearing", "Shear", "NonLoadBearing")), thicknessM: normWireRequired(normWireNumber), heightM: normWireRequired(normWireNumber), lengthM: normWireRequired(normWireNumber), supportSides: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), openings: normWireRequired(normWireArray(normWireObject<{ id: string; widthM: number; heightM: number; sillHeightM: number; }>({ id: normWireRequired(normWireString), widthM: normWireRequired(normWireNumber), heightM: normWireRequired(normWireNumber), sillHeightM: normWireRequired(normWireNumber) }))), slabBearingDepthM: normWireRequired(normWireNumber), eccentricityTopM: normWireRequired(normWireNumber), eccentricityBottomM: normWireRequired(normWireNumber), unitGroup: normWireRequired(normWireLiteral("Group1", "Group2", "Group3", "Group4")), unitMaterial: normWireRequired(normWireLiteral("Clay", "CalciumSilicate", "Aerated", "Concrete")), fBPa: normWireRequired(normWireNumber), unitLengthM: normWireRequired(normWireNumber), unitWidthM: normWireRequired(normWireNumber), unitHeightM: normWireRequired(normWireNumber), mortarType: normWireRequired(normWireLiteral("GeneralPurpose", "ThinLayer", "Lightweight")), mortarClass: normWireRequired(normWireLiteral("M1", "M2_5", "M5", "M10", "M15", "M20")), mortarStrengthPa: normWireRequired(normWireNumber), bedJointThicknessM: normWireRequired(normWireNumber), reinforced: normWireRequired(normWireBoolean), asVerticalM2: normWireRequired(normWireNumber), asHorizontalM2: normWireRequired(normWireNumber), fYdPa: normWireRequired(normWireNumber), fireReiMin: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), exposure: normWireRequired(normWireLiteral("Mx1", "Mx2", "Mx3", "Mx4", "Mx5")), mu: normWireRequired(normWireNumber), densityKgM3: normWireRequired(normWireNumber), phiInfinity: normWireRequired(normWireNumber), isBasement: normWireRequired(normWireBoolean), loadCases: normWireRequired(normWireArray(normWireObject<{ id: string; designSituation: string; imposedCategory: string; gKSlabN: number; qKImposedPa: number; tributaryAreaM2: number; slabSpanM: number; qKSnowPa: number; qPWindPa: number; cPe: number; hKEarthN: number; concentrated: { id: string; forceN: number; bearingAreaM2: number; bearingLengthM: number; }[]; }>({ id: normWireRequired(normWireString), designSituation: normWireRequired(normWireString), imposedCategory: normWireRequired(normWireString), gKSlabN: normWireRequired(normWireNumber), qKImposedPa: normWireRequired(normWireNumber), tributaryAreaM2: normWireRequired(normWireNumber), slabSpanM: normWireRequired(normWireNumber), qKSnowPa: normWireRequired(normWireNumber), qPWindPa: normWireRequired(normWireNumber), cPe: normWireRequired(normWireNumber), hKEarthN: normWireRequired(normWireNumber), concentrated: normWireRequired(normWireArray(normWireObject<{ id: string; forceN: number; bearingAreaM2: number; bearingLengthM: number; }>({ id: normWireRequired(normWireString), forceN: normWireRequired(normWireNumber), bearingAreaM2: normWireRequired(normWireNumber), bearingLengthM: normWireRequired(normWireNumber) }))) }))) }))) });
export const parseConcentratedLoad: NormWireReader<ConcentratedLoad> = normWireObject<ConcentratedLoad>({ id: normWireRequired(normWireString), forceN: normWireRequired(normWireNumber), bearingAreaM2: normWireRequired(normWireNumber), bearingLengthM: normWireRequired(normWireNumber) });
export const parseWallLoadCase: NormWireReader<WallLoadCase> = normWireObject<WallLoadCase>({ id: normWireRequired(normWireString), designSituation: normWireRequired(normWireString), imposedCategory: normWireRequired(normWireString), gKSlabN: normWireRequired(normWireNumber), qKImposedPa: normWireRequired(normWireNumber), tributaryAreaM2: normWireRequired(normWireNumber), slabSpanM: normWireRequired(normWireNumber), qKSnowPa: normWireRequired(normWireNumber), qPWindPa: normWireRequired(normWireNumber), cPe: normWireRequired(normWireNumber), hKEarthN: normWireRequired(normWireNumber), concentrated: normWireRequired(normWireArray(normWireRef(() => parseConcentratedLoad))) });
export const parseWallOpening: NormWireReader<WallOpening> = normWireObject<WallOpening>({ id: normWireRequired(normWireString), widthM: normWireRequired(normWireNumber), heightM: normWireRequired(normWireNumber), sillHeightM: normWireRequired(normWireNumber) });
export const parseMasonryWall: NormWireReader<MasonryWall> = normWireObject<MasonryWall>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), wallType: normWireRequired(normWireRef(() => parseEn1996WallType)), thicknessM: normWireRequired(normWireNumber), heightM: normWireRequired(normWireNumber), lengthM: normWireRequired(normWireNumber), supportSides: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), openings: normWireRequired(normWireArray(normWireRef(() => parseWallOpening))), slabBearingDepthM: normWireRequired(normWireNumber), eccentricityTopM: normWireRequired(normWireNumber), eccentricityBottomM: normWireRequired(normWireNumber), unitGroup: normWireRequired(normWireRef(() => parseEn1996UnitGroup)), unitMaterial: normWireRequired(normWireRef(() => parseEn1996UnitMaterial)), fBPa: normWireRequired(normWireNumber), unitLengthM: normWireRequired(normWireNumber), unitWidthM: normWireRequired(normWireNumber), unitHeightM: normWireRequired(normWireNumber), mortarType: normWireRequired(normWireRef(() => parseEn1996MortarType)), mortarClass: normWireRequired(normWireRef(() => parseEn1996MortarClass)), mortarStrengthPa: normWireRequired(normWireNumber), bedJointThicknessM: normWireRequired(normWireNumber), reinforced: normWireRequired(normWireBoolean), asVerticalM2: normWireRequired(normWireNumber), asHorizontalM2: normWireRequired(normWireNumber), fYdPa: normWireRequired(normWireNumber), fireReiMin: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), exposure: normWireRequired(normWireRef(() => parseEn1996ExposureClass)), mu: normWireRequired(normWireNumber), densityKgM3: normWireRequired(normWireNumber), phiInfinity: normWireRequired(normWireNumber), isBasement: normWireRequired(normWireBoolean), loadCases: normWireRequired(normWireArray(normWireRef(() => parseWallLoadCase))) });
export const parseEn1996AnnexChoice: NormWireReader<En1996AnnexChoice> = normWireLiteral("En", "De");
export const parseEn1996DesignSituation: NormWireReader<En1996DesignSituation> = normWireLiteral("Persistent", "Transient", "Accidental", "Seismic");
export const parseEn1996MasonryClass: NormWireReader<En1996MasonryClass> = normWireLiteral("Class1", "Class2", "Class3", "Class4", "Class5");
export const parseEn1996ExposureClass: NormWireReader<En1996ExposureClass> = normWireLiteral("Mx1", "Mx2", "Mx3", "Mx4", "Mx5");
export const parseEn1996MortarClass: NormWireReader<En1996MortarClass> = normWireLiteral("M1", "M2_5", "M5", "M10", "M15", "M20");
export const parseEn1996MortarType: NormWireReader<En1996MortarType> = normWireLiteral("GeneralPurpose", "ThinLayer", "Lightweight");
export const parseEn1996UnitGroup: NormWireReader<En1996UnitGroup> = normWireLiteral("Group1", "Group2", "Group3", "Group4");
export const parseEn1996UnitMaterial: NormWireReader<En1996UnitMaterial> = normWireLiteral("Clay", "CalciumSilicate", "Aerated", "Concrete");
export const parseEn1996WallType: NormWireReader<En1996WallType> = normWireLiteral("LoadBearing", "Shear", "NonLoadBearing");
