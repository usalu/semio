import { t, mat, layer, region, path, rect, ell, line, rep, unavailable, regionBindings, operatingKeyDef, emitter } from "./lib.mjs";

const chrome = mat("chrome-steel", t("Brushed chromed steel sheet", "Gebürstetes verchromtes Stahlblech"), 0.12, 0.35, 7850, 50, 450, "authored-estimate", "incropera-2018");
const hotplate = mat("mica-steel", t("Dark steel top plate with slot rims", "Dunkle Stahl-Deckplatte mit Schlitzrändern"), 0.8, 0.8, 7850, 50, 450, "authored-estimate", "incropera-2018");
const bread = mat("bread-crust", t("Toasting bread crust", "Röstende Brotkruste"), 0.9, 0.6, 250, 0.1, 2500, "authored-estimate", "incropera-2018");
const plastic = mat("pp-black", t("Black polypropylene", "Schwarzes Polypropylen"), 0.9, 0.9, 900, 0.22, 1900, "authored-estimate", "iso-10456-2007");
const rubber = mat("rubber-foot", t("Rubber foot", "Gummifuß"), 0.9, 0.9, 1100, 0.2, 2000, "authored-estimate", "iso-10456-2007");
const materials = [chrome, hotplate, bread, plastic, rubber];

const ids = ["casing", "top-plate", "bread", "control", "feet"];

const bodyPath = "M -0.138 0.012 L 0.138 0.012 C 0.145 0.012 0.15 0.017 0.15 0.024 L 0.15 0.15 C 0.15 0.172 0.136 0.188 0.114 0.188 L -0.114 0.188 C -0.136 0.188 -0.15 0.172 -0.15 0.15 L -0.15 0.024 C -0.15 0.017 -0.145 0.012 -0.138 0.012 Z";
const body200 = "M -0.15 0 L 0.15 0 L 0.15 0.15 C 0.15 0.172 0.136 0.188 0.114 0.188 L -0.114 0.188 C -0.136 0.188 -0.15 0.172 -0.15 0.15 Z";
const body100 = "M -0.14 0.02 L 0.14 0.02 C 0.146 0.02 0.15 0.026 0.15 0.034 L 0.15 0.15 C 0.15 0.172 0.136 0.188 0.114 0.188 L -0.114 0.188 C -0.136 0.188 -0.15 0.172 -0.15 0.15 L -0.15 0.034 C -0.15 0.026 -0.146 0.02 -0.14 0.02 Z";
const slice50 = "M -0.052 0.085 L -0.052 0.168 C -0.062 0.172 -0.064 0.19 -0.046 0.196 C -0.03 0.202 0.03 0.202 0.046 0.196 C 0.064 0.19 0.062 0.172 0.052 0.168 L 0.052 0.085 Z";
const slice100 = "M -0.052 0.085 L -0.052 0.166 C -0.064 0.172 -0.063 0.19 -0.046 0.196 C -0.03 0.201 0.03 0.201 0.046 0.196 C 0.063 0.19 0.064 0.172 0.052 0.166 L 0.052 0.085 Z";
const slice200 = "M -0.052 0.1 L -0.052 0.166 C -0.062 0.172 -0.062 0.19 -0.046 0.196 C -0.03 0.2 0.03 0.2 0.046 0.196 C 0.062 0.19 0.062 0.172 0.052 0.166 L 0.052 0.1 Z";
const topStrip = "M -0.128 0.172 L 0.128 0.172 L 0.114 0.188 L -0.114 0.188 Z";

const el50 = [
  rect("foot-left", "base", 2.8, 0.15, -0.125, 0, 0.05, 0.012, { region: "feet", material: "rubber-foot" }),
  rect("foot-right", "base", 2.8, 0.15, 0.075, 0, 0.05, 0.012, { region: "feet", material: "rubber-foot" }),
  path("slice-rear", "slices", 2.4, 0.7, slice50.replace(/-0.052/g, "-0.049").replace(/0.052/g, "0.049"), { region: "bread", material: "bread-crust" }),
  path("slice-front", "slices", 2.5, 0.78, slice50, { region: "bread", material: "bread-crust" }),
  line("crumb-edge", "slices", 2.6, [[-0.04, 0.176], [-0.038, 0.186], [-0.025, 0.192], [0.025, 0.192], [0.038, 0.186], [0.04, 0.176]], { region: "bread" }),
  path("casing", "base", 3, 0.82, bodyPath, { region: "casing", material: "chrome-steel" }),
  path("top-plate", "base", 3.2, 0.55, topStrip, { region: "top-plate", material: "mica-steel" }),
  line("seam-top", "base", 3.3, [[-0.128, 0.172], [0.128, 0.172]], { region: "top-plate" }),
  line("tray-slot", "base", 3.3, [[-0.1, 0.036], [0.1, 0.036]], { region: "casing" }),
  line("highlight", "base", 3.3, [[-0.138, 0.06], [-0.138, 0.14]], { region: "casing", tone: 0.1 }),
  rect("lever-track", "base", 3.3, 0.3, 0.094, 0.052, 0.012, 0.11, { role: "detail", filled: false, rr: 0.006, region: "casing" }),
  rect("lever", "lever", 3.5, 0.2, -0.015, -0.012, 0.03, 0.024, { rr: 0.006, region: "control", material: "pp-black" }),
  ell("dial", "dial", 3.5, 0.25, 0, 0, 0.015, 0.015, { region: "control", material: "pp-black" }),
  line("dial-pointer", "dial", 3.6, [[0, 0], [0, 0.012]], { region: "control", tone: 0.9 }),
  rect("cancel-button", "base", 3.4, 0.2, -0.04, 0.12, 0.024, 0.014, { rr: 0.004, region: "control", material: "pp-black" }),
];

const el100 = [
  rect("foot-left", "base", 2.8, 0.15, -0.125, 0, 0.05, 0.02, { region: "feet", material: "rubber-foot" }),
  rect("foot-right", "base", 2.8, 0.15, 0.075, 0, 0.05, 0.02, { region: "feet", material: "rubber-foot" }),
  path("slice-front", "slices", 2.5, 0.78, slice100, { region: "bread", material: "bread-crust" }),
  path("casing", "base", 3, 0.82, body100, { region: "casing", material: "chrome-steel" }),
  line("top-bevel", "base", 3.3, [[-0.128, 0.172], [0.128, 0.172]], { region: "top-plate" }),
  rect("lever-track", "base", 3.3, 0.3, 0.094, 0.052, 0.012, 0.11, { role: "detail", filled: false, rr: 0.006, region: "casing" }),
  rect("lever", "lever", 3.5, 0.2, -0.015, -0.012, 0.03, 0.024, { region: "control", material: "pp-black" }),
  ell("dial", "dial", 3.5, 0.25, 0, 0, 0.015, 0.015, { region: "control", material: "pp-black" }),
];

const el200 = [
  path("slice", "slices", 2.5, 0.78, slice200, { region: "bread", material: "bread-crust" }),
  path("casing", "base", 3, 0.82, body200, { regions: ["casing", "top-plate", "control", "feet"], material: "chrome-steel" }),
];


const dec = {
  50: {
    silhouette: t("A chromed two-slice toaster 15 mm wide in front view: rounded box on two rubber feet with a bevelled dark top plate, a bread slice with its shoulders rising out of the slot, a sliding lever in its track, a browning dial with pointer and a cancel button.", "Ein verchromter Zweischeiben-Toaster in Vorderansicht, 15 mm breit: gerundeter Kasten auf zwei Gummifüßen mit abgeschrägter dunkler Deckplatte, eine Brotscheibe mit Schultern aus dem Schlitz, ein Hebel in seiner Führung, ein Bräunungsregler mit Zeiger und eine Abbruchtaste."),
    retained: t("Width 0.30 m, body height 0.188 m, feet 0.012 m and the 12 mm of bread above the plate exactly; both slices (the rear one peeks out beside the front one), tray slot, lever track and the chrome highlight.", "Breite 0,30 m, Gehäusehöhe 0,188 m, Füße 0,012 m und die 12 mm Brot über der Platte exakt; beide Scheiben (die hintere guckt neben der vorderen hervor), Krümelschublade, Hebelführung und die Chromlichtkante."),
    merged: t("The slot rims are merged into the top plate; the crumb edge is one line instead of a crust and a crumb contour.", "Die Schlitzränder gehen in der Deckplatte auf; die Krume ist eine Linie statt Kruste und Krumenkontur."),
    omitted: t("Logo, mains cable (it leaves at the back), bun-warmer rack, slot ribs and screws.", "Logo, Netzkabel (es verlässt das Gerät hinten), Brötchenaufsatz, Schlitzrippen und Schrauben."),
    thinElements: t("The lever track is 0.012 m = 0.24 mm wide, drawn as a 0.13 mm outline; the crumb edge and the dial pointer are 0.13 mm strokes.", "Die Hebelführung ist 0,012 m = 0,24 mm breit und als 0,13-mm-Kontur gezeichnet; Krumenkante und Reglerzeiger sind 0,13-mm-Striche."),
    hatching: t("None; the crust would need dots finer than 0.4 mm.", "Keine; die Kruste bräuchte Punkte feiner als 0,4 mm."),
    styles: t("Chrome casing 0.82, dark top plate 0.55, bread 0.78, plastic controls 0.2 and feet 0.15; body pen 0.35 mm, detail 0.18 mm.", "Chromgehäuse 0,82, dunkle Deckplatte 0,55, Brot 0,78, Kunststoffbedienung 0,2 und Füße 0,15; Körperstift 0,35 mm, Detail 0,18 mm."),
    overlays: t("Casing, top plate, bread, controls and feet are separate regions: the thermal view shows the hot top plate and the browning bread above a warm casing.", "Gehäuse, Deckplatte, Brot, Bedienung und Füße sind eigene Regionen: Die Thermalansicht zeigt die heiße Deckplatte und das bräunende Brot über einem warmen Gehäuse."),
  },
  100: {
    silhouette: t("A toaster 30 mm wide and 20 mm high: one chromed block with a bevel line, the bread shoulders above it, the lever in its track, the round dial and two stub feet.", "Ein Toaster, 30 mm breit und 20 mm hoch: ein verchromter Block mit Fasenlinie, die Brotschultern darüber, der Hebel in seiner Führung, der runde Regler und zwei Stummelfüße."),
    retained: t("Overall size, the bread shoulders, bevel line, lever and track, dial.", "Gesamtgröße, Brotschultern, Fasenlinie, Hebel und Führung, Regler."),
    merged: t("The rear slice merges behind the front one; the top plate is only the bevel line; the cancel button joins the casing.", "Die hintere Scheibe geht hinter der vorderen auf; die Deckplatte ist nur die Fasenlinie; die Abbruchtaste geht im Gehäuse auf."),
    omitted: t("Crumb tray slot, highlight, dial pointer, slot ribs, logo, cable.", "Krümelschubladenfuge, Lichtkante, Reglerzeiger, Schlitzrippen, Logo, Kabel."),
    thinElements: t("The rubber feet are 0.012 m = 0.12 mm high in reality: thickened to 0.02 m (0.2 mm) so they print; the body bottom rises by the same 0.008 m.", "Die Gummifüße sind real 0,012 m = 0,12 mm hoch: auf 0,02 m (0,2 mm) verdickt, damit sie drucken; der Gehäuseboden steigt um dieselben 0,008 m."),
    hatching: t("None.", "Keine."),
    styles: t("Chrome casing 0.82, bread 0.78, controls 0.2-0.25, feet 0.15; body pen 0.25 mm, detail 0.13 mm.", "Chromgehäuse 0,82, Brot 0,78, Bedienung 0,2-0,25, Füße 0,15; Körperstift 0,25 mm, Detail 0,13 mm."),
    overlays: t("The bevel line carries the top plate, track and bread their regions; the casing block carries the casing.", "Die Fasenlinie trägt die Deckplatte, Führung und Brot ihre Regionen; der Gehäuseblock trägt das Gehäuse."),
  },
  200: {
    silhouette: t("A toaster 1.5 mm wide and 1 mm high: one rounded block with a bread dome rising 0.06 mm out of the top edge.", "Ein Toaster, 1,5 mm breit und 1 mm hoch: ein gerundeter Block mit einer Brotkuppe, die 0,06 mm aus der Oberkante ragt."),
    retained: t("Width 0.30 m, body height 0.188 m, the bread shoulders above the top edge.", "Breite 0,30 m, Gehäusehöhe 0,188 m, die Brotschultern über der Oberkante."),
    merged: t("Casing, top plate, lever, dial and feet merge into one block (its shape lists the four regions); the two slices merge into one dome.", "Gehäuse, Deckplatte, Hebel, Regler und Füße gehen in einem Block auf (seine Form listet die vier Regionen); die zwei Scheiben bilden eine Kuppe."),
    omitted: t("Feet, lever, dial, track, bevel and every line.", "Füße, Hebel, Regler, Führung, Fase und jede Linie."),
    thinElements: t("The 0.012 m feet (0.06 mm) are merged into the block; the bread above the plate is 0.012 m = 0.06 mm and stays true: only the dome shoulders (0.3 mm wide) show.", "Die 0,012 m hohen Füße (0,06 mm) gehen im Block auf; das Brot über der Platte ist 0,012 m = 0,06 mm hoch und bleibt wahr: nur die Schultern der Kuppe (0,3 mm breit) sind zu sehen."),
    hatching: t("None.", "Keine."),
    styles: t("Block 0.82 and bread 0.78; body pen 0.18 mm.", "Block 0,82 und Brot 0,78; Körperstift 0,18 mm."),
    overlays: t("The block lists casing, top plate, controls and feet, so the thermal overlay paints it with their area-weighted value; the dome keeps the bread.", "Der Block listet Gehäuse, Deckplatte, Bedienung und Füße, die Thermalansicht färbt ihn mit deren flächengewichtetem Wert; die Kuppe behält das Brot."),
  },
};

const mk = (den, lo, hi, pens, hatches, elev) => rep(den, lo, hi, pens, hatches, elev, dec[den]);

const reps = {
  500: unavailable(500,
    t("A toaster is 0.30 m x 0.20 m = 0.6 x 0.4 mm on paper at 1:500: the bread dome (0.012 m = 0.02 mm), the feet and the lever are far below the 0.2 mm feature width, so only an anonymous speck would print.", "Ein Toaster misst 0,30 m x 0,20 m = 0,6 x 0,4 mm auf dem Papier bei 1:500: Brotkuppe (0,012 m = 0,02 mm), Füße und Hebel liegen weit unter der Merkmalsbreite von 0,2 mm, es würde nur ein namenloser Fleck drucken."),
    t("Bread dome, feet and lever (0.012-0.03 m = 0.02-0.06 mm)", "Brotkuppe, Füße und Hebel (0,012-0,03 m = 0,02-0,06 mm)"), 0.012,
    [{ kind: "scale", denominator: 200 }]),
  200: mk(200, 150, 350, { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 }, [], el200),
  100: mk(100, 75, 150, { body: 0.25, detail: 0.13, texture: null, glazing: null, ground: 0.35 }, [], el100),
  50: mk(50, 20, 75, { body: 0.35, detail: 0.18, texture: 0.13, glazing: null, ground: 0.5 }, [], el50),
};

const bones = [
  { id: "base", parent: null, title: t("Housing with top plate and feet", "Gehäuse mit Deckplatte und Füßen"), x: 0, y: 0, angleDeg: 0, lengthM: 0.188, joint: { type: "fixed" }, mass: { kg: 1.18, comX: 0, comY: 0.09, distribution: "point", source: "authored-estimate", reference: "iec-60335-2-9" } },
  { id: "slices", parent: "base", title: t("Carriage with two bread slices", "Schlitten mit zwei Brotscheiben"), x: 0, y: 0, angleDeg: 0, lengthM: 0.115, joint: { type: "prismatic", axisDeg: 90, minM: -0.06, maxM: 0.06 }, mass: { kg: 0.07, comX: 0, comY: 0.14, distribution: "point", source: "authored-estimate" } },
  { id: "lever", parent: "base", title: t("Lowering lever", "Absenkhebel"), x: 0.1, y: 0.145, angleDeg: 0, lengthM: 0.03, joint: { type: "prismatic", axisDeg: 90, minM: -0.06, maxM: 0 }, mass: { kg: 0.03, comX: 0, comY: 0, distribution: "point", source: "authored-estimate" } },
  { id: "dial", parent: "base", title: t("Browning dial", "Bräunungsregler"), x: -0.1, y: 0.13, angleDeg: 0, lengthM: 0.015, joint: { type: "revolute", minDeg: -10, maxDeg: 270 }, mass: { kg: 0.02, comX: 0, comY: 0, distribution: "point", source: "authored-estimate" } },
];

const anchors = [
  { id: "foot-left", bone: "base", x: -0.1, y: 0, kind: "contact", title: t("Left rubber foot", "Linker Gummifuß"), support: "contact", normalDeg: 90, footprintLengthM: 0.05, footprintWidthM: 0.03, frictionCoefficient: 0.6 },
  { id: "foot-right", bone: "base", x: 0.1, y: 0, kind: "contact", title: t("Right rubber foot", "Rechter Gummifuß"), support: "contact", normalDeg: 90, footprintLengthM: 0.05, footprintWidthM: 0.03, frictionCoefficient: 0.6 },
  { id: "slot-outlet", bone: "base", x: 0, y: 0.188, kind: "emitter", title: t("Warm air above the slots", "Warme Luft über den Schlitzen") },
];

const refs = [
  { id: "iec-60335-2-9", citation: "IEC 60335-2-9 Household and similar electrical appliances - Safety - Part 2-9: Particular requirements for grills, toasters and similar portable cooking appliances (product class; rated input of the class 800-1000 W; the split of the input is an authored estimate)." },
  { id: "incropera-2018", citation: "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (surface emissivities, natural convection, plumes)." },
  { id: "iso-10456-2007", citation: "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (steel, polymers)." },
];

const keyElements = operatingKeyDef("elements", t("Heating elements", "Heizdrähte"), [[0, "Off", "Aus"], [1, "Toasting (900 W)", "Toasten (900 W)"]]);

const casingRegion = region({ id: "casing", title: t("Housing walls (chromed sheet)", "Gehäusewände (verchromtes Blech)"), kind: "surface", model: "operating", material: chrome, areaM2: 0.24, L: 0.19, geometry: "vertical-plate", svf: 0.5, layers: [layer("sheet", t("Chromed steel sheet 0.8 mm", "Verchromtes Stahlblech 0,8 mm"), "metal", chrome, 0.0008, "authored-estimate", "iso-10456-2007")], operating: { key: "elements", heatW: 180 }, reference: "iec-60335-2-9" });
const regions = [
  casingRegion,
  region({ id: "top-plate", title: t("Top plate and slot rims", "Deckplatte und Schlitzränder"), kind: "heater", model: "operating", material: hotplate, areaM2: 0.045, L: 0.17, geometry: "horizontal-plate-up", svf: 0.9, layers: [layer("plate", t("Steel plate 1 mm", "Stahlplatte 1 mm"), "metal", hotplate, 0.001, "authored-estimate", "iso-10456-2007")], operating: { key: "elements", heatW: 100 }, reference: "iec-60335-2-9" }),
  region({ id: "bread", title: t("Two bread slices (crust surface)", "Zwei Brotscheiben (Krustenoberfläche)"), kind: "surface", model: "operating", material: bread, areaM2: 0.045, L: 0.11, geometry: "vertical-plate", svf: 0.4, layers: [layer("crust", t("Crust and outer crumb 5 mm", "Kruste und äußere Krume 5 mm"), "insulation", bread, 0.005, "authored-estimate", "incropera-2018")], operating: { key: "elements", heatW: 150 }, reference: "incropera-2018" }),
  region({ id: "control", title: t("Lever and dial", "Hebel und Regler"), kind: "surface", model: "passive", material: plastic, areaM2: 0.003, L: 0.03, geometry: "sphere", svf: 0.5, layers: [layer("knob", t("Polypropylene 3 mm", "Polypropylen 3 mm"), "polymer", plastic, 0.003, "authored-estimate", "iso-10456-2007")], reference: "iso-10456-2007" }),
  region({ id: "feet", title: t("Rubber feet", "Gummifüße"), kind: "surface", model: "passive", material: rubber, areaM2: 0.002, L: 0.04, geometry: "horizontal-plate-down", svf: 0.1, layers: [layer("foot", t("Rubber 8 mm", "Gummi 8 mm"), "polymer", rubber, 0.008, "authored-estimate", "iso-10456-2007")], reference: "iso-10456-2007" }),
];

const emitters = [emitter({ id: "slot-air", kind: "vent", title: t("Hot air rising from the slots", "Heiße Luft aus den Schlitzen"), anchor: "slot-outlet", dir: 90, d: 0.05, key: "elements", flow: 0.0044, rise: 88, vapour: 0.03, reference: "incropera-2018" })];

const bind = (over, elements, slot, bonesOffsets, dialDeg) => ({
  bones: bonesOffsets, ground: "auto", contacts: ["foot-left", "foot-right"], loads: [],
  thermal: { operating: { elements }, regions: regionBindings(ids, {}, over), emitters: { "slot-air": slot } },
});

const poses = [
  { id: "idle", title: t("Idle and empty", "Leer und aus"), description: t("Cold toaster without bread, lever up, dial on the middle setting.", "Kalter Toaster ohne Brot, Hebel oben, Regler in der mittleren Stufe."), ...bind({ bread: { exposedFraction: 0 }, "top-plate": { sunlitFraction: 0 } }, 0, 0, { slices: { dy: -0.04 }, lever: { dy: 0 }, dial: { angleDeg: 120 } }) },
  { id: "toasting", title: t("Toasting", "Toastet"), description: t("Lever down: the two slices sit in the slots with 12 mm above the plate, the elements draw 900 W and hot air rises from the slots.", "Hebel unten: Die zwei Scheiben stecken in den Schlitzen, 12 mm ragen über die Platte, die Heizdrähte ziehen 900 W und heiße Luft steigt aus den Schlitzen."), ...bind({ bread: { exposedFraction: 0.9 } }, 1, 1, { slices: { dy: 0 }, lever: { dy: -0.045 }, dial: { angleDeg: 120 } }) },
  { id: "popped", title: t("Toast popped up", "Toast ausgeworfen"), description: t("The elements are off, the carriage has lifted the browned slices 40 mm; the casing and the plate are still warm.", "Die Heizdrähte sind aus, der Schlitten hat die gebräunten Scheiben 40 mm angehoben; Gehäuse und Platte sind noch warm."), ...bind({ bread: { exposedFraction: 1 } }, 0, 0, { slices: { dy: 0.04 }, lever: { dy: 0 }, dial: { angleDeg: 120 } }) },
];

const contactsFor = (d) => [{ anchor: "foot-left", fromS: 0, toS: d }, { anchor: "foot-right", fromS: 0, toS: d }];

const clips = [
  { id: "toast-cycle", title: t("One toast cycle", "Ein Toastvorgang"), description: t("Bread goes in and the lever is pressed: the elements heat for 150 s, then the carriage pops the toast up.", "Brot kommt hinein und der Hebel wird gedrückt: Die Heizdrähte heizen 150 s, dann wirft der Schlitten den Toast aus."), durationS: 300, fps: 1, loop: false, rootMotion: "none",
    keys: [{ t: 0, pose: "idle", ease: "step" }, { t: 5, pose: "toasting", ease: "step" }, { t: 155, pose: "popped", ease: "step" }, { t: 300, pose: "popped", ease: "step" }],
    contacts: contactsFor(300),
    phases: [{ id: "empty", fromS: 0, toS: 5, title: t("Empty", "Leer") }, { id: "toasting", fromS: 5, toS: 155, title: t("Toasting", "Toastet") }, { id: "popped", fromS: 155, toS: 300, title: t("Toast up, cooling", "Toast oben, kühlt ab") }],
    events: [{ t: 5, kind: "ignition", title: t("Lever down, elements on", "Hebel unten, Heizung an"), target: "elements" }, { t: 155, kind: "shutdown", title: t("Carriage pops, elements off", "Schlitten wirft aus, Heizung aus"), target: "elements" }],
    tracks: [{ target: "operating:elements", ease: "step", keys: [{ t: 0, v: 0 }, { t: 5, v: 1 }, { t: 155, v: 0 }] }, { target: "emitter:slot-air", ease: "step", keys: [{ t: 0, v: 0 }, { t: 5, v: 1 }, { t: 155, v: 0 }] }] },
  { id: "cool-down", title: t("Cooling down after toasting", "Abkühlen nach dem Toasten"), description: t("Right after the pop: the toast is taken out and the casing cools for ten minutes.", "Gleich nach dem Auswerfen: Der Toast wird entnommen und das Gehäuse kühlt zehn Minuten ab."), durationS: 600, fps: 1, loop: false, rootMotion: "none",
    keys: [{ t: 0, pose: "popped", ease: "step" }, { t: 60, pose: "idle", ease: "step" }, { t: 600, pose: "idle", ease: "step" }],
    contacts: contactsFor(600),
    phases: [{ id: "popped", fromS: 0, toS: 60, title: t("Toast up", "Toast oben") }, { id: "emptied", fromS: 60, toS: 600, title: t("Empty, cooling", "Leer, kühlt ab") }],
    events: [{ t: 60, kind: "operating", title: t("Toast taken out", "Toast entnommen"), target: "elements" }],
    tracks: [{ target: "operating:elements", ease: "step", keys: [{ t: 0, v: 0 }, { t: 600, v: 0 }] }] },
];

export const toaster = {
  id: "toaster", category: "appliances",
  title: t("Two-Slice Toaster", "Zweischeiben-Toaster"),
  summary: t("A chromed two-slice toaster of 900 W, 0.30 m wide and 0.20 m high with bread: while toasting, 180 W warm the housing walls, 100 W the top plate, 150 W the crust of the bread and a 470 W stream of hot air rises out of the slots.", "Ein verchromter Zweischeiben-Toaster von 900 W, 0,30 m breit und 0,20 m hoch mit Brot: Beim Toasten erwärmen 180 W die Gehäusewände, 100 W die Deckplatte, 150 W die Brotkruste, und ein Strom heißer Luft von 470 W steigt aus den Schlitzen."),
  tags: ["toaster", "kitchen", "appliance", "internal-gain", "electric", "heat-source"],
  provenance: { author: "Semio staffage content (A4)", method: "handcrafted", created: "2026-11-01", references: refs,
    notes: { en: "Front view of a typical two-slice toaster with slots along the width (no single product): 0.30 m x 0.17 m x 0.188 m body, feet 0.012 m, bread slices 0.115 m x 0.115 m x 0.012 m above the plate when lowered, mass 1.3 kg including two 35 g slices; dimensions and rated input 900 W (class 800-1000 W, IEC 60335-2-9) are authored from the product class. The 900 W end up as 180 W in the housing walls, 100 W in the top plate, 150 W in the crust (browning, drying) and 470 W as hot slot air (0.0044 m3/s at 88 K rise: 470 W; moisture of the toasting bread 0.03 kg/kg); the split is an authored estimate.", de: "Vorderansicht eines typischen Zweischeiben-Toasters mit Schlitzen entlang der Breite (kein einzelnes Produkt): Gehäuse 0,30 m x 0,17 m x 0,188 m, Füße 0,012 m, Brotscheiben 0,115 m x 0,115 m, abgesenkt 0,012 m über der Platte, Masse 1,3 kg einschließlich zweier Scheiben zu je 35 g; Maße und Nennaufnahme 900 W (Klasse 800-1000 W, IEC 60335-2-9) sind aus der Produktklasse geschätzt. Die 900 W teilen sich auf in 180 W in den Gehäusewänden, 100 W in der Deckplatte, 150 W in der Kruste (Bräunung, Trocknung) und 470 W als heiße Schlitzluft (0,0044 m3/s bei 88 K Anstieg: 470 W; Feuchte des Toastguts 0,03 kg/kg); die Aufteilung ist ein Schätzwert." } },
  dimensions: { heightM: 0.2, widthM: 0.3, depthM: 0.17, massKg: 1.3 },
  materials,
  rig: { kind: "mechanical", root: "base", bones },
  anchors,
  representations: reps,
  symbols: [],
  poses, clips,
  thermal: {
    environment: { airTempC: 20, meanRadiantTempC: 20, relativeHumidity: 0.5, windMS: 0, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325, skyTempC: 20, groundTempC: 20 },
    operating: [keyElements], passengers: [], regions, emitters,
    assumptions: [
      t("The heating wires inside are not a region: their 900 W leave through the housing walls (180 W), the top plate (100 W), the bread crust (150 W) and the hot air through the slots (470 W). A room sees all of it as an internal gain.", "Die Heizdrähte im Inneren sind keine Region: Ihre 900 W verlassen das Gerät durch die Gehäusewände (180 W), die Deckplatte (100 W), die Brotkruste (150 W) und die heiße Luft aus den Schlitzen (470 W). Ein Raum erlebt alles davon als innere Last."),
      t("The bread is a thermal region that is part of the object while it sits in the toaster; the crust releases its drying moisture into the slot air.", "Das Brot ist eine thermische Region des Objekts, solange es im Toaster steckt; die Kruste gibt ihre Trocknungsfeuchte an die Schlitzluft ab."),
    ],
  },
  structure: { gravityMS2: 9.80665, dynamics: "quasi-static", supports: ["foot-left", "foot-right"], assumptions: [t("A rigid body standing on two rubber feet on a worktop.", "Ein starrer Körper auf zwei Gummifüßen auf einer Arbeitsplatte.")], clinical: false },
  presentation: { defaultPose: "toasting", defaultClip: "toast-cycle", defaultScale: 50, viewpoints: ["elevation"] },
  exports: ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"],
  completion: "authored",
};

export const toasterManifest = { id: "toaster", category: "appliances", title: toaster.title, heightM: 0.2, widthM: 0.3, poses: poses.map((p) => p.id), clips: clips.map((c) => c.id), representations: { 50: "authored", 100: "authored", 200: "authored", 500: "unavailable" }, symbols: [], analytics: ["thermal", "transport", "structural"], viewpoints: ["elevation"], exports: toaster.exports, distinction: { en: "A 900 W two-slice toaster, 0.30 m wide: in toasting pose the housing, the top plate and the bread crust heat up and hot air rises from the slots.", de: "Ein Zweischeiben-Toaster von 900 W, 0,30 m breit: In der Pose Toasten erwärmen sich Gehäuse, Deckplatte und Brotkruste, und heiße Luft steigt aus den Schlitzen." }, completion: "authored" };
