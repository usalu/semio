/** 📸️ `Vdi3805Snapshot` wire twin: the persisted snapshot and every record it holds, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireDefault, normWireInteger, normWireLiteral, normWireMap, normWireNullable, normWireNumber, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString, normWireTagged } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface Vdi3805Snapshot {
  /** @state artifact */
  catalog: ManufacturerCatalog;
  /** @state artifact */
  editionProfile: { [key: string]: "legacy" | "current" };
  /** @state artifact */
  correctionAsOf: EditionId;
  /** @state artifact */
  strictMode: boolean;
  /** @state artifact */
  index: CatalogIndex;
  /** @state artifact */
  geometry: { [key: string]: ParametricGeometry };
  /** @state artifact */
  curves: { [key: string]: CharacteristicCurve };
  limits: SecurityLimits;
}

export interface LocalizedText {
  locale: string;
  text: string;
}

export interface BuildingSystemNumber {
  systemCode: string;
  subsystem: string;
  sequence: number;
}

export interface ExtensionFields {
  fields: { [key: string]: string };
}

export interface ManufacturerFile {
  headerVersion: string;
  manufacturer: string;
  buildingSystemNumber: BuildingSystemNumber;
  created: string;
  charset: string;
  recordCount: number;
  extensions: Vdi3805ExtensionBag;
}

export interface NativeRecord {
  family: string;
  fields: string[];
  extensions: Vdi3805ExtensionBag;
}

export type SheetAttributes = { kind: "valveHeating"; dn: number; kvsM3S: number; pressureClass: string; connectionType: string; authorityMin: number; authorityMax: number; } | { kind: "radiator"; standardOutputW: number; heatExponentN: number; lengthM: number; heightM: number; depthM: number; connectionType: string; } | ({ kind: "pumpHeating"; dnSuction: number; dnDischarge: number; nominalFlowM3S: number; nominalHeadM: number; motorPowerW: number; hydraulicEfficiency: number; qhCurveRef: string | null; }) | { kind: "heatGenerator"; nominalHeatOutputW: number; fuelType: string; flowTempMaxC: number; returnTempMinC: number; } | { kind: "generic"; entries?: Vdi3805GenericAttribute[]; };

export interface Configuration {
  id: string;
  attributes: SheetAttributes;
  geometryRef: string | null;
  functionRefs: string[];
}

export interface ProductIdentity {
  manufacturerCode: string;
  productGroup: string;
  articleNumber: string;
}

export interface Product {
  id: string;
  identity: ProductIdentity;
  title: LocalizedText[];
  sheet: number;
  records: NativeRecord[];
  configuration: Configuration;
  accessories: AccessoryLink[];
  components: CompositionLink[];
  extensions: ExtensionFields;
}

export interface ManufacturerCatalog {
  file: ManufacturerFile;
  products: Product[];
  extensions: ExtensionFields;
}

export interface EditionId {
  year: number;
  month: number;
}

export interface CatalogIndexEntry {
  productId: string;
  sheet: number;
  tags: string[];
  dn: number | null;
}

export interface CatalogIndex {
  entries: CatalogIndexEntry[];
}

export interface BoundingBox {
  minX: number;
  minY: number;
  minZ: number;
  maxX: number;
  maxY: number;
  maxZ: number;
}

export interface GeometryConnection {
  id: string;
  medium: string;
  position: number[];
  direction: number[];
  diameterMm: number | null;
}

export interface ParametricGeometry {
  id: string;
  bbox: BoundingBox;
  connections: Vdi3805ConnectionPoint[];
  parameters: { [key: string]: number };
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

export interface SecurityLimits {
  maxFileBytes: number;
  maxRecords: number;
  maxFieldLength: number;
  maxNestingDepth: number;
}

export interface VdiUnit {
  symbol: string;
  kind: VdiQuantityKind;
  delta: boolean;
  siFactor: number;
}

export interface Vdi3805GenericAttribute {
  key: string;
  value: string;
  unit: string | null;
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

export interface Vdi3805GenericAttributes {
  entries: Vdi3805GenericAttribute[];
}

export interface Vdi3805CatalogueProduct {
  id: string;
  identity: ProductIdentity;
  title: LocalizedText[];
  sheet: number;
  records: NativeRecord[];
  configuration: Configuration;
  accessories: AccessoryLink[];
  components: CompositionLink[];
  extensions: Vdi3805ExtensionBag;
}

export interface Vdi3805ConnectionPoint {
  id: string;
  medium: string;
  position: number[];
  direction: number[];
  diameterMm: number | null;
}

export type Vdi3805EditionProfileChoice = "legacy" | "current";

export interface Vdi3805ExtensionBag {
  fields: { [key: string]: string };
}

export type VdiQuantityKind = "dimensionless" | "length" | "area" | "volume" | "mass" | "time" | "temperature" | "force" | "pressure" | "stress" | "moment" | "energy" | "power" | "thermalConductivity" | "thermalResistance" | "heatTransferCoefficient" | "airPermeability" | "ventilationRate" | "acceleration";

export const parseVdi3805Snapshot: NormWireReader<Vdi3805Snapshot> = normWireObject<Vdi3805Snapshot>({ catalog: normWireRequired(normWireRef(() => parseManufacturerCatalog)), editionProfile: normWireRequired(normWireMap(normWireLiteral("legacy", "current"))), correctionAsOf: normWireRequired(normWireRef(() => parseEditionId)), strictMode: normWireRequired(normWireBoolean), index: normWireRequired(normWireRef(() => parseCatalogIndex)), geometry: normWireRequired(normWireMap(normWireRef(() => parseParametricGeometry))), curves: normWireRequired(normWireMap(normWireRef(() => parseCharacteristicCurve))), limits: normWireRequired(normWireRef(() => parseSecurityLimits)) });
export const parseLocalizedText: NormWireReader<LocalizedText> = normWireObject<LocalizedText>({ locale: normWireRequired(normWireString), text: normWireRequired(normWireString) });
export const parseBuildingSystemNumber: NormWireReader<BuildingSystemNumber> = normWireObject<BuildingSystemNumber>({ systemCode: normWireRequired(normWireString), subsystem: normWireRequired(normWireString), sequence: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseExtensionFields: NormWireReader<ExtensionFields> = normWireObject<ExtensionFields>({ fields: normWireRequired(normWireMap(normWireString)) });
export const parseManufacturerFile: NormWireReader<ManufacturerFile> = normWireObject<ManufacturerFile>({ headerVersion: normWireRequired(normWireString), manufacturer: normWireRequired(normWireString), buildingSystemNumber: normWireRequired(normWireRef(() => parseBuildingSystemNumber)), created: normWireRequired(normWireString), charset: normWireRequired(normWireString), recordCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), extensions: normWireRequired(normWireRef(() => parseVdi3805ExtensionBag)) });
export const parseNativeRecord: NormWireReader<NativeRecord> = normWireObject<NativeRecord>({ family: normWireRequired(normWireString), fields: normWireRequired(normWireArray(normWireString)), extensions: normWireRequired(normWireRef(() => parseVdi3805ExtensionBag)) });
export const parseSheetAttributes: NormWireReader<SheetAttributes> = normWireTagged<SheetAttributes, "kind">("kind", {
  "valveHeating": normWireObject<{ kind: "valveHeating"; dn: number; kvsM3S: number; pressureClass: string; connectionType: string; authorityMin: number; authorityMax: number; }>({ kind: normWireRequired(normWireLiteral("valveHeating")), dn: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":65535})), kvsM3S: normWireRequired(normWireNumber), pressureClass: normWireRequired(normWireString), connectionType: normWireRequired(normWireString), authorityMin: normWireRequired(normWireNumber), authorityMax: normWireRequired(normWireNumber) }),
  "radiator": normWireObject<{ kind: "radiator"; standardOutputW: number; heatExponentN: number; lengthM: number; heightM: number; depthM: number; connectionType: string; }>({ kind: normWireRequired(normWireLiteral("radiator")), standardOutputW: normWireRequired(normWireNumber), heatExponentN: normWireRequired(normWireNumber), lengthM: normWireRequired(normWireNumber), heightM: normWireRequired(normWireNumber), depthM: normWireRequired(normWireNumber), connectionType: normWireRequired(normWireString) }),
  "pumpHeating": normWireObject<{ kind: "pumpHeating"; dnSuction: number; dnDischarge: number; nominalFlowM3S: number; nominalHeadM: number; motorPowerW: number; hydraulicEfficiency: number; qhCurveRef: string | null; }>({ kind: normWireRequired(normWireLiteral("pumpHeating")), dnSuction: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":65535})), dnDischarge: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":65535})), nominalFlowM3S: normWireRequired(normWireNumber), nominalHeadM: normWireRequired(normWireNumber), motorPowerW: normWireRequired(normWireNumber), hydraulicEfficiency: normWireRequired(normWireNumber), qhCurveRef: normWireDefault(normWireNullable(normWireString), () => null) }),
  "heatGenerator": normWireObject<{ kind: "heatGenerator"; nominalHeatOutputW: number; fuelType: string; flowTempMaxC: number; returnTempMinC: number; }>({ kind: normWireRequired(normWireLiteral("heatGenerator")), nominalHeatOutputW: normWireRequired(normWireNumber), fuelType: normWireRequired(normWireString), flowTempMaxC: normWireRequired(normWireNumber), returnTempMinC: normWireRequired(normWireNumber) }),
  "generic": normWireObject<{ kind: "generic"; entries?: Vdi3805GenericAttribute[]; }>({ kind: normWireRequired(normWireLiteral("generic")), entries: normWireOptional(normWireArray(normWireRef(() => parseVdi3805GenericAttribute))) }),
});
export const parseConfiguration: NormWireReader<Configuration> = normWireObject<Configuration>({ id: normWireRequired(normWireString), attributes: normWireRequired(normWireRef(() => parseSheetAttributes)), geometryRef: normWireDefault(normWireNullable(normWireString), () => null), functionRefs: normWireRequired(normWireArray(normWireString)) });
export const parseProductIdentity: NormWireReader<ProductIdentity> = normWireObject<ProductIdentity>({ manufacturerCode: normWireRequired(normWireString), productGroup: normWireRequired(normWireString), articleNumber: normWireRequired(normWireString) });
export const parseProduct: NormWireReader<Product> = normWireObject<Product>({ id: normWireRequired(normWireString), identity: normWireRequired(normWireRef(() => parseProductIdentity)), title: normWireRequired(normWireArray(normWireRef(() => parseLocalizedText))), sheet: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":65535})), records: normWireRequired(normWireArray(normWireRef(() => parseNativeRecord))), configuration: normWireRequired(normWireRef(() => parseConfiguration)), accessories: normWireRequired(normWireArray(normWireRef(() => parseAccessoryLink))), components: normWireRequired(normWireArray(normWireRef(() => parseCompositionLink))), extensions: normWireRequired(normWireRef(() => parseExtensionFields)) });
export const parseManufacturerCatalog: NormWireReader<ManufacturerCatalog> = normWireObject<ManufacturerCatalog>({ file: normWireRequired(normWireRef(() => parseManufacturerFile)), products: normWireRequired(normWireArray(normWireRef(() => parseProduct))), extensions: normWireRequired(normWireRef(() => parseExtensionFields)) });
export const parseEditionId: NormWireReader<EditionId> = normWireObject<EditionId>({ year: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":65535})), month: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})) });
export const parseCatalogIndexEntry: NormWireReader<CatalogIndexEntry> = normWireObject<CatalogIndexEntry>({ productId: normWireRequired(normWireString), sheet: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":65535})), tags: normWireRequired(normWireArray(normWireString)), dn: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":65535})), () => null) });
export const parseCatalogIndex: NormWireReader<CatalogIndex> = normWireObject<CatalogIndex>({ entries: normWireRequired(normWireArray(normWireRef(() => parseCatalogIndexEntry))) });
export const parseBoundingBox: NormWireReader<BoundingBox> = normWireObject<BoundingBox>({ minX: normWireRequired(normWireNumber), minY: normWireRequired(normWireNumber), minZ: normWireRequired(normWireNumber), maxX: normWireRequired(normWireNumber), maxY: normWireRequired(normWireNumber), maxZ: normWireRequired(normWireNumber) });
export const parseGeometryConnection: NormWireReader<GeometryConnection> = normWireObject<GeometryConnection>({ id: normWireRequired(normWireString), medium: normWireRequired(normWireString), position: normWireRequired(normWireArray(normWireNumber, 3, 3)), direction: normWireRequired(normWireArray(normWireNumber, 3, 3)), diameterMm: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseParametricGeometry: NormWireReader<ParametricGeometry> = normWireObject<ParametricGeometry>({ id: normWireRequired(normWireString), bbox: normWireRequired(normWireRef(() => parseBoundingBox)), connections: normWireRequired(normWireArray(normWireRef(() => parseVdi3805ConnectionPoint))), parameters: normWireRequired(normWireMap(normWireNumber)) });
export const parseCurvePoint: NormWireReader<CurvePoint> = normWireObject<CurvePoint>({ x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber) });
export const parseCharacteristicCurve: NormWireReader<CharacteristicCurve> = normWireObject<CharacteristicCurve>({ id: normWireRequired(normWireString), xUnit: normWireRequired(normWireRef(() => parseVdiUnit)), yUnit: normWireRequired(normWireRef(() => parseVdiUnit)), points: normWireRequired(normWireArray(normWireRef(() => parseCurvePoint))) });
export const parseSecurityLimits: NormWireReader<SecurityLimits> = normWireObject<SecurityLimits>({ maxFileBytes: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), maxRecords: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), maxFieldLength: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), maxNestingDepth: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseVdiUnit: NormWireReader<VdiUnit> = normWireObject<VdiUnit>({ symbol: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseVdiQuantityKind)), delta: normWireRequired(normWireBoolean), siFactor: normWireRequired(normWireNumber) });
export const parseVdi3805GenericAttribute: NormWireReader<Vdi3805GenericAttribute> = normWireObject<Vdi3805GenericAttribute>({ key: normWireRequired(normWireString), value: normWireRequired(normWireString), unit: normWireDefault(normWireNullable(normWireString), () => null) });
export const parseAccessoryLink: NormWireReader<AccessoryLink> = normWireObject<AccessoryLink>({ accessoryId: normWireRequired(normWireString), required: normWireRequired(normWireBoolean), quantity: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseCompositionLink: NormWireReader<CompositionLink> = normWireObject<CompositionLink>({ componentId: normWireRequired(normWireString), quantity: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseVdi3805GenericAttributes: NormWireReader<Vdi3805GenericAttributes> = normWireObject<Vdi3805GenericAttributes>({ entries: normWireRequired(normWireArray(normWireRef(() => parseVdi3805GenericAttribute))) });
export const parseVdi3805CatalogueProduct: NormWireReader<Vdi3805CatalogueProduct> = normWireObject<Vdi3805CatalogueProduct>({ id: normWireRequired(normWireString), identity: normWireRequired(normWireRef(() => parseProductIdentity)), title: normWireRequired(normWireArray(normWireRef(() => parseLocalizedText))), sheet: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":65535})), records: normWireRequired(normWireArray(normWireRef(() => parseNativeRecord))), configuration: normWireRequired(normWireRef(() => parseConfiguration)), accessories: normWireRequired(normWireArray(normWireRef(() => parseAccessoryLink))), components: normWireRequired(normWireArray(normWireRef(() => parseCompositionLink))), extensions: normWireRequired(normWireRef(() => parseVdi3805ExtensionBag)) });
export const parseVdi3805ConnectionPoint: NormWireReader<Vdi3805ConnectionPoint> = normWireObject<Vdi3805ConnectionPoint>({ id: normWireRequired(normWireString), medium: normWireRequired(normWireString), position: normWireRequired(normWireArray(normWireNumber, 3, 3)), direction: normWireRequired(normWireArray(normWireNumber, 3, 3)), diameterMm: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseVdi3805EditionProfileChoice: NormWireReader<Vdi3805EditionProfileChoice> = normWireLiteral("legacy", "current");
export const parseVdi3805ExtensionBag: NormWireReader<Vdi3805ExtensionBag> = normWireObject<Vdi3805ExtensionBag>({ fields: normWireRequired(normWireMap(normWireString)) });
export const parseVdiQuantityKind: NormWireReader<VdiQuantityKind> = normWireLiteral("dimensionless", "length", "area", "volume", "mass", "time", "temperature", "force", "pressure", "stress", "moment", "energy", "power", "thermalConductivity", "thermalResistance", "heatTransferCoefficient", "airPermeability", "ventilationRate", "acceleration");

export * from "./🪶️sqlite/🟦️.ts";
