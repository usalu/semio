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

export interface UpdateScriptLimits {
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

export interface CreateProductGroup {
  product_group: ProductGroup;
  index?: number;
}

export interface DeleteProductGroup {
  id: string;
}

export interface RenameProductGroup {
  id: string;
  new_name: string;
}

export interface CreateProduct {
  product: Product;
  index?: number;
}

export interface DeleteProduct {
  id: string;
}

export interface RenameProduct {
  id: string;
  new_name: string;
}

export interface CreatePropertyDefinition {
  property_definition: PropertyDefinition;
  index?: number;
}

export interface DeletePropertyDefinition {
  id: string;
}

export interface CreateSubject {
  subject: Subject;
  index?: number;
}

export interface DeleteSubject {
  id: string;
}

export type Iso16757Mutation =
  | { ChangeExchangeProcess: ChangeExchangeProcess }
  | { UpdateScriptLimits: UpdateScriptLimits }
  | { ReplacePartNumberRule: ReplacePartNumberRule }
  | { ChangePartNumberInput: ChangePartNumberInput }
  | { RemovePartNumberInput: RemovePartNumberInput }
  | { ChangeSelectionClass: ChangeSelectionClass }
  | { ChangeSelectionSeries: ChangeSelectionSeries }
  | { AddSelectionConstraint: AddSelectionConstraint }
  | { RemoveSelectionConstraint: RemoveSelectionConstraint }
  | { RenameCatalogue: RenameCatalogue }
  | { RenameManufacturer: RenameManufacturer }
  | { CreateProductGroup: CreateProductGroup }
  | { DeleteProductGroup: DeleteProductGroup }
  | { RenameProductGroup: RenameProductGroup }
  | { CreateProduct: CreateProduct }
  | { DeleteProduct: DeleteProduct }
  | { RenameProduct: RenameProduct }
  | { CreatePropertyDefinition: CreatePropertyDefinition }
  | { DeletePropertyDefinition: DeletePropertyDefinition }
  | { CreateSubject: CreateSubject }
  | { DeleteSubject: DeleteSubject };
