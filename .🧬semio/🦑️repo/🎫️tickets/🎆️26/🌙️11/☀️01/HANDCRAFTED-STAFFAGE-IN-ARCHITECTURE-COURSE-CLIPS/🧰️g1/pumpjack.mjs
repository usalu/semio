import { ALL_EXPORTS, INCROPERA, authored, clip, level, pose, ref, tx, wrap } from "./lib.mjs";
import { materialOf, space, sum } from "./space.mjs";
import { SRC, environment, fixed, m, region, single } from "./kit.mjs";

const r5 = (n) => Math.round(n * 1e5) / 1e5;
const rad = (d) => (d * Math.PI) / 180;
const deg = (r) => (r * 180) / Math.PI;

const P = [0, 4.4];
const R = 3.0;
const T0 = [2.7, 4.4];
const C = [2.7, 1.75];
const CR = 0.85;
const L = 2.65;

const sp = space([
  ["base", null, 0, 0, { title: tx("Foundation slab, skid and wellhead", "Fundamentplatte, Rahmen und Bohrlochkopf"), lengthM: 7.8, joint: fixed, mass: m(5400, 1.9, 0.3) }],
  ["post", "base", 0, 0.55, { title: tx("Samson post (A-frame)", "Lagerbock (A-Gestell)"), lengthM: 3.6, joint: fixed, mass: m(2400, 0, 1.75) }],
  ["beam", "post", P[0], P[1], { title: tx("Walking beam with horsehead", "Balancier mit Pferdekopf"), lengthM: 5.0, joint: { type: "revolute", minDeg: -25, maxDeg: 25 }, mass: m(2900, -0.3, 0.1) }],
  ["pitman", "beam", T0[0], T0[1], { title: tx("Pitman rod", "Schubstange"), lengthM: L, joint: { type: "revolute", minDeg: -45, maxDeg: 45 }, mass: m(350, 0, -1.4) }],
  ["gearbox", "base", C[0], C[1], { title: tx("Gear reducer with belt guard", "Getriebe mit Riemenschutz"), lengthM: 1.4, joint: fixed, mass: m(3200, 0, -0.45) }],
  ["crank", "gearbox", C[0], C[1], { title: tx("Crank arm with counterweight", "Kurbel mit Gegengewicht"), lengthM: CR, joint: { type: "revolute" }, mass: m(2600, -0.3, 0) }],
  ["rod", "base", -3.0, 3.18, { title: tx("Polished rod carrier bar", "Gestängeträger der Polierstange"), lengthM: 0.6, joint: { type: "prismatic", axisDeg: 90, minM: -1.2, maxM: 1.2 }, mass: m(180, 0, 0) }],
  ["motor", "base", 4.9, 0.9, { title: tx("Electric motor", "Elektromotor"), lengthM: 1.1, joint: fixed, mass: m(1100, 0, 0.05) }],
]);
const { rect, ellipse, poly } = sp;

const solveBeam = (phiDeg) => {
  const phi = rad(phiDeg);
  const q = [C[0] + CR * Math.cos(phi), C[1] + CR * Math.sin(phi)];
  const t = (th) => [P[0] + (T0[0] - P[0]) * Math.cos(th), P[1] + (T0[0] - P[0]) * Math.sin(th)];
  const f = (th) => Math.hypot(t(th)[0] - q[0], t(th)[1] - q[1]) - L;
  let lo = -0.6;
  let hi = 0.6;
  for (let i = 0; i < 80; i += 1) {
    const mid = (lo + hi) / 2;
    if (f(lo) * f(mid) <= 0) hi = mid;
    else lo = mid;
  }
  const th = (lo + hi) / 2;
  const tp = t(th);
  const dx = (q[0] - tp[0]) / L;
  const dy = (q[1] - tp[1]) / L;
  const psi = Math.atan2(dx, -dy);
  return { crank: { angleDeg: r5(phiDeg) }, beam: { angleDeg: r5(deg(th)) }, pitman: { angleDeg: r5(deg(psi - th)) }, rod: { dy: r5(-R * th) } };
};

const arc = (radius, fromDeg, toDeg, steps, centre = P) =>
  Array.from({ length: steps + 1 }, (_, i) => {
    const a = rad(fromDeg + ((toDeg - fromDeg) * i) / steps);
    return [r5(centre[0] + radius * Math.cos(a)), r5(centre[1] + radius * Math.sin(a))];
  });

const head = (detail) => [...arc(R, 160, 200, 8), [-2.1, 3.95], [-2.1, 4.85]];
const headHole = () => [...arc(2.65, 164, 196, 6), [-2.3, 4.1], [-2.3, 4.7]];
const beamOutline = () => [[-2.15, 4.2], [-2.15, 4.62], [-1.0, 4.72], [1.0, 4.72], [2.8, 4.58], [2.8, 4.22], [1.0, 4.08], [-1.0, 4.08]];
const weight = (outer, inner) => [...arc(outer, 142, 218, 8, C), ...arc(inner, 218, 142, 4, C)];

const bar = (id, bone, z, tone, x0, y0, x1, y1, o) => rect(id, bone, "body", z, true, tone, x0, y0, x1 - x0, y1 - y0, o);

const wellhead50 = [
  bar("wellhead-base", "base", 3.1, 0.55, -3.17, 0, -2.83, 0.5, { region: "wellhead", material: "painted-steel" }),
  bar("wellhead-valve", "base", 3.1, 0.7, -3.1, 0.5, -2.9, 1.0, { region: "wellhead", material: "painted-steel" }),
  bar("stuffing-box", "base", 3.1, 0.45, -3.08, 1.0, -2.92, 1.28, { region: "wellhead", material: "painted-steel" }),
  bar("flowline", "base", 3.1, 0.55, -2.83, 0.62, -2.0, 0.74, { region: "wellhead", material: "painted-steel" }),
];

const common = (lv) => {
  const w = lv === 500 ? 0.1 : 0.05;
  const list = [];
  const slab = lv >= 200 ? [bar("slab", "base", 3, 0.82, -1.9, 0, 5.9, 0.3, { region: "base", material: "concrete" }), bar("skid", "base", 3.05, 0.5, -1.6, 0.3, 5.6, 0.55, { region: "base", material: "painted-steel" })] : [bar("slab", "base", 3, 0.82, -1.9, 0, 5.9, 0.3, { region: "base", material: "concrete" }), bar("skid", "base", 3.05, 0.5, -1.6, 0.3, 5.6, 0.55, { region: "base", material: "painted-steel" })];
  list.push(...slab);
  list.push(...(lv === 50 ? wellhead50 : lv === 100 ? [bar("wellhead-base", "base", 3.1, 0.55, -3.17, 0, -2.83, 0.5, { region: "wellhead", material: "painted-steel" }), bar("wellhead-top", "base", 3.1, 0.6, -3.1, 0.5, -2.9, 1.28, { region: "wellhead", material: "painted-steel" })] : [bar("wellhead", "base", 3.1, 0.6, -3.15, 0, -2.85, 1.28, { region: "wellhead", material: "painted-steel" })]));
  list.push(bar("polished-rod", "base", 3.2, 0.9, -3.0 - w / 2, 1.28, -3.0 + w / 2, 4.4, { region: "steel", material: "painted-steel" }));
  const legW = lv === 500 ? 0.3 : 0.24;
  list.push(
    poly("leg-left", "post", "body", 3.3, true, 0.62, [[-1.6, 0.55], [-1.6 + legW, 0.55], [0.06, 4.15], [-0.14, 4.15]], true, { region: "steel", material: "painted-steel" }),
    poly("leg-right", "post", "body", 3.3, true, 0.62, [[1.6, 0.55], [1.6 - legW, 0.55], [-0.06, 4.15], [0.14, 4.15]], true, { region: "steel", material: "painted-steel" }),
  );
  if (lv !== 500) list.push(bar("brace", "post", 3.35, 0.55, -0.95, 1.6, 0.95, 1.76, { region: "steel", material: "painted-steel" }));
  if (lv === 50) list.push(ellipse("saddle", "post", "detail", 3.45, true, 0.3, 0, 4.28, 0.16, 0.16, { region: "steel", material: "painted-steel" }));
  list.push(poly("beam", "beam", "body", 3.5, true, 0.7, beamOutline(), true, { region: "steel", material: "painted-steel" }));
  list.push(poly("horsehead", "beam", "body", 3.55, true, 0.78, head(lv === 50), true, { region: "steel", material: "painted-steel" }));
  if (lv === 50) {
    list.push(poly("horsehead-web", "beam", "detail", 3.6, true, 0.15, headHole(), true, { region: "steel", material: "painted-steel" }));
    list.push(ellipse("centre-bearing", "beam", "detail", 3.65, true, 0.3, P[0], P[1], 0.2, 0.2, { region: "steel", material: "painted-steel" }));
    list.push(ellipse("tail-pin", "beam", "detail", 3.65, true, 0.3, T0[0], T0[1], 0.14, 0.14, { region: "steel", material: "painted-steel" }));
  }
  const pw = lv === 500 ? 0.2 : 0.14;
  list.push(bar("pitman", "pitman", 3.4, 0.6, T0[0] - pw / 2, C[1], T0[0] + pw / 2, T0[1], { region: "steel", material: "painted-steel" }));
  if (lv === 50) list.push(ellipse("pitman-eye", "pitman", "detail", 3.45, true, 0.3, C[0], C[1], 0.12, 0.12, { region: "steel", material: "painted-steel" }));
  list.push(bar("carrier", "rod", 3.3, 0.3, -3.3, 3.1, -2.7, 3.26, { region: "steel", material: "painted-steel" }));
  list.push(bar("gearbox", "gearbox", 3.2, 0.62, 2.0, 0.7, 3.4, 2.0, { r: 0.1, region: "gearbox", material: "cast-iron" }));
  list.push(bar("pedestal", "gearbox", 3.2, 0.5, 2.15, 0.55, 3.25, 0.7, { region: "gearbox", material: "cast-iron" }));
  list.push(poly("belt-guard", "gearbox", "body", 3.15, true, 0.45, [[3.4, 1.6], [4.45, 1.22], [4.45, 0.78], [3.4, 0.7]], true, { region: "steel", material: "painted-steel" }));
  const armW = lv === 500 ? 0.3 : 0.28;
  list.push(poly("counterweight", "crank", "body", 3.7, true, 0.4, weight(lv === 500 ? 1.1 : 1.12, 0.35), true, { region: "crank", material: "cast-iron" }));
  list.push(bar("crank-arm", "crank", 3.75, 0.58, C[0], C[1] - armW / 2, C[0] + CR + 0.1, C[1] + armW / 2, { region: "crank", material: "cast-iron" }));
  if (lv <= 100) list.push(ellipse("crank-pin", "crank", "detail", 3.8, true, 0.25, C[0] + CR, C[1], 0.1, 0.1, { region: "crank", material: "cast-iron" }));
  if (lv === 50) list.push(ellipse("crank-hub", "crank", "detail", 3.85, true, 0.3, C[0], C[1], 0.2, 0.2, { region: "crank", material: "cast-iron" }));
  list.push(bar("motor", "motor", 3.2, 0.68, 4.35, 0.65, 5.45, 1.35, { r: 0.08, region: "motor", material: "motor-steel" }));
  list.push(bar("motor-feet", "motor", 3.15, 0.5, 4.4, 0.55, 5.4, 0.65, { region: "motor", material: "motor-steel" }));
  list.push(bar("motor-cowl", "motor", 3.2, 0.5, 5.45, 0.75, 5.65, 1.25, { region: "motor", material: "motor-steel" }));
  if (lv === 50) {
    list.push(bar("terminal-box", "motor", 3.25, 0.5, 4.75, 1.35, 5.05, 1.55, { region: "motor", material: "motor-steel" }));
    list.push(ellipse("motor-sheave", "motor", "detail", 3.3, true, 0.3, 4.25, 1.0, 0.2, 0.2, { region: "steel", material: "painted-steel" }));
  }
  return list;
};

const dec = (silhouette, retained, merged, omitted, thin, hatching, styles, overlays) => ({ silhouette, retained, merged, omitted, thinElements: thin, hatching, styles, overlays });

const rep50 = authored(50, [20, 75], { body: 0.22, detail: 0.13, texture: null, glazing: null, ground: 0.35 }, common(50), dec(
  ["A 9 m wide, 5.5 m tall conventional oil pumpjack, 18 cm by 11 cm on paper: slab and skid, an A-frame Samson post, a walking beam with a curved horsehead on the left and the pitman rod down to a crank with a half-moon counterweight on the right, a belt guard to the electric motor and the wellhead with its polished rod and carrier bar on the left.", "Eine 9 m breite und 5,5 m hohe klassische Erdölpumpe, auf Papier 18 cm mal 11 cm: Platte und Rahmen, ein A-förmiger Lagerbock, ein Balancier mit gebogenem Pferdekopf links und der Schubstange hinunter zu einer Kurbel mit halbmondförmigem Gegengewicht rechts, ein Riemenschutz zum Elektromotor und der Bohrlochkopf mit Polierstange und Gestängeträger links."],
  ["All moving parts: beam with horsehead, pitman, crank with counterweight and pin, the carrier bar; the saddle bearing, tail pin, horsehead web, wellhead valve, stuffing box and flowline, the motor with its feet, cowl, terminal box and sheave.", "Alle bewegten Teile: Balancier mit Pferdekopf, Schubstange, Kurbel mit Gegengewicht und Zapfen, der Gestängeträger; das Sattellager, der Schwanzbolzen, das Steg des Pferdekopfs, Ventil, Stopfbuchse und Flowline des Bohrlochkopfs, der Motor mit Füßen, Lüfterhaube, Klemmkasten und Riemenscheibe."],
  ["The polished rod, the bridle ropes and the vertical rope run are one 50 mm line from the stuffing box to the tangent point of the horsehead: the rope always leaves the head at its leftmost point, only the carrier bar travels along it.", "Polierstange, Seile und senkrechter Seilverlauf sind eine 50 mm breite Linie von der Stopfbuchse bis zum Tangentenpunkt des Pferdekopfs: das Seil verlässt den Kopf immer an seinem linken Punkt, nur der Gestängeträger wandert an ihm entlang."],
  ["The second pitman rod and crank arm behind the first, bolts, ladders and railings, the brake, the belt itself and the gearbox oil gauge.", "Die zweite Schubstange und Kurbel hinter der ersten, Schrauben, Leitern und Geländer, die Bremse, der Riemen selbst und das Ölschauglas des Getriebes."],
  ["The polished rod is 50 mm (1.0 mm on paper), the pitman 0.14 m (2.8 mm), the carrier bar 0.16 m high (3.2 mm).", "Die Polierstange ist 50 mm breit (1,0 mm auf Papier), die Schubstange 0,14 m (2,8 mm), der Gestängeträger 0,16 m hoch (3,2 mm)."],
  ["None; the concrete slab is a light tone.", "Keine; die Betonplatte ist ein heller Ton."],
  ["Slab 0.82, skid 0.5, steel 0.55 to 0.78, gearbox 0.62, counterweight 0.4, motor 0.68; body pen 0.22 mm, detail 0.13 mm, ground 0.35 mm.", "Platte 0,82, Rahmen 0,5, Stahl 0,55 bis 0,78, Getriebe 0,62, Gegengewicht 0,4, Motor 0,68; Körperstift 0,22 mm, Detail 0,13 mm, Boden 0,35 mm."],
  ["Slab and skid, wellhead, steel members, gearbox, crank and motor are six regions: in the thermal view the motor and the gearbox warm while the pump runs and the wellhead carries the warm produced fluid; the beam, the pitman, the crank and the carrier bar move with their bones.", "Platte und Rahmen, Bohrlochkopf, Stahlteile, Getriebe, Kurbel und Motor sind sechs Regionen: in der Thermalansicht erwärmen sich Motor und Getriebe, solange die Pumpe läuft, und der Bohrlochkopf trägt das warme Fördermedium; Balancier, Schubstange, Kurbel und Gestängeträger bewegen sich mit ihren Knochen."],
));

const rep100 = authored(100, [75, 150], { body: 0.18, detail: 0.13, texture: null, glazing: null, ground: 0.25 }, common(100), dec(
  ["A 9 cm wide pumpjack: slab and skid, A-frame post, beam with horsehead, pitman, crank with counterweight, gearbox with belt guard, motor and wellhead.", "Eine 9 cm breite Erdölpumpe: Platte und Rahmen, A-Bock, Balancier mit Pferdekopf, Schubstange, Kurbel mit Gegengewicht, Getriebe mit Riemenschutz, Motor und Bohrlochkopf."],
  ["Every moving part and the main masses: post with brace, beam, horsehead, pitman, crank arm, pin and counterweight, gearbox with pedestal, belt guard, motor with feet and cowl.", "Alle bewegten Teile und die Hauptmassen: Bock mit Strebe, Balancier, Pferdekopf, Schubstange, Kurbelarm, Zapfen und Gegengewicht, Getriebe mit Sockel, Riemenschutz, Motor mit Füßen und Haube."],
  ["The wellhead valve, stuffing box and flowline are two blocks; the rope and the polished rod are one 50 mm line.", "Ventil, Stopfbuchse und Flowline des Bohrlochkopfs sind zwei Blöcke; Seil und Polierstange sind eine 50 mm breite Linie."],
  ["Saddle bearing, tail pin, pitman eye, crank hub, horsehead web, terminal box and sheave.", "Sattellager, Schwanzbolzen, Schubstangenauge, Kurbelnabe, Steg des Pferdekopfs, Klemmkasten und Riemenscheibe."],
  ["The polished rod is 50 mm (0.5 mm on paper), the pitman 0.14 m (1.4 mm).", "Die Polierstange ist 50 mm breit (0,5 mm auf Papier), die Schubstange 0,14 m (1,4 mm)."],
  ["None.", "Keine."],
  ["Same tones as 1:50; one body pen of 0.18 mm and a detail pen of 0.13 mm.", "Dieselben Töne wie bei 1:50; ein Körperstift von 0,18 mm und ein Detailstift von 0,13 mm."],
  ["The six regions stay; the moving parts keep their bones.", "Die sechs Regionen bleiben; die bewegten Teile behalten ihre Knochen."],
));

const rep200 = authored(200, [150, 350], { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 }, common(200), dec(
  ["A 4.5 cm wide pumpjack silhouette: the A-frame, the nodding beam with its head, the crank with the counterweight and the motor on one slab.", "Die Silhouette einer 4,5 cm breiten Erdölpumpe: der A-Bock, der nickende Balancier mit Kopf, die Kurbel mit Gegengewicht und der Motor auf einer Platte."],
  ["Slab with skid, both post legs and the brace, beam, horsehead, pitman, crank arm, pin and counterweight, gearbox, belt guard and motor.", "Platte mit Rahmen, beide Bockbeine und die Strebe, Balancier, Pferdekopf, Schubstange, Kurbelarm, Zapfen und Gegengewicht, Getriebe, Riemenschutz und Motor."],
  ["The wellhead is one block; the rope and the rod are one line.", "Der Bohrlochkopf ist ein Block; Seil und Stange sind eine Linie."],
  ["All details of 1:100 and the sheave.", "Alle Details von 1:100 und die Riemenscheibe."],
  ["The rod is 50 mm (0.25 mm on paper) and the pitman 0.14 m (0.7 mm).", "Die Stange ist 50 mm breit (0,25 mm auf Papier) und die Schubstange 0,14 m (0,7 mm)."],
  ["None.", "Keine."],
  ["Same tones; one body pen of 0.13 mm.", "Dieselben Töne; ein Körperstift von 0,13 mm."],
  ["The six regions stay.", "Die sechs Regionen bleiben."],
));

const rep500 = authored(500, [350, 1000], { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 }, common(500), dec(
  ["An 18 mm wide pumpjack mark: slab, A-frame, beam with head, crank with counterweight and motor.", "Ein 18 mm breites Zeichen einer Erdölpumpe: Platte, A-Bock, Balancier mit Kopf, Kurbel mit Gegengewicht und Motor."],
  ["Slab, both legs, beam with head, pitman, crank arm and counterweight, gearbox, belt guard, motor, wellhead block and rod.", "Platte, beide Beine, Balancier mit Kopf, Schubstange, Kurbelarm und Gegengewicht, Getriebe, Riemenschutz, Motor, Bohrlochkopf und Stange."],
  ["The brace is dropped, the legs are widened to 0.3 m.", "Die Strebe entfällt, die Beine sind auf 0,3 m verbreitert."],
  ["All details, the brace and the pedestal detail.", "Alle Details, die Strebe und das Sockeldetail."],
  ["The rod is widened to 0.1 m (0.2 mm on paper) and the pitman to 0.2 m (0.4 mm).", "Die Stange ist auf 0,1 m (0,2 mm auf Papier) und die Schubstange auf 0,2 m (0,4 mm) verbreitert."],
  ["None.", "Keine."],
  ["Same tones; one body pen of 0.13 mm.", "Dieselben Töne; ein Körperstift von 0,13 mm."],
  ["The six regions stay.", "Die sechs Regionen bleiben."],
));

const regions = [
  region("base", ["Foundation slab and steel skid", "Fundamentplatte und Stahlrahmen"], "surface", "passive", "concrete", 14, 7.8, "horizontal-plate-up", [single("slab", "Concrete 0.3 m", "Beton 0,3 m", "masonry", "concrete", 0.3, 0.2, 700000)], 700000, {}, "incropera-2018", 0.8),
  region("wellhead", ["Wellhead with valve and flowline", "Bohrlochkopf mit Ventil und Flowline"], "surface", "operating", "painted-steel", 2.6, 1.3, "cylinder", [single("wall", "Steel wall 20 mm", "Stahlwand 20 mm", "metal", "painted-steel", 0.02, 0.0004, 158000)], 158000, { operating: { key: "pumping", heatW: 300 } }),
  region("steel", ["Post, beam, horsehead, pitman, rod and guard", "Bock, Balancier, Pferdekopf, Schubstange, Stange und Schutz"], "surface", "passive", "painted-steel", 38, 5, "vertical-plate", [single("plate", "Steel plate 10 mm", "Stahlblech 10 mm", "metal", "painted-steel", 0.01, 0.0002, 79000)], 79000, {}, "incropera-2018", 0.7),
  region("gearbox", ["Gear reducer", "Getriebe"], "engine", "operating", "cast-iron", 6.5, 1.4, "vertical-plate", [single("casting", "Cast-iron housing 12 mm", "Gussgehäuse 12 mm", "metal", "cast-iron", 0.012, 0.0002, 86000)], 86000, { operating: { key: "pumping", heatW: 900 } }),
  region("crank", ["Crank arm and counterweight", "Kurbel und Gegengewicht"], "surface", "passive", "cast-iron", 6, 2.2, "vertical-plate", [single("casting", "Cast iron 80 mm", "Guss 80 mm", "metal", "cast-iron", 0.08, 0.0014, 570000)], 570000, {}, "incropera-2018", 0.7),
  region("motor", ["Electric motor", "Elektromotor"], "engine", "operating", "motor-steel", 2.6, 1.1, "cylinder", [single("frame", "Steel frame 6 mm", "Stahlgehäuse 6 mm", "metal", "motor-steel", 0.006, 0.0001, 47000)], 47000, { operating: { key: "pumping", heatW: 700 } }),
];

const bind = (pumping) => ({
  regions: {
    base: { exposedFraction: 0.8, sunlitFraction: 0.3, contact: { fraction: 0.2, tempC: 8 } },
    wellhead: { exposedFraction: 1, sunlitFraction: 0.3 },
    steel: { exposedFraction: 1, sunlitFraction: 0.5 },
    gearbox: { exposedFraction: 0.9, sunlitFraction: 0.4 },
    crank: { exposedFraction: 1, sunlitFraction: 0.5 },
    motor: { exposedFraction: 0.9, sunlitFraction: 0.4 },
  },
  emitters: {},
  operating: { pumping },
});

const ground = ["contact-l", "contact-r"];
const poseAt = (id, title, description, phi, pumping) => ({ ...pose(id, title, description, solveBeam(phi), ground, "authored", bind(pumping)) });
const poses = [
  poseAt("stopped", ["Stopped at the top of the stroke", "Stillstand am oberen Hubende"], ["The motor is off and the horsehead stands up at the top of the stroke; the counterweights hang down.", "Der Motor ist aus, der Pferdekopf steht oben am Hubende; die Gegengewichte hängen unten."], 270, 0),
  poseAt("mid-stroke", ["Running, mid-stroke", "Läuft, Hubmitte"], ["The crank stands horizontal, the beam is level and the rod moves down at full speed; motor and gearbox lose 1.6 kW together.", "Die Kurbel steht waagrecht, der Balancier liegt waagrecht und die Stange bewegt sich mit voller Geschwindigkeit nach unten; Motor und Getriebe verlieren zusammen 1,6 kW."], 180, 1),
  poseAt("bottom-of-stroke", ["Running, bottom of the stroke", "Läuft, unteres Hubende"], ["The crank pin is at the top, the horsehead is at its lowest point and the carrier bar is 0.96 m below mid-stroke.", "Der Kurbelzapfen steht oben, der Pferdekopf an seinem tiefsten Punkt und der Gestängeträger 0,96 m unter der Hubmitte."], 90, 1),
];

const contacts = (d) => ground.map((anchor) => ({ anchor, fromS: 0, toS: d }));
const PERIOD = 3;
const STEP = 0.25;
const cycleKeys = Array.from({ length: PERIOD / STEP + 1 }, (_, i) => ({ t: i * STEP, pose: "mid-stroke", ease: "smooth", bones: solveBeam(270 + (360 * i * STEP) / PERIOD) }));
const startKeys = [];
for (let i = 0; i <= 32; i += 1) {
  const t = i * STEP;
  const phi = t <= 1 ? 270 : t <= 4 ? 270 + 20 * (t - 1) ** 2 : 450 + 120 * (t - 4);
  startKeys.push({ t, pose: "mid-stroke", ease: "smooth", bones: solveBeam(phi) });
}

const clips = [
  { ...clip(
    "pump-cycle",
    ["Nodding cycle", "Pumpzyklus"],
    ["The crank turns once in three seconds (a shown speed twenty times the real 20 strokes per minute at slow motion) and the beam nods: the horsehead goes down, the carrier bar drops 0.96 m and comes back up.", "Die Kurbel dreht sich in drei Sekunden einmal (die gezeigte Geschwindigkeit entspricht den echten 20 Hüben je Minute) und der Balancier nickt: der Pferdekopf geht nach unten, der Gestängeträger sinkt 0,96 m und steigt wieder."],
    PERIOD,
    30,
    cycleKeys,
    contacts(PERIOD),
    [["downstroke", 0, 1.5, ["Downstroke of the rod", "Abwärtshub der Stange"]], ["upstroke", 1.5, 3, ["Upstroke of the rod", "Aufwärtshub der Stange"]]],
    [[0, "operating", ["Pumping at full speed", "Pumpen mit voller Drehzahl"], "pumping"]],
    [["operating:pumping", "step", [[0, 1], [PERIOD, 1]]]],
  ), loop: true },
  clip(
    "start-up",
    ["Starting the pump", "Anlaufen der Pumpe"],
    ["The motor starts after one second, the crank accelerates for three seconds to full speed and the pump runs on; motor and gearbox warm.", "Der Motor startet nach einer Sekunde, die Kurbel beschleunigt drei Sekunden lang auf volle Drehzahl, und die Pumpe läuft weiter; Motor und Getriebe erwärmen sich."],
    8,
    30,
    startKeys,
    contacts(8),
    [["standstill", 0, 1, ["Standstill", "Stillstand"]], ["run-up", 1, 4, ["Run-up", "Hochlauf"]], ["running", 4, 8, ["Full speed", "Volle Drehzahl"]]],
    [[1, "ignition", ["Motor switched on", "Motor eingeschaltet"], "pumping"]],
    [["operating:pumping", "step", [[0, 0], [1, 1], [8, 1]]]],
  ),
];

export const pumpjack = () => {
  const massKg = sum(sp.bones.map((b) => b.mass.kg));
  return wrap({
    id: "oil-pumpjack",
    category: "machinery",
    title: tx("Conventional Oil Pumpjack with Nodding Beam", "Klassische Erdölpumpe mit nickendem Balancier"),
    summary: tx(
      "A 9 m wide conventional beam pumpjack in side elevation: slab and skid, an A-frame post, a walking beam with a curved horsehead that nods over a crank with counterweight and pitman rod, an electric motor with belt guard and the wellhead with its polished rod.",
      "Eine 9 m breite klassische Balancier-Erdölpumpe in der Seitenansicht: Platte und Rahmen, ein A-förmiger Bock, ein Balancier mit gebogenem Pferdekopf, der über eine Kurbel mit Gegengewicht und Schubstange nickt, ein Elektromotor mit Riemenschutz und der Bohrlochkopf mit Polierstange.",
    ),
    tags: ["pumpjack", "nodding-donkey", "oil", "extraction", "primary-energy", "fossil", "machine", "oilfield", "rotating", "linkage"],
    provenance: {
      author: "Semio staffage content (G1)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [
        ref("api-11e-2013", "API Specification 11E (2013) Specification for Pumping Units (geometry of conventional beam pumping units: beam, horsehead, Samson post, pitman, crank and counterweights; rated stroke lengths of 0.9 to 4.3 m)."),
        ref("takacs-2015", "Takacs, G. (2015). Sucker-Rod Pumping Handbook, Gulf Professional Publishing (kinematics of the four-bar linkage of a conventional unit, stroke and beam angle)."),
        INCROPERA,
      ],
      notes: tx(
        "A small conventional beam pumping unit in the class of the API 114 size, seen from the side: concrete slab 7.8 m by 0.3 m, a 0.25 m steel skid, a 4.15 m high A-frame Samson post with the saddle bearing at 4.4 m, a 5 m walking beam with a horsehead of 3.0 m radius (front 3.0 m, rear 2.7 m) and a gear reducer with a 0.85 m crank. The pitman of 2.65 m links the crank pin to the beam tail; the stroke is 2 x 0.85 x 3.0 / 2.7 = 1.9 m at the polished rod. The origin is the ground at the wellhead axis on the foundation line; the wellhead stands 3.0 m left of the post. The rope leaves the horsehead tangentially at its leftmost point and hangs vertical, so the polished rod and the rope are drawn as one line and only the carrier bar travels along it. Every moving pose follows the closed four-bar linkage: for each crank angle the beam angle is solved so that the pitman keeps its length. All masses (18 t in all) and the 0.9 kW and 0.7 kW losses of gear reducer and motor are authored estimates for this class (about 20 kW rated motor power at 95 percent efficiency and 97 percent for the gears).",
        "Eine kleine klassische Balancier-Pumpeneinheit der Größenklasse API 114, von der Seite gesehen: Betonplatte 7,8 m mal 0,3 m, ein 0,25 m hoher Stahlrahmen, ein 4,15 m hoher A-förmiger Lagerbock mit dem Sattellager auf 4,4 m, ein 5 m langer Balancier mit einem Pferdekopf von 3,0 m Radius (vorn 3,0 m, hinten 2,7 m) und ein Getriebe mit einer 0,85 m langen Kurbel. Die Schubstange von 2,65 m verbindet den Kurbelzapfen mit dem Balancierende; der Hub an der Polierstange beträgt 2 x 0,85 x 3,0 / 2,7 = 1,9 m. Der Ursprung ist der Boden auf der Fundamentlinie; der Bohrlochkopf steht 3,0 m links vom Bock. Das Seil verlässt den Pferdekopf tangential an dessen linkem Punkt und hängt senkrecht, daher sind Polierstange und Seil als eine Linie gezeichnet, und nur der Gestängeträger wandert an ihr entlang. Jede bewegte Pose folgt dem geschlossenen Viergelenk: zu jedem Kurbelwinkel wird der Balancierwinkel so gelöst, dass die Schubstange ihre Länge behält. Alle Massen (zusammen 18 t) und die Verluste von 0,9 kW und 0,7 kW von Getriebe und Motor sind Schätzwerte für diese Klasse (rund 20 kW Motornennleistung bei 95 Prozent Wirkungsgrad und 97 Prozent der Zahnräder).",
      ),
    },
    dimensions: { heightM: 5.5, widthM: 9.2, depthM: 2.4, massKg },
    materials: [
      materialOf("concrete", "Concrete", "Beton", 0.9, 0.6, 2300, 1.8, 1000, SRC, "incropera-2018"),
      materialOf("painted-steel", "Painted structural steel", "Lackierter Baustahl", 0.9, 0.6, 7850, 50, 490, SRC, "incropera-2018"),
      materialOf("cast-iron", "Cast iron", "Gusseisen", 0.85, 0.7, 7200, 50, 500, SRC, "incropera-2018"),
      materialOf("motor-steel", "Steel motor frame", "Stahlgehäuse des Motors", 0.85, 0.6, 7800, 50, 490, SRC, "incropera-2018"),
    ],
    rig: { kind: "mechanical", root: "base", bones: sp.bones.map((b) => ({ ...b, mass: { ...b.mass } })) },
    anchors: [
      { id: "contact-l", bone: "base", x: -1.4, y: 0, kind: "contact", title: tx("Foundation bearing, left", "Fundamentauflager links"), support: "contact", normalDeg: 90, footprintLengthM: 1.0, footprintWidthM: 2.4, frictionCoefficient: 0.6 },
      { id: "contact-r", bone: "base", x: 5.4, y: 0, kind: "contact", title: tx("Foundation bearing, right", "Fundamentauflager rechts"), support: "contact", normalDeg: 90, footprintLengthM: 1.0, footprintWidthM: 2.4, frictionCoefficient: 0.6 },
    ],
    representations: { 500: rep500, 200: rep200, 100: rep100, 50: rep50 },
    symbols: [],
    poses,
    clips,
    thermal: {
      environment: { airTempC: 10, meanRadiantTempC: 8, relativeHumidity: 0.7, windMS: 2, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325 },
      passengers: [],
      operating: [{ id: "pumping", title: tx("Pumping", "Pumpbetrieb"), levels: [level(0, "Stopped", "Stillstand"), level(1, "Pumping at full speed", "Pumpen mit voller Drehzahl")] }],
      regions,
      emitters: [],
      assumptions: [
        tx("Motor and gear reducer lose 700 W and 900 W as heat while pumping; the wellhead carries 300 W from the warm produced fluid in the flowline.", "Motor und Getriebe verlieren beim Pumpen 700 W und 900 W als Wärme; der Bohrlochkopf trägt 300 W vom warmen Fördermedium in der Flowline."),
        tx("The steel members and the crank are passive: they follow the air, the wind and the sun and do not carry friction heat.", "Die Stahlteile und die Kurbel sind passiv: sie folgen der Luft, dem Wind und der Sonne und tragen keine Reibungswärme."),
      ],
    },
    structure: {
      gravityMS2: 9.80665,
      dynamics: "quasi-static",
      supports: [],
      assumptions: [
        tx("The unit rests on its slab at two bearings; the weight of the rod string and the fluid load on the polished rod are not applied.", "Die Einheit ruht mit ihrer Platte auf zwei Auflagern; das Gewicht des Gestänges und die Flüssigkeitslast an der Polierstange werden nicht angesetzt."),
      ],
      clinical: false,
    },
    presentation: { defaultPose: "mid-stroke", defaultClip: "pump-cycle", defaultScale: 100, viewpoints: ["elevation"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
};
