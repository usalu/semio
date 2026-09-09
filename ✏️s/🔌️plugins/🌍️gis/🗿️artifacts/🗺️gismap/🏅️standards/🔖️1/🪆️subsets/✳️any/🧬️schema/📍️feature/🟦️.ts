/** 📍️ GIS features own identifiers and framework dynamic value payloads. */
import { parseSchemaRecord } from "../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧾️record/🟦️.ts";
import { parseDslValue, type DslValue } from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";
export type { DslValue } from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts";
export interface GisMapFeature { id: string; data: DslValue; }
export interface GisMapFeaturePatch { data: DslValue; }

/** 🧬️ Parses every JSON value supported by the native feature payload. */
export function parseGisMapFeature(value: unknown, at = "$"): GisMapFeature {
  const row = parseSchemaRecord(value, ["id", "data"], at);
  if (typeof row.id !== "string") throw new Error(`${at}.id: string required`);
  return { id: row.id, data: parseDslValue(row.data) };
}

/** 🩹️ Omitted patch data is native unchanged null. */
export function parseGisMapFeaturePatch(value: unknown, at = "$"): GisMapFeaturePatch {
  const row = parseSchemaRecord(value, ["data"], at);
  return { data: row.data == null ? null : parseDslValue(row.data) };
}
