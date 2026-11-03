import { ASHRAE, EN_308, SRC, VDI_3803, baseContact, bindOf, clip, contacts, dec, fixed, level, m, materialOf, mkAsset, pose, ref, rep, reg, space2 as space, tx, un } from "./g2.mjs";

const CY = 0.31;
const sp = space([
  ["casing", null, 0, 0, { title: tx("Casing ring with stand", "Gehäusering mit Fuß"), lengthM: 0.62, joint: fixed, mass: m(6, 0, CY) }],
  ["seal", "casing", 0, CY, { title: tx("Regeneration sector with seals", "Regenerationssektor mit Dichtungen"), lengthM: 0.29, joint: fixed, mass: m(2, 0.15, CY) }],
  ["rotor", "casing", 0, CY, { title: tx("Rotating desiccant matrix", "Rotierende Sorptionsmatrix"), lengthM: 0.285, joint: { type: "revolute" }, mass: m(14, 0, CY) }],
]);
const { path, ellipse, poly } = sp;
const r5 = (n) => Math.round(n * 1e5) / 1e5;

const cells = (a, limit) => {
  const dx = Math.sqrt(3) * a;
  const parts = [];
  for (let row = -14; row <= 14; row += 1) {
    for (let col = -14; col <= 14; col += 1) {
      const cx = col * dx + (row % 2 ? dx / 2 : 0);
      const cy = row * 1.5 * a;
      if (Math.hypot(cx, cy) + a > limit || Math.hypot(cx, cy) < 0.055) continue;
      const pts = Array.from({ length: 6 }, (_, i) => {
        const t = (Math.PI / 180) * (60 * i + 30);
        return `${r5(cx + a * Math.cos(t))} ${r5(CY + cy + a * Math.sin(t))}`;
      });
      parts.push(`M ${pts.join(" L ")} Z`);
    }
  }
  return parts.join(" ");
};
const spokes = (n, r0, r1) => Array.from({ length: n }, (_, k) => {
  const a = (Math.PI * 2 * k) / n;
  return poly(`spoke-${k + 1}`, "rotor", "detail", 3.45, false, 0.2, [[r0 * Math.cos(a), CY + r0 * Math.sin(a)], [r1 * Math.cos(a), CY + r1 * Math.sin(a)]], false, { region: "disc", material: "silica-gel" });
});
const sector = (r, a0, a1) => {
  const pts = [];
  for (let a = a0; a <= a1; a += 6) pts.push(`${r5(r * Math.cos((a * Math.PI) / 180))} ${r5(CY + r * Math.sin((a * Math.PI) / 180))}`);
  return `M 0 ${CY} L ${pts.join(" L ")} Z`;
};

const rep50 = rep(50, [20, 75], { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 }, [
  ellipse("casing", "casing", "body", 3.2, true, 0.35, 0, CY, 0.31, 0.31, { region: "casing", material: "galvanised-steel" }),
  ellipse("disc", "rotor", "body", 3.3, true, 0.6, 0, CY, 0.285, 0.285, { region: "disc", material: "silica-gel" }),
  path("honeycomb", "rotor", "detail", 3.4, false, 0.15, cells(0.02, 0.28), { region: "disc", material: "silica-gel" }),
  ...spokes(8, 0.045, 0.285),
  path("regen-sector", "seal", "body", 3.6, true, 0.9, sector(0.285, -42, 42), { region: "regen", material: "silica-gel" }),
  ellipse("hub", "rotor", "body", 3.7, true, 0.4, 0, CY, 0.045, 0.045, { region: "hub", material: "galvanised-steel" }),
  poly("stand", "casing", "body", 3.1, true, 0.35, [[-0.12, 0], [0.12, 0], [0.1, 0.03], [-0.1, 0.03]], true, { region: "casing", material: "galvanised-steel" }),
], dec(
  ["A 0.62 m desiccant wheel seen face-on, 12 mm across on paper: a casing ring around a honeycomb disc with eight spokes and a hub, a hot regeneration sector on the right and a stand below.", "Ein 0,62 m großes Sorptionsrad in der Frontansicht, auf Papier 12 mm im Durchmesser: ein Gehäusering um eine Wabenscheibe mit acht Speichen und einer Nabe, ein heißer Regenerationssektor rechts und ein Fuß unten."],
  ["Casing ring, disc, a honeycomb of 0.02 m cells, eight spokes, the regeneration sector, the hub and the stand.", "Gehäusering, Scheibe, eine Wabe aus Zellen von 0,02 m, acht Speichen, der Regenerationssektor, die Nabe und der Fuß."],
  ["Nothing is merged.", "Nichts ist zusammengeführt."],
  ["The drive belt, the purge sector and the seals.", "Der Antriebsriemen, der Spülsektor und die Dichtungen."],
  ["The honeycomb has 0.02 m cells instead of the real 0.003 m flutes, so that the cells stay 0.4 mm apart on paper.", "Die Wabe hat Zellen von 0,02 m statt der realen Wellen von 0,003 m, damit die Zellen auf Papier 0,4 mm Abstand behalten."],
  ["Casing, disc, regeneration sector and hub are four regions: the sector glows hot while the rest of the disc warms moderately; honeycomb and spokes turn with the rotor bone.", "Gehäuse, Scheibe, Regenerationssektor und Nabe sind vier Regionen: der Sektor glüht heiß, während sich die Scheibe mäßig erwärmt; Wabe und Speichen drehen sich mit dem Rotorknochen."],
  { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 },
));
const rep100 = rep(100, [75, 150], { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 }, [
  ellipse("casing", "casing", "body", 3.2, true, 0.35, 0, CY, 0.31, 0.31, { region: "casing", material: "galvanised-steel" }),
  ellipse("disc", "rotor", "body", 3.3, true, 0.6, 0, CY, 0.275, 0.275, { region: "disc", material: "silica-gel" }),
  ...spokes(4, 0.05, 0.275),
  path("regen-sector", "seal", "body", 3.6, true, 0.9, sector(0.275, -45, 45), { region: "regen", material: "silica-gel" }),
  ellipse("hub", "rotor", "body", 3.7, true, 0.4, 0, CY, 0.055, 0.055, { region: "hub", material: "galvanised-steel" }),
  poly("stand", "casing", "body", 3.1, true, 0.35, [[-0.14, 0], [0.14, 0], [0.11, 0.035], [-0.11, 0.035]], true, { region: "casing", material: "galvanised-steel" }),
], dec(
  ["A 6 mm wheel: casing ring, disc with four spokes, a hot sector and a stand.", "Ein 6 mm großes Rad: Gehäusering, Scheibe mit vier Speichen, ein heißer Sektor und ein Fuß."],
  ["Casing, disc, four spokes (they show the rotation), sector, hub and stand.", "Gehäuse, Scheibe, vier Speichen (sie zeigen die Drehung), Sektor, Nabe und Fuß."],
  ["Nothing is merged.", "Nichts ist zusammengeführt."],
  ["The honeycomb and the drive belt.", "Die Wabe und der Antriebsriemen."],
  ["Spokes carry 0.13 mm lines; the hub is 110 mm across.", "Die Speichen sind 0,13-mm-Linien; die Nabe ist 110 mm groß."],
  ["Same four regions as at 1:50.", "Dieselben vier Regionen wie bei 1:50."],
  { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 },
));
const rep200 = rep(200, [150, 350], { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 }, [
  ellipse("casing", "casing", "body", 3.2, true, 0.35, 0, CY, 0.31, 0.31, { regions: ["casing", "hub"], material: "galvanised-steel" }),
  ellipse("disc", "rotor", "body", 3.3, true, 0.6, 0, CY, 0.26, 0.26, { region: "disc", material: "silica-gel" }),
  path("regen-sector", "seal", "body", 3.6, true, 0.9, sector(0.26, -45, 45), { region: "regen", material: "silica-gel" }),
  poly("stand", "casing", "body", 3.1, true, 0.35, [[-0.15, 0], [0.15, 0], [0.12, 0.04], [-0.12, 0.04]], true, { region: "casing", material: "galvanised-steel" }),
], dec(
  ["A 3 mm wheel of four shapes: casing circle, disc, hot sector and stand.", "Ein 3 mm großes Rad aus vier Formen: Gehäusekreis, Scheibe, heißer Sektor und Fuß."],
  ["Casing circle, disc, sector and stand.", "Gehäusekreis, Scheibe, Sektor und Fuß."],
  ["The hub is part of the casing circle (regions casing and hub).", "Die Nabe gehört zum Gehäusekreis (Regionen Gehäuse und Nabe)."],
  ["Honeycomb, spokes and hub; at this scale the rotation cannot be seen.", "Wabe, Speichen und Nabe; bei diesem Maßstab ist die Drehung nicht zu sehen."],
  ["The casing ring is a 50 mm band, the stand 40 mm tall.", "Der Gehäusering ist ein 50-mm-Band, der Fuß 40 mm hoch."],
  ["The casing circle lists casing and hub as merged regions.", "Der Gehäusekreis führt Gehäuse und Nabe als zusammengeführte Regionen."],
  { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 },
));

const regions = [
  reg("casing", ["Casing ring and stand", "Gehäusering und Fuß"], "surface", "passive", "galvanised-steel", 0.3, 0.62, "vertical-plate", 0.001, 0.00003, 3500),
  reg("disc", ["Desiccant matrix", "Sorptionsmatrix"], "heater", "operating", "silica-gel", 0.6, 0.57, "vertical-plate", 0.02, 0.1, 5000, { operating: { key: "run", heatW: 60 } }, { layerKind: "polymer" }),
  reg("regen", ["Regeneration sector", "Regenerationssektor"], "heater", "operating", "silica-gel", 0.12, 0.3, "vertical-plate", 0.02, 0.1, 5000, { operating: { key: "run", heatW: 55 } }, { layerKind: "polymer" }),
  reg("hub", ["Drive hub", "Antriebsnabe"], "engine", "operating", "galvanised-steel", 0.012, 0.09, "vertical-plate", 0.004, 0.00008, 3000, { operating: { key: "run", heatW: 4 } }),
];
const bind = bindOf({ casing: { exposedFraction: 0.9, contact: { fraction: 0.1, tempC: 21 } }, disc: { exposedFraction: 1 }, regen: { exposedFraction: 1 }, hub: { exposedFraction: 1 } });
const sup = ["base-contact"];
const poses = [
  pose("off", ["Off", "Aus"], ["The wheel stands still at room temperature.", "Das Rad steht still bei Raumtemperatur."], {}, sup, "auto", bind({ run: 0 })),
  pose("slow", ["Slow", "Langsam"], ["The wheel turns slowly and the regeneration sector starts to warm.", "Das Rad dreht langsam und der Regenerationssektor beginnt sich zu erwärmen."], { rotor: { angleDeg: 30 } }, sup, "auto", bind({ run: 0.5 })),
  pose("rated", ["Rated", "Nenndrehzahl"], ["The wheel turns at 12 revolutions per hour: the sector runs hot with solar-heated regeneration air.", "Das Rad dreht mit 12 Umdrehungen je Stunde: der Sektor läuft heiß mit solar erwärmter Regenerationsluft."], { rotor: { angleDeg: 60 } }, sup, "auto", bind({ run: 1 })),
];
const clips = [
  clip("spin", ["Turning", "Drehen"], ["The matrix turns once in 5 seconds (the real wheel needs 5 minutes) through the dry process air and the hot regeneration sector.", "Die Matrix dreht sich einmal in 5 Sekunden (das reale Rad braucht 5 Minuten) durch die trockene Prozessluft und den heißen Regenerationssektor."],
    5, 15,
    [{ t: 0, pose: "rated", ease: "linear", bones: { rotor: { angleDeg: 0 } } }, { t: 5, pose: "rated", ease: "linear", bones: { rotor: { angleDeg: 360 } } }],
    contacts(sup, 5), [["turn", 0, 5, ["Turning", "Drehen"]]], [], []),
  clip("start-and-run", ["Start and run", "Anlaufen und laufen"], ["The wheel starts, the regeneration sector heats and the disc warms over five minutes.", "Das Rad läuft an, der Regenerationssektor heizt sich auf und die Scheibe erwärmt sich in fünf Minuten."],
    300, 1,
    [{ t: 0, pose: "off", ease: "step" }, { t: 5, pose: "off", ease: "smooth" }, { t: 10, pose: "rated", ease: "linear" }, { t: 300, pose: "rated", ease: "linear", bones: { rotor: { angleDeg: 720 } } }],
    contacts(sup, 300), [["off", 0, 5, ["Off", "Aus"]], ["ramp", 5, 10, ["Starting", "Anlaufen"]], ["run", 10, 300, ["Running", "Läuft"]]],
    [[5, "ignition", ["Drive started", "Antrieb gestartet"], "run"]],
    [["operating:run", "smooth", [[0, 0], [5, 0], [10, 1]]]]),
];

export const wheel = () =>
  mkAsset({
    id: "sorption-wheel",
    category: "appliances",
    title: ["Desiccant Sorption Wheel", "Sorptionsrad"],
    summary: [
      "A 0.62 m desiccant wheel seen face-on: a honeycomb silica-gel matrix on spokes turns inside a casing ring, and its regeneration sector runs hot with solar-heated air while the rest of the disc warms from the adsorption heat.",
      "Ein 0,62 m großes Sorptionsrad in der Frontansicht: eine Silicagel-Wabenmatrix auf Speichen dreht sich in einem Gehäusering, und ihr Regenerationssektor läuft heiß mit solar erwärmter Luft, während sich der Rest der Scheibe an der Adsorptionswärme erwärmt.",
    ],
    tags: ["sorption-wheel", "desiccant", "dec", "dehumidification", "regeneration", "air-handling", "appliance"],
    references: [EN_308, VDI_3803, ASHRAE],
    notes: [
      "A rotary desiccant dehumidifier wheel of a small sorption air handler: a 0.57 m silica-gel coated honeycomb matrix of 0.2 m depth in a 0.62 m galvanised casing ring on a stand, driven by a belt at about 12 revolutions per hour; a 90 degree sector is closed off for regeneration air heated to about 70 degC by solar heat. The mass is 22 kg (casing and stand 6, matrix 14, sector plates and seals 2). The disc warms by the adsorption heat (to about 31 degC, 60 W against room air) and the regeneration sector to about 60 degC (55 W); both are authored estimates of the surface temperatures against still room air. The origin is the centre of the casing on the base line of the stand, x at the hub.",
      "Ein rotierendes Sorptionsrad eines kleinen Sorptionslüftungsgeräts: eine silicagelbeschichtete Wabenmatrix von 0,57 m Durchmesser und 0,2 m Tiefe in einem 0,62 m großen verzinkten Gehäusering auf einem Fuß, über einen Riemen mit rund 12 Umdrehungen je Stunde angetrieben; ein 90-Grad-Sektor ist für Regenerationsluft abgetrennt, die solar auf etwa 70 degC erwärmt wird. Die Masse beträgt 22 kg (Gehäuse und Fuß 6, Matrix 14, Sektorbleche und Dichtungen 2). Die Scheibe erwärmt sich durch die Adsorptionswärme (auf etwa 31 degC, 60 W gegen Raumluft) und der Regenerationssektor auf etwa 60 degC (55 W); beides sind Schätzwerte der Oberflächentemperaturen gegen ruhende Raumluft. Der Ursprung liegt auf der Grundlinie des Fußes, x an der Nabe.",
    ],
    dims: [0.62, 0.62, 0.22],
    materials: [
      materialOf("galvanised-steel", "Galvanised steel", "Verzinkter Stahl", 0.25, 0.5, 7800, 50, 480, "literature", "incropera-2018"),
      materialOf("silica-gel", "Silica-gel honeycomb", "Silicagel-Wabe", 0.9, 0.7, 700, 0.2, 920, SRC, "incropera-2018"),
    ],
    root: "casing",
    bones: sp.bones,
    anchors: [baseContact("casing", 0, 0, 0.24, 0.2)],
    reps: { 500: un(500, "the 0.02 m honeycomb cells and the 50 mm casing band", "die Wabenzellen von 0,02 m und das 50 mm breite Gehäuseband", 0.02, 1.2, [200, 100]), 200: rep200, 100: rep100, 50: rep50 },
    poses,
    clips,
    operating: [{ id: "run", title: tx("Wheel drive and regeneration", "Radantrieb und Regeneration"), levels: [level(0, "Off", "Aus"), level(0.5, "Slow", "Langsam"), level(1, "Rated (12 rev/h)", "Nenndrehzahl (12 U/h)")] }],
    regions,
    emitters: [],
    assumptions: [
      ["Process and regeneration air are not modelled as emitters of this asset: the diagram draws them, the asset gives the hot sector and the warm matrix.", "Prozess- und Regenerationsluft sind in diesem Asset nicht als Emitter modelliert: das Diagramm zeichnet sie, das Asset liefert den heißen Sektor und die warme Matrix."],
    ],
    structure: [["The 22 kg wheel stands on its 0.24 m by 0.2 m stand: a unilateral contact carries the weight.", "Das 22 kg schwere Rad steht auf seinem 0,24 m mal 0,2 m großen Fuß: ein einseitiger Kontakt trägt das Gewicht."]],
    defaultPose: "rated",
    defaultClip: "spin",
  });
