
export type SheetAttributes =
  | { kind: "valveHeating"; dn: number; kvsM3S: number; pressureClass: string; connectionType: string; authorityMin: number; authorityMax: number }
  | { kind: "radiator"; standardOutputW: number; heatExponentN: number; lengthM: number; heightM: number; depthM: number; connectionType: string }
  | { kind: "pumpHeating"; dnSuction: number; dnDischarge: number; nominalFlowM3S: number; nominalHeadM: number; motorPowerW: number; hydraulicEfficiency: number; qhCurveRef?: string | null }
  | { kind: "heatGenerator"; nominalHeatOutputW: number; fuelType: string; flowTempMaxC: number; returnTempMinC: number }
  | { kind: "generic" };

/** 🧬️ Vdi3805Mutation — mirrors `Vdi3805Mutation` in `🦀️.rs` (19 variants over the
 * manufacturer-file header, correction/strict-mode/limits scalars, edition profile overrides, and
 * full create/delete(+rename/replace) coverage of catalogue products, parametric geometry and
 * characteristic curves). `Vdi3805Mutation` is EXTERNALLY TAGGED on the wire: `{ "<PascalCaseVariantName>":
 * { ...leaf-struct-fields } }` (e.g. `{"ChangeStrictMode": {"newStrictMode": true}}`). Every leaf struct and
 * every shared value type it embeds (`ManufacturerFile`, `SecurityLimits`, `EditionId`, `BuildingSystemNumber`,
 * `VdiUnit`, `ProductIdentity`, `NativeRecord`, `AccessoryLink`, `CompositionLink`, `Configuration`,
 * `CatalogueProduct`, `BoundingBox`, `ConnectionPoint`, `ParametricGeometry`, `CurvePoint`,
 * `CharacteristicCurve`, `ExtensionBag`) carries `rename_all = "camelCase"` on its value wire and its test serde
 * twin alike, so field names are camelCase (`{"xUnit":{"symbol":"%","kind":"dimensionless","delta":true,
 * "siFactor":0.01}}`), and the unit enums `VdiQuantityKind` and `EditionProfileChoice` spell their variants
 * camelCase too. `VdiValue` is internally tagged on `kind` with camelCase tags (`"boolean"`, `"decimal"`, …);
 * the fields inside each of its variants (`value`, `unit`, `min`, `max`, `code`, `items`) are single words. */

export interface LocalizedText {
  locale: string;
  text: string;
}

export type VdiQuantityKind =
  | "dimensionless"
  | "length"
  | "area"
  | "volume"
  | "mass"
  | "time"
  | "temperature"
  | "force"
  | "pressure"
  | "stress"
  | "moment"
  | "energy"
  | "power"
  | "thermalConductivity"
  | "thermalResistance"
  | "heatTransferCoefficient"
  | "airPermeability"
  | "ventilationRate"
  | "acceleration";

export interface VdiUnit {
  symbol: string;
  kind: VdiQuantityKind;
  delta: boolean;
  siFactor: number;
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
  systemCode: string;
  subsystem: string;
  sequence: number;
}

export interface ManufacturerFile {
  headerVersion: string;
  manufacturer: string;
  buildingSystemNumber: BuildingSystemNumber;
  created: string;
  charset: string;
  recordCount: number;
  extensions: ExtensionBag;
}

export interface SecurityLimits {
  maxFileBytes: number;
  maxRecords: number;
  maxFieldLength: number;
  maxNestingDepth: number;
}

export interface EditionId {
  year: number;
  month: number;
}

export type EditionProfileChoice = "legacy" | "current";

export interface ProductIdentity {
  manufacturerCode: string;
  productGroup: string;
  articleNumber: string;
}

export interface NativeRecord {
  family: string;
  fields: string[];
  extensions: ExtensionBag;
}

export interface AccessoryLink {
  accessoryId: string;
  required: boolean;
  quantity: number;
}

export interface CompositionLink {
  componentId: string;
  quantity: number;
}

export interface Configuration {
  id: string;
  attributes: SheetAttributes;
  geometryRef?: string;
  functionRefs: string[];
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
  minX: number;
  minY: number;
  minZ: number;
  maxX: number;
  maxY: number;
  maxZ: number;
}

export interface ConnectionPoint {
  id: string;
  medium: string;
  position: [number, number, number];
  direction: [number, number, number];
  diameterMm?: number;
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
  xUnit: VdiUnit;
  yUnit: VdiUnit;
  points: CurvePoint[];
}

export interface ChangeManufacturerFile {
  newManufacturerFile: ManufacturerFile;
}

export interface ChangeCorrectionAsOf {
  newCorrectionAsOf: EditionId;
}

export interface ChangeStrictMode {
  newStrictMode: boolean;
}


export interface ChangeEditionProfile {
  sheet: string;
  newChoice: EditionProfileChoice;
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
  newTitle: LocalizedText[];
}

export interface ChangeProductConfiguration {
  id: string;
  newConfiguration: Configuration;
}

export interface AddGeometry {
  geometry: ParametricGeometry;
}

export interface RemoveGeometry {
  id: string;
}

export interface ResizeGeometry {
  id: string;
  newBbox: BoundingBox;
}

export interface AddGeometryConnection {
  id: string;
  connection: ConnectionPoint;
}

export interface RemoveGeometryConnection {
  id: string;
  connectionId: string;
}

export interface ChangeGeometryParameters {
  id: string;
  newParameters: Record<string, number>;
}

export interface AddCurve {
  curve: CharacteristicCurve;
}

export interface RemoveCurve {
  id: string;
}

export interface ChangeCurvePoints {
  id: string;
  newPoints: CurvePoint[];
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
