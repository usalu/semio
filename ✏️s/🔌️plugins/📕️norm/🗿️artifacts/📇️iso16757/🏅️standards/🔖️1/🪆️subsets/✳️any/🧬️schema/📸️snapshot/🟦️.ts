/** 🧬️ Iso16757 snapshot schema — mirrors Rust `Iso16757Snapshot` (serde test JSON). */

export type LocalizedText = {
  locale: string;
  text: string;
};

export type Names = {
  preferred: LocalizedText;
  shortName?: string | null;
  alternatives: LocalizedText[];
};

export type DimensionSignature = {
  length: number;
  mass: number;
  time: number;
  temperature: number;
};

export type CatalogueUnit = {
  symbol: string;
  dimension: DimensionSignature;
  siFactor: number;
};

export type CatalogueValue =
  | { kind: "boolean"; value: boolean }
  | { kind: "integer"; value: number }
  | { kind: "decimal"; value: number }
  | { kind: "text"; value: string }
  | { kind: "identifier"; value: string }
  | { kind: "enumeration"; value: string }
  | { kind: "controlled"; value: string; list_id: string }
  | { kind: "quantity"; value: number; unit: CatalogueUnit }
  | { kind: "range"; min: number; max: number; unit?: CatalogueUnit }
  | { kind: "null"; state: string }
  | { kind: "reference"; target_id: string }
  | { kind: "list"; items: CatalogueValue[] };

export type Lifecycle = {
  revision: string;
  status: string;
  validFrom?: string | null;
  validTo?: string | null;
};

export type CatalogueMetadata = {
  names: Names;
  lifecycle: Lifecycle;
  editionProfile: string;
};

export type Manufacturer = {
  id: string;
  names: Names;
};

export type DictionaryRef = {
  id: string;
  version: string;
};

export type ProductGroup = {
  id: string;
  names: Names;
  dictionarySubjectId?: string | null;
};

export type ProductClass = {
  id: string;
  groupId: string;
  parentId?: string | null;
  names: Names;
  requiredPropertyIds: string[];
  optionalPropertyIds: string[];
};

export type ProductSeries = {
  id: string;
  classId: string;
  names: Names;
  sharedPropertyValues: Record<string, CatalogueValue>;
  geometryId?: string | null;
};

export type ParameterDomain = {
  parameterId: string;
  allowedValues: CatalogueValue[];
  defaultValue?: CatalogueValue;
};

export type PropertyValue = {
  definitionId: string;
  value: CatalogueValue;
  functionId?: string | null;
};

export type ProductVariant = {
  id: string;
  parameterValues: Record<string, CatalogueValue>;
  propertyValues: PropertyValue[];
  articleNumber?: string | null;
  geometryId?: string | null;
};

export type Product = {
  id: string;
  seriesId: string;
  names: Names;
  parameterDomains: ParameterDomain[];
  variants: ProductVariant[];
  staticProperties: PropertyValue[];
};

export type ProductIndex = {
  id: string;
  productId: string;
  variantId?: string | null;
  searchTags: string[];
};

export type PropertyDefinition = {
  id: string;
  names: Names;
  dataType: string;
  unit?: CatalogueUnit | null;
  cardinality: { min: number; max?: number | null };
  kind: string;
  dictionaryPropertyId?: string | null;
};

export type AccessoryRelationship = {
  accessoryProductId: string;
  quantity?: number;
};

export type CompositionRelationship = {
  componentProductId: string;
  quantity: number;
};

export type Catalogue = {
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
  accessories: Record<string, AccessoryRelationship[]>;
  compositions: Record<string, CompositionRelationship[]>;
  descriptiveObjects: DescriptiveObject[];
  extensions: ExtensionBag;
};

export type DescriptiveObject = {
  id: string;
  mediaType: string;
  uri: string;
  language?: string | null;
  checksum?: string | null;
};

export type ExtensionBag = {
  fields: Record<string, CatalogueValue | string | number | boolean | null>;
};

export type Subject = {
  id: string;
  kind: string;
  names: Names;
  definition: LocalizedText;
  parentId?: string | null;
};

export type Relationship = {
  id?: string;
  kind: string;
  fromId: string;
  toId: string;
};

export type DictionaryProperty = {
  id: string;
  names: Names;
  dataType?: string;
  unit?: CatalogueUnit | null;
};

export type ControlledValueList = {
  id: string;
  values: string[];
};

export type Dictionary = {
  reference: DictionaryRef;
  subjects: Subject[];
  relationships: Relationship[];
  properties: DictionaryProperty[];
  controlledLists: ControlledValueList[];
  metaSubjects: Subject[];
};

export type BoundingBox = {
  min: [number, number, number];
  max: [number, number, number];
};

/** 📦️ Part 2 space envelope (`SpaceEnvelope` in Rust). */
export type Space = {
  id: string;
  kind: string;
  bounds: BoundingBox;
};

/** 🎨️ Part 2 surface (`SurfaceDefinition` in Rust). */
export type Surface = {
  id: string;
  purpose: string;
  bounds: BoundingBox;
};

/** 🔌️ Part 2 port (`PortDefinition` in Rust). */
export type Port = {
  id: string;
  medium: string;
  position: [number, number, number];
  direction: [number, number, number];
  port_type: string;
};

export type GeometryNode =
  | { node: "primitive"; kind: string; parameters: Record<string, number> }
  | { node: "transform"; translation: number[]; rotation_deg: number[]; child: GeometryNode }
  | { node: "boolean"; operator: string; children: GeometryNode[] }
  | { node: "reference"; geometry_id: string };

export type GeometryObject = {
  id: string;
  shape?: GeometryNode | null;
  symbolic?: GeometryNode | null;
  spaces: Space[];
  surfaces: Surface[];
  ports: Port[];
  parameterBindings: Record<string, string>;
};

export type PrimitiveKind = {
  id: string;
  parameters: string[];
};

export type GeometryCatalogue = {
  objects: Record<string, GeometryObject>;
  primitiveRegistry: PrimitiveKind[];
};

export type SelectionConstraint = {
  id?: string;
  propertyId: string;
  operator: string;
  value: CatalogueValue;
};

export type SelectionRequest = {
  classId: string;
  constraints: SelectionConstraint[];
  seriesId?: string | null;
};

export type PartNumberRule =
  | { kind: "literal"; value: string }
  | { kind: "table"; rows: Record<string, string>[]; output_column: string }
  | { kind: "script"; function_id: string; source: string };

export type ScriptLimits = {
  maxSteps: number;
  maxRecursion: number;
  timeoutMs: number;
};

export interface Iso16757Snapshot {
  /** @state artifact */
  catalogue: Catalogue;
  /** @state artifact */
  dictionary: Dictionary;
  /** @state artifact */
  geometry: GeometryCatalogue;
  /** @state artifact */
  selection: SelectionRequest;
  /** @state artifact */
  partNumberRule: PartNumberRule;
  /** @state artifact */
  partNumberInputs: Record<string, CatalogueValue>;
  /** @state artifact */
  scriptLimits: ScriptLimits;
  /** @state artifact */
  exchangeProcess: string;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normIso16757SnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normIso16757SnapshotGuardReject = (at: string, why: string): never => {
  throw new normIso16757SnapshotGuardRefusal(at, why);
};

type normIso16757SnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normIso16757SnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normIso16757SnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normIso16757SnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normIso16757SnapshotGuardReject(at, "value is not an object");
export const normIso16757SnapshotGuardArray = (value: unknown, at: string, bounds: normIso16757SnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normIso16757SnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normIso16757SnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normIso16757SnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normIso16757SnapshotGuardString = (value: unknown, at: string, bounds: normIso16757SnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normIso16757SnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normIso16757SnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normIso16757SnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normIso16757SnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normIso16757SnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normIso16757SnapshotGuardReject(at, "value is not a boolean"));
export const normIso16757SnapshotGuardNumber = (value: unknown, at: string, bounds: normIso16757SnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normIso16757SnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normIso16757SnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normIso16757SnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normIso16757SnapshotGuardInteger = (value: unknown, at: string, bounds: normIso16757SnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normIso16757SnapshotGuardNumber(value, at, bounds) : normIso16757SnapshotGuardReject(at, "value is not an integer");
export const normIso16757SnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normIso16757SnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normIso16757SnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normIso16757SnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseIso16757Snapshot(value: unknown, at = "$"): Iso16757Snapshot {
  const row = normIso16757SnapshotGuardObject(value, at);
  return {
    catalogue: normIso16757SnapshotGuardObject(row["catalogue"], `${at}.catalogue`) as Catalogue,
    dictionary: normIso16757SnapshotGuardObject(row["dictionary"], `${at}.dictionary`) as Dictionary,
    geometry: normIso16757SnapshotGuardObject(row["geometry"], `${at}.geometry`) as GeometryCatalogue,
    selection: normIso16757SnapshotGuardObject(row["selection"], `${at}.selection`) as SelectionRequest,
    partNumberRule: normIso16757SnapshotGuardObject(row["partNumberRule"], `${at}.partNumberRule`) as PartNumberRule,
    partNumberInputs: normIso16757SnapshotGuardObject(row["partNumberInputs"], `${at}.partNumberInputs`) as Record<string, CatalogueValue>,
    scriptLimits: normIso16757SnapshotGuardObject(row["scriptLimits"], `${at}.scriptLimits`) as ScriptLimits,
    exchangeProcess: normIso16757SnapshotGuardText(row["exchangeProcess"], `${at}.exchangeProcess`) as string,
  };
}
