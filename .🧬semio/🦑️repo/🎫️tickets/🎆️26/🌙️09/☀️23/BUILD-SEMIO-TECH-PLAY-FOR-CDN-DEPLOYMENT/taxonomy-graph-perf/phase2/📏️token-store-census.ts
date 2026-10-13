/** 📏️ Census of a taxonomy token store namespace: records, token counts, serialized bytes, distinct resolution queries and their bytes,
 * to size a compact record encoding. argv: <namespaceDirectory> */
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
const [directory] = process.argv.slice(2);
let records = 0, tokens = 0, bytes = 0, queryBytes = 0, queries = 0, unsupported = 0, withTargets = 0;
const largest: [number, number][] = [];
for (const name of readdirSync(directory!).filter((entry) => entry.endsWith(".idx"))) {
  const index = JSON.parse(readFileSync(join(directory!, name), "utf8")) as { pack: string; records: [string, number, number, string][] };
  const pack = readFileSync(join(directory!, `${index.pack}.pack`));
  for (const [, offset, length] of index.records) {
    records++; bytes += length;
    const value = JSON.parse(pack.subarray(offset, offset + length).toString("utf8")) as { tokens?: { value: string; targetValues?: string[]; unsupportedReason?: string }[] } | { value: string }[];
    const list = Array.isArray(value) ? value : value.tokens ?? [];
    tokens += list.length;
    const distinct = new Set<string>();
    for (const token of list) { if (token.unsupportedReason) { unsupported++; continue; } if (token.targetValues) withTargets++; distinct.add(JSON.stringify(token.targetValues ?? [token.value])); }
    queries += distinct.size;
    let size = 0; for (const query of distinct) size += query.length + 1;
    queryBytes += size;
    largest.push([length, size]);
  }
}
largest.sort((a, b) => b[0] - a[0]);
console.log(`[DEBUG] records=${records} tokens=${tokens} bytes=${(bytes / 2 ** 20).toFixed(0)}MiB distinctQueries=${queries} queryBytes=${(queryBytes / 2 ** 20).toFixed(0)}MiB unsupported=${unsupported} withTargetValues=${withTargets}`);
console.log(`[DEBUG] largest records (bytes, distinct query bytes): ${largest.slice(0, 8).map(([a, b]) => `${(a / 2 ** 20).toFixed(1)}M/${(b / 2 ** 20).toFixed(1)}M`).join(" ")}`);
