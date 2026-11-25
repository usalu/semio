/** 🎺️ `used-trumpet`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-trumpet.ts <asset document> [--force] */
import { type Script, box, cap, ext, main, of, rect, rev } from "./lib.ts";

const face = { texture: true };

const script: Script = {
  id: "used-trumpet",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: the open case keeps its wall ring and its lid rim ring as hollow parts judged by their overall extents, the bottom as its declared 15 mm sheet under the bed lining as its declared 3 mm sheet in place, the lid top as its declared sheet and the handle as one 75 mm block with its outer face in place; the trumpet is its leadpipe, its lower main tube and its bell tube at their true section as slender parts through one 75 mm valve block, and the bell is the filled flare of its true profile; the two mutes are cones of four and five points; hinge, divider, cloth, bottle, crooks and the two short slides are below the feature and left out.", de: "Körper für das 75-mm-Merkmal: der offene Koffer behält Wandring und Deckelrandring als Hohlteile, nach ihren Gesamtmaßen beurteilt, den Boden als erklärte 15-mm-Platte unter dem Bettfutter als erklärter 3-mm-Platte am Ort, die Deckelplatte als erklärte Platte und den Griff als einen 75-mm-Block mit der Außenfläche am Ort; die Trompete ist ihr Mundrohr, ihr unteres Hauptrohr und ihr Schallrohr im wahren Querschnitt als schlanke Teile durch einen 75-mm-Ventilblock, und der Schallbecher ist der gefüllte Becher seines wahren Profils; die zwei Dämpfer sind Kegel aus vier und fünf Punkten; Scharnier, Trennsteg, Tuch, Flasche, Bögen und die zwei kurzen Züge liegen unter dem Merkmal und entfallen." },
        { en: "Thermal: the body solids in their regions by reference; worn brass, mouthpiece and accessories as texture faces so that every region keeps a solid, and a 75 mm breath disc at the bell mouth.", de: "Thermik: die Körper in ihren Regionen als Verweis; abgegriffenes Messing, Mundstück und Zubehör als Texturflächen, damit jede Region einen Körper behält, und eine 75-mm-Atemscheibe an der Bechermündung." },
        { en: "Structure: the members by reference, the hinge as a 75 mm joint and three mass markers.", de: "Tragwerk: die Stäbe als Verweis, das Scharnier als 75-mm-Gelenk und drei Massenmarken." },
      ],
      body: [
        ext(["walls", "case", "case-ply-tolex", 0.12], "zx", rect(0.025, 0.355, 0, 0.64), -0.1, 0.02, { holes: [rect(0.031, 0.349, 0.006, 0.634)] }),
        box(["bottom", "case", "case-ply-tolex", 0.12], [0.628, 0.015, 0.318], [0.32, -0.0925, 0.19], { sheet: true }),
        box(["bed-lining", "case", "plush-foam", 0.32], [0.628, 0.003, 0.318], [0.32, -0.0725, 0.19], { sheet: true }),
        box(["handle", "case", "handle-pvc", 0.15], [0.15, 0.075, 0.075], [0.32, -0.0175, 0.0265]),
        ext(["lid-rims", "lid", "case-ply-tolex", 0.14], "zx", rect(-0.335, -0.005, 0, 0.64), 0, 0.049, { holes: [rect(-0.329, -0.011, 0.006, 0.634)] }),
        box(["lid-top", "lid", "case-ply-tolex", 0.14], [0.64, 0.01, 0.33], [0.32, 0.05, -0.17], { sheet: true }),
        rev(["straight-mute", "straight-mute", "aluminium-spun", 0.82], [1, 0, 0], [0, 0, 0], [[0.004, 0], [0.004, 0.049], [0.135, 0.02], [0.135, 0]]),
        rev(["cup-mute", "cup-mute", "fibre-painted", 0.2], [1, 0, 0], [0, 0, 0], [[0.002, 0], [0.002, 0.053], [0.09, 0.053], [0.13, 0.02], [0.13, 0]]),
        box(["valve-block", "trumpet", "brass-lacquer", 0.72], [0.075, 0.075, 0.11], [0.224, 0.025, 0.029]),
        cap(["leadpipe", "trumpet", "brass-lacquer", 0.74], [0, 0, 0], [0.41, 0, 0.014], 0.0068, 0.0068, { slender: true }),
        cap(["main-lower", "trumpet", "brass-lacquer", 0.7], [0.135, 0, 0.046], [0.41, 0, 0.046], 0.0063, 0.0063, { slender: true }),
        cap(["bell-tail", "bell", "brass-lacquer", 0.76], [0.029, 0, 0.004], [0.345, 0, 0], 0.0075, 0.0075, { slender: true }),
        rev(["bell", "bell", "brass-lacquer", 0.78], [1, 0, 0], [0, 0, 0], [[0.3, 0], [0.3, 0.0078], [0.37, 0.0138], [0.415, 0.0236], [0.446, 0.0383], [0.4774, 0.0615], [0.4774, 0]]),
      ],
      thermal: [
        of("th-walls", "walls", { region: "case-shell" }),
        of("th-bottom", "bottom", { region: "case-shell" }),
        of("th-lid-rims", "lid-rims", { region: "case-shell" }),
        of("th-lid-top", "lid-top", { region: "case-shell" }),
        of("th-bed-lining", "bed-lining", { region: "case-lining" }),
        of("th-handle", "handle", { region: "case-hardware" }),
        of("th-straight-mute", "straight-mute", { region: "straight-mute" }),
        of("th-cup-mute", "cup-mute", { region: "cup-mute" }),
        of("th-valve-block", "valve-block", { region: "trumpet-brass" }),
        of("th-leadpipe", "leadpipe", { region: "trumpet-brass" }),
        of("th-main-lower", "main-lower", { region: "trumpet-brass" }),
        of("th-bell-tail", "bell-tail", { region: "trumpet-brass" }),
        of("th-bell", "bell", { region: "trumpet-brass" }),
        box(["th-valve-block-wear", "trumpet", "brass-worn", 0.5, "texture"], [0.072, 0.001, 0.042], [0.224, 0.063, 0.029], { ...face, region: "worn-brass" }),
        box(["th-mouthpiece", "mouthpiece", "silver-plate", 0.9, "texture"], [0.066, 0.001, 0.026], [-0.033, -0.012, 0], { ...face, region: "mouthpiece" }),
        box(["th-bottle", "oil-bottle", "pe-bottle", 0.88, "texture"], [0.09, 0.001, 0.032], [0.045, -0.0065, 0], { ...face, region: "accessories" }),
        rev(["th-breath-bell", "bell", "brass-lacquer", 0.2, "detail"], [1, 0, 0], [0, 0, 0], [[0.477, 0], [0.477, 0.0375], [0.552, 0.0375], [0.552, 0]], { region: "trumpet-brass", tags: ["emitter", "breath-bell"] }),
      ],
      structure: [
        of("st-walls", "walls", { member: "case", part: "member" }),
        of("st-bottom", "bottom", { member: "case", part: "member" }),
        of("st-lid-rims", "lid-rims", { member: "lid", part: "member" }),
        of("st-lid-top", "lid-top", { member: "lid", part: "member" }),
        of("st-valve-block", "valve-block", { member: "trumpet", part: "member" }),
        of("st-leadpipe", "leadpipe", { member: "trumpet", part: "member" }),
        of("st-main-lower", "main-lower", { member: "trumpet", part: "member" }),
        of("st-bell-tail", "bell-tail", { member: "bell", part: "member" }),
        of("st-bell", "bell", { member: "bell", part: "member" }),
        box(["st-hinge", "case", "nickel-steel", 0.7], [0.46, 0.075, 0.075], [0.32, 0.02, 0.36], { member: "lid", part: "joint" }),
        box(["st-mass-case", "case", "case-ply-tolex", 0.3, "detail"], [0.075, 0.075, 0.075], [0.32, -0.06, 0.19], { member: "case", part: "mass" }),
        box(["st-mass-lid", "lid", "case-ply-tolex", 0.3, "detail"], [0.075, 0.075, 0.075], [0.32, 0.025, -0.17], { member: "lid", part: "mass" }),
        box(["st-mass-trumpet", "trumpet", "brass-lacquer", 0.3, "detail"], [0.075, 0.075, 0.075], [0.2, 0.02, 0.03], { member: "trumpet", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
