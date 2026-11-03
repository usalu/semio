import { ALL_EXPORTS, INCROPERA, ISO10456, authored, clip, level, pose, ref, tx, unavailable, wrap } from "./lib.mjs";
import { materialOf, space, sum } from "./space.mjs";
import { SRC, circlePts, exp, regionOf, rrectPts, ringPath, single } from "./geo.mjs";

const m = (kg, comX, comY, reference) => ({ kg, comX, comY, distribution: "point", source: SRC, ...(reference ? { reference } : {}) });
const fixed = { type: "fixed" };
const WIN = [0, 0.46];
const KNOB = [0.04, 0.14];

const sp = space([
  ["casing", null, 0, 0, { title: tx("Casing with heat exchanger, burner and hydraulics", "Gehäuse mit Wärmetauscher, Brenner und Hydraulik"), lengthM: 0.67, joint: fixed, mass: m(27.5, 0, 0.4) }],
  ["connections", "casing", 0, 0, { title: tx("Connection stubs below", "Anschlussstutzen unten"), lengthM: 0.052, joint: fixed, mass: m(1.3, 0, 0.03) }],
  ["collar", "casing", 0, 0.67, { title: tx("Concentric flue collar", "Konzentrischer Abgasstutzen"), lengthM: 0.05, joint: fixed, mass: m(0.9, 0, 0.025) }],
  ["mode-knob", "casing", KNOB[0], KNOB[1], { title: tx("Mode knob", "Betriebsartenknopf"), lengthM: 0.026, joint: { type: "revolute", minDeg: -135, maxDeg: 135 }, mass: m(0.05, 0, 0) }],
  ["flame-tall", "casing", WIN[0], 0.441, { title: tx("Outer flame at high output", "Äußere Flamme bei hoher Leistung"), lengthM: 0.03, joint: { type: "prismatic", axisDeg: 90, minM: -0.12, maxM: 0 } }],
  ["flame", "casing", WIN[0], 0.441, { title: tx("Inner flame", "Innere Flamme"), lengthM: 0.016, joint: { type: "prismatic", axisDeg: 90, minM: -0.12, maxM: 0 } }],
]);
const { poly, rect, ellipse, path } = sp;

const slits = (tone, role, z) => [-0.09, -0.03, 0.03, 0.09].map((x, i) => poly(`slit-${i}`, "casing", role, z, false, tone, [[x, 0.628], [x, 0.652]], false));
const tick = (id, deg, r0, r1) => {
  const a = (deg * Math.PI) / 180;
  const p = (r) => [KNOB[0] - r * Math.sin(a), KNOB[1] + r * Math.cos(a)];
  return poly(id, "casing", "detail", 3.3, false, 0.4, [p(r0), p(r1)], false);
};
const CASING = rrectPts(-0.22, 0.05, 0.22, 0.67, 0.014);
const casingWithWindow = (r) => ringPath(CASING, circlePts(WIN[0], WIN[1], r, 32));
const FLAME_LOW = [[-0.018, 0.441], [-0.018, 0.447], [-0.012, 0.455], [-0.007, 0.449], [0, 0.458], [0.007, 0.449], [0.012, 0.455], [0.018, 0.447], [0.018, 0.441]];
const FLAME_TALL = [[-0.024, 0.441], [-0.022, 0.452], [-0.015, 0.463], [-0.009, 0.452], [0, 0.473], [0.009, 0.452], [0.015, 0.463], [0.022, 0.452], [0.024, 0.441]];
const stubs = (z, w, withNuts) => [-0.16, -0.08, 0, 0.08, 0.16].flatMap((x, i) => [
  rect(`stub-${i}`, "connections", "body", z, true, 0.7, x - w / 2, 0, w, 0.052, { region: "connections", material: "brass" }),
  ...(withNuts ? [rect(`nut-${i}`, "connections", "body", z + 0.05, true, 0.45, x - 0.017, 0.018, 0.034, 0.02, { region: "connections", material: "brass" })] : []),
]);

const rep50 = authored(
  50,
  [20, 75],
  { body: 0.18, detail: 0.13, texture: null, glazing: 0.1, ground: 0.35 },
  [
    path("casing", "casing", "body", 3, true, 0.88, casingWithWindow(0.0365), { region: "casing", material: "painted-steel" }),
    ...slits(0.4, "detail", 3.05),
    poly("door", "casing", "detail", 3.06, false, 0.4, rrectPts(-0.205, 0.225, 0.205, 0.655, 0.012), true),
    poly("logo-1", "casing", "detail", 3.07, false, 0.4, [[-0.06, 0.59], [0.06, 0.59]], false),
    poly("logo-2", "casing", "detail", 3.07, false, 0.4, [[-0.04, 0.575], [0.04, 0.575]], false),
    ellipse("led", "casing", "detail", 3.07, true, 0.95, 0.17, 0.245, 0.006, 0.006),
    path("bezel", "casing", "body", 3.25, true, 0.55, ringPath(circlePts(WIN[0], WIN[1], 0.0445, 32), circlePts(WIN[0], WIN[1], 0.0365, 32)), { region: "sight-glass", material: "sight-glass" }),
    ellipse("chamber", "casing", "body", 2.5, true, 0.1, WIN[0], WIN[1], 0.0372, 0.0372),
    rect("burner-tube", "casing", "body", 2.6, true, 0.45, -0.026, 0.43, 0.052, 0.011, { region: "burner", material: "burner-mesh" }),
    poly("flame-tall", "flame-tall", "detail", 2.7, true, 0.75, FLAME_TALL, true, { region: "burner", material: "burner-mesh" }),
    poly("flame", "flame", "detail", 2.8, true, 0.95, FLAME_LOW, true, { region: "burner", material: "burner-mesh" }),
    rect("panel", "casing", "body", 3.1, true, 0.35, -0.205, 0.065, 0.41, 0.145, { material: "polycarbonate", r: 0.008 }),
    rect("display", "casing", "detail", 3.2, true, 0.95, -0.17, 0.125, 0.11, 0.06, { region: "controls", material: "polycarbonate", r: 0.004 }),
    poly("text-1", "casing", "detail", 3.25, false, 0.3, [[-0.155, 0.168], [-0.08, 0.168]], false),
    poly("text-2", "casing", "detail", 3.25, false, 0.3, [[-0.155, 0.145], [-0.11, 0.145]], false),
    tick("tick-off", 100, 0.034, 0.042),
    tick("tick-heat", 0, 0.034, 0.042),
    tick("tick-water", -90, 0.034, 0.042),
    ellipse("knob", "mode-knob", "body", 3.4, true, 0.72, KNOB[0], KNOB[1], 0.026, 0.026, { material: "polycarbonate" }),
    poly("pointer", "mode-knob", "detail", 3.5, false, 0.2, [[KNOB[0], KNOB[1]], [KNOB[0], KNOB[1] + 0.022]], false),
    ellipse("set-knob", "casing", "body", 3.4, true, 0.72, 0.12, 0.14, 0.018, 0.018, { material: "polycarbonate" }),
    poly("set-notch", "casing", "detail", 3.5, false, 0.2, [[0.12, 0.14], [0.12, 0.154]], false),
    ellipse("gauge", "casing", "detail", 3.4, true, 0.92, 0.173, 0.14, 0.016, 0.016, { material: "sight-glass" }),
    poly("gauge-needle", "casing", "detail", 3.5, false, 0.2, [[0.173, 0.14], [0.181, 0.149]], false),
    rect("collar-base", "collar", "body", 2.9, true, 0.7, -0.0625, 0.67, 0.125, 0.03, { region: "flue-collar", material: "flue-pipe", r: 0.004 }),
    rect("collar-pipe", "collar", "body", 2.9, true, 0.55, -0.04, 0.7, 0.08, 0.02, { region: "flue-collar", material: "flue-pipe" }),
    ...stubs(2.9, 0.022, true),
  ],
  {
    silhouette: ["A 0.44 m by 0.72 m front elevation, 8.8 mm by 14.4 mm on paper: a white rounded casing with a round burner sight glass in its upper half, a dark control panel with display and three dials in its lower part, a concentric flue collar on top and five connection stubs below.", "Eine Vorderansicht von 0,44 m mal 0,72 m, auf Papier 8,8 mm mal 14,4 mm: ein weißes Gehäuse mit gerundeten Ecken und rundem Brennerschauglas in der oberen Hälfte, ein dunkles Bedienfeld mit Display und drei Rundinstrumenten im unteren Teil, oben ein konzentrischer Abgasstutzen und unten fünf Anschlussstutzen."],
    retained: ["Casing with four vent slits, the door line, a logo bar and a status LED, sight glass with bezel and the burner behind it (tube, inner and outer flame), control panel with display, two text lines, mode knob with three marked positions, set-point knob and pressure gauge with needle, flue collar with pipe, and five stubs with union nuts.", "Gehäuse mit vier Lüftungsschlitzen, Türlinie, Logobalken und Statusleuchte, Schauglas mit Blende und dem Brenner dahinter (Rohr, innere und äußere Flamme), Bedienfeld mit Display, zwei Textzeilen, Betriebsartenknopf mit drei markierten Stellungen, Sollwertknopf und Manometer mit Zeiger, Abgasstutzen mit Rohr und fünf Stutzen mit Überwurfmuttern."],
    merged: ["The wall bracket, the door seam and the casing fixings are part of the casing shape; stubs and nuts form one silhouette per connection.", "Wandhalter, Türfuge und Gehäusebefestigungen gehören zur Gehäuseform; Stutzen und Muttern bilden je Anschluss eine Silhouette."],
    omitted: ["The heat exchanger, fan, pump and gas valve behind the casing, the labels and the condensate trap.", "Wärmetauscher, Gebläse, Pumpe und Gasarmatur hinter dem Gehäuse, die Beschriftungen und der Kondensatsiphon."],
    thinElements: ["The slits, text lines, ticks, pointer, notch and needle are 0.13 mm detail strokes; the stubs are 22 mm wide (0.44 mm on paper).", "Schlitze, Textzeilen, Marken, Zeiger, Kerbe und Nadel sind 0,13-mm-Detailstriche; die Stutzen sind 22 mm breit (0,44 mm auf Papier)."],
    hatching: ["None; the burner chamber is a dark tone behind the window cut-out in the casing.", "Keine; die Brennkammer ist ein dunkler Ton hinter dem Fensterausschnitt im Gehäuse."],
    styles: ["Casing 0.88, panel 0.35, display 0.95, knobs 0.72, bezel 0.55, chamber 0.1, burner tube 0.45, outer flame 0.75, inner flame 0.95, collar 0.55 to 0.7, stubs 0.7 with nuts 0.45; body pen 0.18 mm, detail 0.13 mm, glazing 0.1 mm.", "Gehäuse 0,88, Bedienfeld 0,35, Display 0,95, Knöpfe 0,72, Blende 0,55, Kammer 0,1, Brennerrohr 0,45, äußere Flamme 0,75, innere Flamme 0,95, Stutzen 0,55 bis 0,7, Stutzen 0,7 mit Muttern 0,45; Körperstift 0,18 mm, Detail 0,13 mm, Verglasung 0,1 mm."],
    overlays: ["Casing, controls, sight glass, burner, flue collar and connections are separate regions: in the thermal view the casing is slightly warm, the flue collar and stubs warmer, and the flame behind the glass the hottest; at standby and off nothing glows.", "Gehäuse, Regelung, Schauglas, Brenner, Abgasstutzen und Anschlüsse sind getrennte Regionen: in der Thermalansicht ist das Gehäuse leicht warm, Abgasstutzen und Anschlüsse wärmer und die Flamme hinter dem Glas am heißesten; im Aus-Zustand glüht nichts."],
  },
);

const rep100 = authored(
  100,
  [75, 150],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    path("casing", "casing", "body", 3, true, 0.88, casingWithWindow(0.039), { region: "casing", material: "painted-steel" }),
    ellipse("chamber", "casing", "body", 2.5, true, 0.1, WIN[0], WIN[1], 0.04, 0.04),
    poly("flame-tall", "flame-tall", "body", 2.7, true, 0.75, FLAME_TALL.map(([x, y]) => [x * 1.1, 0.431 + (y - 0.431) * 1.15]), true, { regions: ["burner", "sight-glass"], material: "burner-mesh" }),
    poly("flame", "flame", "body", 2.8, true, 0.95, FLAME_LOW, true, { regions: ["burner", "sight-glass"], material: "burner-mesh" }),
    rect("panel", "casing", "body", 3.1, true, 0.35, -0.205, 0.065, 0.41, 0.145, { region: "controls", material: "polycarbonate" }),
    ellipse("knob", "mode-knob", "body", 3.4, true, 0.72, KNOB[0], KNOB[1], 0.028, 0.028, { material: "polycarbonate" }),
    rect("collar-base", "collar", "body", 2.9, true, 0.7, -0.0625, 0.67, 0.125, 0.03, { region: "flue-collar", material: "flue-pipe" }),
    rect("collar-pipe", "collar", "body", 2.9, true, 0.55, -0.04, 0.7, 0.08, 0.02, { region: "flue-collar", material: "flue-pipe" }),
    ...stubs(2.9, 0.026, false),
  ],
  {
    silhouette: ["A 4.4 mm by 7.2 mm front elevation: a rounded white box with a round window, a dark band across its lower part, a double-stepped collar on top and five short stubs below.", "Eine Vorderansicht von 4,4 mm mal 7,2 mm: ein weißer Kasten mit gerundeten Ecken und rundem Fenster, ein dunkles Band quer im unteren Teil, oben ein zweifach abgesetzter Stutzen und unten fünf kurze Stutzen."],
    retained: ["The casing with the burner window and its two flames, the dark control band with the mode knob, the flue collar with its pipe and the five stubs.", "Das Gehäuse mit dem Brennerfenster und seinen zwei Flammen, das dunkle Bedienband mit dem Betriebsartenknopf, der Abgasstutzen mit seinem Rohr und die fünf Stutzen."],
    merged: ["Bezel, burner tube and flames merge into the flames (regions burner and sight glass); the panel carries the display (region controls) without a separate display shape.", "Blende, Brennerrohr und Flammen gehen in den Flammen auf (Regionen Brenner und Schauglas); das Bedienfeld trägt das Display (Region Regelung) ohne eigene Displayform."],
    omitted: ["Vent slits, text lines, ticks, pointer, set-point knob, gauge, needle and the union nuts.", "Lüftungsschlitze, Textzeilen, Marken, Zeiger, Sollwertknopf, Manometer, Nadel und die Überwurfmuttern."],
    thinElements: ["The stubs are thickened from 22 to 26 mm (0.26 mm on paper).", "Die Stutzen sind von 22 auf 26 mm (0,26 mm auf Papier) verdickt."],
    hatching: ["None.", "Keine."],
    styles: ["Casing 0.88, chamber 0.1, flames 0.75 and 0.95, panel 0.35, knob 0.72, collar 0.55 to 0.7, stubs 0.7; one body pen of 0.18 mm.", "Gehäuse 0,88, Kammer 0,1, Flammen 0,75 und 0,95, Bedienfeld 0,35, Knopf 0,72, Stutzen 0,55 bis 0,7, Stutzen 0,7; ein Körperstift von 0,18 mm."],
    overlays: ["The flames list burner and sight glass as merged regions, so the flame heat still shows; the panel lists the controls.", "Die Flammen führen Brenner und Schauglas als zusammengeführte Regionen, sodass die Flammenwärme sichtbar bleibt; das Bedienfeld führt die Regelung."],
  },
);

const rep200 = authored(
  200,
  [150, 350],
  { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.2 },
  [
    rect("casing", "casing", "body", 3, true, 0.88, -0.22, 0.05, 0.44, 0.62, { regions: ["casing", "sight-glass", "burner"], material: "painted-steel", r: 0.014 }),
    rect("panel", "casing", "body", 3.1, true, 0.35, -0.2, 0.065, 0.4, 0.145, { region: "controls", material: "polycarbonate" }),
    rect("collar", "collar", "body", 2.9, true, 0.6, -0.0625, 0.67, 0.125, 0.05, { region: "flue-collar", material: "flue-pipe" }),
    rect("stubs", "connections", "body", 2.9, true, 0.6, -0.17, 0.0, 0.34, 0.05, { region: "connections", material: "brass" }),
  ],
  {
    silhouette: ["A 2.2 mm by 3.6 mm mark: a white block with a dark band across its lower third, a short collar on top and a bar below.", "Ein Zeichen von 2,2 mm mal 3,6 mm: ein weißer Block mit einem dunklen Band im unteren Drittel, oben ein kurzer Stutzen und unten ein Balken."],
    retained: ["The 0.44 m by 0.62 m casing, the 0.4 m control band, the 0.125 m flue collar and the connection row of 0.34 m.", "Das Gehäuse von 0,44 m mal 0,62 m, das 0,4 m breite Bedienband, der 0,125 m breite Abgasstutzen und die Anschlussreihe von 0,34 m."],
    merged: ["Casing, sight glass and burner share one block (three regions merged); the five stubs form one bar.", "Gehäuse, Schauglas und Brenner teilen einen Block (drei Regionen zusammengeführt); die fünf Stutzen bilden einen Balken."],
    omitted: ["Window, flames, knobs, display, slits, nuts and the collar pipe.", "Fenster, Flammen, Knöpfe, Display, Schlitze, Muttern und das Stutzenrohr."],
    thinElements: ["The stub row is 50 mm high (0.25 mm on paper) and replaces five stubs of 22 mm.", "Die Anschlussreihe ist 50 mm hoch (0,25 mm auf Papier) und ersetzt fünf Stutzen von 22 mm."],
    hatching: ["None.", "Keine."],
    styles: ["Casing 0.88, band 0.35, collar and bar 0.6; one body pen of 0.13 mm.", "Gehäuse 0,88, Band 0,35, Stutzen und Balken 0,6; ein Körperstift von 0,13 mm."],
    overlays: ["The block lists casing, sight glass and burner as merged regions; band, collar and bar stay separate.", "Der Block führt Gehäuse, Schauglas und Brenner als zusammengeführte Regionen; Band, Stutzen und Balken bleiben getrennt."],
  },
);

const rep500 = unavailable(
  500,
  ["At 1:500 the 0.44 m by 0.72 m boiler is 0.9 mm by 1.4 mm and its 0.125 m flue collar 0.25 mm wide: the control band and connections cannot be told apart below every print profile.", "Bei 1:500 ist der 0,44 m mal 0,72 m große Kessel 0,9 mm mal 1,4 mm groß und sein 0,125 m breiter Abgasstutzen 0,25 mm breit: Bedienband und Anschlüsse sind unter jedem Druckprofil nicht mehr zu unterscheiden."],
  ["Flue collar of 0.125 m", "Abgasstutzen von 0,125 m"],
  0.125,
  [{ kind: "scale", denominator: 200 }, { kind: "scale", denominator: 100 }],
);

const regions = [
  regionOf("casing", ["Casing", "Gehäuse"], "surface", "operating", "painted-steel", 0.9, 0.7, "vertical-plate", [single("sheet", "Painted steel sheet 0.8 mm", "Lackiertes Stahlblech 0,8 mm", "metal", "painted-steel", 0.0008, 0.000016, 3700, "incropera-2018")], 3700, { operating: { key: "burner", heatW: 55 } }, "incropera-2018"),
  regionOf("controls", ["Control panel with display", "Bedienfeld mit Display"], "surface", "operating", "polycarbonate", 0.03, 0.2, "vertical-plate", [single("panel", "Polycarbonate panel 2 mm", "Polycarbonatfeld 2 mm", "polymer", "polycarbonate", 0.002, 0.01, 2400, "incropera-2018")], 2400, { operating: { key: "controls", heatW: 3 } }, "incropera-2018"),
  regionOf("sight-glass", ["Sight glass bezel", "Schauglasblende"], "glazing", "operating", "sight-glass", 0.005, 0.08, "vertical-plate", [single("glass", "Borosilicate glass 4 mm", "Borosilikatglas 4 mm", "glass", "sight-glass", 0.004, 0.004, 3300, "incropera-2018")], 3300, { operating: { key: "burner", heatW: 2 } }, "incropera-2018"),
  regionOf("burner", ["Burner with flame", "Brenner mit Flamme"], "heater", "operating", "burner-mesh", 0.01, 0.05, "horizontal-plate-up", [single("mesh", "Stainless steel mesh 1 mm", "Edelstahlgewebe 1 mm", "metal", "burner-mesh", 0.001, 0.00007, 800, "incropera-2018")], 800, { operating: { key: "burner", heatW: 3 } }, "incropera-2018"),
  regionOf("flue-collar", ["Concentric flue collar", "Konzentrischer Abgasstutzen"], "exhaust", "operating", "flue-pipe", 0.05, 0.07, "cylinder", [single("pipe", "Polypropylene flue pipe 3 mm", "Abgasrohr aus Polypropylen 3 mm", "polymer", "flue-pipe", 0.003, 0.014, 2700, "incropera-2018")], 2700, { operating: { key: "burner", heatW: 10 } }, "incropera-2018"),
  regionOf("connections", ["Connection stubs", "Anschlussstutzen"], "surface", "operating", "brass", 0.04, 0.05, "cylinder", [single("tube", "Brass tube 1.5 mm", "Messingrohr 1,5 mm", "metal", "brass", 0.0015, 0.000013, 1300, "incropera-2018")], 1300, { operating: { key: "burner", heatW: 12 } }, "incropera-2018"),
];

const bind = (flue) => ({
  regions: {
    casing: exp(0.85, { fraction: 0.15, tempC: 21 }),
    controls: exp(1),
    "sight-glass": exp(1),
    burner: exp(1),
    "flue-collar": exp(0.9, { fraction: 0.1, tempC: 21 }),
    connections: exp(1),
  },
  emitters: { "flue-gas": flue },
});
const withOp = (b, burner, controls) => ({ ...b, operating: { burner, controls } });

const wall = ["wall-bracket"];
const knob = (deg) => ({ "mode-knob": { angleDeg: deg } });
const poses = [
  pose("off", ["Off", "Aus"], ["The boiler is switched off: no flame, the display is dark and the casing is at room temperature.", "Der Kessel ist ausgeschaltet: keine Flamme, das Display ist dunkel und das Gehäuse hat Raumtemperatur."], { ...knob(100), flame: { dy: -0.12 }, "flame-tall": { dy: -0.12 } }, wall, "authored", withOp(bind(0), 0, 0)),
  pose("heating", ["Space heating", "Heizbetrieb"], ["The mode knob stands on heating and the burner is modulated to 14 kW: a small flame burns behind the sight glass, the casing releases 39 W and the flue gas leaves at the top.", "Der Betriebsartenknopf steht auf Heizen und der Brenner ist auf 14 kW moduliert: hinter dem Schauglas brennt eine kleine Flamme, das Gehäuse gibt 39 W ab und das Abgas tritt oben aus."], { ...knob(0), flame: { dy: 0 }, "flame-tall": { dy: -0.12 } }, wall, "authored", withOp(bind(1), 0.7, 1)),
  pose("hot-water", ["Hot water", "Warmwasserbereitung"], ["The mode knob stands on hot water and the burner runs at its full 20 kW: a tall flame burns behind the sight glass, the casing releases 55 W and the warm flue gas rises at the top.", "Der Betriebsartenknopf steht auf Warmwasser und der Brenner läuft mit seinen vollen 20 kW: hinter dem Schauglas brennt eine große Flamme, das Gehäuse gibt 55 W ab und das warme Abgas steigt oben auf."], { ...knob(-90), flame: { dy: 0 }, "flame-tall": { dy: 0 } }, wall, "authored", withOp(bind(1), 1, 1)),
];

const contacts = (d) => wall.map((anchor) => ({ anchor, fromS: 0, toS: d }));

const clips = [
  clip(
    "heating-cycle",
    ["Burner start and heating cycle", "Brennerstart und Heizzyklus"],
    ["The boiler is switched to heating after five seconds: the burner lights at 14 kW, the casing warms and the flue gas leaves; after ten minutes the burner switches off and the boiler cools down.", "Der Kessel wird nach fünf Sekunden auf Heizen geschaltet: der Brenner zündet mit 14 kW, das Gehäuse erwärmt sich und das Abgas tritt aus; nach zehn Minuten schaltet der Brenner ab und der Kessel kühlt aus."],
    900,
    1,
    [{ t: 0, pose: "off", ease: "step" }, { t: 5, pose: "heating", ease: "step" }, { t: 605, pose: "off", ease: "step" }, { t: 900, pose: "off", ease: "step" }],
    contacts(900),
    [["off", 0, 5, ["Off", "Aus"]], ["burn", 5, 605, ["Heating", "Heizbetrieb"]], ["cool-down", 605, 900, ["Cooling down", "Auskühlen"]]],
    [[5, "ignition", ["Burner lights", "Brenner zündet"], "burner"], [605, "shutdown", ["Burner off", "Brenner aus"], "burner"]],
    [["operating:burner", "step", [[0, 0], [5, 0.7], [605, 0]]], ["operating:controls", "step", [[0, 0], [5, 1], [605, 0]]], ["emitter:flue-gas", "step", [[0, 0], [5, 1], [605, 0]]]],
  ),
  clip(
    "hot-water-priority",
    ["Hot-water priority", "Warmwasservorrang"],
    ["While the boiler heats at 14 kW, a hot-water demand switches it to its full 20 kW after half a minute; after five more minutes it returns to heating and after seven minutes it switches off.", "Während der Kessel mit 14 kW heizt, schaltet ihn ein Warmwasserbedarf nach einer halben Minute auf seine vollen 20 kW; nach fünf weiteren Minuten kehrt er zum Heizen zurück und schaltet nach sieben Minuten aus."],
    480,
    1,
    [{ t: 0, pose: "heating", ease: "step" }, { t: 30, pose: "hot-water", ease: "step" }, { t: 330, pose: "heating", ease: "step" }, { t: 420, pose: "off", ease: "step" }, { t: 480, pose: "off", ease: "step" }],
    contacts(480),
    [["heating", 0, 30, ["Heating", "Heizbetrieb"]], ["hot-water", 30, 330, ["Hot water at full output", "Warmwasser mit voller Leistung"]], ["heating-again", 330, 420, ["Heating again", "Wieder Heizbetrieb"]], ["off", 420, 480, ["Off", "Aus"]]],
    [[30, "operating", ["Burner to full output", "Brenner auf volle Leistung"], "burner"], [420, "shutdown", ["Burner off", "Brenner aus"], "burner"]],
    [["operating:burner", "step", [[0, 0.7], [30, 1], [330, 0.7], [420, 0]]], ["operating:controls", "step", [[0, 1], [420, 0]]], ["emitter:flue-gas", "step", [[0, 1], [420, 0]]]],
  ),
];

export const boiler = () => {
  const massKg = sum(sp.bones.map((b) => b.mass?.kg ?? 0));
  return wrap({
    id: "gas-boiler",
    category: "appliances",
    title: tx("Wall-Hung Condensing Gas Boiler", "Wandhängender Gas-Brennwertkessel"),
    summary: tx(
      "A wall-hung condensing gas boiler of 20 kW in front elevation: white casing with a burner sight glass, control panel with display and mode knob, concentric flue collar on top and five connections below; the casing releases 55 to 85 W, the flue gas leaves at 38 K above the room.",
      "Ein wandhängender Gas-Brennwertkessel von 20 kW in der Vorderansicht: weißes Gehäuse mit Brenner-Schauglas, Bedienfeld mit Display und Betriebsartenknopf, oben ein konzentrischer Abgasstutzen und unten fünf Anschlüsse; das Gehäuse gibt 55 bis 85 W ab, das Abgas tritt mit 38 K über der Raumtemperatur aus.",
    ),
    tags: ["gas-boiler", "condensing-boiler", "boiler", "heating", "hydronic", "wall-hung", "burner", "flue", "appliance", "heating-plant"],
    provenance: {
      author: "Semio staffage content (A9)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [
        ref("en-15502-1-2015", "EN 15502-1:2012+A1:2015 Gas-fired heating boilers - Part 1: General requirements and tests (casing temperature, flue gas, modulation range of wall-hung condensing boilers)."),
        ref("turns-2012", "Turns, S. R. (2012). An Introduction to Combustion: Concepts and Applications, 3rd ed., McGraw-Hill (stoichiometric air, flue gas amount and water vapour of natural-gas combustion)."),
        INCROPERA,
        ISO10456,
      ],
      notes: tx(
        "Front elevation of a wall-hung condensing boiler of the 20 kW class (product family of EN 15502-1): casing 0.44 m wide and 0.62 m high, 0.35 m deep, a concentric flue collar of 125 mm on top reaching a total height of 0.72 m, five connection stubs of 0.052 m below (flow, hot water out, gas, cold water in, return) and a mass of about 30 kg are rounded from this product class (authored estimate). The origin is the lowest point of the stubs at the casing centre; the boiler hangs by a wall bracket at the upper edge. The burner takes 20 kW at full output (hot water) and 14 kW when modulated for space heating; this fuel heat does not appear in the room but in the heating water and in the flue gas. Only the losses of the surfaces are released as heat in the room: casing 55 W, flue collar 10 W, connections 12 W, burner window 3 W, glass bezel 2 W and display 3 W, together about 85 W at full output (authored estimate, 0.4 % of the input). The flue gas of a condensing boiler leaves at about 59 degrees C (38 K above the room): 0.0072 m3/s at 20 kW with a water vapour excess of 0.07 kg/kg after condensation, estimated from the natural-gas combustion with excess air 1.2 (Turns). The flame behind the sight glass is drawn by hand: a small inner flame in the heating state and an additional outer flame at full output.",
        "Vorderansicht eines wandhängenden Brennwertkessels der 20-kW-Klasse (Produktfamilie nach EN 15502-1): Gehäuse 0,44 m breit und 0,62 m hoch, 0,35 m tief, oben ein konzentrischer Abgasstutzen von 125 mm bis zu einer Gesamthöhe von 0,72 m, unten fünf Anschlussstutzen von 0,052 m (Vorlauf, Warmwasser aus, Gas, Kaltwasser ein, Rücklauf) und eine Masse von etwa 30 kg sind aus dieser Produktklasse gerundet (Schätzwert). Der Ursprung ist der tiefste Punkt der Stutzen in der Gehäusemitte; der Kessel hängt an einer Wandhalterung am oberen Rand. Der Brenner nimmt 20 kW bei voller Leistung (Warmwasser) und 14 kW im modulierten Heizbetrieb auf; diese Brennstoffwärme erscheint nicht im Raum, sondern im Heizwasser und im Abgas. Nur die Verluste der Oberflächen werden als Wärme im Raum frei: Gehäuse 55 W, Abgasstutzen 10 W, Anschlüsse 12 W, Brennerfenster 3 W, Glasblende 2 W und Display 3 W, zusammen etwa 85 W bei voller Leistung (Schätzwert, 0,4 % der Zufuhr). Das Abgas eines Brennwertkessels tritt mit etwa 59 Grad C (38 K über dem Raum) aus: 0,0072 m3/s bei 20 kW mit einem Wasserdampf-Überschuss von 0,07 kg/kg nach der Kondensation, geschätzt aus der Erdgasverbrennung mit Luftüberschuss 1,2 (Turns). Die Flamme hinter dem Schauglas ist von Hand gezeichnet: eine kleine innere Flamme im Heizbetrieb und zusätzlich eine äußere Flamme bei voller Leistung.",
      ),
    },
    dimensions: { heightM: 0.72, widthM: 0.44, depthM: 0.35, massKg },
    materials: [
      materialOf("painted-steel", "White painted steel sheet", "Weiß lackiertes Stahlblech", 0.9, 0.3, 7850, 50, 460, "literature", "incropera-2018"),
      materialOf("polycarbonate", "Polycarbonate panel", "Polycarbonatfeld", 0.9, 0.3, 1200, 0.2, 1200, SRC, "incropera-2018"),
      materialOf("sight-glass", "Borosilicate sight glass", "Borosilikat-Schauglas", 0.9, 0.1, 2230, 1.1, 830, "literature", "incropera-2018"),
      materialOf("burner-mesh", "Stainless steel burner mesh", "Edelstahl-Brennergewebe", 0.5, 0.5, 7900, 15, 500, SRC, "incropera-2018"),
      materialOf("flue-pipe", "Polypropylene flue pipe", "Abgasrohr aus Polypropylen", 0.9, 0.5, 900, 0.22, 1900, SRC, "incropera-2018"),
      materialOf("brass", "Brass connection stub", "Anschlussstutzen aus Messing", 0.5, 0.5, 8500, 120, 380, "literature", "incropera-2018"),
    ],
    rig: { kind: "mechanical", root: "casing", bones: sp.bones.map((b) => ({ ...b, ...(b.mass ? { mass: { ...b.mass } } : {}) })) },
    anchors: [
      { id: "wall-bracket", bone: "casing", x: 0, y: 0.64, kind: "support", title: tx("Wall bracket at the upper edge", "Wandhalter am oberen Rand"), support: "pin", normalDeg: 90, footprintLengthM: 0.2, footprintWidthM: 0.02 },
      { id: "flue-outlet", bone: "collar", x: 0, y: 0.05, kind: "emitter", title: tx("Flue gas outlet on top of the collar", "Abgasaustritt oben am Stutzen") },
    ],
    representations: { 500: rep500, 200: rep200, 100: rep100, 50: rep50 },
    symbols: [],
    poses,
    clips,
    thermal: {
      environment: { airTempC: 21, meanRadiantTempC: 21, relativeHumidity: 0.5, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325 },
      passengers: [],
      operating: [
        { id: "burner", title: tx("Burner output", "Brennerleistung"), levels: [level(0, "Off", "Aus"), level(0.7, "Space heating (14 kW)", "Heizbetrieb (14 kW)"), level(1, "Hot water (20 kW)", "Warmwasser (20 kW)")] },
        { id: "controls", title: tx("Controls and display", "Regelung und Display"), levels: [level(0, "Off", "Aus"), level(1, "On (3 W)", "An (3 W)")] },
      ],
      regions,
      emitters: [{ id: "flue-gas", kind: "flue", title: tx("Flue gas leaving at the top of the collar", "Abgas am oberen Ende des Stutzens"), anchor: "flue-outlet", directionDeg: 90, diameterM: 0.06, operatingKey: "burner", volumeFlowM3S: 0.0072, temperatureRiseK: 38, vapourKgPerKg: 0.07, source: SRC, reference: "turns-2012" }],
      assumptions: [
        tx(
          "Only the surface losses (about 85 W at full output) heat the room; the 20 kW of fuel go into the heating water and the flue gas. The flue gas is shown at the collar although a room-sealed boiler leads it outside: switch the emitter off with strength 0 where the flue is not part of the picture.",
          "Nur die Oberflächenverluste (etwa 85 W bei voller Leistung) heizen den Raum; die 20 kW Brennstoff gehen in das Heizwasser und das Abgas. Das Abgas ist am Stutzen gezeigt, obwohl ein raumluftunabhängiger Kessel es nach außen führt: wo das Abgas nicht zum Bild gehört, den Emitter mit Stärke 0 abschalten.",
        ),
        tx(
          "The flame is a drawing detail behind the sight glass: it follows the burner level through the two flame bones (inner flame at heating, outer flame in addition at full output).",
          "Die Flamme ist ein Zeichnungsdetail hinter dem Schauglas: sie folgt der Brennerstufe über die beiden Flammenknochen (innere Flamme im Heizbetrieb, zusätzlich äußere Flamme bei voller Leistung).",
        ),
      ],
    },
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: wall,
      assumptions: [
        tx(
          "The 30 kg boiler hangs on one wall bracket above its centre of mass (pin): the bracket carries the full weight; the lateral stability of the real boiler is given by the lower fixings, which are out of the plane of this view.",
          "Der 30 kg schwere Kessel hängt an einem Wandhalter über seinem Schwerpunkt (Festlager): der Halter trägt das ganze Gewicht; die seitliche Stabilität des echten Kessels geben die unteren Befestigungen, die außerhalb der Ebene dieser Ansicht liegen.",
        ),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "hot-water", defaultClip: "heating-cycle", defaultScale: 50, viewpoints: ["elevation"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
};
