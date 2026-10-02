/** 🔺️ `Vdi3805Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireDefault, normWireInteger, normWireLiteral, normWireMap, normWireNullable, normWireNumber, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString, normWireTagged } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface Vdi3805Diff {
  /** @state artifact */
  artifact?: Vdi3805Artifact;
  manufacturerFile?: ManufacturerFile;
  /** @state artifact */
  catalog?: ManufacturerCatalog;
  /** @state artifact */
  editionProfile?: { [key: string]: string };
  /** @state artifact */
  correctionAsOf?: EditionId;
  /** @state artifact */
  strictMode?: boolean;
  /** @state artifact */
  index?: CatalogIndex;
  /** @state artifact */
  geometry?: { [key: string]: ParametricGeometry };
  /** @state artifact */
  curves?: { [key: string]: CharacteristicCurve };
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
  extensions: ExtensionFields;
}

export interface NativeRecord {
  family: string;
  fields: string[];
  extensions: ExtensionFields;
}

export type SheetAttributes = { kind: "valveHeating"; dn: number; kvsM3S: number; pressureClass: string; connectionType: string; authorityMin: number; authorityMax: number; } | { kind: "radiator"; standardOutputW: number; heatExponentN: number; lengthM: number; heightM: number; depthM: number; connectionType: string; } | ({ kind: "pumpHeating"; dnSuction: number; dnDischarge: number; nominalFlowM3S: number; nominalHeadM: number; motorPowerW: number; hydraulicEfficiency: number; qhCurveRef: string | null; }) | { kind: "heatGenerator"; nominalHeatOutputW: number; fuelType: string; flowTempMaxC: number; returnTempMinC: number; } | { kind: "generic"; };

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
  identity: ProductIdentity;
  title: LocalizedText[];
  sheet: number;
  records: NativeRecord[];
  configuration: Configuration;
  accessories: AccessoryLink[];
  components: CompositionLink[];
  extensions: ExtensionFields;
  id: string;
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
  x: number;
  y: number;
  z: number;
  nx: number;
  ny: number;
  nz: number;
}

export interface ParametricGeometry {
  id: string;
  bbox: BoundingBox;
  connections: GeometryConnection[];
  parameters: { [key: string]: number };
}

export interface CurvePoint {
  x: number;
  y: number;
}

export interface CharacteristicCurve {
  id: string;
  points: CurvePoint[];
}

export interface SecurityLimits {
  maxFileBytes: number;
  maxRecords: number;
  maxFieldLength: number;
  maxNestingDepth: number;
}

export interface Vdi3805Artifact {
  catalog: ManufacturerCatalog;
  /** @state artifact */
  editionProfile: { [key: string]: string };
  correctionAsOf: EditionId;
  /** @state artifact */
  strictMode: boolean;
  index: CatalogIndex;
  /** @state artifact */
  geometry: { [key: string]: ParametricGeometry };
  /** @state artifact */
  curves: { [key: string]: CharacteristicCurve };
  limits: SecurityLimits | null;
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

export interface GenericAttribute {
  key: string;
  value: string;
  unit: string | null;
}

export interface GenericAttributes {
  entries: GenericAttribute[];
}

export const parseVdi3805Diff: NormWireReader<Vdi3805Diff> = normWireObject<Vdi3805Diff>({ artifact: normWireOptional(normWireRef(() => parseVdi3805Artifact)), manufacturerFile: normWireOptional(normWireRef(() => parseManufacturerFile)), catalog: normWireOptional(normWireRef(() => parseManufacturerCatalog)), editionProfile: normWireOptional(normWireMap(normWireString)), correctionAsOf: normWireOptional(normWireRef(() => parseEditionId)), strictMode: normWireOptional(normWireBoolean), index: normWireOptional(normWireRef(() => parseCatalogIndex)), geometry: normWireOptional(normWireMap(normWireRef(() => parseParametricGeometry))), curves: normWireOptional(normWireMap(normWireRef(() => parseCharacteristicCurve))) });
export const parseLocalizedText: NormWireReader<LocalizedText> = normWireObject<LocalizedText>({ locale: normWireRequired(normWireString), text: normWireRequired(normWireString) });
export const parseBuildingSystemNumber: NormWireReader<BuildingSystemNumber> = normWireObject<BuildingSystemNumber>({ systemCode: normWireRequired(normWireString), subsystem: normWireRequired(normWireString), sequence: normWireRequired(normWireInteger) });
export const parseExtensionFields: NormWireReader<ExtensionFields> = normWireObject<ExtensionFields>({ fields: normWireRequired(normWireMap(normWireString)) });
export const parseManufacturerFile: NormWireReader<ManufacturerFile> = normWireObject<ManufacturerFile>({ headerVersion: normWireRequired(normWireString), manufacturer: normWireRequired(normWireString), buildingSystemNumber: normWireRequired(normWireRef(() => parseBuildingSystemNumber)), created: normWireRequired(normWireString), charset: normWireRequired(normWireString), recordCount: normWireRequired(normWireInteger), extensions: normWireRequired(normWireRef(() => parseExtensionFields)) });
export const parseNativeRecord: NormWireReader<NativeRecord> = normWireObject<NativeRecord>({ family: normWireRequired(normWireString), fields: normWireRequired(normWireArray(normWireString)), extensions: normWireRequired(normWireRef(() => parseExtensionFields)) });
export const parseSheetAttributes: NormWireReader<SheetAttributes> = normWireTagged<SheetAttributes, "kind">("kind", {
  "valveHeating": normWireObject<{ kind: "valveHeating"; dn: number; kvsM3S: number; pressureClass: string; connectionType: string; authorityMin: number; authorityMax: number; }>({ kind: normWireRequired(normWireLiteral("valveHeating")), dn: normWireRequired(normWireInteger), kvsM3S: normWireRequired(normWireNumber), pressureClass: normWireRequired(normWireString), connectionType: normWireRequired(normWireString), authorityMin: normWireRequired(normWireNumber), authorityMax: normWireRequired(normWireNumber) }, false),
  "radiator": normWireObject<{ kind: "radiator"; standardOutputW: number; heatExponentN: number; lengthM: number; heightM: number; depthM: number; connectionType: string; }>({ kind: normWireRequired(normWireLiteral("radiator")), standardOutputW: normWireRequired(normWireNumber), heatExponentN: normWireRequired(normWireNumber), lengthM: normWireRequired(normWireNumber), heightM: normWireRequired(normWireNumber), depthM: normWireRequired(normWireNumber), connectionType: normWireRequired(normWireString) }, false),
  "pumpHeating": normWireObject<{ kind: "pumpHeating"; dnSuction: number; dnDischarge: number; nominalFlowM3S: number; nominalHeadM: number; motorPowerW: number; hydraulicEfficiency: number; qhCurveRef: string | null; }>({ kind: normWireRequired(normWireLiteral("pumpHeating")), dnSuction: normWireRequired(normWireInteger), dnDischarge: normWireRequired(normWireInteger), nominalFlowM3S: normWireRequired(normWireNumber), nominalHeadM: normWireRequired(normWireNumber), motorPowerW: normWireRequired(normWireNumber), hydraulicEfficiency: normWireRequired(normWireNumber), qhCurveRef: normWireDefault(normWireNullable(normWireString), () => null) }, false),
  "heatGenerator": normWireObject<{ kind: "heatGenerator"; nominalHeatOutputW: number; fuelType: string; flowTempMaxC: number; returnTempMinC: number; }>({ kind: normWireRequired(normWireLiteral("heatGenerator")), nominalHeatOutputW: normWireRequired(normWireNumber), fuelType: normWireRequired(normWireString), flowTempMaxC: normWireRequired(normWireNumber), returnTempMinC: normWireRequired(normWireNumber) }, false),
  "generic": normWireObject<{ kind: "generic"; }>({ kind: normWireRequired(normWireLiteral("generic")) }, false),
});
export const parseConfiguration: NormWireReader<Configuration> = normWireObject<Configuration>({ id: normWireRequired(normWireString), attributes: normWireRequired(normWireRef(() => parseSheetAttributes)), geometryRef: normWireDefault(normWireNullable(normWireString), () => null), functionRefs: normWireRequired(normWireArray(normWireString)) });
export const parseProductIdentity: NormWireReader<ProductIdentity> = normWireObject<ProductIdentity>({ manufacturerCode: normWireRequired(normWireString), productGroup: normWireRequired(normWireString), articleNumber: normWireRequired(normWireString) });
export const parseProduct: NormWireReader<Product> = normWireObject<Product>({ identity: normWireRequired(normWireRef(() => parseProductIdentity)), title: normWireRequired(normWireArray(normWireRef(() => parseLocalizedText))), sheet: normWireRequired(normWireInteger), records: normWireRequired(normWireArray(normWireRef(() => parseNativeRecord))), configuration: normWireRequired(normWireRef(() => parseConfiguration)), accessories: normWireRequired(normWireArray(normWireRef(() => parseAccessoryLink))), components: normWireRequired(normWireArray(normWireRef(() => parseCompositionLink))), extensions: normWireRequired(normWireRef(() => parseExtensionFields)), id: normWireRequired(normWireString) });
export const parseManufacturerCatalog: NormWireReader<ManufacturerCatalog> = normWireObject<ManufacturerCatalog>({ file: normWireRequired(normWireRef(() => parseManufacturerFile)), products: normWireRequired(normWireArray(normWireRef(() => parseProduct))), extensions: normWireRequired(normWireRef(() => parseExtensionFields)) });
export const parseEditionId: NormWireReader<EditionId> = normWireObject<EditionId>({ year: normWireRequired(normWireInteger), month: normWireRequired(normWireInteger) });
export const parseCatalogIndexEntry: NormWireReader<CatalogIndexEntry> = normWireObject<CatalogIndexEntry>({ productId: normWireRequired(normWireString), sheet: normWireRequired(normWireInteger), tags: normWireRequired(normWireArray(normWireString)), dn: normWireDefault(normWireNullable(normWireInteger), () => null) });
export const parseCatalogIndex: NormWireReader<CatalogIndex> = normWireObject<CatalogIndex>({ entries: normWireRequired(normWireArray(normWireRef(() => parseCatalogIndexEntry))) });
export const parseBoundingBox: NormWireReader<BoundingBox> = normWireObject<BoundingBox>({ minX: normWireRequired(normWireNumber), minY: normWireRequired(normWireNumber), minZ: normWireRequired(normWireNumber), maxX: normWireRequired(normWireNumber), maxY: normWireRequired(normWireNumber), maxZ: normWireRequired(normWireNumber) });
export const parseGeometryConnection: NormWireReader<GeometryConnection> = normWireObject<GeometryConnection>({ id: normWireRequired(normWireString), x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber), z: normWireRequired(normWireNumber), nx: normWireRequired(normWireNumber), ny: normWireRequired(normWireNumber), nz: normWireRequired(normWireNumber) });
export const parseParametricGeometry: NormWireReader<ParametricGeometry> = normWireObject<ParametricGeometry>({ id: normWireRequired(normWireString), bbox: normWireRequired(normWireRef(() => parseBoundingBox)), connections: normWireRequired(normWireArray(normWireRef(() => parseGeometryConnection))), parameters: normWireRequired(normWireMap(normWireNumber)) });
export const parseCurvePoint: NormWireReader<CurvePoint> = normWireObject<CurvePoint>({ x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber) });
export const parseCharacteristicCurve: NormWireReader<CharacteristicCurve> = normWireObject<CharacteristicCurve>({ id: normWireRequired(normWireString), points: normWireRequired(normWireArray(normWireRef(() => parseCurvePoint))) });
export const parseSecurityLimits: NormWireReader<SecurityLimits> = normWireObject<SecurityLimits>({ maxFileBytes: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), maxRecords: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), maxFieldLength: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), maxNestingDepth: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseVdi3805Artifact: NormWireReader<Vdi3805Artifact> = normWireObject<Vdi3805Artifact>({ catalog: normWireRequired(normWireRef(() => parseManufacturerCatalog)), editionProfile: normWireRequired(normWireMap(normWireString)), correctionAsOf: normWireRequired(normWireRef(() => parseEditionId)), strictMode: normWireRequired(normWireBoolean), index: normWireRequired(normWireRef(() => parseCatalogIndex)), geometry: normWireRequired(normWireMap(normWireRef(() => parseParametricGeometry))), curves: normWireRequired(normWireMap(normWireRef(() => parseCharacteristicCurve))), limits: normWireDefault(normWireNullable(normWireRef(() => parseSecurityLimits)), () => null) });
export const parseAccessoryLink: NormWireReader<AccessoryLink> = normWireObject<AccessoryLink>({ accessoryId: normWireRequired(normWireString), required: normWireRequired(normWireBoolean), quantity: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCompositionLink: NormWireReader<CompositionLink> = normWireObject<CompositionLink>({ componentId: normWireRequired(normWireString), quantity: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseGenericAttribute: NormWireReader<GenericAttribute> = normWireObject<GenericAttribute>({ key: normWireRequired(normWireString), value: normWireRequired(normWireString), unit: normWireDefault(normWireNullable(normWireString), () => null) });
export const parseGenericAttributes: NormWireReader<GenericAttributes> = normWireObject<GenericAttributes>({ entries: normWireRequired(normWireArray(normWireRef(() => parseGenericAttribute))) });
