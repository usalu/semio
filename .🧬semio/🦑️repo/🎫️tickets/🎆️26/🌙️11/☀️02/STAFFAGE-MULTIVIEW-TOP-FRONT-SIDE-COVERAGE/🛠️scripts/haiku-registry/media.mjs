// 🎞️ Read-only inventory of the staffage cast cache: key folders, cast folders, frames, indexes, locks and dates.
import { readdirSync, statSync, readFileSync, existsSync } from "node:fs";
import { join } from "node:path";

const [cacheRoot, manifestPath] = process.argv.slice(2);
const iso = (ms) => new Date(ms).toISOString().slice(0, 10);
const span = (items) => {
  if (!items.length) return "none";
  const t = items.map((i) => i.mtime).sort((a, b) => a - b);
  return `oldest ${iso(t[0])}, newest ${iso(t[t.length - 1])}`;
};
const keys = readdirSync(cacheRoot).filter((n) => statSync(join(cacheRoot, n)).isDirectory());
const tmp = keys.filter((k) => k.startsWith(".tmp-"));
const real = keys.filter((k) => !k.startsWith(".tmp-"));
const castCounts = {};
const castKeys = {};
const frames = [];
const indexes = [];
const locks = [];
const keyInfo = [];
const otherFiles = {};
let castFolders = 0;
let emptyKeys = 0;
let keysWithoutViewpoint = 0;
for (const k of real) {
  const kp = join(cacheRoot, k);
  const kst = statSync(kp);
  const casts = readdirSync(kp).filter((n) => statSync(join(kp, n)).isDirectory());
  const files = readdirSync(kp).filter((n) => statSync(join(kp, n)).isFile());
  for (const f of files) if (f.startsWith("lock-")) locks.push({ k, f, mtime: statSync(join(kp, f)).mtimeMs });
  if (casts.length === 0) emptyKeys++;
  let hasVp = true;
  for (const c of casts) {
    castFolders++;
    castCounts[c] = (castCounts[c] ?? 0) + 1;
    (castKeys[c] ??= []).push(k);
    const cp = join(kp, c);
    const entries = readdirSync(cp);
    for (const e of entries) {
      const st = statSync(join(cp, e));
      if (e.endsWith(".svg")) frames.push({ mtime: st.mtimeMs, k, c });
      else if (e === "🔣️.json") {
        const idx = JSON.parse(readFileSync(join(cp, e), "utf8"));
        const has = "viewpoint" in idx;
        indexes.push({ mtime: st.mtimeMs, k, c, has, view: idx.view, viewpoint: idx.viewpoint });
        if (!has) hasVp = false;
      } else {
        const ext = e.includes(".") ? e.slice(e.lastIndexOf(".")) : "(none)";
        otherFiles[ext] = (otherFiles[ext] ?? 0) + 1;
      }
    }
  }
  if (!hasVp) keysWithoutViewpoint++;
  keyInfo.push({ k, casts: casts.length, mtime: kst.mtimeMs });
}

const withVp = indexes.filter((i) => i.has).length;
const withoutVp = indexes.length - withVp;
const perCastVersions = Object.values(castKeys).map((ks) => ks.length);
const multi = Object.entries(castKeys).filter(([, ks]) => ks.length > 1).length;
const manifest = manifestPath ? JSON.parse(readFileSync(manifestPath, "utf8")).casts : {};
const manifestIds = Object.keys(manifest);
const castsInCache = Object.keys(castCounts);
const notInManifest = castsInCache.filter((c) => !(c in manifest));
const manifestNotCached = manifestIds.filter((c) => !(c in castCounts));

const dist = (arr) => arr.reduce((m, v) => ((m[v] = (m[v] ?? 0) + 1), m), {});
const keyDates = {};
for (const ki of keyInfo) keyDates[iso(ki.mtime)] = (keyDates[iso(ki.mtime)] ?? 0) + 1;
const frameDates = {};
for (const f of frames) frameDates[iso(f.mtime)] = (frameDates[iso(f.mtime)] ?? 0) + 1;

console.log(`[keys] real=${real.length} tmp=${tmp.length} emptyKeys=${emptyKeys} keysWithAnyIndexLacking=${keysWithoutViewpoint}`);
console.log(`[keyDateDist] ${JSON.stringify(Object.fromEntries(Object.entries(keyDates).sort()))}`);
console.log(`[keyDateSpan] ${span(keyInfo.map((k) => ({ mtime: k.mtime })))}`);
console.log(`[tmpKeys] ${tmp.join(", ") || "none"}`);
console.log(`[castFolders] ${castFolders} distinctCastNames=${castsInCache.length} castNamesInManyKeys=${multi}`);
console.log(`[castNamesPerKeyCountDist] ${JSON.stringify(dist(perCastVersions))}`);
console.log(`[castsInCacheNotInManifest] ${notInManifest.length}: ${notInManifest.join(", ") || "none"}`);
console.log(`[manifestCastsNotInCache] ${manifestNotCached.length}: ${manifestNotCached.join(", ") || "none"}`);
console.log(`[indexes] total=${indexes.length} withViewpointKey=${withVp} withoutViewpointKey=${withoutVp}`);
console.log(`[indexViewCounts] ${JSON.stringify(dist(indexes.map((i) => i.view)))}`);
console.log(`[indexSpan] ${span(indexes)}`);
console.log(`[frames] svg=${frames.length} ${span(frames)}`);
console.log(`[frameDateDist] ${JSON.stringify(Object.fromEntries(Object.entries(frameDates).sort()))}`);
console.log(`[locks] ${locks.length} ${span(locks)}`);
console.log(`[otherFileExt] ${JSON.stringify(otherFiles)}`);
