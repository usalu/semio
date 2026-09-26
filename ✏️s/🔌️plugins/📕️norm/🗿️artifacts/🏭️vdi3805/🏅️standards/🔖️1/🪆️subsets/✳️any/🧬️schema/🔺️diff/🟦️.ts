/** Sheet attribute payload — replaces untyped configuration.parameters. */
export type SheetAttributes =
  | { kind: "valveHeating"; dn: number; kvsM3S: number; pressureClass: string; connectionType: string; authorityMin: number; authorityMax: number }
  | { kind: "radiator"; standardOutputW: number; heatExponentN: number; lengthM: number; heightM: number; depthM: number; connectionType: string }
  | { kind: "pumpHeating"; dnSuction: number; dnDischarge: number; nominalFlowM3S: number; nominalHeadM: number; motorPowerW: number; hydraulicEfficiency: number; qhCurveRef?: string | null }
  | { kind: "heatGenerator"; nominalHeatOutputW: number; fuelType: string; flowTempMaxC: number; returnTempMinC: number }
  | { kind: "generic" };

export interface LocalizedText { locale: string; text: string }
export interface BuildingSystemNumber { systemCode: string; subsystem: string; sequence: number }
export interface ExtensionFields { fields: Record<string, string> }
export interface ManufacturerFile {
  headerVersion: string;
  manufacturer: string;
  buildingSystemNumber: BuildingSystemNumber;
  created: string;
  charset: string;
  recordCount: number;
  extensions: ExtensionFields;
}
export interface NativeRecord { family: string; fields: string[]; extensions: ExtensionFields }
export interface Configuration {
  id: string;
  attributes: SheetAttributes;
  geometryRef?: string | null;
  functionRefs: string[];
}
export interface ProductIdentity { manufacturerCode: string; productGroup: string; articleNumber: string }
export interface Product {
  identity: ProductIdentity;
  title: LocalizedText[];
  sheet: number;
  records: NativeRecord[];
  configuration: Configuration;
  accessories: string[];
  components: string[];
  extensions: ExtensionFields;
}
export interface ManufacturerCatalog { file: ManufacturerFile; products: Product[]; extensions: ExtensionFields }
export interface EditionId { year: number; month: number }
export interface CatalogIndexEntry { productId: string; sheet: number; tags: string[]; dn?: number | null }
export interface CatalogIndex { entries: CatalogIndexEntry[] }
export interface BoundingBox { minX: number; minY: number; minZ: number; maxX: number; maxY: number; maxZ: number }
export interface GeometryConnection { id: string; x: number; y: number; z: number; nx: number; ny: number; nz: number }
export interface ParametricGeometry { id: string; bbox: BoundingBox; connections: GeometryConnection[]; parameters: Record<string, number> }
export interface CurvePoint { x: number; y: number }
export interface CharacteristicCurve { id: string; points: CurvePoint[] }
export interface SecurityLimits { maxFileBytes: number; maxRecords: number; maxFieldLength: number; maxNestingDepth: number }

export interface Vdi3805Artifact {
  manufacturerFile: ManufacturerFile;
  catalog: ManufacturerCatalog;
  editionProfile: Record<string, string>;
  correctionAsOf: EditionId;
  strictMode: boolean;
  index: CatalogIndex;
  geometry: Record<string, ParametricGeometry>;
  curves: Record<string, CharacteristicCurve>;
}

/** 🧬️ Vdi3805 diff schema — sparse field delta. */
export interface Vdi3805Diff {
  /** @state artifact */
  artifact?: Vdi3805Artifact;
  /** @state artifact */
  manufacturerFile?: ManufacturerFile;
  /** @state artifact */
  catalog?: ManufacturerCatalog;
  /** @state artifact */
  editionProfile?: Record<string, string>;
  /** @state artifact */
  correctionAsOf?: EditionId;
  /** @state artifact */
  strictMode?: boolean;
  /** @state artifact */
  index?: CatalogIndex;
  /** @state artifact */
  geometry?: Record<string, ParametricGeometry>;
  /** @state artifact */
  curves?: Record<string, CharacteristicCurve>;
  /** @state artifact */
}

