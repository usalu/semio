/** 📸️ `En1996Snapshot` wire twin: the persisted snapshot and every record it holds, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface En1996Snapshot {
  /** @state artifact */
  annex: "En" | "De";
  /** @state artifact */
  masonryClass: "Class1" | "Class2" | "Class3" | "Class4" | "Class5";
  /** @state artifact */
  designSituation: "Persistent" | "Transient" | "Accidental" | "Seismic";
  /** @state artifact */
  storeys: number;
  /** @state artifact */
  walls: MasonryWall[];
  /** @state artifact */
  placements: Placement[];
  /** @state artifact */
  cases: CaseRow[];
  /** @state artifact */
  chases: WallChase[];
  /** @state artifact */
  movementJoints: MovementJoint[];
  /** @state artifact */
  reliabilityClass: "RC1" | "RC2" | "RC3";
}

export interface ConcentratedLoad {
  id: string;
  forceN: number;
  bearingAreaM2: number;
  bearingLengthM: number;
  distanceA1M: number;
  positionM: number;
}

export type En1996LoadCaseSituation = "persistent" | "transient" | "accidental" | "seismic";

export type En1996ImposedCategory = "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h";

export interface WallLoadCase {
  id: string;
  designSituation: En1996LoadCaseSituation;
  imposedCategory: En1996ImposedCategory;
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
  positionM: number;
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
  piers: WallPier[];
  lintels: WallLintel[];
  ties: WallTie[];
  prestress: WallPrestress[];
  confined: ConfinedElement[];
  flanges: ShearFlange[];
  bars: WallBar[];
  frostClass: En1996FrostClass;
  coverM: number;
  deflectionLimit: number;
  soilHeightM: number;
  soilDensityKgM3: number;
  isothermDepthM: number;
  planXM: number;
  planYM: number;
  planAngleDeg: number;
  inPlaneShearN: number;
  inPlaneMomentNm: number;
  bracingSystem: En1996BracingSystem;
  diaphragmStiffnessNPerM: number;
  webSpacingM: number;
}

export interface Placement {
  id: string;
  x: number;
  y: number;
  z: number;
  yaw: number;
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

export type En1996TieKind = "WallTie" | "Anchor" | "RingTie";

export type En1996BarKind = "Anchorage" | "Shear" | "DeepBeam";

export type En1996FrostClass = "Exempt" | "Moderate" | "Severe";

export type En1996ChaseKind = "Vertical" | "Horizontal" | "Recess";

export type En1996BracingSystem = "Unreinforced" | "Confined" | "Reinforced";

export interface WallPier {
  id: string;
  positionM: number;
  widthM: number;
}

export interface WallLintel {
  id: string;
  positionM: number;
  spanM: number;
  depthM: number;
  bearingLengthM: number;
}

export interface WallTie {
  id: string;
  kind: En1996TieKind;
  positionM: number;
  levelM: number;
  capacityN: number;
}

export interface WallPrestress {
  id: string;
  forceN: number;
  areaM2: number;
  fPkPa: number;
  positionM: number;
  eccentricityM: number;
  lossRatio: number;
}

export interface ConfinedElement {
  id: string;
  positionM: number;
  widthM: number;
  depthM: number;
  asM2: number;
}

export interface ShearFlange {
  id: string;
  lengthM: number;
  thicknessM: number;
  positionM: number;
}

export interface WallBar {
  id: string;
  kind: En1996BarKind;
  diameterM: number;
  lengthM: number;
  spacingM: number;
}

export interface CaseNumber {
  id: string;
  value: number;
}

export interface CaseText {
  id: string;
  value: string;
}

export interface CaseRow {
  id: string;
  labelEn: string;
  labelDe: string;
  entryId: string;
  numbers: CaseNumber[];
  texts: CaseText[];
}

export interface WallChase {
  id: string;
  wallId: string;
  kind: En1996ChaseKind;
  xM: number;
  yM: number;
  widthM: number;
  lengthM: number;
  depthM: number;
}

export interface MovementJoint {
  id: string;
  wallId: string;
  positionM: number;
}

export const parseEn1996Snapshot: NormWireReader<En1996Snapshot> = normWireObject<En1996Snapshot>({ annex: normWireRequired(normWireLiteral("En", "De")), masonryClass: normWireRequired(normWireLiteral("Class1", "Class2", "Class3", "Class4", "Class5")), designSituation: normWireRequired(normWireLiteral("Persistent", "Transient", "Accidental", "Seismic")), storeys: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), walls: normWireRequired(normWireArray(normWireRef(() => parseMasonryWall))), placements: normWireRequired(normWireArray(normWireRef(() => parsePlacement))), cases: normWireRequired(normWireArray(normWireRef(() => parseCaseRow))), chases: normWireRequired(normWireArray(normWireRef(() => parseWallChase))), movementJoints: normWireRequired(normWireArray(normWireRef(() => parseMovementJoint))), reliabilityClass: normWireRequired(normWireLiteral("RC1", "RC2", "RC3")) });
export const parseConcentratedLoad: NormWireReader<ConcentratedLoad> = normWireObject<ConcentratedLoad>({ id: normWireRequired(normWireString), forceN: normWireRequired(normWireRange(normWireNumber, {"minimum":-1000000000000,"maximum":1000000000000})), bearingAreaM2: normWireRequired(normWireRange(normWireNumber, {"maximum":100000000,"exclusiveMinimum":0})), bearingLengthM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})), distanceA1M: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})), positionM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})) });
export const parseEn1996LoadCaseSituation: NormWireReader<En1996LoadCaseSituation> = normWireLiteral("persistent", "transient", "accidental", "seismic");
export const parseEn1996ImposedCategory: NormWireReader<En1996ImposedCategory> = normWireLiteral("a", "b", "c", "d", "e", "f", "g", "h");
export const parseWallLoadCase: NormWireReader<WallLoadCase> = normWireObject<WallLoadCase>({ id: normWireRequired(normWireString), designSituation: normWireRequired(normWireRef(() => parseEn1996LoadCaseSituation)), imposedCategory: normWireRequired(normWireRef(() => parseEn1996ImposedCategory)), gKSlabN: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":1000000000000})), qKImposedPa: normWireRequired(normWireRange(normWireNumber, {"minimum":-1000000000000,"maximum":1000000000000})), tributaryAreaM2: normWireRequired(normWireRange(normWireNumber, {"maximum":100000000,"exclusiveMinimum":0})), slabSpanM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})), qKSnowPa: normWireRequired(normWireRange(normWireNumber, {"minimum":-1000000000000,"maximum":1000000000000})), qPWindPa: normWireRequired(normWireRange(normWireNumber, {"minimum":-1000000000000,"maximum":1000000000000})), cPe: normWireRequired(normWireRange(normWireNumber, {"minimum":-5,"maximum":5})), hKEarthN: normWireRequired(normWireRange(normWireNumber, {"minimum":-1000000000000,"maximum":1000000000000})), concentrated: normWireRequired(normWireArray(normWireRef(() => parseConcentratedLoad))) });
export const parseWallOpening: NormWireReader<WallOpening> = normWireObject<WallOpening>({ id: normWireRequired(normWireString), widthM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})), heightM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})), sillHeightM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})), positionM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})) });
export const parseMasonryWall: NormWireReader<MasonryWall> = normWireObject<MasonryWall>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), wallType: normWireRequired(normWireRef(() => parseEn1996WallType)), thicknessM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})), heightM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})), lengthM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})), supportSides: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), openings: normWireRequired(normWireArray(normWireRef(() => parseWallOpening))), slabBearingDepthM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})), eccentricityTopM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})), eccentricityBottomM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})), unitGroup: normWireRequired(normWireRef(() => parseEn1996UnitGroup)), unitMaterial: normWireRequired(normWireRef(() => parseEn1996UnitMaterial)), fBPa: normWireRequired(normWireRange(normWireNumber, {"maximum":1000000000000,"exclusiveMinimum":0})), unitLengthM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})), unitWidthM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})), unitHeightM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})), mortarType: normWireRequired(normWireRef(() => parseEn1996MortarType)), mortarClass: normWireRequired(normWireRef(() => parseEn1996MortarClass)), mortarStrengthPa: normWireRequired(normWireRange(normWireNumber, {"maximum":1000000000000,"exclusiveMinimum":0})), bedJointThicknessM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})), reinforced: normWireRequired(normWireBoolean), asVerticalM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":100000000})), asHorizontalM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":100000000})), fYdPa: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":1000000000000})), fireReiMin: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), exposure: normWireRequired(normWireRef(() => parseEn1996ExposureClass)), mu: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":5})), densityKgM3: normWireRequired(normWireRange(normWireNumber, {"maximum":23000,"exclusiveMinimum":0})), phiInfinity: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":20})), isBasement: normWireRequired(normWireBoolean), loadCases: normWireRequired(normWireArray(normWireRef(() => parseWallLoadCase))), piers: normWireRequired(normWireArray(normWireRef(() => parseWallPier))), lintels: normWireRequired(normWireArray(normWireRef(() => parseWallLintel))), ties: normWireRequired(normWireArray(normWireRef(() => parseWallTie))), prestress: normWireRequired(normWireArray(normWireRef(() => parseWallPrestress))), confined: normWireRequired(normWireArray(normWireRef(() => parseConfinedElement))), flanges: normWireRequired(normWireArray(normWireRef(() => parseShearFlange))), bars: normWireRequired(normWireArray(normWireRef(() => parseWallBar))), frostClass: normWireRequired(normWireRef(() => parseEn1996FrostClass)), coverM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})), deflectionLimit: normWireRequired(normWireRange(normWireNumber, {"maximum":100000,"exclusiveMinimum":0})), soilHeightM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})), soilDensityKgM3: normWireRequired(normWireRange(normWireNumber, {"maximum":23000,"exclusiveMinimum":0})), isothermDepthM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})), planXM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})), planYM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})), planAngleDeg: normWireRequired(normWireRange(normWireNumber, {"minimum":-360,"maximum":360})), inPlaneShearN: normWireRequired(normWireRange(normWireNumber, {"minimum":-1000000000000,"maximum":1000000000000})), inPlaneMomentNm: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000000000000,"maximum":10000000000000})), bracingSystem: normWireRequired(normWireRef(() => parseEn1996BracingSystem)), diaphragmStiffnessNPerM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":100000000000000})), webSpacingM: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":10000})) });
export const parsePlacement: NormWireReader<Placement> = normWireObject<Placement>({ id: normWireRequired(normWireString), x: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})), y: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})), z: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})), yaw: normWireRequired(normWireRange(normWireNumber, {"minimum":-6.283185307179586,"maximum":6.283185307179586})) });
export const parseEn1996AnnexChoice: NormWireReader<En1996AnnexChoice> = normWireLiteral("En", "De");
export const parseEn1996DesignSituation: NormWireReader<En1996DesignSituation> = normWireLiteral("Persistent", "Transient", "Accidental", "Seismic");
export const parseEn1996MasonryClass: NormWireReader<En1996MasonryClass> = normWireLiteral("Class1", "Class2", "Class3", "Class4", "Class5");
export const parseEn1996ExposureClass: NormWireReader<En1996ExposureClass> = normWireLiteral("Mx1", "Mx2", "Mx3", "Mx4", "Mx5");
export const parseEn1996MortarClass: NormWireReader<En1996MortarClass> = normWireLiteral("M1", "M2_5", "M5", "M10", "M15", "M20");
export const parseEn1996MortarType: NormWireReader<En1996MortarType> = normWireLiteral("GeneralPurpose", "ThinLayer", "Lightweight");
export const parseEn1996UnitGroup: NormWireReader<En1996UnitGroup> = normWireLiteral("Group1", "Group2", "Group3", "Group4");
export const parseEn1996UnitMaterial: NormWireReader<En1996UnitMaterial> = normWireLiteral("Clay", "CalciumSilicate", "Aerated", "Concrete");
export const parseEn1996WallType: NormWireReader<En1996WallType> = normWireLiteral("LoadBearing", "Shear", "NonLoadBearing");
export const parseEn1996TieKind: NormWireReader<En1996TieKind> = normWireLiteral("WallTie", "Anchor", "RingTie");
export const parseEn1996BarKind: NormWireReader<En1996BarKind> = normWireLiteral("Anchorage", "Shear", "DeepBeam");
export const parseEn1996FrostClass: NormWireReader<En1996FrostClass> = normWireLiteral("Exempt", "Moderate", "Severe");
export const parseEn1996ChaseKind: NormWireReader<En1996ChaseKind> = normWireLiteral("Vertical", "Horizontal", "Recess");
export const parseEn1996BracingSystem: NormWireReader<En1996BracingSystem> = normWireLiteral("Unreinforced", "Confined", "Reinforced");
export const parseWallPier: NormWireReader<WallPier> = normWireObject<WallPier>({ id: normWireRequired(normWireString), positionM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})), widthM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})) });
export const parseWallLintel: NormWireReader<WallLintel> = normWireObject<WallLintel>({ id: normWireRequired(normWireString), positionM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})), spanM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})), depthM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})), bearingLengthM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})) });
export const parseWallTie: NormWireReader<WallTie> = normWireObject<WallTie>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseEn1996TieKind)), positionM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})), levelM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})), capacityN: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":1000000000000})) });
export const parseWallPrestress: NormWireReader<WallPrestress> = normWireObject<WallPrestress>({ id: normWireRequired(normWireString), forceN: normWireRequired(normWireRange(normWireNumber, {"minimum":-1000000000000,"maximum":1000000000000})), areaM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":100000000})), fPkPa: normWireRequired(normWireRange(normWireNumber, {"maximum":1000000000000,"exclusiveMinimum":0})), positionM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})), eccentricityM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})), lossRatio: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":1})) });
export const parseConfinedElement: NormWireReader<ConfinedElement> = normWireObject<ConfinedElement>({ id: normWireRequired(normWireString), positionM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})), widthM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})), depthM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})), asM2: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":100000000})) });
export const parseShearFlange: NormWireReader<ShearFlange> = normWireObject<ShearFlange>({ id: normWireRequired(normWireString), lengthM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})), thicknessM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})), positionM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})) });
export const parseWallBar: NormWireReader<WallBar> = normWireObject<WallBar>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseEn1996BarKind)), diameterM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})), lengthM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})), spacingM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})) });
export const parseCaseNumber: NormWireReader<CaseNumber> = normWireObject<CaseNumber>({ id: normWireRequired(normWireString), value: normWireRequired(normWireRange(normWireNumber, {"minimum":-1000000000000000000,"maximum":1000000000000000000})) });
export const parseCaseText: NormWireReader<CaseText> = normWireObject<CaseText>({ id: normWireRequired(normWireString), value: normWireRequired(normWireString) });
export const parseCaseRow: NormWireReader<CaseRow> = normWireObject<CaseRow>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), entryId: normWireRequired(normWireString), numbers: normWireRequired(normWireArray(normWireRef(() => parseCaseNumber))), texts: normWireRequired(normWireArray(normWireRef(() => parseCaseText))) });
export const parseWallChase: NormWireReader<WallChase> = normWireObject<WallChase>({ id: normWireRequired(normWireString), wallId: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseEn1996ChaseKind)), xM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})), yM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})), widthM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})), lengthM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})), depthM: normWireRequired(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0})) });
export const parseMovementJoint: NormWireReader<MovementJoint> = normWireObject<MovementJoint>({ id: normWireRequired(normWireString), wallId: normWireRequired(normWireString), positionM: normWireRequired(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000})) });
