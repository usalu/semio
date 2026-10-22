import { join } from "node:path";
import { existsSync, readFileSync, statSync, writeFileSync } from "node:fs";
import type { MetricsRunner, UlocByLanguage } from "../🏷️types/🟦️.ts";
import { gitDir, gitRepoRoot, gitTrackedPaths } from "../../🌳️git/🟦️.ts";
import { spawnCapturedSync } from "../../../../🏃️process/🔍️probe/🟦️.ts";
import { gitSpawnEnv } from "../../../../🏃️process/🌿️environment/🌳️git/🟦️.ts";
import { classifyPathForMetrics, MAX_METRICS_FILE_BYTES, shouldSkipPathForUloc } from "../🛤️path-rules/🟦️.ts";
import { countUnifiedLocForFile } from "../🔢️uloc/🟦️.ts";

const METRICS_CACHE_VERSION = 5;

type MetricsCacheFile = {
  version: number;
  head: string;
  trackedFiles: number;
  uloc: { totalLoc: number; langCount: number; counts: UlocByLanguage };
  size: { totalBytes: number; langCount: number; counts: UlocByLanguage };
};

function metricsCachePath(root: string): string {
  return join(gitDir(root), "compose-metrics-cache.json");
}

function metricsMapStats(counts: UlocByLanguage): { total: number; langCount: number } {
  let total = 0;
  let langCount = 0;
  for (const n of Object.values(counts)) {
    if (n > 0) {
      total += n;
      langCount++;
    }
  }
  return { total, langCount };
}

/** 🧪️Whether cached uloc looks like a complete repo scan (rejects partial/stale caches). */
export function isUlocCachePlausible(root: string, counts: UlocByLanguage): boolean {
  const { total: totalLoc, langCount } = metricsMapStats(counts);
  if (totalLoc <= 0 || langCount === 0) return false;
  const tracked = gitTrackedPaths(root).length;
  if (tracked === 0) {
    const head = spawnCapturedSync("git", ["rev-parse", "HEAD"], { cwd: gitRepoRoot(root), env: gitSpawnEnv() });
    if (head.status === 0 && head.stdout.toString("utf8").trim()) return false;
    return true;
  }
  if (tracked < 100) return totalLoc > 0 && (langCount >= 2 || totalLoc >= 50);
  if (langCount < 6 && tracked >= 300) return false;
  if (totalLoc < tracked * 3) return false;
  return true;
}

function isSizeCachePlausible(root: string, counts: UlocByLanguage): boolean {
  const { total: totalBytes, langCount } = metricsMapStats(counts);
  if (totalBytes <= 0 || langCount === 0) return false;
  const tracked = gitTrackedPaths(root).length;
  if (tracked === 0) {
    const head = spawnCapturedSync("git", ["rev-parse", "HEAD"], { cwd: gitRepoRoot(root), env: gitSpawnEnv() });
    if (head.status === 0 && head.stdout.toString("utf8").trim()) return false;
    return true;
  }
  if (tracked < 100) return totalBytes > 0 && langCount >= 1;
  if (langCount < 3 && tracked >= 300) return false;
  return true;
}

function readMetricsCache(root: string): { uloc: UlocByLanguage; size: UlocByLanguage } | null {
  const head = spawnCapturedSync("git", ["rev-parse", "HEAD"], { cwd: root, env: gitSpawnEnv() });
  if (head.status !== 0) return null;
  const h = head.stdout.toString("utf8").trim();
  if (!h) return null;
  try {
    const raw = readFileSync(metricsCachePath(root), "utf8");
    const parsed = JSON.parse(raw) as Partial<MetricsCacheFile>;
    if (parsed.version !== METRICS_CACHE_VERSION) return null;
    if (parsed.head !== h || !parsed.uloc?.counts || !parsed.size?.counts) return null;
    const tracked = gitTrackedPaths(root).length;
    if (typeof parsed.trackedFiles === "number" && Math.abs(tracked - parsed.trackedFiles) > 50) return null;
    if (!isUlocCachePlausible(root, parsed.uloc.counts)) return null;
    if (!isSizeCachePlausible(root, parsed.size.counts)) return null;
    const ulocStats = metricsMapStats(parsed.uloc.counts);
    if (parsed.uloc.totalLoc !== ulocStats.total || parsed.uloc.langCount !== ulocStats.langCount) return null;
    const sizeStats = metricsMapStats(parsed.size.counts);
    if (parsed.size.totalBytes !== sizeStats.total || parsed.size.langCount !== sizeStats.langCount) return null;
    return { uloc: parsed.uloc.counts, size: parsed.size.counts };
  } catch {
    return null;
  }
}

function writeMetricsCache(root: string, uloc: UlocByLanguage, size: UlocByLanguage): void {
  const head = spawnCapturedSync("git", ["rev-parse", "HEAD"], { cwd: root, env: gitSpawnEnv() });
  if (head.status !== 0) return;
  const h = head.stdout.toString("utf8").trim();
  if (!h) return;
  const ulocStats = metricsMapStats(uloc);
  const sizeStats = metricsMapStats(size);
  const payload: MetricsCacheFile = {
    version: METRICS_CACHE_VERSION,
    head: h,
    trackedFiles: gitTrackedPaths(root).length,
    uloc: { totalLoc: ulocStats.total, langCount: ulocStats.langCount, counts: uloc },
    size: { totalBytes: sizeStats.total, langCount: sizeStats.langCount, counts: size },
  };
  writeFileSync(metricsCachePath(root), JSON.stringify(payload));
}

/** 📊️Scans all git-tracked paths and sums unified LOC per language bucket. */
export function scanRepoUnifiedLocUncached(root: string): UlocByLanguage {
  const repoRoot = gitRepoRoot(root);
  const out: UlocByLanguage = {};
  for (const rel of gitTrackedPaths(root)) {
    if (shouldSkipPathForUloc(repoRoot, rel)) continue;
    const lang = classifyPathForMetrics(rel);
    if (!lang) continue;
    const fp = join(repoRoot, rel);
    if (!existsSync(fp)) continue;
    try {
      const st = statSync(fp);
      if (!st.isFile() || st.size > MAX_METRICS_FILE_BYTES) continue;
    } catch {
      continue;
    }
    let data: string;
    try {
      const buf = readFileSync(fp);
      if (buf.includes(0)) continue;
      data = buf.toString("utf8");
    } catch {
      continue;
    }
    const loc = countUnifiedLocForFile(rel, data);
    if (loc <= 0) continue;
    out[lang] = (out[lang] ?? 0) + loc;
  }
  return out;
}

/** 📊️Scans tracked paths and sums on-disk byte size per language (includes binaries and large files). */
export function scanRepoSizeByLanguageUncached(root: string): UlocByLanguage {
  const repoRoot = gitRepoRoot(root);
  const out: UlocByLanguage = {};
  for (const rel of gitTrackedPaths(root)) {
    if (shouldSkipPathForUloc(repoRoot, rel)) continue;
    const lang = classifyPathForMetrics(rel);
    if (!lang) continue;
    const fp = join(repoRoot, rel);
    if (!existsSync(fp)) continue;
    try {
      const st = statSync(fp);
      if (!st.isFile() || st.size <= 0) continue;
      out[lang] = (out[lang] ?? 0) + st.size;
    } catch {
      continue;
    }
  }
  return out;
}

/** 📊️Repo uloc with per-HEAD cache under `.git/compose-metrics-cache.json`. */
export function scanRepoUnifiedLoc(root: string): UlocByLanguage {
  const cached = readMetricsCache(root);
  if (cached) return cached.uloc;
  const uloc = scanRepoUnifiedLocUncached(root);
  const size = scanRepoSizeByLanguageUncached(root);
  writeMetricsCache(root, uloc, size);
  return uloc;
}

/** 📊️Repo byte size per language with per-HEAD cache. */
export function scanRepoSizeByLanguage(root: string): UlocByLanguage {
  const cached = readMetricsCache(root);
  if (cached) return cached.size;
  const uloc = scanRepoUnifiedLocUncached(root);
  const size = scanRepoSizeByLanguageUncached(root);
  writeMetricsCache(root, uloc, size);
  return size;
}

/** 📊️Default metrics runner (uloc + size scans). */
export function createDefaultMetricsRunner(): MetricsRunner {
  return {
    countRepoUlocByLanguage: scanRepoUnifiedLoc,
    countRepoSizeByLanguage: scanRepoSizeByLanguage,
  };
}
