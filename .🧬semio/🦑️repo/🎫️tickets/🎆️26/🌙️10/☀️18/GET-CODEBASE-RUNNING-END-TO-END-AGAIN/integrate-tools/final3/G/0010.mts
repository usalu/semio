/** 📊️ Header digest and distinct observation rows of persisted inference records. */
export function censusInferenceRecordsV1(paths: readonly string[], context?: { progress?(completed: number, total: number): void }): { records: [path: string, headerSha256: string, rows: number][]; rows: string[] };
