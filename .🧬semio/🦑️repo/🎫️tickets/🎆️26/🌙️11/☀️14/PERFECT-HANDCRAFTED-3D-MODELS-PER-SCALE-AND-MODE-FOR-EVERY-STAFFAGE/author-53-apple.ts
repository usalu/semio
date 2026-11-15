import { buildAll, treeDecisions, type TreeCfg } from "./author-53-tree.ts";
import { ROUND, boxOf, dirLocal, openCtx, ringsOf, saveLists, toLocal, type Ctx, type Json, type V3 } from "./author-53-lib.ts";

const SPURS = ["spur-w1", "spur-e1", "spur-w2", "spur-e2"];

export const cfg: TreeCfg = {
  slug: "apple-tree",
  stems: ["root-plate", "trunk", "ld", "ld-b"],
  stemFront: ["root-plate", "trunk", "ld", "ld-b"],
  woody: ["sc-w1a", "sc-w1b", "sc-e1a", "sc-e1b", "sc-w2a", "sc-w2b", "sc-e2a", "sc-e2b"],
  pairs: [["fol-w1", "fol-e1"], ["fol-w2", "fol-e2"]],
  singles: ["fol-core-low", "fol-core-up", "fol-top"],
  crown500: "ld",
  bark: { stem: "trunk-bark", limb: "limb-bark", stemMat: "apple-bark", limbMat: "apple-bark-young" },
  barkOf: (b) => (SPURS.includes(b) ? "spur-twig" : b === "root-plate" || b === "trunk" ? "trunk-bark" : "limb-bark"),
  leaf: { sunRegion: "leaf-sun", shadeRegion: "leaf-shade", shade: (b) => /^fol-core/.test(b) },
  tone: (b) => (/^fol-core/.test(b) ? 0.4 : /^fol-top/.test(b) ? 0.62 : /^fol/.test(b) ? 0.56 : /^sc-/.test(b) ? 0.3 : 0.28),
  mat: (b) => (/^fol-core/.test(b) ? "apple-leaf-shade" : /^fol/.test(b) ? "apple-leaf-sun" : b === "root-plate" || b === "trunk" ? "apple-bark" : SPURS.includes(b) ? "apple-twig" : "apple-bark-young"),
  override: { 500: { stems: ["root-plate", "trunk"], stemFront: ["root-plate", "trunk"], woody: [], pairs: [], singles: ["ld"] } },
  scale: { 50: { tolLobe: 0.02, tolLimb: 0.006, stations: 24, bias: 0.02 }, 100: { tolLobe: 0.03, tolLimb: 0.012, stations: 16, bias: 0.02 }, 200: { tolLobe: 0.04, tolLimb: 0.024, stations: 10, bias: 0.02 }, 500: { tolLobe: 0.06, tolLimb: 0.075, stations: 8, bias: 0 } },
  extra: (c: Ctx, built, woodMat: string) => {
    const body: Json[] = [];
    const thermal: Json[] = [];
    const structure: Json[] = [];
    const rig = c.raw.rig.bones as Json[];
    const f = c.feature;
    const drawnTwig = (c.raw.representations[String(c.scale)].geometry.elevation as Json[]).some((s) => /^twig-spur/.test(s.id));
    const asTexture = (c.raw.representations[String(c.scale)].geometry.elevation as Json[]).some((s) => /^twig-spur/.test(s.id) && s.role === "texture");
    for (const id of SPURS) {
      const b = rig.find((x) => x.id === id)!;
      const fr = c.state.frames[id]!;
      const parent = b.parent as string;
      const pp = built.paths[parent];
      let z0 = 0;
      if (pp !== undefined) {
        let best = Infinity;
        for (const q of pp) {
          const d = Math.hypot(q.P[0] - fr.o[0], q.P[1] - fr.o[1]);
          if (d < best) [best, z0] = [d, q.P[2]];
        }
      }
      const L = b.lengthM as number;
      const p0: V3 = [fr.o[0], fr.o[1], z0];
      const p1: V3 = [fr.o[0] + L * fr.u[0], fr.o[1] + L * fr.u[1], z0];
      const r = Math.sqrt((b.section?.areaM2 ?? 3.1e-4) / Math.PI);
      const twig: Json = { id, kind: "capsule", bone: id, material: "apple-twig", role: "body", tone: 0.3, p0: toLocal(c, id, p0).map(ROUND), p1: toLocal(c, id, p1).map(ROUND), r0: ROUND(r), r1: ROUND(r), slender: true };
      if (drawnTwig) {
        body.push(asTexture ? { ...twig, texture: true } : twig);
        thermal.push({ id: `th-${id}`, of: id, region: "spur-twig" });
      } else thermal.push({ ...twig, id: `th-${id}`, role: "detail", region: "spur-twig" });
      structure.push({ ...twig, id: `member-${id}`, role: "body", material: woodMat, tone: 0.4, member: id, part: "member" });
    }
    const fruitShapes = (c.state.elevation as Json[]).filter((s) => s.shape.region === "fruit" && s.closed && s.rings.length > 0);
    const planFruit = (c.state.plan as Json[]).filter((s) => s.shape.region === "fruit" && s.rings.length > 0);
    const frontFruit = (c.state.front as Json[]).filter((s) => s.shape.region === "fruit" && s.rings.length > 0);
    const mid = (s: Json): { x: number; y: number; r: number } => {
      const bx = boxOf(s.rings);
      return { x: (bx.minX + bx.maxX) / 2, y: (bx.minY + bx.maxY) / 2, r: Math.max(bx.maxX - bx.minX, bx.maxY - bx.minY) / 2 };
    };
    for (const s of fruitShapes) {
      const e = mid(s);
      const pl = planFruit.find((q) => q.shape.id === s.shape.id);
      const fr = frontFruit.find((q) => q.shape.id === s.shape.id);
      const bone = s.shape.bone as string;
      const lobe = built.lobes[`${bone}-mass`];
      const z = pl !== undefined ? mid(pl).y : fr !== undefined ? mid(fr).x : (lobe?.centre[2] ?? 0);
      const x = pl !== undefined ? (e.x + mid(pl).x) / 2 : e.x;
      const id = `fruit-${s.shape.id.replace(/^fol-/, "").replace(/-fruit/, "-")}`;
      const rad = Math.max(e.r, 0.55 * f);
      body.push({ id, kind: "ellipsoid", bone, material: "apple-fruit", role: "detail", tone: 0.7, radii: [rad, rad, rad].map(ROUND), transform: { translation: toLocal(c, bone, [x, e.y, z]).map(ROUND) } });
      thermal.push({ id: `th-${id}`, of: id, region: "fruit" });
    }
    if (fruitShapes.length === 0) {
      const lobes = Object.entries(built.lobes).filter(([k]) => !/core/.test(k)).slice(0, 3);
      lobes.forEach(([k, l], i) => {
        const rad = Math.max(0.12 * Math.sqrt(c.scale / 50), 0.55 * f);
        const at: V3 = [l.centre[0] + 0.55 * l.r[0], l.centre[1] - 0.2 * l.r[1], l.centre[2]];
        thermal.push({ id: `th-fruit-${i + 1}`, kind: "ellipsoid", bone: l.bone, material: "apple-fruit", role: "detail", tone: 0.7, radii: [rad, rad, rad].map(ROUND), transform: { translation: toLocal(c, l.bone, at).map(ROUND) }, region: "fruit" });
        void k;
      });
    }
    void [dirLocal, ringsOf];
    return { body, thermal, structure };
  },
};

if (import.meta.main) {
  if (process.env.BIAS) for (const k of Object.keys(cfg.scale)) cfg.scale[Number(k)]!.bias = Number(process.env.BIAS);
  if (process.env.TOL) for (const k of Object.keys(cfg.scale)) cfg.scale[Number(k)]!.tolLobe = Number(process.env.TOL);
  const scale = Number(process.argv[2]);
  const c = openCtx("vegetation/apple-tree", scale);
  const exp = Number(process.argv[3] ?? 2.0);
  const r = buildAll(c, cfg, { exp, cross: process.argv[4] === "x", joints: scale === 50, core: 0.75, category: "vegetation", split: false });
  console.log(`body ${r.body.length} thermal ${r.thermal.length} structure ${r.structure.length}`, r.built.notes);
  const words = {
    name: { en: "Apple tree", de: "Apfelbaum" },
    coarse: {
      50: { en: "Trunk, leader fork and scaffold limbs are lofts on three-dimensional centre lines, the four spur twigs are slender capsules of their true 10 mm radius (drawn as bark texture at this scale, so flagged texture), the drawn fruit are spheres placed where the three views agree on average, and the crown is the five drawn foliage groups plus the plan drawing's fills as quadrant lobes.", de: "Stamm, Stammgabel und Gerüstäste sind Lofts auf räumlichen Mittellinien, die vier Fruchtspieße sind schlanke Kapseln mit ihrem wahren Radius von 10 mm (in diesem Maßstab als Rindenstruktur gezeichnet, daher als texture markiert), die gezeichneten Früchte sind Kugeln an dem Ort, auf dem sich die drei Ansichten im Mittel einigen, und die Krone besteht aus den fünf gezeichneten Laubgruppen und den Füllungen des Grundrisses als Quadrantenlappen." },
      100: { en: "The spur twigs are drawn as open strokes at 1:100 and stay slender capsules of true size (compared as pen-wide strokes), fruit are drawn no more and live as small thermal spheres inside the crown, the lobes follow the coarser outlines.", de: "Die Fruchtspieße sind bei 1:100 als offene Striche gezeichnet und bleiben schlanke Kapseln in wahrer Größe (als stiftbreite Striche verglichen), Früchte werden nicht mehr gezeichnet und stehen als kleine thermische Kugeln in der Krone, die Lappen folgen den gröberen Konturen." },
      200: { en: "At 1:200 the foliage is drawn as the plan fills and the top group; the spur twigs are no longer drawn and survive as slender thermal capsules and structural members only.", de: "Bei 1:200 ist das Laub als Grundrissfüllungen und Gipfelgruppe gezeichnet; die Fruchtspieße werden nicht mehr gezeichnet und bestehen nur als schlanke thermische Kapseln und Tragglieder fort." },
      500: { en: "At 1:500 the crown is one hull lobe on a bole loft; leader, scaffold limbs and spurs are hidden in the crown and live in the structure list as boxes.", de: "Bei 1:500 ist die Krone ein Hüllenlappen auf einem Schaft-Loft; Stammverlängerung, Gerüstäste und Spieße liegen verdeckt in der Krone und stehen in der Tragwerksliste als Quader." },
    } as Record<number, { en: string; de: string }>,
    hard: { en: "", de: "" },
  };
  const decisions = treeDecisions(c, r, words);
  saveLists(c, { body: r.body, thermal: r.thermal, structure: r.structure, decisions });
}
