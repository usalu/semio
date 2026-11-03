import { ALL_EXPORTS, INCROPERA, authored, clip, level, pose, ref, tx, unavailable, wrap } from "../🧰️a10/lib.mjs";
import { materialOf, space, sum } from "../🧰️a10/space.mjs";
import { SRC, fixed, m, region, single, supportAnchors, wallContacts } from "./common.mjs";

const sp = space([["body", null, 0, 0, { title: tx("Diffuser with collar", "Auslass mit Anschlusskragen"), lengthM: 0.595, joint: fixed, mass: m(3.2, 0, -0.01) }]]);
const { rect, poly } = sp;

const cone = (id, tone, top, bottom, xTop, xBottom, z) =>
  poly(id, "body", "body", z, true, tone, [[-xTop, top], [xTop, top], [xBottom, bottom], [-xBottom, bottom]], true, { region: "plate", material: "powder-coated-steel" });
const collar = (z = 2.9) => rect("collar", "body", "body", z, true, 0.5, -0.125, 0, 0.25, 0.05, { region: "collar", material: "galvanised-steel" });

const rep50 = authored(
  50,
  [20, 75],
  { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 },
  [
    collar(),
    rect("flange", "body", "body", 3, true, 0.88, -0.2975, -0.012, 0.595, 0.012, { r: 0.002, region: "plate", material: "powder-coated-steel" }),
    cone("cone-1", 0.8, -0.012, -0.032, 0.265, 0.245, 3.1),
    cone("cone-2", 0.7, -0.032, -0.052, 0.2, 0.18, 3.2),
    cone("cone-3", 0.6, -0.052, -0.07, 0.135, 0.115, 3.3),
    poly("slot-left", "body", "detail", 3.4, false, 0.15, [[-0.262, -0.0125], [-0.205, -0.0325]], false, { region: "plate", material: "powder-coated-steel" }),
    poly("slot-right", "body", "detail", 3.4, false, 0.15, [[0.262, -0.0125], [0.205, -0.0325]], false, { region: "plate", material: "powder-coated-steel" }),
  ],
  {
    silhouette: ["A 0.595 m square four-way ceiling diffuser in side view, 11.9 mm wide and 2.4 mm high on paper: a thin flange flush with the ceiling, three stepped cones narrowing downwards, and a short round collar on top that goes through the ceiling.", "Ein quadratischer Vierwege-Deckenauslass von 0,595 m in der Seitenansicht, auf Papier 11,9 mm breit und 2,4 mm hoch: ein dünner Flansch bündig mit der Decke, drei nach unten schmaler werdende Kegelstufen und oben ein kurzer runder Kragen, der durch die Decke geht."],
    retained: ["Flange of 12 mm, three cones of 20, 20 and 18 mm height stepping in by 65 mm each, the 250 mm collar of 50 mm height, and one slot line on each side that shows where the air leaves.", "Flansch von 12 mm, drei Kegel von 20, 20 und 18 mm Höhe, die jeweils 65 mm einrücken, der 250 mm breite Kragen von 50 mm Höhe und je eine Schlitzlinie auf jeder Seite, die zeigt, wo die Luft austritt."],
    merged: ["The inner plate behind the cones and the deflector vanes are part of the cones.", "Die Innenplatte hinter den Kegeln und die Leitlamellen gehen in den Kegeln auf."],
    omitted: ["The central screw, the neck damper and the flexible duct behind the collar.", "Die zentrale Schraube, die Drosselklappe im Hals und der flexible Kanal hinter dem Kragen."],
    thinElements: ["The flange is 12 mm (0.24 mm); the slot lines are 0.13 mm detail pens.", "Der Flansch ist 12 mm (0,24 mm) stark; die Schlitzlinien sind Linien mit dem 0,13-mm-Detailstift."],
    hatching: ["None.", "Keine."],
    styles: ["Flange 0.88, cones 0.8, 0.7 and 0.6, collar 0.5; body pen 0.18 mm, detail 0.13 mm, ground 0.35 mm.", "Flansch 0,88, Kegel 0,8, 0,7 und 0,6, Kragen 0,5; Körperstift 0,18 mm, Detail 0,13 mm, Boden 0,35 mm."],
    overlays: ["Plate (flange and cones) and collar are two regions: in the thermal view the plate is cooled by the supply air behind it, and the air leaves left and right along the ceiling.", "Platte (Flansch und Kegel) und Kragen sind zwei Regionen: in der Thermalansicht wird die Platte von der Zuluft dahinter gekühlt, und die Luft tritt links und rechts an der Decke entlang aus."],
  },
);

const rep100 = authored(
  100,
  [75, 150],
  { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    collar(),
    rect("flange", "body", "body", 3, true, 0.88, -0.2975, -0.02, 0.595, 0.02, { region: "plate", material: "powder-coated-steel" }),
    cone("cone-1", 0.75, -0.02, -0.045, 0.255, 0.225, 3.1),
    cone("cone-2", 0.65, -0.045, -0.07, 0.18, 0.12, 3.2),
  ],
  {
    silhouette: ["A 6 mm wide and 1.2 mm high diffuser: a flange, two stepped cones and a short collar.", "Ein Auslass von 6 mm Breite und 1,2 mm Höhe: ein Flansch, zwei Kegelstufen und ein kurzer Kragen."],
    retained: ["Flange, two cones (three merged into two), collar.", "Flansch, zwei Kegelstufen (aus drei zusammengeführt), Kragen."],
    merged: ["The three cones of 1:50 become two.", "Die drei Kegel von 1:50 werden zu zwei."],
    omitted: ["Slot lines.", "Schlitzlinien."],
    thinElements: ["The flange is thickened from 12 to 20 mm (0.2 mm) so that it prints as a filled shape.", "Der Flansch ist von 12 auf 20 mm (0,2 mm) verstärkt, damit er als Fläche druckt."],
    hatching: ["None.", "Keine."],
    styles: ["Flange 0.88, cones 0.75 and 0.65, collar 0.5; one body pen of 0.18 mm.", "Flansch 0,88, Kegel 0,75 und 0,65, Kragen 0,5; ein Körperstift von 0,18 mm."],
    overlays: ["Plate and collar stay two regions.", "Platte und Kragen bleiben zwei Regionen."],
  },
);

const rep200 = authored(
  200,
  [150, 350],
  { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 },
  [
    collar(),
    rect("flange", "body", "body", 3, true, 0.88, -0.2975, -0.04, 0.595, 0.04, { region: "plate", material: "powder-coated-steel" }),
    cone("cone", 0.7, -0.04, -0.07, 0.24, 0.12, 3.1),
  ],
  {
    silhouette: ["A 3 mm wide and 0.6 mm high diffuser: a broad flange with one cone and a collar.", "Ein Auslass von 3 mm Breite und 0,6 mm Höhe: ein breiter Flansch mit einem Kegel und einem Kragen."],
    retained: ["Flange, one merged cone, collar.", "Flansch, ein zusammengeführter Kegel, Kragen."],
    merged: ["All three cones become one trapezoid.", "Alle drei Kegel werden zu einem Trapez."],
    omitted: ["Slot lines and cone steps.", "Schlitzlinien und Kegelstufen."],
    thinElements: ["The flange is thickened to 40 mm (0.2 mm) and is therefore taller than in reality.", "Der Flansch ist auf 40 mm (0,2 mm) verstärkt und damit höher als in Wirklichkeit."],
    hatching: ["None.", "Keine."],
    styles: ["Flange 0.88, cone 0.7, collar 0.5; one body pen of 0.13 mm.", "Flansch 0,88, Kegel 0,7, Kragen 0,5; ein Körperstift von 0,13 mm."],
    overlays: ["Plate and collar stay two regions.", "Platte und Kragen bleiben zwei Regionen."],
  },
);

const rep500 = unavailable(
  500,
  ["At 1:500 the diffuser is 1.2 mm wide and its flange 0.024 mm thick, below every print profile; draw the ceiling opening as a line break instead.", "Bei 1:500 ist der Auslass 1,2 mm breit und sein Flansch 0,024 mm stark, unter jedem Druckprofil; zeichnen Sie stattdessen die Deckenöffnung als Linienunterbrechung."],
  ["Flange of 12 mm", "Flansch von 12 mm"],
  0.012,
  [{ kind: "scale", denominator: 200 }, { kind: "scale", denominator: 100 }],
);

const regions = [
  region("plate", ["Face plate and cones", "Frontplatte und Kegel"], "surface", "operating", "powder-coated-steel", 0.45, 0.595, "horizontal-plate-down", [single("sheet", "Powder-coated steel sheet 0.8 mm", "Pulverbeschichtetes Stahlblech 0,8 mm", "metal", "powder-coated-steel", 0.0008, 0.00002, 3600)], 3600, { operating: { key: "supply", heatW: -25 } }),
  region("collar", ["Round collar", "Runder Kragen"], "surface", "passive", "galvanised-steel", 0.04, 0.25, "cylinder", [single("tube", "Galvanised steel 0.7 mm", "Verzinkter Stahl 0,7 mm", "metal", "galvanised-steel", 0.0007, 0.00002, 3500)], 3500),
];

const bind = (supply) => ({
  regions: {
    plate: { exposedFraction: 0.9, sunlitFraction: 0, contact: { fraction: 0.1, tempC: 26 } },
    collar: { exposedFraction: 0.5, sunlitFraction: 0 },
  },
  emitters: { "jet-left": supply, "jet-right": supply },
  operating: { supply },
});

const hang = ["ceiling-hanger-left", "ceiling-hanger-right"];
const poses = [
  pose("off", ["Supply air off", "Zuluft aus"], ["The fan is off and the room air is at rest.", "Der Ventilator ist aus und die Raumluft ruht."], {}, hang, "authored", bind(0)),
  pose("half", ["Half volume flow", "Halber Volumenstrom"], ["A variable-volume box passes half of the design flow: 150 cubic metres per hour at 18 degC.", "Ein Variabel-Volumenstromregler lässt die Hälfte des Auslegungsstroms durch: 150 Kubikmeter je Stunde bei 18 °C."], {}, hang, "authored", bind(0.5)),
  pose("design", ["Design volume flow", "Auslegungsvolumenstrom"], ["The diffuser supplies 300 cubic metres per hour of 18 degC air; the jets spread left and right along the ceiling.", "Der Auslass bläst 300 Kubikmeter je Stunde Luft von 18 °C ein; die Strahlen breiten sich links und rechts an der Decke aus."], {}, hang, "authored", bind(1)),
];

const clips = [
  clip(
    "switch-on",
    ["Cooling supply switched on", "Kühlende Zuluft wird eingeschaltet"],
    ["The supply air is switched on at the design flow after ten seconds; the face plate cools below room temperature and the jets spread along the ceiling.", "Die Zuluft wird nach zehn Sekunden mit dem Auslegungsstrom eingeschaltet; die Frontplatte kühlt unter die Raumtemperatur ab und die Strahlen breiten sich an der Decke aus."],
    300,
    1,
    [{ t: 0, pose: "off", ease: "step" }, { t: 10, pose: "design", ease: "step" }, { t: 300, pose: "design", ease: "step" }],
    wallContacts(hang, 300),
    [["off", 0, 10, ["Off", "Aus"]], ["supply", 10, 300, ["Cooling supply", "Kühlende Zuluft"]]],
    [[10, "operating", ["Supply air switched on", "Zuluft eingeschaltet"], "supply"]],
    [["operating:supply", "step", [[0, 0], [10, 1]]], ["emitter:jet-left", "step", [[0, 0], [10, 1]]], ["emitter:jet-right", "step", [[0, 0], [10, 1]]]],
  ),
  clip(
    "modulate",
    ["Variable volume flow and switch off", "Variabler Volumenstrom und Ausschalten"],
    ["The diffuser runs at half flow, goes to the design flow after five minutes and is switched off after ten; the plate cools and warms again.", "Der Auslass läuft mit halbem Strom, geht nach fünf Minuten auf den Auslegungsstrom und wird nach zehn ausgeschaltet; die Platte kühlt ab und erwärmt sich wieder."],
    900,
    1,
    [{ t: 0, pose: "half", ease: "step" }, { t: 300, pose: "design", ease: "step" }, { t: 600, pose: "off", ease: "step" }, { t: 900, pose: "off", ease: "step" }],
    wallContacts(hang, 900),
    [["half", 0, 300, ["Half flow", "Halber Strom"]], ["design", 300, 600, ["Design flow", "Auslegungsstrom"]], ["off", 600, 900, ["Off, warming", "Aus, erwärmt sich"]]],
    [[600, "shutdown", ["Supply air switched off", "Zuluft ausgeschaltet"], "supply"]],
    [["operating:supply", "step", [[0, 0.5], [300, 1], [600, 0]]], ["emitter:jet-left", "step", [[0, 0.5], [300, 1], [600, 0]]], ["emitter:jet-right", "step", [[0, 0.5], [300, 1], [600, 0]]]],
  ),
];

export const ceilingDiffuser = () => {
  const massKg = sum(sp.bones.map((b) => b.mass.kg));
  return wrap({
    id: "ceiling-diffuser",
    category: "appliances",
    title: tx("Four-Way Square Ceiling Diffuser", "Quadratischer Vierwege-Deckenauslass"),
    summary: tx(
      "A 0.595 m square four-way ceiling air diffuser in side view: a flange flush with the ceiling, stepped cones below and a round collar above; it supplies 300 m3/h of 18 degC air that spreads left and right along the ceiling.",
      "Ein quadratischer Vierwege-Deckenluftauslass von 0,595 m in der Seitenansicht: ein Flansch bündig mit der Decke, darunter Kegelstufen und darüber ein runder Kragen; er bläst 300 m3/h Luft von 18 °C ein, die sich links und rechts an der Decke entlang ausbreitet.",
    ),
    tags: ["diffuser", "air-outlet", "supply-air", "ceiling", "cooling", "ventilation", "air-conditioning", "appliance", "component"],
    provenance: {
      author: "Semio staffage content (G4)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [
        ref("din-en-12238-2001", "DIN EN 12238:2001 Ventilation for buildings - Air terminal devices - Aerodynamic testing and rating for mixed flow application (throw of a ceiling jet along the ceiling by the Coanda effect)."),
        ref("vdi-2083-4-2019", "VDI 2083 Part 4 and manufacturer data for square four-way ceiling diffusers of nominal size 600 (design flow 100 to 400 cubic metres per hour)."),
        INCROPERA,
      ],
      notes: tx(
        "A square four-way ceiling diffuser of nominal size 600 seen from the side: a flange of 0.595 m by 0.595 m flush with the ceiling (12 mm thick), three stepped cones narrowing downwards to 0.23 m with a total depth of 0.07 m, and a round collar of 0.25 m width and 0.05 m height above the ceiling plane for the duct connection. The origin is the centre of the flange at the ceiling plane, the diffuser hangs below it. At level 1 it supplies 0.083 m3/s (300 m3/h) of air 8 K cooler than a 26 degC room, that is 18 degC, in four directions; the side view shows two of them (left and right) as 0.021 m3/s each. The jets leave horizontally under the flange and cling to the ceiling. The mass of 3.2 kg is rounded from this product class (authored estimate).",
        "Ein quadratischer Vierwege-Deckenauslass der Nennweite 600 von der Seite gesehen: ein Flansch von 0,595 m mal 0,595 m bündig mit der Decke (12 mm stark), drei nach unten auf 0,23 m schmaler werdende Kegelstufen mit einer Gesamttiefe von 0,07 m und oberhalb der Deckenebene ein runder Kragen von 0,25 m Breite und 0,05 m Höhe für den Kanalanschluss. Der Ursprung ist die Mitte des Flansches in der Deckenebene, der Auslass hängt darunter. Auf Stufe 1 bläst er 0,083 m3/s (300 m3/h) Luft ein, die 8 K kühler ist als ein Raum von 26 °C, also 18 °C, in vier Richtungen; die Seitenansicht zeigt zwei davon (links und rechts) mit je 0,021 m3/s. Die Strahlen verlassen den Auslass waagerecht unter dem Flansch und legen sich an die Decke an. Die Masse von 3,2 kg ist aus dieser Produktklasse gerundet (Schätzwert).",
      ),
    },
    dimensions: { heightM: 0.12, widthM: 0.595, depthM: 0.595, massKg },
    materials: [
      materialOf("powder-coated-steel", "Powder-coated steel sheet", "Pulverbeschichtetes Stahlblech", 0.9, 0.3, 7800, 50, 490, SRC, "incropera-2018"),
      materialOf("galvanised-steel", "Galvanised steel", "Verzinkter Stahl", 0.3, 0.5, 7800, 50, 490, SRC, "incropera-2018"),
    ],
    rig: { kind: "mechanical", root: "body", bones: sp.bones.map((b) => ({ ...b, mass: { ...b.mass } })) },
    anchors: [
      ...supportAnchors("body", { id: "ceiling-hanger-left", x: -0.28, y: 0 }, { id: "ceiling-hanger-right", x: 0.28, y: 0 }, [["Left hanger in the ceiling", "Linker Halter in der Decke"], ["Right hanger in the ceiling", "Rechter Halter in der Decke"]], 90),
      { id: "outlet-left", bone: "body", x: -0.262, y: -0.022, kind: "emitter", title: tx("Left outlet slot under the flange", "Linker Auslassschlitz unter dem Flansch") },
      { id: "outlet-right", bone: "body", x: 0.262, y: -0.022, kind: "emitter", title: tx("Right outlet slot under the flange", "Rechter Auslassschlitz unter dem Flansch") },
    ],
    representations: { 500: rep500, 200: rep200, 100: rep100, 50: rep50 },
    symbols: [],
    poses,
    clips,
    thermal: {
      environment: { airTempC: 26, meanRadiantTempC: 26, relativeHumidity: 0.5, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325 },
      passengers: [],
      operating: [{ id: "supply", title: tx("Supply air", "Zuluft"), levels: [level(0, "Off", "Aus"), level(0.5, "Half flow (150 m3/h)", "Halber Strom (150 m3/h)"), level(1, "Design flow (300 m3/h, 18 degC)", "Auslegungsstrom (300 m3/h, 18 °C)")] }],
      regions,
      emitters: [
        { id: "jet-left", kind: "vent", title: tx("Supply air jet leaving to the left along the ceiling", "Zuluftstrahl nach links an der Decke entlang"), anchor: "outlet-left", directionDeg: 180, diameterM: 0.04, operatingKey: "supply", volumeFlowM3S: 0.0208, temperatureRiseK: -8, vapourKgPerKg: 0, source: SRC, reference: "din-en-12238-2001" },
        { id: "jet-right", kind: "vent", title: tx("Supply air jet leaving to the right along the ceiling", "Zuluftstrahl nach rechts an der Decke entlang"), anchor: "outlet-right", directionDeg: 0, diameterM: 0.04, operatingKey: "supply", volumeFlowM3S: 0.0208, temperatureRiseK: -8, vapourKgPerKg: 0, source: SRC, reference: "din-en-12238-2001" },
      ],
      assumptions: [
        tx(
          "The supply air is 8 K cooler than the room air of 26 degC at every level; the face plate is cooled with about 25 W at level 1 (inner heat transfer of about 8 W/m2K over 0.36 m2 and 8 K).",
          "Die Zuluft ist auf jeder Stufe 8 K kühler als die Raumluft von 26 °C; die Frontplatte wird auf Stufe 1 mit etwa 25 W gekühlt (innerer Wärmeübergang von etwa 8 W/m2K über 0,36 m2 und 8 K).",
        ),
        tx(
          "The view shows two of the four jets (left and right); the two jets normal to the drawing plane are not drawn, so each emitter carries one quarter of the 300 m3/h.",
          "Die Ansicht zeigt zwei der vier Strahlen (links und rechts); die beiden Strahlen senkrecht zur Zeichenebene sind nicht gezeichnet, deshalb tragen die Emitter je ein Viertel der 300 m3/h.",
        ),
      ],
    },
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: hang,
      assumptions: [
        tx(
          "The 3.2 kg diffuser hangs on two ceiling hangers: the left one is a pin, the right one a roller.",
          "Der 3,2 kg schwere Auslass hängt an zwei Deckenhaltern: der linke ist ein Festlager, der rechte ein Rollenlager.",
        ),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "design", defaultClip: "switch-on", defaultScale: 50, viewpoints: ["elevation"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
};
