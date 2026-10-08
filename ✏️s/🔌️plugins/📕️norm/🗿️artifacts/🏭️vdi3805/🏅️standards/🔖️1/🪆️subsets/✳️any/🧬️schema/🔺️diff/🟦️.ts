/** 🔺️ `Vdi3805Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireDefault, normWireInteger, normWireLiteral, normWireMap, normWireNullable, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type BoundingBox, type BuildingSystemNumber, type CatalogIndexEntry, type CharacteristicCurve, type Configuration, type CurvePoint, type EditionId, type ExtensionFields, type LocalizedText, type ParametricGeometry, parseBoundingBox, parseBuildingSystemNumber, parseCatalogIndexEntry, parseCharacteristicCurve, parseConfiguration, parseCurvePoint, parseEditionId, parseExtensionFields, parseLocalizedText, parseParametricGeometry, parseVdi3805CatalogueProduct, parseVdi3805ConnectionPoint, parseVdi3805ExtensionBag, type Vdi3805CatalogueProduct, type Vdi3805ConnectionPoint, type Vdi3805ExtensionBag } from "../📸️snapshot/🟦️.ts";

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
  indexEntries: Vdi3805IndexEntriesRows | null;
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

export interface Vdi3805GeometryConnectionsRows {
  added: Vdi3805ConnectionPoint[];
  removed: string[];
  order: string[] | null;
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

export interface Vdi3805IndexEntriesPatch {
  id: string;
  dn: Vdi3805IndexEntriesPatchDnValue | null;
  tags: string[] | null;
}

export interface Vdi3805IndexEntriesPatchDnValue {
  value: number | null;
}

export interface Vdi3805IndexEntriesRows {
  added: CatalogIndexEntry[];
  removed: string[];
  modified: Vdi3805IndexEntriesPatch[];
  order: string[] | null;
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

export interface Vdi3805ProductsPatch {
  id: string;
  title: LocalizedText[] | null;
  configuration: Configuration | null;
}

export interface Vdi3805ProductsRows {
  added: Vdi3805CatalogueProduct[];
  removed: string[];
  modified: Vdi3805ProductsPatch[];
  order: string[] | null;
}

export const parseVdi3805Diff: NormWireReader<Vdi3805Diff> = normWireObject<Vdi3805Diff>({ manufacturerFile: normWireDefault(normWireNullable(normWireRef(() => parseVdi3805ManufacturerFilePatch)), () => null), products: normWireDefault(normWireNullable(normWireRef(() => parseVdi3805ProductsRows)), () => null), catalogExtensions: normWireDefault(normWireNullable(parseExtensionFields), () => null), editionProfile: normWireDefault(normWireNullable(normWireRef(() => parseVdi3805EditionProfileRows)), () => null), correctionAsOf: normWireDefault(normWireNullable(parseEditionId), () => null), strictMode: normWireDefault(normWireNullable(normWireBoolean), () => null), indexEntries: normWireDefault(normWireNullable(normWireRef(() => parseVdi3805IndexEntriesRows)), () => null), geometry: normWireDefault(normWireNullable(normWireRef(() => parseVdi3805GeometryRows)), () => null), curves: normWireDefault(normWireNullable(normWireRef(() => parseVdi3805CurvesRows)), () => null), limits: normWireDefault(normWireNullable(normWireRef(() => parseVdi3805LimitsPatch)), () => null) });
export const parseVdi3805CurvesEntry: NormWireReader<Vdi3805CurvesEntry> = normWireObject<Vdi3805CurvesEntry>({ key: normWireRequired(normWireString), value: normWireRequired(parseCharacteristicCurve) });
export const parseVdi3805CurvesPatch: NormWireReader<Vdi3805CurvesPatch> = normWireObject<Vdi3805CurvesPatch>({ key: normWireRequired(normWireString), points: normWireRequired(normWireNullable(normWireArray(parseCurvePoint))) });
export const parseVdi3805CurvesRows: NormWireReader<Vdi3805CurvesRows> = normWireObject<Vdi3805CurvesRows>({ added: normWireRequired(normWireArray(normWireRef(() => parseVdi3805CurvesEntry))), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseVdi3805CurvesPatch))) });
export const parseVdi3805EditionProfileEntry: NormWireReader<Vdi3805EditionProfileEntry> = normWireObject<Vdi3805EditionProfileEntry>({ key: normWireRequired(normWireString), value: normWireRequired(normWireLiteral("legacy", "current")) });
export const parseVdi3805EditionProfileRows: NormWireReader<Vdi3805EditionProfileRows> = normWireObject<Vdi3805EditionProfileRows>({ added: normWireRequired(normWireArray(normWireRef(() => parseVdi3805EditionProfileEntry))), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseVdi3805EditionProfileEntry))) });
export const parseVdi3805GeometryConnectionsRows: NormWireReader<Vdi3805GeometryConnectionsRows> = normWireObject<Vdi3805GeometryConnectionsRows>({ added: normWireRequired(normWireArray(parseVdi3805ConnectionPoint)), removed: normWireRequired(normWireArray(normWireString)), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseVdi3805GeometryEntry: NormWireReader<Vdi3805GeometryEntry> = normWireObject<Vdi3805GeometryEntry>({ key: normWireRequired(normWireString), value: normWireRequired(parseParametricGeometry) });
export const parseVdi3805GeometryPatch: NormWireReader<Vdi3805GeometryPatch> = normWireObject<Vdi3805GeometryPatch>({ key: normWireRequired(normWireString), bbox: normWireRequired(normWireNullable(parseBoundingBox)), parameters: normWireRequired(normWireNullable(normWireMap(normWireNumber))), connections: normWireRequired(normWireNullable(normWireRef(() => parseVdi3805GeometryConnectionsRows))) });
export const parseVdi3805GeometryRows: NormWireReader<Vdi3805GeometryRows> = normWireObject<Vdi3805GeometryRows>({ added: normWireRequired(normWireArray(normWireRef(() => parseVdi3805GeometryEntry))), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseVdi3805GeometryPatch))) });
export const parseVdi3805IndexEntriesPatch: NormWireReader<Vdi3805IndexEntriesPatch> = normWireObject<Vdi3805IndexEntriesPatch>({ id: normWireRequired(normWireString), dn: normWireRequired(normWireNullable(normWireRef(() => parseVdi3805IndexEntriesPatchDnValue))), tags: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseVdi3805IndexEntriesPatchDnValue: NormWireReader<Vdi3805IndexEntriesPatchDnValue> = normWireObject<Vdi3805IndexEntriesPatchDnValue>({ value: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":65535}))) });
export const parseVdi3805IndexEntriesRows: NormWireReader<Vdi3805IndexEntriesRows> = normWireObject<Vdi3805IndexEntriesRows>({ added: normWireRequired(normWireArray(parseCatalogIndexEntry)), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseVdi3805IndexEntriesPatch))), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseVdi3805LimitsPatch: NormWireReader<Vdi3805LimitsPatch> = normWireObject<Vdi3805LimitsPatch>({ maxFileBytes: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), maxRecords: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), maxFieldLength: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), maxNestingDepth: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))) });
export const parseVdi3805ManufacturerFilePatch: NormWireReader<Vdi3805ManufacturerFilePatch> = normWireObject<Vdi3805ManufacturerFilePatch>({ headerVersion: normWireRequired(normWireNullable(normWireString)), manufacturer: normWireRequired(normWireNullable(normWireString)), buildingSystemNumber: normWireRequired(normWireNullable(parseBuildingSystemNumber)), created: normWireRequired(normWireNullable(normWireString)), charset: normWireRequired(normWireNullable(normWireString)), recordCount: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), extensions: normWireRequired(normWireNullable(parseVdi3805ExtensionBag)) });
export const parseVdi3805ProductsPatch: NormWireReader<Vdi3805ProductsPatch> = normWireObject<Vdi3805ProductsPatch>({ id: normWireRequired(normWireString), title: normWireRequired(normWireNullable(normWireArray(parseLocalizedText))), configuration: normWireRequired(normWireNullable(parseConfiguration)) });
export const parseVdi3805ProductsRows: NormWireReader<Vdi3805ProductsRows> = normWireObject<Vdi3805ProductsRows>({ added: normWireRequired(normWireArray(parseVdi3805CatalogueProduct)), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseVdi3805ProductsPatch))), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
