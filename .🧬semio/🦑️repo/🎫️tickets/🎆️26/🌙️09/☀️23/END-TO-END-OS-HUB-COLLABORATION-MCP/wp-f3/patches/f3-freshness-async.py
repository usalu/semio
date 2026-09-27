"""🔧️ F3 — one asynchronous, bounded source-freshness walk (activation root). Idempotent; --dry-run reports only."""
import sys
DRY = "--dry-run" in sys.argv
ROOT = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts"
t = open(ROOT).read()
orig = t

def rep(old, new, label):
    global t
    if new in t and old not in t:
        print(f"skip (applied) {label}"); return
    n = t.count(old)
    if n != 1: raise SystemExit(f"anchor {label}: {n} matches")
    t = t.replace(old, new); print(f"ok {label}")

rep('''export const COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES = 20_000;
''', '''export const COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES = 20_000;

/** 🚦️ Source files one freshness walk stats or hashes at once, and components one freshness pass walks at once: the pass
 * runs beside a dev server, so it stays asynchronous end to end and bounded — 60 components × 54 131 files hashed
 * synchronously kept `serve s react dev` from spawning Vite for 49 s (ticket 26/09/23 F3). */
export const SOURCE_FRESHNESS_FILE_CONCURRENCY = 32;
export const SOURCE_FRESHNESS_COMPONENT_CONCURRENCY = 4;

/** 🚦️ Maps `items` through `map` with at most `limit` in flight, results in input order; a cancelled `signal` stops
 * taking new items and rejects with its reason. */
export async function mapBoundedV1<T, R>(items: readonly T[], limit: number, map: (item: T) => Promise<R>, signal?: AbortSignal): Promise<R[]> {
  const results = new Array<R>(items.length);
  let next = 0;
  const lane = async (): Promise<void> => {
    while (next < items.length) {
      signal?.throwIfAborted();
      const index = next++;
      results[index] = await map(items[index]!);
    }
  };
  await Promise.all(Array.from({ length: Math.max(1, Math.min(limit, items.length)) }, lane));
  signal?.throwIfAborted();
  return results;
}
''', "concurrency consts + mapBoundedV1")

rep('''/** 🔖️ Content hash of one plugin-owner source tree — same walk bounds as {@link newestComponentSourceMtime},
 * keyed only on plugin sources (never framework). Empty trees yield the empty-input SHA-256. */
export function componentSourceContentHash(sourceRoot: string, maximumEntries: number = COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES): string {
  return buildComponentSourceStatIndex(sourceRoot, maximumEntries).contentSha256;
}''', '''/** 🔖️ Content hash of one plugin-owner source tree — same walk bounds as {@link newestComponentSourceMtime},
 * keyed only on plugin sources (never framework). Empty trees yield the empty-input SHA-256. */
export async function componentSourceContentHash(sourceRoot: string, maximumEntries: number = COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES, signal?: AbortSignal): Promise<string> {
  return (await buildComponentSourceStatIndex(sourceRoot, maximumEntries, signal)).contentSha256;
}''', "componentSourceContentHash async")

# remove sync listComponentSourceFiles
start = t.find("/** 🕰 Lists plugin-owner source files under one root (same bounds as content hashing). */\nexport function listComponentSourceFiles(")
if start >= 0:
    end = t.find("export async function listComponentSourceFilesAsync(", start)
    assert end > start
    t = t[:start] + "/** 🕰 Lists plugin-owner source files under one root (same bounds as content hashing), sorted. */\n" + t[end:]
    print("ok remove sync listComponentSourceFiles")
else:
    print("skip (applied) remove sync listComponentSourceFiles")
t = t.replace("export async function listComponentSourceFilesAsync(", "export async function listComponentSourceFiles(")
t = t.replace("await listComponentSourceFilesAsync(", "await listComponentSourceFiles(")

rep('''/** 🔖 Builds the per-file source stat index and aggregate content hash for one plugin owner tree. */
export function buildComponentSourceStatIndex(sourceRoot: string, maximumEntries: number = COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES): SourceStatIndex {
  const files: SourceStatFileEntry[] = [];
  for (const absolute of listComponentSourceFiles(sourceRoot, maximumEntries)) {
    let stats: ReturnType<typeof statSync>;
    let bytes: Buffer;
    try {
      stats = statSync(absolute);
      bytes = readFileSync(absolute);
    } catch { continue; }
    files.push({
      path: relativeSourcePath(sourceRoot, absolute),
      size: stats.size,
      mtimeNs: fileMtimeNs(stats),
      sha256: createHash("sha256").update(bytes).digest("hex"),
    });
  }
  return {
    schema: "semio.dev.source-stat-index/v1",
    contentSha256: aggregateSourceContentHash(files),
    files,
  };
}''', '''/** 🧮️ One owner tree's walk: every source file stated with bounded parallelism, a prior digest reused when its size and
 * mtime match, every other file read and hashed — asynchronous end to end, so no event loop waits on a tree walk. */
type SourceStatWalk = Readonly<{ files: readonly SourceStatFileEntry[]; newestSourceMs?: number; newestSourcePath?: string; hashedFileCount: number; reusedFileCount: number }>;

async function walkSourceStats(sourceRoot: string, prior: ReadonlyMap<string, SourceStatFileEntry>, maximumEntries: number, signal?: AbortSignal): Promise<SourceStatWalk> {
  const absolutes = await listComponentSourceFiles(sourceRoot, maximumEntries);
  const parts = await mapBoundedV1(absolutes, SOURCE_FRESHNESS_FILE_CONCURRENCY, async (absolute) => {
    let stats: Awaited<ReturnType<typeof statAsync>>;
    try { stats = await statAsync(absolute); } catch { return undefined; }
    const path = relativeSourcePath(sourceRoot, absolute);
    const mtimeNs = fileMtimeNs(stats);
    const previous = prior.get(path);
    if (previous && previous.size === stats.size && previous.mtimeNs === mtimeNs) return { entry: previous, hashed: false, mtimeMs: stats.mtimeMs, absolute };
    let bytes: Buffer;
    try { bytes = await readFileAsync(absolute); } catch { return undefined; }
    return { entry: { path, size: stats.size, mtimeNs, sha256: createHash("sha256").update(bytes).digest("hex") }, hashed: true, mtimeMs: stats.mtimeMs, absolute };
  }, signal);
  const files: SourceStatFileEntry[] = [];
  let hashedFileCount = 0, reusedFileCount = 0, newestSourceMs: number | undefined, newestSourcePath: string | undefined;
  for (const part of parts) {
    if (!part) continue;
    files.push(part.entry);
    if (part.hashed) hashedFileCount += 1; else reusedFileCount += 1;
    if (newestSourceMs === undefined || part.mtimeMs > newestSourceMs) {
      newestSourceMs = part.mtimeMs;
      newestSourcePath = part.absolute;
    }
  }
  return { files, newestSourceMs, newestSourcePath, hashedFileCount, reusedFileCount };
}

/** 🔖 Builds the per-file source stat index and aggregate content hash for one plugin owner tree. */
export async function buildComponentSourceStatIndex(sourceRoot: string, maximumEntries: number = COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES, signal?: AbortSignal): Promise<SourceStatIndex> {
  const { files } = await walkSourceStats(sourceRoot, new Map(), maximumEntries, signal);
  return {
    schema: "semio.dev.source-stat-index/v1",
    contentSha256: aggregateSourceContentHash(files),
    files,
  };
}''', "buildComponentSourceStatIndex async")

rep('''/** 🔖 Persists both the aggregate content-hash marker and the per-file stat index for one staged module. */
export function writeStagedSourceFreshness(moduleDirectory: string, sourceRoot: string): string {
  const index = buildComponentSourceStatIndex(sourceRoot);''', '''/** 🔖 Persists both the aggregate content-hash marker and the per-file stat index for one staged module. */
export async function writeStagedSourceFreshness(moduleDirectory: string, sourceRoot: string, signal?: AbortSignal): Promise<string> {
  const index = await buildComponentSourceStatIndex(sourceRoot, COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES, signal);''', "writeStagedSourceFreshness async")

# replace both resolve functions with one async
s = t.find("/**\n * 🕰 Boot freshness: stat-walk the owner tree,")
e = t.find("//#endregion 🔖️StagedModuleFreshness", s)
if s >= 0 and e > s:
    t = t[:s] + '''/** 🕰 Boot freshness: walk the owner tree asynchronously, reuse per-file digests whose (size, mtimeNs) match the staged
 * index and hash only added or changed files; without an index every file is hashed — still asynchronously, so a dev
 * server keeps answering while it runs. `signal` cancels the walk. */
export async function resolveBootSourceContentHashes(options: {
  readonly sourceRoot: string;
  readonly moduleDirectory: string;
  readonly receiptSourceContentSha256?: string;
  readonly signal?: AbortSignal;
}): Promise<BootSourceHashResolution> {
  const stagedSourceContentSha256 = readStagedSourceContentHash(options.moduleDirectory) ?? options.receiptSourceContentSha256;
  const prior = new Map((readStagedSourceStatIndex(options.moduleDirectory)?.files ?? []).map((file) => [file.path, file]));
  const walk = await walkSourceStats(options.sourceRoot, prior, COMPONENT_SOURCE_SCAN_MAXIMUM_ENTRIES, options.signal);
  return {
    sourceContentSha256: aggregateSourceContentHash(walk.files),
    stagedSourceContentSha256,
    newestSourceMs: walk.newestSourceMs,
    newestSourcePath: walk.newestSourcePath,
    hashedFileCount: walk.hashedFileCount,
    reusedFileCount: walk.reusedFileCount,
  };
}

''' + t[e:]
    print("ok unify resolveBootSourceContentHashes")
else:
    print("skip (applied) unify resolve" if "export async function resolveBootSourceContentHashes(options" in t else "FAIL resolve anchor")

if DRY:
    print("dry-run: changed" if t != orig else "dry-run: no change")
else:
    if t != orig: open(ROOT, "w").write(t); print("written")
