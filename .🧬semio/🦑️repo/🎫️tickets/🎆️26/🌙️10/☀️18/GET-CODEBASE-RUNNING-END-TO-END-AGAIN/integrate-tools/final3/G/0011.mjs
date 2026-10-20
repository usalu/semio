/** 🧮️ Pure census of persisted inference records: the header digest and the distinct observation rows of each, so a graph seal can be verified from one deduplicated fact table instead of by reading every record again. */
import { readFileSync } from "node:fs";
import { brotliDecompressSync } from "node:zlib";
import { createHash } from "node:crypto";

/** 📊️ Reads the records in order; `rows` holds each distinct row once, as its canonical JSON text. */
export function censusInferenceRecordsV1(paths, context) {
  const distinct = new Set(), records = [];
  for (let index = 0; index < paths.length; index++) {
    const text = brotliDecompressSync(readFileSync(paths[index])).toString("utf8"), split = text.indexOf("\n");
    if (split < 0) throw new Error("Inference record header required: " + paths[index]);
    const header = text.slice(0, split), observations = JSON.parse(header).observations;
    if (!Array.isArray(observations)) throw new Error("Inference record observations required: " + paths[index]);
    for (const row of observations) distinct.add(JSON.stringify(row));
    records.push([paths[index], createHash("sha256").update(header).digest("hex"), observations.length]);
    context?.progress?.(index + 1, paths.length);
  }
  return { records, rows: [...distinct] };
}
