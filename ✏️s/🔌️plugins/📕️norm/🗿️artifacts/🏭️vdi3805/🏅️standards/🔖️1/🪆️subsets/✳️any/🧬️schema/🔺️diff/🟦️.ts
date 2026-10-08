/** 🔺️ `Vdi3805Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireDefault, normWireInteger, normWireLiteral, normWireMap, normWireNullable, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type BoundingBox, type BuildingSystemNumber, type CharacteristicCurve, type Configuration, type CurvePoint, type EditionId, type ExtensionFields, type LocalizedText, type ParametricGeometry, parseBoundingBox, parseBuildingSystemNumber, parseCharacteristicCurve, parseConfiguration, parseCurvePoint, parseEditionId, parseExtensionFields, parseLocalizedText, parseParametricGeometry, parseVdi3805CatalogueProduct, parseVdi3805ConnectionPoint, parseVdi3805ExtensionBag, type Vdi3805CatalogueProduct, type Vdi3805ConnectionPoint, type Vdi3805ExtensionBag } from "../📸️snapshot/🟦️.ts";

export interface Vdi3805Diff {
  /** @state artifact */
  manufacturerFile: Vdi3805ManufacturerFilePatch | null;
  /** @state artifact */
  products: Vdi3805ProductsRows | null;
  /** @state artifact */
  catalogExtensions: ExtensionFields | null;
  /** @state artifact */
  editionProfile: Vdi3805EditionProfileRows | null;
  /** @state artifact */
  correctionAsOf: EditionId | null;
  /** @state artifact */
  strictMode: boolean | null;
  /** @state artifact */
  geometry: Vdi3805GeometryRows | null;
  /** @state artifact */
  curves: Vdi3805CurvesRows | null;
  /** @state artifact */
  limits: Vdi3805LimitsPatch | null;
}

export interface Vdi3805CurvesEntry {
  key: string;
  value: CharacteristicCurve;
}

export interface Vdi3805CurvesPatch {
  key: string;
  points: CurvePoint[] | null;
}

export interface Vdi3805CurvesRows {
  added: Vdi3805CurvesEntry[];
  removed: string[];
  modified: Vdi3805CurvesPatch[];
}

export interface Vdi3805EditionProfileEntry {
  key: string;
  value: "legacy" | "current";
}

export interface Vdi3805EditionProfileRows {
  added: Vdi3805EditionProfileEntry[];
  removed: string[];
  modified: Vdi3805EditionProfileEntry[];
}

export interface Vdi3805GeometryConnectionsInserted {
  index: number;
  row: Vdi3805ConnectionPoint;
}

export interface Vdi3805GeometryConnectionsModified {
  id: string;
  patch: Vdi3805GeometryConnectionsPatch;
}

export interface Vdi3805GeometryConnectionsMoved {
  id: string;
  from: number;
  to: number;
}

export interface Vdi3805GeometryConnectionsPatch {
  medium: string | null;
  position: number[] | null;
  direction: number[] | null;
  diameterMm: Vdi3805GeometryConnectionsPatchDiameterMmValue | null;
}

export interface Vdi3805GeometryConnectionsPatchDiameterMmValue {
  value: number | null;
}

export interface Vdi3805GeometryConnectionsRemoved {
  id: string;
  index: number;
}

export interface Vdi3805GeometryConnectionsRows {
  removed: Vdi3805GeometryConnectionsRemoved[];
  inserted: Vdi3805GeometryConnectionsInserted[];
  moved: Vdi3805GeometryConnectionsMoved[];
  modified: Vdi3805GeometryConnectionsModified[];
}

export interface Vdi3805GeometryEntry {
  key: string;
  value: ParametricGeometry;
}

export interface Vdi3805GeometryPatch {
  key: string;
  bbox: BoundingBox | null;
  parameters: { [key: string]: number } | null;
  connections: Vdi3805GeometryConnectionsRows | null;
}

export interface Vdi3805GeometryRows {
  added: Vdi3805GeometryEntry[];
  removed: string[];
  modified: Vdi3805GeometryPatch[];
}

export interface Vdi3805LimitsPatch {
  maxFileBytes: number | null;
  maxRecords: number | null;
  maxFieldLength: number | null;
  maxNestingDepth: number | null;
}

export interface Vdi3805ManufacturerFilePatch {
  headerVersion: string | null;
  manufacturer: string | null;
  buildingSystemNumber: BuildingSystemNumber | null;
  created: string | null;
  charset: string | null;
  recordCount: number | null;
  extensions: Vdi3805ExtensionBag | null;
}

export interface Vdi3805ProductsInserted {
  index: number;
  row: Vdi3805CatalogueProduct;
}

export interface Vdi3805ProductsModified {
  id: string;
  patch: Vdi3805ProductsPatch;
}

export interface Vdi3805ProductsMoved {
  id: string;
  from: number;
  to: number;
}

export interface Vdi3805ProductsPatch {
  title: LocalizedText[] | null;
  configuration: Configuration | null;
}

export interface Vdi3805ProductsRemoved {
  id: string;
  index: number;
}

export interface Vdi3805ProductsRows {
  removed: Vdi3805ProductsRemoved[];
  inserted: Vdi3805ProductsInserted[];
  moved: Vdi3805ProductsMoved[];
  modified: Vdi3805ProductsModified[];
}

export const parseVdi3805Diff: NormWireReader<Vdi3805Diff> = normWireObject<Vdi3805Diff>({ manufacturerFile: normWireDefault(normWireNullable(normWireRef(() => parseVdi3805ManufacturerFilePatch)), () => null), products: normWireDefault(normWireNullable(normWireRef(() => parseVdi3805ProductsRows)), () => null), catalogExtensions: normWireDefault(normWireNullable(parseExtensionFields), () => null), editionProfile: normWireDefault(normWireNullable(normWireRef(() => parseVdi3805EditionProfileRows)), () => null), correctionAsOf: normWireDefault(normWireNullable(parseEditionId), () => null), strictMode: normWireDefault(normWireNullable(normWireBoolean), () => null), geometry: normWireDefault(normWireNullable(normWireRef(() => parseVdi3805GeometryRows)), () => null), curves: normWireDefault(normWireNullable(normWireRef(() => parseVdi3805CurvesRows)), () => null), limits: normWireDefault(normWireNullable(normWireRef(() => parseVdi3805LimitsPatch)), () => null) });
export const parseVdi3805CurvesEntry: NormWireReader<Vdi3805CurvesEntry> = normWireObject<Vdi3805CurvesEntry>({ key: normWireRequired(normWireString), value: normWireRequired(parseCharacteristicCurve) });
export const parseVdi3805CurvesPatch: NormWireReader<Vdi3805CurvesPatch> = normWireObject<Vdi3805CurvesPatch>({ key: normWireRequired(normWireString), points: normWireRequired(normWireNullable(normWireArray(parseCurvePoint))) });
export const parseVdi3805CurvesRows: NormWireReader<Vdi3805CurvesRows> = normWireObject<Vdi3805CurvesRows>({ added: normWireRequired(normWireArray(normWireRef(() => parseVdi3805CurvesEntry))), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseVdi3805CurvesPatch))) });
export const parseVdi3805EditionProfileEntry: NormWireReader<Vdi3805EditionProfileEntry> = normWireObject<Vdi3805EditionProfileEntry>({ key: normWireRequired(normWireString), value: normWireRequired(normWireLiteral("legacy", "current")) });
export const parseVdi3805EditionProfileRows: NormWireReader<Vdi3805EditionProfileRows> = normWireObject<Vdi3805EditionProfileRows>({ added: normWireRequired(normWireArray(normWireRef(() => parseVdi3805EditionProfileEntry))), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseVdi3805EditionProfileEntry))) });
export const parseVdi3805GeometryConnectionsInserted: NormWireReader<Vdi3805GeometryConnectionsInserted> = normWireObject<Vdi3805GeometryConnectionsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseVdi3805ConnectionPoint) });
export const parseVdi3805GeometryConnectionsModified: NormWireReader<Vdi3805GeometryConnectionsModified> = normWireObject<Vdi3805GeometryConnectionsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseVdi3805GeometryConnectionsPatch)) });
export const parseVdi3805GeometryConnectionsMoved: NormWireReader<Vdi3805GeometryConnectionsMoved> = normWireObject<Vdi3805GeometryConnectionsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseVdi3805GeometryConnectionsPatch: NormWireReader<Vdi3805GeometryConnectionsPatch> = normWireObject<Vdi3805GeometryConnectionsPatch>({ medium: normWireRequired(normWireNullable(normWireString)), position: normWireRequired(normWireNullable(normWireArray(normWireNumber, 3, 3))), direction: normWireRequired(normWireNullable(normWireArray(normWireNumber, 3, 3))), diameterMm: normWireRequired(normWireNullable(normWireRef(() => parseVdi3805GeometryConnectionsPatchDiameterMmValue))) });
export const parseVdi3805GeometryConnectionsPatchDiameterMmValue: NormWireReader<Vdi3805GeometryConnectionsPatchDiameterMmValue> = normWireObject<Vdi3805GeometryConnectionsPatchDiameterMmValue>({ value: normWireRequired(normWireNullable(normWireNumber)) });
export const parseVdi3805GeometryConnectionsRemoved: NormWireReader<Vdi3805GeometryConnectionsRemoved> = normWireObject<Vdi3805GeometryConnectionsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseVdi3805GeometryConnectionsRows: NormWireReader<Vdi3805GeometryConnectionsRows> = normWireObject<Vdi3805GeometryConnectionsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseVdi3805GeometryConnectionsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseVdi3805GeometryConnectionsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseVdi3805GeometryConnectionsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseVdi3805GeometryConnectionsModified))) });
export const parseVdi3805GeometryEntry: NormWireReader<Vdi3805GeometryEntry> = normWireObject<Vdi3805GeometryEntry>({ key: normWireRequired(normWireString), value: normWireRequired(parseParametricGeometry) });
export const parseVdi3805GeometryPatch: NormWireReader<Vdi3805GeometryPatch> = normWireObject<Vdi3805GeometryPatch>({ key: normWireRequired(normWireString), bbox: normWireRequired(normWireNullable(parseBoundingBox)), parameters: normWireRequired(normWireNullable(normWireMap(normWireNumber))), connections: normWireRequired(normWireNullable(normWireRef(() => parseVdi3805GeometryConnectionsRows))) });
export const parseVdi3805GeometryRows: NormWireReader<Vdi3805GeometryRows> = normWireObject<Vdi3805GeometryRows>({ added: normWireRequired(normWireArray(normWireRef(() => parseVdi3805GeometryEntry))), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseVdi3805GeometryPatch))) });
export const parseVdi3805LimitsPatch: NormWireReader<Vdi3805LimitsPatch> = normWireObject<Vdi3805LimitsPatch>({ maxFileBytes: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), maxRecords: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), maxFieldLength: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), maxNestingDepth: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))) });
export const parseVdi3805ManufacturerFilePatch: NormWireReader<Vdi3805ManufacturerFilePatch> = normWireObject<Vdi3805ManufacturerFilePatch>({ headerVersion: normWireRequired(normWireNullable(normWireString)), manufacturer: normWireRequired(normWireNullable(normWireString)), buildingSystemNumber: normWireRequired(normWireNullable(parseBuildingSystemNumber)), created: normWireRequired(normWireNullable(normWireString)), charset: normWireRequired(normWireNullable(normWireString)), recordCount: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), extensions: normWireRequired(normWireNullable(parseVdi3805ExtensionBag)) });
export const parseVdi3805ProductsInserted: NormWireReader<Vdi3805ProductsInserted> = normWireObject<Vdi3805ProductsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseVdi3805CatalogueProduct) });
export const parseVdi3805ProductsModified: NormWireReader<Vdi3805ProductsModified> = normWireObject<Vdi3805ProductsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseVdi3805ProductsPatch)) });
export const parseVdi3805ProductsMoved: NormWireReader<Vdi3805ProductsMoved> = normWireObject<Vdi3805ProductsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseVdi3805ProductsPatch: NormWireReader<Vdi3805ProductsPatch> = normWireObject<Vdi3805ProductsPatch>({ title: normWireRequired(normWireNullable(normWireArray(parseLocalizedText))), configuration: normWireRequired(normWireNullable(parseConfiguration)) });
export const parseVdi3805ProductsRemoved: NormWireReader<Vdi3805ProductsRemoved> = normWireObject<Vdi3805ProductsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseVdi3805ProductsRows: NormWireReader<Vdi3805ProductsRows> = normWireObject<Vdi3805ProductsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseVdi3805ProductsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseVdi3805ProductsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseVdi3805ProductsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseVdi3805ProductsModified))) });
