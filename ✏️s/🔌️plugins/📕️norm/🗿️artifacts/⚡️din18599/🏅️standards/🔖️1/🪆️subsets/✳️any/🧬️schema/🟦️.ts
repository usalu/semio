/** 🧬️ Din18599Artifact schema — artifact-lane fields matching Rust camelCase wire names. */

import { parseArtifactChild, type ArtifactChild } from "../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface ThermalZone {
  /** @state artifact */
  id: string;
  /** @state artifact */
  labelEn: string;
  /** @state artifact */
  labelDe: string;
  /** @state artifact */
  usageProfile: "WFH" | "Office" | "School";
  /** @state artifact */
  areaM2: number;
  /** @state artifact */
  volumeM3: number;
  /** @state artifact */
  thetaIHeatC: number;
  /** @state artifact */
  thetaICoolC: number;
  /** @state artifact */
  occupants: number;
  /** @state artifact */
  internalGainsWM2: number;
  /** @state artifact */
  lightingPowerWM2: number;
}

export interface EnvelopeElement {
  /** @state artifact */
  id: string;
  /** @state artifact */
  labelEn: string;
  /** @state artifact */
  labelDe: string;
  /** @state artifact */
  kind: string;
  /** @state artifact */
  zoneId: string;
  /** @state artifact */
  areaM2: number;
  /** @state artifact */
  uValueWM2k: number;
  /** @state artifact */
  orientationDeg: number;
  /** @state artifact */
  tiltDeg: number;
  /** @state artifact */
  gValue: number;
  /** @state artifact */
  fc: number;
  /** @state artifact */
  adjacency: string;
}

export interface HeatingSystem {
  /** @state artifact */
  generationEfficiency: number;
  /** @state artifact */
  distributionEfficiency: number;
  /** @state artifact */
  storageEfficiency: number;
  /** @state artifact */
  transferEfficiency: number;
  /** @state artifact */
  energyCarrier: string;
}

export interface DhwSystem {
  /** @state artifact */
  specificDemandKwhPersonA: number;
  /** @state artifact */
  storageLossKwhA: number;
  /** @state artifact */
  distributionLossKwhA: number;
  /** @state artifact */
  energyCarrier: string;
}

export interface VentilationSystem {
  /** @state artifact */
  airflowM3H: number;
  /** @state artifact */
  heatRecoveryEta: number;
  /** @state artifact */
  fanPowerW: number;
}

export interface CoolingPlant {
  /** @state artifact */
  eer: number;
  /** @state artifact */
  energyCarrier: string;
}
export interface CoolingSystem {
  /** @state artifact */
  plant: CoolingPlant | null;
}

export interface LightingSystem {
  /** @state artifact */
  controlFactor: number;
}

export interface Renewables {
  /** @state artifact */
  pvAreaM2: number;
  /** @state artifact */
  pvEfficiency: number;
  /** @state artifact */
  solarThermalKwhA: number;
}

export interface Din18599Artifact {
  /** @state artifact */
  buildingCategory: string;
  /** @state artifact */
  attachment: string;
  /** @state artifact */
  useClass: string;
  /** @state artifact */
  method: string;
  /** @state artifact */
  netFloorAreaM2: number;
  /** @state artifact */
  heatedVolumeM3: number;
  /** @state artifact */
  gegQpFactor: number;
  /** @state artifact */
  deltaUWbWM2k: number;
  /** @state artifact */
  automationClass: string;
  /** @state artifact */
  zones: ThermalZone[];
  /** @state artifact */
  elements: EnvelopeElement[];
  /** @state artifact */
  heating: HeatingSystem;
  /** @state artifact */
  dhw: DhwSystem;
  /** @state artifact */
  ventilation: VentilationSystem;
  /** @state artifact */
  cooling: CoolingSystem;
  /** @state artifact */
  lighting: LightingSystem;
  /** @state artifact */
  renewables: Renewables;
  /** @state artifact @child kind=s.stdio.semio */
  climate: ArtifactChild;
}

//#region 🔖️ExactDecoding
type FieldSpec = "string" | "number" | "integer" | "boolean" | "child" | { readonly array: FieldSpec } | { readonly record: RecordSpec } | { readonly nullable: FieldSpec };
type RecordSpec = Readonly<Record<string, FieldSpec>>;

const ZONE: RecordSpec = { id: "string", labelEn: "string", labelDe: "string", usageProfile: "string", areaM2: "number", volumeM3: "number", thetaIHeatC: "number", thetaICoolC: "number", occupants: "integer", internalGainsWM2: "number", lightingPowerWM2: "number" };
const ELEMENT: RecordSpec = { id: "string", labelEn: "string", labelDe: "string", kind: "string", zoneId: "string", areaM2: "number", uValueWM2k: "number", orientationDeg: "number", tiltDeg: "number", gValue: "number", fc: "number", adjacency: "string" };

/** 🗺️ The Din18599Artifact wire contract (Rust `Din18599Snapshot`, camelCase), one spec per top-level field. */
const DIN18599_FIELDS: RecordSpec = {
  buildingCategory: "string",
  attachment: "string",
  useClass: "string",
  method: "string",
  netFloorAreaM2: "number",
  heatedVolumeM3: "number",
  gegQpFactor: "number",
  deltaUWbWM2k: "number",
  automationClass: "string",
  zones: { array: { record: ZONE } },
  elements: { array: { record: ELEMENT } },
  heating: { record: { generationEfficiency: "number", distributionEfficiency: "number", storageEfficiency: "number", transferEfficiency: "number", energyCarrier: "string" } },
  dhw: { record: { specificDemandKwhPersonA: "number", storageLossKwhA: "number", distributionLossKwhA: "number", energyCarrier: "string" } },
  ventilation: { record: { airflowM3H: "number", heatRecoveryEta: "number", fanPowerW: "number" } },
  cooling: { record: { plant: { nullable: { record: { eer: "number", energyCarrier: "string" } } } } },
  lighting: { record: { controlFactor: "number" } },
  renewables: { record: { pvAreaM2: "number", pvEfficiency: "number", solarThermalKwhA: "number" } },
  climate: "child",
};

/** 🔺️ The sparse Din18599Diff overrides: whole-list fields travel as `{ values }` (Rust `Din18599ZoneList`/`Din18599ElementList`). */
const DIN18599_DIFF_LISTS: RecordSpec = { zones: { record: { values: { array: { record: ZONE } } } }, elements: { record: { values: { array: { record: ELEMENT } } } } };

function decodeField(spec: FieldSpec, value: unknown, at: string): unknown {
  if (spec === "child") return parseArtifactChild(value);
  if (spec === "string") {
    if (typeof value !== "string") throw new Error(`${at}: expected a string`);
    return value;
  }
  if (spec === "number" || spec === "integer") {
    if (typeof value !== "number" || !Number.isFinite(value) || (spec === "integer" && !Number.isInteger(value))) throw new Error(`${at}: expected ${spec === "integer" ? "an integer" : "a number"}`);
    return value;
  }
  if (spec === "boolean") {
    if (typeof value !== "boolean") throw new Error(`${at}: expected a boolean`);
    return value;
  }
  if ("nullable" in spec) return value === null ? null : decodeField(spec.nullable, value, at);
  if ("array" in spec) {
    if (!Array.isArray(value)) throw new Error(`${at}: expected an array`);
    return value.map((item, index) => decodeField(spec.array, item, `${at}[${index}]`));
  }
  return decodeRecord(spec.record, value, false, at);
}

function decodeRecord(spec: RecordSpec, value: unknown, partial: boolean, at: string): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: expected an object`);
  const row = value as Record<string, unknown>;
  const unknown = Object.keys(row).find((key) => !Object.hasOwn(spec, key));
  if (unknown !== undefined) throw new Error(`${at}.${unknown}: not a field of this record`);
  const out: Record<string, unknown> = {};
  for (const [key, field] of Object.entries(spec)) {
    if (!Object.hasOwn(row, key)) {
      if (partial) continue;
      throw new Error(`${at}.${key}: required`);
    }
    out[key] = partial && row[key] === null ? null : decodeField(field, row[key], `${at}.${key}`);
  }
  return out;
}

/** 📥️ Decodes the Din18599 wire fields exactly: every field of the contract (or, `partial`, any subset of them, each nullable, with
 * the whole-list fields in their `{ values }` delta form); unknown fields, editor state and malformed children are refused. */
export function parseDin18599Fields(value: unknown, partial: boolean, at = "$"): Partial<Din18599Artifact> {
  return decodeRecord(partial ? { ...DIN18599_FIELDS, ...DIN18599_DIFF_LISTS } : DIN18599_FIELDS, value, partial, at) as Partial<Din18599Artifact>;
}

/** 📥️ Decodes the exact Din18599Artifact wire contract. */
export function parseDin18599Artifact(value: unknown, at = "$"): Din18599Artifact {
  return parseDin18599Fields(value, false, at) as Din18599Artifact;
}
//#endregion 🔖️ExactDecoding

export { parseArtifactChild };
