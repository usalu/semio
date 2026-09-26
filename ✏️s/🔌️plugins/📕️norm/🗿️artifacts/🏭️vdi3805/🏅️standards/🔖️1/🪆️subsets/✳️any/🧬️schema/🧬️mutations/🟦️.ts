
export type SheetAttributes =
  | { kind: "valveHeating"; dn: number; kvsM3S: number; pressureClass: string; connectionType: string; authorityMin: number; authorityMax: number }
  | { kind: "radiator"; standardOutputW: number; heatExponentN: number; lengthM: number; heightM: number; depthM: number; connectionType: string }
  | { kind: "pumpHeating"; dnSuction: number; dnDischarge: number; nominalFlowM3S: number; nominalHeadM: number; motorPowerW: number; hydraulicEfficiency: number; qhCurveRef?: string | null }
  | { kind: "heatGenerator"; nominalHeatOutputW: number; fuelType: string; flowTempMaxC: number; returnTempMinC: number }
  | { kind: "generic" };

/** 🧬️ Vdi3805Mutation — mirrors `Vdi3805Mutation` in `🦀️.rs` (19 variants over the
 * manufacturer-file header, correction/strict-mode/limits scalars, edition profile overrides, and
 * full create/delete(+rename/replace) coverage of catalogue products, parametric geometry and
 * characteristic curves). `Vdi3805Mutation` carries only `#[derive(dsl::Mutations)]` — no
 * `#[serde(tag = ...)]` — so it serializes with serde's default EXTERNALLY TAGGED shape:
 * `{ "<PascalCaseVariantName>": { ...leaf-struct-fields } }`, confirmed by every committed
 * `🧪️tests/*​/🦠️mutation/🔣️.json` fixture (e.g. `{"ChangeStrictMode":
 * {"new_strict_mode":true}}`). None of the 19 leaf structs — nor any of the shared value types they
 * embed (`ManufacturerFile`, `SecurityLimits`, `EditionId`, `BuildingSystemNumber`, `VdiUnit`,
 * `ProductIdentity`, `NativeRecord`, `AccessoryLink`, `CompositionLink`, `Configuration`,
 * `CatalogueProduct`, `BoundingBox`, `ConnectionPoint`, `ParametricGeometry`, `CurvePoint`,
 * `CharacteristicCurve`, `ExtensionBag`) — carry `#[serde(rename_all = ...)]`, so every one of
 * their own field names is the literal Rust snake_case name verbatim (confirmed field-by-field
 * against the committed fixtures, e.g. `{"x_unit":{"symbol":"%","kind":"Dimensionless",
 * "delta":true,"si_factor":0.01}}`). `VdiQuantityKind` (the `VdiUnit.kind` value) has NO
 * `#[serde(rename_all)]` either, so it serializes as the literal PascalCase Rust variant name
 * (`"Dimensionless"`, `"ThermalConductivity"`, …) — the `#[dsl(key = "thermalConductivity")]`
 * annotations on some of its variants are a DSL-engine binding key, not a serde rename, and do NOT
 * affect the JSON wire form. `VdiValue` is the one exception: its enum carries
 * `#[serde(tag = "kind", rename_all = "camelCase")]`, so its OWN tag values are camelCase
 * (`"boolean"`, `"decimal"`, …) — confirmed by fixture (`{"kind":"integer","value":80}`) — while
 * the fields nested inside each of its variants (`value`, `unit`, `min`, `max`, `code`, `items`)
 * are untouched by that container-level `rename_all` (which only renames variant tags, not
 * variant-payload field names) and stay their own literal (already lowercase, single-word) spelling. */

export interface LocalizedText {
  locale: string;
  text: string;
}

export type VdiQuantityKind =
  | "Dimensionless"
  | "Length"
  | "Area"
  | "Volume"
  | "Mass"
  | "Time"
  | "Temperature"
  | "Force"
  | "Pressure"
  | "Stress"
  | "Moment"
  | "Energy"
  | "Power"
  | "ThermalConductivity"
  | "ThermalResistance"
  | "HeatTransferCoefficient"
  | "AirPermeability"
  | "VentilationRate"
  | "Acceleration";

export interface VdiUnit {
  symbol: string;
  kind: VdiQuantityKind;
  delta: boolean;
  si_factor: number;
}

export type VdiValue =
  | { kind: "boolean"; value: boolean }
  | { kind: "integer"; value: number }
  | { kind: "decimal"; value: number; unit?: VdiUnit }
  | { kind: "text"; value: string }
  | { kind: "enumeration"; code: string }
  | { kind: "range"; min: number; max: number; unit?: VdiUnit }
  | { kind: "list"; items: VdiValue[] }
  | { kind: "null" };

export type ExtensionFieldValue = string;
export interface ExtensionBag {
  fields: { readonly [key: string]: ExtensionFieldValue };
}

export interface BuildingSystemNumber {
  system_code: string;
  subsystem: string;
  sequence: number;
}

export interface ManufacturerFile {
  header_version: string;
  manufacturer: string;
  building_system_number: BuildingSystemNumber;
  created: string;
  charset: string;
  record_count: number;
  extensions: ExtensionBag;
}

export interface SecurityLimits {
  max_file_bytes: number;
  max_records: number;
  max_field_length: number;
  max_nesting_depth: number;
}

export interface EditionId {
  year: number;
  month: number;
}

export type EditionProfileChoice = "legacy" | "current";

export interface ProductIdentity {
  manufacturer_code: string;
  product_group: string;
  article_number: string;
}

export interface NativeRecord {
  family: string;
  fields: string[];
  extensions: ExtensionBag;
}

export interface AccessoryLink {
  accessory_id: string;
  required: boolean;
  quantity: number;
}

export interface CompositionLink {
  component_id: string;
  quantity: number;
}

export interface Configuration {
  id: string;
  attributes: SheetAttributes;
  geometry_ref?: string;
  function_refs: string[];
}

export interface CatalogueProduct {
  identity: ProductIdentity;
  title: LocalizedText[];
  sheet: number;
  records: NativeRecord[];
  configuration: Configuration;
  accessories: AccessoryLink[];
  components: CompositionLink[];
  extensions: ExtensionBag;
}

export interface BoundingBox {
  min_x: number;
  min_y: number;
  min_z: number;
  max_x: number;
  max_y: number;
  max_z: number;
}

export interface ConnectionPoint {
  id: string;
  medium: string;
  position: [number, number, number];
  direction: [number, number, number];
  diameter_mm?: number;
}

export interface ParametricGeometry {
  id: string;
  bbox: BoundingBox;
  connections: ConnectionPoint[];
  parameters: Record<string, number>;
}

export interface CurvePoint {
  x: number;
  y: number;
}

export interface CharacteristicCurve {
  id: string;
  x_unit: VdiUnit;
  y_unit: VdiUnit;
  points: CurvePoint[];
}

export interface ChangeManufacturerFile {
  new_manufacturer_file: ManufacturerFile;
}

export interface ChangeCorrectionAsOf {
  new_correction_as_of: EditionId;
}

export interface ChangeStrictMode {
  new_strict_mode: boolean;
}


export interface ChangeEditionProfile {
  sheet: string;
  new_choice: EditionProfileChoice;
}

export interface RemoveEditionProfile {
  sheet: string;
}

export interface AddProduct {
  product: CatalogueProduct;
  index?: number;
}

export interface RemoveProduct {
  id: string;
}

export interface RenameProduct {
  id: string;
  new_title: LocalizedText[];
}

export interface ChangeProductConfiguration {
  id: string;
  new_configuration: Configuration;
}

export interface AddGeometry {
  geometry: ParametricGeometry;
}

export interface RemoveGeometry {
  id: string;
}

export interface ResizeGeometry {
  id: string;
  new_bbox: BoundingBox;
}

export interface AddGeometryConnection {
  id: string;
  connection: ConnectionPoint;
}

export interface RemoveGeometryConnection {
  id: string;
  connection_id: string;
}

export interface ChangeGeometryParameters {
  id: string;
  new_parameters: Record<string, number>;
}

export interface AddCurve {
  curve: CharacteristicCurve;
}

export interface RemoveCurve {
  id: string;
}

export interface ChangeCurvePoints {
  id: string;
  new_points: CurvePoint[];
}

export type Vdi3805Mutation =
  | { ChangeManufacturerFile: ChangeManufacturerFile }
  | { ChangeCorrectionAsOf: ChangeCorrectionAsOf }
  | { ChangeStrictMode: ChangeStrictMode }
  | { ChangeEditionProfile: ChangeEditionProfile }
  | { RemoveEditionProfile: RemoveEditionProfile }
  | { AddProduct: AddProduct }
  | { RemoveProduct: RemoveProduct }
  | { RenameProduct: RenameProduct }
  | { ChangeProductConfiguration: ChangeProductConfiguration }
  | { AddGeometry: AddGeometry }
  | { RemoveGeometry: RemoveGeometry }
  | { ResizeGeometry: ResizeGeometry }
  | { AddGeometryConnection: AddGeometryConnection }
  | { RemoveGeometryConnection: RemoveGeometryConnection }
  | { ChangeGeometryParameters: ChangeGeometryParameters }
  | { AddCurve: AddCurve }
  | { RemoveCurve: RemoveCurve }
  | { ChangeCurvePoints: ChangeCurvePoints };
