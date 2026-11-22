/** 🔗️ Probe of the validator maintainer: over the whole catalog (catalog asset with the author's `volumes` source merged over it, default pose) it measures, per scale, which body parts and structure parts touch nothing (true geometry: the kernel's triangles, exact triangle distances, tolerance a fifth of the scale's feature), what each loose group rests on, and which solids are below the feature size of the scale without being `slender`, `texture` or a thin role.
 *
 * Usage: `bun validator-connectivity-probe.ts [<category>/<slug> …] [--rules] [--force]`: one JSON line per asset in `🗑️generated/validator-connectivity/<probe|rules>.jsonl` (an asset whose source hash is already there is skipped). `--rules` records the findings of the real rules `volumes-connected` and `volumes-feature` instead of the raw measurement. `validator-connectivity-table.py` turns the lines into `validator-connectivity.md`.
 * @see ./validator-connectivity.md
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🔗️connected/🟦️.ts */
import { createHash } from "node:crypto";
import { appendFileSync, existsSync, mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { checkVolumes, contactPlanes, featureOf, authoredVolumes, solidsOf, VOLUME_LIMITS } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { groupGap, partsOf, touchingGroups } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🔗️connected/🟦️.ts";
import { SEAM_FEATURES } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts";
import { featureRule, smallestDimension } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { GENERATED, catalogEntries, loadAsset, loadSource, merged, type Json } from "./volumes-lib.ts";

const args = process.argv.slice(2);
const real = args.includes("--rules");
const wanted = args.filter((a) => !a.startsWith("--"));
const out = join(GENERATED, "validator-connectivity");
mkdirSync(out, { recursive: true });
const file = join(out, real ? "rules.jsonl" : "probe.jsonl");
const done = new Map<string, string>();
if (existsSync(file) && !args.includes("--force")) for (const line of readFileSync(file, "utf8").split("\n")) if (line) { const r = JSON.parse(line); done.set(r.asset, r.hash); }
const strip = ({ id: _id, of: _of, tone: _tone, region: _region, layer: _layer, member: _member, part: _part, anchor: _anchor, tags: _tags, ...rest }: Json): string => JSON.stringify(rest);

for (const entry of catalogEntries()) {
  const name = `${entry.category}/${entry.slug}`;
  if (wanted.length > 0 && !wanted.includes(name) && !wanted.includes(entry.slug)) continue;
  const source = loadSource(entry);
  const catalog = loadAsset(entry.path);
  if (catalog?.rig === undefined) continue;
  const asset = source === null ? catalog : merged(catalog, source);
  const hash = createHash("sha256").update(JSON.stringify(asset)).digest("hex").slice(0, 16);
  if (done.get(name) === hash) continue;
  const started = Date.now();
  const record: Json = { asset: name, hash, scales: {} };
  try {
    if (real) {
      record.findings = (checkVolumes(asset as never, { rules: ["volumes-connected", "volumes-feature"] as never }) as readonly Json[]).map((f) => ({ rule: f.rule, scale: f.scale, list: f.list, en: f.message.en, data: f.data }));
    } else {
      const finest = asset.volumes?.["50"];
      for (const volume of authoredVolumes(asset as never) as readonly Json[]) {
        const d = volume.denominator as number;
        const feature = featureOf(d);
        const tolerance = SEAM_FEATURES * feature;
        const state = evaluate(asset as never, { kind: "pose", pose: asset.presentation.defaultPose }, asset.representations[`${d}`]?.status === "authored" ? ({ kind: "scale", scale: { denominator: d } } as never) : undefined);
        const planes = contactPlanes(state, feature);
        const body = worldSolids(asset as never, state.bones as never, d, "body");
        const scale: Json = { feature: {} };
        for (const list of ["body", "structure"] as const) {
          const partOf = new Map((volume[list] as readonly Json[]).map((s) => [s.id, s.part] as const));
          const solids = (solidsOf(volume as never, list) as readonly Json[]).map((s) => (list === "structure" ? { ...s, part: partOf.get(s.id) } : s));
          const worlds = list === "body" ? body : worldSolids(asset as never, state.bones as never, d, list, body);
          const parts = partsOf(worlds, solids.map((s) => s.repeat?.count ?? 1));
          const groups = touchingGroups(parts, tolerance);
          const described = [];
          for (let g = 0; g < groups.count; g++) {
            const own = parts.filter((_, i) => groups.group[i] === g);
            const ids = [...new Set(own.map((p) => p.id))];
            const of = ids.map((id) => solids.find((s) => s.id === id)!);
            const grounded = own.some((p) => p.min[1] <= tolerance);
            const onPlane = own.some((p) => planes.some((plane) => { for (let i = 0; i < p.positions.length; i += 3) if (Math.abs((p.positions[i]! - plane.point[0]) * plane.normal[0] + (p.positions[i + 1]! - plane.point[1]) * plane.normal[1] + (p.positions[i + 2]! - plane.point[2]) * plane.normal[2]) <= tolerance) return true; return false; }));
            const gap = g === 0 ? { gapM: 0, nearest: -1 } : groupGap(parts, groups, g, 6 * feature);
            described.push({ parts: own.length, ids: ids.slice(0, 6), more: Math.max(0, ids.length - 6), grounded, onPlane, gapM: gap.gapM, nearest: gap.nearest < 0 ? null : parts[gap.nearest]!.id, kinds: [...new Set(of.map((s) => (list === "structure" ? s.part : s.role)))], slender: of.every((s) => s.slender === true), texture: of.every((s) => s.texture === true), bones: [...new Set(of.map((s) => s.bone))].slice(0, 4) });
          }
          scale[list] = { solids: solids.length, parts: parts.length, groups: described };
        }
        for (const list of ["body", "thermal", "structure"] as const) {
          const own = (volume[list] as readonly Json[]).filter((s) => typeof s.of !== "string");
          const fine = new Set(((finest?.[list] ?? []) as readonly Json[]).filter((s) => typeof s.of !== "string").map(strip));
          const under = own.filter((s) => !(VOLUME_LIMITS.thinRoles as readonly string[]).includes(s.role) && s.slender !== true && s.texture !== true && !(list === "thermal" && s.layer !== undefined) && smallestDimension(s as never) < feature - 1e-9);
          scale.feature[list] = { solids: own.length, under: under.length, carried: d === 50 ? 0 : under.filter((s) => fine.has(strip(s))).length, worst: under.map((s) => ({ id: s.id, mm: +(smallestDimension(s as never) * 1000).toFixed(2) })).sort((p, q) => p.mm - q.mm).slice(0, 5) };
        }
        scale.featureFindings = featureRule(asset as never, [d as never]).findings.length;
        record.scales[d] = scale;
      }
    }
  } catch (error) {
    record.error = (error as Error).message.split("\n")[0];
  }
  record.seconds = +((Date.now() - started) / 1000).toFixed(1);
  appendFileSync(file, `${JSON.stringify(record)}\n`);
  process.stdout.write(`${name} ${record.seconds}s${record.error ? ` ERROR ${record.error}` : ""}\n`);
}
