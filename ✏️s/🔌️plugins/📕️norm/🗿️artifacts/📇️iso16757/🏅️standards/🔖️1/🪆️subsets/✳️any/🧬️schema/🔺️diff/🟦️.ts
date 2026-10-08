/** 🔺️ `Iso16757Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireInteger, normWireMap, normWireNullable, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type CatalogueUnit, type CatalogueValue, type GeometryObject, type Iso16757Cardinality, type Iso16757ExchangeProcess, type Iso16757PropertyKind, type Iso16757SubjectKind, type LocalizedText, type Names, parseCatalogueUnit, parseCatalogueValue, parseGeometryObject, parseIso16757Cardinality, parseIso16757ExchangeProcess, parseIso16757PropertyKind, parseIso16757SubjectKind, parseLocalizedText, parseNames, parsePartNumberRule, parseProduct, parseProductClass, parseProductGroup, parseProductIndex, parseProductSeries, parsePropertyDefinition, parseSelectionConstraint, parseSubject, type PartNumberRule, type Product, type ProductClass, type ProductGroup, type ProductIndex, type ProductSeries, type PropertyDefinition, type SelectionConstraint, type Subject } from "../📸️snapshot/🟦️.ts";

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

export interface Iso16757ProductClassesInserted {
  index: number;
  row: ProductClass;
}

export interface Iso16757ProductClassesModified {
  id: string;
  patch: Iso16757ProductClassesPatch;
}

export interface Iso16757ProductClassesMoved {
  id: string;
  from: number;
  to: number;
}

export interface Iso16757ProductClassesPatch {
  groupId: string | null;
  parentId: Iso16757ProductClassesPatchParentIdValue | null;
  names: Names | null;
  requiredPropertyIds: string[] | null;
  optionalPropertyIds: string[] | null;
}

export interface Iso16757ProductClassesPatchParentIdValue {
  value: string | null;
}

export interface Iso16757ProductClassesRemoved {
  id: string;
  index: number;
}

export interface Iso16757ProductClassesRows {
  removed: Iso16757ProductClassesRemoved[];
  inserted: Iso16757ProductClassesInserted[];
  moved: Iso16757ProductClassesMoved[];
  modified: Iso16757ProductClassesModified[];
}

export interface Iso16757ProductGroupsInserted {
  index: number;
  row: ProductGroup;
}

export interface Iso16757ProductGroupsModified {
  id: string;
  patch: Iso16757ProductGroupsPatch;
}

export interface Iso16757ProductGroupsMoved {
  id: string;
  from: number;
  to: number;
}

export interface Iso16757ProductGroupsPatch {
  name: string | null;
}

export interface Iso16757ProductGroupsRemoved {
  id: string;
  index: number;
}

export interface Iso16757ProductGroupsRows {
  removed: Iso16757ProductGroupsRemoved[];
  inserted: Iso16757ProductGroupsInserted[];
  moved: Iso16757ProductGroupsMoved[];
  modified: Iso16757ProductGroupsModified[];
}

export interface Iso16757ProductIndexesInserted {
  index: number;
  row: ProductIndex;
}

export interface Iso16757ProductIndexesModified {
  id: string;
  patch: Iso16757ProductIndexesPatch;
}

export interface Iso16757ProductIndexesMoved {
  id: string;
  from: number;
  to: number;
}

export interface Iso16757ProductIndexesPatch {
  productId: string | null;
  variantId: Iso16757ProductIndexesPatchVariantIdValue | null;
  searchTags: string[] | null;
}

export interface Iso16757ProductIndexesPatchVariantIdValue {
  value: string | null;
}

export interface Iso16757ProductIndexesRemoved {
  id: string;
  index: number;
}

export interface Iso16757ProductIndexesRows {
  removed: Iso16757ProductIndexesRemoved[];
  inserted: Iso16757ProductIndexesInserted[];
  moved: Iso16757ProductIndexesMoved[];
  modified: Iso16757ProductIndexesModified[];
}

export interface Iso16757ProductSeriesInserted {
  index: number;
  row: ProductSeries;
}

export interface Iso16757ProductSeriesModified {
  id: string;
  patch: Iso16757ProductSeriesPatch;
}

export interface Iso16757ProductSeriesMoved {
  id: string;
  from: number;
  to: number;
}

export interface Iso16757ProductSeriesPatch {
  classId: string | null;
  names: Names | null;
  sharedPropertyValues: { [key: string]: CatalogueValue } | null;
  geometryId: Iso16757ProductSeriesPatchGeometryIdValue | null;
}

export interface Iso16757ProductSeriesPatchGeometryIdValue {
  value: string | null;
}

export interface Iso16757ProductSeriesRemoved {
  id: string;
  index: number;
}

export interface Iso16757ProductSeriesRows {
  removed: Iso16757ProductSeriesRemoved[];
  inserted: Iso16757ProductSeriesInserted[];
  moved: Iso16757ProductSeriesMoved[];
  modified: Iso16757ProductSeriesModified[];
}

export interface Iso16757ProductsInserted {
  index: number;
  row: Product;
}

export interface Iso16757ProductsModified {
  id: string;
  patch: Iso16757ProductsPatch;
}

export interface Iso16757ProductsMoved {
  id: string;
  from: number;
  to: number;
}

export interface Iso16757ProductsPatch {
  name: string | null;
}

export interface Iso16757ProductsRemoved {
  id: string;
  index: number;
}

export interface Iso16757ProductsRows {
  removed: Iso16757ProductsRemoved[];
  inserted: Iso16757ProductsInserted[];
  moved: Iso16757ProductsMoved[];
  modified: Iso16757ProductsModified[];
}

export interface Iso16757PropertyDefinitionsInserted {
  index: number;
  row: PropertyDefinition;
}

export interface Iso16757PropertyDefinitionsModified {
  id: string;
  patch: Iso16757PropertyDefinitionsPatch;
}

export interface Iso16757PropertyDefinitionsMoved {
  id: string;
  from: number;
  to: number;
}

export interface Iso16757PropertyDefinitionsPatch {
  names: Names | null;
  dataType: string | null;
  unit: Iso16757PropertyDefinitionsPatchUnitValue | null;
  cardinality: Iso16757Cardinality | null;
  kind: Iso16757PropertyKind | null;
  dictionaryPropertyId: Iso16757PropertyDefinitionsPatchDictionaryPropertyIdValue | null;
}

export interface Iso16757PropertyDefinitionsPatchDictionaryPropertyIdValue {
  value: string | null;
}

export interface Iso16757PropertyDefinitionsPatchUnitValue {
  value: CatalogueUnit | null;
}

export interface Iso16757PropertyDefinitionsRemoved {
  id: string;
  index: number;
}

export interface Iso16757PropertyDefinitionsRows {
  removed: Iso16757PropertyDefinitionsRemoved[];
  inserted: Iso16757PropertyDefinitionsInserted[];
  moved: Iso16757PropertyDefinitionsMoved[];
  modified: Iso16757PropertyDefinitionsModified[];
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

export interface Iso16757SubjectsInserted {
  index: number;
  row: Subject;
}

export interface Iso16757SubjectsModified {
  id: string;
  patch: Iso16757SubjectsPatch;
}

export interface Iso16757SubjectsMoved {
  id: string;
  from: number;
  to: number;
}

export interface Iso16757SubjectsPatch {
  kind: Iso16757SubjectKind | null;
  names: Names | null;
  definition: LocalizedText | null;
  parentId: Iso16757SubjectsPatchParentIdValue | null;
}

export interface Iso16757SubjectsPatchParentIdValue {
  value: string | null;
}

export interface Iso16757SubjectsRemoved {
  id: string;
  index: number;
}

export interface Iso16757SubjectsRows {
  removed: Iso16757SubjectsRemoved[];
  inserted: Iso16757SubjectsInserted[];
  moved: Iso16757SubjectsMoved[];
  modified: Iso16757SubjectsModified[];
}

export const parseIso16757Diff: NormWireReader<Iso16757Diff> = normWireObject<Iso16757Diff>({ catalogueName: normWireDefault(normWireNullable(normWireString), () => null), manufacturerName: normWireDefault(normWireNullable(normWireString), () => null), productGroups: normWireDefault(normWireNullable(normWireRef(() => parseIso16757ProductGroupsRows)), () => null), productClasses: normWireDefault(normWireNullable(normWireRef(() => parseIso16757ProductClassesRows)), () => null), productSeries: normWireDefault(normWireNullable(normWireRef(() => parseIso16757ProductSeriesRows)), () => null), products: normWireDefault(normWireNullable(normWireRef(() => parseIso16757ProductsRows)), () => null), productIndexes: normWireDefault(normWireNullable(normWireRef(() => parseIso16757ProductIndexesRows)), () => null), propertyDefinitions: normWireDefault(normWireNullable(normWireRef(() => parseIso16757PropertyDefinitionsRows)), () => null), subjects: normWireDefault(normWireNullable(normWireRef(() => parseIso16757SubjectsRows)), () => null), geometryObjects: normWireDefault(normWireNullable(normWireRef(() => parseIso16757GeometryObjectsRows)), () => null), selectionClassId: normWireDefault(normWireNullable(normWireString), () => null), selectionSeriesId: normWireDefault(normWireNullable(normWireRef(() => parseIso16757SelectionSeriesIdValue)), () => null), selectionConstraints: normWireDefault(normWireNullable(normWireRef(() => parseIso16757SelectionConstraintsRows)), () => null), partNumberRule: normWireDefault(normWireNullable(parsePartNumberRule), () => null), partNumberInputs: normWireDefault(normWireNullable(normWireRef(() => parseIso16757PartNumberInputsRows)), () => null), scriptLimits: normWireDefault(normWireNullable(normWireRef(() => parseIso16757ScriptLimitsPatch)), () => null), exchangeProcess: normWireDefault(normWireNullable(parseIso16757ExchangeProcess), () => null) });
export const parseIso16757GeometryObjectsEntry: NormWireReader<Iso16757GeometryObjectsEntry> = normWireObject<Iso16757GeometryObjectsEntry>({ key: normWireRequired(normWireString), value: normWireRequired(parseGeometryObject) });
export const parseIso16757GeometryObjectsRows: NormWireReader<Iso16757GeometryObjectsRows> = normWireObject<Iso16757GeometryObjectsRows>({ added: normWireRequired(normWireArray(normWireRef(() => parseIso16757GeometryObjectsEntry))), removed: normWireRequired(normWireArray(normWireString)) });
export const parseIso16757PartNumberInputsEntry: NormWireReader<Iso16757PartNumberInputsEntry> = normWireObject<Iso16757PartNumberInputsEntry>({ key: normWireRequired(normWireString), value: normWireRequired(parseCatalogueValue) });
export const parseIso16757PartNumberInputsRows: NormWireReader<Iso16757PartNumberInputsRows> = normWireObject<Iso16757PartNumberInputsRows>({ added: normWireRequired(normWireArray(normWireRef(() => parseIso16757PartNumberInputsEntry))), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseIso16757PartNumberInputsEntry))) });
export const parseIso16757ProductClassesInserted: NormWireReader<Iso16757ProductClassesInserted> = normWireObject<Iso16757ProductClassesInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseProductClass) });
export const parseIso16757ProductClassesModified: NormWireReader<Iso16757ProductClassesModified> = normWireObject<Iso16757ProductClassesModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseIso16757ProductClassesPatch)) });
export const parseIso16757ProductClassesMoved: NormWireReader<Iso16757ProductClassesMoved> = normWireObject<Iso16757ProductClassesMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseIso16757ProductClassesPatch: NormWireReader<Iso16757ProductClassesPatch> = normWireObject<Iso16757ProductClassesPatch>({ groupId: normWireRequired(normWireNullable(normWireString)), parentId: normWireRequired(normWireNullable(normWireRef(() => parseIso16757ProductClassesPatchParentIdValue))), names: normWireRequired(normWireNullable(parseNames)), requiredPropertyIds: normWireRequired(normWireNullable(normWireArray(normWireString))), optionalPropertyIds: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseIso16757ProductClassesPatchParentIdValue: NormWireReader<Iso16757ProductClassesPatchParentIdValue> = normWireObject<Iso16757ProductClassesPatchParentIdValue>({ value: normWireRequired(normWireNullable(normWireString)) });
export const parseIso16757ProductClassesRemoved: NormWireReader<Iso16757ProductClassesRemoved> = normWireObject<Iso16757ProductClassesRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseIso16757ProductClassesRows: NormWireReader<Iso16757ProductClassesRows> = normWireObject<Iso16757ProductClassesRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseIso16757ProductClassesRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseIso16757ProductClassesInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseIso16757ProductClassesMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseIso16757ProductClassesModified))) });
export const parseIso16757ProductGroupsInserted: NormWireReader<Iso16757ProductGroupsInserted> = normWireObject<Iso16757ProductGroupsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseProductGroup) });
export const parseIso16757ProductGroupsModified: NormWireReader<Iso16757ProductGroupsModified> = normWireObject<Iso16757ProductGroupsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseIso16757ProductGroupsPatch)) });
export const parseIso16757ProductGroupsMoved: NormWireReader<Iso16757ProductGroupsMoved> = normWireObject<Iso16757ProductGroupsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseIso16757ProductGroupsPatch: NormWireReader<Iso16757ProductGroupsPatch> = normWireObject<Iso16757ProductGroupsPatch>({ name: normWireRequired(normWireNullable(normWireString)) });
export const parseIso16757ProductGroupsRemoved: NormWireReader<Iso16757ProductGroupsRemoved> = normWireObject<Iso16757ProductGroupsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseIso16757ProductGroupsRows: NormWireReader<Iso16757ProductGroupsRows> = normWireObject<Iso16757ProductGroupsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseIso16757ProductGroupsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseIso16757ProductGroupsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseIso16757ProductGroupsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseIso16757ProductGroupsModified))) });
export const parseIso16757ProductIndexesInserted: NormWireReader<Iso16757ProductIndexesInserted> = normWireObject<Iso16757ProductIndexesInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseProductIndex) });
export const parseIso16757ProductIndexesModified: NormWireReader<Iso16757ProductIndexesModified> = normWireObject<Iso16757ProductIndexesModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseIso16757ProductIndexesPatch)) });
export const parseIso16757ProductIndexesMoved: NormWireReader<Iso16757ProductIndexesMoved> = normWireObject<Iso16757ProductIndexesMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseIso16757ProductIndexesPatch: NormWireReader<Iso16757ProductIndexesPatch> = normWireObject<Iso16757ProductIndexesPatch>({ productId: normWireRequired(normWireNullable(normWireString)), variantId: normWireRequired(normWireNullable(normWireRef(() => parseIso16757ProductIndexesPatchVariantIdValue))), searchTags: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseIso16757ProductIndexesPatchVariantIdValue: NormWireReader<Iso16757ProductIndexesPatchVariantIdValue> = normWireObject<Iso16757ProductIndexesPatchVariantIdValue>({ value: normWireRequired(normWireNullable(normWireString)) });
export const parseIso16757ProductIndexesRemoved: NormWireReader<Iso16757ProductIndexesRemoved> = normWireObject<Iso16757ProductIndexesRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseIso16757ProductIndexesRows: NormWireReader<Iso16757ProductIndexesRows> = normWireObject<Iso16757ProductIndexesRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseIso16757ProductIndexesRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseIso16757ProductIndexesInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseIso16757ProductIndexesMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseIso16757ProductIndexesModified))) });
export const parseIso16757ProductSeriesInserted: NormWireReader<Iso16757ProductSeriesInserted> = normWireObject<Iso16757ProductSeriesInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseProductSeries) });
export const parseIso16757ProductSeriesModified: NormWireReader<Iso16757ProductSeriesModified> = normWireObject<Iso16757ProductSeriesModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseIso16757ProductSeriesPatch)) });
export const parseIso16757ProductSeriesMoved: NormWireReader<Iso16757ProductSeriesMoved> = normWireObject<Iso16757ProductSeriesMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseIso16757ProductSeriesPatch: NormWireReader<Iso16757ProductSeriesPatch> = normWireObject<Iso16757ProductSeriesPatch>({ classId: normWireRequired(normWireNullable(normWireString)), names: normWireRequired(normWireNullable(parseNames)), sharedPropertyValues: normWireRequired(normWireNullable(normWireMap(parseCatalogueValue))), geometryId: normWireRequired(normWireNullable(normWireRef(() => parseIso16757ProductSeriesPatchGeometryIdValue))) });
export const parseIso16757ProductSeriesPatchGeometryIdValue: NormWireReader<Iso16757ProductSeriesPatchGeometryIdValue> = normWireObject<Iso16757ProductSeriesPatchGeometryIdValue>({ value: normWireRequired(normWireNullable(normWireString)) });
export const parseIso16757ProductSeriesRemoved: NormWireReader<Iso16757ProductSeriesRemoved> = normWireObject<Iso16757ProductSeriesRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseIso16757ProductSeriesRows: NormWireReader<Iso16757ProductSeriesRows> = normWireObject<Iso16757ProductSeriesRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseIso16757ProductSeriesRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseIso16757ProductSeriesInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseIso16757ProductSeriesMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseIso16757ProductSeriesModified))) });
export const parseIso16757ProductsInserted: NormWireReader<Iso16757ProductsInserted> = normWireObject<Iso16757ProductsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseProduct) });
export const parseIso16757ProductsModified: NormWireReader<Iso16757ProductsModified> = normWireObject<Iso16757ProductsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseIso16757ProductsPatch)) });
export const parseIso16757ProductsMoved: NormWireReader<Iso16757ProductsMoved> = normWireObject<Iso16757ProductsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseIso16757ProductsPatch: NormWireReader<Iso16757ProductsPatch> = normWireObject<Iso16757ProductsPatch>({ name: normWireRequired(normWireNullable(normWireString)) });
export const parseIso16757ProductsRemoved: NormWireReader<Iso16757ProductsRemoved> = normWireObject<Iso16757ProductsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseIso16757ProductsRows: NormWireReader<Iso16757ProductsRows> = normWireObject<Iso16757ProductsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseIso16757ProductsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseIso16757ProductsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseIso16757ProductsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseIso16757ProductsModified))) });
export const parseIso16757PropertyDefinitionsInserted: NormWireReader<Iso16757PropertyDefinitionsInserted> = normWireObject<Iso16757PropertyDefinitionsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parsePropertyDefinition) });
export const parseIso16757PropertyDefinitionsModified: NormWireReader<Iso16757PropertyDefinitionsModified> = normWireObject<Iso16757PropertyDefinitionsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseIso16757PropertyDefinitionsPatch)) });
export const parseIso16757PropertyDefinitionsMoved: NormWireReader<Iso16757PropertyDefinitionsMoved> = normWireObject<Iso16757PropertyDefinitionsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseIso16757PropertyDefinitionsPatch: NormWireReader<Iso16757PropertyDefinitionsPatch> = normWireObject<Iso16757PropertyDefinitionsPatch>({ names: normWireRequired(normWireNullable(parseNames)), dataType: normWireRequired(normWireNullable(normWireString)), unit: normWireRequired(normWireNullable(normWireRef(() => parseIso16757PropertyDefinitionsPatchUnitValue))), cardinality: normWireRequired(normWireNullable(parseIso16757Cardinality)), kind: normWireRequired(normWireNullable(parseIso16757PropertyKind)), dictionaryPropertyId: normWireRequired(normWireNullable(normWireRef(() => parseIso16757PropertyDefinitionsPatchDictionaryPropertyIdValue))) });
export const parseIso16757PropertyDefinitionsPatchDictionaryPropertyIdValue: NormWireReader<Iso16757PropertyDefinitionsPatchDictionaryPropertyIdValue> = normWireObject<Iso16757PropertyDefinitionsPatchDictionaryPropertyIdValue>({ value: normWireRequired(normWireNullable(normWireString)) });
export const parseIso16757PropertyDefinitionsPatchUnitValue: NormWireReader<Iso16757PropertyDefinitionsPatchUnitValue> = normWireObject<Iso16757PropertyDefinitionsPatchUnitValue>({ value: normWireRequired(normWireNullable(parseCatalogueUnit)) });
export const parseIso16757PropertyDefinitionsRemoved: NormWireReader<Iso16757PropertyDefinitionsRemoved> = normWireObject<Iso16757PropertyDefinitionsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseIso16757PropertyDefinitionsRows: NormWireReader<Iso16757PropertyDefinitionsRows> = normWireObject<Iso16757PropertyDefinitionsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseIso16757PropertyDefinitionsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseIso16757PropertyDefinitionsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseIso16757PropertyDefinitionsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseIso16757PropertyDefinitionsModified))) });
export const parseIso16757ScriptLimitsPatch: NormWireReader<Iso16757ScriptLimitsPatch> = normWireObject<Iso16757ScriptLimitsPatch>({ maxSteps: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), maxRecursion: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))), timeoutMs: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0}))) });
export const parseIso16757SelectionConstraintsInserted: NormWireReader<Iso16757SelectionConstraintsInserted> = normWireObject<Iso16757SelectionConstraintsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseSelectionConstraint) });
export const parseIso16757SelectionConstraintsRows: NormWireReader<Iso16757SelectionConstraintsRows> = normWireObject<Iso16757SelectionConstraintsRows>({ removed: normWireRequired(normWireArray(normWireRange(normWireInteger, {"minimum":0}))), inserted: normWireRequired(normWireArray(normWireRef(() => parseIso16757SelectionConstraintsInserted))) });
export const parseIso16757SelectionSeriesIdValue: NormWireReader<Iso16757SelectionSeriesIdValue> = normWireObject<Iso16757SelectionSeriesIdValue>({ value: normWireRequired(normWireNullable(normWireString)) });
export const parseIso16757SubjectsInserted: NormWireReader<Iso16757SubjectsInserted> = normWireObject<Iso16757SubjectsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseSubject) });
export const parseIso16757SubjectsModified: NormWireReader<Iso16757SubjectsModified> = normWireObject<Iso16757SubjectsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseIso16757SubjectsPatch)) });
export const parseIso16757SubjectsMoved: NormWireReader<Iso16757SubjectsMoved> = normWireObject<Iso16757SubjectsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseIso16757SubjectsPatch: NormWireReader<Iso16757SubjectsPatch> = normWireObject<Iso16757SubjectsPatch>({ kind: normWireRequired(normWireNullable(parseIso16757SubjectKind)), names: normWireRequired(normWireNullable(parseNames)), definition: normWireRequired(normWireNullable(parseLocalizedText)), parentId: normWireRequired(normWireNullable(normWireRef(() => parseIso16757SubjectsPatchParentIdValue))) });
export const parseIso16757SubjectsPatchParentIdValue: NormWireReader<Iso16757SubjectsPatchParentIdValue> = normWireObject<Iso16757SubjectsPatchParentIdValue>({ value: normWireRequired(normWireNullable(normWireString)) });
export const parseIso16757SubjectsRemoved: NormWireReader<Iso16757SubjectsRemoved> = normWireObject<Iso16757SubjectsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseIso16757SubjectsRows: NormWireReader<Iso16757SubjectsRows> = normWireObject<Iso16757SubjectsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseIso16757SubjectsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseIso16757SubjectsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseIso16757SubjectsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseIso16757SubjectsModified))) });
