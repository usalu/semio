/** 🔺️ `En1996Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireDefault, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type ConcentratedLoad, type En1996ExposureClass, type En1996MortarClass, type En1996MortarType, type En1996UnitGroup, type En1996UnitMaterial, type En1996WallType, type MasonryWall, parseConcentratedLoad, parseEn1996ExposureClass, parseEn1996MortarClass, parseEn1996MortarType, parseEn1996UnitGroup, parseEn1996UnitMaterial, parseEn1996WallType, parseMasonryWall, parseWallLoadCase, parseWallOpening, type WallLoadCase, type WallOpening } from "../📸️snapshot/🟦️.ts";

export interface En1996Diff {
  /** @state artifact */
  annex: ("En" | "De") | null;
  /** @state artifact */
  masonryClass: ("Class1" | "Class2" | "Class3" | "Class4" | "Class5") | null;
  /** @state artifact */
  designSituation: ("Persistent" | "Transient" | "Accidental" | "Seismic") | null;
  /** @state artifact */
  storeys: number | null;
  /** @state artifact */
  walls?: En1996WallDelta;
}

export interface En1996OpeningPatch {
  widthM: number | null;
  heightM: number | null;
  sillHeightM: number | null;
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
  forceN: number | null;
  bearingAreaM2: number | null;
  bearingLengthM: number | null;
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
  designSituation: string | null;
  imposedCategory: string | null;
  gKSlabN: number | null;
  qKImposedPa: number | null;
  tributaryAreaM2: number | null;
  slabSpanM: number | null;
  qKSnowPa: number | null;
  qPWindPa: number | null;
  cPe: number | null;
  hKEarthN: number | null;
  concentrated?: En1996ConcentratedDelta;
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
  labelEn: string | null;
  labelDe: string | null;
  wallType: En1996WallType | null;
  thicknessM: number | null;
  heightM: number | null;
  lengthM: number | null;
  supportSides: number | null;
  slabBearingDepthM: number | null;
  eccentricityTopM: number | null;
  eccentricityBottomM: number | null;
  unitGroup: En1996UnitGroup | null;
  unitMaterial: En1996UnitMaterial | null;
  fBPa: number | null;
  unitLengthM: number | null;
  unitWidthM: number | null;
  unitHeightM: number | null;
  mortarType: En1996MortarType | null;
  mortarClass: En1996MortarClass | null;
  mortarStrengthPa: number | null;
  bedJointThicknessM: number | null;
  reinforced: boolean | null;
  asVerticalM2: number | null;
  asHorizontalM2: number | null;
  fYdPa: number | null;
  fireReiMin: number | null;
  exposure: En1996ExposureClass | null;
  mu: number | null;
  densityKgM3: number | null;
  phiInfinity: number | null;
  isBasement: boolean | null;
  openings?: En1996OpeningDelta;
  loadCases?: En1996LoadCaseDelta;
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

export const parseEn1996Diff: NormWireReader<En1996Diff> = normWireObject<En1996Diff>({ annex: normWireDefault(normWireNullable(normWireLiteral("En", "De")), () => null), masonryClass: normWireDefault(normWireNullable(normWireLiteral("Class1", "Class2", "Class3", "Class4", "Class5")), () => null), designSituation: normWireDefault(normWireNullable(normWireLiteral("Persistent", "Transient", "Accidental", "Seismic")), () => null), storeys: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), () => null), walls: normWireOptional(normWireRef(() => parseEn1996WallDelta)) });
export const parseEn1996OpeningPatch: NormWireReader<En1996OpeningPatch> = normWireObject<En1996OpeningPatch>({ widthM: normWireDefault(normWireNullable(normWireNumber), () => null), heightM: normWireDefault(normWireNullable(normWireNumber), () => null), sillHeightM: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseEn1996OpeningRemoval: NormWireReader<En1996OpeningRemoval> = normWireObject<En1996OpeningRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996OpeningInsertion: NormWireReader<En1996OpeningInsertion> = normWireObject<En1996OpeningInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseWallOpening) });
export const parseEn1996OpeningRelocation: NormWireReader<En1996OpeningRelocation> = normWireObject<En1996OpeningRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996OpeningModification: NormWireReader<En1996OpeningModification> = normWireObject<En1996OpeningModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1996OpeningPatch)) });
export const parseEn1996OpeningDelta: NormWireReader<En1996OpeningDelta> = normWireObject<En1996OpeningDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1996OpeningRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1996OpeningInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1996OpeningRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1996OpeningModification))) });
export const parseEn1996ConcentratedPatch: NormWireReader<En1996ConcentratedPatch> = normWireObject<En1996ConcentratedPatch>({ forceN: normWireDefault(normWireNullable(normWireNumber), () => null), bearingAreaM2: normWireDefault(normWireNullable(normWireNumber), () => null), bearingLengthM: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseEn1996ConcentratedRemoval: NormWireReader<En1996ConcentratedRemoval> = normWireObject<En1996ConcentratedRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996ConcentratedInsertion: NormWireReader<En1996ConcentratedInsertion> = normWireObject<En1996ConcentratedInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseConcentratedLoad) });
export const parseEn1996ConcentratedRelocation: NormWireReader<En1996ConcentratedRelocation> = normWireObject<En1996ConcentratedRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996ConcentratedModification: NormWireReader<En1996ConcentratedModification> = normWireObject<En1996ConcentratedModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1996ConcentratedPatch)) });
export const parseEn1996ConcentratedDelta: NormWireReader<En1996ConcentratedDelta> = normWireObject<En1996ConcentratedDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1996ConcentratedRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1996ConcentratedInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1996ConcentratedRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1996ConcentratedModification))) });
export const parseEn1996LoadCasePatch: NormWireReader<En1996LoadCasePatch> = normWireObject<En1996LoadCasePatch>({ designSituation: normWireDefault(normWireNullable(normWireString), () => null), imposedCategory: normWireDefault(normWireNullable(normWireString), () => null), gKSlabN: normWireDefault(normWireNullable(normWireNumber), () => null), qKImposedPa: normWireDefault(normWireNullable(normWireNumber), () => null), tributaryAreaM2: normWireDefault(normWireNullable(normWireNumber), () => null), slabSpanM: normWireDefault(normWireNullable(normWireNumber), () => null), qKSnowPa: normWireDefault(normWireNullable(normWireNumber), () => null), qPWindPa: normWireDefault(normWireNullable(normWireNumber), () => null), cPe: normWireDefault(normWireNullable(normWireNumber), () => null), hKEarthN: normWireDefault(normWireNullable(normWireNumber), () => null), concentrated: normWireOptional(normWireRef(() => parseEn1996ConcentratedDelta)) });
export const parseEn1996LoadCaseRemoval: NormWireReader<En1996LoadCaseRemoval> = normWireObject<En1996LoadCaseRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996LoadCaseInsertion: NormWireReader<En1996LoadCaseInsertion> = normWireObject<En1996LoadCaseInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseWallLoadCase) });
export const parseEn1996LoadCaseRelocation: NormWireReader<En1996LoadCaseRelocation> = normWireObject<En1996LoadCaseRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996LoadCaseModification: NormWireReader<En1996LoadCaseModification> = normWireObject<En1996LoadCaseModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1996LoadCasePatch)) });
export const parseEn1996LoadCaseDelta: NormWireReader<En1996LoadCaseDelta> = normWireObject<En1996LoadCaseDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1996LoadCaseRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1996LoadCaseInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1996LoadCaseRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1996LoadCaseModification))) });
export const parseEn1996WallPatch: NormWireReader<En1996WallPatch> = normWireObject<En1996WallPatch>({ labelEn: normWireDefault(normWireNullable(normWireString), () => null), labelDe: normWireDefault(normWireNullable(normWireString), () => null), wallType: normWireDefault(normWireNullable(parseEn1996WallType), () => null), thicknessM: normWireDefault(normWireNullable(normWireNumber), () => null), heightM: normWireDefault(normWireNullable(normWireNumber), () => null), lengthM: normWireDefault(normWireNullable(normWireNumber), () => null), supportSides: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0})), () => null), slabBearingDepthM: normWireDefault(normWireNullable(normWireNumber), () => null), eccentricityTopM: normWireDefault(normWireNullable(normWireNumber), () => null), eccentricityBottomM: normWireDefault(normWireNullable(normWireNumber), () => null), unitGroup: normWireDefault(normWireNullable(parseEn1996UnitGroup), () => null), unitMaterial: normWireDefault(normWireNullable(parseEn1996UnitMaterial), () => null), fBPa: normWireDefault(normWireNullable(normWireNumber), () => null), unitLengthM: normWireDefault(normWireNullable(normWireNumber), () => null), unitWidthM: normWireDefault(normWireNullable(normWireNumber), () => null), unitHeightM: normWireDefault(normWireNullable(normWireNumber), () => null), mortarType: normWireDefault(normWireNullable(parseEn1996MortarType), () => null), mortarClass: normWireDefault(normWireNullable(parseEn1996MortarClass), () => null), mortarStrengthPa: normWireDefault(normWireNullable(normWireNumber), () => null), bedJointThicknessM: normWireDefault(normWireNullable(normWireNumber), () => null), reinforced: normWireDefault(normWireNullable(normWireBoolean), () => null), asVerticalM2: normWireDefault(normWireNullable(normWireNumber), () => null), asHorizontalM2: normWireDefault(normWireNullable(normWireNumber), () => null), fYdPa: normWireDefault(normWireNullable(normWireNumber), () => null), fireReiMin: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0})), () => null), exposure: normWireDefault(normWireNullable(parseEn1996ExposureClass), () => null), mu: normWireDefault(normWireNullable(normWireNumber), () => null), densityKgM3: normWireDefault(normWireNullable(normWireNumber), () => null), phiInfinity: normWireDefault(normWireNullable(normWireNumber), () => null), isBasement: normWireDefault(normWireNullable(normWireBoolean), () => null), openings: normWireOptional(normWireRef(() => parseEn1996OpeningDelta)), loadCases: normWireOptional(normWireRef(() => parseEn1996LoadCaseDelta)) });
export const parseEn1996WallRemoval: NormWireReader<En1996WallRemoval> = normWireObject<En1996WallRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996WallInsertion: NormWireReader<En1996WallInsertion> = normWireObject<En1996WallInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseMasonryWall) });
export const parseEn1996WallRelocation: NormWireReader<En1996WallRelocation> = normWireObject<En1996WallRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1996WallModification: NormWireReader<En1996WallModification> = normWireObject<En1996WallModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1996WallPatch)) });
export const parseEn1996WallDelta: NormWireReader<En1996WallDelta> = normWireObject<En1996WallDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1996WallRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1996WallInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1996WallRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1996WallModification))) });
