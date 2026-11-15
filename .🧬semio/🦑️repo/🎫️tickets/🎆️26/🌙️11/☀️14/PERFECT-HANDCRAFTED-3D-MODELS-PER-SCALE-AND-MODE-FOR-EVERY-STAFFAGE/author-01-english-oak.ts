import { Author, type V3 } from "./author-01-kit.ts";
import { treeBody, treeFinish, type TreeCfg } from "./author-01-tree.ts";

const cfg: TreeCfg = {
  slug: "english-oak",
  woody: (b) => !/^fol/.test(b.id) && b.id !== "root-plate",
  foliage: (b) => /^fol/.test(b.id),
  pick: {
    "trunk-bark": (s) => s.material === "oak-bark" && /^(root-flare|trunk|bole|leader|root)/.test(s.id),
    "limb-bark": (s) => s.material === "oak-bark" && !/^(root-flare|trunk|bole|leader|root)/.test(s.id),
    "leaf-sun": (s) => s.material === "oak-leaf-sun",
    "leaf-shade": (s) => s.material === "oak-leaf-shade",
  },
  memberR: (b, sc) => [Math.max(0.05, 0.02 * Math.sqrt(sc / 50)), 0.04],
  emitterD: 1.5,
  notes: {},
  fallback: (b, id) => (/^(bole|limb)/.test(id) ? { material: "oak-bark", tone: 0.3 } : { material: "oak-leaf-sun", tone: 0.55 }),
  foliageExp: 2.4,
};
export function build(scale: 50 | 100 | 200 | 500): Author {
  const a = new Author(cfg.slug, scale);
  treeBody(a, cfg);
  if (scale === 500) {
    a.into("body");
    const crown = a.lists.body.findIndex((x) => x.id === "leader-mass");
    if (crown >= 0) a.lists.body.splice(crown, 1);
    const E = a.shapesOf("elevation", "leader"), F = a.shapesOf("front", "leader");
    const eb = a.box2("elevation", E);
    const ym = eb.minY + (eb.maxY - eb.minY) * 0.45;
    a.loftFromViews("crown-shade", "leader", { elev: E, front: F, range: [eb.minY, ym], n: 4, exp: 2.4, pinch: 0.5, m: a.mat("oak-leaf-shade", 0.4) });
    a.loftFromViews("crown-sun", "leader", { elev: E, front: F, range: [ym, eb.maxY], n: 4, exp: 2.4, pinch: 0.25, m: a.mat("oak-leaf-sun", 0.6) });
  }
  treeFinish(a, cfg);
  return a;
}
void (0 as unknown as V3);
