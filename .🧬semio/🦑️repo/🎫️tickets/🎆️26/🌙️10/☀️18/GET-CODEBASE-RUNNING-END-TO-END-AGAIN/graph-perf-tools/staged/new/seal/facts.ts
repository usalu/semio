import { createHash } from "node:crypto";
import { lstat, readdir, readFile, rm } from "node:fs/promises";
import { join } from "node:path";
import { brotliCompressSync, brotliDecompressSync } from "node:zlib";
import { createWorkerPoolV1 } from "../../../../🏗️build/🧵️workers/🟨️.mjs";
import { censusInferenceRecordsV1 } from "../🧮️census/🟨️.mjs";

/** 🏷️ The identity of one node: device, inode, size and both times; any content or entry change alters it. */
export type FactStamp = readonly [dev: number, ino: number, size: number, mtimeMs: number, ctimeMs: number];

/** 🧾️ What a seal records about the inference records a graph was computed from and the distinct facts they observed. */
export interface SealFactsV1 {
  readonly records: readonly (readonly [path: string, stamp: FactStamp])[];
  readonly facts: Readonly<{ sha256: string; count: number }>;
}

const FACTS_PREFIX = "semio-graph-facts.", FACTS_SUFFIX = ".json.br", CONCURRENCY = 64;
const CENSUS_TASK = Object.freeze({ module: new URL("../🧮️census/🟨️.mjs", import.meta.url).href, export: "censusInferenceRecordsV1" });

/** 🏷️ One node's stamp. */
export function factStamp(value: { dev: number; ino: number; size: number; mtimeMs: number; ctimeMs: number }): FactStamp {
  return [value.dev, value.ino, value.size, value.mtimeMs, value.ctimeMs];
}

/** 🗃️ The immutable file that holds the fact table with the given digest. */
export function sealFactsFileNameV1(sha256: string): string {
  if (!/^[0-9a-f]{64}$/u.test(sha256)) throw new Error("Fact table digest required");
  return FACTS_PREFIX + sha256 + FACTS_SUFFIX;
}

/** 🧵️ Runs `visit` over `rows`, at most `CONCURRENCY` at a time. */
async function bounded<T>(rows: readonly T[], visit: (row: T) => Promise<void>, signal?: AbortSignal): Promise<void> {
  let next = 0;
  await Promise.all(Array.from({ length: Math.min(CONCURRENCY, rows.length) }, async () => { while (next < rows.length) { signal?.throwIfAborted(); await visit(rows[next++]!); } }));
}

/** 🏷️ Stamps of the named files, or `null` when one is missing. */
async function stamps(paths: readonly string[], signal?: AbortSignal): Promise<Map<string, FactStamp> | null> {
  const rows = new Map<string, FactStamp>();
  try { await bounded(paths, async (path) => { rows.set(path, factStamp(await lstat(path))); }, signal); } catch (error) { signal?.throwIfAborted(); return null; }
  return rows;
}

/**
 * 📸️ Captures the deduplicated fact table of the inference records the receipts name. The census of every record's header (read, decompress, parse, digest) runs on a
 * worker pool; each header must still carry the digest its receipt recorded and every record must keep its stamp across the census. Returns `null` when any of that fails.
 */
export async function captureSealFactsV1(workspaceData: string, receiptNames: readonly string[], signal?: AbortSignal): Promise<{ seal: SealFactsV1; bytes: Buffer } | null> {
  const rows = new Set<string>(), digests = new Map<string, string>();
  for (const name of receiptNames) {
    let receipt: { inference: [string, string][]; observations: unknown[] };
    try { receipt = JSON.parse(await readFile(join(workspaceData, "inference", name), "utf8")); } catch { return null; }
    for (const [path, digest] of receipt.inference) { if (digests.get(path) !== undefined && digests.get(path) !== digest) return null; digests.set(path, digest); }
    for (const row of receipt.observations) rows.add(JSON.stringify(row));
  }
  const paths = [...digests.keys()].sort(), before = await stamps(paths, signal);
  if (!before) return null;
  const pool = createWorkerPoolV1({ signal }), size = Math.max(1, Math.ceil(paths.length / (pool.size * 4))), chunks: string[][] = [];
  for (let at = 0; at < paths.length; at += size) chunks.push(paths.slice(at, at + size));
  let results: { records: [string, string, number][]; rows: string[] }[];
  try { results = await pool.map(CENSUS_TASK, chunks, { signal }); }
  catch (error) { signal?.throwIfAborted(); results = chunks.map((chunk) => censusInferenceRecordsV1(chunk)); }
  finally { await pool.close(); }
  for (const result of results) {
    for (const [path, header] of result.records) if (header !== digests.get(path)) return null;
    for (const row of result.rows) rows.add(row);
  }
  const after = await stamps(paths, signal);
  if (!after || paths.some((path) => before.get(path)!.some((value, index) => value !== after.get(path)![index]))) return null;
  const bytes = brotliCompressSync(Buffer.from("[" + [...rows].sort().join(",") + "]"));
  return { seal: { records: paths.map((path) => [path, before.get(path)!] as const), facts: { sha256: createHash("sha256").update(bytes).digest("hex"), count: rows.size } }, bytes };
}

/** ✅️ True when the fact table file is intact, every recorded inference record keeps its stamp and `hold` accepts the table's distinct facts. */
export async function sealFactsHoldV1(workspaceData: string, seal: SealFactsV1, hold: (facts: unknown[]) => Promise<boolean>, signal?: AbortSignal): Promise<boolean> {
  let table: unknown;
  try {
    const bytes = await readFile(join(workspaceData, sealFactsFileNameV1(seal.facts.sha256)));
    if (createHash("sha256").update(bytes).digest("hex") !== seal.facts.sha256) return false;
    table = JSON.parse(brotliDecompressSync(bytes).toString("utf8"));
  } catch { return false; }
  if (!Array.isArray(table) || table.length !== seal.facts.count) return false;
  const current = await stamps(seal.records.map(([path]) => path), signal);
  if (!current || seal.records.some(([path, stamp]) => stamp.some((value, index) => value !== current.get(path)![index]))) return false;
  return hold(table);
}

/** 🧹️ Removes every fact table file that no live seal names; failure only costs disk. */
export async function retireSealFactsV1(workspaceData: string, keep: ReadonlySet<string>): Promise<void> {
  let names: string[];
  try { names = await readdir(workspaceData); } catch { return; }
  for (const name of names) if (name.startsWith(FACTS_PREFIX) && name.endsWith(FACTS_SUFFIX) && !keep.has(name)) await rm(join(workspaceData, name), { force: true }).catch(() => {});
}
