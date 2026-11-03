/** 🔺️ `En1996Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, normWireOptional, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type CaseNumber, parseCaseNumber, type CaseRow, parseCaseRow, type CaseText, parseCaseText, type ConcentratedLoad, parseConcentratedLoad, type ConfinedElement, parseConfinedElement, type En1996BarKind, parseEn1996BarKind, type En1996BracingSystem, parseEn1996BracingSystem, type En1996ChaseKind, parseEn1996ChaseKind, type En1996ExposureClass, parseEn1996ExposureClass, type En1996FrostClass, parseEn1996FrostClass, type En1996ImposedCategory, parseEn1996ImposedCategory, type En1996LoadCaseSituation, parseEn1996LoadCaseSituation, type En1996MortarClass, parseEn1996MortarClass, type En1996MortarType, parseEn1996MortarType, type En1996TieKind, parseEn1996TieKind, type En1996UnitGroup, parseEn1996UnitGroup, type En1996UnitMaterial, parseEn1996UnitMaterial, type En1996WallType, parseEn1996WallType, type MasonryWall, parseMasonryWall, type MovementJoint, parseMovementJoint, type Placement, parsePlacement, type ShearFlange, parseShearFlange, type WallBar, parseWallBar, type WallChase, parseWallChase, type WallLintel, parseWallLintel, type WallLoadCase, parseWallLoadCase, type WallOpening, parseWallOpening, type WallPier, parseWallPier, type WallPrestress, parseWallPrestress, type WallTie, parseWallTie } from "../📸️snapshot/🟦️.ts";

export interface En1996Diff {
  /** @state artifact */
  annex?: ("En" | "De") | null;
  /** @state artifact */
  masonryClass?: ("Class1" | "Class2" | "Class3" | "Class4" | "Class5") | null;
  /** @state artifact */
  designSituation?: ("Persistent" | "Transient" | "Accidental" | "Seismic") | null;
  /** @state artifact */
  storeys?: number | null;
  /** @state artifact */
  walls: En1996WallDelta;
  /** @state artifact */
  placements: En1996PlacementDelta;
  /** @state artifact */
  cases: CaseRowDelta;
  /** @state artifact */
  chases: En1996ChaseDelta;
  /** @state artifact */
  movementJoints: En1996JointDelta;
  /** @state artifact */
  reliabilityClass?: ("RC1" | "RC2" | "RC3") | null;
}

export interface En1996OpeningPatch {
  widthM?: number | null;
  heightM?: number | null;
  sillHeightM?: number | null;
  positionM?: number | null;
}

export interface En1996OpeningRemoval {
  id: string;
  index: number;
}

export interface En1996OpeningInsertion {
  index: number;
  row: WallOpening;
}

export interface En1996OpeningRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1996OpeningModification {
  id: string;
  patch: En1996OpeningPatch;
}

export interface En1996OpeningDelta {
  removed: En1996OpeningRemoval[];
  inserted: En1996OpeningInsertion[];
  moved: En1996OpeningRelocation[];
  modified: En1996OpeningModification[];
}

export interface En1996ConcentratedPatch {
  forceN?: number | null;
  bearingAreaM2?: number | null;
  bearingLengthM?: number | null;
  distanceA1M?: number | null;
  positionM?: number | null;
}

export interface En1996ConcentratedRemoval {
  id: string;
  index: number;
}

export interface En1996ConcentratedInsertion {
  index: number;
  row: ConcentratedLoad;
}

export interface En1996ConcentratedRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1996ConcentratedModification {
  id: string;
  patch: En1996ConcentratedPatch;
}

export interface En1996ConcentratedDelta {
  removed: En1996ConcentratedRemoval[];
  inserted: En1996ConcentratedInsertion[];
  moved: En1996ConcentratedRelocation[];
  modified: En1996ConcentratedModification[];
}

export interface En1996LoadCasePatch {
  designSituation?: En1996LoadCaseSituation | null;
  imposedCategory?: En1996ImposedCategory | null;
  gKSlabN?: number | null;
  qKImposedPa?: number | null;
  tributaryAreaM2?: number | null;
  slabSpanM?: number | null;
  qKSnowPa?: number | null;
  qPWindPa?: number | null;
  cPe?: number | null;
  hKEarthN?: number | null;
  concentrated: En1996ConcentratedDelta;
}

export interface En1996LoadCaseRemoval {
  id: string;
  index: number;
}

export interface En1996LoadCaseInsertion {
  index: number;
  row: WallLoadCase;
}

export interface En1996LoadCaseRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1996LoadCaseModification {
  id: string;
  patch: En1996LoadCasePatch;
}

export interface En1996LoadCaseDelta {
  removed: En1996LoadCaseRemoval[];
  inserted: En1996LoadCaseInsertion[];
  moved: En1996LoadCaseRelocation[];
  modified: En1996LoadCaseModification[];
}

export interface En1996WallPatch {
  labelEn?: string | null;
  labelDe?: string | null;
  wallType?: En1996WallType | null;
  thicknessM?: number | null;
  heightM?: number | null;
  lengthM?: number | null;
  supportSides?: number | null;
  slabBearingDepthM?: number | null;
  eccentricityTopM?: number | null;
  eccentricityBottomM?: number | null;
  unitGroup?: En1996UnitGroup | null;
  unitMaterial?: En1996UnitMaterial | null;
  fBPa?: number | null;
  unitLengthM?: number | null;
  unitWidthM?: number | null;
  unitHeightM?: number | null;
  mortarType?: En1996MortarType | null;
  mortarClass?: En1996MortarClass | null;
  mortarStrengthPa?: number | null;
  bedJointThicknessM?: number | null;
  reinforced?: boolean | null;
  asVerticalM2?: number | null;
  asHorizontalM2?: number | null;
  fYdPa?: number | null;
  fireReiMin?: number | null;
  exposure?: En1996ExposureClass | null;
  mu?: number | null;
  densityKgM3?: number | null;
  phiInfinity?: number | null;
  isBasement?: boolean | null;
  openings: En1996OpeningDelta;
  loadCases: En1996LoadCaseDelta;
  piers: En1996PierDelta;
  lintels: En1996LintelDelta;
  ties: En1996TieDelta;
  prestress: En1996PrestressDelta;
  confined: En1996ConfinedDelta;
  flanges: En1996FlangeDelta;
  bars: En1996BarDelta;
  frostClass?: En1996FrostClass | null;
  coverM?: number | null;
  deflectionLimit?: number | null;
  soilHeightM?: number | null;
  soilDensityKgM3?: number | null;
  isothermDepthM?: number | null;
  planXM?: number | null;
  planYM?: number | null;
  planAngleDeg?: number | null;
  inPlaneShearN?: number | null;
  inPlaneMomentNm?: number | null;
  bracingSystem?: En1996BracingSystem | null;
  diaphragmStiffnessNPerM?: number | null;
}

export interface En1996WallRemoval {
  id: string;
  index: number;
}

export interface En1996WallInsertion {
  index: number;
  row: MasonryWall;
}

export interface En1996WallRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1996WallModification {
  id: string;
  patch: En1996WallPatch;
}

export interface En1996WallDelta {
  removed: En1996WallRemoval[];
  inserted: En1996WallInsertion[];
  moved: En1996WallRelocation[];
  modified: En1996WallModification[];
}

export interface En1996PlacementPatch {
  x?: number | null;
  y?: number | null;
  z?: number | null;
  yaw?: number | null;
}

export interface En1996PlacementRemoval {
  id: string;
  index: number;
}

export interface En1996PlacementInsertion {
  index: number;
  row: Placement;
}

export interface En1996PlacementRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1996PlacementModification {
  id: string;
  patch: En1996PlacementPatch;
}

export interface En1996PlacementDelta {
  removed: En1996PlacementRemoval[];
  inserted: En1996PlacementInsertion[];
  moved: En1996PlacementRelocation[];
  modified: En1996PlacementModification[];
}

export interface En1996ChasePatch {
  wallId?: string | null;
  kind?: En1996ChaseKind | null;
  xM?: number | null;
  yM?: number | null;
  widthM?: number | null;
  lengthM?: number | null;
  depthM?: number | null;
}

export interface En1996ChaseRemoval {
  id: string;
  index: number;
}

export interface En1996ChaseInsertion {
  index: number;
  row: WallChase;
}

export interface En1996ChaseRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1996ChaseModification {
  id: string;
  patch: En1996ChasePatch;
}

export interface En1996ChaseDelta {
  removed: En1996ChaseRemoval[];
  inserted: En1996ChaseInsertion[];
  moved: En1996ChaseRelocation[];
  modified: En1996ChaseModification[];
}

export interface En1996JointPatch {
  wallId?: string | null;
  positionM?: number | null;
}

export interface En1996JointRemoval {
  id: string;
  index: number;
}

export interface En1996JointInsertion {
  index: number;
  row: MovementJoint;
}

export interface En1996JointRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1996JointModification {
  id: string;
  patch: En1996JointPatch;
}

export interface En1996JointDelta {
  removed: En1996JointRemoval[];
  inserted: En1996JointInsertion[];
  moved: En1996JointRelocation[];
  modified: En1996JointModification[];
}

export interface En1996PierPatch {
  positionM?: number | null;
  widthM?: number | null;
}

export interface En1996PierRemoval {
  id: string;
  index: number;
}

export interface En1996PierInsertion {
  index: number;
  row: WallPier;
}

export interface En1996PierRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1996PierModification {
  id: string;
  patch: En1996PierPatch;
}

export interface En1996PierDelta {
  removed: En1996PierRemoval[];
  inserted: En1996PierInsertion[];
  moved: En1996PierRelocation[];
  modified: En1996PierModification[];
}

export interface En1996LintelPatch {
  positionM?: number | null;
  spanM?: number | null;
  depthM?: number | null;
  bearingLengthM?: number | null;
}

export interface En1996LintelRemoval {
  id: string;
  index: number;
}

export interface En1996LintelInsertion {
  index: number;
  row: WallLintel;
}

export interface En1996LintelRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1996LintelModification {
  id: string;
  patch: En1996LintelPatch;
}

export interface En1996LintelDelta {
  removed: En1996LintelRemoval[];
  inserted: En1996LintelInsertion[];
  moved: En1996LintelRelocation[];
  modified: En1996LintelModification[];
}

export interface En1996TiePatch {
  kind?: En1996TieKind | null;
  positionM?: number | null;
  levelM?: number | null;
  capacityN?: number | null;
}

export interface En1996TieRemoval {
  id: string;
  index: number;
}

export interface En1996TieInsertion {
  index: number;
  row: WallTie;
}

export interface En1996TieRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1996TieModification {
  id: string;
  patch: En1996TiePatch;
}

export interface En1996TieDelta {
  removed: En1996TieRemoval[];
  inserted: En1996TieInsertion[];
  moved: En1996TieRelocation[];
  modified: En1996TieModification[];
}

export interface En1996PrestressPatch {
  forceN?: number | null;
  areaM2?: number | null;
  fPkPa?: number | null;
  positionM?: number | null;
  eccentricityM?: number | null;
  lossRatio?: number | null;
}

export interface En1996PrestressRemoval {
  id: string;
  index: number;
}

export interface En1996PrestressInsertion {
  index: number;
  row: WallPrestress;
}

export interface En1996PrestressRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1996PrestressModification {
  id: string;
  patch: En1996PrestressPatch;
}

export interface En1996PrestressDelta {
  removed: En1996PrestressRemoval[];
  inserted: En1996PrestressInsertion[];
  moved: En1996PrestressRelocation[];
  modified: En1996PrestressModification[];
}

export interface En1996ConfinedPatch {
  positionM?: number | null;
  widthM?: number | null;
  depthM?: number | null;
  asM2?: number | null;
}

export interface En1996ConfinedRemoval {
  id: string;
  index: number;
}

export interface En1996ConfinedInsertion {
  index: number;
  row: ConfinedElement;
}

export interface En1996ConfinedRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1996ConfinedModification {
  id: string;
  patch: En1996ConfinedPatch;
}

export interface En1996ConfinedDelta {
  removed: En1996ConfinedRemoval[];
  inserted: En1996ConfinedInsertion[];
  moved: En1996ConfinedRelocation[];
  modified: En1996ConfinedModification[];
}

export interface En1996FlangePatch {
  lengthM?: number | null;
  thicknessM?: number | null;
  positionM?: number | null;
}

export interface En1996FlangeRemoval {
  id: string;
  index: number;
}

export interface En1996FlangeInsertion {
  index: number;
  row: ShearFlange;
}

export interface En1996FlangeRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1996FlangeModification {
  id: string;
  patch: En1996FlangePatch;
}

export interface En1996FlangeDelta {
  removed: En1996FlangeRemoval[];
  inserted: En1996FlangeInsertion[];
  moved: En1996FlangeRelocation[];
  modified: En1996FlangeModification[];
}

export interface En1996BarPatch {
  kind?: En1996BarKind | null;
  diameterM?: number | null;
  lengthM?: number | null;
  spacingM?: number | null;
}

export interface En1996BarRemoval {
  id: string;
  index: number;
}

export interface En1996BarInsertion {
  index: number;
  row: WallBar;
}

export interface En1996BarRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1996BarModification {
  id: string;
  patch: En1996BarPatch;
}

export interface En1996BarDelta {
  removed: En1996BarRemoval[];
  inserted: En1996BarInsertion[];
  moved: En1996BarRelocation[];
  modified: En1996BarModification[];
}

export interface CaseNumberPatch {
  value?: number | null;
}

export interface CaseNumberRemoval {
  id: string;
  index: number;
}

export interface CaseNumberInsertion {
  index: number;
  row: CaseNumber;
}

export interface CaseNumberRelocation {
  id: string;
  from: number;
  to: number;
}

export interface CaseNumberModification {
  id: string;
  patch: CaseNumberPatch;
}

export interface CaseNumberDelta {
  removed: CaseNumberRemoval[];
  inserted: CaseNumberInsertion[];
  moved: CaseNumberRelocation[];
  modified: CaseNumberModification[];
}

export interface CaseTextPatch {
  value?: string | null;
}

export interface CaseTextRemoval {
  id: string;
  index: number;
}

export interface CaseTextInsertion {
  index: number;
  row: CaseText;
}

export interface CaseTextRelocation {
  id: string;
  from: number;
  to: number;
}

export interface CaseTextModification {
  id: string;
  patch: CaseTextPatch;
}

export interface CaseTextDelta {
  removed: CaseTextRemoval[];
  inserted: CaseTextInsertion[];
  moved: CaseTextRelocation[];
  modified: CaseTextModification[];
}

export interface CaseRowPatch {
  labelEn?: string | null;
  labelDe?: string | null;
  entryId?: string | null;
  numbers: CaseNumberDelta;
  texts: CaseTextDelta;
}

export interface CaseRowRemoval {
  id: string;
  index: number;
}

export interface CaseRowInsertion {
  index: number;
  row: CaseRow;
}

export interface CaseRowRelocation {
  id: string;
  from: number;
  to: number;
}

export interface CaseRowModification {
  id: string;
  patch: CaseRowPatch;
}

export interface CaseRowDelta {
  removed: CaseRowRemoval[];
  inserted: CaseRowInsertion[];
  moved: CaseRowRelocation[];
  modified: CaseRowModification[];
}

export const parseEn1996Diff: NormWireReader<En1996Diff> = normWireObject<En1996Diff>({ annex: normWireOptional(normWireNullable(normWireLiteral("En", "De"))), masonryClass: normWireOptional(normWireNullable(normWireLiteral("Class1", "Class2", "Class3", "Class4", "Class5"))), designSituation: normWireOptional(normWireNullable(normWireLiteral("Persistent", "Transient", "Accidental", "Seismic"))), storeys: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), walls: normWireRequired(normWireRef(() => parseEn1996WallDelta)), placements: normWireRequired(normWireRef(() => parseEn1996PlacementDelta)), cases: normWireRequired(normWireRef(() => parseCaseRowDelta)), chases: normWireRequired(normWireRef(() => parseEn1996ChaseDelta)), movementJoints: normWireRequired(normWireRef(() => parseEn1996JointDelta)), reliabilityClass: normWireOptional(normWireNullable(normWireLiteral("RC1", "RC2", "RC3"))) });
export const parseEn1996OpeningPatch: NormWireReader<En1996OpeningPatch> = normWireObject<En1996OpeningPatch>({ widthM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0}))), heightM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0}))), sillHeightM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":0,"maximum":10000}))), positionM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000}))) });
export const parseEn1996OpeningRemoval: NormWireReader<En1996OpeningRemoval> = normWireObject<En1996OpeningRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996OpeningInsertion: NormWireReader<En1996OpeningInsertion> = normWireObject<En1996OpeningInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseWallOpening) });
export const parseEn1996OpeningRelocation: NormWireReader<En1996OpeningRelocation> = normWireObject<En1996OpeningRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996OpeningModification: NormWireReader<En1996OpeningModification> = normWireObject<En1996OpeningModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1996OpeningPatch)) });
export const parseEn1996OpeningDelta: NormWireReader<En1996OpeningDelta> = normWireObject<En1996OpeningDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1996OpeningRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1996OpeningInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1996OpeningRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1996OpeningModification))) });
export const parseEn1996ConcentratedPatch: NormWireReader<En1996ConcentratedPatch> = normWireObject<En1996ConcentratedPatch>({ forceN: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-1000000000000,"maximum":1000000000000}))), bearingAreaM2: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":100000000,"exclusiveMinimum":0}))), bearingLengthM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0}))), distanceA1M: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":0,"maximum":10000}))), positionM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000}))) });
export const parseEn1996ConcentratedRemoval: NormWireReader<En1996ConcentratedRemoval> = normWireObject<En1996ConcentratedRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996ConcentratedInsertion: NormWireReader<En1996ConcentratedInsertion> = normWireObject<En1996ConcentratedInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseConcentratedLoad) });
export const parseEn1996ConcentratedRelocation: NormWireReader<En1996ConcentratedRelocation> = normWireObject<En1996ConcentratedRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996ConcentratedModification: NormWireReader<En1996ConcentratedModification> = normWireObject<En1996ConcentratedModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1996ConcentratedPatch)) });
export const parseEn1996ConcentratedDelta: NormWireReader<En1996ConcentratedDelta> = normWireObject<En1996ConcentratedDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1996ConcentratedRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1996ConcentratedInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1996ConcentratedRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1996ConcentratedModification))) });
export const parseEn1996LoadCasePatch: NormWireReader<En1996LoadCasePatch> = normWireObject<En1996LoadCasePatch>({ designSituation: normWireOptional(normWireNullable(parseEn1996LoadCaseSituation)), imposedCategory: normWireOptional(normWireNullable(parseEn1996ImposedCategory)), gKSlabN: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":0,"maximum":1000000000000}))), qKImposedPa: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-1000000000000,"maximum":1000000000000}))), tributaryAreaM2: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":100000000,"exclusiveMinimum":0}))), slabSpanM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0}))), qKSnowPa: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-1000000000000,"maximum":1000000000000}))), qPWindPa: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-1000000000000,"maximum":1000000000000}))), cPe: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-5,"maximum":5}))), hKEarthN: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-1000000000000,"maximum":1000000000000}))), concentrated: normWireRequired(normWireRef(() => parseEn1996ConcentratedDelta)) });
export const parseEn1996LoadCaseRemoval: NormWireReader<En1996LoadCaseRemoval> = normWireObject<En1996LoadCaseRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996LoadCaseInsertion: NormWireReader<En1996LoadCaseInsertion> = normWireObject<En1996LoadCaseInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseWallLoadCase) });
export const parseEn1996LoadCaseRelocation: NormWireReader<En1996LoadCaseRelocation> = normWireObject<En1996LoadCaseRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996LoadCaseModification: NormWireReader<En1996LoadCaseModification> = normWireObject<En1996LoadCaseModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1996LoadCasePatch)) });
export const parseEn1996LoadCaseDelta: NormWireReader<En1996LoadCaseDelta> = normWireObject<En1996LoadCaseDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1996LoadCaseRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1996LoadCaseInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1996LoadCaseRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1996LoadCaseModification))) });
export const parseEn1996WallPatch: NormWireReader<En1996WallPatch> = normWireObject<En1996WallPatch>({ labelEn: normWireOptional(normWireNullable(normWireString)), labelDe: normWireOptional(normWireNullable(normWireString)), wallType: normWireOptional(normWireNullable(parseEn1996WallType)), thicknessM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0}))), heightM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0}))), lengthM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0}))), supportSides: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))), slabBearingDepthM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0}))), eccentricityTopM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000}))), eccentricityBottomM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000}))), unitGroup: normWireOptional(normWireNullable(parseEn1996UnitGroup)), unitMaterial: normWireOptional(normWireNullable(parseEn1996UnitMaterial)), fBPa: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":1000000000000,"exclusiveMinimum":0}))), unitLengthM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0}))), unitWidthM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0}))), unitHeightM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0}))), mortarType: normWireOptional(normWireNullable(parseEn1996MortarType)), mortarClass: normWireOptional(normWireNullable(parseEn1996MortarClass)), mortarStrengthPa: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":1000000000000,"exclusiveMinimum":0}))), bedJointThicknessM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0}))), reinforced: normWireOptional(normWireNullable(normWireBoolean)), asVerticalM2: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":0,"maximum":100000000}))), asHorizontalM2: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":0,"maximum":100000000}))), fYdPa: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":0,"maximum":1000000000000}))), fireReiMin: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), exposure: normWireOptional(normWireNullable(parseEn1996ExposureClass)), mu: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":0,"maximum":5}))), densityKgM3: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":23000,"exclusiveMinimum":0}))), phiInfinity: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":0,"maximum":20}))), isBasement: normWireOptional(normWireNullable(normWireBoolean)), openings: normWireRequired(normWireRef(() => parseEn1996OpeningDelta)), loadCases: normWireRequired(normWireRef(() => parseEn1996LoadCaseDelta)), piers: normWireRequired(normWireRef(() => parseEn1996PierDelta)), lintels: normWireRequired(normWireRef(() => parseEn1996LintelDelta)), ties: normWireRequired(normWireRef(() => parseEn1996TieDelta)), prestress: normWireRequired(normWireRef(() => parseEn1996PrestressDelta)), confined: normWireRequired(normWireRef(() => parseEn1996ConfinedDelta)), flanges: normWireRequired(normWireRef(() => parseEn1996FlangeDelta)), bars: normWireRequired(normWireRef(() => parseEn1996BarDelta)), frostClass: normWireOptional(normWireNullable(parseEn1996FrostClass)), coverM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":0,"maximum":10000}))), deflectionLimit: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":100000,"exclusiveMinimum":0}))), soilHeightM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":0,"maximum":10000}))), soilDensityKgM3: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":23000,"exclusiveMinimum":0}))), isothermDepthM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":0,"maximum":10000}))), planXM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000}))), planYM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000}))), planAngleDeg: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-360,"maximum":360}))), inPlaneShearN: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-1000000000000,"maximum":1000000000000}))), inPlaneMomentNm: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-10000000000000,"maximum":10000000000000}))), bracingSystem: normWireOptional(normWireNullable(parseEn1996BracingSystem)), diaphragmStiffnessNPerM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":0,"maximum":100000000000000}))) });
export const parseEn1996WallRemoval: NormWireReader<En1996WallRemoval> = normWireObject<En1996WallRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996WallInsertion: NormWireReader<En1996WallInsertion> = normWireObject<En1996WallInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseMasonryWall) });
export const parseEn1996WallRelocation: NormWireReader<En1996WallRelocation> = normWireObject<En1996WallRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996WallModification: NormWireReader<En1996WallModification> = normWireObject<En1996WallModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1996WallPatch)) });
export const parseEn1996WallDelta: NormWireReader<En1996WallDelta> = normWireObject<En1996WallDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1996WallRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1996WallInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1996WallRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1996WallModification))) });
export const parseEn1996PlacementPatch: NormWireReader<En1996PlacementPatch> = normWireObject<En1996PlacementPatch>({ x: normWireOptional(normWireNullable(normWireNumber)), y: normWireOptional(normWireNullable(normWireNumber)), z: normWireOptional(normWireNullable(normWireNumber)), yaw: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1996PlacementRemoval: NormWireReader<En1996PlacementRemoval> = normWireObject<En1996PlacementRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996PlacementInsertion: NormWireReader<En1996PlacementInsertion> = normWireObject<En1996PlacementInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parsePlacement) });
export const parseEn1996PlacementRelocation: NormWireReader<En1996PlacementRelocation> = normWireObject<En1996PlacementRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996PlacementModification: NormWireReader<En1996PlacementModification> = normWireObject<En1996PlacementModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1996PlacementPatch)) });
export const parseEn1996PlacementDelta: NormWireReader<En1996PlacementDelta> = normWireObject<En1996PlacementDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1996PlacementRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1996PlacementInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1996PlacementRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1996PlacementModification))) });
export const parseEn1996ChasePatch: NormWireReader<En1996ChasePatch> = normWireObject<En1996ChasePatch>({ wallId: normWireOptional(normWireNullable(normWireString)), kind: normWireOptional(normWireNullable(parseEn1996ChaseKind)), xM: normWireOptional(normWireNullable(normWireNumber)), yM: normWireOptional(normWireNullable(normWireNumber)), widthM: normWireOptional(normWireNullable(normWireNumber)), lengthM: normWireOptional(normWireNullable(normWireNumber)), depthM: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1996ChaseRemoval: NormWireReader<En1996ChaseRemoval> = normWireObject<En1996ChaseRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996ChaseInsertion: NormWireReader<En1996ChaseInsertion> = normWireObject<En1996ChaseInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseWallChase) });
export const parseEn1996ChaseRelocation: NormWireReader<En1996ChaseRelocation> = normWireObject<En1996ChaseRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996ChaseModification: NormWireReader<En1996ChaseModification> = normWireObject<En1996ChaseModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1996ChasePatch)) });
export const parseEn1996ChaseDelta: NormWireReader<En1996ChaseDelta> = normWireObject<En1996ChaseDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1996ChaseRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1996ChaseInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1996ChaseRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1996ChaseModification))) });
export const parseEn1996JointPatch: NormWireReader<En1996JointPatch> = normWireObject<En1996JointPatch>({ wallId: normWireOptional(normWireNullable(normWireString)), positionM: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1996JointRemoval: NormWireReader<En1996JointRemoval> = normWireObject<En1996JointRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996JointInsertion: NormWireReader<En1996JointInsertion> = normWireObject<En1996JointInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseMovementJoint) });
export const parseEn1996JointRelocation: NormWireReader<En1996JointRelocation> = normWireObject<En1996JointRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996JointModification: NormWireReader<En1996JointModification> = normWireObject<En1996JointModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1996JointPatch)) });
export const parseEn1996JointDelta: NormWireReader<En1996JointDelta> = normWireObject<En1996JointDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1996JointRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1996JointInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1996JointRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1996JointModification))) });
export const parseEn1996PierPatch: NormWireReader<En1996PierPatch> = normWireObject<En1996PierPatch>({ positionM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000}))), widthM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0}))) });
export const parseEn1996PierRemoval: NormWireReader<En1996PierRemoval> = normWireObject<En1996PierRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996PierInsertion: NormWireReader<En1996PierInsertion> = normWireObject<En1996PierInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseWallPier) });
export const parseEn1996PierRelocation: NormWireReader<En1996PierRelocation> = normWireObject<En1996PierRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996PierModification: NormWireReader<En1996PierModification> = normWireObject<En1996PierModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1996PierPatch)) });
export const parseEn1996PierDelta: NormWireReader<En1996PierDelta> = normWireObject<En1996PierDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1996PierRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1996PierInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1996PierRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1996PierModification))) });
export const parseEn1996LintelPatch: NormWireReader<En1996LintelPatch> = normWireObject<En1996LintelPatch>({ positionM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000}))), spanM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0}))), depthM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0}))), bearingLengthM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0}))) });
export const parseEn1996LintelRemoval: NormWireReader<En1996LintelRemoval> = normWireObject<En1996LintelRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996LintelInsertion: NormWireReader<En1996LintelInsertion> = normWireObject<En1996LintelInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseWallLintel) });
export const parseEn1996LintelRelocation: NormWireReader<En1996LintelRelocation> = normWireObject<En1996LintelRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996LintelModification: NormWireReader<En1996LintelModification> = normWireObject<En1996LintelModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1996LintelPatch)) });
export const parseEn1996LintelDelta: NormWireReader<En1996LintelDelta> = normWireObject<En1996LintelDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1996LintelRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1996LintelInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1996LintelRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1996LintelModification))) });
export const parseEn1996TiePatch: NormWireReader<En1996TiePatch> = normWireObject<En1996TiePatch>({ kind: normWireOptional(normWireNullable(parseEn1996TieKind)), positionM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000}))), levelM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000}))), capacityN: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":0,"maximum":1000000000000}))) });
export const parseEn1996TieRemoval: NormWireReader<En1996TieRemoval> = normWireObject<En1996TieRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996TieInsertion: NormWireReader<En1996TieInsertion> = normWireObject<En1996TieInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseWallTie) });
export const parseEn1996TieRelocation: NormWireReader<En1996TieRelocation> = normWireObject<En1996TieRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996TieModification: NormWireReader<En1996TieModification> = normWireObject<En1996TieModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1996TiePatch)) });
export const parseEn1996TieDelta: NormWireReader<En1996TieDelta> = normWireObject<En1996TieDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1996TieRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1996TieInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1996TieRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1996TieModification))) });
export const parseEn1996PrestressPatch: NormWireReader<En1996PrestressPatch> = normWireObject<En1996PrestressPatch>({ forceN: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-1000000000000,"maximum":1000000000000}))), areaM2: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":0,"maximum":100000000}))), fPkPa: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":1000000000000,"exclusiveMinimum":0}))), positionM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000}))), eccentricityM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000}))), lossRatio: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":0,"maximum":1}))) });
export const parseEn1996PrestressRemoval: NormWireReader<En1996PrestressRemoval> = normWireObject<En1996PrestressRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996PrestressInsertion: NormWireReader<En1996PrestressInsertion> = normWireObject<En1996PrestressInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseWallPrestress) });
export const parseEn1996PrestressRelocation: NormWireReader<En1996PrestressRelocation> = normWireObject<En1996PrestressRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996PrestressModification: NormWireReader<En1996PrestressModification> = normWireObject<En1996PrestressModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1996PrestressPatch)) });
export const parseEn1996PrestressDelta: NormWireReader<En1996PrestressDelta> = normWireObject<En1996PrestressDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1996PrestressRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1996PrestressInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1996PrestressRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1996PrestressModification))) });
export const parseEn1996ConfinedPatch: NormWireReader<En1996ConfinedPatch> = normWireObject<En1996ConfinedPatch>({ positionM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000}))), widthM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0}))), depthM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0}))), asM2: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":0,"maximum":100000000}))) });
export const parseEn1996ConfinedRemoval: NormWireReader<En1996ConfinedRemoval> = normWireObject<En1996ConfinedRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996ConfinedInsertion: NormWireReader<En1996ConfinedInsertion> = normWireObject<En1996ConfinedInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseConfinedElement) });
export const parseEn1996ConfinedRelocation: NormWireReader<En1996ConfinedRelocation> = normWireObject<En1996ConfinedRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996ConfinedModification: NormWireReader<En1996ConfinedModification> = normWireObject<En1996ConfinedModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1996ConfinedPatch)) });
export const parseEn1996ConfinedDelta: NormWireReader<En1996ConfinedDelta> = normWireObject<En1996ConfinedDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1996ConfinedRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1996ConfinedInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1996ConfinedRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1996ConfinedModification))) });
export const parseEn1996FlangePatch: NormWireReader<En1996FlangePatch> = normWireObject<En1996FlangePatch>({ lengthM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0}))), thicknessM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0}))), positionM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-10000,"maximum":10000}))) });
export const parseEn1996FlangeRemoval: NormWireReader<En1996FlangeRemoval> = normWireObject<En1996FlangeRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996FlangeInsertion: NormWireReader<En1996FlangeInsertion> = normWireObject<En1996FlangeInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseShearFlange) });
export const parseEn1996FlangeRelocation: NormWireReader<En1996FlangeRelocation> = normWireObject<En1996FlangeRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996FlangeModification: NormWireReader<En1996FlangeModification> = normWireObject<En1996FlangeModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1996FlangePatch)) });
export const parseEn1996FlangeDelta: NormWireReader<En1996FlangeDelta> = normWireObject<En1996FlangeDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1996FlangeRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1996FlangeInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1996FlangeRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1996FlangeModification))) });
export const parseEn1996BarPatch: NormWireReader<En1996BarPatch> = normWireObject<En1996BarPatch>({ kind: normWireOptional(normWireNullable(parseEn1996BarKind)), diameterM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0}))), lengthM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0}))), spacingM: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"maximum":10000,"exclusiveMinimum":0}))) });
export const parseEn1996BarRemoval: NormWireReader<En1996BarRemoval> = normWireObject<En1996BarRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996BarInsertion: NormWireReader<En1996BarInsertion> = normWireObject<En1996BarInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseWallBar) });
export const parseEn1996BarRelocation: NormWireReader<En1996BarRelocation> = normWireObject<En1996BarRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996BarModification: NormWireReader<En1996BarModification> = normWireObject<En1996BarModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1996BarPatch)) });
export const parseEn1996BarDelta: NormWireReader<En1996BarDelta> = normWireObject<En1996BarDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1996BarRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1996BarInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1996BarRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1996BarModification))) });
export const parseCaseNumberPatch: NormWireReader<CaseNumberPatch> = normWireObject<CaseNumberPatch>({ value: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":-1000000000000000000,"maximum":1000000000000000000}))) });
export const parseCaseNumberRemoval: NormWireReader<CaseNumberRemoval> = normWireObject<CaseNumberRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseNumberInsertion: NormWireReader<CaseNumberInsertion> = normWireObject<CaseNumberInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCaseNumber) });
export const parseCaseNumberRelocation: NormWireReader<CaseNumberRelocation> = normWireObject<CaseNumberRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseNumberModification: NormWireReader<CaseNumberModification> = normWireObject<CaseNumberModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseCaseNumberPatch)) });
export const parseCaseNumberDelta: NormWireReader<CaseNumberDelta> = normWireObject<CaseNumberDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseCaseNumberRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseCaseNumberInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseCaseNumberRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseCaseNumberModification))) });
export const parseCaseTextPatch: NormWireReader<CaseTextPatch> = normWireObject<CaseTextPatch>({ value: normWireOptional(normWireNullable(normWireString)) });
export const parseCaseTextRemoval: NormWireReader<CaseTextRemoval> = normWireObject<CaseTextRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseTextInsertion: NormWireReader<CaseTextInsertion> = normWireObject<CaseTextInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCaseText) });
export const parseCaseTextRelocation: NormWireReader<CaseTextRelocation> = normWireObject<CaseTextRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseTextModification: NormWireReader<CaseTextModification> = normWireObject<CaseTextModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseCaseTextPatch)) });
export const parseCaseTextDelta: NormWireReader<CaseTextDelta> = normWireObject<CaseTextDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseCaseTextRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseCaseTextInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseCaseTextRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseCaseTextModification))) });
export const parseCaseRowPatch: NormWireReader<CaseRowPatch> = normWireObject<CaseRowPatch>({ labelEn: normWireOptional(normWireNullable(normWireString)), labelDe: normWireOptional(normWireNullable(normWireString)), entryId: normWireOptional(normWireNullable(normWireString)), numbers: normWireRequired(normWireRef(() => parseCaseNumberDelta)), texts: normWireRequired(normWireRef(() => parseCaseTextDelta)) });
export const parseCaseRowRemoval: NormWireReader<CaseRowRemoval> = normWireObject<CaseRowRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseRowInsertion: NormWireReader<CaseRowInsertion> = normWireObject<CaseRowInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCaseRow) });
export const parseCaseRowRelocation: NormWireReader<CaseRowRelocation> = normWireObject<CaseRowRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseRowModification: NormWireReader<CaseRowModification> = normWireObject<CaseRowModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseCaseRowPatch)) });
export const parseCaseRowDelta: NormWireReader<CaseRowDelta> = normWireObject<CaseRowDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseCaseRowRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseCaseRowInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseCaseRowRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseCaseRowModification))) });
