import { parseSchemaRecord } from "@semio-tech/framework/schema/record";

export const ELEVATION_MIN_SIDE = 2;
export const ELEVATION_MAX_SIDE = 65;

/** 🏔️ A rectangular WGS84 grid of signed millimetres before exaggeration. */
export interface ElevationSource {
  readonly bounds: readonly [number, number, number, number];
  readonly rowsMillimetres: readonly (readonly number[])[];
  readonly missingSamples?: string;
}

function isElevationSide(size: number): boolean {
  return Number.isInteger(size) && size >= ELEVATION_MIN_SIDE && size <= ELEVATION_MAX_SIDE;
}

/** 📐️ Parses ordered WGS84 sample-centre bounds and a rectangular elevation grid. */
export function parseElevationSource(input: unknown): ElevationSource {
  const record = parseSchemaRecord(input, ["bounds", "rowsMillimetres", "missingSamples"], "$.elevationSource");
  const { bounds, rowsMillimetres, missingSamples } = record;
  if (!Array.isArray(bounds) || bounds.length !== 4 || ![...bounds].every(Number.isFinite)) {
    throw Error("Elevation source requires four finite bounds.");
  }
  const [west, south, east, north] = bounds;
  if (west < -180 || east > 180 || west >= east || south < -90 || north > 90 || south >= north) {
    throw Error("Elevation bounds must be ordered WGS84 coordinates.");
  }
  if (!Array.isArray(rowsMillimetres) || !isElevationSide(rowsMillimetres.length)) {
    throw Error("Elevation source requires 2 to 65 rows.");
  }
  const first = rowsMillimetres[0];
  if (!Array.isArray(first) || !isElevationSide(first.length)) {
    throw Error("Elevation source requires 2 to 65 columns.");
  }
  for (const row of rowsMillimetres) {
    if (!Array.isArray(row) || row.length !== first.length) throw Error("Elevation source requires equal-width rows.");
    for (const sample of row) {
      if (!Number.isInteger(sample) || sample < -2147483648 || sample > 2147483647) {
        throw Error("Elevation sample must be a signed 32-bit millimetre value.");
      }
    }
  }
  const source = { bounds: [west, south, east, north] as const, rowsMillimetres };
  if (!Object.hasOwn(record, "missingSamples")) return source;
  const count = rowsMillimetres.length * first.length;
  const byteCount = Math.ceil(count / 8);
  if (typeof missingSamples !== "string" || missingSamples.length !== 2 * byteCount || /[^0-9a-f]/.test(missingSamples) || !/[1-9a-f]/.test(missingSamples)) {
    throw Error("Elevation source requires a canonical missing sample mask.");
  }
  for (let byteIndex = 0; byteIndex < byteCount; byteIndex++) {
    const byte = Number.parseInt(missingSamples.slice(byteIndex * 2, byteIndex * 2 + 2), 16);
    for (let bit = 0; bit < 8; bit++) {
      if ((byte & (1 << bit)) === 0) continue;
      const ordinal = 8 * byteIndex + bit;
      if (ordinal >= count || rowsMillimetres[Math.floor(ordinal / first.length)][ordinal % first.length] !== 0) {
        throw Error("Missing elevation samples require zero payload and padding.");
      }
    }
  }
  return { ...source, missingSamples };
}
