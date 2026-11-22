/** ✍️ PW-04 design notation: expands the compact hand tables of a species (stem stations, limb rows, branch rows, cluster rows) into the {@link Species} parts. The numbers are the author's; this only chains segments and places points.
 *
 * @see ./tree-lib.ts */
import { T, add, dir, lerp, mul, type Branch, type Cluster, type FoliageBone, type Scale, type Text, type V3, type WoodBone } from "./tree-lib.ts";

/** 🪜️ A stem station: height, lateral wobble (x, z) and radius. */
export type Station = [y: number, x: number, z: number, radius: number];

/** 🌲️ The stem: the trunk from the root collar to the first station and the leader segments between the following stations. */
export function stem(stations: readonly Station[], rootLengthM: number, trunkTitle: Text, leaderTitle: (i: number) => Text): WoodBone[] {
  const out: WoodBone[] = [];
  let prev: V3 = [0, rootLengthM, 0];
  let parent = "root-plate";
  let r0 = stations[0]![3];
  stations.forEach(([y, x, z, rr], i) => {
    const id = i === 0 ? "trunk" : `leader-${i}`;
    const b: V3 = [x, y, z];
    out.push({ id, parent, a: prev, b, r0: i === 0 ? r0 * 1.06 : r0, r1: rr, level: i === 0 ? 0 : 1, title: i === 0 ? trunkTitle : leaderTitle(i), region: "stem-bark" });
    prev = b;
    parent = id;
    r0 = rr;
  });
  return out;
}

/** 🦾️ One row of a scaffold limb: id, the stem bone and the fraction along it where it starts, azimuth (from ahead towards the left), base radius and the segments (elevation, length, end radius); the last segment ends in a foliage bone. */
export type LimbRow = { id: string; on: string; t: number; az: number; r0: number; segs: [elevationDeg: number, lengthM: number, radiusEnd: number, azimuthTurnDeg?: number][]; title: Text };

/** 🦴️ The bones of a limb row, its foliage bone and the points along it. */
export function limb(row: LimbRow, stemBones: readonly WoodBone[]): { wood: WoodBone[]; foliage: FoliageBone; points: V3[] } {
  const host = stemBones.find((b) => b.id === row.on)!;
  let a = lerp(host.a, host.b, row.t);
  let r0 = row.r0;
  let az = row.az;
  const wood: WoodBone[] = [];
  const points: V3[] = [a];
  row.segs.forEach(([el, L, r1, turn], i) => {
    az += turn ?? 0;
    const b = add(a, mul(dir(az, el), L));
    const id = i === 0 ? row.id : `${row.id}-${i + 1}`;
    wood.push({ id, parent: i === 0 ? row.on : i === 1 ? row.id : `${row.id}-${i}`, a, b, r0, r1, level: i === 0 ? 2 : 3, title: i === 0 ? row.title : T(`${row.title.en}, outer part ${i}`, `${row.title.de}, äußerer Teil ${i}`), region: "limb-bark" });
    points.push(b);
    a = b;
    r0 = r1;
  });
  const last = row.segs[row.segs.length - 1]!;
  const tipDir = dir(az, Math.min(70, last[0] + 15));
  const foliage: FoliageBone = { id: `fol-${row.id}`, parent: wood[wood.length - 1]!.id, a, b: add(a, mul(tipDir, 0.4)), title: T(`Foliage of ${row.title.en.toLowerCase()}`, `Laub von ${row.title.de}`) };
  return { wood, foliage, points };
}

/** 🌿️ One side branch row on a limb segment: fraction along it, azimuth turn and elevation, length, base radius, end radius, the scales it is drawn at. */
export type BranchRow = [segment: string, t: number, dAz: number, el: number, L: number, r0: number, r1: number, scales?: readonly Scale[]];

/** 🍃️ Expands branch rows on the wood bones of a species. */
export function branches(rows: readonly BranchRow[], wood: readonly WoodBone[], azOf: (id: string) => number): Branch[] {
  return rows.map(([seg, t, dAz, el, L, r0, r1, scales], i) => {
    const w = wood.find((b) => b.id === seg)!;
    const a = lerp(w.a, w.b, t);
    return { id: `br-${seg}-${i + 1}`, host: seg, a, b: add(a, mul(dir(azOf(seg) + dAz, el), L)), r0, r1, scales: scales ?? [100, 50] };
  });
}

/** ☁️ One cluster row: id, host bone, centre as an offset from a named point (or absolute), semi-axes, region, 1:500 mass and seed. */
export type ClusterRow = [id: string, host: string, at: V3, rad: V3, region: "leaf-sun" | "leaf-shade", mass: string, seed: number];

/** ☁️ Expands cluster rows. */
export const clusters = (rows: readonly ClusterRow[]): Cluster[] => rows.map(([id, host, c, rad, region, mass, seed]) => ({ id, host, c, rad, region, mass, seed }));

/** 🍂️ One cluster along a limb: fraction of the limb length, size factor, lift above the axis, sideways offset (left of the limb is +), region. */
export type AlongRow = [f: number, size: number, lift: number, side: number, region: "leaf-sun" | "leaf-shade"];

/** 🌳️ The clusters of one limb from its along-rows: the point at that fraction of the chain, lifted and shifted sideways, hosted by the bone of that part (the foliage bone at the tip), with world semi-axes `base` times the size factor. */
export function along(l: { wood: WoodBone[]; foliage: FoliageBone; points: V3[] }, rows: readonly AlongRow[], base: V3, mass: string, seed: number): ClusterRow[] {
  const segLen = l.wood.map((w) => Math.hypot(w.b[0] - w.a[0], w.b[1] - w.a[1], w.b[2] - w.a[2]));
  const total = segLen.reduce((s, x) => s + x, 0);
  return rows.map(([f, size, lift, side, region], i) => {
    let at = f * total;
    let k = 0;
    while (k < segLen.length - 1 && at > segLen[k]!) at -= segLen[k++]!;
    const w = l.wood[k]!;
    const t = Math.min(1, at / segLen[k]!);
    const p = lerp(w.a, w.b, t);
    const dx = w.b[0] - w.a[0];
    const dz = w.b[2] - w.a[2];
    const h = Math.hypot(dx, dz) || 1;
    const c: V3 = [p[0] - (dz / h) * side, p[1] + lift, p[2] + (dx / h) * side];
    const host = f >= 0.97 ? l.foliage.id : w.id;
    return [`${l.wood[0]!.id.replace("limb-", "cl-").replace("main-", "cl-m").replace("riser-", "cl-r").replace("upper-", "cl-u")}-${i + 1}`, host, c, mul(base, size), region, mass, seed + i];
  });
}
