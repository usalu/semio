/** 🎻️ `used-violin`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-violin.ts <asset document> [--force] */
import { type Script, box, cap, ext, main, of, rect } from "./lib.ts";

const cube = [0.075, 0.075, 0.075] as const;
const outline = [[-0.064, 0.013], [-0.1015, 0.062], [-0.0985, 0.115], [-0.056, 0.18], [-0.082, 0.25], [-0.079, 0.315], [0, 0.356], [0.079, 0.315], [0.082, 0.25], [0.056, 0.18], [0.0985, 0.115], [0.1015, 0.062], [0.064, 0.013], [0, 0]] as const;
const face = { texture: true };

const script: Script = {
  id: "used-violin",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: the open case keeps its shell ring and its lid ring as hollow parts judged by their overall extents with bottom, bed and lid top as declared sheets, so the violin still shows in it; the violin is one prism of its outline reduced from 20 to 14 points and thickened from 61 to 75 mm, its neck with fingerboard and scroll one 75 mm bar; the bow is its slender stick in two sections, the strap its webbing sheet, the front hardware one texture strip; compartment, lid lining, hair and hooks are left out.", de: "Körper für das 75-mm-Merkmal: der offene Koffer behält Schalenring und Deckelring als Hohlteile, nach ihren Gesamtmaßen beurteilt, mit Boden, Bett und Deckelplatte als erklärten Platten, damit die Geige darin sichtbar bleibt; die Geige ist ein Prisma ihres von 20 auf 14 Punkte reduzierten Umrisses, von 61 auf 75 mm verdickt, ihr Hals mit Griffbrett und Schnecke ein 75-mm-Stab; der Bogen ist seine schlanke Stange in zwei Schnitten, der Gurt seine Gurtbandplatte, die Beschläge vorn ein Texturstreifen; Fach, Deckelfutter, Haar und Haken entfallen." },
        { en: "Thermal: the body solids in their regions by reference; top, fingerboard, chinrest, bridge and the worn shoulder as texture faces on the prism and the strings as one slender run, so every region keeps a solid.", de: "Thermik: die Körper in ihren Regionen als Verweis; Decke, Griffbrett, Kinnhalter, Steg und die abgegriffene Schulter als Texturflächen auf dem Prisma und die Saiten als ein schlanker Strang, sodass jede Region einen Körper behält." },
        { en: "Structure: the members by reference, the bed as the joint of the violin, hooks, rings and bow clips as 75 mm joints and three mass markers.", de: "Tragwerk: die Stäbe als Verweis, das Bett als Gelenk der Geige, Haken, Ringe und Bogenhalter als 75-mm-Gelenke und drei Massenmarken." },
      ],
      body: [
        ext(["shell-walls", "case", "covered-plywood", 0.18], "zx", rect(-0.1225, 0.1225, 0, 0.8), -0.074, 0, { holes: [rect(-0.1165, 0.1165, 0.006, 0.794)] }),
        box(["shell-bottom", "case", "covered-plywood", 0.16], [0.8, 0.006, 0.245], [0.4, -0.077, 0], { sheet: true }),
        box(["lining-bed", "case", "velvet-lining", 0.32], [0.788, 0.008, 0.233], [0.4, -0.07, 0], { sheet: true }),
        box(["hardware-front", "case", "nickel-steel", 0.7, "texture"], [0.765, 0.028, 0.008], [0.4005, -0.018, -0.127], face),
        ext(["lid-walls", "lid", "covered-plywood", 0.2], "xy", rect(0, 0.8, 0, 0.245), 0, 0.039, { holes: [rect(0.006, 0.794, 0.006, 0.239)] }),
        box(["lid-top", "lid", "covered-plywood", 0.16], [0.8, 0.245, 0.006], [0.4, 0.1225, 0.042], { sheet: true }),
        ext(["violin-body", "violin", "maple-flamed", 0.45], "zx", outline, -0.0455, 0.0295),
        box(["violin-neck", "violin", "maple-bare-worn", 0.4], [0.377, 0.075, 0.075], [0.4045, -0.008, 0]),
        { id: "bow-stick", kind: "loft", bone: "bow", material: "brazilwood", role: "body", tone: 0.3, sections: [{ at: [0, 0.0238, 0], rx: 0.0043, ry: 0.0043, exponent: 2, rotation: [0.5, 0.5, 0.5, 0.5] }, { at: [0.725, 0.018, 0], rx: 0.00275, ry: 0.00275, exponent: 2, rotation: [0.5, 0.5, 0.5, 0.5] }], capStart: true, capEnd: true, slender: true },
        ext(["strap-webbing", "strap", "pp-webbing", 0.15], "xy", [[0.01556, -0.010904], [0.37, 0.494892], [0.72444, -0.010904], [0.75556, 0.010904], [0.37, 0.561108], [-0.01556, 0.010904]], -0.0015, 0.0015, { sheet: true }),
      ],
      thermal: [
        of("th-shell-walls", "shell-walls", { region: "case-cover" }),
        of("th-shell-bottom", "shell-bottom", { region: "case-cover" }),
        of("th-lid-walls", "lid-walls", { region: "case-cover" }),
        of("th-lid-top", "lid-top", { region: "case-cover" }),
        of("th-lining-bed", "lining-bed", { region: "case-lining" }),
        of("th-hardware-front", "hardware-front", { region: "case-hardware" }),
        of("th-violin-body", "violin-body", { region: "violin-back" }),
        of("th-violin-neck", "violin-neck", { region: "violin-neck" }),
        of("th-bow-stick", "bow-stick", { region: "bow" }),
        of("th-strap-webbing", "strap-webbing", { region: "strap" }),
        box(["th-top-face", "violin", "spruce-top-aged", 0.66, "texture"], [0.284, 0.001, 0.162], [0.178, 0.03, 0], { ...face, region: "violin-top" }),
        box(["th-fingerboard-face", "violin", "ebony", 0.12, "texture"], [0.27, 0.001, 0.03], [0.351, 0.0315, 0], { ...face, region: "fingerboard-fittings" }),
        box(["th-chinrest-face", "violin", "ebony", 0.15, "texture"], [0.058, 0.001, 0.07], [0.029, 0.0315, 0.035], { ...face, region: "chinrest" }),
        box(["th-bridge-face", "violin", "maple-bridge", 0.78, "texture"], [0.002, 0.03, 0.042], [0.162, 0.0465, 0], { ...face, region: "bridge" }),
        box(["th-worn-shoulder-back", "violin", "maple-bare-worn", 0.7, "texture"], [0.065, 0.001, 0.04], [0.0625, -0.046, 0.065], { ...face, region: "worn-varnish" }),
        cap(["th-strings", "violin", "string-steel", 0.8, "detail"], [0.161, 0.0625, 0], [0.487, 0.0625, 0], 0.001, 0.001, { slender: true, region: "strings" }),
      ],
      structure: [
        of("st-shell-walls", "shell-walls", { member: "case", part: "member" }),
        of("st-shell-bottom", "shell-bottom", { member: "case", part: "member" }),
        of("st-lid-walls", "lid-walls", { member: "lid", part: "member" }),
        of("st-lid-top", "lid-top", { member: "lid", part: "member" }),
        of("st-violin-body", "violin-body", { member: "violin", part: "member" }),
        of("st-violin-neck", "violin-neck", { member: "violin", part: "member" }),
        of("st-bow-stick", "bow-stick", { member: "bow", part: "member" }),
        of("st-strap-webbing", "strap-webbing", { member: "strap", part: "member" }),
        of("st-bed", "lining-bed", { member: "violin", part: "joint" }),
        box(["st-hooks", "strap", "nickel-steel", 0.6], cube, [0.0245, 0.004, 0], { repeat: { count: 2, translation: [0.6955, 0, 0] }, member: "strap", part: "joint" }),
        box(["st-rings", "case", "nickel-steel", 0.6], cube, [0.0545, -0.0375, -0.127], { repeat: { count: 2, translation: [0.691, 0, 0] }, member: "case", part: "joint" }),
        box(["st-bow-clips", "lid", "velvet-lining", 0.6], cube, [0.1475, 0.2075, 0.0075], { repeat: { count: 2, translation: [0.535, 0, 0] }, member: "bow", part: "joint" }),
        box(["mass-case", "case", "covered-plywood", 0.3, "detail"], [0.075, 0.075, 0.075], [0.4, -0.04, 0], { member: "case", part: "mass" }),
        box(["mass-lid", "lid", "covered-plywood", 0.3, "detail"], [0.075, 0.075, 0.075], [0.4, 0.1225, 0.02], { member: "lid", part: "mass" }),
        box(["mass-violin", "violin", "maple-flamed", 0.3, "detail"], [0.075, 0.075, 0.075], [0.18, -0.008, 0], { member: "violin", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
