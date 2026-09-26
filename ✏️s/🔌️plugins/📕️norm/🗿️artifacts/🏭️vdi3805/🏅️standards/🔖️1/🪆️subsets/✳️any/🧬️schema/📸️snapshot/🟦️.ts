/** Sheet attribute payload — replaces untyped configuration.parameters. */
export type SheetAttributes =
  | { kind: "valveHeating"; dn: number; kvsM3S: number; pressureClass: string; connectionType: string; authorityMin: number; authorityMax: number }
  | { kind: "radiator"; standardOutputW: number; heatExponentN: number; lengthM: number; heightM: number; depthM: number; connectionType: string }
  | { kind: "pumpHeating"; dnSuction: number; dnDischarge: number; nominalFlowM3S: number; nominalHeadM: number; motorPowerW: number; hydraulicEfficiency: number; qhCurveRef?: string | null }
  | { kind: "heatGenerator"; nominalHeatOutputW: number; fuelType: string; flowTempMaxC: number; returnTempMinC: number }
  | { kind: "generic"; entries: Array<{ key: string; value: string; unit?: string | null }> };

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
  id: string;
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

/** 🧬️ Vdi3805 snapshot schema — artifact-lane fields only. */
export interface Vdi3805Snapshot {
  /** @state artifact */
  manufacturerFile: ManufacturerFile;
  /** @state artifact */
  catalog: ManufacturerCatalog;
  /** @state artifact */
  editionProfile: Record<string, string>;
  /** @state artifact */
  correctionAsOf: EditionId;
  /** @state artifact */
  strictMode: boolean;
  /** @state artifact */
  index: CatalogIndex;
  /** @state artifact */
  geometry: Record<string, ParametricGeometry>;
  /** @state artifact */
  curves: Record<string, CharacteristicCurve>;
  /** @state artifact */
  limits: SecurityLimits;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normVdi3805SnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normVdi3805SnapshotGuardReject = (at: string, why: string): never => {
  throw new normVdi3805SnapshotGuardRefusal(at, why);
};

type normVdi3805SnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normVdi3805SnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normVdi3805SnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normVdi3805SnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normVdi3805SnapshotGuardReject(at, "value is not an object");
export const normVdi3805SnapshotGuardArray = (value: unknown, at: string, bounds: normVdi3805SnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normVdi3805SnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normVdi3805SnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normVdi3805SnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normVdi3805SnapshotGuardString = (value: unknown, at: string, bounds: normVdi3805SnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normVdi3805SnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normVdi3805SnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normVdi3805SnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normVdi3805SnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normVdi3805SnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normVdi3805SnapshotGuardReject(at, "value is not a boolean"));
export const normVdi3805SnapshotGuardNumber = (value: unknown, at: string, bounds: normVdi3805SnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normVdi3805SnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normVdi3805SnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normVdi3805SnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normVdi3805SnapshotGuardInteger = (value: unknown, at: string, bounds: normVdi3805SnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normVdi3805SnapshotGuardNumber(value, at, bounds) : normVdi3805SnapshotGuardReject(at, "value is not an integer");
export const normVdi3805SnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normVdi3805SnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normVdi3805SnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normVdi3805SnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseVdi3805Snapshot(value: unknown, at = "$"): Vdi3805Snapshot {
  const row = normVdi3805SnapshotGuardObject(value, at);
  return {
    manufacturerFile: normVdi3805SnapshotGuardObject(row["manufacturerFile"], `${at}.manufacturerFile`) as unknown as ManufacturerFile,
    catalog: normVdi3805SnapshotGuardObject(row["catalog"], `${at}.catalog`) as unknown as ManufacturerCatalog,
    editionProfile: normVdi3805SnapshotGuardObject(row["editionProfile"], `${at}.editionProfile`) as Record<string, string>,
    correctionAsOf: normVdi3805SnapshotGuardObject(row["correctionAsOf"], `${at}.correctionAsOf`) as unknown as EditionId,
    strictMode: normVdi3805SnapshotGuardBoolean(row["strictMode"], `${at}.strictMode`),
    index: normVdi3805SnapshotGuardObject(row["index"], `${at}.index`) as unknown as CatalogIndex,
    geometry: normVdi3805SnapshotGuardObject(row["geometry"], `${at}.geometry`) as Record<string, ParametricGeometry>,
    curves: normVdi3805SnapshotGuardObject(row["curves"], `${at}.curves`) as Record<string, CharacteristicCurve>,
    limits: normVdi3805SnapshotGuardObject(row["limits"], `${at}.limits`) as unknown as SecurityLimits,
  };
}

