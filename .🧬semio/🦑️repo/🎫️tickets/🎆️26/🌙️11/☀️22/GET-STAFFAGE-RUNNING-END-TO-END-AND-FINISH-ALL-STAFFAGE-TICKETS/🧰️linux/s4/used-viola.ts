/** 🎻️ `used-viola`: the handcrafted volume of 1:500, simplified by hand from its 1:200 volume. usage: used-viola.ts <asset document> [--force] */
import { type Script, box, cap, ext, main, of } from "./lib.ts";

const cube = [0.075, 0.075, 0.075] as const;
const base = [[-0.0646, 0.7426], [-0.0432, 0.7836], [0, 0.8], [0.0432, 0.7836], [0.0646, 0.7426], [0.1341, 0.1508], [0.1218, 0.0768], [0.072, 0.0208], [0, 0], [-0.072, 0.0208], [-0.1218, 0.0768], [-0.1341, 0.1508]] as const;
const baseInner = [[-0.0546, 0.7414], [-0.0366, 0.7761], [0, 0.79], [0.0366, 0.7761], [0.0546, 0.7414], [0.1241, 0.1496], [0.1128, 0.0811], [0.0667, 0.0293], [0, 0.01], [-0.0667, 0.0293], [-0.1128, 0.0811], [-0.1241, 0.1496]] as const;
const bed = [[-0.0541, 0.7414], [-0.0362, 0.7757], [0, 0.7895], [0.0362, 0.7757], [0.0541, 0.7414], [0.1237, 0.1495], [0.1123, 0.0813], [0.0664, 0.0297], [0, 0.0105], [-0.0664, 0.0297], [-0.1123, 0.0813], [-0.1237, 0.1495]] as const;
const lid = [[0.3911, 0.1282], [0.4293, 0.1022], [0.4406, 0.0574], [0.4193, 0.0164], [0.3761, 0], [-0.2198, 0], [-0.2918, 0.0208], [-0.3417, 0.0768], [-0.3539, 0.1508], [-0.3249, 0.2198], [-0.2634, 0.2628], [-0.1886, 0.2663]] as const;
const lidInner = [[0.3888, 0.1185], [0.4211, 0.0965], [0.4307, 0.0586], [0.4126, 0.0239], [0.3761, 0.01], [-0.2198, 0.01], [-0.2865, 0.0293], [-0.3326, 0.0811], [-0.344, 0.1496], [-0.3171, 0.2136], [-0.2602, 0.2533], [-0.189, 0.2562]] as const;
const viola = [[0, 0.0085], [-0.0767, 0.0241], [-0.1077, 0.0619], [-0.115, 0.1109], [-0.1038, 0.1568], [-0.0741, 0.1844], [-0.0625, 0.2209], [-0.0714, 0.2564], [-0.0905, 0.2963], [-0.0847, 0.373], [-0.056, 0.4017], [0, 0.4095], [0.056, 0.4017], [0.0847, 0.373], [0.0905, 0.2963], [0.0714, 0.2564], [0.0625, 0.2209], [0.0741, 0.1844], [0.1038, 0.1568], [0.115, 0.1109], [0.1077, 0.0619], [0.0767, 0.0241]] as const;
const turn = [0.5, 0.5, 0.5, 0.5];

const script: Script = {
  id: "used-viola",
  volumes: {
    "500": {
      status: "authored",
      denominator: 500,
      decisions: [
        { en: "Body for the 75 mm feature: the open case keeps floor, wall ring, bed, lid ring and lid skin with their outlines reduced from 22 to 12 points (the rings are hollow parts judged by their overall extents, floor, bed and skin declared sheets), so the viola still shows in it; the viola is one 76 mm prism of its outline in 22 instead of 44 points (back, ribs and top merged), the neck with scroll and fingerboard one 75 mm bar, the bow its slender stick in two sections; the bow holders are below the feature and left out.", de: "Körper für das 75-mm-Merkmal: der offene Koffer behält Boden, Wandring, Bett, Deckelring und Deckelhaut mit von 22 auf 12 Punkte reduzierten Umrissen (die Ringe sind Hohlteile, nach ihren Gesamtmaßen beurteilt, Boden, Bett und Haut erklärte Platten), damit die Bratsche darin sichtbar bleibt; die Bratsche ist ein 76-mm-Prisma ihres Umrisses in 22 statt 44 Punkten (Boden, Zargen und Decke verschmolzen), der Hals mit Schnecke und Griffbrett ein 75-mm-Stab, der Bogen seine schlanke Stange in zwei Schnitten; die Bogenhalter liegen unter dem Merkmal und entfallen." },
        { en: "Thermal: the body solids in their regions by reference, the top as a shell of its layer on the prism, the fingerboard as a shell of its layer on the bar and one slender string.", de: "Thermik: die Körper in ihren Regionen als Verweis, die Decke als Schale ihrer Schicht auf dem Prisma, das Griffbrett als Schale seiner Schicht auf dem Stab und eine schlanke Saite." },
        { en: "Structure: the members by reference, the hinge and the two bow holders as 75 mm joints and four mass markers.", de: "Tragwerk: die Stäbe als Verweis, das Scharnier und die zwei Bogenhalter als 75-mm-Gelenke und vier Massenmarken." },
      ],
      body: [
        ext(["base-floor", "case", "case-fabric", 0.3], "zx", base, 0, 0.012, { taperTo: [0.97, 1], sheet: true }),
        ext(["base-wall", "case", "case-fabric", 0.3], "zx", base, 0.0115, 0.092, { holes: [baseInner] }),
        ext(["bed", "case", "velour", 0.85], "zx", bed, 0.0115, 0.0195, { sheet: true }),
        ext(["lid-wall", "lid", "case-fabric", 0.3], "zx", lid, 0, 0.0485, { holes: [lidInner] }),
        ext(["lid-skin", "lid", "case-fabric", 0.32], "zx", lid, 0.048, 0.058, { taperTo: [1, 0.97], sheet: true }),
        ext(["viola-box", "viola", "maple", 0.42], "zx", viola, -0.035, 0.041),
        box(["neck-bar", "viola", "maple", 0.4], [0.26, 0.075, 0.075], [0.542, 0.003, 0]),
        { id: "bow-stick", kind: "loft", bone: "bow-stroke", material: "pernambuco", role: "body", tone: 0.42, sections: [{ at: [-0.065, 0.0183, 0], rx: 0.0072, ry: 0.0042, exponent: 2, rotation: turn }, { at: [0.66, 0.0132, 0], rx: 0.00562, ry: 0.00262, exponent: 2, rotation: turn }], capStart: true, capEnd: true, slender: true },
      ],
      thermal: [
        of("th-base-floor", "base-floor", { region: "case-cover", sectionFaces: ["+v"] }),
        of("th-base-wall", "base-wall", { region: "case-cover" }),
        of("th-bed", "bed", { region: "case-lining", sectionFaces: ["-v"] }),
        of("th-lid-wall", "lid-wall", { region: "case-cover" }),
        of("th-lid-skin", "lid-skin", { region: "case-cover", sectionFaces: ["-v"] }),
        of("th-viola-box", "viola-box", { region: "viola-body" }),
        of("th-neck-bar", "neck-bar", { region: "viola-body" }),
        of("th-bow-stick", "bow-stick", { region: "bow" }),
        ext(["th-top-plate", "viola", "spruce", 0.66], "zx", viola, 0.041, 0.044, { region: "viola-top", layer: "spruce-plate" }),
        box(["th-fingerboard", "viola", "ebony", 0.1], [0.295, 0.01, 0.044], [0.4195, 0.0455, 0], { region: "fittings", layer: "ebony-wall" }),
        cap(["th-string", "viola", "string-steel", 0.6], [0.197, 0.052, 0], [0.563, 0.052, 0], 0.001, 0.001, { region: "strings", slender: true }),
      ],
      structure: [
        of("st-base-floor", "base-floor", { member: "case", part: "member" }),
        of("st-base-wall", "base-wall", { member: "case", part: "member" }),
        of("st-bed", "bed", { member: "case", part: "member" }),
        of("st-lid-wall", "lid-wall", { member: "lid", part: "member" }),
        of("st-lid-skin", "lid-skin", { member: "lid", part: "member" }),
        of("st-viola-box", "viola-box", { member: "viola", part: "member" }),
        of("st-neck-bar", "neck-bar", { member: "viola", part: "member" }),
        of("st-bow-stick", "bow-stick", { member: "bow-stroke", part: "member" }),
        box(["st-hinge", "lid", "case-fabric", 0.25], [0.075, 0.075, 0.592], [0.0375, 0.0245, 0], { member: "lid", part: "joint" }),
        box(["st-bow-holder-frog", "lid", "ebony", 0.2], cube, [0.178, 0.0245, -0.21], { member: "lid", part: "joint" }),
        box(["st-bow-holder-tip", "lid", "ebony", 0.2], cube, [0.078, 0.0245, 0.351], { member: "lid", part: "joint" }),
        box(["st-mass-case", "case", "case-shell", 0.6, "detail"], [0.075, 0.075, 0.075], [0.36, 0.04, 0], { member: "case", part: "mass" }),
        box(["st-mass-lid", "lid", "case-shell", 0.6, "detail"], [0.075, 0.075, 0.075], [0.115, 0.0245, 0], { member: "lid", part: "mass" }),
        box(["st-mass-bow", "bow-stroke", "pernambuco", 0.6, "detail"], [0.075, 0.075, 0.075], [0.26, 0.012, 0], { member: "bow-stroke", part: "mass" }),
        box(["st-mass-viola", "viola", "maple", 0.6, "detail"], [0.075, 0.075, 0.075], [0.2, 0.003, 0], { member: "viola", part: "mass" }),
      ],
    },
  },
};
export default script;
if (import.meta.main) main(script);
