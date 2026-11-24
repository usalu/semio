/** 🌲️ The multiview repair of `european-larch` at 1:500 (idempotent, the last step of its chain): the crown was two solids of revolution with one common axis offset and one common ellipse (lateral scale 0.8392), so the front elevation was the side elevation squeezed by one factor (normalised Hausdorff 0.024, below the 0.03 of the derived-view check). The eleven storeys of the stepped envelope now are eleven solids, each on the axis offset and with the ellipse of its own whorl as the 1:200 body has them (tier-01 … tier-11: the larch is the same individual in both bands), its radii brought from the common envelope back to the reach of its whorl; every storey but the lowest ends in a stub of 0.08 m that laps into the storey below, so the crown is one connected body. The three views are redrawn from these solids as their exact outlines: the sunlit storeys 5 to 11 and the shaded storeys 1 to 4 as two crown paths per view. Thermal references and the decision texts follow.
 *  bun fix-european-larch.ts [--in file] [--out file] */
import { load, round, save } from "./lib.ts";

type P = [number, number];
const loaded = await load("european-larch");
const asset = loaded.asset;
const volume = asset.volumes["500"];
const rep = asset.representations["500"];
const bone = (id: string): any => asset.rig.bones.find((b: any) => b.id === id);
const heightOf = (id: string): number => {
  let [y, b] = [0, bone(id)];
  while (b.parent !== null && b.parent !== undefined) { y += b.x; b = bone(b.parent); }
  return y;
};
const Y0 = heightOf("trunk-2");
const tiers = asset.volumes["200"].body.filter((solid: any) => /^tier-\d\d$/u.test(solid.id)).map((solid: any) => ({ n: Number(solid.id.slice(5)), u: heightOf(solid.bone) - Y0, reach: Math.max(...solid.profile.map((p: P) => p[1])), v: solid.origin[1], w: solid.origin[2], ls: solid.lateralScale ?? 1, sun: solid.material.endsWith("-sun") }));
const FLOOR = 0.6;
const LAP = 0.08;
const name = (n: number): string => `crown-${String(n).padStart(2, "0")}`;

const sunOld = volume.body.find((solid: any) => solid.id === "crown-sun");
const shadeOld = volume.body.find((solid: any) => solid.id === "crown-shade");
if (sunOld !== undefined && shadeOld !== undefined) {
  const outline: P[] = [...sunOld.profile.slice(0, -1), ...shadeOld.profile.slice(2)];
  const notch = (low: number, high: number): number => {
    let best = -1;
    outline.forEach((p, i) => { if (p[0] > low && p[0] < high && p[1] > 0 && (best < 0 || p[1] < outline[best]![1])) best = i; });
    if (best < 0) throw new Error(`no notch between ${low} and ${high}`);
    return best;
  };
  const cuts = [0, ...tiers.slice(0, -1).map((_: unknown, k: number) => notch(tiers[tiers.length - 2 - k].u, tiers[tiers.length - 1 - k].u)), outline.length - 1];
  const storeys = tiers.map((tier: any, k: number) => {
    const span = outline.slice(cuts[tiers.length - 1 - k]!, cuts[tiers.length - k]! + 1);
    const part = span.filter((p) => p[1] > 0);
    const envelope = Math.max(...part.map((p) => p[1]));
    const radius = (r: number): number => (r <= FLOOR ? r : FLOOR + ((r - FLOOR) * (tier.reach - FLOOR)) / (envelope - FLOOR));
    const body = part.map((p): P => [p[0], round(radius(p[1]), 4)]);
    const foot: P[] = k === 0 ? [[span[span.length - 1]![0], 0]] : [[round(body[body.length - 1]![0] - LAP, 4), body[body.length - 1]![1]], [round(body[body.length - 1]![0] - LAP, 4), 0]];
    const profile: P[] = [[span[0]![0], 0], ...body, ...foot];
    const like = tier.sun ? sunOld : shadeOld;
    return { id: name(tier.n), bone: "trunk-2", material: like.material, role: "body", tone: like.tone, kind: "revolve", profile, axis: [1, 0, 0], origin: [0, tier.v, tier.w], lateralScale: tier.ls };
  });
  volume.body = [...volume.body.filter((solid: any) => solid !== sunOld && solid !== shadeOld), ...storeys];
  volume.thermal = volume.thermal.flatMap((entry: any) => (entry.id === "th-crown-sun" ? storeys.filter((s: any) => s.material.endsWith("-sun")).map((s: any) => ({ id: `th-${s.id}`, of: s.id, region: entry.region })) : entry.id === "th-crown-shade" ? storeys.filter((s: any) => s.material.endsWith("-shade")).map((s: any) => ({ id: `th-${s.id}`, of: s.id, region: entry.region })) : [entry]));
}
const storeys: any[] = volume.body.filter((solid: any) => /^crown-\d\d$/u.test(solid.id));
if (storeys.length !== tiers.length) throw new Error(`european-larch: ${storeys.length} crown storeys at 1:500, ${tiers.length} whorls at 1:200`);

const radiusAt = (profile: readonly P[], u: number): number | undefined => {
  let best: number | undefined;
  for (let i = 0; i + 1 < profile.length; i++) {
    const [a, b] = [profile[i]!, profile[i + 1]!];
    const [low, high] = a[0] < b[0] ? [a, b] : [b, a];
    if (u < low[0] - 1e-9 || u > high[0] + 1e-9) continue;
    const r = high[0] === low[0] ? Math.max(low[1], high[1]) : low[1] + ((u - low[0]) / (high[0] - low[0])) * (high[1] - low[1]);
    best = best === undefined ? r : Math.max(best, r);
  }
  return best;
};
const near = (a: P, b: P, c: P, eps: number): boolean => {
  const [dx, dy] = [c[0] - a[0], c[1] - a[1]];
  const length = Math.hypot(dx, dy);
  return (length === 0 ? Math.hypot(b[0] - a[0], b[1] - a[1]) : Math.abs(dx * (b[1] - a[1]) - dy * (b[0] - a[0])) / length) <= eps;
};
const thin = (points: P[], eps = 0.002): P[] => {
  const out = points.slice();
  for (let i = 1; i + 1 < out.length; ) if (near(out[i - 1]!, out[i]!, out[i + 1]!, eps)) out.splice(i, 1); else i++;
  return out;
};
/** The two sides of the union of stacked storeys: per axial position the lowest and the highest lateral reach, `centre ± scale · radius`. */
const sides = (group: readonly any[], centre: (s: any) => number, scale: (s: any) => number): { low: P[]; high: P[] } => {
  const stations = [...new Set(group.flatMap((s) => s.profile.map((p: P) => p[0])))].sort((a, b) => b - a);
  const [low, high]: [P[], P[]] = [[], []];
  for (const u of stations) {
    for (const side of [1e-7, -1e-7]) {
      let [lo, hi] = [Infinity, -Infinity];
      for (const s of group) {
        const r = radiusAt(s.profile, u + side);
        if (r === undefined) continue;
        lo = Math.min(lo, centre(s) - scale(s) * r);
        hi = Math.max(hi, centre(s) + scale(s) * r);
      }
      if (lo === Infinity) continue;
      low.push([u, lo]);
      high.push([u, hi]);
    }
  }
  return { low: thin(low), high: thin(high) };
};
const text = (points: readonly P[]): string => `M ${points.map((p) => `${round(p[0], 4)} ${round(p[1], 4)}`).join(" L ")} Z`;
const groups = { "crown-sun": storeys.filter((s) => s.material.endsWith("-sun")), "crown-shade": storeys.filter((s) => s.material.endsWith("-shade")) };
const redraw = (view: string, id: string, d: string): void => {
  const list = rep.geometry[view];
  const index = list.findIndex((shape: any) => shape.id === id);
  if (index < 0) throw new Error(`european-larch 1:500 ${view}: no shape "${id}"`);
  const { kind: _k, d: _d, points: _p, closed: _c, ...rest } = list[index];
  list[index] = { ...rest, kind: "path", d };
};
for (const [id, group] of Object.entries(groups)) {
  const side = sides(group, (s) => s.origin[1], () => 1);
  redraw("elevation", id, text([...side.low, ...side.high.reverse()]));
  const face = sides(group, (s) => s.origin[2], (s) => s.lateralScale);
  redraw("front", id, text([...face.high.map((p): P => [p[1], Y0 + p[0]]), ...face.low.reverse().map((p): P => [p[1], Y0 + p[0]])]));
  const ellipses = group.map((s) => ({ x: -s.origin[1], z: s.origin[2], a: Math.max(...s.profile.map((p: P) => p[1])), b: s.lateralScale * Math.max(...s.profile.map((p: P) => p[1])) }));
  const [cx, cz] = [ellipses.reduce((sum, e) => sum + e.x, 0) / ellipses.length, ellipses.reduce((sum, e) => sum + e.z, 0) / ellipses.length];
  const ring: P[] = [];
  for (let k = 0; k < 48; k++) {
    const [dx, dz] = [Math.sin((k * Math.PI) / 24), Math.cos((k * Math.PI) / 24)];
    let reach = 0;
    for (const e of ellipses) {
      const [px, pz] = [(cx - e.x) / e.a, (cz - e.z) / e.b];
      const [qx, qz] = [dx / e.a, dz / e.b];
      const [A, B, C] = [qx * qx + qz * qz, 2 * (px * qx + pz * qz), px * px + pz * pz - 1];
      const disc = B * B - 4 * A * C;
      if (disc >= 0) reach = Math.max(reach, (-B + Math.sqrt(disc)) / (2 * A));
    }
    ring.push([cx + reach * dx, cz + reach * dz]);
  }
  redraw("plan", id, text(ring));
}
const top = round(Y0 + Math.min(...groups["crown-sun"].flatMap((s) => s.profile.map((p: P) => p[0]))) + LAP, 1);
const d = rep.decisions;
d.silhouette = {
  en: "At 1:500 (1 m is 2 mm, the tree 60 mm high) the larch is one open, spiky cone on a tall bare stem: the crown paths are the true views of the eleven storey solids of the 1:500 crown, whose envelope steps in to the stem between the storeys, so the airy layering still reads. Every storey sits on the axis offset and has the ellipse of its own whorl, as at 1:200, so the front elevation is another drawing than the side elevation: the lowest storey reaches 4.6 m to the side and 3.8 m to the front.",
  de: "Bei 1:500 (1 m sind 2 mm, der Baum 60 mm hoch) ist die Lärche ein lichter, zackiger Kegel auf hohem kahlem Stamm: die Kronenpfade sind die wahren Ansichten der elf Etagenkörper der 1:500-Krone, deren Hülle zwischen den Etagen bis an den Stamm einspringt, so dass die luftige Schichtung lesbar bleibt. Jede Etage sitzt auf dem Achsversatz und hat die Ellipse ihres eigenen Quirls wie bei 1:200, daher ist die Vorderansicht eine andere Zeichnung als die Seitenansicht: die unterste Etage reicht 4,6 m zur Seite und 3,8 m nach vorn.",
};
d.merged = {
  en: `Merged: branch ends and pendulous branchlets of each whorl into one storey solid, the eleven storeys drawn as two crown paths (sunlit from ${top} m, shaded below); the stem inside the crown into the crown.`,
  de: `Vereinigt: Astenden und hängende Zweige jedes Quirls zu einem Etagenkörper, die elf Etagen gezeichnet als zwei Kronenpfade (besonnt ab ${String(top).replace(".", ",")} m, beschattet darunter); der Stamm in der Krone in die Krone.`,
};
volume.decisions[0] = {
  en: "Body at 1:500 (smallest feature 75 mm): the stem with its flare up into the crown and the crown as eleven solids of revolution, one per storey of the stepped envelope of the 1:50 crown, each with the axis offset and the elliptic section of its whorl and never thinner than the stem; 12 solids.",
  de: "Körper bei 1:500 (kleinstes Merkmal 75 mm): der Stamm mit Anlauf bis in die Krone und die Krone als elf Drehkörper, einer je Etage der gestuften Hülle der 1:50-Krone, jeder mit dem Achsversatz und dem elliptischen Querschnitt seines Quirls und nie dünner als der Stamm; 12 Körper.",
};
volume.decisions[1] = {
  en: volume.decisions[1].en.replace("the two crown solids", "the eleven crown solids"),
  de: volume.decisions[1].de.replace("die zwei Kronenkörper", "die elf Kronenkörper"),
};
console.log(`european-larch: ${storeys.length} storeys (${groups["crown-sun"].length} sunlit from ${top} m), file ${(await save(loaded)) ? "written" : "unchanged"}`);
