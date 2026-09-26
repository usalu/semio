/** 🧬️ Iso16757Mutation — mirrors `Iso16757Mutation` in `🦀️.rs` (21 variants over
 * document-root scalars, catalogue/manufacturer naming, and full create/delete(+rename) coverage
 * of `product_groups`/`products`/`property_definitions`/dictionary `subjects`). `Iso16757Mutation`
 * carries only `#[derive(dsl::Mutations)]` — no `#[serde(tag = ...)]` — so it serializes with
 * serde's default EXTERNALLY TAGGED shape: `{ "<PascalCaseVariantName>": { ...leaf-struct-fields }
 * }`, confirmed by every committed `🧪️tests/*​/🦠️mutation/🔣️.json` fixture (e.g.
 * `{"ChangeExchangeProcess": {"new_exchange_process": "DetermineProduct"}}`). None of the 21 leaf
 * structs carry `#[serde(rename_all = ...)]`, so every leaf's own field names are the literal Rust
 * snake_case names verbatim. */

export interface LocalizedText {
  locale: string;
  text: string;
}

export interface Names {
  preferred: LocalizedText;
  shortName?: string;
  alternatives: LocalizedText[];
}

export interface DimensionSignature {
  length: number;
  mass: number;
  time: number;
  temperature: number;
}

export interface CatalogueUnit {
  symbol: string;
  dimension: DimensionSignature;
  siFactor: number;
}

export type NullState = "unavailable" | "unknown" | "notApplicable";

export type CatalogueValue =
  | { kind: "boolean"; value: boolean }
  | { kind: "integer"; value: number }
  | { kind: "decimal"; value: number }
  | { kind: "text"; value: string }
  | { kind: "identifier"; value: string }
  | { kind: "enumeration"; value: string }
  | { kind: "controlled"; value: string; listId: string }
  | { kind: "quantity"; value: number; unit: CatalogueUnit }
  | { kind: "range"; min: number; max: number; unit?: CatalogueUnit }
  | { kind: "null"; state: NullState }
  | { kind: "reference"; targetId: string }
  | { kind: "list"; items: CatalogueValue[] };

export interface Cardinality {
  min: number;
  max?: number;
}

export type PropertyKind = "static" | "dynamic" | "selection" | "external";

export type SubjectKind =
  | "productGroup"
  | "productClass"
  | "productSpecialization"
  | "catalogueMetadata"
  | "manufacturerMetadata"
  | "propertyBlock"
  | "port"
  | "inlet"
  | "outlet"
  | "inOutlet";

export interface Subject {
  id: string;
  kind: SubjectKind;
  names: Names;
  definition: LocalizedText;
  parentId?: string;
}

export interface ProductGroup {
  id: string;
  names: Names;
  dictionarySubjectId?: string;
}

export interface ProductClass {
  id: string;
  groupId: string;
  parentId?: string;
  names: Names;
  requiredPropertyIds: string[];
  optionalPropertyIds: string[];
}

export interface ProductSeries {
  id: string;
  classId: string;
  names: Names;
  sharedPropertyValues: Record<string, CatalogueValue>;
  geometryId?: string;
}

export interface ProductIndex {
  id: string;
  productId: string;
  variantId?: string;
  searchTags: string[];
}

export type BoundingBox = {
  min: [number, number, number];
  max: [number, number, number];
};

/** 📦️ Part 2 space envelope. */
export type Space = {
  id: string;
  kind: string;
  bounds: BoundingBox;
};

/** 🎨️ Part 2 semantic surface. */
export type Surface = {
  id: string;
  purpose: string;
  bounds: BoundingBox;
};

/** 🔌️ Part 2 port definition. */
export type Port = {
  id: string;
  medium: string;
  position: [number, number, number];
  direction: [number, number, number];
  portType: string;
};

export type GeometryNode =
  | { node: "primitive"; kind: string; parameters: Record<string, number> }
  | { node: "transform"; translation: number[]; rotationDeg: number[]; child: GeometryNode }
  | { node: "boolean"; operator: string; children: GeometryNode[] }
  | { node: "reference"; geometryId: string };

export interface GeometryObject {
  id: string;
  shape?: GeometryNode;
  symbolic?: GeometryNode;
  spaces: Space[];
  surfaces: Surface[];
  ports: Port[];
  parameterBindings: Record<string, string>;
}

export interface PropertyDefinition {
  id: string;
  names: Names;
  dataType: string;
  unit?: CatalogueUnit;
  cardinality: Cardinality;
  kind: PropertyKind;
  dictionaryPropertyId?: string;
}

export interface ParameterDomain {
  parameterId: string;
  allowedValues: CatalogueValue[];
  defaultValue?: CatalogueValue;
}

export interface PropertyValue {
  definitionId: string;
  value: CatalogueValue;
  functionId?: string;
}

export interface ProductVariant {
  id: string;
  parameterValues: Record<string, CatalogueValue>;
  propertyValues: PropertyValue[];
  articleNumber?: string;
  geometryId?: string;
}

export interface Product {
  id: string;
  seriesId: string;
  names: Names;
  parameterDomains: ParameterDomain[];
  variants: ProductVariant[];
  staticProperties: PropertyValue[];
}

export type ExchangeProcess = "createFromDictionary" | "provideCatalogue" | "determineProduct" | "integrateIntoSystem" | "exchangeSystemModel";

export type PartNumberRule =
  | { kind: "literal"; value: string }
  | { kind: "table"; rows: Record<string, string>[]; outputColumn: string }
  | { kind: "script"; functionId: string; source: string };

export type ConstraintOperator = "equal" | "notEqual" | "lessThan" | "greaterThan" | "inRange";

export interface SelectionConstraint {
  propertyId: string;
  operator: ConstraintOperator;
  value: CatalogueValue;
}

export interface ChangeExchangeProcess {
  new_exchange_process: ExchangeProcess;
}

export interface ChangeScriptLimits {
  new_max_steps: number;
  new_max_recursion: number;
  new_timeout_ms: number;
}

export interface ReplacePartNumberRule {
  new_rule: PartNumberRule;
}

export interface ChangePartNumberInput {
  key: string;
  new_value: CatalogueValue;
}

export interface RemovePartNumberInput {
  key: string;
}

export interface ChangeSelectionClass {
  new_class_id: string;
}

export interface ChangeSelectionSeries {
  new_series_id?: string;
}

export interface AddSelectionConstraint {
  constraint: SelectionConstraint;
}

export interface RemoveSelectionConstraint {
  index: number;
}

export interface RenameCatalogue {
  new_name: string;
}

export interface RenameManufacturer {
  new_name: string;
}

export interface IntroduceProductGroup {
  product_group: ProductGroup;
  index?: number;
}

export interface RetireProductGroup {
  id: string;
}

export interface RenameProductGroup {
  id: string;
  new_name: string;
}

export interface IntroduceProduct {
  product: Product;
  index?: number;
}

export interface RetireProduct {
  id: string;
}

export interface RenameProduct {
  id: string;
  new_name: string;
}

export interface IntroducePropertyDefinition {
  property_definition: PropertyDefinition;
  index?: number;
}

export interface RetirePropertyDefinition {
  id: string;
}

export interface IntroduceSubject {
  subject: Subject;
  index?: number;
}

export interface RetireSubject {
  id: string;
}

export interface IntroduceProductClass {
  product_class: ProductClass;
  index?: number;
}

export interface RetireProductClass {
  id: string;
}

export interface IntroduceProductSeries {
  product_series: ProductSeries;
  index?: number;
}

export interface RetireProductSeries {
  id: string;
}

export interface IntroduceProductIndex {
  product_index: ProductIndex;
  index?: number;
}

export interface RetireProductIndex {
  id: string;
}

export interface IntroduceGeometryObject {
  geometry_object: GeometryObject;
}

export interface RetireGeometryObject {
  id: string;
}

export type Iso16757Mutation =
  | { ChangeExchangeProcess: ChangeExchangeProcess }
  | { ChangeScriptLimits: ChangeScriptLimits }
  | { ReplacePartNumberRule: ReplacePartNumberRule }
  | { ChangePartNumberInput: ChangePartNumberInput }
  | { RemovePartNumberInput: RemovePartNumberInput }
  | { ChangeSelectionClass: ChangeSelectionClass }
  | { ChangeSelectionSeries: ChangeSelectionSeries }
  | { AddSelectionConstraint: AddSelectionConstraint }
  | { RemoveSelectionConstraint: RemoveSelectionConstraint }
  | { RenameCatalogue: RenameCatalogue }
  | { RenameManufacturer: RenameManufacturer }
  | { IntroduceProductGroup: IntroduceProductGroup }
  | { RetireProductGroup: RetireProductGroup }
  | { RenameProductGroup: RenameProductGroup }
  | { IntroduceProduct: IntroduceProduct }
  | { RetireProduct: RetireProduct }
  | { RenameProduct: RenameProduct }
  | { IntroducePropertyDefinition: IntroducePropertyDefinition }
  | { RetirePropertyDefinition: RetirePropertyDefinition }
  | { IntroduceSubject: IntroduceSubject }
  | { RetireSubject: RetireSubject }
  | { IntroduceProductClass: IntroduceProductClass }
  | { RetireProductClass: RetireProductClass }
  | { IntroduceProductSeries: IntroduceProductSeries }
  | { RetireProductSeries: RetireProductSeries }
  | { IntroduceProductIndex: IntroduceProductIndex }
  | { RetireProductIndex: RetireProductIndex }
  | { IntroduceGeometryObject: IntroduceGeometryObject }
  | { RetireGeometryObject: RetireGeometryObject };
