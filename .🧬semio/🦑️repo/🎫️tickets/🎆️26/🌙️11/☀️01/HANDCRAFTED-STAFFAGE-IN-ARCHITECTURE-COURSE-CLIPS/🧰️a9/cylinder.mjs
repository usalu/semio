import { ALL_EXPORTS, INCROPERA, ISO10456, authored, clip, level, pose, ref, tx, unavailable, wrap } from "./lib.mjs";
import { materialOf, space, sum } from "./space.mjs";
import { SRC, arcPts, circlePts, exp, regionOf, single } from "./geo.mjs";

const m = (kg, comX, comY, reference) => ({ kg, comX, comY, distribution: "point", source: SRC, ...(reference ? { reference } : {}) });
const fixed = { type: "fixed" };
const DIAL = [0, 0.96];
const BANDS = { "zone-4": [0.06, 0.42], "zone-3": [0.42, 0.78], "zone-2": [0.78, 1.14], "zone-1": [1.14, 1.5] };

const sp = space([
  ["base", null, 0, 0, { title: tx("Feet and base ring", "Füße und Bodenring"), lengthM: 0.06, joint: fixed, mass: m(13, 0, 0.03) }],
  ["zone-4", "base", 0, 0.06, { title: tx("Lower quarter of the tank", "Unteres Viertel des Speichers"), lengthM: 0.36, joint: fixed, mass: m(62, 0, 0.18) }],
  ["zone-3", "zone-4", 0, 0.42, { title: tx("Second quarter from below", "Zweites Viertel von unten"), lengthM: 0.36, joint: fixed, mass: m(62, 0, 0.18) }],
  ["zone-2", "zone-3", 0, 0.78, { title: tx("Second quarter from above", "Zweites Viertel von oben"), lengthM: 0.36, joint: fixed, mass: m(62, 0, 0.18) }],
  ["zone-1", "zone-2", 0, 1.14, { title: tx("Upper quarter of the tank", "Oberes Viertel des Speichers"), lengthM: 0.36, joint: fixed, mass: m(62, 0, 0.18) }],
  ["cap", "zone-1", 0, 1.5, { title: tx("Domed top cover", "Gewölbte Abdeckung oben"), lengthM: 0.1, joint: fixed, mass: m(9, 0, 0.04) }],
  ["connections", "base", 0, 0, { title: tx("Connection stubs", "Anschlussstutzen"), lengthM: 1.6, joint: fixed, mass: m(5, 0, 0.8) }],
  ["needle", "zone-2", DIAL[0], DIAL[1], { title: tx("Thermometer needle", "Thermometernadel"), lengthM: 0.04, joint: { type: "revolute", minDeg: -85, maxDeg: 85 } }],
]);
const { poly, rect, ellipse } = sp;

const bandRect = (id, z, tone, region) => rect(id, id, "body", z, true, tone, -0.27, BANDS[id][0], 0.54, BANDS[id][1] - BANDS[id][0], { region, material: "foam-jacket" });
const DOME = [...arcPts(0, 1.5, 0.27, 0.1, 0, Math.PI, 24)];
const sideStubs = [[-1, 1.3], [-1, 0.85], [-1, 0.2], [1, 1.0], [1, 0.55]];
const stubs = (w, nuts, z = 3.2) => [
  ...sideStubs.flatMap(([s, y], i) => [
    rect(`stub-${i}`, "connections", "body", z, true, 0.7, s < 0 ? -0.27 - w : 0.27, y - 0.012, w, 0.024, { region: "connections", material: "brass" }),
    ...(nuts ? [rect(`nut-${i}`, "connections", "body", z + 0.05, true, 0.45, s < 0 ? -0.27 - w : 0.27 + w - 0.012, y - 0.017, 0.012, 0.034, { region: "connections", material: "brass" })] : []),
  ]),
  rect("hot-out", "connections", "body", z, true, 0.7, 0.17, 1.55, 0.03, 0.048, { region: "connections", material: "brass" }),
];
const dialTick = (id, deg) => {
  const a = (deg * Math.PI) / 180;
  const p = (r) => [DIAL[0] - r * Math.sin(a), DIAL[1] + r * Math.cos(a)];
  return poly(id, "zone-2", "detail", 3.3, false, 0.4, [p(0.034), p(0.042)], false);
};

const rep50 = authored(
  50,
  [20, 75],
  { body: 0.18, detail: 0.13, texture: null, glazing: 0.1, ground: 0.35 },
  [
    rect("foot-l", "base", "body", 2.9, true, 0.35, -0.22, 0, 0.07, 0.06, { material: "powder-steel" }),
    rect("foot-r", "base", "body", 2.9, true, 0.35, 0.15, 0, 0.07, 0.06, { material: "powder-steel" }),
    rect("base-ring", "base", "body", 2.95, true, 0.4, -0.25, 0.04, 0.5, 0.025, { material: "powder-steel" }),
    ...["zone-4", "zone-3", "zone-2", "zone-1"].map((id, i) => bandRect(id, 3 + i * 0.01, 0.9 - i * 0.02, id)),
    poly("cap", "cap", "body", 3.1, true, 0.78, DOME, true, { region: "top-cap", material: "cap-plastic" }),
    poly("cap-seam", "cap", "detail", 3.15, false, 0.4, arcPts(0, 1.5, 0.27, 0.1, 0.6, 2.54, 12), false),
    ...stubs(0.03, true),
    poly("dial-bezel", "zone-2", "detail", 3.3, true, 0.95, circlePts(DIAL[0], DIAL[1], 0.05, 28), true, { material: "window-glass" }),
    dialTick("tick-cold", 70),
    dialTick("tick-mid", 0),
    dialTick("tick-hot", -70),
    poly("needle", "needle", "detail", 3.5, false, 0.2, [[DIAL[0], DIAL[1]], [DIAL[0], DIAL[1] + 0.036]], false),
    poly("label", "zone-3", "detail", 3.3, false, 0.4, [[-0.1, 0.62], [0.1, 0.62], [0.1, 0.52], [-0.1, 0.52]], true),
    poly("label-line", "zone-3", "detail", 3.31, false, 0.4, [[-0.07, 0.58], [0.07, 0.58]], false),
    poly("zip", "zone-3", "detail", 3.3, false, 0.4, [[0.22, 0.45], [0.22, 0.75]], false),
  ],
  {
    silhouette: ["A 0.6 m by 1.6 m front elevation, 12 mm by 32 mm on paper: a tall insulated cylinder on two feet with a domed cover, four stacked jacket shells, five side stubs and one stub on the dome, and a round thermometer in the upper half.", "Eine Vorderansicht von 0,6 m mal 1,6 m, auf Papier 12 mm mal 32 mm: ein hoher gedämmter Zylinder auf zwei Füßen mit gewölbter Abdeckung, vier übereinanderliegende Mantelschalen, fünf seitliche Stutzen und ein Stutzen auf der Wölbung und ein rundes Thermometer in der oberen Hälfte."],
    retained: ["Both feet, the base ring, the four jacket shells with their seams (the thermal quarters), the domed cover with its seam, five side stubs with union nuts, the hot-water stub on the dome, the thermometer with three ticks and its needle, and the label with the zip seam.", "Beide Füße, der Bodenring, die vier Mantelschalen mit ihren Fugen (die Wärmeviertel), die gewölbte Abdeckung mit ihrer Fuge, fünf seitliche Stutzen mit Überwurfmuttern, der Warmwasserstutzen auf der Wölbung, das Thermometer mit drei Marken und seiner Nadel und das Schild mit der Reißverschlussnaht."],
    merged: ["The steel tank, the coil and the foam jacket are one shell per quarter; the third foot behind is hidden.", "Stahlbehälter, Rohrwendel und Schaumummantelung sind eine Schale je Viertel; der dritte Fuß dahinter ist verdeckt."],
    omitted: ["The tank, coil and anode inside, the sensor pockets, the drain and the safety valve.", "Behälter, Rohrwendel und Anode im Inneren, die Fühlertaschen, die Entleerung und das Sicherheitsventil."],
    thinElements: ["Ticks, needle, seam, label lines and the zip are 0.13 mm detail strokes; the stubs are 24 mm high (0.48 mm on paper).", "Marken, Nadel, Fuge, Schildlinien und Naht sind 0,13-mm-Detailstriche; die Stutzen sind 24 mm hoch (0,48 mm auf Papier)."],
    hatching: ["None; the quarters step lightly in tone from 0.90 at the bottom to 0.84 at the top.", "Keine; die Viertel verlaufen im Ton leicht von 0,90 unten bis 0,84 oben."],
    styles: ["Jacket 0.84 to 0.90, cover 0.78, feet 0.35, base ring 0.4, stubs 0.7 with nuts 0.45, dial 0.95; body pen 0.18 mm, detail 0.13 mm, glazing 0.1 mm.", "Mantel 0,84 bis 0,90, Abdeckung 0,78, Füße 0,35, Bodenring 0,4, Stutzen 0,7 mit Muttern 0,45, Thermometer 0,95; Körperstift 0,18 mm, Detail 0,13 mm, Verglasung 0,1 mm."],
    overlays: ["Each quarter is its own region: in the thermal view a half-charged tank is warm in the upper two quarters and cold below, a charged tank warm top to bottom with the warmest top; the cover and stubs release heat at the top.", "Jedes Viertel ist eine eigene Region: in der Thermalansicht ist ein halbgeladener Speicher in den oberen zwei Vierteln warm und unten kalt, ein geladener von oben bis unten warm mit dem wärmsten Kopf; Abdeckung und Stutzen geben oben Wärme ab."],
  },
);

const rep100 = authored(
  100,
  [75, 150],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    rect("feet", "base", "body", 2.9, true, 0.35, -0.22, 0, 0.44, 0.06, { material: "powder-steel" }),
    ...["zone-4", "zone-3", "zone-2", "zone-1"].map((id, i) => bandRect(id, 3 + i * 0.01, 0.9 - i * 0.02, id)),
    poly("cap", "cap", "body", 3.1, true, 0.78, DOME, true, { region: "top-cap", material: "cap-plastic" }),
    ...stubs(0.03, false),
    poly("dial", "zone-2", "body", 3.3, true, 0.95, circlePts(DIAL[0], DIAL[1], 0.05, 20), true, { material: "window-glass" }),
  ],
  {
    silhouette: ["A 6 mm by 16 mm front elevation: a tall cylinder with four stacked shells, a dome, a foot bar, a round dial and short stubs.", "Eine Vorderansicht von 6 mm mal 16 mm: ein hoher Zylinder mit vier übereinanderliegenden Schalen, einer Wölbung, einem Fußbalken, einem runden Thermometer und kurzen Stutzen."],
    retained: ["The four shells with their seams, the dome, the foot bar, the dial and the six stubs.", "Die vier Schalen mit ihren Fugen, die Wölbung, der Fußbalken, das Thermometer und die sechs Stutzen."],
    merged: ["The two feet and the base ring form one bar of 0.44 m.", "Die zwei Füße und der Bodenring bilden einen Balken von 0,44 m."],
    omitted: ["Union nuts, dome seam, ticks, needle, label and zip.", "Überwurfmuttern, Wölbungsfuge, Marken, Nadel, Schild und Naht."],
    thinElements: ["The stubs are thickened from 30 to 40 mm (0.4 mm on paper).", "Die Stutzen sind von 30 auf 40 mm (0,4 mm auf Papier) verdickt."],
    hatching: ["None.", "Keine."],
    styles: ["Shells 0.84 to 0.90, dome 0.78, foot bar 0.35, stubs 0.7, dial 0.95; one body pen of 0.18 mm.", "Schalen 0,84 bis 0,90, Wölbung 0,78, Fußbalken 0,35, Stutzen 0,7, Thermometer 0,95; ein Körperstift von 0,18 mm."],
    overlays: ["The four shells keep their own regions, so the stratification of the tank still shows.", "Die vier Schalen behalten ihre eigenen Regionen, sodass die Schichtung des Speichers sichtbar bleibt."],
  },
);

const rep200 = authored(
  200,
  [150, 350],
  { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.2 },
  [
    rect("feet", "base", "body", 2.9, true, 0.35, -0.22, 0, 0.44, 0.06, { material: "powder-steel" }),
    rect("lower", "zone-4", "body", 3, true, 0.9, -0.27, 0.06, 0.54, 0.72, { regions: ["zone-4", "zone-3"], material: "foam-jacket" }),
    rect("upper", "zone-2", "body", 3.01, true, 0.86, -0.27, 0.78, 0.54, 0.72, { regions: ["zone-2", "zone-1"], material: "foam-jacket" }),
    poly("cap", "cap", "body", 3.1, true, 0.78, DOME, true, { region: "top-cap", material: "cap-plastic" }),
    rect("hot-out", "connections", "body", 3.2, true, 0.7, 0.16, 1.55, 0.05, 0.05, { region: "connections", material: "brass" }),
    rect("side-l", "connections", "body", 3.2, true, 0.7, -0.3, 1.275, 0.04, 0.05, { region: "connections", material: "brass" }),
    rect("side-r", "connections", "body", 3.2, true, 0.7, 0.26, 0.975, 0.04, 0.05, { region: "connections", material: "brass" }),
  ],
  {
    silhouette: ["A 3 mm by 8 mm mark: a tall rounded block of two stacked halves on a foot bar, with a dome and one stub.", "Ein Zeichen von 3 mm mal 8 mm: ein hoher Block aus zwei übereinanderliegenden Hälften auf einem Fußbalken, mit Wölbung und einem Stutzen."],
    retained: ["The foot bar, two jacket halves of 0.72 m, the dome and the hot-water stub.", "Der Fußbalken, zwei Mantelhälften von 0,72 m, die Wölbung und der Warmwasserstutzen."],
    merged: ["The lower two quarters form the lower half and the upper two the upper half (two regions each); the side stubs are reduced to two thickened stubs of 50 mm height and the hot-water stub is thickened to 50 mm.", "Die unteren zwei Viertel bilden die untere Hälfte und die oberen zwei die obere (je zwei Regionen); die seitlichen Stutzen sind auf zwei verdickte Stutzen von 50 mm Höhe reduziert und der Warmwasserstutzen ist auf 50 mm verdickt."],
    omitted: ["Four of the five side stubs, dial, needle, label, seams and the base ring.", "Vier der fünf seitlichen Stutzen, Thermometer, Nadel, Schild, Fugen und der Bodenring."],
    thinElements: ["None below the feature width; the stub is 0.25 mm on paper.", "Nichts unterhalb der Mindestbreite; der Stutzen ist auf Papier 0,25 mm breit."],
    hatching: ["None.", "Keine."],
    styles: ["Lower half 0.90, upper half 0.86, dome 0.78, feet 0.35, stub 0.7; one body pen of 0.13 mm.", "Untere Hälfte 0,90, obere 0,86, Wölbung 0,78, Füße 0,35, Stutzen 0,7; ein Körperstift von 0,13 mm."],
    overlays: ["The halves list their quarters as merged regions, so a half-charged tank still shows a warm top half and a cold bottom half.", "Die Hälften führen ihre Viertel als zusammengeführte Regionen, sodass ein halbgeladener Speicher eine warme obere und eine kalte untere Hälfte zeigt."],
  },
);

const rep500 = unavailable(
  500,
  ["At 1:500 the 0.6 m by 1.6 m cylinder is 1.2 mm by 3.2 mm and its 0.03 m stubs 0.06 mm: the quarters of the stratification and all connections vanish below every print profile.", "Bei 1:500 ist der 0,6 m mal 1,6 m große Zylinder 1,2 mm mal 3,2 mm groß und seine 0,03 m Stutzen 0,06 mm: die Viertel der Schichtung und alle Anschlüsse verschwinden unter jedem Druckprofil."],
  ["Stubs of 0.03 m", "Stutzen von 0,03 m"],
  0.03,
  [{ kind: "scale", denominator: 200 }, { kind: "scale", denominator: 100 }],
);

const zoneRegion = (id, title, heatW) => regionOf(id, title, "surface", "operating", "foam-jacket", 0.61, 0.36, "vertical-plate", [single("jacket", "Polyurethane foam jacket 80 mm with sheet skin", "Polyurethan-Schaummantel 80 mm mit Blechhaut", "insulation", "foam-jacket", 0.08, 2.3, 3000, "iso-10456-2007")], 3000, { operating: { key: id, heatW } }, "iso-10456-2007", 0.3);
const regions = [
  zoneRegion("zone-1", ["Upper quarter of the jacket", "Oberes Mantelviertel"], 20),
  zoneRegion("zone-2", ["Second quarter from above", "Zweites Mantelviertel von oben"], 17),
  zoneRegion("zone-3", ["Second quarter from below", "Zweites Mantelviertel von unten"], 14),
  zoneRegion("zone-4", ["Lower quarter of the jacket", "Unteres Mantelviertel"], 10),
  regionOf("top-cap", ["Domed top cover", "Gewölbte Abdeckung"], "surface", "operating", "cap-plastic", 0.4, 0.5, "horizontal-plate-up", [single("shell", "ABS cover 3 mm over foam", "ABS-Abdeckung 3 mm über Schaum", "polymer", "cap-plastic", 0.003, 0.02, 2500, "iso-10456-2007")], 2500, { operating: { key: "zone-1", heatW: 6 } }, "iso-10456-2007", 0.8),
  regionOf("connections", ["Connection stubs", "Anschlussstutzen"], "surface", "operating", "brass", 0.03, 0.04, "cylinder", [single("tube", "Brass tube 2 mm", "Messingrohr 2 mm", "metal", "brass", 0.002, 0.00002, 1700, "incropera-2018")], 1700, { operating: { key: "zone-1", heatW: 9 } }, "incropera-2018"),
];

const bind = (plume, ops) => ({
  regions: Object.fromEntries(regions.map((r) => [r.id, exp(1)])),
  emitters: { "top-plume": plume },
  operating: ops,
});
const ops = (a, b, c, d) => ({ "zone-1": a, "zone-2": b, "zone-3": c, "zone-4": d });
const feet = ["foot-left", "foot-right"];
const needle = (deg) => ({ needle: { angleDeg: deg } });
const poses = [
  pose("cold", ["Cold", "Kalt"], ["The tank is filled with cold water: all four quarters are at room temperature and the needle stands left.", "Der Speicher ist mit kaltem Wasser gefüllt: alle vier Viertel haben Raumtemperatur und die Nadel steht links."], needle(70), feet, "auto", bind(0, ops(0, 0, 0, 0))),
  pose("half-charged", ["Half charged", "Halb geladen"], ["The upper half is hot and the lower half still cold with a thin transition between them: the upper quarters lose most heat, the needle stands in the middle.", "Die obere Hälfte ist heiß und die untere noch kalt mit einem dünnen Übergang: die oberen Viertel verlieren am meisten Wärme, die Nadel steht in der Mitte."], needle(0), feet, "auto", bind(0.6, ops(1, 0.9, 0.15, 0))),
  pose("charged", ["Fully charged", "Voll geladen"], ["The whole tank is hot, the top slightly warmer than the bottom: the standby loss is 76 W and the needle stands right.", "Der ganze Speicher ist heiß, oben etwas wärmer als unten: der Bereitschaftsverlust beträgt 76 W und die Nadel steht rechts."], needle(-70), feet, "auto", bind(1, ops(1, 1, 1, 1))),
];

const contacts = (d) => feet.map((anchor) => ({ anchor, fromS: 0, toS: d }));
const keyed = (pairs) => pairs.map(([t, p, ease]) => ({ t, pose: p, ease }));
const clips = [
  clip(
    "charge-up",
    ["Charging from cold", "Aufladen aus dem kalten Zustand"],
    ["The coil heats the tank from the top: the upper quarter is hot after two minutes, the tank is half charged after five minutes and fully charged after fifteen minutes.", "Die Rohrwendel heizt den Speicher von oben: das obere Viertel ist nach zwei Minuten heiß, der Speicher nach fünf Minuten halb und nach fünfzehn Minuten voll geladen."],
    900,
    1,
    keyed([[0, "cold", "smooth"], [300, "half-charged", "smooth"], [900, "charged", "smooth"]]),
    contacts(900),
    [["top-hot", 0, 300, ["Top fills with heat", "Kopf füllt sich mit Wärme"]], ["bottom-hot", 300, 900, ["Bottom follows", "Boden folgt"]]],
    [[0, "operating", ["Charging starts", "Aufladen beginnt"], "zone-1"], [900, "shutdown", ["Fully charged", "Voll geladen"], "zone-4"]],
    [
      ["operating:zone-1", "linear", [[0, 0], [100, 1]]],
      ["operating:zone-2", "linear", [[0, 0], [200, 0.9], [500, 1]]],
      ["operating:zone-3", "linear", [[0, 0], [300, 0.15], [700, 1]]],
      ["operating:zone-4", "linear", [[0, 0], [500, 0.02], [900, 1]]],
      ["emitter:top-plume", "linear", [[0, 0], [100, 1]]],
    ],
  ),
  clip(
    "draw-off",
    ["Drawing off hot water", "Warmwasser entnehmen"],
    ["Hot water is drawn off at the top while cold water enters at the bottom: within ten minutes the cold layer rises from the bottom to the middle and the tank is half charged.", "Oben wird Warmwasser entnommen und unten fließt Kaltwasser nach: binnen zehn Minuten steigt die kalte Schicht vom Boden bis zur Mitte und der Speicher ist halb geladen."],
    600,
    1,
    keyed([[0, "charged", "smooth"], [600, "half-charged", "smooth"]]),
    contacts(600),
    [["cold-rises", 0, 600, ["Cold layer rises", "Kalte Schicht steigt"]]],
    [[0, "load-on", ["Draw-off starts", "Entnahme beginnt"]]],
    [
      ["operating:zone-1", "linear", [[0, 1], [600, 1]]],
      ["operating:zone-2", "linear", [[0, 1], [500, 0.9]]],
      ["operating:zone-3", "linear", [[0, 1], [400, 0.15]]],
      ["operating:zone-4", "linear", [[0, 1], [300, 0]]],
      ["emitter:top-plume", "linear", [[0, 1], [600, 0.6]]],
    ],
  ),
];

export const cylinder = () => {
  const massKg = sum(sp.bones.map((b) => b.mass?.kg ?? 0));
  return wrap({
    id: "hot-water-cylinder",
    category: "appliances",
    title: tx("Hot-Water Storage Cylinder, 200 L", "Warmwasser-Standspeicher, 200 l"),
    summary: tx(
      "A free-standing insulated hot-water storage tank of 0.6 m diameter and 1.6 m height in front elevation: four jacket quarters that carry the stratification (cold, half charged, fully charged), a domed cover, side stubs and a thermometer; the standby loss is 76 W when fully charged.",
      "Ein freistehender gedämmter Warmwasser-Standspeicher von 0,6 m Durchmesser und 1,6 m Höhe in der Vorderansicht: vier Mantelviertel, die die Schichtung tragen (kalt, halb geladen, voll geladen), eine gewölbte Abdeckung, seitliche Stutzen und ein Thermometer; der Bereitschaftsverlust beträgt 76 W bei voller Ladung.",
    ),
    tags: ["hot-water-cylinder", "storage-tank", "buffer", "dhw", "stratification", "heating", "hydronic", "appliance", "heating-plant", "standby-loss"],
    provenance: {
      author: "Semio staffage content (A9)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [
        ref("en-12897-2016", "EN 12897:2016 Water supply - Specification for indirectly heated unvented (closed) storage water heaters (dimensions, insulation and standby heat loss of storage cylinders)."),
        ref("eu-812-2013", "Commission Delegated Regulation (EU) No 812/2013 supplementing Directive 2010/30/EU with regard to the energy labelling of water heaters, hot water storage tanks (standby loss as a function of the volume)."),
        INCROPERA,
        ISO10456,
      ],
      notes: tx(
        "Front elevation of a floor-standing hot-water storage cylinder of about 200 litres (product class of EN 12897): outer diameter of the foam jacket 0.54 m, 0.6 m with the side stubs, height 1.6 m with the dome, standing on feet 0.06 m high; the mass of 275 kg is the tank filled with water (authored estimate: 200 kg water, 75 kg steel, coil, jacket and fittings). The origin is the floor at the tank axis. The jacket is drawn in four quarters because the thermal stratification of the water is shown through them: charged, every quarter loses heat, the upper ones more (20, 17, 14 and 10 W: together 61 W); the dome with the hot-water stub adds 6 W and the connections 9 W, so the standby loss at full charge is 76 W (energy label class B limit for 200 l about 80 W, Delegated Regulation 812/2013, authored estimate). Half charged, the upper half is hot, the lower half cold and a thin transition lies between them, therefore the operating levels of the quarters are 1, 0.9, 0.15 and 0 (a thermocline of about 0.1 m height). The heat of the stored water itself (60 degrees C) is not released in the room except through these losses.",
        "Vorderansicht eines bodenstehenden Warmwasser-Standspeichers von etwa 200 Litern (Produktklasse nach EN 12897): Außendurchmesser des Schaummantels 0,54 m, mit den seitlichen Stutzen 0,6 m, Höhe 1,6 m mit der Wölbung, auf Füßen von 0,06 m Höhe; die Masse von 275 kg ist der mit Wasser gefüllte Speicher (Schätzwert: 200 kg Wasser, 75 kg Stahl, Rohrwendel, Mantel und Armaturen). Der Ursprung ist der Boden in der Speicherachse. Der Mantel ist in vier Vierteln gezeichnet, weil durch sie die thermische Schichtung des Wassers gezeigt wird: geladen verliert jedes Viertel Wärme, die oberen mehr (20, 17, 14 und 10 W: zusammen 61 W); die Wölbung mit dem Warmwasserstutzen fügt 6 W und die Anschlüsse 9 W hinzu, sodass der Bereitschaftsverlust bei voller Ladung 76 W beträgt (Grenze der Energieeffizienzklasse B für 200 l etwa 80 W, Delegierte Verordnung 812/2013, Schätzwert). Halb geladen ist die obere Hälfte heiß, die untere kalt und dazwischen liegt ein dünner Übergang, daher sind die Betriebsstufen der Viertel 1, 0,9, 0,15 und 0 (eine Sprungschicht von etwa 0,1 m Höhe). Die Wärme des gespeicherten Wassers selbst (60 Grad C) wird nur über diese Verluste in den Raum abgegeben.",
      ),
    },
    dimensions: { heightM: 1.6, widthM: 0.6, depthM: 0.6, massKg },
    materials: [
      materialOf("foam-jacket", "Polyurethane foam jacket with sheet skin", "Polyurethan-Schaummantel mit Blechhaut", 0.9, 0.4, 40, 0.035, 1400, "literature", "iso-10456-2007"),
      materialOf("cap-plastic", "ABS cover", "ABS-Abdeckung", 0.9, 0.4, 1050, 0.17, 1400, SRC, "incropera-2018"),
      materialOf("powder-steel", "Powder-coated steel", "Pulverbeschichteter Stahl", 0.9, 0.5, 7850, 50, 460, "literature", "incropera-2018"),
      materialOf("window-glass", "Thermometer glass", "Thermometerglas", 0.9, 0.1, 2500, 1, 840, SRC, "incropera-2018"),
      materialOf("brass", "Brass connection stub", "Anschlussstutzen aus Messing", 0.5, 0.5, 8500, 120, 380, "literature", "incropera-2018"),
    ],
    rig: { kind: "mechanical", root: "base", bones: sp.bones.map((b) => ({ ...b, ...(b.mass ? { mass: { ...b.mass } } : {}) })) },
    anchors: [
      { id: "foot-left", bone: "base", x: -0.185, y: 0, kind: "contact", title: tx("Left foot", "Linker Fuß"), support: "contact", normalDeg: 90, footprintLengthM: 0.07, footprintWidthM: 0.07, frictionCoefficient: 0.5 },
      { id: "foot-right", bone: "base", x: 0.185, y: 0, kind: "contact", title: tx("Right foot", "Rechter Fuß"), support: "contact", normalDeg: 90, footprintLengthM: 0.07, footprintWidthM: 0.07, frictionCoefficient: 0.5 },
      { id: "top", bone: "cap", x: 0, y: 0.1, kind: "emitter", title: tx("Apex of the dome", "Scheitel der Wölbung") },
    ],
    representations: { 500: rep500, 200: rep200, 100: rep100, 50: rep50 },
    symbols: [],
    poses,
    clips,
    thermal: {
      environment: { airTempC: 21, meanRadiantTempC: 21, relativeHumidity: 0.5, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325 },
      passengers: [],
      operating: ["zone-1", "zone-2", "zone-3", "zone-4"].map((id, i) => ({
        id,
        title: tx(`Stored heat in quarter ${i + 1} from the top`, `Gespeicherte Wärme im ${i + 1}. Viertel von oben`),
        levels: [level(0, "Cold (room temperature)", "Kalt (Raumtemperatur)"), level(0.02, "Cold with a trace of heat", "Kalt mit einem Hauch Wärme"), level(0.15, "Thermocline, mostly cold", "Sprungschicht, überwiegend kalt"), level(0.9, "Thermocline, mostly hot", "Sprungschicht, überwiegend heiß"), level(1, "Hot (60 degrees C)", "Heiß (60 Grad C)")],
      })),
      regions,
      emitters: [{ id: "top-plume", kind: "plume", title: tx("Warm air rising over the dome", "Warme Luft über der Wölbung"), anchor: "top", directionDeg: 90, diameterM: 0.4, region: "top-cap", source: SRC, reference: "incropera-2018" }],
      assumptions: [
        tx(
          "The heat loss of each quarter is proportional to the heat stored in it: the levels 0 to 1 scale the heat release of the quarter from none (cold) to its charged value (hot 60 degrees C).",
          "Der Wärmeverlust jedes Viertels ist proportional zu der in ihm gespeicherten Wärme: die Stufen 0 bis 1 skalieren die Wärmeabgabe des Viertels von keiner (kalt) bis zu seinem Wert bei Ladung (heiß 60 Grad C).",
        ),
        tx(
          "The plume of the warm dome and the connections follow the upper quarter: they are cold when that quarter is cold.",
          "Die Fahne der warmen Wölbung und die Anschlüsse folgen dem oberen Viertel: sie sind kalt, wenn dieses Viertel kalt ist.",
        ),
      ],
    },
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: feet,
      assumptions: [
        tx(
          "The filled 275 kg tank stands on two feet in the plane of the view (a third foot lies behind them); the load of 2.7 kN is carried by the floor.",
          "Der gefüllte Speicher von 275 kg steht auf zwei Füßen in der Bildebene (ein dritter liegt dahinter); die Last von 2,7 kN trägt der Boden.",
        ),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "half-charged", defaultClip: "charge-up", defaultScale: 50, viewpoints: ["elevation"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
};
