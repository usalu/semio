import { Author } from "./author-01-kit.ts";
import { treeBody, treeFinish, type TreeCfg } from "./author-01-tree.ts";

const cfg: TreeCfg = {
  slug: "apple-tree",
  woody: (b) => !/^fol|^spur|root-plate/.test(b.id),
  foliage: (b) => /^fol/.test(b.id),
  pick: {
    "trunk-bark": (s) => s.material === "apple-bark",
    "limb-bark": (s) => s.material === "apple-bark-young",
    "spur-twig": (s) => s.material === "apple-twig",
    "leaf-sun": (s) => s.material === "apple-leaf-sun",
    "leaf-shade": (s) => s.material === "apple-leaf-shade",
    fruit: (s) => s.material === "apple-fruit",
  },
  memberR: (b, sc) => [Math.max(0.03, 0.02 * Math.sqrt(sc / 50)), 0.03],
  emitterD: 1.0,
  fallback: (b, id) => (/^(stem|wood)/.test(id) ? { material: "apple-bark", tone: 0.35 } : { material: "apple-leaf-sun", tone: 0.55 }),
  regionBones: { "spur-twig": /^spur/, "limb-bark": /^sc-.*a$/, fruit: /^fol-top$/, "leaf-shade": /^fol-core-low$/, "leaf-sun": /^fol-top$/ },
  notes: {},
};
export function build(scale: 50 | 100 | 200 | 500): Author {
  const a = new Author(cfg.slug, scale);
  treeBody(a, cfg);
  a.into("body");
  const frs = ["w1", "e1", "w2", "e2", "top"];
  if (scale <= 100) frs.forEach((k, i) => { const c = [[-1.4, 2.2, 1.0], [1.5, 2.3, -1.0], [-1.1, 4.0, 0.7], [1.0, 4.2, -0.7], [0, 4.9, 0.3]][i]!; a.ell(`fruit-${k}`, k === "top" ? "fol-top" : `fol-${k}`, [c[0]!, c[1]! - 0.9, c[2]!], [0.05, 0.05, 0.05], a.mat("apple-fruit", 0.4, "detail")); });
  else a.ell("fruit-crown", scale === 500 ? "ld" : "fol-top", [0, 4.2, 0], [0.12, 0.12, 0.12], a.mat("apple-fruit", 0.4, "detail"));
  treeFinish(a, cfg);
  return a;
}
