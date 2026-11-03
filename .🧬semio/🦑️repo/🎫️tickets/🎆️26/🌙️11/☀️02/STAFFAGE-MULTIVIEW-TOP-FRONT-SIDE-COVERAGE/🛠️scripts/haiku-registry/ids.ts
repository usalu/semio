// 🔎 Read-only id audit: pose and clip ids of each asset file against its manifest row (schema: asset.poses[].id with key flag, asset.clips[].{id, durationS, loop}).
import { readFileSync, readdirSync, statSync, existsSync, writeFileSync, mkdirSync } from "node:fs";
import { join, dirname } from "node:path";

const [catalogDir, manifestPath, outPath] = process.argv.slice(2);

type Pose = { id: string; key?: boolean };
type Clip = { id: string; durationS: number; loop: boolean };
type Row = { id: string; category: string; poses: string[]; clips: string[] };
type Manifest = { categories: { id: string; folder: string }[]; entries: Row[] };

const manifest = JSON.parse(readFileSync(manifestPath, "utf8")) as Manifest;
const folderOf = new Map(manifest.categories.map((c) => [c.id, c.folder]));

const dupes = (ids: string[]): string[] => [...new Set(ids.filter((id, i) => ids.indexOf(id) !== i))];
const minus = (a: string[], b: string[]): string[] => [...new Set(a.filter((x) => !b.includes(x)))];

const assets = [];
for (const row of manifest.entries) {
  const folder = folderOf.get(row.category);
  const dir = join(catalogDir, folder ?? "", row.id);
  const problems: string[] = [];
  if (folder === undefined || !existsSync(dir) || !statSync(dir).isDirectory()) {
    problems.push("no asset folder");
    assets.push({ id: row.id, category: row.category, problems, pose: null });
    continue;
  }
  const files = readdirSync(dir).filter((n) => n.endsWith(".json") && statSync(join(dir, n)).isFile());
  if (files.length !== 1) {
    problems.push(`json files: ${files.length}`);
    assets.push({ id: row.id, category: row.category, problems, pose: null });
    continue;
  }
  const doc = JSON.parse(readFileSync(join(dir, files[0]!), "utf8")) as { asset: { id: string; poses: Pose[]; clips: Clip[] } };
  const file = doc.asset;
  if (file.id !== row.id) problems.push(`asset id ${file.id} differs from row ${row.id}`);

  const allPoseIds = file.poses.map((p) => p.id);
  const namedPoseIds = file.poses.filter((p) => p.key !== true).map((p) => p.id);
  const keyPoseIds = file.poses.filter((p) => p.key === true).map((p) => p.id);
  const clipIds = file.clips.map((c) => c.id);
  const manifestPoses = row.poses;
  const manifestClips = row.clips;

  const pose = {
    namedCount: namedPoseIds.length,
    keyCount: keyPoseIds.length,
    inFileNotManifest: minus(namedPoseIds, manifestPoses),
    inManifestNotFile: minus(manifestPoses, namedPoseIds),
    duplicateInFile: dupes(allPoseIds),
    duplicateInManifest: dupes(manifestPoses),
  };
  const clip = {
    count: clipIds.length,
    list: file.clips.map((c) => ({ id: c.id, durationS: c.durationS, loop: c.loop })),
    inFileNotManifest: minus(clipIds, manifestClips),
    inManifestNotFile: minus(manifestClips, clipIds),
    duplicateInFile: dupes(clipIds),
    duplicateInManifest: dupes(manifestClips),
  };
  const mismatch = pose.inFileNotManifest.length + pose.inManifestNotFile.length + clip.inFileNotManifest.length + clip.inManifestNotFile.length + pose.duplicateInFile.length + pose.duplicateInManifest.length + clip.duplicateInFile.length + clip.duplicateInManifest.length;
  assets.push({ id: row.id, category: row.category, problems, pose, clip, mismatchCount: mismatch });
}

const withMismatch = assets.filter((a: any) => a.mismatchCount > 0 || a.problems.length > 0);
const summary = {
  assets: assets.length,
  assetsWithMismatch: withMismatch.length,
  namedPoseMismatchAssets: assets.filter((a: any) => a.pose && (a.pose.inFileNotManifest.length || a.pose.inManifestNotFile.length)).length,
  clipMismatchAssets: assets.filter((a: any) => a.clip && (a.clip.inFileNotManifest.length || a.clip.inManifestNotFile.length)).length,
  duplicateAssets: assets.filter((a: any) => a.pose && (a.pose.duplicateInFile.length || a.pose.duplicateInManifest.length || a.clip.duplicateInFile.length || a.clip.duplicateInManifest.length)).length,
  totalNamedPoses: assets.reduce((s: number, a: any) => s + (a.pose?.namedCount ?? 0), 0),
  totalClips: assets.reduce((s: number, a: any) => s + (a.clip?.count ?? 0), 0),
};

const result = { summary, assets };
mkdirSync(dirname(outPath), { recursive: true });
writeFileSync(outPath, `${JSON.stringify(result, null, 2)}\n`);

console.log(`[summary] ${JSON.stringify(summary)}`);
console.log("| asset | poses (named/key) | clips | in file not manifest (poses) | in manifest not file (poses) | in file not manifest (clips) | in manifest not file (clips) | duplicates | problems |");
console.log("|---|---|---|---|---|---|---|---|---|");
for (const a of assets as any[]) {
  if (!a.pose) {
    console.log(`| ${a.id} | - | - | - | - | - | - | - | ${a.problems.join("; ")} |`);
    continue;
  }
  const dup = [...a.pose.duplicateInFile, ...a.pose.duplicateInManifest, ...a.clip.duplicateInFile, ...a.clip.duplicateInManifest];
  console.log(`| ${a.id} | ${a.pose.namedCount}/${a.pose.keyCount} | ${a.clip.count} | ${a.pose.inFileNotManifest.join(", ") || "-"} | ${a.pose.inManifestNotFile.join(", ") || "-"} | ${a.clip.inFileNotManifest.join(", ") || "-"} | ${a.clip.inManifestNotFile.join(", ") || "-"} | ${dup.join(", ") || "-"} | ${a.problems.join("; ") || "-"} |`);
}
