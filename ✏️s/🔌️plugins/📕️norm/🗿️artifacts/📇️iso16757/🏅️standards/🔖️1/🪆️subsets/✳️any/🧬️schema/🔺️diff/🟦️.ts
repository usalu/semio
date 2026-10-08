/** 🔺️ `Iso16757Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireInteger, normWireNullable, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type CatalogueValue, type GeometryObject, type Iso16757ExchangeProcess, parseCatalogueValue, parseGeometryObject, parseIso16757ExchangeProcess, parsePartNumberRule, parseProduct, parseProductClass, parseProductGroup, parseProductIndex, parseProductSeries, parsePropertyDefinition, parseSelectionConstraint, parseSubject, type PartNumberRule, type Product, type ProductClass, type ProductGroup, type ProductIndex, type ProductSeries, type PropertyDefinition, type SelectionConstraint, type Subject } from "../📸️snapshot/🟦️.ts";

export interface Iso16757Diff {
  /** @state artifact */
  catalogueName: string | null;
  /** @state artifact */
  manufacturerName: string | null;
  /** @state artifact */
  productGroups: Iso16757ProductGroupsRows | null;
  /** @state artifact */
  productClasses: Iso16757ProductClassesRows | null;
  /** @state artifact */
  productSeries: Iso16757ProductSeriesRows | null;
  /** @state artifact */
  products: Iso16757ProductsRows | null;
  /** @state artifact */
  productIndexes: Iso16757ProductIndexesRows | null;
  /** @state artifact */
  propertyDefinitions: Iso16757PropertyDefinitionsRows | null;
  /** @state artifact */
  subjects: Iso16757SubjectsRows | null;
  /** @state artifact */
  geometryObjects: Iso16757GeometryObjectsRows | null;
  /** @state artifact */
  selectionClassId: string | null;
  /** @state artifact */
  selectionSeriesId: Iso16757SelectionSeriesIdValue | null;
  /** @state artifact */
  selectionConstraints: Iso16757SelectionConstraintsRows | null;
  /** @state artifact */
  partNumberRule: PartNumberRule | null;
  /** @state artifact */
  partNumberInputs: Iso16757PartNumberInputsRows | null;
  /** @state artifact */
  scriptLimits: Iso16757ScriptLimitsPatch | null;
  /** @state artifact */
  exchangeProcess: Iso16757ExchangeProcess | null;
}

export interface Iso16757GeometryObjectsEntry {
  key: string;
  value: GeometryObject;
}

export interface Iso16757GeometryObjectsRows {
  added: Iso16757GeometryObjectsEntry[];
  removed: string[];
}

export interface Iso16757PartNumberInputsEntry {
  key: string;
  value: CatalogueValue;
}

export interface Iso16757PartNumberInputsRows {
  added: Iso16757PartNumberInputsEntry[];
  removed: string[];
  modified: Iso16757PartNumberInputsEntry[];
}

export interface Iso16757ProductClassesRows {
  added: ProductClass[];
  removed: string[];
  order: string[] | null;
}

export interface Iso16757ProductGroupsPatch {
  id: string;
  name: string | null;
}

export interface Iso16757ProductGroupsRows {
  added: ProductGroup[];
  removed: string[];
  modified: Iso16757ProductGroupsPatch[];
  order: string[] | null;
}

export interface Iso16757ProductIndexesRows {
  added: ProductIndex[];
  removed: string[];
  order: string[] | null;
}

export interface Iso16757ProductSeriesRows {
  added: ProductSeries[];
  removed: string[];
  order: string[] | null;
}

export interface Iso16757ProductsPatch {
  id: string;
  name: string | null;
}

export interface Iso16757ProductsRows {
  added: Product[];
  removed: string[];
  modified: Iso16757ProductsPatch[];
  order: string[] | null;
}

export interface Iso16757PropertyDefinitionsRows {
  added: PropertyDefinition[];
  removed: string[];
  order: string[] | null;
}

export interface Iso16757ScriptLimitsPatch {
  maxSteps: number | null;
  maxRecursion: number | null;
  timeoutMs: number | null;
}

export interface Iso16757SelectionConstraintsInserted {
  index: number;
  row: SelectionConstraint;
}

export interface Iso16757SelectionConstraintsRows {
  removed: number[];
  inserted: Iso16757SelectionConstraintsInserted[];
}

export interface Iso16757SelectionSeriesIdValue {
  value: string | null;
}

export interface Iso16757SubjectsRows {
  added: Subject[];
  removed: string[];
  order: string[] | null;
}

export const parseIso16757Diff: NormWireReader<Iso16757Diff> = normWireObject<Iso16757Diff>({ catalogueName: normWireDefault(normWireNullable(normWireString), () => null), manufacturerName: normWireDefault(normWireNullable(normWireString), () => null), productGroups: normWireDefault(normWireNullable(normWireRef(() => parseIso16757ProductGroupsRows)), () => null), productClasses: normWireDefault(normWireNullable(normWireRef(() => parseIso16757ProductClassesRows)), () => null), productSeries: normWireDefault(normWireNullable(normWireRef(() => parseIso16757ProductSeriesRows)), () => null), products: normWireDefault(normWireNullable(normWireRef(() => parseIso16757ProductsRows)), () => null), productIndexes: normWireDefault(normWireNullable(normWireRef(() => parseIso16757ProductIndexesRows)), () => null), propertyDefinitions: normWireDefault(normWireNullable(normWireRef(() => parseIso16757PropertyDefinitionsRows)), () => null), subjects: normWireDefault(normWireNullable(normWireRef(() => parseIso16757SubjectsRows)), () => null), geometryObjects: normWireDefault(normWireNullable(normWireRef(() => parseIso16757GeometryObjectsRows)), () => null), selectionClassId: normWireDefault(normWireNullable(normWireString), () => null), selectionSeriesId: normWireDefault(normWireNullable(normWireRef(() => parseIso16757SelectionSeriesIdValue)), () => null), selectionConstraints: normWireDefault(normWireNullable(normWireRef(() => parseIso16757SelectionConstraintsRows)), () => null), partNumberRule: normWireDefault(normWireNullable(parsePartNumberRule), () => null), partNumberInputs: normWireDefault(normWireNullable(normWireRef(() => parseIso16757PartNumberInputsRows)), () => null), scriptLimits: normWireDefault(normWireNullable(normWireRef(() => parseIso16757ScriptLimitsPatch)), () => null), exchangeProcess: normWireDefault(normWireNullable(parseIso16757ExchangeProcess), () => null) });
export const parseIso16757GeometryObjectsEntry: NormWireReader<Iso16757GeometryObjectsEntry> = normWireObject<Iso16757GeometryObjectsEntry>({ key: normWireRequired(normWireString), value: normWireRequired(parseGeometryObject) });
export const parseIso16757GeometryObjectsRows: NormWireReader<Iso16757GeometryObjectsRows> = normWireObject<Iso16757GeometryObjectsRows>({ added: normWireRequired(normWireArray(normWireRef(() => parseIso16757GeometryObjectsEntry))), removed: normWireRequired(normWireArray(normWireString)) });
export const parseIso16757PartNumberInputsEntry: NormWireReader<Iso16757PartNumberInputsEntry> = normWireObject<Iso16757PartNumberInputsEntry>({ key: normWireRequired(normWireString), value: normWireRequired(parseCatalogueValue) });
export const parseIso16757PartNumberInputsRows: NormWireReader<Iso16757PartNumberInputsRows> = normWireObject<Iso16757PartNumberInputsRows>({ added: normWireRequired(normWireArray(normWireRef(() => parseIso16757PartNumberInputsEntry))), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseIso16757PartNumberInputsEntry))) });
export const parseIso16757ProductClassesRows: NormWireReader<Iso16757ProductClassesRows> = normWireObject<Iso16757ProductClassesRows>({ added: normWireRequired(normWireArray(parseProductClass)), removed: normWireRequired(normWireArray(normWireString)), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseIso16757ProductGroupsPatch: NormWireReader<Iso16757ProductGroupsPatch> = normWireObject<Iso16757ProductGroupsPatch>({ id: normWireRequired(normWireString), name: normWireRequired(normWireNullable(normWireString)) });
export const parseIso16757ProductGroupsRows: NormWireReader<Iso16757ProductGroupsRows> = normWireObject<Iso16757ProductGroupsRows>({ added: normWireRequired(normWireArray(parseProductGroup)), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseIso16757ProductGroupsPatch))), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseIso16757ProductIndexesRows: NormWireReader<Iso16757ProductIndexesRows> = normWireObject<Iso16757ProductIndexesRows>({ added: normWireRequired(normWireArray(parseProductIndex)), removed: normWireRequired(normWireArray(normWireString)), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseIso16757ProductSeriesRows: NormWireReader<Iso16757ProductSeriesRows> = normWireObject<Iso16757ProductSeriesRows>({ added: normWireRequired(normWireArray(parseProductSeries)), removed: normWireRequired(normWireArray(normWireString)), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseIso16757ProductsPatch: NormWireReader<Iso16757ProductsPatch> = normWireObject<Iso16757ProductsPatch>({ id: normWireRequired(normWireString), name: normWireRequired(normWireNullable(normWireString)) });
export const parseIso16757ProductsRows: NormWireReader<Iso16757ProductsRows> = normWireObject<Iso16757ProductsRows>({ added: normWireRequired(normWireArray(parseProduct)), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseIso16757ProductsPatch))), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseIso16757PropertyDefinitionsRows: NormWireReader<Iso16757PropertyDefinitionsRows> = normWireObject<Iso16757PropertyDefinitionsRows>({ added: normWireRequired(normWireArray(parsePropertyDefinition)), removed: normWireRequired(normWireArray(normWireString)), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseIso16757ScriptLimitsPatch: NormWireReader<Iso16757ScriptLimitsPatch> = normWireObject<Iso16757ScriptLimitsPatch>({ maxSteps: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), maxRecursion: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), timeoutMs: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))) });
export const parseIso16757SelectionConstraintsInserted: NormWireReader<Iso16757SelectionConstraintsInserted> = normWireObject<Iso16757SelectionConstraintsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseSelectionConstraint) });
export const parseIso16757SelectionConstraintsRows: NormWireReader<Iso16757SelectionConstraintsRows> = normWireObject<Iso16757SelectionConstraintsRows>({ removed: normWireRequired(normWireArray(normWireRange(normWireInteger, {"minimum":0}))), inserted: normWireRequired(normWireArray(normWireRef(() => parseIso16757SelectionConstraintsInserted))) });
export const parseIso16757SelectionSeriesIdValue: NormWireReader<Iso16757SelectionSeriesIdValue> = normWireObject<Iso16757SelectionSeriesIdValue>({ value: normWireRequired(normWireNullable(normWireString)) });
export const parseIso16757SubjectsRows: NormWireReader<Iso16757SubjectsRows> = normWireObject<Iso16757SubjectsRows>({ added: normWireRequired(normWireArray(parseSubject)), removed: normWireRequired(normWireArray(normWireString)), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
