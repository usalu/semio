/** ⏱️ Progress and cancellation of the model exports: the staffage `JobControl` (signal, bilingual progress) adapted to the kernel `Operation` (fraction, cancelled), split into weighted phases so one bar runs from 0 to 1 over modelling, tessellation, edges and encoding, and the tessellation quality of an asset.
 *
 * Cancellation throws the signal's reason (an `AbortError` when the signal carries none), as every other staffage export does; the kernel's own cancellation errors are translated.
 *
 * @see ../../🔗️contracts/🟦️.ts — `JobControl`
 */
import { VOLUME_LISTS, type Asset, type AuthoredVolume, type ScaleVolume, type Text, type VolumeSolid } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { CancelledError, type Operation, type Quality } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { circleSegments } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/📏️segments/🟦️.ts";
import type { JobControl } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔗️contracts/🟦️.ts";
import { listOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🧊️solids/🟦️.ts";

/** 🧩️ One weighted phase of a job. */
export type Phase = { readonly label: Text; readonly weight: number };

/** ⏳️ A job split into phases: `operation(i)` is the kernel operation of phase `i`; progress is reported as permille of the whole. */
export type Job = { readonly operation: (phase: number) => Operation; readonly finish: () => void };

const PERMILLE = 1000;

/** 📤️ The {@link Job} of a control over weighted phases. */
export function jobOf(control: JobControl, phases: readonly Phase[]): Job {
  const total = phases.reduce((sum, p) => sum + p.weight, 0);
  const before = (index: number): number => phases.slice(0, index).reduce((sum, p) => sum + p.weight, 0) / total;
  return {
    operation: (index) => ({
      report: (fraction) => control.progress?.(Math.round((before(index) + (Math.max(0, Math.min(1, fraction)) * phases[index]!.weight) / total) * PERMILLE), PERMILLE, phases[index]!.label),
      cancelled: () => control.signal?.aborted === true,
    }),
    finish: () => control.progress?.(PERMILLE, PERMILLE, phases[phases.length - 1]!.label),
  };
}

/** 🛑️ Runs `work` and turns the kernel's cancellation errors into the signal's reason. */
export async function cancellable<T>(control: JobControl, work: () => Promise<T>): Promise<T> {
  try {
    return await work();
  } catch (error) {
    if (error instanceof CancelledError || (error instanceof Error && error.name === "FbxCancelledError")) throw control.signal?.reason ?? new DOMException("This operation was aborted", "AbortError");
    throw error;
  }
}

/** 📏️ The roundness rule of the 3D models: the smallest feature of a scale is `paperFeatureM` of paper (0.15 mm: 7.5, 15, 30 and 75 mm at 1:50, 1:100, 1:200 and 1:500); the chord deviation is at most `chordPerFeature` of that feature and at most 1/`largestPerChord` of the largest dimension of the asset (a quarter of a pixel when the asset fills 800 pixels); a full circle whose radius exceeds one feature has at least `roundSegments` segments and no circle fewer than `minSegments`; the validators measure at a chord of 1/`referencePerChord` of the largest dimension, clamped to `referenceChordM`. All of it holds on the full roundness level `fullLevel`; the solids of all authored scales of one asset hold at most `modelTriangles` nominal triangles ({@link nominalTriangles}), and a heavier model, or a scale whose lists would grow, is on a lower level ({@link roundnessLevels}). */
export const ROUNDNESS = { paperFeatureM: 0.00015, chordPerFeature: 0.02, largestPerChord: 3200, roundSegments: 24, minSegments: 12, referencePerChord: 800, referenceChordM: [0.0005, 0.02], fullLevel: 64, modelTriangles: 300000 } as const;

/** 🧢️ The most segments of one full circle at a scale on the full roundness level: 96 up to 1:20, 64 up to 1:50, 48 up to 1:100, 40 up to 1:200 and 32 beyond, so that a coarser scale never has more triangles of the same solid than a finer one. */
export const maxSegmentsAt = (scale: number): number => (scale <= 20 ? 96 : scale <= 50 ? 64 : scale <= 100 ? 48 : scale <= 200 ? 40 : 32);

/** 🪜️ The most segments of one full circle at a scale on roundness level `level`: on the full level the cap of {@link maxSegmentsAt}; below it every cap falls in proportion but not under the round floor of 24; below 24 every cap is the level itself. */
export const levelSegmentsAt = (scale: number, level: number): number => (level >= ROUNDNESS.roundSegments ? Math.max(ROUNDNESS.roundSegments, Math.floor((maxSegmentsAt(scale) * level) / ROUNDNESS.fullLevel)) : Math.min(maxSegmentsAt(scale), level));

/** 🔢️ The nominal triangles of an authored solid at a quality, the measure of the triangle budget: the grid of its tessellator without seams, creases and end discs. With n the segments of its largest circle: an ellipsoid 2 n (⌈n/2⌉ − 1), a capsule 4 n ⌈n/4⌉, a loft n (2 (sections − 1) + caps), a revolve 2 columns per profile point (n columns for a full turn), a rounded box 12 (2 ⌈n/4⌉ + 1)², a box 12, an extrusion 4 per outline point, a polyhedron its triangles; times its copies. */
export function nominalTriangles(solid: VolumeSolid, quality: Quality): number {
  const copies = solid.repeat?.count ?? 1;
  const quarter = (n: number): number => Math.max(2, Math.ceil(n / 4));
  switch (solid.kind) {
    case "ellipsoid": {
      const n = circleSegments(Math.max(...solid.radii), quality);
      return copies * 2 * n * (Math.max(2, Math.ceil(n / 2)) - 1);
    }
    case "capsule": {
      const n = circleSegments(Math.max(solid.r0, solid.r1) * Math.max(1, solid.lateralScale ?? 1), quality);
      return copies * 4 * n * quarter(n);
    }
    case "loft": {
      const n = Math.max(4, circleSegments(Math.max(...solid.sections.map((section) => Math.max(section.rx, section.ry))), quality));
      return copies * n * (2 * (solid.sections.length - 1) + Number(solid.capStart) + Number(solid.capEnd));
    }
    case "revolve": {
      const n = circleSegments(Math.max(...solid.profile.map((point) => point[1])) * Math.max(1, solid.lateralScale ?? 1), quality);
      return copies * 2 * Math.max(1, Math.ceil((n * Math.min(solid.sweepRad ?? 2 * Math.PI, 2 * Math.PI)) / (2 * Math.PI))) * solid.profile.length;
    }
    case "box": {
      const radius = Math.min(solid.cornerRadius ?? 0, ...solid.size.map((edge) => edge / 2));
      return copies * (radius > 0 ? 12 * (2 * quarter(circleSegments(radius, quality)) + 1) ** 2 : 12);
    }
    case "extrude":
      return copies * 4 * (solid.profile.length + (solid.holes ?? []).reduce((sum, hole) => sum + hole.length, 0));
    case "polyhedron":
      return copies * solid.triangles.length;
  }
}

/** 🎛️ The quality of an asset at a scale on roundness level `level`: the rule on the full level; below it the chord deviation grows by (`fullLevel` / level)², so that a circle the chord decides has about level / `fullLevel` of its segments as a capped one has ({@link levelSegmentsAt}), while the floors of 24 and 12 segments stay. */
export function qualityOn(asset: Asset, scale: number, level: number): Quality {
  const featureM = ROUNDNESS.paperFeatureM * scale;
  const chordM = Math.min(Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM) / ROUNDNESS.largestPerChord, ROUNDNESS.chordPerFeature * featureM);
  return { chordM: chordM * (ROUNDNESS.fullLevel / level) ** 2, minSegments: ROUNDNESS.minSegments, maxSegments: levelSegmentsAt(scale, level), round: { radiusM: featureM, segments: ROUNDNESS.roundSegments } };
}

/** 🧮️ The nominal triangles of one authored scale on a roundness level: every solid of its body, thermal and structure lists once (a reference adds nothing to the solid it reuses). */
export function volumeTriangles(asset: Asset, volume: AuthoredVolume, level: number): number {
  const quality = qualityOn(asset, volume.denominator, level);
  return [...volume.body, ...volume.thermal, ...volume.structure].reduce((sum, solid) => sum + ("kind" in solid ? nominalTriangles(solid, quality) : 0), 0);
}

/** 🏋️ The roundness levels of an asset: `model` for a scale it has not authored, `scales` per authored denominator. */
export type RoundnessLevels = { readonly model: number; readonly scales: ReadonlyMap<number, number> };

const remembered = new WeakMap<Asset, RoundnessLevels>();

/** 🪂️ The roundness levels of an asset, remembered per asset. `model` is the full level unless the solids of all its authored scales hold more than `modelTriangles` nominal triangles ({@link volumeTriangles}), then the highest level on which they stay within it, never below `minSegments`: a tree of some hundred foliage masses or a figure with its tools trades rounder for lighter, so the triangle and file budgets of its 3D files hold, and every model within the budget is tessellated by the rule alone. A scale is on the model's level unless one of its lists (references resolved) would then hold more nominal triangles than the same list one scale finer, as happens where markers and rods at the smallest feature grow with the scale: then on the highest level on which no list grows. */
export function roundnessLevels(asset: Asset): RoundnessLevels {
  const known = remembered.get(asset);
  if (known !== undefined) return known;
  const volumes = Object.values((asset.volumes ?? {}) as Readonly<Record<string, ScaleVolume>>).filter((volume): volume is AuthoredVolume => volume.status === "authored").sort((a, b) => a.denominator - b.denominator);
  const lists = (volume: AuthoredVolume, level: number): number[] => VOLUME_LISTS.map((list) => listOf(volume, list).reduce((sum, solid) => sum + nominalTriangles(solid, qualityOn(asset, volume.denominator, level)), 0));
  let model: number = ROUNDNESS.fullLevel;
  while (model > ROUNDNESS.minSegments && volumes.reduce((sum, volume) => sum + volumeTriangles(asset, volume, model), 0) > ROUNDNESS.modelTriangles) model -= 1;
  const scales = new Map<number, number>();
  volumes.forEach((volume, index) => {
    let level = model;
    const finer = index === 0 ? undefined : lists(volumes[index - 1]!, scales.get(volumes[index - 1]!.denominator)!);
    while (finer !== undefined && level > ROUNDNESS.minSegments && lists(volume, level).some((triangles, list) => triangles > finer[list]!)) level -= 1;
    scales.set(volume.denominator, level);
  });
  const levels = { model, scales };
  remembered.set(asset, levels);
  return levels;
}

/** 🎚️ The tessellation quality of an asset at a scale, the one rule of the viewer, every download and the budget oracle ({@link ROUNDNESS}): adaptive to curvature and size, the chord deviation is the smaller of 2 % of the smallest feature of the scale and 1/3200 of the largest dimension, a circle larger than one feature never has fewer than 24 segments, and the segments are capped by {@link maxSegmentsAt}; a model heavier than the triangle budget, and a scale whose lists would grow, is tessellated on its lower roundness level ({@link roundnessLevels}, {@link qualityOn}). Without a scale it is the reference of the validators that measure silhouettes, areas and volumes: a chord of 1/800 of the largest dimension (0.5 to 20 mm) with 24 to 64 segments on every circle. */
export function qualityOf(asset: Asset, scale?: number): Quality {
  const largest = Math.max(asset.dimensions.heightM, asset.dimensions.widthM, asset.dimensions.depthM);
  if (scale === undefined) return { chordM: Math.min(ROUNDNESS.referenceChordM[1], Math.max(ROUNDNESS.referenceChordM[0], largest / ROUNDNESS.referencePerChord)), minSegments: ROUNDNESS.roundSegments, maxSegments: 64 };
  const levels = roundnessLevels(asset);
  return qualityOn(asset, scale, levels.scales.get(scale) ?? levels.model);
}

/** 🏷️ The phases every export shares. */
export const PHASES = {
  model: { en: "Building the model", de: "Modell wird aufgebaut" },
  tessellate: { en: "Tessellating solids", de: "Körper werden vernetzt" },
  edges: { en: "Finding feature edges", de: "Kanten werden gesucht" },
  encode: { en: "Encoding", de: "Kodiere" },
} as const satisfies Record<string, Text>;
