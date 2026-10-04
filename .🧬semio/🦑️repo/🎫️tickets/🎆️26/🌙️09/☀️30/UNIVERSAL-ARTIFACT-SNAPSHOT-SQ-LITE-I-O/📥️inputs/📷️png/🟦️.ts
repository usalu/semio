/** 🧬️ Exact byte-authoritative PNG snapshot. */
export interface PngSnapshot {
  schema: string;
  bytes: number[];
}

export class stdioPng12AnySnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const reject = (at: string, why: string): never => {
  throw new stdioPng12AnySnapshotGuardRefusal(at, why);
};

export function parsePngSnapshot(value: unknown, at = "$"): PngSnapshot {
  if (value === null || typeof value !== "object" || Array.isArray(value)) return reject(at, "value is not an object");
  const row = value as Record<string, unknown>;
  if (typeof row.schema !== "string") return reject(`${at}.schema`, "value is not text");
  if (Object.keys(row).some(key => key !== "schema" && key !== "bytes")) return reject(at, "value contains an undeclared field");
  if (!Array.isArray(row.bytes)) return reject(`${at}.bytes`, "value is not an array");
  const bytes = row.bytes.map((item, index) => Number.isSafeInteger(item) && Number(item) >= 0 && Number(item) <= 255 ? Number(item) : reject(`${at}.bytes[${index}]`, "value is not a byte"));
  return { schema: row.schema, bytes };
}
