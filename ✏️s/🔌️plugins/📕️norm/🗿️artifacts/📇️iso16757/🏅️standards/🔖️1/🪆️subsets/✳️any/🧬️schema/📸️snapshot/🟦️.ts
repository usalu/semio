/** 📸️ `Iso16757Snapshot` wire twin: the persisted snapshot and every record it holds, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormJson, normWireArray, normWireBoolean, normWireDefault, normWireInteger, normWireJson, normWireLiteral, normWireMap, normWireNullable, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString, normWireTagged } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface Iso16757Snapshot {
  catalogue: Catalogue;
  dictionary: Dictionary;
  geometry: GeometryCatalogue;
  selection: SelectionRequest;
  partNumberRule: PartNumberRule;
  partNumberInputs: { [key: string]: CatalogueValue };
  scriptLimits: ScriptLimits;
  exchangeProcess: Iso16757ExchangeProcess;
}

export interface Subject {
  id: string;
  kind: Iso16757SubjectKind;
  names: Names;
  definition: LocalizedText;
  parentId: string | null;
}

export type CatalogueValue = { kind: "boolean"; value: boolean; } | { kind: "integer"; value: number; } | { kind: "decimal"; value: number; } | { kind: "text"; value: string; } | { kind: "identifier"; value: string; } | { kind: "enumeration"; value: string; } | { kind: "controlled"; value: string; list_id: string; } | { kind: "quantity"; value: number; unit: CatalogueUnit; } | ({ kind: "range"; min: number; max: number; unit: CatalogueUnit | null; }) | { kind: "null"; state: Iso16757NullState; } | { kind: "reference"; target_id: string; } | { kind: "list"; items: CatalogueValue[]; };

export interface ProductClass {
  id: string;
  groupId: string;
  parentId: string | null;
  names: Names;
  requiredPropertyIds: string[];
  optionalPropertyIds: string[];
}

export interface GeometryObject {
  id: string;
  shape: GeometryNode | null;
  symbolic: GeometryNode | null;
  spaces: Iso16757SpaceEnvelope[];
  surfaces: Iso16757SurfaceDefinition[];
  ports: Iso16757PortDefinition[];
  parameterBindings: { [key: string]: string };
}

export interface PropertyDefinition {
  id: string;
  names: Names;
  dataType: string;
  unit: CatalogueUnit | null;
  cardinality: Iso16757Cardinality;
  kind: Iso16757PropertyKind;
  dictionaryPropertyId: string | null;
}

export interface ProductSeries {
  id: string;
  classId: string;
  names: Names;
  sharedPropertyValues: { [key: string]: CatalogueValue };
  geometryId: string | null;
}

export interface Product {
  id: string;
  seriesId: string;
  names: Names;
  parameterDomains: ParameterDomain[];
  variants: ProductVariant[];
  staticProperties: PropertyValue[];
}

export type Iso16757ExchangeProcess = "CreateFromDictionary" | "ProvideCatalogue" | "DetermineProduct" | "IntegrateIntoSystem" | "ExchangeSystemModel";

export interface ProductIndex {
  id: string;
  productId: string;
  variantId: string | null;
  searchTags: string[];
}

export interface SelectionConstraint {
  id: string;
  propertyId: string;
  operator: Iso16757ConstraintOperator;
  value: CatalogueValue;
}

export type PartNumberRule = { kind: "literal"; value: string; } | { kind: "table"; rows: { [key: string]: string }[]; output_column: string; } | { kind: "script"; function_id: string; source: string; };

export interface ProductGroup {
  id: string;
  names: Names;
  dictionarySubjectId: string | null;
}

export type Iso16757SubjectKind = "ProductGroup" | "ProductClass" | "ProductSpecialization" | "CatalogueMetadata" | "ManufacturerMetadata" | "PropertyBlock" | "Port" | "Inlet" | "Outlet" | "InOutlet";

export interface Names {
  preferred: LocalizedText;
  shortName: string | null;
  alternatives: LocalizedText[];
}

export interface LocalizedText {
  locale: string;
  text: string;
}

export interface CatalogueUnit {
  symbol: string;
  dimension: DimensionSignature;
  siFactor: number;
}

export type Iso16757NullState = "Unavailable" | "Unknown" | "NotApplicable";

export type GeometryNode = { node: "primitive"; kind: string; parameters: { [key: string]: number }; } | { node: "transform"; translation: number[]; rotation_deg: number[]; child: GeometryNode; } | { node: "boolean"; operator: Iso16757BooleanOperator; children: GeometryNode[]; } | { node: "reference"; geometry_id: string; };

export interface Iso16757SpaceEnvelope {
  id: string;
  kind: Iso16757SpaceKind;
  bounds: BoundingBox;
}

export interface Iso16757SurfaceDefinition {
  id: string;
  purpose: string;
  bounds: BoundingBox;
}

export interface Iso16757PortDefinition {
  id: string;
  medium: string;
  position: number[];
  direction: number[];
  portType: string;
}

export interface Iso16757Cardinality {
  min: number;
  max: number | null;
}

export type Iso16757PropertyKind = "Static" | "Dynamic" | "Selection" | "External";

export interface ParameterDomain {
  parameterId: string;
  allowedValues: CatalogueValue[];
  defaultValue: CatalogueValue | null;
}

export interface ProductVariant {
  id: string;
  parameterValues: { [key: string]: CatalogueValue };
  propertyValues: PropertyValue[];
  articleNumber: string | null;
  geometryId: string | null;
}

export interface PropertyValue {
  definitionId: string;
  value: CatalogueValue;
  functionId: string | null;
}

export type Iso16757ConstraintOperator = "Equal" | "NotEqual" | "LessThan" | "GreaterThan" | "InRange";

export interface DimensionSignature {
  length: number;
  mass: number;
  time: number;
  temperature: number;
}

export type Iso16757BooleanOperator = "Union" | "Intersection" | "Difference";

export type Iso16757SpaceKind = "Overall" | "Operation" | "Access" | "PlacementTransportation" | "Installation";

export interface BoundingBox {
  min: number[];
  max: number[];
}

export interface Catalogue {
  id: string;
  metadata: CatalogueMetadata;
  manufacturer: Manufacturer;
  dictionary: DictionaryRef;
  productGroups: ProductGroup[];
  productClasses: ProductClass[];
  productSeries: ProductSeries[];
  products: Product[];
  productIndexes: ProductIndex[];
  propertyDefinitions: PropertyDefinition[];
  accessories: { [key: string]: AccessoryRelationship[] };
  compositions: { [key: string]: CompositionRelationship[] };
  descriptiveObjects: DescriptiveObject[];
  extensions: ExtensionBag;
}

export interface Dictionary {
  reference: DictionaryRef;
  subjects: Subject[];
  relationships: Relationship[];
  properties: DictionaryProperty[];
  controlledLists: ControlledValueList[];
  metaSubjects: Subject[];
}

export interface GeometryCatalogue {
  objects: { [key: string]: GeometryObject };
  primitiveRegistry: PrimitiveKind[];
}

export interface SelectionRequest {
  classId: string;
  constraints: SelectionConstraint[];
  seriesId: string | null;
}

export interface ScriptLimits {
  maxSteps: number;
  maxRecursion: number;
  timeoutMs: number;
}

export interface CatalogueMetadata {
  names: Names;
  lifecycle: Lifecycle;
  editionProfile: Iso16757EditionProfile;
}

export interface Manufacturer {
  id: string;
  names: Names;
}

export interface DictionaryRef {
  id: string;
  version: string;
}

export interface AccessoryRelationship {
  accessoryProductId: string;
  required: boolean;
  quantity: Iso16757Cardinality;
  compatibilityCondition: string | null;
}

export interface CompositionRelationship {
  componentProductId: string;
  quantity: number;
}

export interface DescriptiveObject {
  id: string;
  mediaType: string;
  uri: string;
  language: string | null;
  checksum: string | null;
}

export interface ExtensionBag {
  fields: { [key: string]: NormJson };
}

export interface Relationship {
  id: string;
  kind: Iso16757RelationshipKind;
  sourceId: string;
  targetId: string;
  cardinality: Iso16757Cardinality;
}

export interface DictionaryProperty {
  id: string;
  names: Names;
  kind: Iso16757PropertyKind;
  dataType: string;
  unit: CatalogueUnit | null;
  applicableSubjectIds: string[];
  valueConstraints: Iso16757ValueConstraint[];
}

export interface ControlledValueList {
  id: string;
  values: string[];
  contextSubjectIds: string[];
}

export interface PrimitiveKind {
  id: string;
  parameters: string[];
}

export interface Lifecycle {
  revision: string;
  status: string;
  validFrom: string | null;
  validTo: string | null;
}

export type Iso16757EditionProfile = "Part1_2015" | "Part2_2016" | "Part4_2025" | "Part5_2025" | "FullPublished";

export type Iso16757RelationshipKind = "IsSubtypeOf" | "HasPart" | "HasBlock" | "IsDependentOn" | "IsSubkindOf";

export interface Iso16757ValueConstraint {
  min: number | null;
  max: number | null;
  allowedValues: string[];
}

export const parseIso16757Snapshot: NormWireReader<Iso16757Snapshot> = normWireObject<Iso16757Snapshot>({ catalogue: normWireRequired(normWireRef(() => parseCatalogue)), dictionary: normWireRequired(normWireRef(() => parseDictionary)), geometry: normWireRequired(normWireRef(() => parseGeometryCatalogue)), selection: normWireRequired(normWireRef(() => parseSelectionRequest)), partNumberRule: normWireRequired(normWireRef(() => parsePartNumberRule)), partNumberInputs: normWireRequired(normWireMap(normWireRef(() => parseCatalogueValue))), scriptLimits: normWireRequired(normWireRef(() => parseScriptLimits)), exchangeProcess: normWireRequired(normWireRef(() => parseIso16757ExchangeProcess)) });
export const parseSubject: NormWireReader<Subject> = normWireObject<Subject>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseIso16757SubjectKind)), names: normWireRequired(normWireRef(() => parseNames)), definition: normWireRequired(normWireRef(() => parseLocalizedText)), parentId: normWireDefault(normWireNullable(normWireString), () => null) });
export const parseCatalogueValue: NormWireReader<CatalogueValue> = normWireTagged<CatalogueValue, "kind">("kind", {
  "boolean": normWireObject<{ kind: "boolean"; value: boolean; }>({ kind: normWireRequired(normWireLiteral("boolean")), value: normWireRequired(normWireBoolean) }),
  "integer": normWireObject<{ kind: "integer"; value: number; }>({ kind: normWireRequired(normWireLiteral("integer")), value: normWireRequired(normWireInteger) }),
  "decimal": normWireObject<{ kind: "decimal"; value: number; }>({ kind: normWireRequired(normWireLiteral("decimal")), value: normWireRequired(normWireNumber) }),
  "text": normWireObject<{ kind: "text"; value: string; }>({ kind: normWireRequired(normWireLiteral("text")), value: normWireRequired(normWireString) }),
  "identifier": normWireObject<{ kind: "identifier"; value: string; }>({ kind: normWireRequired(normWireLiteral("identifier")), value: normWireRequired(normWireString) }),
  "enumeration": normWireObject<{ kind: "enumeration"; value: string; }>({ kind: normWireRequired(normWireLiteral("enumeration")), value: normWireRequired(normWireString) }),
  "controlled": normWireObject<{ kind: "controlled"; value: string; list_id: string; }>({ kind: normWireRequired(normWireLiteral("controlled")), value: normWireRequired(normWireString), list_id: normWireRequired(normWireString) }),
  "quantity": normWireObject<{ kind: "quantity"; value: number; unit: CatalogueUnit; }>({ kind: normWireRequired(normWireLiteral("quantity")), value: normWireRequired(normWireNumber), unit: normWireRequired(normWireRef(() => parseCatalogueUnit)) }),
  "range": normWireObject<{ kind: "range"; min: number; max: number; unit: CatalogueUnit | null; }>({ kind: normWireRequired(normWireLiteral("range")), min: normWireRequired(normWireNumber), max: normWireRequired(normWireNumber), unit: normWireDefault(normWireNullable(normWireRef(() => parseCatalogueUnit)), () => null) }),
  "null": normWireObject<{ kind: "null"; state: Iso16757NullState; }>({ kind: normWireRequired(normWireLiteral("null")), state: normWireRequired(normWireRef(() => parseIso16757NullState)) }),
  "reference": normWireObject<{ kind: "reference"; target_id: string; }>({ kind: normWireRequired(normWireLiteral("reference")), target_id: normWireRequired(normWireString) }),
  "list": normWireObject<{ kind: "list"; items: CatalogueValue[]; }>({ kind: normWireRequired(normWireLiteral("list")), items: normWireRequired(normWireArray(normWireRef(() => parseCatalogueValue))) }),
});
export const parseProductClass: NormWireReader<ProductClass> = normWireObject<ProductClass>({ id: normWireRequired(normWireString), groupId: normWireRequired(normWireString), parentId: normWireDefault(normWireNullable(normWireString), () => null), names: normWireRequired(normWireRef(() => parseNames)), requiredPropertyIds: normWireRequired(normWireArray(normWireString)), optionalPropertyIds: normWireRequired(normWireArray(normWireString)) });
export const parseGeometryObject: NormWireReader<GeometryObject> = normWireObject<GeometryObject>({ id: normWireRequired(normWireString), shape: normWireDefault(normWireNullable(normWireRef(() => parseGeometryNode)), () => null), symbolic: normWireDefault(normWireNullable(normWireRef(() => parseGeometryNode)), () => null), spaces: normWireRequired(normWireArray(normWireRef(() => parseIso16757SpaceEnvelope))), surfaces: normWireRequired(normWireArray(normWireRef(() => parseIso16757SurfaceDefinition))), ports: normWireRequired(normWireArray(normWireRef(() => parseIso16757PortDefinition))), parameterBindings: normWireRequired(normWireMap(normWireString)) });
export const parsePropertyDefinition: NormWireReader<PropertyDefinition> = normWireObject<PropertyDefinition>({ id: normWireRequired(normWireString), names: normWireRequired(normWireRef(() => parseNames)), dataType: normWireRequired(normWireString), unit: normWireDefault(normWireNullable(normWireRef(() => parseCatalogueUnit)), () => null), cardinality: normWireRequired(normWireRef(() => parseIso16757Cardinality)), kind: normWireRequired(normWireRef(() => parseIso16757PropertyKind)), dictionaryPropertyId: normWireDefault(normWireNullable(normWireString), () => null) });
export const parseProductSeries: NormWireReader<ProductSeries> = normWireObject<ProductSeries>({ id: normWireRequired(normWireString), classId: normWireRequired(normWireString), names: normWireRequired(normWireRef(() => parseNames)), sharedPropertyValues: normWireRequired(normWireMap(normWireRef(() => parseCatalogueValue))), geometryId: normWireDefault(normWireNullable(normWireString), () => null) });
export const parseProduct: NormWireReader<Product> = normWireObject<Product>({ id: normWireRequired(normWireString), seriesId: normWireRequired(normWireString), names: normWireRequired(normWireRef(() => parseNames)), parameterDomains: normWireRequired(normWireArray(normWireRef(() => parseParameterDomain))), variants: normWireRequired(normWireArray(normWireRef(() => parseProductVariant))), staticProperties: normWireRequired(normWireArray(normWireRef(() => parsePropertyValue))) });
export const parseIso16757ExchangeProcess: NormWireReader<Iso16757ExchangeProcess> = normWireLiteral("CreateFromDictionary", "ProvideCatalogue", "DetermineProduct", "IntegrateIntoSystem", "ExchangeSystemModel");
export const parseProductIndex: NormWireReader<ProductIndex> = normWireObject<ProductIndex>({ id: normWireRequired(normWireString), productId: normWireRequired(normWireString), variantId: normWireDefault(normWireNullable(normWireString), () => null), searchTags: normWireRequired(normWireArray(normWireString)) });
export const parseSelectionConstraint: NormWireReader<SelectionConstraint> = normWireObject<SelectionConstraint>({ id: normWireRequired(normWireString), propertyId: normWireRequired(normWireString), operator: normWireRequired(normWireRef(() => parseIso16757ConstraintOperator)), value: normWireRequired(normWireRef(() => parseCatalogueValue)) });
export const parsePartNumberRule: NormWireReader<PartNumberRule> = normWireTagged<PartNumberRule, "kind">("kind", {
  "literal": normWireObject<{ kind: "literal"; value: string; }>({ kind: normWireRequired(normWireLiteral("literal")), value: normWireRequired(normWireString) }),
  "table": normWireObject<{ kind: "table"; rows: { [key: string]: string }[]; output_column: string; }>({ kind: normWireRequired(normWireLiteral("table")), rows: normWireRequired(normWireArray(normWireMap(normWireString))), output_column: normWireRequired(normWireString) }),
  "script": normWireObject<{ kind: "script"; function_id: string; source: string; }>({ kind: normWireRequired(normWireLiteral("script")), function_id: normWireRequired(normWireString), source: normWireRequired(normWireString) }),
});
export const parseProductGroup: NormWireReader<ProductGroup> = normWireObject<ProductGroup>({ id: normWireRequired(normWireString), names: normWireRequired(normWireRef(() => parseNames)), dictionarySubjectId: normWireDefault(normWireNullable(normWireString), () => null) });
export const parseIso16757SubjectKind: NormWireReader<Iso16757SubjectKind> = normWireLiteral("ProductGroup", "ProductClass", "ProductSpecialization", "CatalogueMetadata", "ManufacturerMetadata", "PropertyBlock", "Port", "Inlet", "Outlet", "InOutlet");
export const parseNames: NormWireReader<Names> = normWireObject<Names>({ preferred: normWireRequired(normWireRef(() => parseLocalizedText)), shortName: normWireDefault(normWireNullable(normWireString), () => null), alternatives: normWireRequired(normWireArray(normWireRef(() => parseLocalizedText))) });
export const parseLocalizedText: NormWireReader<LocalizedText> = normWireObject<LocalizedText>({ locale: normWireRequired(normWireString), text: normWireRequired(normWireString) });
export const parseCatalogueUnit: NormWireReader<CatalogueUnit> = normWireObject<CatalogueUnit>({ symbol: normWireRequired(normWireString), dimension: normWireRequired(normWireRef(() => parseDimensionSignature)), siFactor: normWireRequired(normWireNumber) });
export const parseIso16757NullState: NormWireReader<Iso16757NullState> = normWireLiteral("Unavailable", "Unknown", "NotApplicable");
export const parseGeometryNode: NormWireReader<GeometryNode> = normWireTagged<GeometryNode, "node">("node", {
  "primitive": normWireObject<{ node: "primitive"; kind: string; parameters: { [key: string]: number }; }>({ node: normWireRequired(normWireLiteral("primitive")), kind: normWireRequired(normWireString), parameters: normWireRequired(normWireMap(normWireNumber)) }),
  "transform": normWireObject<{ node: "transform"; translation: number[]; rotation_deg: number[]; child: GeometryNode; }>({ node: normWireRequired(normWireLiteral("transform")), translation: normWireRequired(normWireArray(normWireNumber, 3, 3)), rotation_deg: normWireRequired(normWireArray(normWireNumber, 3, 3)), child: normWireRequired(normWireRef(() => parseGeometryNode)) }),
  "boolean": normWireObject<{ node: "boolean"; operator: Iso16757BooleanOperator; children: GeometryNode[]; }>({ node: normWireRequired(normWireLiteral("boolean")), operator: normWireRequired(normWireRef(() => parseIso16757BooleanOperator)), children: normWireRequired(normWireArray(normWireRef(() => parseGeometryNode))) }),
  "reference": normWireObject<{ node: "reference"; geometry_id: string; }>({ node: normWireRequired(normWireLiteral("reference")), geometry_id: normWireRequired(normWireString) }),
});
export const parseIso16757SpaceEnvelope: NormWireReader<Iso16757SpaceEnvelope> = normWireObject<Iso16757SpaceEnvelope>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseIso16757SpaceKind)), bounds: normWireRequired(normWireRef(() => parseBoundingBox)) });
export const parseIso16757SurfaceDefinition: NormWireReader<Iso16757SurfaceDefinition> = normWireObject<Iso16757SurfaceDefinition>({ id: normWireRequired(normWireString), purpose: normWireRequired(normWireString), bounds: normWireRequired(normWireRef(() => parseBoundingBox)) });
export const parseIso16757PortDefinition: NormWireReader<Iso16757PortDefinition> = normWireObject<Iso16757PortDefinition>({ id: normWireRequired(normWireString), medium: normWireRequired(normWireString), position: normWireRequired(normWireArray(normWireNumber, 3, 3)), direction: normWireRequired(normWireArray(normWireNumber, 3, 3)), portType: normWireRequired(normWireString) });
export const parseIso16757Cardinality: NormWireReader<Iso16757Cardinality> = normWireObject<Iso16757Cardinality>({ min: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), max: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0})), () => null) });
export const parseIso16757PropertyKind: NormWireReader<Iso16757PropertyKind> = normWireLiteral("Static", "Dynamic", "Selection", "External");
export const parseParameterDomain: NormWireReader<ParameterDomain> = normWireObject<ParameterDomain>({ parameterId: normWireRequired(normWireString), allowedValues: normWireRequired(normWireArray(normWireRef(() => parseCatalogueValue))), defaultValue: normWireDefault(normWireNullable(normWireRef(() => parseCatalogueValue)), () => null) });
export const parseProductVariant: NormWireReader<ProductVariant> = normWireObject<ProductVariant>({ id: normWireRequired(normWireString), parameterValues: normWireRequired(normWireMap(normWireRef(() => parseCatalogueValue))), propertyValues: normWireRequired(normWireArray(normWireRef(() => parsePropertyValue))), articleNumber: normWireDefault(normWireNullable(normWireString), () => null), geometryId: normWireDefault(normWireNullable(normWireString), () => null) });
export const parsePropertyValue: NormWireReader<PropertyValue> = normWireObject<PropertyValue>({ definitionId: normWireRequired(normWireString), value: normWireRequired(normWireRef(() => parseCatalogueValue)), functionId: normWireDefault(normWireNullable(normWireString), () => null) });
export const parseIso16757ConstraintOperator: NormWireReader<Iso16757ConstraintOperator> = normWireLiteral("Equal", "NotEqual", "LessThan", "GreaterThan", "InRange");
export const parseDimensionSignature: NormWireReader<DimensionSignature> = normWireObject<DimensionSignature>({ length: normWireRequired(normWireInteger), mass: normWireRequired(normWireInteger), time: normWireRequired(normWireInteger), temperature: normWireRequired(normWireInteger) });
export const parseIso16757BooleanOperator: NormWireReader<Iso16757BooleanOperator> = normWireLiteral("Union", "Intersection", "Difference");
export const parseIso16757SpaceKind: NormWireReader<Iso16757SpaceKind> = normWireLiteral("Overall", "Operation", "Access", "PlacementTransportation", "Installation");
export const parseBoundingBox: NormWireReader<BoundingBox> = normWireObject<BoundingBox>({ min: normWireRequired(normWireArray(normWireNumber, 3, 3)), max: normWireRequired(normWireArray(normWireNumber, 3, 3)) });
export const parseCatalogue: NormWireReader<Catalogue> = normWireObject<Catalogue>({ id: normWireRequired(normWireString), metadata: normWireRequired(normWireRef(() => parseCatalogueMetadata)), manufacturer: normWireRequired(normWireRef(() => parseManufacturer)), dictionary: normWireRequired(normWireRef(() => parseDictionaryRef)), productGroups: normWireRequired(normWireArray(normWireRef(() => parseProductGroup))), productClasses: normWireRequired(normWireArray(normWireRef(() => parseProductClass))), productSeries: normWireRequired(normWireArray(normWireRef(() => parseProductSeries))), products: normWireRequired(normWireArray(normWireRef(() => parseProduct))), productIndexes: normWireRequired(normWireArray(normWireRef(() => parseProductIndex))), propertyDefinitions: normWireRequired(normWireArray(normWireRef(() => parsePropertyDefinition))), accessories: normWireRequired(normWireMap(normWireArray(normWireRef(() => parseAccessoryRelationship)))), compositions: normWireRequired(normWireMap(normWireArray(normWireRef(() => parseCompositionRelationship)))), descriptiveObjects: normWireRequired(normWireArray(normWireRef(() => parseDescriptiveObject))), extensions: normWireRequired(normWireRef(() => parseExtensionBag)) });
export const parseDictionary: NormWireReader<Dictionary> = normWireObject<Dictionary>({ reference: normWireRequired(normWireRef(() => parseDictionaryRef)), subjects: normWireRequired(normWireArray(normWireRef(() => parseSubject))), relationships: normWireRequired(normWireArray(normWireRef(() => parseRelationship))), properties: normWireRequired(normWireArray(normWireRef(() => parseDictionaryProperty))), controlledLists: normWireRequired(normWireArray(normWireRef(() => parseControlledValueList))), metaSubjects: normWireRequired(normWireArray(normWireRef(() => parseSubject))) });
export const parseGeometryCatalogue: NormWireReader<GeometryCatalogue> = normWireObject<GeometryCatalogue>({ objects: normWireRequired(normWireMap(normWireRef(() => parseGeometryObject))), primitiveRegistry: normWireRequired(normWireArray(normWireRef(() => parsePrimitiveKind))) });
export const parseSelectionRequest: NormWireReader<SelectionRequest> = normWireObject<SelectionRequest>({ classId: normWireRequired(normWireString), constraints: normWireRequired(normWireArray(normWireRef(() => parseSelectionConstraint))), seriesId: normWireDefault(normWireNullable(normWireString), () => null) });
export const parseScriptLimits: NormWireReader<ScriptLimits> = normWireObject<ScriptLimits>({ maxSteps: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), maxRecursion: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), timeoutMs: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCatalogueMetadata: NormWireReader<CatalogueMetadata> = normWireObject<CatalogueMetadata>({ names: normWireRequired(normWireRef(() => parseNames)), lifecycle: normWireRequired(normWireRef(() => parseLifecycle)), editionProfile: normWireRequired(normWireRef(() => parseIso16757EditionProfile)) });
export const parseManufacturer: NormWireReader<Manufacturer> = normWireObject<Manufacturer>({ id: normWireRequired(normWireString), names: normWireRequired(normWireRef(() => parseNames)) });
export const parseDictionaryRef: NormWireReader<DictionaryRef> = normWireObject<DictionaryRef>({ id: normWireRequired(normWireString), version: normWireRequired(normWireString) });
export const parseAccessoryRelationship: NormWireReader<AccessoryRelationship> = normWireObject<AccessoryRelationship>({ accessoryProductId: normWireRequired(normWireString), required: normWireRequired(normWireBoolean), quantity: normWireRequired(normWireRef(() => parseIso16757Cardinality)), compatibilityCondition: normWireDefault(normWireNullable(normWireString), () => null) });
export const parseCompositionRelationship: NormWireReader<CompositionRelationship> = normWireObject<CompositionRelationship>({ componentProductId: normWireRequired(normWireString), quantity: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseDescriptiveObject: NormWireReader<DescriptiveObject> = normWireObject<DescriptiveObject>({ id: normWireRequired(normWireString), mediaType: normWireRequired(normWireString), uri: normWireRequired(normWireString), language: normWireDefault(normWireNullable(normWireString), () => null), checksum: normWireDefault(normWireNullable(normWireString), () => null) });
export const parseExtensionBag: NormWireReader<ExtensionBag> = normWireObject<ExtensionBag>({ fields: normWireRequired(normWireMap(normWireJson)) });
export const parseRelationship: NormWireReader<Relationship> = normWireObject<Relationship>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseIso16757RelationshipKind)), sourceId: normWireRequired(normWireString), targetId: normWireRequired(normWireString), cardinality: normWireRequired(normWireRef(() => parseIso16757Cardinality)) });
export const parseDictionaryProperty: NormWireReader<DictionaryProperty> = normWireObject<DictionaryProperty>({ id: normWireRequired(normWireString), names: normWireRequired(normWireRef(() => parseNames)), kind: normWireRequired(normWireRef(() => parseIso16757PropertyKind)), dataType: normWireRequired(normWireString), unit: normWireDefault(normWireNullable(normWireRef(() => parseCatalogueUnit)), () => null), applicableSubjectIds: normWireRequired(normWireArray(normWireString)), valueConstraints: normWireRequired(normWireArray(normWireRef(() => parseIso16757ValueConstraint))) });
export const parseControlledValueList: NormWireReader<ControlledValueList> = normWireObject<ControlledValueList>({ id: normWireRequired(normWireString), values: normWireRequired(normWireArray(normWireString)), contextSubjectIds: normWireRequired(normWireArray(normWireString)) });
export const parsePrimitiveKind: NormWireReader<PrimitiveKind> = normWireObject<PrimitiveKind>({ id: normWireRequired(normWireString), parameters: normWireRequired(normWireArray(normWireString)) });
export const parseLifecycle: NormWireReader<Lifecycle> = normWireObject<Lifecycle>({ revision: normWireRequired(normWireString), status: normWireRequired(normWireString), validFrom: normWireDefault(normWireNullable(normWireString), () => null), validTo: normWireDefault(normWireNullable(normWireString), () => null) });
export const parseIso16757EditionProfile: NormWireReader<Iso16757EditionProfile> = normWireLiteral("Part1_2015", "Part2_2016", "Part4_2025", "Part5_2025", "FullPublished");
export const parseIso16757RelationshipKind: NormWireReader<Iso16757RelationshipKind> = normWireLiteral("IsSubtypeOf", "HasPart", "HasBlock", "IsDependentOn", "IsSubkindOf");
export const parseIso16757ValueConstraint: NormWireReader<Iso16757ValueConstraint> = normWireObject<Iso16757ValueConstraint>({ min: normWireDefault(normWireNullable(normWireNumber), () => null), max: normWireDefault(normWireNullable(normWireNumber), () => null), allowedValues: normWireRequired(normWireArray(normWireString)) });
