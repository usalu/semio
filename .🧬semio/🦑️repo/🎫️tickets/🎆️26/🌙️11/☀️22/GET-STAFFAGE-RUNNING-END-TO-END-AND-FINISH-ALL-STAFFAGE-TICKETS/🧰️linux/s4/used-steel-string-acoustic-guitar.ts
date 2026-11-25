/** 🎸️ `used-steel-string-acoustic-guitar`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-steel-string-acoustic-guitar.ts <asset document> [--force] */
import { type Script, type Solid, box, cap, ext, main, of } from "./lib.ts";

const guitar = (id: string, tone: number, binding: Solid): Solid => ext([id, "guitar", "mahogany", tone], "zx", [[-0.19, 0.015], [0.19, 0.015], [0.19, 0.497], [0.0375, 0.497], [0.0375, 1.037], [-0.0375, 1.037], [-0.0375, 0.497], [-0.19, 0.497]], -0.12, -0.004, binding);

const script: Script = {
  id: "used-steel-string-acoustic-guitar",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: the closed case is two blocks that still part at the lid joint, the bottom shell and the lid thickened from 63 to 75 mm downwards with the top in place, and the handle as one 75 mm block with its outer face in place; corner caps, latches and everything inside the closed case are below the feature or hidden and left out of the body.", de: "Körper für das 75-mm-Merkmal: der geschlossene Koffer sind zwei Blöcke, die sich an der Deckelfuge weiter trennen, die Bodenschale und der von 63 auf 75 mm nach unten verdickte Deckel mit der Oberseite am Ort, und der Griff als ein 75-mm-Block mit der Außenfläche am Ort; Eckkappen, Schlösser und alles im geschlossenen Koffer liegen unter dem Merkmal oder sind verdeckt und fehlen im Körper." },
        { en: "Thermal: shells and handle by reference, the hardware as one texture strip on the front; the hidden parts as solids of their own inside the case: the lining block, the guitar as one prism of soundbox and a 75 mm neck, the top as its declared sheet, the fretboard as a shell of its layer, one slender string, the pickguard sheet and the strap as one 75 mm sheet.", de: "Thermik: Schalen und Griff als Verweis, die Beschläge als ein Texturstreifen an der Front; die verdeckten Teile als eigene Körper im Koffer: der Futterblock, die Gitarre als ein Prisma aus Korpus und 75-mm-Hals, die Decke als erklärte Platte, das Griffbrett als Schale seiner Schicht, eine schlanke Saite, die Schlagbrettplatte und der Gurt als eine 75-mm-Platte." },
        { en: "Structure: the shells and the handle by reference, the bottom also holding the four latch bones and the capo, the guitar prism holding neck, head and the four strap bones, two 75 mm hinges, the guitar rest as a joint and two mass markers.", de: "Tragwerk: die Schalen und der Griff als Verweis, der Boden fasst auch die vier Schlossknochen und den Kapodaster, das Gitarrenprisma fasst Hals, Kopf und die vier Gurtknochen, zwei 75-mm-Scharniere, die Gitarrenauflage als Gelenk und zwei Massenmarken." },
      ],
      body: [
        box(["shell-bottom", "case", "tolex-ply", 0.22], [0.446, 0.108, 1.106], [0, 0.054, 0]),
        box(["shell-lid", "lid", "tolex-ply", 0.26], [0.445, 0.075, 1.106], [0.2265, 0.0255, 0]),
        box(["handle-grip", "handle", "handle-vinyl", 0.16], [0.075, 0.075, 0.132], [0.02, -0.0115, 0]),
      ],
      thermal: [
        of("t-shell-bottom", "shell-bottom", { region: "case-bottom" }),
        of("t-shell-lid", "shell-lid", { region: "case-lid" }),
        of("t-handle-grip", "handle-grip", { region: "case-handle" }),
        box(["t-hardware", "case", "nickel-steel", 0.6, "texture"], [0.001, 0.04, 0.9], [0.2235, 0.092, 0], { texture: true, region: "case-hardware" }),
        box(["t-lining", "case", "plush", 0.42], [0.43, 0.09, 1.09], [0, 0.063, 0], { region: "case-lining" }),
        guitar("t-guitar", 0.4, { region: "guitar-body" }),
        box(["t-top", "guitar", "spruce-lacquer", 0.82], [0.508, 0.004, 0.398], [0.254, -0.002, 0], { region: "guitar-top", layer: "top", sheet: true }),
        box(["t-fretboard-layer", "neck", "rosewood", 0.16], [0.446, 0.006, 0.056], [0.135, 0.003, 0], { region: "guitar-rosewood", layer: "fretboard" }),
        cap(["t-string", "guitar", "string-metal", 0.7, "texture"], [0.217, 0.0115, 0], [0.866, 0.0115, 0], 0.0025, 0.0025, { region: "guitar-metal", slender: true }),
        box(["t-pickguard", "guitar", "celluloid", 0.2, "detail"], [0.15, 0.001, 0.092], [0.367, 0.0005, -0.086], { region: "pickguard", sheet: true }),
        box(["t-strap", "strap-cross", "leather", 0.3, "detail"], [0.482, 0.002, 0.075], [0.241, 0.001, 0], { region: "strap", sheet: true }),
      ],
      structure: [
        of("s-case", "shell-bottom", { member: "case", part: "member", members: ["latch-1", "latch-2", "latch-3", "latch-4", "capo"] }),
        of("s-lid", "shell-lid", { member: "lid", part: "member" }),
        of("s-handle", "handle-grip", { member: "handle", part: "member" }),
        guitar("s-guitar", 0.5, { member: "guitar", part: "member", members: ["neck", "head", "strap-tab", "strap-rear", "strap-cross", "strap-front"] }),
        box(["s-hinges", "lid", "nickel-steel", 0.5], [0.075, 0.075, 0.075], [0.0375, 0, -0.3], { repeat: { count: 2, translation: [0, 0, 0.6] }, member: "lid", part: "joint" }),
        box(["s-guitar-rest", "case", "plush", 0.5], [0.16, 0.075, 0.46], [0, 0.0375, -0.275], { member: "guitar", part: "joint" }),
        box(["s-mass-case", "case", "nickel-steel", 0.5], [0.075, 0.075, 0.075], [0, 0.07, 0], { member: "case", part: "mass" }),
        box(["s-mass-guitar", "guitar", "nickel-steel", 0.5], [0.075, 0.075, 0.075], [0.3, -0.06, 0], { member: "guitar", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
