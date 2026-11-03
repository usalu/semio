import { build } from "../humans/bands.ts";
const t = (s: string) => Object.fromEntries(["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"].map((k) => [k, s]));
const den = Number(process.argv[2]); const mf = Number(process.argv[3]); const pen = Number(process.argv[4]); const dropx = process.argv[5] ?? "";
build({ slug: "kneeling-gardener", den: den as any, drop: new RegExp(`^(eye|hat-band|clog-strap-l|patch-l|knee-patch-[lr]|ear-[lr]${dropx ? "|" + dropx : ""})$`), minFeatureM: mf, epsM: den === 200 ? 0.005 : 0.012, pens: { body: pen, detail: null, texture: null, glazing: null, ground: den === 200 ? 0.25 : 0.18 }, range: den === 200 ? [150, 350] : [350, 900], en: t("x"), de: t("x") }, false);
