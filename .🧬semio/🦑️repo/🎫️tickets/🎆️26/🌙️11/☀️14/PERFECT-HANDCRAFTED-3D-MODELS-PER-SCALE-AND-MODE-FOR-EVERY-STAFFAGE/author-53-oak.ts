import { buildAll, treeDecisions, type TreeCfg } from "./author-53-tree.ts";
import { openCtx, saveLists } from "./author-53-lib.ts";

export const cfg: TreeCfg = {
  slug: "english-oak",
  stems: ["root-plate", "trunk", "trunk-upper", "leader", "leader-top"],
  stemFront: ["root-plate", "trunk", "trunk-upper", "leader", "leader-top"],
  woody: ["limb-w-low", "limb-e-low", "limb-w-mid", "limb-e-mid", "br-w-low", "br-e-low", "br-w-mid", "br-e-mid", "br-leader-w", "br-leader-e", "br-low-w-up", "br-low-e-up"],
  pairs: [["fol-w-low", "fol-e-low"], ["fol-w-mid", "fol-e-mid"], ["fol-leader-w", "fol-leader-e"]],
  singles: ["fol-top", "fol-core"],
  crown500: "leader",
  bark: { stem: "trunk-bark", limb: "limb-bark", stemMat: "oak-bark", limbMat: "oak-bark" },
  leaf: { sunRegion: "leaf-sun", shadeRegion: "leaf-shade", shade: (b) => b === "fol-core" },
  tone: (b) => (/^fol-core/.test(b) ? 0.4 : /^fol-leader/.test(b) ? 0.58 : /^fol-top/.test(b) ? 0.62 : /^fol/.test(b) ? 0.54 : /^(br-|limb)/.test(b) ? 0.28 : 0.27),
  mat: (b) => (/^fol-core/.test(b) ? "oak-leaf-shade" : /^fol/.test(b) ? "oak-leaf-sun" : "oak-bark"),
  override: { 500: { stems: ["root-plate", "trunk"], stemFront: ["root-plate", "trunk"], woody: ["limb-w-low", "limb-e-low"], pairs: [], singles: ["leader"] } },
  scale: { 50: { tolLobe: 0.03, tolLimb: 0.006, stations: 24, bias: 0.03 }, 100: { tolLobe: 0.04, tolLimb: 0.012, stations: 16, bias: 0.03 }, 200: { tolLobe: 0.06, tolLimb: 0.024, stations: 10, bias: 0.03 }, 500: { tolLobe: 0.15, tolLimb: 0.075, stations: 8, bias: 0 } },
};

if (import.meta.main) {
  if (process.env.BIAS) for (const k of Object.keys(cfg.scale)) cfg.scale[Number(k)]!.bias = Number(process.env.BIAS);
  if (process.env.TOL) for (const k of Object.keys(cfg.scale)) cfg.scale[Number(k)]!.tolLobe = Number(process.env.TOL);
  const scale = Number(process.argv[2]);
  const c = openCtx("vegetation/english-oak", scale);
  const exp = Number(process.argv[3] ?? 2.6);
  const r = buildAll(c, cfg, { exp, cross: process.argv[4] === "x", joints: scale === 50, core: 0.75, category: "vegetation", split: false });
  console.log(`body ${r.body.length} thermal ${r.thermal.length} structure ${r.structure.length}`, r.built.notes);
  const skip = (process.env.SKIP ?? "").split(",").filter(Boolean);
  const drop = (l: any[]) => l.filter((x) => !skip.some((k) => x.id === k || x.id === `th-${k}` || x.of === k));
  const words = {
    name: { en: "English oak", de: "Stieleiche" },
    coarse: {
      50: { en: "The seven drawn foliage groups are single lobes, the plan drawing's four crown fills add the lobes in the quadrants and on the north and south sides that elevation and front hide behind the limb-carrying lobes, and the inner shade crown is the central lobe; the root plate is a flared loft read from the flare outline.", de: "Die sieben gezeichneten Laubgruppen sind je ein Lappen, die vier Kronenfüllungen des Grundrisses ergänzen die Lappen in den Quadranten und auf der Nord- und Südseite, die Aufriss und Front hinter den astttragenden Lappen verbergen, und die Schattenkrone innen ist der mittlere Lappen; der Wurzelteller ist ein ausgeweiteter Loft nach der Anlaufkontur." },
      100: { en: "The 1:100 drawings merge the small branches into the limbs and draw only four branch bones, so the limbs are longer and slimmer than at 1:50, trunk and root flare are one bole loft, the lobes follow the coarser outlines with a larger section tolerance, and the elevation and front limb widths are limited to a ratio of 1.6 because the two views draw different thicknesses.", de: "Die Zeichnungen 1:100 fassen die kleinen Äste mit den Hauptästen zusammen und zeichnen nur vier Astknochen, die Hauptäste sind daher länger und schlanker als bei 1:50, Stamm und Wurzelanlauf sind ein Schaft-Loft, die Lappen folgen den gröberen Konturen mit größerer Schnitttoleranz, und die Aufriss- und Frontbreiten der Äste sind auf ein Verhältnis von 1,6 begrenzt, weil beide Ansichten verschiedene Dicken zeichnen." },
      200: { en: "At 1:200 the branch bones are no longer drawn at all; the limbs stay lofts, the crown lobes are fewer stations per lobe, and the plan fills still stand for the crown quadrants.", de: "Bei 1:200 werden die Astknochen gar nicht mehr gezeichnet; die Hauptäste bleiben Lofts, die Kronenlappen haben weniger Stationen, und die Grundrissfüllungen stehen weiter für die Kronenquadranten." },
      500: { en: "At 1:500 the 2D draws one crown outline per view, so the whole crown is one hull lobe (x span from the elevation crown, z span from the front crown at every height) on a bole loft and two limb lofts; the leader and branches are hidden inside the crown and live only in the structure list.", de: "Bei 1:500 zeichnet die 2D je Ansicht nur eine Kronenkontur, die ganze Krone ist daher ein Hüllenlappen (x-Spanne der Aufrisskrone, z-Spanne der Frontkrone in jeder Höhe) auf einem Schaft-Loft und zwei Ast-Lofts; Stammverlängerung und Äste liegen verdeckt in der Krone und nur in der Tragwerksliste." },
    } as Record<number, { en: string; de: string }>,
    hard: { en: "", de: "" },
  };
  const decisions = treeDecisions(c, r, words);
  saveLists(c, { body: drop(r.body), thermal: drop(r.thermal), structure: r.structure, decisions });
}
