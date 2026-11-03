// 🔎 Read-only session-2 comparison of every manifest row with its asset file: scale status, dimensions (1 mm), poses, clips and symbol ids.
import { readFileSync, readdirSync, statSync, existsSync } from "node:fs";
import { join } from "node:path";

const [manifestPath, catalogDir] = process.argv.slice(2);
const manifest = JSON.parse(readFileSync(manifestPath, "utf8"));
const folderOf = new Map(manifest.categories.map((c) => [c.id, c.folder]));
const jsonIn = (p) => readdirSync(p).filter((n) => n.endsWith(".json") && statSync(join(p, n)).isFile());
const out = { rows: manifest.entries.length, statusMismatch: [], dimMismatch: [], poseMismatch: [], clipMismatch: [], symbolMismatch: [], unavailableNoReason: [], missingFile: [], oldPlanSymbols: [], noDimensions: [] };
const SCALES = ["50", "100", "200", "500"];

for (const e of manifest.entries) {
  const dir = join(catalogDir, folderOf.get(e.category), e.id);
  if (!existsSync(dir) || jsonIn(dir).length !== 1) { out.missingFile.push(e.id); continue; }
  const asset = JSON.parse(readFileSync(join(dir, jsonIn(dir)[0]), "utf8")).asset;
  for (const s of SCALES) {
    const r = asset.representations?.[s];
    if (!r) { out.statusMismatch.push(`${e.id}@${s}: no band in file`); continue; }
    if (r.status !== e.representations[s]) out.statusMismatch.push(`${e.id}@${s}: manifest ${e.representations[s]}, file ${r.status}`);
    if (r.status === "unavailable" && !r.reason) out.unavailableNoReason.push(`${e.id}@${s}`);
  }
  const d = asset.dimensions;
  if (!d) out.noDimensions.push(e.id);
  else {
    if (Math.abs(d.heightM - e.heightM) > 0.001) out.dimMismatch.push(`${e.id} heightM manifest ${e.heightM} file ${d.heightM}`);
    if (Math.abs(d.widthM - e.widthM) > 0.001) out.dimMismatch.push(`${e.id} widthM manifest ${e.widthM} file ${d.widthM}`);
  }
  const filePoses = (asset.poses ?? []).filter((p) => p.key === undefined).map((p) => p.id).sort();
  const manPoses = [...e.poses].sort();
  if (JSON.stringify(filePoses) !== JSON.stringify(manPoses)) out.poseMismatch.push(`${e.id}: file-only ${filePoses.filter((x) => !manPoses.includes(x))} manifest-only ${manPoses.filter((x) => !filePoses.includes(x))}`);
  const fileClips = (asset.clips ?? []).map((c) => c.id).sort();
  const manClips = [...e.clips].sort();
  if (JSON.stringify(fileClips) !== JSON.stringify(manClips)) out.clipMismatch.push(`${e.id}: file-only ${fileClips.filter((x) => !manClips.includes(x))} manifest-only ${manClips.filter((x) => !fileClips.includes(x))}`);
  const fileSyms = (asset.symbols ?? []).map((s) => s.id).sort();
  const manSyms = [...e.symbols].sort();
  if (JSON.stringify(fileSyms) !== JSON.stringify(manSyms)) out.symbolMismatch.push(`${e.id}: file ${fileSyms} manifest ${manSyms}`);
  for (const id of fileSyms) if (id.endsWith("-plan")) out.oldPlanSymbols.push(`${e.id}: ${id}`);
}
out.counts = Object.fromEntries(Object.entries(out).filter(([, v]) => Array.isArray(v)).map(([k, v]) => [k, v.length]));
console.log(JSON.stringify(out, null, 1));
