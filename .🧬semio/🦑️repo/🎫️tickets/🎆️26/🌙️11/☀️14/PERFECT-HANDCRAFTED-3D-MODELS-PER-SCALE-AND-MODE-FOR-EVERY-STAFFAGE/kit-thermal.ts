/** 🌡️ Builds the thermal list of one scale from its body, measures the exposed area per region with the product check and proposes (or applies) the `areaM2` root corrections.
 *
 * Usage: `bun kit-thermal.ts <category>/<slug> --scale N [--map region=id+id,region=id] [--rounds 8] [--write] [--apply-area] [--section-faces] [--source file]`
 * - binding: a body solid is bound to every thermal region whose drawn shapes (same bone, this scale's own drawings, any view) cover at least half of the solid's projection; the 1:50 seed bindings of the old `volume` win for solids that still carry their old id; `--map` overrides. Solids bound to nothing and regions without solid are listed.
 * - layers (B2): at 1:50 a region with several layers becomes nested shells, outermost layer = the body solid (or its area-fitted copy), inner layers = `shellOffset` by the cumulative real `thicknessM` of the layers outside (else resistance x declared conductivity, else a 2 mm shell and a note for `decisions`). Coarser scales: one surface solid per bound solid. Regions with 0 or 1 layer: one solid.
 * - hidden regions (B4): proposed as an ellipsoid inside the body with the region's surface area, centred in the body box, flagged `needs-author` (position).
 * - emitters (B3): a capsule jet (diameter at least 1.25 features) or, for plumes and transpiration (diameter over 5 % of the height), a flat disc, at the emitter's anchor(s), tagged `[emitter, <id>]`, region = the emitter's region else the best-covering region of the anchor's bone; direction from `directionDeg` (0 = up, positive turns towards -x).
 * - C18: `areaM2` of body-model regions (anything with a `body` block, kinds skin, clothing, hair, fur, feather) is physiological data and is never proposed or changed (like C16 regions); their tolerance is 30 % at 1:50 and 1:100 and a larger deviation is printed as a model finding naming the bound solids and the solids of other regions that overlap or bury the surface.
 * - area (C1, C15, C16): thermal solids are never scaled or shrunk to hit an area; the region's coverage is what the drawings show. The exposed area per region is measured with the product `volumes-thermal-coverage` and reported (per region at 1:50 and 1:100, total at 1:200 and 1:500). `areaM2` corrections are proposed ONLY from the 1:50 model, NEVER for regions of thermal model `leaf` or kind foliage/fur/feather/hair (exchange area, C16); `--apply-area` writes a correction only when new/old is within a factor 1.5, larger discrepancies are appended unapplied to `T/area-review.md`.
 * - `--section-faces` marks the in-plane edge faces of thin boxes of `wall` regions as section faces (C1) in the proposal.
 * `--write` saves `thermal` of this scale. The proposed `decisions` lines are printed, never written.
 * @see ./kit.md */
import { appendFileSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { rulingMeasurements } from "./volumes-rulings.ts";
import { TICKET, Mask, dirToLocal, extentOf, installCancellation } from "./volumes-lib.ts";
import { Fitter } from "./kit-fitter.ts";
import { VIEWS, capsuleOf, clone, drawn, ellipsoidOf, flag, has, log, openCtx, positional, saveLists, scaledSolid, shellOffset, writeOut, type Ctx, type Json } from "./kit-lib.ts";

const args = process.argv.slice(2);
const spec = positional(args, ["--scale", "--map", "--rounds", "--source"]);
const scale = Number(flag(args, "--scale"));
if (spec === undefined || ![50, 100, 200, 500].includes(scale)) {
  process.stderr.write("usage: kit-thermal.ts <category>/<slug> --scale N [--map region=id+id] [--write] [--apply-area]\n");
  process.exit(2);
}
installCancellation();
const c = openCtx(spec, scale, flag(args, "--source"));
const body: Json[] = c.vol.body;
if (body.length === 0) {
  process.stderr.write(`1:${scale} has no body; run kit-fit.ts first\n`);
  process.exit(2);
}
const regions: Json[] = c.m.thermal.regions;
const byId = new Map(body.map((s) => [s.id, s]));
const notes: string[] = [];

function bind(): Map<string, string[]> {
  const fit = new Fitter(c, 800);
  const out = new Map<string, string[]>(regions.map((r) => [r.id, []]));
  const manual = new Map<string, string[]>((flag(args, "--map") ?? "").split(",").filter(Boolean).map((kv) => [kv.split("=")[0]!, kv.split("=")[1]!.split("+")]));
  const old = new Map<string, string[]>();
  for (const s of c.raw.volume?.solids ?? []) if (scale === 50) old.set(s.id, [...(s.region ? [s.region] : []), ...(s.regions ?? [])]);
  const masks = new Map<string, Map<string, Mask>>();
  const maskOf = (v: (typeof VIEWS)[number], region: string, bone?: string): Mask => {
    const key = `${v}|${region}|${bone ?? ""}`;
    const g = fit.views.get(v)!;
    let per = masks.get(v);
    if (per === undefined) masks.set(v, (per = new Map()));
    let m = per.get(key);
    if (m === undefined) {
      m = new Mask(g.box, g.cell);
      for (const sh of drawn(c, v)) if (sh.region === region && (bone === undefined || sh.bone === bone)) m.fill(sh.rings);
      per.set(key, m);
    }
    return m;
  };
  const unbound: string[] = [];
  const score = new Map<string, number>();
  const scores = new Map<string, Map<string, number>>();
  for (const s of body) {
    if (s.slender === true || s.role === "texture") continue;
    const rs: string[] = [];
    const forced = [...manual.entries()].filter(([, ids]) => ids.includes(s.id)).map(([r]) => r);
    if (forced.length > 0) rs.push(...forced);
    else if (old.get(s.id)?.length) rs.push(...old.get(s.id)!.filter((r) => out.has(r)));
    else {
      const cells = fit.cellsOf(fit.mesh(s));
      for (const r of regions) {
        let best = 0;
        for (const v of VIEWS) {
          const l = cells.get(v)!;
          if (l.length === 0) continue;
          for (const bone of [s.bone, undefined]) {
            const m = maskOf(v, r.id, bone);
            let inside = 0;
            for (const k of l) inside += m.cells[k]!;
            best = Math.max(best, bone === undefined ? 0.8 * inside / l.length : inside / l.length);
          }
        }
        score.set(r.id, best);
        if (best >= 0.5) rs.push(r.id);
      }
      if (scale >= 100 && rs.length > 1) {
        const top = rs.reduce((a, b) => ((score.get(b) ?? 0) > (score.get(a) ?? 0) ? b : a));
        rs.length = 0;
        rs.push(top);
      }
      scores.set(s.id, new Map(score));
      score.clear();
    }
    if (rs.length === 0) unbound.push(s.id);
    for (const r of rs) out.get(r)!.push(s.id);
  }
  for (const r of regions) {
    if (r.hidden || out.get(r.id)!.length > 0) continue;
    const rb = new Set<string>();
    for (const k of Object.keys(c.m.representations)) for (const v of VIEWS) for (const sh of c.m.representations[k].geometry?.[v] ?? []) if (sh.region === r.id && sh.bone) rb.add(sh.bone);
    const near = body.filter((sd) => rb.has(sd.bone) && sd.slender !== true && sd.role !== "texture").slice(0, 2).map((sd) => sd.id);
    const cand = near.length > 0 && !scores.size ? [] : [...scores.entries()].map(([sid, m]) => [sid, m.get(r.id) ?? 0] as const).filter(([, v]) => v > 0.15).sort((a, b) => b[1] - a[1]).slice(0, 2);
    for (const [sid] of cand) out.get(r.id)!.push(sid);
    if (cand.length === 0 && near.length > 0) {
      out.get(r.id)!.push(...near);
      notes.push(`region ${r.id} is not drawn at 1:${scale}: bound to ${near.join(", ")} on the bone(s) where other scales draw it`);
    }
    if (cand.length > 0) notes.push(`region ${r.id} had no solid of its own at 1:${scale}: bound to the best-overlapping solids ${cand.map(([i]) => i).join(", ")} (shared with another region)`);
  }
  if (unbound.length > 0) notes.push(`unbound body solids (no thermal region drawn over them at 1:${scale}): ${unbound.join(", ")}`);
  return out;
}

const thickness = (layer: Json, region: Json): number => {
  if (typeof layer.thicknessM === "number") return layer.thicknessM;
  const k = region.conductivityWMK ?? layer.conductivityWMK;
  if (typeof layer.resistanceM2KW === "number" && typeof k === "number") return layer.resistanceM2KW * k;
  notes.push(`layer ${region.id}/${layer.id} has no thickness and no conductivity: modelled as a 2 mm shell, say so in decisions (B2)`);
  return 0.002;
};

const anchorOf = (id: string): Json => c.m.anchors.find((a: Json) => a.id === id);

function emitterSolids(regionOf: (bone: string) => string): Json[] {
  const out: Json[] = [];
  const H = c.m.dimensions.heightM;
  for (const e of c.m.thermal.emitters ?? []) {
    const ids: string[] = e.anchors?.length ? [e.anchors[0]] : [e.anchor];
    for (const aid of ids) {
      const a = anchorOf(aid);
      if (a === undefined) continue;
      const f = c.state.frames[a.bone];
      const th = (e.directionDeg * Math.PI) / 180;
      const dirW: [number, number, number] = [-Math.sin(th), Math.cos(th), 0];
      const dirL = f === undefined ? dirW : dirToLocal(f, dirW);
      const at: [number, number, number] = [a.x, a.y, a.z ?? 0];
      const region = e.region ?? regionOf(a.bone);
      const bigDisc = e.diameterM > 0.05 * H;
      const mat = body.find((s) => s.bone === a.bone)?.material ?? c.m.materials[0].id;
      const id = `th-${e.id}${ids.length > 1 ? `-${aid}` : ""}`;
      const common = { id, bone: a.bone, material: mat, tone: 0.8, role: "body" };
      let s: Json;
      if (bigDisc) {
        s = ellipsoidOf(common, at, [Math.max(0.6 * c.feature, 0.004), e.diameterM / 2, e.diameterM / 2]);
        const q = Math.atan2(dirL[1], dirL[0]);
        s.transform.rotation = [0, 0, Math.sin(q / 2), Math.cos(q / 2)];
      } else {
        const d = Math.max(e.diameterM, 1.25 * c.feature);
        const L = Math.max(0.02, 2 * c.feature);
        s = capsuleOf(common, at, [0, 1, 2].map((k) => at[k]! + dirL[k]! * L) as never, d / 2);
      }
      s.region = region;
      s.tags = ["emitter", e.id];
      out.push(s);
    }
  }
  return out;
}

function hiddenSolids(): Json[] {
  const e = extentOf(c.state ? [] : []);
  void e;
  const out: Json[] = [];
  for (const r of regions) {
    if (!r.hidden) continue;
    const rad = Math.max(0.55 * c.feature, Math.sqrt(r.areaM2 / (4 * Math.PI)));
    const bone = body[0]!.bone;
    out.push({ ...ellipsoidOf({ id: `th-${r.id}-hidden`, bone, material: body[0]!.material, tone: 0.3 }, [0, 0, 0], [rad, rad, rad]), region: r.id });
    notes.push(`hidden region ${r.id}: placeholder ellipsoid on bone ${bone}; move it to where the real part is inside the body (needs-author)`);
  }
  return out;
}

function sectionFacesOf(s: Json, r: Json): Json {
  if (!has(args, "--section-faces") || r.kind !== "wall" || s.kind !== "box") return s;
  const thin = s.size.indexOf(Math.min(...s.size));
  const axes = ["u", "v", "w"].filter((_, i) => i !== thin);
  return { ...s, sectionFaces: axes.flatMap((a) => [`-${a}`, `+${a}`]) };
}

function build(bind: Map<string, string[]>, k: Map<string, number>): Json[] {
  const out: Json[] = [];
  for (const r of regions) {
    if (r.hidden) continue;
    const layers: Json[] = r.layers ?? [];
    const several = scale === 50 && layers.length > 1;
    for (const sid of bind.get(r.id) ?? []) {
      const s = byId.get(sid)!;
      const f = k.get(r.id) ?? 1;
      const multi = [...bind.values()].filter((ids) => ids.includes(sid)).length > 1;
      const id = multi ? `th-${r.id}-${sid}` : `th-${sid}`;
      const top = layers.length > 0 ? layers[layers.length - 1]!.id : undefined;
      const outer: Json = f === 1 ? { id: several ? `${id}-${top}` : id, of: sid, region: r.id, ...(top ? { layer: top } : {}) } : { ...sectionFacesOf(scaledSolid(s, 1, several ? `${id}-${top}` : id, 0.55 * c.feature), r), region: r.id, role: "body", ...(top ? { layer: top } : {}) };
      if (f === 1 && has(args, "--section-faces") && sectionFacesOf(s, r) !== s) Object.assign(outer, { of: undefined });
      out.push(outer);
      if (several) {
        let cum = 0;
        for (let i = layers.length - 2; i >= 0; i--) {
          cum += thickness(layers[i + 1]!, r);
          let shell: Json | null = null;
          let used = cum;
          for (let q = 1; q > 0.02 && shell === null; q *= 0.6) {
            used = cum * q;
            shell = shellOffset(s, used, `${id}-${layers[i]!.id}`, 0.5e-3);
          }
          if (shell !== null && used < cum) notes.push(`layer ${r.id}/${layers[i]!.id} of ${sid}: inset reduced from ${(cum * 1000).toFixed(1)} to ${(used * 1000).toFixed(1)} mm (solid too small); refine by hand`);
          if (shell === null) {
            notes.push(`layer ${r.id}/${layers[i]!.id} of ${sid}: solid too small for a ${(cum * 1000).toFixed(1)} mm inset; model that shell by hand`);
            continue;
          }
          out.push({ ...shell, region: r.id, layer: layers[i]!.id, role: "body", tone: Math.max(0, (s.tone ?? 0.5) - 0.1 * (layers.length - 1 - i)) });
        }
      }
    }
  }
  return out;
}

const binding = bind();
for (const [r, m] of flag(args, "--map") ? [] : []) void r, m;
const regionOf = (bone: string): string => {
  const counts = new Map<string, number>();
  for (const sh of [...drawn(c, "elevation", bone)]) if (sh.region) counts.set(sh.region, (counts.get(sh.region) ?? 0) + 1);
  return [...counts.entries()].sort((a, b) => b[1] - a[1])[0]?.[0] ?? regions[0].id;
};
const extra = (): Json[] => [...hiddenSolids(), ...emitterSolids(regionOf)];

function exposedOf(thermal: Json[]): Map<string, { exposed: number; target: number; dev: number; ok: boolean }> {
  const m2: Json = { ...c.m, volumes: { ...c.m.volumes, [String(scale)]: { ...c.vol, thermal } } };
  const out = new Map<string, { exposed: number; target: number; dev: number; ok: boolean }>();
  const product = measureVolumes(m2 as never, { scale: scale as never, rules: ["volumes-thermal-coverage"] as never });
  const owned = rulingMeasurements(m2, { volumes: m2.volumes }, { scale, rules: ["volumes-thermal-coverage"] });
  void product;
  for (const x of owned.length > 0 ? owned : rulingMeasurements(m2, { volumes: m2.volumes }, { scale, rules: ["volumes-thermal-coverage"] })) {
    if (!x.name.startsWith("areaDeviation:")) continue;
    const mm = /exposed ([\d.]+) m2 of ([\d.]+) m2 mesh, areaM2 ([\d.]+)/.exec(x.note ?? "");
    if (mm) out.set(x.name.slice(14), { exposed: Number(mm[1]), target: Number(mm[3]), dev: x.value, ok: x.ok });
  }
  return out;
}

const k = new Map<string, number>();
const thermal: Json[] = [...build(binding, k), ...extra()];
const area = exposedOf(thermal);
const limit = scale === 50 ? 0.15 : 0.3;
const C16 = (r: Json): boolean => r.model === "leaf" || ["foliage", "fur", "feather", "hair"].includes(r.kind);
const C18 = (r: Json): boolean => r.body !== undefined || ["skin", "clothing", "hair", "fur", "feather"].includes(r.kind);
const NOAPPLY = (r: Json): boolean => C16(r) || C18(r);
const totalE = [...area.values()].reduce((q, a) => q + a.exposed, 0);
const totalT = [...area.values()].reduce((q, a) => q + a.target, 0);
const buriers = (rid: string): string => {
  const mine = binding.get(rid) ?? [];
  const bones = new Set(mine.map((id) => byId.get(id)?.bone));
  const others = [...binding.entries()].filter(([o]) => o !== rid).flatMap(([o, ids]) => ids.filter((id) => bones.has(byId.get(id)?.bone) && !mine.includes(id)).map((id) => `${id}(${o})`));
  return `bound solids ${mine.join(", ") || "none"}; solids of other regions on the same bones that overlap or bury the surface: ${[...new Set(others)].slice(0, 10).join(", ") || "none"}`;
};
const lines: string[] = [`kit-thermal 1:${scale} ${spec}: ${thermal.length} thermal solids (${thermal.filter((s) => s.of !== undefined).length} references)`];
lines.push("region".padEnd(22) + "solids".padStart(7) + "factor".padStart(8) + "exposed".padStart(10) + "areaM2".padStart(9) + "ratio".padStart(7));
const proposals: { region: string; old: number; neu: number }[] = [];
for (const r of regions) {
  const a = area.get(r.id);
  const n = (binding.get(r.id) ?? []).length;
  lines.push(`${r.id.padEnd(22)}${String(n).padStart(7)}${(k.get(r.id) ?? 1).toFixed(2).padStart(8)}${a ? a.exposed.toFixed(3).padStart(10) : "-".padStart(10)}${r.areaM2.toFixed(3).padStart(9)}${a ? (a.exposed / a.target).toFixed(2).padStart(7) : "-".padStart(7)}${n === 0 && !r.hidden ? "  NO SOLID: bind with --map or draw it" : ""}${C16(r) ? "  (exchange area, C16: coverage only)" : C18(r) ? (scale <= 100 && a && Math.abs(a.exposed - a.target) / a.target > 0.3 ? "  MODEL FINDING (C18, physiological areaM2, 30 %)" : "  (physiological areaM2, C18: never changed)") : scale <= 100 && a && !a.ok ? "  OUT OF TOLERANCE" : ""}`);
  if (scale === 50 && a && !NOAPPLY(r) && Math.abs(a.exposed - a.target) / a.target > limit) proposals.push({ region: r.id, old: r.areaM2, neu: Math.round(a.exposed * 1000) / 1000 });
}
for (const r of regions) {
  const a = area.get(r.id);
  if (C18(r) && !C16(r) && scale <= 100 && a && Math.abs(a.exposed - a.target) / a.target > 0.3) lines.push(`model finding ${r.id}: exposed ${a.exposed.toFixed(3)} m2 vs physiological areaM2 ${a.target} (${((a.exposed / a.target - 1) * 100).toFixed(0)} %); ${buriers(r.id)}`);
}
for (const n of notes) lines.push(`note: ${n}`);
if (scale > 100) lines.push(`C15: only the total counts at 1:${scale}: exposed ${totalE.toFixed(3)} m2 vs areaM2 ${totalT.toFixed(3)} (ratio ${(totalE / totalT).toFixed(2)}, limit 30 %)`);
if (scale !== 50) lines.push("areaM2 is corrected at the root only from the 1:50 model (C15)");
if (proposals.length > 0) {
  lines.push(`areaM2 corrections at the root (what the honestly modelled object exposes at 1:${scale}${scale === 50 ? "" : "; derive them from the finest scale"}):`);
  for (const p of proposals) lines.push(`  ${p.region}: ${p.old} -> ${p.neu}`);
}
if (has(args, "--apply-area") && proposals.length > 0) {
  const ok = proposals.filter((p) => p.neu / p.old <= 1.5 && p.neu / p.old >= 1 / 1.5);
  const review = proposals.filter((p) => !ok.includes(p));
  if (ok.length > 0) applyArea(ok);
  if (review.length > 0) {
    const f = join(TICKET, "area-review.md");
    appendFileSync(f, review.map((p) => `- ${spec} ${p.region}: old ${p.old}, measured ${p.neu} (factor ${(p.neu / p.old).toFixed(2)}), not applied (C16 limit 1.5); the model exposes ${p.neu} m2 at 1:50\n`).join(""));
  }
  lines.push(`applied ${ok.length} areaM2 correction(s) within factor 1.5; ${review.length} listed in area-review.md`);
}
const layersTxt = regions.filter((r) => (r.layers ?? []).length > 1);
lines.push("decisions to adapt (en):", `  Thermal: ${regions.length} regions bound to the solids of this scale's own drawings${scale === 50 && layersTxt.length > 0 ? `; ${layersTxt.map((r) => r.id).join(", ")} modelled as nested shells per layer` : "; layers merged into one surface solid per region"}; emitters as tagged jets and discs at their anchors; thermal solids reference the body solids, never scaled copies.`);
lines.push("decisions to adapt (de):", `  Thermik: ${regions.length} Regionen an die Körper aus den eigenen Zeichnungen dieses Maßstabs gebunden${scale === 50 && layersTxt.length > 0 ? `; ${layersTxt.map((r) => r.id).join(", ")} als verschachtelte Schalen je Schicht` : "; Schichten zu einem Oberflächenkörper je Region zusammengefasst"}; Emitter als markierte Strahlen und Scheiben an ihren Ankern; Thermikkörper verweisen auf die Körper, keine skalierten Kopien.`);
process.stdout.write(lines.join("\n") + "\n");
writeOut(`thermal/${spec.replace("/", "-")}.s${scale}.json`, JSON.stringify({ thermal, k: Object.fromEntries(k), proposals }, null, 1));
if (has(args, "--write")) {
  saveLists(c, { thermal: thermal.map((s) => JSON.parse(JSON.stringify(s))) });
  process.stdout.write(`written thermal of 1:${scale} to ${c.file}\n`);
}

function applyArea(list: { region: string; old: number; neu: number }[]): void {
  const path = assetPath(c);
  const lines = readFileSync(path, "utf8").split("\n");
  const start = lines.findIndex((l) => l === '    "thermal": {');
  for (const p of list) {
    const at = lines.findIndex((l, i) => i > start && l.trim() === `"id": "${p.region}",`);
    const ai = lines.findIndex((l, i) => i > at && /^\s+"areaM2": /.test(l));
    if (at < 0 || ai < 0 || ai - at > 12) throw new Error(`cannot locate areaM2 of ${p.region}`);
    lines[ai] = lines[ai]!.replace(/"areaM2": [\d.eE+-]+/, `"areaM2": ${p.neu}`);
  }
  writeFileSync(path, lines.join("\n"));
}

function assetPath(ctx: Ctx): string {
  return resolveAssetPath(ctx.spec);
}

import { resolveAsset } from "./volumes-lib.ts";
function resolveAssetPath(s: string): string {
  return resolveAsset(s).path;
}
void clone;
