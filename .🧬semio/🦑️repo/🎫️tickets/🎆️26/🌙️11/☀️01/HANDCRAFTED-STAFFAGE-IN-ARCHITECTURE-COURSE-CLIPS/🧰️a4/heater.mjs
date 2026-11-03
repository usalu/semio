import { t, mat, layer, region, path, rect, ell, line, rep, unavailable, regionBindings, operatingKeyDef, emitter } from "./lib.mjs";

const abs = mat("abs-cream", t("Cream ABS housing", "Cremefarbenes ABS-Gehäuse"), 0.9, 0.5, 1050, 0.17, 1400, "authored-estimate", "iso-10456-2007");
const pc = mat("pc-heat", t("Heat-stabilised polycarbonate grille", "Hitzestabilisiertes Polycarbonat-Gitter"), 0.9, 0.85, 1200, 0.2, 1200, "authored-estimate", "iso-10456-2007");
const mesh = mat("steel-mesh", t("Black-painted steel grille", "Schwarz lackiertes Stahlgitter"), 0.9, 0.9, 7850, 50, 450, "authored-estimate", "incropera-2018");
const knobM = mat("pp-dial", t("Dark polypropylene", "Dunkles Polypropylen"), 0.9, 0.9, 900, 0.22, 1900, "authored-estimate", "iso-10456-2007");
const rubber = mat("rubber-foot", t("Rubber foot", "Gummifuß"), 0.9, 0.9, 1100, 0.2, 2000, "authored-estimate", "iso-10456-2007");
const materials = [abs, pc, mesh, knobM, rubber];

const ids = ["casing", "top-cover", "grille", "intake", "control", "feet"];

const housing = "M -0.09 0.032 C -0.09 0.022 -0.085 0.016 -0.075 0.016 L 0.08 0.016 C 0.09 0.016 0.095 0.022 0.095 0.032 L 0.095 0.25 C 0.095 0.265 0.088 0.272 0.075 0.272 L -0.07 0.272 C -0.083 0.272 -0.09 0.265 -0.09 0.25 Z";
const handle = "M -0.045 0.268 C -0.045 0.3 -0.04 0.31 -0.02 0.31 L 0.03 0.31 C 0.05 0.31 0.055 0.3 0.055 0.268 Z";
const hood = "M -0.09 0.25 L 0.095 0.25 C 0.095 0.265 0.088 0.272 0.075 0.272 L -0.07 0.272 C -0.083 0.272 -0.09 0.265 -0.09 0.25 Z";
const block200 = "M -0.09 0 L 0.095 0 L 0.095 0.25 C 0.095 0.265 0.088 0.272 0.075 0.272 L 0.055 0.272 L 0.055 0.3 C 0.055 0.31 0.05 0.31 0.03 0.31 L -0.02 0.31 C -0.04 0.31 -0.045 0.3 -0.045 0.272 L -0.07 0.272 C -0.083 0.272 -0.09 0.265 -0.09 0.25 Z";

const louvres = (n, z) => Array.from({ length: n }, (_, i) => {
  const y = 0.075 + (i * 0.14) / (n - 1);
  return line(`louvre-${i}`, "body", z, [[0.079, y], [0.098, y - 0.009]], { region: "grille", tone: 0.9 });
});
const slots = (n, z) => Array.from({ length: n }, (_, i) => line(`inlet-slot-${i}`, "body", z, [[-0.098, 0.06 + (i * 0.13) / (n - 1)], [-0.088, 0.06 + (i * 0.13) / (n - 1)]], { region: "intake", tone: 0.9 }));

const el50 = [
  rect("foot-back", "body", 2.8, 0.15, -0.075, 0, 0.035, 0.016, { region: "feet", material: "rubber-foot" }),
  rect("foot-front", "body", 2.8, 0.15, 0.04, 0, 0.035, 0.016, { region: "feet", material: "rubber-foot" }),
  path("handle", "body", 2.9, 0.75, handle, { region: "casing", material: "abs-cream" }),
  rect("handle-opening", "body", 3.3, 0.2, -0.028, 0.278, 0.065, 0.02, { role: "detail", filled: false, rr: 0.008, region: "casing" }),
  path("housing", "body", 3, 0.78, housing, { region: "casing", material: "abs-cream" }),
  path("top-cover", "body", 3.1, 0.6, hood, { region: "top-cover", material: "abs-cream" }),
  rect("outlet-grille", "body", 3.2, 0.2, 0.078, 0.06, 0.02, 0.17, { rr: 0.004, region: "grille", material: "steel-mesh" }),
  ...louvres(8, 3.4),
  rect("inlet-grille", "body", 3.2, 0.3, -0.098, 0.05, 0.01, 0.16, { rr: 0.003, region: "intake", material: "pc-heat" }),
  ...slots(7, 3.4),
  line("seam-lower", "body", 3.3, [[-0.09, 0.045], [0.095, 0.045]], { region: "casing" }),
  line("highlight", "body", 3.3, [[-0.06, 0.255], [-0.06, 0.2]], { region: "casing", tone: 0.95 }),
  ell("dial", "knob", 3.5, 0.2, 0, 0, 0.024, 0.024, { region: "control", material: "pp-dial" }),
  ell("dial-hub", "knob", 3.6, 0.35, 0, 0, 0.012, 0.012, { region: "control" }),
  line("dial-pointer", "knob", 3.7, [[0, 0.006], [0, 0.022]], { region: "control", tone: 0.95 }),
  rect("mode-switch", "body", 3.4, 0.2, -0.02, 0.092, 0.04, 0.02, { rr: 0.006, region: "control", material: "pp-dial" }),
  line("mode-mark", "body", 3.6, [[-0.01, 0.102], [0.01, 0.102]], { region: "control", tone: 0.95 }),
];

const el100 = [
  rect("foot-back", "body", 2.8, 0.15, -0.075, 0, 0.035, 0.02, { region: "feet", material: "rubber-foot" }),
  rect("foot-front", "body", 2.8, 0.15, 0.04, 0, 0.035, 0.02, { region: "feet", material: "rubber-foot" }),
  path("handle", "body", 2.9, 0.75, handle, { region: "casing", material: "abs-cream" }),
  path("housing", "body", 3, 0.78, housing.replace(/ 0\.016/g, " 0.02"), { region: "casing", material: "abs-cream" }),
  line("top-cover-edge", "body", 3.1, [[-0.08, 0.258], [0.09, 0.258]], { region: "top-cover" }),
  rect("outlet-grille", "body", 3.2, 0.2, 0.078, 0.06, 0.02, 0.17, { region: "grille", material: "steel-mesh" }),
  ...louvres(4, 3.4),
  rect("inlet-grille", "body", 3.2, 0.3, -0.098, 0.05, 0.01, 0.16, { region: "intake", material: "pc-heat" }),
  ell("dial", "knob", 3.5, 0.2, 0, 0, 0.024, 0.024, { region: "control", material: "pp-dial" }),
  line("dial-pointer", "knob", 3.7, [[0, 0.006], [0, 0.022]], { region: "control", tone: 0.95 }),
];

const el200 = [
  path("block", "body", 3, 0.78, block200, { regions: ["casing", "top-cover", "intake", "control", "feet"], material: "abs-cream" }),
  rect("outlet-grille", "body", 3.2, 0.2, 0.078, 0.06, 0.02, 0.17, { region: "grille", material: "steel-mesh" }),
];

const dec = {
  50: {
    silhouette: t("A fan heater 3.9 mm wide and 6.2 mm high in side view: a cream upright housing with a carry handle, a protruding louvred outlet grille at the front, a ribbed inlet grille at the back, a thermostat dial with pointer and a mode switch on the side wall, on two rubber feet.", "Ein Heizlüfter, in der Seitenansicht 3,9 mm breit und 6,2 mm hoch: ein cremefarbenes stehendes Gehäuse mit Tragegriff, ein vorstehendes Lamellengitter als Auslass vorn, ein geschlitztes Einlassgitter hinten, ein Thermostatregler mit Zeiger und ein Betriebsartenschalter an der Seitenwand, auf zwei Gummifüßen."),
    retained: t("Width 0.196 m, height 0.31 m, feet 0.016 m, the 3 mm protrusion of the outlet grille and the 10 mm of the inlet grille; eight louvre fins at an angle, seven inlet slots, handle opening, lower seam, top-cover edge, dial pointer.", "Breite 0,196 m, Höhe 0,31 m, Füße 0,016 m, die 3 mm Auskragung des Auslassgitters und die 10 mm des Einlassgitters; acht schräge Lamellen, sieben Einlassschlitze, Griffloch, untere Fuge, Haubenkante, Reglerzeiger."),
    merged: t("Handle and housing keep one region; the two grille frames and their bars are one shape each.", "Griff und Gehäuse behalten eine Region; die zwei Gitterrahmen und ihre Stäbe sind je eine Form."),
    omitted: t("Mains cable, tip-over switch, label, side vent slots, fan blades inside.", "Netzkabel, Kippschalter, Typenschild, seitliche Lüftungsschlitze, Lüfterflügel im Inneren."),
    thinElements: t("Louvre fins and inlet slots are 0.13 mm strokes at 0.02 m (0.4 mm) spacing; the grille frames are 0.01-0.02 m = 0.2-0.4 mm.", "Lamellen und Einlassschlitze sind 0,13-mm-Striche im Abstand 0,02 m (0,4 mm); die Gitterrahmen sind 0,01-0,02 m = 0,2-0,4 mm."),
    hatching: t("None.", "Keine."),
    styles: t("Housing 0.78, handle 0.75, top cover 0.6, grille 0.2, inlet 0.3, controls 0.2, feet 0.15; body pen 0.35 mm, detail 0.18 mm.", "Gehäuse 0,78, Griff 0,75, Haube 0,6, Gitter 0,2, Einlass 0,3, Bedienung 0,2, Füße 0,15; Körperstift 0,35 mm, Detail 0,18 mm."),
    overlays: t("Six regions: the thermal view paints the hot outlet grille and the warm top cover, the cool inlet and the housing; the warm jet leaves the grille to the right.", "Sechs Regionen: Die Thermalansicht färbt heißes Auslassgitter und warme Haube, kühlen Einlass und Gehäuse; der warme Luftstrahl verlässt das Gitter nach rechts."),
  },
  100: {
    silhouette: t("A fan heater 2 mm wide and 3.1 mm high: an upright cream block with a handle bump, a front outlet grille with four louvre lines, a back inlet grille, a round dial and two stub feet.", "Ein Heizlüfter, 2 mm breit und 3,1 mm hoch: ein stehender cremefarbener Block mit Griffbuckel, ein vorderes Auslassgitter mit vier Lamellenlinien, ein hinteres Einlassgitter, ein runder Regler und zwei Stummelfüße."),
    retained: t("Overall size, grille protrusions, four of the eight louvre lines, handle, dial, top-cover edge.", "Gesamtgröße, Gitterauskragungen, vier der acht Lamellenlinien, Griff, Regler, Haubenkante."),
    merged: t("Handle opening, mode switch, lower seam and inlet slots merge into the block.", "Griffloch, Betriebsartenschalter, untere Fuge und Einlassschlitze gehen im Block auf."),
    omitted: t("Highlight, dial hub, mode mark, louvre details, cable.", "Lichtkante, Reglernabe, Schaltermarke, Lamellendetails, Kabel."),
    thinElements: t("The feet are 0.016 m = 0.16 mm high in reality: thickened to 0.02 m (0.2 mm); the louvre lines are 0.13 mm strokes at 0.04 m (0.4 mm).", "Die Füße sind real 0,016 m = 0,16 mm hoch: auf 0,02 m (0,2 mm) verdickt; die Lamellenlinien sind 0,13-mm-Striche im Abstand 0,04 m (0,4 mm)."),
    hatching: t("None.", "Keine."),
    styles: t("As at 1:50; body pen 0.25 mm, detail 0.13 mm.", "Wie bei 1:50; Körperstift 0,25 mm, Detail 0,13 mm."),
    overlays: t("All six regions are drawn; the top cover is the edge line.", "Alle sechs Regionen sind gezeichnet; die Haube ist die Kantenlinie."),
  },
  200: {
    silhouette: t("A fan heater 1 mm wide and 1.55 mm high: one upright block with a handle bump and a small protruding outlet grille.", "Ein Heizlüfter, 1 mm breit und 1,55 mm hoch: ein stehender Block mit Griffbuckel und einem kleinen vorstehenden Auslassgitter."),
    retained: t("Height 0.31 m, width 0.196 m, the grille at the front where the jet leaves.", "Höhe 0,31 m, Breite 0,196 m, das Gitter vorn, wo der Luftstrahl austritt."),
    merged: t("Housing, top cover, inlet, controls and feet are one block (regions listed).", "Gehäuse, Haube, Einlass, Bedienung und Füße sind ein Block (Regionen gelistet)."),
    omitted: t("Louvres, dial, switch, handle opening, feet gap, seams.", "Lamellen, Regler, Schalter, Griffloch, Fußlücke, Fugen."),
    thinElements: t("The outlet grille protrudes 0.02 m = 0.1 mm in reality and is drawn at that depth within the 0.15 mm stroke of the block.", "Das Auslassgitter steht real 0,02 m = 0,1 mm vor und ist in dieser Tiefe innerhalb des 0,15-mm-Strichs des Blocks gezeichnet."),
    hatching: t("None.", "Keine."),
    styles: t("Block 0.78, grille 0.2; body pen 0.18 mm.", "Block 0,78, Gitter 0,2; Körperstift 0,18 mm."),
    overlays: t("The block lists five regions and the grille keeps its own, so the hot grille stays distinct on the thermal view.", "Der Block listet fünf Regionen und das Gitter behält seine eigene, sodass das heiße Gitter in der Thermalansicht erkennbar bleibt."),
  },
};

const mk = (den, lo, hi, pens, elev) => rep(den, lo, hi, pens, [], elev, dec[den]);
const reps = {
  500: unavailable(500,
    t("A fan heater is 0.196 m x 0.31 m = 0.4 x 0.6 mm on paper at 1:500: the grille, handle and feet are far below the 0.2 mm feature width and only a speck would print.", "Ein Heizlüfter misst 0,196 m x 0,31 m = 0,4 x 0,6 mm auf dem Papier bei 1:500: Gitter, Griff und Füße liegen weit unter der Merkmalsbreite von 0,2 mm, es würde nur ein Fleck drucken."),
    t("Grille, handle and feet (0.016-0.05 m = 0.03-0.1 mm)", "Gitter, Griff und Füße (0,016-0,05 m = 0,03-0,1 mm)"), 0.02,
    [{ kind: "scale", denominator: 200 }]),
  200: mk(200, 150, 350, { body: 0.15, detail: null, texture: null, glazing: null, ground: 0.25 }, el200),
  100: mk(100, 75, 150, { body: 0.25, detail: 0.13, texture: null, glazing: null, ground: 0.35 }, el100),
  50: mk(50, 20, 75, { body: 0.35, detail: 0.18, texture: 0.13, glazing: null, ground: 0.5 }, el50),
};

const bones = [
  { id: "body", parent: null, title: t("Housing with grilles, handle and feet", "Gehäuse mit Gittern, Griff und Füßen"), x: 0, y: 0, angleDeg: 0, lengthM: 0.31, joint: { type: "fixed" }, mass: { kg: 1.47, comX: 0, comY: 0.14, distribution: "point", source: "authored-estimate", reference: "iec-60335-2-30" } },
  { id: "knob", parent: "body", title: t("Thermostat dial", "Thermostatregler"), x: 0, y: 0.185, angleDeg: 0, lengthM: 0.024, joint: { type: "revolute", minDeg: 0, maxDeg: 270 }, mass: { kg: 0.03, comX: 0, comY: 0, distribution: "point", source: "authored-estimate" } },
];
const anchors = [
  { id: "foot-back", bone: "body", x: -0.058, y: 0, kind: "contact", title: t("Rear foot", "Hinterer Fuß"), support: "contact", normalDeg: 90, footprintLengthM: 0.035, footprintWidthM: 0.04, frictionCoefficient: 0.6 },
  { id: "foot-front", bone: "body", x: 0.058, y: 0, kind: "contact", title: t("Front foot", "Vorderer Fuß"), support: "contact", normalDeg: 90, footprintLengthM: 0.035, footprintWidthM: 0.04, frictionCoefficient: 0.6 },
  { id: "outlet", bone: "body", x: 0.098, y: 0.145, kind: "emitter", title: t("Warm air outlet in the front grille", "Warmluftauslass im vorderen Gitter") },
];

const refs = [
  { id: "iec-60335-2-30", citation: "IEC 60335-2-30 Household and similar electrical appliances - Safety - Part 2-30: Particular requirements for room heaters (product class; fan heaters of 1000 and 2000 W stages)." },
  { id: "eu-2015-1188", citation: "Commission Regulation (EU) 2015/1188 implementing Directive 2009/125/EC with regard to ecodesign requirements for local space heaters (portable electric fan heaters fall into its scope; the full input leaves as heat in the room)." },
  { id: "incropera-2018", citation: "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (jets, emissivities, convection)." },
  { id: "iso-10456-2007", citation: "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (polymers, steel)." },
];

const m1 = (id, title, kind, m, d) => [layer(id, title, kind, m, d, "authored-estimate", "iso-10456-2007")];
const regions = [
  region({ id: "casing", title: t("Housing walls and handle", "Gehäusewände und Griff"), kind: "surface", model: "operating", material: abs, areaM2: 0.17, L: 0.28, geometry: "vertical-plate", svf: 0.6, layers: m1("wall", t("ABS wall 2.5 mm", "ABS-Wand 2,5 mm"), "polymer", abs, 0.0025), operating: { key: "stage-1", heatW: 33 }, reference: "iec-60335-2-30" }),
  region({ id: "top-cover", title: t("Top cover over the heating coils", "Haube über den Heizwendeln"), kind: "heater", model: "operating", material: abs, areaM2: 0.035, L: 0.17, geometry: "horizontal-plate-up", svf: 0.9, layers: m1("cover", t("ABS cover 2.5 mm", "ABS-Haube 2,5 mm"), "polymer", abs, 0.0025), operating: { key: "stage-2", heatW: 93 }, reference: "iec-60335-2-30" }),
  region({ id: "grille", title: t("Warm air outlet grille", "Warmluft-Auslassgitter"), kind: "heater", model: "operating", material: mesh, areaM2: 0.03, L: 0.17, geometry: "vertical-plate", svf: 0.5, layers: m1("bars", t("Steel grille bars 1.5 mm", "Stahlgitterstäbe 1,5 mm"), "metal", mesh, 0.0015), operating: { key: "stage-1", heatW: 60 }, reference: "iec-60335-2-30" }),
  region({ id: "intake", title: t("Air inlet grille at the back", "Lufteinlassgitter hinten"), kind: "surface", model: "passive", material: pc, areaM2: 0.03, L: 0.16, geometry: "vertical-plate", svf: 0.5, layers: m1("inlet", t("Polycarbonate grille 2 mm", "Polycarbonatgitter 2 mm"), "polymer", pc, 0.002), reference: "iso-10456-2007" }),
  region({ id: "control", title: t("Dial and mode switch", "Regler und Betriebsartenschalter"), kind: "surface", model: "passive", material: knobM, areaM2: 0.004, L: 0.04, geometry: "vertical-plate", svf: 0.5, layers: m1("dial", t("Polypropylene 3 mm", "Polypropylen 3 mm"), "polymer", knobM, 0.003), reference: "iso-10456-2007" }),
  region({ id: "feet", title: t("Rubber feet", "Gummifüße"), kind: "surface", model: "passive", material: rubber, areaM2: 0.003, L: 0.04, geometry: "horizontal-plate-down", svf: 0.1, layers: m1("foot", t("Rubber 10 mm", "Gummi 10 mm"), "polymer", rubber, 0.01), reference: "iso-10456-2007" }),
];

const emitters = [
  emitter({ id: "warm-air-1", kind: "fan", title: t("Warm air of the first heating stage", "Warmluft der ersten Heizstufe"), anchor: "outlet", dir: 0, d: 0.1, key: "stage-1", flow: 0.016, rise: 47, vapour: 0, reference: "iec-60335-2-30" }),
  emitter({ id: "warm-air-2", kind: "fan", title: t("Warm air of the second heating stage", "Warmluft der zweiten Heizstufe"), anchor: "outlet", dir: 0, d: 0.1, key: "stage-2", flow: 0.016, rise: 47, vapour: 0, reference: "iec-60335-2-30" }),
];
const key1 = operatingKeyDef("stage-1", t("First heating stage and fan", "Erste Heizstufe und Lüfter"), [[0, "Off", "Aus"], [1, "On (1000 W)", "Ein (1000 W)"]]);
const key2 = operatingKeyDef("stage-2", t("Second heating stage", "Zweite Heizstufe"), [[0, "Off", "Aus"], [1, "On (further 1000 W)", "Ein (weitere 1000 W)"]]);

const mkPose = (id, title, description, knob, s1, s2, over = {}) => ({
  id, title, description, bones: { knob: { angleDeg: knob } }, ground: "auto", contacts: ["foot-back", "foot-front"], loads: [],
  thermal: { operating: { "stage-1": s1, "stage-2": s2 }, regions: regionBindings(ids, {}, over), emitters: { "warm-air-1": s1, "warm-air-2": s2 } },
});
const poses = [
  mkPose("off", t("Off", "Aus"), t("Cold heater, dial at zero, mode switch off.", "Kalter Heizlüfter, Regler auf null, Betriebsartenschalter aus."), 0, 0, 0, { feet: { exposedFraction: 0.2 } }),
  mkPose("heating-low", t("Heating on the first stage", "Heizt auf der ersten Stufe"), t("1000 W: the fan blows 0.016 m3/s of air warmed by 47 K out of the front grille.", "1000 W: Der Lüfter bläst 0,016 m3/s um 47 K erwärmte Luft aus dem vorderen Gitter."), 120, 1, 0, { feet: { exposedFraction: 0.2 } }),
  mkPose("heating", t("Heating on both stages", "Heizt auf beiden Stufen"), t("2000 W: 0.032 m3/s of air warmed by 47 K leaves the front grille as a warm jet; grille and top cover are the hottest surfaces.", "2000 W: 0,032 m3/s um 47 K erwärmte Luft verlassen das vordere Gitter als warmer Strahl; Gitter und Haube sind die heißesten Flächen."), 240, 1, 1, { feet: { exposedFraction: 0.2 } }),
];
const contactsFor = (d) => ["foot-back", "foot-front"].map((anchor) => ({ anchor, fromS: 0, toS: d }));
const tr = (d, pts, name) => ({ target: name, ease: "step", keys: pts.map(([tt, v]) => ({ t: tt, v })) });

const clips = [
  { id: "warm-up", title: t("Switch on and warm up", "Einschalten und aufwärmen"), description: t("Cold start on both stages: the grille and cover heat up within a minute and the room air starts to warm up.", "Kaltstart auf beiden Stufen: Gitter und Haube heizen sich binnen einer Minute auf und die Raumluft beginnt sich zu erwärmen."), durationS: 900, fps: 1, loop: false, rootMotion: "none",
    keys: [{ t: 0, pose: "off", ease: "step" }, { t: 5, pose: "heating", ease: "step" }, { t: 900, pose: "heating", ease: "step" }],
    contacts: contactsFor(900), phases: [{ id: "cold", fromS: 0, toS: 5, title: t("Cold", "Kalt") }, { id: "warm-up", fromS: 5, toS: 900, title: t("Heating at 2000 W", "Heizt mit 2000 W") }],
    events: [{ t: 5, kind: "ignition", title: t("Both stages on", "Beide Stufen an"), target: "stage-1" }],
    tracks: [tr(900, [[0, 0], [5, 1]], "operating:stage-1"), tr(900, [[0, 0], [5, 1]], "operating:stage-2"), tr(900, [[0, 0], [5, 1]], "emitter:warm-air-1"), tr(900, [[0, 0], [5, 1]], "emitter:warm-air-2")] },
  { id: "step-down-and-off", title: t("Step down and switch off", "Herunterschalten und ausschalten"), description: t("After half an hour on 2000 W the second stage goes off, later the first one; grille and cover cool down.", "Nach einer halben Stunde mit 2000 W schaltet die zweite Stufe ab, später die erste; Gitter und Haube kühlen ab."), durationS: 1800, fps: 1, loop: false, rootMotion: "none",
    keys: [{ t: 0, pose: "heating", ease: "step" }, { t: 60, pose: "heating-low", ease: "step" }, { t: 600, pose: "off", ease: "step" }, { t: 1800, pose: "off", ease: "step" }],
    contacts: contactsFor(1800), phases: [{ id: "full", fromS: 0, toS: 60, title: t("2000 W", "2000 W") }, { id: "low", fromS: 60, toS: 600, title: t("1000 W", "1000 W") }, { id: "cool-down", fromS: 600, toS: 1800, title: t("Off, cooling", "Aus, kühlt ab") }],
    events: [{ t: 60, kind: "operating", title: t("Second stage off", "Zweite Stufe aus"), target: "stage-2" }, { t: 600, kind: "shutdown", title: t("First stage off", "Erste Stufe aus"), target: "stage-1" }],
    tracks: [tr(0, [[0, 1], [600, 0]], "operating:stage-1"), tr(0, [[0, 1], [60, 0]], "operating:stage-2"), tr(0, [[0, 1], [600, 0]], "emitter:warm-air-1"), tr(0, [[0, 1], [60, 0]], "emitter:warm-air-2")] },
];

export const heater = {
  id: "electric-fan-heater", category: "appliances",
  title: t("Electric Fan Heater", "Elektrischer Heizlüfter"),
  summary: t("A portable 2000 W fan heater, 0.31 m high, in two 1000 W stages: the fan blows 0.032 m3/s of air warmed by 47 K out of the front grille as a warm jet, while grille, top cover and housing release the remaining 186 W.", "Ein tragbarer Heizlüfter von 2000 W, 0,31 m hoch, in zwei Stufen zu je 1000 W: Der Lüfter bläst 0,032 m3/s um 47 K erwärmte Luft als warmen Strahl aus dem vorderen Gitter, während Gitter, Haube und Gehäuse die restlichen 186 W abgeben."),
  tags: ["fan-heater", "heater", "appliance", "internal-gain", "electric", "heat-source", "air-jet"],
  provenance: { author: "Semio staffage content (A4)", method: "handcrafted", created: "2026-11-01", references: refs,
    notes: { en: "Side view of a typical upright portable fan heater (no single product): 0.196 m x 0.24 m x 0.272 m housing plus a 0.038 m handle, feet 0.016 m, mass 1.5 kg, rated 2000 W in two stages (class of IEC 60335-2-30). All 2000 W end up in the room: 1814 W as warm air (2 x 0.016 m3/s at 47 K rise), 60 W in the outlet grille, 93 W in the top cover and 33 W in the housing; the split is an authored estimate. Each stage owns one emitter and one operating key, because the emitter model scales flow and temperature rise together with its strength, so one emitter at half strength would carry only a quarter of the heat.", de: "Seitenansicht eines typischen stehenden tragbaren Heizlüfters (kein einzelnes Produkt): Gehäuse 0,196 m x 0,24 m x 0,272 m plus 0,038 m Griff, Füße 0,016 m, Masse 1,5 kg, Nennleistung 2000 W in zwei Stufen (Klasse der IEC 60335-2-30). Alle 2000 W landen im Raum: 1814 W als Warmluft (2 x 0,016 m3/s bei 47 K Anstieg), 60 W im Auslassgitter, 93 W in der Haube und 33 W im Gehäuse; die Aufteilung ist ein Schätzwert. Jede Stufe hat einen eigenen Emitter und einen eigenen Betriebsschlüssel, weil das Emittermodell Volumenstrom und Temperaturanstieg gemeinsam mit seiner Stärke skaliert, ein Emitter bei halber Stärke also nur ein Viertel der Wärme trüge." } },
  dimensions: { heightM: 0.31, widthM: 0.196, depthM: 0.24, massKg: 1.5 },
  materials,
  rig: { kind: "mechanical", root: "body", bones },
  anchors,
  representations: reps,
  symbols: [],
  poses, clips,
  thermal: {
    environment: { airTempC: 20, meanRadiantTempC: 20, relativeHumidity: 0.5, windMS: 0, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325, skyTempC: 20, groundTempC: 20 },
    operating: [key1, key2], passengers: [], regions, emitters,
    assumptions: [
      t("The heating coils and the fan motor inside are not regions: their 2000 W leave as warm jet air (1814 W), through the outlet grille (60 W), top cover (93 W) and housing (33 W). A room sees all of it as an internal gain.", "Heizwendeln und Lüftermotor im Inneren sind keine Regionen: Ihre 2000 W verlassen das Gerät als warme Strahlluft (1814 W), durch das Auslassgitter (60 W), die Haube (93 W) und das Gehäuse (33 W). Ein Raum erlebt alles davon als innere Last."),
      t("The jet leaves horizontally to the right (direction 0 degrees) at about 3 m/s through the 0.1 m outlet and is carried by the air-jet transport model; room air entrainment is left to that model.", "Der Strahl tritt waagerecht nach rechts (Richtung 0 Grad) mit etwa 3 m/s durch den 0,1-m-Auslass aus und wird vom Luftstrahl-Transportmodell getragen; die Raumlufteinmischung bleibt diesem Modell überlassen."),
    ],
  },
  structure: { gravityMS2: 9.80665, dynamics: "quasi-static", supports: ["foot-back", "foot-front"], assumptions: [t("A rigid body standing on two rubber feet.", "Ein starrer Körper auf zwei Gummifüßen.")], clinical: false },
  presentation: { defaultPose: "heating", defaultClip: "warm-up", defaultScale: 50, viewpoints: ["elevation"] },
  exports: ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"],
  completion: "authored",
};

export const heaterManifest = { id: "electric-fan-heater", category: "appliances", title: heater.title, heightM: 0.31, widthM: 0.196, poses: poses.map((p) => p.id), clips: clips.map((c) => c.id), representations: { 50: "authored", 100: "authored", 200: "authored", 500: "unavailable" }, symbols: [], analytics: ["thermal", "transport", "structural"], viewpoints: ["elevation"], exports: heater.exports, distinction: { en: "A 2000 W portable fan heater in two stages: a 0.032 m3/s warm jet leaves the front grille while grille, cover and housing warm up.", de: "Ein tragbarer Heizlüfter von 2000 W in zwei Stufen: Ein warmer Strahl von 0,032 m3/s verlässt das vordere Gitter, während Gitter, Haube und Gehäuse sich erwärmen." }, completion: "authored" };
