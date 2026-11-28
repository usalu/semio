/** 🧰️ The repairs of lane CL-T3 per asset id, each idempotent on the asset document, and the log of what they changed.
 *
 * - `dairy-cow`, `parent-carrying-toddler`: the loft stations that the drawing of a scale cannot show are dropped ({@link thin}): one at a time the station whose removal moves the skin least, as long as every ring point of the authored loft stays within a quarter of the smallest feature of the scale from the thinned skin (1.9, 3.8, 7.5 and 18.8 mm at 1:50, 1:100, 1:200 and 1:500, 0.04 mm of paper). In the cow these are mostly the three-step roundings of the ends of the limb masses, carried from 1:50 into every coarser scale at the same millimetres, and the stations of the coat patches beyond what the curve of the trunk needs. The first, the last and the widest stations of every loft stay, so no solid changes its length or its thickness (without the widest one the tail switch of 1:500 fell to 60 mm, below its feature of 70.2 mm, `volumes-feature`); every loft is bound to one bone or to one constant blend of bones, which moves it as a whole, so no pose bends a loft. In `parent-carrying-toddler` the skin under a breath or plume outlet (the thermal solids of its bone and region and the body solids they refer to) stays as authored: with the head thinned at 1:500 the toddler's mouth outlet (4.5 mm) no longer sat on the skin and `volumes-feature` refused it; the outlets of the cow stay on its skin (validated), so its head is thinned. The decision of a scale records the thinning, and a scale that records it is not thinned again (the measure is against the authored skin, which a thinned loft no longer holds).
 * - `tower-crane`: what the drawing of a scale cannot show as round is not round: the corner rounding of a box goes where the arc leaves the sharp corner by less than one smallest feature ({@link sharpen}: radius below 18 mm at 1:50, 36 mm at 1:100), and the 163 rungs and brackets of the ladder texture of 1:50 (25 mm round bars, each with a mesh of its own in the GLB) are square bars of 25 mm ({@link square}). Its lattice bars are two-point revolves at the feature size of their scale and members whose section is pinned; they have nothing to thin. Joining the bars of a constant pitch into repeated solids was tried and dropped: the pose GLBs grew from 8.39 to 10.09 MB.
 * @see ./fix.ts — the command */
import { loftFrames } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🧵️loft/🟦️.ts";

type Solid = Record<string, any>;
type Text = { en: string; de: string };
type Volume = { body: Solid[]; thermal: Solid[]; structure: Solid[]; decisions: Text[] };

/** 📜️ One line per change of the running repair. */
export const log: string[] = [];

/** 📏️ The smallest feature of a scale in metres: 0.15 mm of paper. */
export const featureOf = (scale: number): number => 0.00015 * scale;

const volume = (asset: any, scale: number): Volume => asset.volumes[String(scale)];
const append = (decision: Text, text: Text, what: string): void => {
  for (const language of ["en", "de"] as const) {
    if (decision[language].includes(text[language])) continue;
    decision[language] = `${decision[language]} ${text[language]}`;
    log.push(`${what}: ${language} text added`);
  }
};

/** 🧿️ The rings of a loft as the kernel tessellates them (64 points each, the kernel's own frames and turn signs). */
const ringsOf = (sections: Solid[]): number[][][] => {
  const { frames, turn } = loftFrames(sections as never);
  return sections.map((s, i) => {
    const f = frames[i]!, e = 2 / (s.exponent ?? 2);
    return Array.from({ length: SAMPLES }, (_, j) => {
      const theta = (turn[i]! * 2 * Math.PI * j) / SAMPLES, c = Math.cos(theta), n = Math.sin(theta);
      const x = s.rx * Math.sign(c) * Math.abs(c) ** e, y = s.ry * Math.sign(n) * Math.abs(n) ** e;
      return [0, 1, 2].map((k) => s.at[k] + f.x[k]! * x + f.y[k]! * y);
    });
  });
};
const SAMPLES = 64;
const toSegment = (p: number[], a: number[], b: number[]): number => {
  const d = [0, 1, 2].map((k) => b[k]! - a[k]!), w = [0, 1, 2].map((k) => p[k]! - a[k]!), dd = d[0]! * d[0]! + d[1]! * d[1]! + d[2]! * d[2]!;
  const t = dd > 0 ? Math.max(0, Math.min(1, (w[0]! * d[0]! + w[1]! * d[1]! + w[2]! * d[2]!) / dd)) : 0;
  return Math.hypot(w[0]! - t * d[0]!, w[1]! - t * d[1]!, w[2]! - t * d[2]!);
};

/** 📐️ The largest distance of any ring point of the loft `sections` from the skin of the loft through the stations `kept` (indices into `sections`, first and last included): each point against the ruled line between the same ring points of the two kept stations around it, with the frames the thinned loft itself has. */
export const deviation = (sections: Solid[], kept: readonly number[]): number => {
  const full = ringsOf(sections), thin = ringsOf(kept.map((i) => sections[i]!));
  let worst = 0;
  for (let i = 0; i < sections.length; i++) {
    const after = Math.max(1, kept.findIndex((k) => k >= i)), a = thin[after - 1]!, b = thin[after]!;
    for (let j = 0; j < SAMPLES; j++) worst = Math.max(worst, toSegment(full[i]![j]!, a[j]!, b[j]!));
  }
  return worst;
};

/** 🪒️ Drops from every loft of the lists `modes` of a scale the inner stations the drawing cannot show: one at a time the station whose removal moves the skin least, as long as no ring point of the authored loft lies further than `share` of the smallest feature of the scale from the thinned skin. The first and the last station stay, and the widest ones (largest `rx`, largest `ry`, largest smaller radius), so that no solid changes its length or the thickness the feature rule reads. The solids `spare` (ids) stay as authored. Returns the number of dropped stations. */
export const thin = (asset: any, scale: number, modes: readonly ("body" | "thermal" | "structure")[], share = SHARE, spare: ReadonlySet<string> = new Set()): number => {
  const limit = share * featureOf(scale);
  let dropped = 0;
  for (const mode of modes) {
    for (const solid of volume(asset, scale)[mode]) {
      if (solid.kind !== "loft" || spare.has(solid.id)) continue;
      const sections = solid.sections as Solid[];
      const most = (of: (s: Solid) => number): number => sections.reduce((best, s, i) => (of(s) > of(sections[best]!) ? i : best), 0);
      const widest = new Set([most((s) => s.rx), most((s) => s.ry), most((s) => Math.min(s.rx, s.ry))]);
      let kept = sections.map((_, i) => i);
      for (;;) {
        let best: { kept: number[]; d: number } | undefined;
        for (let at = 1; at < kept.length - 1; at++) {
          if (widest.has(kept[at]!)) continue;
          const trial = kept.filter((_, i) => i !== at), d = deviation(sections, trial);
          if (d < limit && (best === undefined || d < best.d)) best = { kept: trial, d };
        }
        if (best === undefined) break;
        kept = best.kept;
      }
      if (kept.length === sections.length) continue;
      dropped += sections.length - kept.length;
      log.push(`1:${scale} ${mode}: ${solid.id} ${sections.length} -> ${kept.length} stations, skin moved by at most ${(1000 * deviation(sections, kept)).toFixed(2)} mm (limit ${(1000 * limit).toFixed(2)} mm)`);
      solid.sections = kept.map((i) => sections[i]);
    }
  }
  return dropped;
};

/** 🎚️ The share of the smallest feature by which a thinned loft may leave its authored skin. */
export const SHARE = 0.25;

const THIN = (scale: number): Text => ({
  en: `The lofts keep only the stations the drawing can show: a station is left out when the skin without it stays within ${(1000 * SHARE * featureOf(scale)).toFixed(1)} mm of the authored skin (a quarter of the smallest feature of 1:${scale}).`,
  de: `Die Lofts behalten nur die Stationen, die die Zeichnung zeigen kann: eine Station entfällt, wenn die Haut ohne sie höchstens ${(1000 * SHARE * featureOf(scale)).toFixed(1).replace(".", ",")} mm von der modellierten Haut abweicht (ein Viertel des kleinsten Merkmals von 1:${scale}).`,
});

/** 🫁️ The solids of a scale that are the skin under an emitter (a breath or plume outlet): the thermal solids of the bone and region of an emitter and the body solids they refer to. An emitter is a solid far below the feature that `volumes-feature` admits only while it sits on that skin, so that skin must not move. */
const emitterSkins = (asset: any, scale: number): Set<string> => {
  const v = volume(asset, scale), places = new Set(v.thermal.filter((s) => s.tags?.includes("emitter")).map((s) => `${s.bone} ${s.region}`));
  const body = new Map(v.body.map((s) => [s.id as string, s]));
  return new Set(v.thermal.flatMap((s) => (places.has(`${s.of === undefined ? s.bone : body.get(s.of)?.bone} ${s.region}`) ? [s.id as string, ...(s.of === undefined ? [] : [s.of as string])] : [])));
};

/** 🪒️ Thins every authored scale of an asset once and records it in the first decision of the scale; with `emitters` the skin under every emitter stays as authored. */
const thinned = (emitters: boolean) => (asset: any): void => {
  for (const scale of Object.keys(asset.volumes).map(Number)) {
    if (volume(asset, scale).decisions[0]!.en.includes(THIN(scale).en)) continue;
    thin(asset, scale, ["body", "thermal", "structure"], SHARE, emitters ? emitterSkins(asset, scale) : new Set());
    append(volume(asset, scale).decisions[0]!, emitters ? { en: `${THIN(scale).en} The skin under a breath or plume outlet keeps every station, so that the outlet stays on it.`, de: `${THIN(scale).de} Die Haut unter einem Atem- oder Fahnenauslass behält jede Station, damit der Auslass auf ihr bleibt.` } : THIN(scale), `1:${scale} decision`);
  }
};

/** 🔲️ Removes the corner rounding of every box of a scale whose rounded corner stays within one smallest feature of the sharp one (the arc leaves the corner by `(sqrt 2 - 1) r`), and returns the ids. */
export const sharpen = (asset: any, scale: number): string[] => {
  const ids: string[] = [];
  for (const mode of ["body", "thermal", "structure"] as const) {
    for (const solid of volume(asset, scale)[mode]) {
      if (solid.kind !== "box" || !(solid.cornerRadius > 0) || (Math.SQRT2 - 1) * solid.cornerRadius >= featureOf(scale)) continue;
      log.push(`1:${scale} ${mode}: ${solid.id} corner radius ${1000 * solid.cornerRadius} mm removed (corner cut ${(1000 * (Math.SQRT2 - 1) * solid.cornerRadius).toFixed(1)} mm, feature ${1000 * featureOf(scale)} mm)`);
      delete solid.cornerRadius;
      ids.push(solid.id);
    }
  }
  return ids;
};

const SHARP = (scale: number): Text => ({
  en: `A box is rounded only where the drawing can show it: a corner radius below ${(1000 * featureOf(scale) / (Math.SQRT2 - 1)).toFixed(0)} mm cuts the corner by less than the smallest feature of 1:${scale} (${1000 * featureOf(scale)} mm) and is left out.`,
  de: `Ein Quader ist nur dort gerundet, wo die Zeichnung es zeigen kann: ein Eckradius unter ${(1000 * featureOf(scale) / (Math.SQRT2 - 1)).toFixed(0)} mm schneidet die Ecke um weniger als das kleinste Merkmal von 1:${scale} (${String(1000 * featureOf(scale)).replace(".", ",")} mm) und entfällt.`,
});

/** ⬛️ Turns the round texture bars of a scale whose roundness the drawing cannot show into square bars of the same width, length and place: a two-point revolve along a coordinate axis with `texture` whose circle stays within one smallest feature of its square (`(sqrt 2 - 1) r`, as in {@link sharpen}). Only texture bars: a structure member must keep the section its member states (`volumes-rules`, section agreement). Returns the ids. */
export const square = (asset: any, scale: number): string[] => {
  const v = volume(asset, scale), ids: string[] = [];
  v.body.forEach((solid, at) => {
    if (solid.kind !== "revolve" || solid.texture !== true || solid.profile?.length !== 2) return;
    const [[from, r], [to, r1]] = solid.profile as [number, number][], along = (solid.axis as number[]).findIndex((x) => Math.abs(Math.abs(x) - 1) < 1e-9);
    if (r !== r1 || along < 0 || (solid.axis as number[]).some((x, k) => k !== along && Math.abs(x) > 1e-9) || (Math.SQRT2 - 1) * r >= featureOf(scale)) return;
    const round = (x: number): number => Number(x.toFixed(6)), sign = Math.sign(solid.axis[along]);
    const swap: Record<string, [string, unknown] | null> = { kind: ["kind", "box"], profile: ["size", [0, 1, 2].map((k) => round(k === along ? to - from : 2 * r))], axis: ["transform", { translation: [0, 1, 2].map((k) => round(solid.origin[k] + (k === along ? (sign * (from + to)) / 2 : 0))) }], origin: null };
    v.body[at] = Object.fromEntries(Object.entries(solid).flatMap(([key, value]) => (key in swap ? (swap[key] === null ? [] : [swap[key]!]) : [[key, value]])));
    log.push(`1:${scale} body: ${solid.id} round bar of ${2000 * r} mm -> square bar ${JSON.stringify(v.body[at]!.size)} (circle within ${(1000 * (Math.SQRT2 - 1) * r).toFixed(1)} mm of the square, feature ${1000 * featureOf(scale)} mm)`);
    ids.push(solid.id);
  });
  return ids;
};

const SQUARE = (scale: number): Text => ({
  en: `The rungs and brackets of the ladder texture are square bars of their 25 mm: a round bar that thin stays within the smallest feature of 1:${scale} of its square, so the drawing cannot show the roundness.`,
  de: `Die Sprossen und Halter der Leitertextur sind Vierkantstäbe ihrer 25 mm: ein so dünner Rundstab bleibt innerhalb des kleinsten Merkmals von 1:${scale} an seinem Quadrat, die Zeichnung kann die Rundung nicht zeigen.`,
});

/** 🏗️ The crane: rounded boxes sharpened and round texture bars squared where the drawing cannot show the rounding. */
const crane = (asset: any): void => {
  for (const scale of Object.keys(asset.volumes).map(Number)) {
    if (sharpen(asset, scale).length > 0) append(volume(asset, scale).decisions[0]!, SHARP(scale), `1:${scale} decision`);
    if (square(asset, scale).length > 0) append(volume(asset, scale).decisions[0]!, SQUARE(scale), `1:${scale} decision`);
  }
};

/** 🗂️ The repair of each asset of the lane. */
export const repairs: Record<string, (asset: any) => void> = { "dairy-cow": thinned(false), "parent-carrying-toddler": thinned(true), "tower-crane": crane };
