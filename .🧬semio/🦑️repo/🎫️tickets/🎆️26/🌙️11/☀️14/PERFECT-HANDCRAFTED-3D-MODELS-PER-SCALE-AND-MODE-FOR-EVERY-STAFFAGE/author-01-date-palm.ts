import { Author } from "./author-01-kit.ts";
import { treeFinish, draftBase, type TreeCfg } from "./author-01-tree.ts";

const cfg: TreeCfg = {
  slug: "date-palm",
  woody: () => false, foliage: () => false,
  pick: {
    "stem-bark": (s) => s.material === "palm-stem-bark",
    fruit: (s) => s.material === "date-fruit",
    fronds: (s) => s.material === "palm-frond",
  },
  memberR: (b, sc) => [Math.max(0.06, 0.05 * Math.sqrt(sc / 50)), 0.1],
  emitterD: 2.0,
  notes: {},
  kind: "the stem, crown shaft, root bulb and fronds are exact-profile extrusions and lofts proposed by the drafter from the scale's own shapes (not yet hand-remodelled: the fronds are flat polygon plates, a known crudeness)",
  kindDe: "Stamm, Kronenschaft, Wurzelknolle und Wedel sind Extrusionen und Lofts mit exakter Kontur, vom Zeichner aus den eigenen Formen des Maßstabs vorgeschlagen (noch nicht von Hand überarbeitet: die Wedel sind flache Polygonplatten, eine bekannte Grobheit)",
};
export function build(scale: 50 | 100 | 200 | 500): Author {
  const a = new Author(cfg.slug, scale);
  a.into("body");
  draftBase(a);
  treeFinish(a, cfg);
  return a;
}
