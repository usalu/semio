/** ✏️ Proposes solids for one scale of one asset from that scale's own 2D shapes; the author then handcrafts them.
 *
 * Usage: `bun volumes-draft.ts <category>/<slug> --scale N [--list body|thermal|structure] [--apply] [--force]`
 * - body: lofts, capsules, extrudes... per bone from the scale's elevation, front and plan shapes (the 1:50 drafter of the previous ticket run on that scale).
 * - thermal: `{ of }` references from the region of each drafted body solid; at 1:50 a region with several layers becomes nested shells (outer layer = reference, inner layers = the body solid shrunk by the thickness of the layers outside it); hidden regions, emitters and regions without solid are listed as `needs-author`.
 * - structure: members along the member bones (amendment B1: `eligibleBones` plus bones of 2D `member` shapes) with plausible sections (bone `section`, 2D member section, else estimated from mass), 2D `member` shapes as bars, joints (1:50 and 1:100), supports of `structure.supports` as pads sized by the feature of the scale (B3), concentrated masses.
 * Output: `🗑️generated/volumes-tools/draft/<category>/<slug>.s<N>.<list>.json` = `{ solids, flags }`; with `--apply` the list is written into the source (needs `--force` when the list is not empty). Without `--list` all three lists are drafted.
 * @see ./design.md
 * @see ./volumes-tools.md */
import { LISTS, SCALES, entryOf, featureM, frames, loadSource, memberBones, memberShapes, outPath, round5, sourcePath, writeJson, type Json, type ListName } from "./volumes-lib.ts";
import { draftAsset } from "./volumes-draft-base.ts";
import { regionsOf, seed, strip, thermalRefs } from "./volumes-seed.ts";

export type Flag = { code: string; subject: string; message: string };

export function draftBody(asset: Json, scale: number): { solids: Json[]; flags: Flag[]; raw: Json[] } {
  const result = draftAsset(asset, scale, false);
  const raw = result.volume.solids as Json[];
  const flags: Flag[] = (result.report.flags as Json[]).filter((f) => f.severity !== "info").map((f) => ({ code: f.code, subject: f.subject, message: f.message }));
  return { solids: raw.map(strip), flags, raw };
}

const shrinkable = new Set(["box", "ellipsoid", "capsule", "revolve", "loft"]);

/** 🧅️ The body solid shrunk inwards by `d` metres, or null when the kind cannot be offset analytically or the solid would vanish. */
export function shrink(solid: Json, d: number): Json | null {
  if (!shrinkable.has(solid.kind)) return null;
  const c = JSON.parse(JSON.stringify(solid));
  const r = (x: number) => round5(x);
  if (c.kind === "box") {
    c.size = c.size.map((x: number) => r(x - 2 * d));
    if (c.size.some((x: number) => x <= 1e-4)) return null;
    if (c.cornerRadius !== undefined) c.cornerRadius = r(Math.max(0, c.cornerRadius - d));
  } else if (c.kind === "ellipsoid") {
    c.radii = c.radii.map((x: number) => r(x - d));
    if (c.radii.some((x: number) => x <= 5e-5)) return null;
  } else if (c.kind === "capsule") {
    c.r0 = r(c.r0 - d);
    c.r1 = r(c.r1 - d);
    if (c.r0 <= 5e-5 || c.r1 <= 5e-5) return null;
  } else if (c.kind === "revolve") {
    c.profile = c.profile.map(([a, rad]: number[]) => [a, r(Math.max(0, rad - d))]);
    if (c.profile.every(([, rad]: number[]) => rad <= 5e-5)) return null;
  } else {
    c.sections = c.sections.map((s: Json) => ({ ...s, rx: r(s.rx - d), ry: r(s.ry - d) }));
    if (c.sections.some((s: Json) => s.rx <= 5e-5 || s.ry <= 5e-5)) return null;
  }
  return c;
}

const layerThickness = (layer: Json, region: Json): number => {
  if (typeof layer.thicknessM === "number") return layer.thicknessM;
  const mat = region.conductivityWMK ?? layer.conductivityWMK;
  return typeof layer.resistanceM2KW === "number" && typeof mat === "number" ? layer.resistanceM2KW * mat : 0;
};

export function draftThermal(asset: Json, scale: number, body: readonly Json[], bindings: readonly Json[]): { solids: Json[]; flags: Flag[] } {
  const flags: Flag[] = [];
  const regions: Json[] = asset.thermal?.regions ?? [];
  const byId = new Map(regions.map((r) => [r.id, r]));
  const solids: Json[] = [];
  const bodyById = new Map(body.map((s) => [s.id, s]));
  for (const b of bindings) {
    for (const region of regionsOf(b)) {
      const r = byId.get(region);
      if (r === undefined) {
        flags.push({ code: "unknown-region", subject: b.id, message: `body shape names region "${region}" which does not exist` });
        continue;
      }
      const layers: Json[] = r.layers ?? [];
      const several = scale === 50 && layers.length > 1 && !r.hidden;
      const tag = regionsOf(b).length === 1 ? `th-${b.id}` : `th-${region}-${b.id}`;
      if (!several) {
        solids.push({ id: tag, of: b.id, region, ...(layers.length > 0 ? { layer: layers[layers.length - 1].id } : {}) });
        continue;
      }
      let outside = 0;
      for (let i = layers.length - 1; i >= 0; i--) {
        const layer = layers[i];
        if (i === layers.length - 1) solids.push({ id: `${tag}-${layer.id}`, of: b.id, region, layer: layer.id });
        else {
          const shell = shrink(bodyById.get(b.id)!, outside);
          if (shell === null) flags.push({ code: "layer-not-modelled", subject: `${b.id}/${layer.id}`, message: "body solid cannot be offset inwards by the outer layers (kind or size); model this layer by hand" });
          else solids.push({ ...shell, id: `${tag}-${layer.id}`, region, layer: layer.id, tone: Math.max(0, (shell.tone ?? 0.5) - 0.1) });
        }
        const t = layerThickness(layer, r);
        if (t === 0) flags.push({ code: "layer-without-thickness", subject: `${region}/${layer.id}`, message: "no thicknessM and no conductivity to derive one: merge into the neighbouring shell and say so in decisions (B2)" });
        outside += t;
      }
    }
  }
  const covered = new Set(solids.map((s) => s.region));
  for (const r of regions) if (!covered.has(r.id)) flags.push({ code: "needs-author", subject: r.id, message: r.hidden ? "hidden region: model it as a real solid inside the body (B4)" : "no body shape of this scale is bound to the region; add a reference or a solid" });
  for (const e of asset.thermal?.emitters ?? []) flags.push({ code: "needs-author", subject: e.id, message: `emitter (${e.kind}, anchor ${e.anchor}, diameter ${e.diameterM ?? "?"} m) needs a thermal solid whose id contains "${e.id}" (B3)` });
  return { solids, flags };
}

const sectionOf = (asset: Json, bone: Json, scale: number, memberSection?: Json): { r: number; ri: number; note: string } => {
  const feature = featureM(scale, asset);
  const min = feature / 2;
  if (memberSection?.kind === "circle") return { r: Math.max(min, memberSection.d / 2), ri: 0, note: "2D member section" };
  if (memberSection?.kind === "rect") return { r: Math.max(min, Math.hypot(memberSection.w, memberSection.h) / 2), ri: 0, note: "2D member section (rect approximated by its circumscribed circle)" };
  const s = bone.section;
  if (s?.areaM2 > 0) {
    const d = s.areaM2 / Math.PI;
    const sum = s.secondMomentM4 > 0 ? (4 * s.secondMomentM4) / s.areaM2 : 0;
    if (sum > d * 1.05) {
      const R2 = (sum + d) / 2;
      const ri2 = R2 - d;
      return { r: Math.max(min, Math.sqrt(R2)), ri: ri2 > 0 ? Math.sqrt(ri2) : 0, note: "bone section (tube from area and second moment)" };
    }
    return { r: Math.max(min, Math.sqrt(d)), ri: 0, note: "bone section (solid circle from area)" };
  }
  const L = Math.max(bone.lengthM ?? 0.1, 0.05);
  const mass = bone.mass?.kg ?? 0;
  return { r: Math.min(L / 2, Math.max(min, Math.sqrt(mass / (1000 * Math.PI * L)))), ri: 0, note: "estimated from mass and length" };
};

export function draftStructure(asset: Json, scale: number): { solids: Json[]; flags: Flag[] } {
  const flags: Flag[] = [];
  const solids: Json[] = [];
  const feature = featureM(scale, asset);
  const bones: Json[] = asset.rig.bones;
  const byId = new Map(bones.map((b) => [b.id, b]));
  const radiusOf = new Map<string, number>();
  const material = (b: Json): string => b.section?.material ?? asset.materials?.[0]?.id ?? "unknown";
  const bars = memberShapes(asset, scale);
  const barBones = new Set(bars.map((s) => s.bone));
  for (const id of memberBones(asset, scale)) {
    const b = byId.get(id);
    if (b === undefined) continue;
    const { r, ri, note } = sectionOf(asset, b, scale);
    radiusOf.set(id, r);
    const L = Math.max(b.lengthM ?? 0, feature);
    if (barBones.has(id) && b.section === undefined && (b.lengthM ?? 0) < 0.05 * asset.dimensions.heightM) continue;
    const solid: Json = ri > 0.3 * r
      ? { id: `member-${id}`, kind: "revolve", bone: id, material: material(b), role: "body", tone: 0.5, member: id, part: "member", profile: [[0, round5(ri)], [round5(L), round5(ri)], [round5(L), round5(r)], [0, round5(r)]], axis: [1, 0, 0], origin: [0, 0, 0] }
      : { id: `member-${id}`, kind: "loft", bone: id, material: material(b), role: "body", tone: 0.5, member: id, part: "member", sections: [{ at: [0, 0, 0], rx: round5(r), ry: round5(r), exponent: 2 }, { at: [round5(L), 0, 0], rx: round5(r), ry: round5(r), exponent: 2 }], capStart: true, capEnd: true };
    solids.push(solid);
    if (note.startsWith("estimated")) flags.push({ code: "section-estimated", subject: id, message: `no section: radius ${(r * 1000).toFixed(1)} mm ${note}` });
  }
  bars.forEach((s) => {
    const b = byId.get(s.bone);
    if (b === undefined) return;
    const { r } = sectionOf(asset, b, scale, s.section);
    const p = (e: Json): number[] => [round5(e.x), round5(e.y), round5(e.z)];
    solids.push({ id: `bar-${s.id}`, kind: "capsule", bone: s.bone, material: s.material ?? material(b), role: "body", tone: 0.5, member: s.bone, part: "member", p0: p(s.from), p1: p(s.to), r0: round5(r), r1: round5(r) });
  });
  if (scale <= 100)
    for (const b of bones) {
      if (b.parent === null || b.joint?.type === "fixed" || !radiusOf.has(b.id)) continue;
      const r = Math.max(feature / 2, 1.1 * Math.min(radiusOf.get(b.id) ?? 0, radiusOf.get(b.parent) ?? 0), Math.min(0.5 * (b.lengthM ?? 1), 1.15 * Math.max(radiusOf.get(b.id) ?? 0, 0)));
      solids.push({ id: `joint-${b.id}`, kind: "ellipsoid", bone: b.id, material: material(b), role: "detail", tone: 0.35, member: b.id, part: "joint", radii: [round5(r), round5(r), round5(r)] });
    }
  for (const id of asset.structure?.supports ?? []) {
    const a = (asset.anchors ?? []).find((x: Json) => x.id === id);
    if (a === undefined) {
      flags.push({ code: "needs-author", subject: id, message: "support anchor not found" });
      continue;
    }
    const w = Math.max(feature, a.footprintLengthM ?? 0, 0.02);
    const dpt = Math.max(feature, a.footprintWidthM ?? 0, 0.02);
    solids.push({ id: `support-${id}`, kind: "box", bone: a.bone, material: material(byId.get(a.bone) ?? {}), role: "detail", tone: 0.25, member: a.bone, part: "support", size: [round5(w), round5(Math.max(feature, 0.02)), round5(dpt)], transform: { translation: [round5(a.x), round5(a.y), 0] } });
    flags.push({ code: "support-type", subject: id, message: `support "${a.support}": shape the pad (pin, roller, fixed block, contact pad) by hand` });
  }
  const total = bones.reduce((n, b) => n + (b.mass?.kg ?? 0), 0);
  for (const b of bones) {
    const kg = b.mass?.kg ?? 0;
    if (kg <= 0 || kg < 0.1 * total || b.mass?.distribution === "uniform") continue;
    const r = Math.max(feature / 2, Math.min(Math.cbrt((3 * kg) / (4 * Math.PI * 900)), radiusOf.get(b.id) ?? Infinity));
    solids.push({ id: `mass-${b.id}`, kind: "ellipsoid", bone: b.id, material: material(b), role: "detail", tone: 0.15, member: b.id, part: "mass", radii: [round5(r), round5(r), round5(r)], transform: { translation: [round5(b.mass.comX ?? 0), round5(b.mass.comY ?? 0), 0] } });
  }
  void frames;
  return { solids, flags };
}

if (import.meta.main) {
  const args = process.argv.slice(2);
  const flag = (n: string) => (args.indexOf(n) < 0 ? undefined : args[args.indexOf(n) + 1]);
  const spec = args.find((a, i) => !a.startsWith("--") && !["--scale", "--list"].includes(args[i - 1] ?? ""));
  const scale = Number(flag("--scale"));
  if (spec === undefined || !SCALES.includes(scale as never)) {
    process.stderr.write("usage: volumes-draft.ts <category>/<slug> --scale 50|100|200|500 [--list body|thermal|structure] [--apply] [--force]\n");
    process.exit(2);
  }
  const lists: readonly ListName[] = flag("--list") === undefined ? LISTS : [flag("--list") as ListName];
  const { entry, asset } = entryOf(spec);
  if (asset.representations?.[String(scale)]?.status !== "authored") {
    process.stdout.write(`1:${scale} is unavailable for ${spec}; nothing to draft\n`);
    process.exit(0);
  }
  const source = loadSource(entry);
  const existing = source?.volumes?.[String(scale)];
  const draft = draftBody(asset, scale);
  const out: Record<string, { solids: Json[]; flags: Flag[] }> = {};
  for (const l of lists) {
    if (l === "body") out.body = { solids: draft.solids, flags: draft.flags };
    else if (l === "thermal") out.thermal = draftThermal(asset, scale, draft.solids, draft.raw);
    else out.structure = draftStructure(asset, scale);
    const path = outPath("draft", entry, `${entry.slug}.s${scale}.${l}.json`);
    writeJson(path, out[l], true);
    const f = out[l]!.flags;
    process.stdout.write(`1:${scale} ${l}: ${out[l]!.solids.length} solids, ${f.length} flags (${[...new Set(f.map((x) => x.code))].join(", ") || "none"}) -> ${path}\n`);
  }
  if (args.includes("--apply")) {
    const target = source ?? seed(asset);
    const volume = target.volumes[String(scale)];
    for (const l of lists) {
      if ((volume[l] ?? []).length > 0 && !args.includes("--force")) {
        process.stderr.write(`source ${l} at 1:${scale} is not empty; pass --force to overwrite\n`);
        continue;
      }
      volume[l] = out[l]!.solids;
    }
    writeJson(sourcePath(entry), target, true);
    process.stdout.write(`applied to ${sourcePath(entry)}\n`);
  }
  void existing;
  void thermalRefs;
}
