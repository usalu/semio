import { parseSchemaRecord } from "@semio-tech/framework/schema/record";

/** 🏔️ A 33 by 33 grid of whole-meter elevations in WGS84 bounds. */
export interface ElevationSource {
  readonly bounds: readonly [number, number, number, number];
  readonly rows: readonly (readonly number[])[];
}

/** 📐️ Parses ordered and finite WGS84 bounds and the signed metre samples. */
export function parseElevationSource(input: unknown): ElevationSource {
  const { bounds, rows } = parseSchemaRecord(input, ["bounds", "rows"], "$.elevationSource");
  if (!Array.isArray(bounds) || bounds.length !== 4 || !bounds.every(Number.isFinite)) {
    throw Error("Elevation source requires four finite bounds.");
  }
  const [west, south, east, north] = bounds;
  if (west < -180 || east > 180 || west >= east || south < -90 || north > 90 || south >= north) {
    throw Error("Elevation bounds must be ordered WGS84 coordinates.");
  }
  if (!Array.isArray(rows) || rows.length !== 33 || !rows.every(row =>
    Array.isArray(row) && row.length === 33 && row.every(sample => Number.isInteger(sample) && sample >= -32768 && sample <= 32767)
  )) {
    throw Error("Elevation source requires 33 rows of 33 signed metre samples.");
  }
  return { bounds: [west, south, east, north], rows };
}
