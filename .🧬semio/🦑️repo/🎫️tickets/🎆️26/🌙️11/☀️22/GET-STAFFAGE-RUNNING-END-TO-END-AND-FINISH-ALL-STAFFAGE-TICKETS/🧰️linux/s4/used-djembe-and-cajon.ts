/** 🪘️ `used-djembe-and-cajon`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-djembe-and-cajon.ts <asset document> [--force] */
import { type Script, box, ell, main, of, rev } from "./lib.ts";

const script: Script = {
  id: "used-djembe-and-cajon",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: the cajon is one closed box (carcass tube, back and port merged) with the tapa as its declared 3 mm sheet on the front; the djembe is its hollow goblet shell reduced to twelve profile points (a hollow part judged by its overall extents; its 140 mm waist still prints and its open foot keeps the inner surface), under its skin as the declared sheet and the crown ring at its true section as the one slender hoop; bottom ring, lacing and the rolled collar are below the feature and left out.", de: "Körper für das 75-mm-Merkmal: das Cajon ist ein geschlossener Kasten (Korpusrohr, Rückwand und Schallloch verschmolzen) mit der Schlagplatte als erklärter 3-mm-Platte an der Front; die Djembe ist ihre hohle Kelchschale, auf zwölf Profilpunkte reduziert (ein Hohlteil, nach seinen Gesamtmaßen beurteilt; ihre 140-mm-Taille druckt noch und ihr offener Fuß behält die Innenfläche), unter ihrem Fell als erklärter Platte und mit dem Kronenring im wahren Querschnitt als einzigem schlanken Reif; unterer Ring, Schnürung und der gerollte Kragen liegen unter dem Merkmal und entfallen." },
        { en: "Thermal: one surface per region by reference.", de: "Thermik: eine Oberfläche je Region als Verweis." },
        { en: "Structure: the members by reference, the two joints as 75 mm balls and the two mass markers.", de: "Tragwerk: die Stäbe als Verweis, die zwei Gelenke als 75-mm-Kugeln und die zwei Massenmarken." },
      ],
      body: [
        box(["cajon", "cajon", "birch-ply", 0.6], [0.48, 0.297, 0.3], [0.24, 0.0015, 0]),
        box(["tapa", "cajon", "birch-tapa", 0.74], [0.48, 0.003, 0.3], [0.24, -0.1485, 0], { sheet: true }),
        rev(["goblet", "djembe", "lenke", 0.38], [1, 0, 0], [0, 0, 0], [[0, 0.113], [0.2, 0.081], [0.322, 0.07], [0.48, 0.1445], [0.565, 0.1586], [0.598, 0.147], [0.598, 0.14], [0.5, 0.13], [0.38, 0.069], [0.3, 0.048], [0.15, 0.065], [0, 0.094], [0, 0.113]]),
        rev(["head", "djembe-head", "goatskin", 0.88], [1, 0, 0], [0, 0, 0], [[0, 0], [0, 0.141], [0.0012, 0.141], [0.0012, 0]], { sheet: true }),
        rev(["crown-ring", "djembe", "ring-rope", 0.25], [1, 0, 0], [0, 0, 0], [[0.562, 0.1653], [0.560243, 0.169543], [0.556, 0.1713], [0.551757, 0.169543], [0.55, 0.1653], [0.551757, 0.161057], [0.556, 0.1593], [0.560243, 0.161057], [0.562, 0.1653]], { slender: true }),
      ],
      thermal: [
        of("th-cajon", "cajon", { region: "cajon-carcass" }),
        of("th-tapa", "tapa", { region: "cajon-tapa" }),
        of("th-goblet", "goblet", { region: "djembe-shell" }),
        of("th-head", "head", { region: "djembe-head" }),
        of("th-crown-ring", "crown-ring", { region: "djembe-rope" }),
      ],
      structure: [
        of("st-cajon", "cajon", { member: "cajon", part: "member" }),
        of("st-tapa", "tapa", { member: "cajon", part: "member" }),
        of("st-goblet", "goblet", { member: "djembe", part: "member" }),
        of("st-head", "head", { member: "djembe-head", part: "member" }),
        of("st-crown-ring", "crown-ring", { member: "djembe", part: "member" }),
        ell(["joint-tilt-pivot", "djembe", "lenke", 0.25, "detail"], [0.0375, 0.0375, 0.0375], [0.0375, 0.098, 0], { member: "djembe", part: "joint" }),
        ell(["joint-head", "djembe-head", "goatskin", 0.25, "detail"], [0.0375, 0.0375, 0.0375], [-0.0375, 0, 0], { member: "djembe-head", part: "joint" }),
        ell(["mass-cajon", "cajon", "birch-ply", 0.2, "detail"], [0.06, 0.06, 0.06], [0.24, 0.01, 0], { member: "cajon", part: "mass" }),
        ell(["mass-djembe", "djembe", "lenke", 0.2, "detail"], [0.06, 0.06, 0.06], [0.333, 0, 0], { member: "djembe", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
