// 🧾 Read-only registry inspection: catalog manifest rows against folders, collections and cast manifest viewpoints.
import { readFileSync, readdirSync, statSync, existsSync } from "node:fs";
import { join } from "node:path";

const [manifestPath, catalogDir, castPath] = process.argv.slice(2);
const read = (p) => JSON.parse(readFileSync(p, "utf8"));
const isDir = (p) => statSync(p).isDirectory();
const names = (p) => (existsSync(p) ? readdirSync(p) : []);
const jsonFiles = (p) => names(p).filter((n) => n.endsWith(".json") && !isDir(join(p, n)));

const manifest = read(manifestPath);
const folderOf = new Map(manifest.categories.map((c) => [c.id, c.folder]));
const problems = [];
const ids = new Map();
const rows = [];

for (const e of manifest.entries) {
  ids.set(e.id, (ids.get(e.id) ?? 0) + 1);
  const folder = folderOf.get(e.category);
  if (folder === undefined) {
    problems.push(`row ${e.id}: unknown category ${e.category}`);
    continue;
  }
  const dir = join(catalogDir, folder, e.id);
  let state = "ok";
  if (!existsSync(dir) || !isDir(dir)) state = "missing folder";
  else if (jsonFiles(dir).length !== 1) state = `json files: ${jsonFiles(dir).length}`;
  const reps = e.representations ?? {};
  const repStr = ["50", "100", "200", "500"].map((k) => `${k}=${reps[k] ?? "-"}`).join(" ");
  const extra = Object.keys(e).filter((k) => !["id", "category", "title", "heightM", "widthM", "poses", "clips", "representations", "symbols", "analytics", "exports", "distinction", "completion"].includes(k));
  rows.push({ id: e.id, category: e.category, completion: e.completion, reps: repStr, exports: e.exports.length, analytics: e.analytics.join("+") || "-", symbols: e.symbols.length, poses: e.poses.length, clips: e.clips.length, extra: extra.join(",") || "-", folderState: state });
  if (state !== "ok") problems.push(`row ${e.id}: ${state}`);
}

const orphanFolders = [];
for (const c of manifest.categories) {
  const base = join(catalogDir, c.folder);
  for (const n of names(base)) {
    if (!isDir(join(base, n))) continue;
    if (!manifest.entries.some((e) => e.id === n && e.category === c.id)) orphanFolders.push(`${c.folder}/${n}`);
  }
}
const categoryFolders = new Set(manifest.categories.map((c) => c.folder));
const topLevelOther = names(catalogDir).filter((n) => isDir(join(catalogDir, n)) && !categoryFolders.has(n));
const dupIds = [...ids].filter(([, n]) => n > 1).map(([id]) => id);
const entryIds = new Set(manifest.entries.map((e) => e.id));
const collectionRefs = manifest.collections.flatMap((c) => c.assets.filter((a) => !entryIds.has(a)).map((a) => `${c.id}:${a}`));

const byCat = {};
for (const r of rows) byCat[r.category] = (byCat[r.category] ?? 0) + 1;
const byCompletion = {};
for (const r of rows) byCompletion[r.completion] = (byCompletion[r.completion] ?? 0) + 1;
const extraKeys = rows.filter((r) => r.extra !== "-");

console.log(`[counts] rows=${rows.length} categories=${manifest.categories.length} collections=${manifest.collections.length} duplicateIds=${dupIds.length}`);
console.log(`[byCategory] ${JSON.stringify(byCat)}`);
console.log(`[byCompletion] ${JSON.stringify(byCompletion)}`);
console.log(`[problems] ${problems.length ? problems.join(" | ") : "none"}`);
console.log(`[orphanFolders] ${orphanFolders.length ? orphanFolders.join(", ") : "none"}`);
console.log(`[topLevelNonCategoryFolders] ${topLevelOther.join(", ")}`);
console.log(`[duplicateIds] ${dupIds.join(", ") || "none"}`);
console.log(`[collectionRefsUnresolved] ${collectionRefs.join(", ") || "none"}`);
console.log(`[extraRowKeys] ${extraKeys.length}`);
console.log("[rows] id | category | completion | reps | exports | analytics | symbols | poses | clips | folder");
for (const r of rows) console.log(`${r.id} | ${r.category} | ${r.completion} | ${r.reps} | ${r.exports} | ${r.analytics} | ${r.symbols} | ${r.poses} | ${r.clips} | ${r.folderState}`);

if (castPath) {
  const casts = Object.entries(read(castPath).casts).map(([id, c]) => ({ id, ...c }));
  const byVp = {};
  for (const c of casts) {
    const k = c.viewpoint ?? "(absent)";
    byVp[k] = (byVp[k] ?? 0) + 1;
  }
  const noVp = casts.filter((c) => c.viewpoint === undefined);
  const bothPoseClip = casts.filter((c) => c.pose !== undefined && c.clip !== undefined).map((c) => c.id);
  const neither = casts.filter((c) => c.pose === undefined && c.clip === undefined).map((c) => c.id);
  const unknownAsset = casts.filter((c) => !entryIds.has(c.asset)).map((c) => `${c.id}:${c.asset}`);
  const byView = {};
  for (const c of casts) byView[c.view] = (byView[c.view] ?? 0) + 1;
  console.log(`[casts] total=${casts.length} byViewpoint=${JSON.stringify(byVp)} byView=${JSON.stringify(byView)}`);
  console.log(`[castPoseClipProblems] bothPoseAndClip=${bothPoseClip.join(",") || "none"} neither=${neither.join(",") || "none"}`);
  console.log(`[castAssetsNotInCatalog] ${unknownAsset.length} : ${unknownAsset.join(", ") || "none"}`);
  console.log("[castsWithoutViewpoint] id | asset | pose/clip | view | theme");
  for (const c of noVp) console.log(`${c.id} | ${c.asset} | ${c.pose !== undefined ? "pose " + c.pose : "clip " + c.clip} | ${c.view} | ${c.theme}`);
}
