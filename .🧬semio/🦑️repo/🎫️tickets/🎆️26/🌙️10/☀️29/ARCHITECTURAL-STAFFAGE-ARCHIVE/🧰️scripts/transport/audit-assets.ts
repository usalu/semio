/** Runs the transport solver for every catalog asset that has emitters and reports data defects. Usage: bun audit-assets.ts <out.json> [assetId...] */
import { writeFileSync } from "node:fs";
import { loadCatalogFromFiles } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveTransport, respirationState, respirationSubjects, exhaledVolumeM3 } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/💨️transport/🟦️.ts";
import type { Asset } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import type { EvaluationQuery, Point } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔗️contracts/🟦️.ts";

const [out, ...only] = process.argv.slice(2);
const catalog = await loadCatalogFromFiles();
const SEL = { kind: "scale", scale: { denominator: 50 } } as const;
const BODY = new Set(["humans", "animals"]);

type Row = { asset: string; category: string; emitters: Record<string, { kind: string; works: boolean; parcels: number; maxFlowLs: number; maxExitMS: number; peakExcessK: number; note: string[] }>; issues: string[]; ms: number; breathVolumeErrorL: number | null };

function inside(p: Point, ring: readonly Point[]): boolean {
  let c = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const a = ring[i]!;
    const b = ring[j]!;
    if (a.y > p.y !== b.y > p.y && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) c = !c;
  }
  return c;
}

const queries = (asset: Asset): EvaluationQuery[] => [...asset.poses.slice(0, 6).map((p) => ({ kind: "pose", pose: p.id }) as EvaluationQuery), ...asset.clips.map((c) => ({ kind: "clip", clip: c.id }) as EvaluationQuery)];

const rows: Row[] = [];
for (const original of catalog.assets) {
  const asset: Asset = original.thermal.passengers === undefined ? { ...original, thermal: { ...original.thermal, passengers: [] } } : original;
  if (only.length > 0 && !only.includes(asset.id)) continue;
  if (asset.thermal.emitters.length === 0 && asset.thermal.respiration === undefined) continue;
  const row: Row = { asset: asset.id, category: asset.category, emitters: {}, issues: [], ms: 0, breathVolumeErrorL: null };
  const th = asset.thermal;
  const issue = (code: string, text: string): void => void row.issues.push(`${code}: ${text}`);
  if (original.thermal.passengers === undefined) issue("passengers-missing", "thermal.passengers is required (use [])");
  const anchorIds = new Set(asset.anchors.map((a) => a.id));
  const kinds = new Set(th.emitters.map((e) => e.kind));
  const subjects = respirationSubjects(asset);
  const owned = new Set(subjects.flatMap((s) => [s.respiration.noseEmitter, s.respiration.mouthEmitter].filter((x): x is string => x !== undefined)));
  if (BODY.has(asset.category) && th.respiration === undefined) issue("missing-respiration", "living body without thermal.respiration");
  if (BODY.has(asset.category) && ![...kinds].includes("breath")) issue("no-breath-emitter", "living body without breath emitters");
  if (!BODY.has(asset.category) && (th.respiration !== undefined || kinds.has("breath"))) issue("breath-on-non-body", "respiration or breath emitter on a non-living asset");
  if (asset.category === "vegetation" && th.emitters.some((e) => e.kind !== "transpiration")) issue("vegetation-non-transpiration", "vegetation emitter other than transpiration");
  for (const e of th.emitters) {
    if (!anchorIds.has(e.anchor)) issue("anchor-missing", `${e.id}: anchor ${e.anchor}`);
    for (const a of e.anchors ?? []) if (!anchorIds.has(a)) issue("anchor-missing", `${e.id}: anchors[] ${a}`);
    if (e.kind === "breath" && !owned.has(e.id)) issue("orphan-breath", `${e.id} not referenced by any respiration`);
    if (e.kind === "transpiration" && e.region !== undefined && !th.regions.some((r) => r.id === e.region)) issue("region-missing", `${e.id}: region ${e.region}`);
    if (e.kind === "transpiration" && (e.anchors ?? []).length === 0) issue("transpiration-single-anchor", `${e.id}: no anchors[] (distributed sources need several)`);
    if ((e.kind === "exhaust" || e.kind === "vent" || e.kind === "flue" || e.kind === "fan") && e.operatingKey !== undefined && !th.operating.some((o) => o.id === e.operatingKey)) issue("operating-key-missing", `${e.id}: ${e.operatingKey}`);
    if ((e.kind === "exhaust" || e.kind === "vent") && /child|baby|toddler|infant|passenger/i.test(`${e.id} ${e.title.en}`) && (th.passengers ?? []).length === 0) issue("passenger-stand-in", `${e.id} (${e.kind}) stands in for a living passenger; use ThermalModel.passengers with respiration (ruling 4)`);
    if (e.kind !== "breath" && e.kind !== "transpiration" && /breath|nose|mouth/i.test(e.id) && !/child/i.test(e.id)) issue("breath-name-nonbreath", `${e.id} is ${e.kind}`);
    row.emitters[e.id] = { kind: e.kind, works: false, parcels: 0, maxFlowLs: 0, maxExitMS: 0, peakExcessK: 0, note: [] };
  }
  for (const s of subjects) {
    for (const [k, id] of [["noseEmitter", s.respiration.noseEmitter], ["mouthEmitter", s.respiration.mouthEmitter]] as const) if (id !== undefined && th.emitters.find((e) => e.id === id)?.kind !== "breath") issue("respiration-emitter-invalid", `${k} ${id} is not a breath emitter`);
    if (s.respiration.patterns.length === 0) issue("no-patterns", "respiration without patterns");
    for (const m of s.respiration.motion) if (!asset.rig.bones.some((b) => b.id === m.bone)) issue("motion-bone-missing", m.bone);
    if (s.respiration.motion.length === 0) issue("no-chest-motion", "respiration.motion is empty (no chest/abdomen motion)");
    const first = s.respiration.patterns[0]!;
    const q: EvaluationQuery = asset.poses.length > 0 ? { kind: "pose", pose: asset.poses[0]!.id } : { kind: "clip", clip: asset.clips[0]!.id };
    const st = respirationState(asset, { ...q, timeS: 0.4 } as EvaluationQuery, s.subject)!;
    const T = 60 / st.rateBpm;
    const vol = exhaledVolumeM3(asset, { ...q, timeS: 0 } as EvaluationQuery, 0, 3 * T, s.subject) * 1000;
    row.breathVolumeErrorL = Math.max(row.breathVolumeErrorL ?? 0, Math.abs(vol - 3 * st.tidalVolumeL));
  }
  const start = performance.now();
  const qs = queries(asset);
  if (qs.length === 0) issue("no-queries", "no poses or clips");
  for (const base of qs) {
    for (const t of [4.37, 9.11]) {
      const query = { ...base, timeS: t } as EvaluationQuery;
      let field;
      try {
        field = solveTransport(asset, query, asset.thermal.environment, { streamlines: false });
      } catch (error) {
        issue("solve-error", `${JSON.stringify(base)}: ${(error as Error).message}`);
        continue;
      }
      const state = evaluate(asset, query, SEL);
      for (const e of field.emitters) {
        const r = row.emitters[e.emitter]!;
        r.maxFlowLs = Math.max(r.maxFlowLs, e.volumeFlowM3S * 1000);
        r.maxExitMS = Math.max(r.maxExitMS, e.exitVelocityMS);
        const n = field.parcels.filter((p) => p.emitter === e.emitter);
        r.parcels = Math.max(r.parcels, n.length);
        r.peakExcessK = Math.max(r.peakExcessK, ...n.map((p) => p.excessK), 0);
        if (!Number.isFinite(e.at.x + e.at.y + e.directionRad)) r.note.push("non-finite emitter state");
        if (e.kind === "breath") {
          const def = th.emitters.find((d) => d.id === e.emitter)!;
          const placed = state.anchors[def.anchor];
          const bone = placed?.anchor.bone;
          const rings = state.shapes.elevation.filter((s) => s.shape.bone === bone && s.closed).flatMap((s) => s.rings);
          if (placed !== undefined && rings.length > 0) {
            const d = { x: Math.cos(e.directionRad), y: Math.sin(e.directionRad) };
            const probe = { x: e.at.x + 0.03 * d.x, y: e.at.y + 0.03 * d.y };
            if (rings.some((ring) => inside(probe, ring))) r.note.push(`direction into the ${bone} outline (t=${t})`);
            const xs = rings.flatMap((ring) => ring.map((p) => p.x));
            const centre = (Math.min(...xs) + Math.max(...xs)) / 2;
            if ((e.at.x - centre) * d.x < -1e-6 && Math.abs(d.x) > 0.2) r.note.push(`breath points towards the back of the ${bone} (anchor at ${(e.at.x - centre).toFixed(3)} m from centre, dir x ${d.x.toFixed(2)})`);
            const inAny = rings.some((ring) => inside(e.at, ring));
            if (!inAny) {
              const ys = rings.flatMap((ring) => ring.map((p) => p.y));
              const x0 = Math.min(...xs), x1 = Math.max(...xs), y0 = Math.min(...ys), y1 = Math.max(...ys);
              const dist = Math.max(x0 - e.at.x, 0, e.at.x - x1, 0) + Math.max(y0 - e.at.y, 0, e.at.y - y1, 0);
              if (dist > 0.05) r.note.push(`anchor ${dist.toFixed(2)} m away from the ${bone} outline`);
            }
          }
          if (e.at.y < 0.05) r.note.push(`anchor at y=${e.at.y.toFixed(3)} m (on the ground)`);
        }
      }
    }
  }
  row.ms = Math.round(performance.now() - start);
  for (const [id, r] of Object.entries(row.emitters)) {
    r.note = [...new Set(r.note)];
    r.works = r.parcels > 0 && r.note.length === 0;
    if (r.parcels === 0) r.note.push("never emitted a parcel in any pose/clip at t = 4.37 s / 9.11 s");
    if (r.note.length > 0) issue("emitter", `${id} (${r.kind}): ${r.note.join("; ")}`);
  }
  rows.push(row);
  console.log("[DEBUG]", asset.id, row.ms, "ms", row.issues.length, "issues");
}
writeFileSync(out!, JSON.stringify(rows, null, 1));
