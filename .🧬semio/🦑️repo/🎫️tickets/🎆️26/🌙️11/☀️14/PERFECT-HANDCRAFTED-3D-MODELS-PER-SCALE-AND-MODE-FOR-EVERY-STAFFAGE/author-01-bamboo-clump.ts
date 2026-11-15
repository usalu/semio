import { Author } from "./author-01-kit.ts";
import { treeFinish, draftBase, type TreeCfg } from "./author-01-tree.ts";

const cfg: TreeCfg = {
  slug: "bamboo-clump",
  woody: () => false, foliage: () => false,
  pick: {
    culms: (s) => s.material === "bamboo-culm",
    "leaves-sun": (s) => s.material === "bamboo-leaf" && /sun/.test(s.id),
    "leaves-shade": (s) => s.material === "bamboo-leaf" && !/sun/.test(s.id),
  },
  memberR: (b, sc) => [Math.max(0.02, 0.015 * Math.sqrt(sc / 50)), 0.02],
  emitterD: 0.8,
  notes: {},
  kind: "culms and leaf masses are exact-profile extrusions and capsules proposed by the drafter from the scale's own shapes (not yet hand-remodelled)",
  kindDe: "Halme und Laubmassen sind Extrusionen und Kapseln mit exakter Kontur, vom Zeichner aus den eigenen Formen des Maßstabs vorgeschlagen (noch nicht von Hand überarbeitet)",
};
export function build(scale: 50 | 100 | 200 | 500): Author {
  const a = new Author(cfg.slug, scale);
  a.into("body");
  draftBase(a);
  treeFinish(a, cfg);
  return a;
}
