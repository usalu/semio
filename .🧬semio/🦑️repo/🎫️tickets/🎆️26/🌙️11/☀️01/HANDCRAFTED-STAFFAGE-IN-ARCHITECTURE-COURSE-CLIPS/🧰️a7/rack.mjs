import { ALL_EXPORTS, INCROPERA, ISO10456, WILKINS_HOSNI, authored, bone, clip, ellipse, layer, level, path, pose, rect, ref, tx, unavailable, wrap } from "../🧰️a3/lib.mjs";

const SOURCE = "authored-estimate";
const U = 0.04445;
const Y0 = 0.1;
const yb = (u) => Y0 + (u - 1) * U;
const yt = (u) => Y0 + u * U;
const W19 = 0.4826;
const XE = W19 / 2;
const r4 = (n) => Math.round(n * 1e4) / 1e4;

const STACK = [
  [1, 4, "storage"],
  [5, 6, "server"],
  [7, 8, "server"],
  [9, 10, "server"],
  [11, 12, "server"],
  [13, 14, "server"],
  [15, 16, "server"],
  [40, 40, "patch"],
  [41, 41, "brush"],
  [42, 42, "switch"],
];

const dashes = (x0, x1, y0, y1, pitch, dash, gap) => {
  let d = "";
  let row = 0;
  for (let y = y0; y <= y1 + 1e-9; y += pitch, row += 1) {
    const off = row % 2 === 0 ? 0 : (dash + gap) / 2;
    for (let x = x0 + off; x + dash <= x1 + 1e-9; x += dash + gap) d += `M ${r4(x)} ${r4(y)} h ${dash} `;
  }
  return d.trim();
};

const boxes = (cells) => cells.map(([x, y, w, h]) => `M ${r4(x)} ${r4(y)} h ${r4(w)} v ${r4(h)} h ${r4(-w)} Z`).join(" ");

const frame50 = () => [
  rect("post-left", "frame", "body", 3.0, true, 0.5, -0.3, 0.07, 0.03, 1.93, { region: "cabinet", material: "rack-steel" }),
  rect("post-right", "frame", "body", 3.0, true, 0.5, 0.27, 0.07, 0.03, 1.93, { region: "cabinet", material: "rack-steel" }),
  rect("roof", "frame", "body", 3.0, true, 0.45, -0.3, 1.967, 0.6, 0.033, { region: "cabinet", material: "rack-steel" }),
  rect("base-rail", "frame", "body", 3.0, true, 0.45, -0.27, 0.07, 0.54, 0.03, { region: "cabinet", material: "rack-steel" }),
  rect("caster-left", "feet", "body", 3.0, true, 0.2, -0.265, 0, 0.03, 0.07, { r: 0.012, region: "feet", material: "caster-rubber" }),
  rect("caster-right", "feet", "body", 3.0, true, 0.2, 0.235, 0, 0.03, 0.07, { r: 0.012, region: "feet", material: "caster-rubber" }),
];

const equipment50 = () => {
  const out = [];
  for (const [a, b, kind] of STACK) {
    const y = yb(a) + 0.0005;
    const h = (b - a + 1) * U - 0.001;
    const id = `${kind}-${a}`;
    if (kind === "storage") {
      out.push(rect(id, "equipment", "body", 3.2, true, 0.4, -XE, y, W19, h, { region: "servers", material: "chassis" }));
      for (let row = 0; row < 2; row += 1) {
        for (let col = 0; col < 6; col += 1) {
          out.push(rect(`${id}-bay-${row}-${col}`, "equipment", "detail", 3.3, false, 0.3, -0.19 + col * 0.0645, y + 0.032 + row * 0.0555, 0.052, 0.045));
        }
      }
      out.push(ellipse(`${id}-led`, "equipment", "detail", 3.3, true, 0.1, 0.215, y + 0.0885, 0.0065, 0.0065, { region: "servers" }));
    } else if (kind === "server") {
      out.push(rect(id, "equipment", "body", 3.2, true, 0.45, -XE, y, W19, h, { region: "servers", material: "chassis" }));
      for (let col = 0; col < 4; col += 1) {
        out.push(rect(`${id}-bay-${col}`, "equipment", "detail", 3.3, false, 0.3, 0.02 + col * 0.0555, y + 0.0155, 0.043, 0.06));
      }
      out.push(ellipse(`${id}-led`, "equipment", "detail", 3.3, true, 0.1, -0.216, y + h / 2, 0.0065, 0.0065, { region: "servers" }));
      out.push(path(`${id}-vent`, "equipment", "detail", 3.3, false, 0.3, boxes([-0.205, -0.17, -0.135, -0.1, -0.065].flatMap((x) => [[x, y + 0.0155, 0.022, 0.0255], [x, y + 0.0485, 0.022, 0.0255]]))));
    } else if (kind === "blank") {
      out.push(rect(id, "equipment", "body", 3.2, true, 0.32, -XE, y, W19, h, { region: "blanks", material: "blank-steel" }));
    } else if (kind === "patch") {
      out.push(rect(id, "equipment", "body", 3.2, true, 0.5, -XE, y, W19, h, { region: "blanks", material: "blank-steel" }));
      out.push(path(`${id}-slot`, "equipment", "detail", 3.3, false, 0.2, `M -0.2 ${r4(y + h / 2)} h 0.4`));
    } else if (kind === "brush") {
      out.push(rect(id, "equipment", "body", 3.2, true, 0.38, -XE, y, W19, h, { region: "blanks", material: "blank-steel" }));
      out.push(path(`${id}-slot`, "equipment", "detail", 3.3, false, 0.2, `M -0.19 ${r4(y + h / 2)} h 0.38`));
    } else if (kind === "switch") {
      out.push(rect(id, "equipment", "body", 3.2, true, 0.3, -XE, y, W19, h, { region: "switch", material: "chassis" }));
      const cells = [];
      for (let i = 0; i < 12; i += 1) {
        cells.push([-0.19 + i * 0.0245, y + 0.0095, 0.0155, 0.0105]);
        cells.push([-0.19 + i * 0.0245, y + 0.0235, 0.0155, 0.0105]);
      }
      out.push(path(`${id}-ports`, "equipment", "detail", 3.3, false, 0.3, boxes(cells)));
      out.push(ellipse(`${id}-led`, "equipment", "detail", 3.3, true, 0.1, 0.215, y + h / 2, 0.0065, 0.0065, { region: "switch" }));
    }
  }
  return out;
};

const door50 = () => [
  rect("door-field", "door", "body", 2.9, true, 0.18, -0.27, 0.1, 0.54, 1.867, { region: "door", material: "mesh-steel" }),
  path("door-perforation", "door", "detail", 4.0, false, 0.3, dashes(-0.255, 0.255, yb(1) + U / 2, yb(42) + U / 2, U, 0.012, 0.024)),
  rect("door-handle", "door", "body", 4.1, true, 0.2, 0.232, 0.95, 0.022, 0.13, { r: 0.004, region: "door", material: "mesh-steel" }),
];

const rep50 = authored(
  50,
  [20, 75],
  { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.35 },
  [...frame50(), ...equipment50(), ...door50()],
  {
    silhouette: ["A front view 12 mm wide and 40 mm tall on paper: two corner posts, a roof, a base rail and two casters frame a perforated door field, in front of which the 19-inch equipment stack stands.", "Eine Vorderansicht von 12 mm Breite und 40 mm Höhe auf Papier: zwei Eckpfosten, ein Dach, eine Fußschiene und zwei Rollen rahmen ein gelochtes Türfeld, vor dem der 19-Zoll-Geräteturm steht."],
    retained: ["Height 2.0 m, width 0.6 m, the 42 height units of 44.45 mm, the 482.6 mm equipment width, the storage array and six 2U servers at the bottom, patch panel, cable brush and switch at the top, the open units between them, the door handle.", "Höhe 2,0 m, Breite 0,6 m, die 42 Höheneinheiten zu 44,45 mm, die Gerätebreite von 482,6 mm, der Speicher und sechs 2-HE-Server unten, Patchfeld, Kabelbürste und Switch oben, die offenen Einheiten dazwischen, der Türgriff."],
    merged: ["Hinges, lock and the door frame merge into the door field; rails and cable entries merge into the posts and the roof.", "Scharniere, Schloss und der Türrahmen gehen im Türfeld auf; Schienen und Kabeleinführungen gehen in den Pfosten und das Dach auf."],
    omitted: ["The mounting rails, the rear door, the levelling feet, the PDU strips and all cabling; the door is drawn behind the equipment fronts so that the units are seen through its 70 percent open perforation.", "Die Montageschienen, die Rücktür, die Nivellierfüße, die Steckdosenleisten und alle Kabel; die Tür ist hinter den Gerätefronten gezeichnet, damit die Geräte durch ihre zu 70 Prozent offene Lochung zu sehen sind."],
    thinElements: ["The perforation is one detail path of 12 mm dashes in staggered rows of 44 mm pitch over the door field; drive bays, vent slits, ports and slots are 0.13 mm lines, LEDs are 13 mm discs.", "Die Lochung ist ein Detailzug aus 12-mm-Strichen in versetzten Reihen im Abstand von 44 mm über dem Türfeld; Laufwerksschächte, Lüftungsschlitze, Anschlüsse und Schlitze sind 0,13-mm-Linien, LEDs sind Scheiben von 13 mm."],
    hatching: ["None.", "Keine."],
    styles: ["Posts and roof 0.5 and 0.45, servers 0.45, storage 0.4, switch 0.3, door field 0.18, casters 0.2; body pen 0.18 mm, detail pen 0.13 mm.", "Pfosten und Dach 0,5 und 0,45, Server 0,45, Speicher 0,4, Switch 0,3, Türfeld 0,18, Rollen 0,2; Körperstift 0,18 mm, Detailstift 0,13 mm."],
    overlays: ["Cabinet, servers, switch, patch block, door and casters are separate regions: in the thermal view the warm server fronts stand out from the cool door field and the warm cabinet sheets.", "Schrank, Server, Switch, Patchblock, Tür und Rollen sind getrennte Regionen: In der Thermalansicht heben sich die warmen Serverfronten vom kühlen Türfeld und den warmen Schrankblechen ab."],
  },
);

const rep100 = authored(
  100,
  [75, 150],
  { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 },
  [
    rect("post-left", "frame", "body", 3.0, true, 0.5, -0.3, 0.07, 0.034, 1.93, { region: "cabinet", material: "rack-steel" }),
    rect("post-right", "frame", "body", 3.0, true, 0.5, 0.266, 0.07, 0.034, 1.93, { region: "cabinet", material: "rack-steel" }),
    rect("roof", "frame", "body", 3.0, true, 0.45, -0.3, 1.96, 0.6, 0.04, { region: "cabinet", material: "rack-steel" }),
    rect("base-rail", "frame", "body", 3.0, true, 0.45, -0.27, 0.07, 0.54, 0.034, { region: "cabinet", material: "rack-steel" }),
    rect("caster-left", "feet", "body", 3.0, true, 0.2, -0.265, 0, 0.04, 0.07, { r: 0.012, region: "feet", material: "caster-rubber" }),
    rect("caster-right", "feet", "body", 3.0, true, 0.2, 0.225, 0, 0.04, 0.07, { r: 0.012, region: "feet", material: "caster-rubber" }),
    rect("storage", "equipment", "body", 3.2, true, 0.4, -XE, yb(1), W19, 4 * U, { region: "servers", material: "chassis" }),
    rect("server-5", "equipment", "body", 3.2, true, 0.45, -XE, yb(5), W19, 2 * U - 0.003, { region: "servers", material: "chassis" }),
    rect("server-7", "equipment", "body", 3.2, true, 0.45, -XE, yb(7), W19, 2 * U - 0.003, { region: "servers", material: "chassis" }),
    rect("server-9", "equipment", "body", 3.2, true, 0.45, -XE, yb(9), W19, 2 * U - 0.003, { region: "servers", material: "chassis" }),
    rect("server-11", "equipment", "body", 3.2, true, 0.45, -XE, yb(11), W19, 2 * U - 0.003, { region: "servers", material: "chassis" }),
    rect("server-13", "equipment", "body", 3.2, true, 0.45, -XE, yb(13), W19, 2 * U - 0.003, { region: "servers", material: "chassis" }),
    rect("server-15", "equipment", "body", 3.2, true, 0.45, -XE, yb(15), W19, 2 * U - 0.003, { region: "servers", material: "chassis" }),
    rect("door-field", "door", "body", 2.9, true, 0.18, -0.266, 0.104, 0.532, 1.856, { region: "door", material: "mesh-steel" }),
    path("door-perforation", "door", "detail", 4.0, false, 0.3, dashes(-0.25, 0.25, yb(1) + U, yb(42), 2 * U, 0.02, 0.03)),
    rect("patch-brush", "equipment", "body", 3.2, true, 0.44, -XE, yb(40), W19, 2 * U, { region: "blanks", material: "blank-steel" }),
    rect("switch", "equipment", "body", 3.2, true, 0.3, -XE, yb(42), W19, U, { region: "switch", material: "chassis" }),
    rect("door-handle", "door", "body", 4.1, true, 0.2, 0.228, 0.93, 0.026, 0.16, { r: 0.006, region: "door", material: "mesh-steel" }),
  ],
  {
    silhouette: ["A front view 6 mm wide and 20 mm tall on paper: the corner posts, roof and base frame a dotted door field with the lower server stack and the upper switch group in front.", "Eine Vorderansicht von 6 mm Breite und 20 mm Höhe auf Papier: Eckpfosten, Dach und Fußrahmen fassen ein gepunktetes Türfeld ein, davor der untere Serverturm und die obere Switchgruppe."],
    retained: ["Height 2.0 m, width 0.6 m, the storage array, each of the six servers, the patch and brush block, the switch, the casters and the door handle.", "Höhe 2,0 m, Breite 0,6 m, der Speicher, jeder der sechs Server, der Patch- und Bürstenblock, der Switch, die Rollen und der Türgriff."],
    merged: ["Patch panel and cable brush are one 2U block.", "Patchfeld und Kabelbürste sind ein 2-HE-Block."],
    omitted: ["Drive bays, vent slits, ports and LEDs of the units and the rails.", "Laufwerksschächte, Lüftungsschlitze, Anschlüsse und LEDs der Geräte sowie die Schienen."],
    thinElements: ["Posts are widened from 30 to 34 mm and the roof to 40 mm so that fill and pen reach 0.34 mm; the 2U servers keep a 3 mm joint to the neighbours.", "Pfosten sind von 30 auf 34 mm und das Dach auf 40 mm verbreitert, damit Füllung und Stift 0,34 mm erreichen; die 2-HE-Server behalten eine Fuge von 3 mm zu den Nachbarn."],
    hatching: ["None.", "Keine."],
    styles: ["Posts 0.5, servers 0.45, storage 0.4, switch 0.3, door field 0.18; body pen 0.18 mm.", "Pfosten 0,5, Server 0,45, Speicher 0,4, Switch 0,3, Türfeld 0,18; Körperstift 0,18 mm."],
    overlays: ["The same six regions as at 1:50; the patch block is drawn with the blanks region, the switch with its own.", "Dieselben sechs Regionen wie bei 1:50; der Patchblock gehört zur Region Patchfeld, der Switch zu seiner eigenen."],
  },
);

const rep200 = authored(
  200,
  [150, 350],
  { body: 0.13, detail: 0.1, texture: null, glazing: null, ground: 0.25 },
  [
    rect("post-left", "frame", "body", 3.0, true, 0.5, -0.3, 0.07, 0.05, 1.93, { region: "cabinet", material: "rack-steel" }),
    rect("post-right", "frame", "body", 3.0, true, 0.5, 0.25, 0.07, 0.05, 1.93, { region: "cabinet", material: "rack-steel" }),
    rect("roof", "frame", "body", 3.0, true, 0.45, -0.3, 1.94, 0.6, 0.06, { region: "cabinet", material: "rack-steel" }),
    rect("base-rail", "frame", "body", 3.0, true, 0.45, -0.25, 0.07, 0.5, 0.05, { region: "cabinet", material: "rack-steel" }),
    rect("caster-left", "feet", "body", 3.0, true, 0.2, -0.27, 0, 0.05, 0.07, { region: "feet", material: "caster-rubber" }),
    rect("caster-right", "feet", "body", 3.0, true, 0.2, 0.22, 0, 0.05, 0.07, { region: "feet", material: "caster-rubber" }),
    rect("server-block", "equipment", "body", 3.2, true, 0.42, -XE, yb(1), W19, 16 * U, { region: "servers", material: "chassis" }),
    rect("door-field", "door", "body", 2.9, true, 0.18, -0.25, 0.12, 0.5, 1.82, { region: "door", material: "mesh-steel" }),
    path("door-perforation", "door", "detail", 4.0, false, 0.3, dashes(-0.2, 0.2, 0.2, 1.85, 0.18, 0.06, 0.12)),
    rect("top-group", "equipment", "body", 3.2, true, 0.4, -XE, yb(40), W19, 3 * U, { regions: ["switch", "blanks"], material: "chassis" }),
  ],
  {
    silhouette: ["A front view 3 mm wide and 10 mm tall on paper: a framed tower with one dark server block at the bottom, a top group and a dotted door field between.", "Eine Vorderansicht von 3 mm Breite und 10 mm Höhe auf Papier: ein gerahmter Turm mit einem dunklen Serverblock unten, einer oberen Gruppe und einem gepunkteten Türfeld dazwischen."],
    retained: ["Height 2.0 m, width 0.6 m, the frame, the 16U block of storage and six servers, the top group of switch, patch panel and brush, the door field.", "Höhe 2,0 m, Breite 0,6 m, der Rahmen, der 16-HE-Block aus Speicher und sechs Servern, die obere Gruppe aus Switch, Patchfeld und Bürste, das Türfeld."],
    merged: ["Storage and six servers are one block; switch, patch panel and brush are one top group (regions switch and blanks).", "Speicher und sechs Server sind ein Block; Switch, Patchfeld und Bürste sind eine obere Gruppe (Regionen Switch und Patchfeld)."],
    omitted: ["All unit details, the casters' wheels, the handle and the door edge.", "Alle Gerätedetails, die Räder der Rollen, der Griff und die Türkante."],
    thinElements: ["Posts are widened from 30 to 50 mm (0.25 mm) and the roof to 60 mm; the pen is 0.13 mm.", "Pfosten sind von 30 auf 50 mm (0,25 mm) und das Dach auf 60 mm verbreitert; der Stift ist 0,13 mm."],
    hatching: ["None.", "Keine."],
    styles: ["Posts 0.5, server block 0.42, top group 0.4, door field 0.18; one body pen of 0.13 mm.", "Pfosten 0,5, Serverblock 0,42, obere Gruppe 0,4, Türfeld 0,18; ein Körperstift von 0,13 mm."],
    overlays: ["Merged shapes carry area-weighted colour; the door field is the cool region behind the equipment.", "Zusammengeführte Formen tragen flächengewichtete Farbe; das Türfeld ist die kühle Region hinter den Geräten."],
  },
);


const rep500 = unavailable(
  500,
  ["At 1:500 the rack is 1.2 mm wide and 4 mm tall: the 30 mm corner posts are 0.06 mm, the perforation, the 44 mm units and the casters are far below every print profile, and a plain block would show no rack.", "Bei 1:500 ist das Rack 1,2 mm breit und 4 mm hoch: die 30-mm-Eckpfosten sind 0,06 mm, Lochung, 44-mm-Einheiten und Rollen liegen weit unter jedem Druckprofil, und ein einfacher Block zeigte kein Rack."],
  ["Corner post width of 30 mm", "Breite der Eckpfosten von 30 mm"],
  0.03,
  [{ kind: "scale", denominator: 200 }, { kind: "scale", denominator: 100 }],
);

const steel = (id, en, de, mat, t, r, c) => layer(id, [en, de], "metal", mat, t, r, c, SOURCE, "iso-10456-2007");

const mkRegion = (id, title, model, material, areaM2, characteristicLengthM, geometry, skyViewFactor, layers, heatCapacityJM2K, reference, heatW) => ({
  id,
  title: tx(...title),
  kind: "surface",
  model,
  material,
  areaM2,
  characteristicLengthM,
  geometry,
  skyViewFactor,
  layers,
  heatCapacityJM2K,
  ...(heatW === undefined ? {} : { operating: { key: "load", heatW } }),
  source: SOURCE,
  reference,
});

const regions = [
  mkRegion("door", ["Perforated front door", "Gelochte Fronttür"], "passive", "mesh-steel", 1.05, 1.9, "vertical-plate", 0.5, [steel("door-sheet", "Perforated steel sheet 1.5 mm, 70 percent open", "Lochblech 1,5 mm, 70 Prozent offen", "mesh-steel", 0.0015, 0.00003, 1800)], 1800, "iso-10456-2007"),
  mkRegion("servers", ["Server and storage fronts", "Server- und Speicherfronten"], "operating", "chassis", 0.343, 0.09, "vertical-plate", 0.5, [steel("front-bezel", "Steel front with plastic bezel 2 mm", "Stahlfront mit Kunststoffblende 2 mm", "chassis", 0.002, 0.00005, 3000)], 3000, "hosni-1999", 120),
  mkRegion("switch", ["Network switch front", "Front des Netzwerk-Switches"], "operating", "chassis", 0.021, 0.044, "vertical-plate", 0.5, [steel("switch-front", "Steel front 1.5 mm", "Stahlfront 1,5 mm", "chassis", 0.0015, 0.00004, 2400)], 2400, "hosni-1999", 20),
  mkRegion("blanks", ["Patch panel and cable brush", "Patchfeld und Kabelbürste"], "passive", "blank-steel", 0.043, 0.044, "vertical-plate", 0.5, [steel("blank-sheet", "Steel sheet 1.5 mm", "Stahlblech 1,5 mm", "blank-steel", 0.0015, 0.00003, 2400)], 2400, "iso-10456-2007"),
  mkRegion("cabinet", ["Cabinet posts, roof, base and panels", "Schrankpfosten, Dach, Sockel und Wände"], "operating", "rack-steel", 6.4, 2, "vertical-plate", 0.5, [steel("cabinet-sheet", "Powder-coated steel sheet 1.2 mm", "Pulverbeschichtetes Stahlblech 1,2 mm", "rack-steel", 0.0012, 0.00002, 5800)], 5800, "iso-10456-2007", 160),
  mkRegion("feet", ["Casters", "Rollen"], "passive", "caster-rubber", 0.06, 0.07, "vertical-plate", 0.5, [steel("caster-body", "Nylon wheel and steel bracket", "Nylonrad mit Stahlbügel", "caster-rubber", 0.006, 0.02, 4000)], 4000, "iso-10456-2007"),
];

const bind = (load, airflow) => ({
  regions: {
    door: { exposedFraction: 1, sunlitFraction: 0 },
    servers: { exposedFraction: 1, sunlitFraction: 0 },
    switch: { exposedFraction: 1, sunlitFraction: 0 },
    blanks: { exposedFraction: 1, sunlitFraction: 0 },
    cabinet: { exposedFraction: 1, sunlitFraction: 0 },
    feet: { exposedFraction: 0.5, sunlitFraction: 0, contact: { fraction: 0.5, tempC: 21 } },
  },
  emitters: { "exhaust-air": 1 },
  operating: { load, airflow },
});

const poses = [
  pose("idle", ["Idle", "Leerlauf"], ["The servers wait for work: 0.3 of the rated load, about 1.5 kW, and the fans run at 0.55 of full speed; warm air leaves at the rear and rises over the roof.", "Die Server warten auf Arbeit: 0,3 der Nennlast, etwa 1,5 kW, und die Lüfter laufen mit 0,55 der vollen Drehzahl; warme Luft tritt hinten aus und steigt über das Dach."], {}, ["base-contact"], "auto", bind(0.3, 0.55)),
  pose("full-load", ["Full load", "Volllast"], ["Rated load of 5 kW: the fans blow 0.26 m3/s from the cold front through the servers and out of the rear, 15 K warmer.", "Nennlast von 5 kW: die Lüfter blasen 0,26 m3/s von der kalten Front durch die Server und hinten 15 K wärmer wieder hinaus."], {}, ["base-contact"], "auto", bind(1, 1)),
  pose("off", ["Powered down", "Ausgeschaltet"], ["All equipment off, the rack stands at room temperature.", "Alle Geräte aus, das Rack steht auf Raumtemperatur."], {}, ["base-contact"], "auto", bind(0, 0)),
];

const hold = (d) => [{ anchor: "base-contact", fromS: 0, toS: d }];

const loadRamp = clip(
  "load-ramp",
  ["Load ramp", "Lastanstieg"],
  ["A job arrives: over 15 s the load climbs from idle to full load, the fans follow, after 30 s at full load the job ends and the rack falls back to idle. The loop closes at the idle state.", "Ein Auftrag trifft ein: In 15 s steigt die Last vom Leerlauf auf Volllast, die Lüfter folgen, nach 30 s Volllast endet der Auftrag und das Rack fällt in den Leerlauf zurück. Die Schleife schließt im Leerlauf."],
  60,
  5,
  [{ t: 0, pose: "idle", ease: "smooth" }, { t: 15, pose: "full-load", ease: "smooth" }, { t: 45, pose: "full-load", ease: "smooth" }, { t: 60, pose: "idle", ease: "smooth" }],
  hold(60),
  [["ramp-up", 0, 15, ["Load climbs", "Last steigt"]], ["full-load", 15, 45, ["Full load", "Volllast"]], ["ramp-down", 45, 60, ["Load falls", "Last fällt"]]],
  [[0, "load-on", ["Job arrives", "Auftrag trifft ein"], "load"], [45, "load-off", ["Job ends", "Auftrag endet"], "load"]],
  [
    ["operating:load", "smooth", [[0, 0.3], [15, 1], [45, 1], [60, 0.3]]],
    ["operating:airflow", "smooth", [[0, 0.55], [15, 1], [45, 1], [60, 0.55]]],
  ],
);
loadRamp.loop = true;

const powerUp = clip(
  "power-up",
  ["Power up to idle", "Hochfahren in den Leerlauf"],
  ["From cold: after 5 s the equipment is switched on, the fans spin up within 10 s and the cabinet warms up in the following 5 minutes.", "Aus dem kalten Zustand: nach 5 s werden die Geräte eingeschaltet, die Lüfter laufen innerhalb von 10 s hoch und der Schrank erwärmt sich in den folgenden 5 Minuten."],
  300,
  1,
  [{ t: 0, pose: "off", ease: "step" }, { t: 5, pose: "idle", ease: "step" }, { t: 300, pose: "idle", ease: "step" }],
  hold(300),
  [["cold", 0, 5, ["Off", "Aus"]], ["warm-up", 5, 300, ["Idle, warming up", "Leerlauf, wärmt auf"]]],
  [[5, "operating", ["Equipment switched on", "Geräte eingeschaltet"], "load"]],
  [
    ["operating:load", "step", [[0, 0], [5, 0.3]]],
    ["operating:airflow", "step", [[0, 0], [5, 0.55]]],
  ],
);

const clips = [loadRamp, powerUp];

const material = (id, en, de, e, a, rho, k, c, src, rf) => ({ id, title: tx(en, de), emissivity: e, solarAbsorptivity: a, densityKgM3: rho, conductivityWmK: k, specificHeatJKgK: c, source: src, reference: rf });

export const rack = () =>
  wrap({
    id: "server-rack",
    category: "appliances",
    title: tx("Server Rack, 42U", "Serverschrank, 42 HE"),
    summary: tx(
      "A 42U 19-inch server rack, 2.0 m high, 0.6 m wide and 1.0 m deep, with a perforated front door, six servers, a storage array and a switch: about 1.5 kW of heat at idle, 5 kW at full load, almost all of it in the air that the fans blow from the front to the rear.",
      "Ein 19-Zoll-Serverschrank mit 42 HE, 2,0 m hoch, 0,6 m breit und 1,0 m tief, mit gelochter Fronttür, sechs Servern, einem Speicher und einem Switch: im Leerlauf etwa 1,5 kW Wärme, bei Volllast 5 kW, fast alles in der Luft, die die Lüfter von vorn nach hinten blasen.",
    ),
    tags: ["server-rack", "data-centre", "it-equipment", "internal-gain", "electrical", "heat-source", "ventilation"],
    provenance: {
      author: "Semio staffage content (A7)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [
        ref("iec-60297-3-100-2008", "IEC 60297-3-100:2008 Mechanical structures for electronic equipment - Dimensions of mechanical structures of the 482,6 mm (19 in) series - Part 3-100: Basic dimensions of front panels, subracks, chassis, racks and cabinets (equipment width 482.6 mm, height unit 44.45 mm)."),
        ref("ashrae-tc99-2021", "ASHRAE TC 9.9 (2021). Thermal Guidelines for Data Processing Environments, 5th ed., ASHRAE (front-to-back airflow, air temperature rise of 10 to 20 K across servers, rack power densities)."),
        ref("ashrae-datacom-2012", "ASHRAE TC 9.9 (2012). Datacom Equipment Power Trends and Cooling Applications, 2nd ed., ASHRAE (heat loads of racks of servers, idle to full load)."),
        WILKINS_HOSNI,
        { id: "hosni-1999", citation: "Hosni, M. H., Jones, B. W., Xu, H. (1999). Experimental results for heat gain and radiant/convective split from equipment in buildings. ASHRAE Transactions 105(2), 527-539." },
        INCROPERA,
        ISO10456,
      ],
      notes: tx(
        "Front elevation of a typical freestanding 42U cabinet (no single product): 0.6 m x 1.0 m footprint, 2.0 m high on casters, 42 height units of 44.45 mm and the 482.6 mm equipment width follow IEC 60297-3-100; the mass of 234 kg is cabinet 95 kg, door 12 kg, casters 7 kg and equipment 120 kg (authored estimate). The stack is a 4U storage array, six 2U servers at the bottom, a 1U switch, a 1U patch panel and a 1U cable brush at the top; the units between stay open. Rated load 5 kW (about 0.8 kW per 2U server) and idle at 30 percent are an authored estimate within the 3 to 10 kW rack class of the ASHRAE datacom trends. All electrical power ends as heat: 4703 W leave as a warm air stream (0.26 m3/s at 15 K rise, the front-to-back flow of the fans, rising over the roof; the airflow itself runs perpendicular to the drawing plane), 120 W in the server fronts, 20 W in the switch front and 160 W in the cabinet sheets (hot-aisle air warming the steel). Because the emitter model scales flow and temperature rise together, the air stream carries heat with the square of the airflow level; the airflow level 0.55 therefore gives 30 percent of the rated air heat at idle, and the sum at idle is 1.51 kW.",
        "Vorderansicht eines typischen freistehenden 42-HE-Schranks (kein einzelnes Produkt): Grundfläche 0,6 m x 1,0 m, 2,0 m hoch auf Rollen, 42 Höheneinheiten zu 44,45 mm und die Gerätebreite von 482,6 mm folgen IEC 60297-3-100; die Masse von 234 kg setzt sich aus Schrank 95 kg, Tür 12 kg, Rollen 7 kg und Geräten 120 kg zusammen (Schätzwert). Der Turm besteht aus einem 4-HE-Speicher, sechs 2-HE-Servern unten, einem 1-HE-Switch, einem 1-HE-Patchfeld und einer 1-HE-Kabelbürste oben; die Einheiten dazwischen bleiben offen. Die Nennlast von 5 kW (etwa 0,8 kW je 2-HE-Server) und der Leerlauf bei 30 Prozent sind ein Schätzwert innerhalb der Rackklasse von 3 bis 10 kW der ASHRAE-Datacom-Trends. Alle elektrische Leistung endet als Wärme: 4703 W verlassen das Rack als warmer Luftstrom (0,26 m3/s bei 15 K Anstieg, der Strom der Lüfter von vorn nach hinten, über das Dach aufsteigend; die Luftströmung selbst verläuft senkrecht zur Zeichenebene), 120 W in den Serverfronten, 20 W in der Switch-Front und 160 W in den Schrankblechen (Warmgangluft erwärmt den Stahl). Weil das Emittermodell Volumenstrom und Temperaturanstieg gemeinsam skaliert, trägt der Luftstrom Wärme mit dem Quadrat der Luftstufe; die Luftstufe 0,55 liefert daher im Leerlauf 30 Prozent der Nennluftwärme, und die Summe im Leerlauf beträgt 1,51 kW.",
      ),
    },
    dimensions: { heightM: 2.0, widthM: 0.6, depthM: 1.0, massKg: 234 },
    materials: [
      material("rack-steel", "Powder-coated steel sheet", "Pulverbeschichtetes Stahlblech", 0.9, 0.9, 7850, 50, 490, SOURCE, "iso-10456-2007"),
      material("mesh-steel", "Perforated steel sheet", "Lochblech aus Stahl", 0.9, 0.9, 2400, 15, 490, SOURCE, "iso-10456-2007"),
      material("chassis", "Server chassis with plastic bezel", "Servergehäuse mit Kunststoffblende", 0.85, 0.8, 3000, 20, 700, SOURCE, "iso-10456-2007"),
      material("blank-steel", "Black steel blank panel", "Schwarze Stahlblindplatte", 0.9, 0.9, 7850, 50, 490, SOURCE, "iso-10456-2007"),
      material("caster-rubber", "Nylon caster wheel", "Rolle aus Nylon", 0.9, 0.9, 1140, 0.25, 1700, SOURCE, "iso-10456-2007"),
    ],
    rig: {
      kind: "mechanical",
      root: "frame",
      bones: [
        bone("frame", null, ["Cabinet frame", "Schrankrahmen"], 0, 0, 2, { type: "fixed" }, { kg: 95, comX: 0, comY: 1.0, source: SOURCE }),
        bone("feet", "frame", ["Casters", "Rollen"], 0, 0, 0.07, { type: "fixed" }, { kg: 7, comX: 0, comY: 0.03, source: SOURCE }),
        bone("equipment", "frame", ["Equipment stack", "Geräteturm"], 0, 0, 1.87, { type: "fixed" }, { kg: 120, comX: 0, comY: 0.8, source: SOURCE }),
        bone("door", "frame", ["Perforated door", "Gelochte Tür"], 0, 0, 1.87, { type: "fixed" }, { kg: 12, comX: 0, comY: 1.03, source: SOURCE }),
      ],
    },
    anchors: [
      { id: "base-contact", bone: "feet", x: 0, y: 0, kind: "contact", title: tx("Casters on the floor", "Rollen auf dem Boden"), support: "contact", footprintLengthM: 0.5, footprintWidthM: 0.9, frictionCoefficient: 0.6 },
      { id: "exhaust-top", bone: "frame", x: 0, y: 2, kind: "emitter", title: tx("Warm exhaust air over the roof", "Warme Abluft über dem Dach") },
    ],
    representations: { 500: rep500, 200: rep200, 100: rep100, 50: rep50 },
    symbols: [],
    poses,
    clips,
    thermal: {
      environment: { airTempC: 21, meanRadiantTempC: 21, relativeHumidity: 0.5, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325 },
      passengers: [],
      operating: [
        { id: "load", title: tx("Compute load", "Rechenlast"), levels: [level(0, "Off", "Aus"), level(0.3, "Idle (30 percent)", "Leerlauf (30 Prozent)"), level(1, "Full load (5 kW)", "Volllast (5 kW)")] },
        { id: "airflow", title: tx("Fan airflow", "Lüfterluftstrom"), levels: [level(0, "Off", "Aus"), level(0.55, "Idle speed", "Leerlaufdrehzahl"), level(1, "Full speed (0.26 m3/s)", "Volle Drehzahl (0,26 m3/s)")] },
      ],
      regions,
      emitters: [
        { id: "exhaust-air", kind: "fan", title: tx("Warm exhaust air of the server fans, front to rear", "Warme Abluft der Serverlüfter, von vorn nach hinten"), anchor: "exhaust-top", directionDeg: 90, diameterM: 0.5, operatingKey: "airflow", volumeFlowM3S: 0.26, temperatureRiseK: 15, vapourKgPerKg: 0, source: SOURCE, reference: "ashrae-tc99-2021" },
      ],
      assumptions: [
        tx(
          "Every watt ends as heat in the room: at level 1 the load heats the server fronts with 120 W, the switch front with 20 W and the cabinet sheets with 160 W; the remaining 4700 W leave with the exhaust air of the fans.",
          "Jedes Watt endet als Wärme im Raum: bei Stufe 1 erwärmt die Last die Serverfronten mit 120 W, die Switch-Front mit 20 W und die Schrankbleche mit 160 W; die übrigen 4700 W verlassen das Rack mit der Abluft der Lüfter.",
        ),
        tx(
          "The air crosses the rack from the cold front to the hot rear, perpendicular to the front elevation; the exhaust emitter sits on the roof edge and points upward, because the hot-aisle air rises there.",
          "Die Luft durchquert das Rack von der kalten Front zur heißen Rückseite, senkrecht zur Vorderansicht; der Abluft-Emitter sitzt an der Dachkante und zeigt nach oben, weil die Warmgangluft dort aufsteigt.",
        ),
      ],
    },
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: ["base-contact"],
      assumptions: [
        tx(
          "A rigid cabinet on four casters 0.5 m by 0.9 m apart; the centre of mass lies on the vertical axis of the footprint, 0.95 m above the floor.",
          "Ein starrer Schrank auf vier Rollen im Abstand von 0,5 m mal 0,9 m; der Schwerpunkt liegt auf der Mittelachse der Standfläche, 0,95 m über dem Boden.",
        ),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "full-load", defaultClip: "load-ramp", defaultScale: 50, viewpoints: ["elevation"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
