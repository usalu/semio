import { t, mat, layer, region, path, rect, ell, cap, line, rep, unavailable, regionBindings, operatingKeyDef, emitter, r4 } from "./lib.mjs";

const deg = Math.PI / 180;
const HOSE_N = 10, HOSE_L = 0.11, WAND_L = 0.78, NOZ_H = 0.045;
const INLET = [0.27, 0.125];

function hoseAngles(g, bulge) {
  let best = null;
  for (let a0 = -180; a0 <= 180; a0 += 1) {
    for (let d = -45; d <= 45; d += 0.5) {
      let x = INLET[0], y = INLET[1], mx = 0, my = 0;
      for (let i = 0; i < HOSE_N; i++) {
        const a = (a0 + i * d) * deg;
        x += HOSE_L * Math.cos(a); y += HOSE_L * Math.sin(a);
        if (i === 2) { mx = x - HOSE_L * 0.5 * Math.cos(a); my = y - HOSE_L * 0.5 * Math.sin(a); }
      }
      const err = Math.hypot(x - g[0], y - g[1]);
      const cx = g[0] - INLET[0], cy = g[1] - INLET[1];
      const side = Math.sign(cx * (my - INLET[1]) - cy * (mx - INLET[0]));
      if (side !== bulge) continue;
      if (!best || err < best.err) best = { err, a0, d };
    }
  }
  let { a0, d } = best, step = 0.5;
  const endpoint = (A, D) => { let x = INLET[0], y = INLET[1]; for (let i = 0; i < HOSE_N; i++) { const a = (A + i * D) * deg; x += HOSE_L * Math.cos(a); y += HOSE_L * Math.sin(a); } return Math.hypot(x - g[0], y - g[1]); };
  for (let k = 0; k < 60; k++) { for (const [da, dd] of [[step, 0], [-step, 0], [0, step], [0, -step]]) if (endpoint(a0 + da, d + dd) < endpoint(a0, d)) { a0 += da; d += dd; } step *= 0.8; }
  return { a0, d, err: endpoint(a0, d) };
}

function chain(px, wandDeg, bulge) {
  const dir = [Math.cos(wandDeg * deg), Math.sin(wandDeg * deg)];
  const g = [px - WAND_L * dir[0], NOZ_H - WAND_L * dir[1]];
  const h = hoseAngles(g, bulge);
  const abs = Array.from({ length: HOSE_N }, (_, i) => h.a0 + i * h.d);
  const bones = {};
  abs.forEach((a, i) => { bones[`hose-${i + 1}`] = { angleDeg: r4(a - (i ? abs[i - 1] : 0)) }; });
  bones.grip = { angleDeg: r4(wandDeg - abs[HOSE_N - 1]) };
  bones.nozzle = { angleDeg: r4(-wandDeg) };
  return { bones, g, err: h.err };
}

const plasticRed = mat("abs-red", t("Red ABS housing", "Rotes ABS-Gehäuse"), 0.9, 0.7, 1050, 0.17, 1400, "authored-estimate", "iso-10456-2007");
const plasticDark = mat("abs-grey", t("Dark grey ABS", "Dunkelgraues ABS"), 0.9, 0.85, 1050, 0.17, 1400, "authored-estimate", "iso-10456-2007");
const hosePvc = mat("hose-pvc", t("Corrugated PVC hose, grey", "Gewellter PVC-Schlauch, grau"), 0.9, 0.7, 1200, 0.16, 1000, "authored-estimate", "iso-10456-2007");
const steelTube = mat("tube-steel", t("Chromed steel telescopic tube", "Verchromtes Stahl-Teleskoprohr"), 0.1, 0.35, 7850, 50, 450, "authored-estimate", "incropera-2018");
const rubberM = mat("rubber-wheel", t("Rubber tyre", "Gummireifen"), 0.9, 0.9, 1100, 0.2, 2000, "authored-estimate", "iso-10456-2007");
const bristle = mat("nylon-bristle", t("Nylon brush strip", "Nylon-Bürstenleiste"), 0.9, 0.9, 1100, 0.25, 1700, "authored-estimate", "iso-10456-2007");
const materials = [plasticRed, plasticDark, hosePvc, steelTube, rubberM, bristle];

const regionIds = ["body-shell", "lid", "exhaust", "wheels", "hose", "wand", "grip", "nozzle", "brush"];
const hoseBones = Array.from({ length: HOSE_N }, (_, i) => `hose-${i + 1}`);

const shellPath = "M -0.215 0.045 C -0.238 0.045 -0.252 0.07 -0.252 0.1 L -0.252 0.17 C -0.252 0.19 -0.238 0.2 -0.218 0.2 L 0.2 0.2 C 0.23 0.2 0.255 0.175 0.262 0.145 L 0.262 0.1 C 0.256 0.07 0.24 0.045 0.215 0.045 Z";
const lidPath = "M -0.24 0.2 C -0.24 0.24 -0.205 0.268 -0.15 0.268 L 0.11 0.268 C 0.2 0.268 0.248 0.245 0.258 0.2 Z";
const handlePath = "M -0.07 0.266 L -0.058 0.296 C -0.056 0.303 -0.05 0.305 -0.04 0.305 L 0.06 0.305 C 0.07 0.305 0.076 0.303 0.078 0.296 L 0.09 0.266 Z";
const nozzlePath = "M -0.09 -0.045 L 0.17 -0.045 C 0.185 -0.045 0.19 -0.035 0.19 -0.025 L 0.19 -0.005 C 0.19 0.005 0.18 0.012 0.17 0.015 L 0.06 0.028 L 0.035 0.05 L -0.035 0.05 L -0.06 0.022 L -0.09 -0.01 Z";
const shell200 = "M -0.215 0.045 C -0.238 0.045 -0.252 0.07 -0.252 0.1 L -0.252 0.2 C -0.24 0.245 -0.205 0.268 -0.15 0.268 L 0.11 0.268 C 0.2 0.268 0.255 0.235 0.262 0.145 L 0.262 0.1 C 0.256 0.07 0.24 0.045 0.215 0.045 Z";
const noz200 = "M -0.09 -0.045 L 0.19 -0.045 L 0.19 0.01 L 0.06 0.03 L 0.03 0.05 L -0.04 0.05 L -0.09 0 Z";

const hoseCaps = (r, z, o = {}) => hoseBones.map((b) => cap(`${b}-tube`, b, z, 0.35, 0, 0, HOSE_L, 0, r, r, { region: "hose", material: "hose-pvc" }));
const hoseRibs = () => hoseBones.flatMap((b) => [0.03, 0.08].map((x, i) => line(`${b}-rib-${i}`, b, 2.3, [[x, -0.019], [x, 0.019]], { region: "hose", role: "texture", tone: 0.15 })));

const el50 = [
  ...hoseCaps(0.019, 2.2), ...hoseRibs(),
  cap("cuff", "hose-1", 2.4, 0.25, -0.03, 0, 0.012, 0, 0.026, 0.026, { region: "hose", material: "abs-grey" }),
  cap("wand-tube", "wand", 2.5, 0.8, 0, 0, WAND_L, 0, 0.0175, 0.0175, { region: "wand", material: "tube-steel" }),
  line("wand-joint", "wand", 2.6, [[0.4, -0.0175], [0.4, 0.0175]], { region: "wand" }),
  line("wand-shine", "wand", 2.6, [[0.04, 0.008], [WAND_L - 0.04, 0.008]], { region: "wand", tone: 0.95 }),
  cap("grip-sleeve", "grip", 2.7, 0.3, 0, 0, 0.14, 0, 0.026, 0.026, { region: "grip", material: "abs-grey" }),
  ell("air-slide", "grip", 2.8, 0.7, 0.06, 0.026, 0.014, 0.007, { region: "grip" }),
  path("nozzle", "nozzle", 2.5, 0.45, nozzlePath, { region: "nozzle", material: "abs-grey" }),
  rect("brush", "nozzle", 2.6, 0.15, 0.0, -0.045, 0.18, 0.012, { region: "brush", material: "nylon-bristle" }),
  ell("swivel", "nozzle", 2.7, 0.5, 0, 0.0, 0.028, 0.028, { region: "nozzle" }),
  line("nozzle-slot", "nozzle", 2.7, [[0.06, 0.0], [0.17, 0.0]], { region: "nozzle", tone: 0.1 }),
  path("shell", "body", 3, 0.5, shellPath, { region: "body-shell", material: "abs-red" }),
  line("shell-seam", "body", 3.2, [[-0.245, 0.12], [0.26, 0.12]], { region: "body-shell" }),
  path("lid", "body", 3.1, 0.62, lidPath, { region: "lid", material: "abs-red" }),
  path("handle", "body", 3.0, 0.62, handlePath, { region: "lid", material: "abs-red" }),
  rect("handle-opening", "body", 3.3, 0.1, -0.04, 0.275, 0.09, 0.016, { role: "detail", filled: false, rr: 0.007, region: "lid" }),
  ell("rewind-button", "body", 3.3, 0.8, 0.175, 0.256, 0.014, 0.007, { region: "lid" }),
  rect("exhaust-grille", "body", 3.4, 0.25, -0.262, 0.09, 0.02, 0.1, { rr: 0.005, region: "exhaust", material: "abs-grey" }),
  ...[0.1, 0.12, 0.14, 0.16, 0.18].map((y, i) => line(`louvre-${i}`, "body", 3.5, [[-0.262, y], [-0.242, y]], { region: "exhaust", tone: 0.8 })),
  rect("front-bumper", "body", 3.3, 0.25, 0.245, 0.06, 0.02, 0.1, { rr: 0.008, region: "body-shell", material: "abs-grey" }),
  ell("rear-wheel", "wheel", 3.6, 0.2, 0, 0, 0.06, 0.06, { region: "wheels", material: "rubber-wheel" }),
  ell("rear-hub", "wheel", 3.7, 0.75, 0, 0, 0.026, 0.026, { region: "wheels", material: "abs-grey" }),
  line("rear-spoke-a", "wheel", 3.8, [[-0.022, 0], [0.022, 0]], { region: "wheels", tone: 0.2 }),
  line("rear-spoke-b", "wheel", 3.8, [[0, -0.022], [0, 0.022]], { region: "wheels", tone: 0.2 }),
  ell("caster-wheel", "caster", 3.6, 0.2, 0, 0, 0.025, 0.025, { region: "wheels", material: "rubber-wheel" }),
  ell("caster-hub", "caster", 3.7, 0.7, 0, 0, 0.009, 0.009, { region: "wheels", material: "abs-grey" }),
];

const el100 = [
  ...hoseCaps(0.019, 2.2),
  cap("cuff", "hose-1", 2.4, 0.25, -0.03, 0, 0.012, 0, 0.026, 0.026, { region: "hose", material: "abs-grey" }),
  cap("wand-tube", "wand", 2.5, 0.8, 0, 0, WAND_L, 0, 0.0175, 0.0175, { region: "wand", material: "tube-steel" }),
  cap("grip-sleeve", "grip", 2.7, 0.3, 0, 0, 0.14, 0, 0.026, 0.026, { region: "grip", material: "abs-grey" }),
  path("nozzle", "nozzle", 2.5, 0.45, nozzlePath, { region: "nozzle", material: "abs-grey" }),
  rect("brush", "nozzle", 2.6, 0.15, 0.0, -0.045, 0.18, 0.014, { region: "brush", material: "nylon-bristle" }),
  path("shell", "body", 3, 0.5, shellPath, { region: "body-shell", material: "abs-red" }),
  path("lid", "body", 3.1, 0.62, lidPath, { region: "lid", material: "abs-red" }),
  path("handle", "body", 3.0, 0.62, handlePath, { region: "lid", material: "abs-red" }),
  line("shell-seam", "body", 3.2, [[-0.245, 0.12], [0.26, 0.12]], { region: "body-shell" }),
  rect("exhaust-grille", "body", 3.4, 0.25, -0.262, 0.09, 0.022, 0.1, { rr: 0.005, region: "exhaust", material: "abs-grey" }),
  ell("rear-wheel", "wheel", 3.6, 0.2, 0, 0, 0.06, 0.06, { region: "wheels", material: "rubber-wheel" }),
  ell("rear-hub", "wheel", 3.7, 0.75, 0, 0, 0.024, 0.024, { region: "wheels", material: "abs-grey" }),
  ell("caster-wheel", "caster", 3.6, 0.2, 0, 0, 0.025, 0.025, { region: "wheels", material: "rubber-wheel" }),
];

const el200 = [
  ...hoseBones.map((b) => cap(`${b}-tube`, b, 2.2, 0.35, 0, 0, HOSE_L, 0, 0.024, 0.024, { region: "hose", material: "hose-pvc" })),
  cap("wand-tube", "wand", 2.5, 0.8, 0, 0, WAND_L, 0, 0.024, 0.024, { region: "wand", material: "tube-steel" }),
  cap("grip-sleeve", "grip", 2.7, 0.3, 0, 0, 0.14, 0, 0.03, 0.03, { region: "grip", material: "abs-grey" }),
  path("nozzle", "nozzle", 2.5, 0.45, noz200, { regions: ["nozzle", "brush"], material: "abs-grey" }),
  path("shell", "body", 3, 0.5, shell200, { regions: ["body-shell", "lid", "exhaust"], material: "abs-red" }),
  ell("rear-wheel", "wheel", 3.6, 0.2, 0, 0, 0.06, 0.06, { region: "wheels", material: "rubber-wheel" }),
];

const dec = {
  50: {
    silhouette: t("A canister vacuum cleaner 1.5 m long at 30 mm on the sheet: a red canister with lid and carry handle on one big rear wheel and a front caster, a corrugated ten-segment hose rising to the grip of a telescopic tube that ends in a swivelling floor nozzle with a brush strip.", "Ein Bodenstaubsauger, 1,5 m lang und 30 mm auf dem Blatt: ein roter Kessel mit Deckel und Tragegriff auf einem großen Hinterrad und einer Lenkrolle, ein gewellter Schlauch aus zehn Gliedern, der zum Griff eines Teleskoprohrs ansteigt, das in einer schwenkbaren Bodendüse mit Bürstenleiste endet."),
    retained: t("The overall length and the 0.78 m wand exactly; hose cuff, corrugation ribs, wand joint and highlight, grip sleeve with air slide, nozzle swivel and slot, brush strip, louvred exhaust grille, front bumper, lid seam, rewind button, handle opening, wheel hub with spokes.", "Gesamtlänge und das 0,78 m lange Rohr exakt; Schlauchmuffe, Wellenrippen, Rohrstoß und Lichtkante, Griffhülse mit Nebenluftschieber, Düsengelenk und Schlitz, Bürstenleiste, geschlitztes Abluftgitter, Frontpuffer, Deckelfuge, Kabelaufwickeltaste, Griffloch, Radnabe mit Speichen."),
    merged: t("The two rear wheels are one wheel in side view; the canister's lower edge and the dust-bag cover are one shell.", "Die zwei Hinterräder sind in der Seitenansicht ein Rad; Kesselunterkante und Staubbeutelabdeckung sind eine Schale."),
    omitted: t("Mains cable and plug, filter flap, parking hook, telescopic lock button, label.", "Netzkabel und Stecker, Filterklappe, Parkhaken, Teleskopverriegelung, Typenschild."),
    thinElements: t("Wand 0.035 m = 0.7 mm and hose 0.038 m = 0.76 mm are true; the corrugation ribs are 0.13 mm strokes at 1.1 mm spacing.", "Rohr 0,035 m = 0,7 mm und Schlauch 0,038 m = 0,76 mm sind wahr; die Wellenrippen sind 0,13-mm-Striche im Abstand 1,1 mm."),
    hatching: t("None; the ribs are texture strokes.", "Keine; die Rippen sind Texturstriche."),
    styles: t("Canister 0.5, lid 0.62, hose 0.35, chromed wand 0.8, grip 0.3, nozzle 0.45, brush 0.15, tyres 0.2; body pen 0.35 mm, detail 0.18 mm, texture 0.13 mm.", "Kessel 0,5, Deckel 0,62, Schlauch 0,35, verchromtes Rohr 0,8, Griff 0,3, Düse 0,45, Bürste 0,15, Reifen 0,2; Körperstift 0,35 mm, Detail 0,18 mm, Textur 0,13 mm."),
    overlays: t("Nine regions: the thermal view paints the warm canister shell, lid and exhaust grille above cool hose, wand and nozzle.", "Neun Regionen: Die Thermalansicht färbt Kesselschale, Deckel und Abluftgitter warm über kühlem Schlauch, Rohr und Düse."),
  },
  100: {
    silhouette: t("A canister vacuum cleaner 15 mm long: red canister with a lid and handle bump, one rear wheel, a hose of ten segments, a straight chromed wand with a grip sleeve and a floor nozzle with a brush foot.", "Ein Bodenstaubsauger, 15 mm lang: roter Kessel mit Deckel und Griffbuckel, ein Hinterrad, ein Schlauch aus zehn Gliedern, ein gerades verchromtes Rohr mit Griffhülse und eine Bodendüse mit Bürstenfuß."),
    retained: t("Wand, hose segments, grip, nozzle with brush, canister with seam, lid, handle and exhaust grille, rear wheel with hub, caster.", "Rohr, Schlauchglieder, Griff, Düse mit Bürste, Kessel mit Fuge, Deckel, Griff und Abluftgitter, Hinterrad mit Nabe, Lenkrolle."),
    merged: t("Hose cuff and hose are one tube; the front bumper joins the shell; hub spokes and louvres are left out.", "Schlauchmuffe und Schlauch sind ein Rohr; der Frontpuffer geht in der Schale auf; Nabenspeichen und Lamellen entfallen."),
    omitted: t("Corrugation ribs, air slide, nozzle slot, rewind button, handle opening, caster hub, cable.", "Wellenrippen, Nebenluftschieber, Düsenschlitz, Aufwickeltaste, Griffloch, Rollennabe, Kabel."),
    thinElements: t("Hose 0.038 m and wand 0.035 m are 0.38 and 0.35 mm wide and true; the brush strip is thickened from 0.01 m to 0.014 m (0.14 mm).", "Schlauch 0,038 m und Rohr 0,035 m sind 0,38 und 0,35 mm breit und wahr; die Bürstenleiste ist von 0,01 m auf 0,014 m (0,14 mm) verdickt."),
    hatching: t("None.", "Keine."),
    styles: t("As at 1:50; body pen 0.25 mm, detail 0.13 mm.", "Wie bei 1:50; Körperstift 0,25 mm, Detail 0,13 mm."),
    overlays: t("Every region of the physical model has a shape: the thermal view shows shell, lid, grille, hose, wand, grip, nozzle, brush and wheels.", "Jede Region des physikalischen Modells hat eine Form: Die Thermalansicht zeigt Schale, Deckel, Gitter, Schlauch, Rohr, Griff, Düse, Bürste und Räder."),
  },
  200: {
    silhouette: t("A canister vacuum cleaner 7.5 mm long: one canister block with a rounded lid line, a wheel disc, ten thick hose segments, a wand rod with a grip and one wedge-shaped nozzle.", "Ein Bodenstaubsauger, 7,5 mm lang: ein Kesselblock mit gerundeter Deckellinie, eine Radscheibe, zehn dicke Schlauchglieder, ein Rohr mit Griff und eine keilförmige Düse."),
    retained: t("Length, wand length 0.78 m, height 0.3 m of the canister, nozzle length.", "Länge, Rohrlänge 0,78 m, Kesselhöhe 0,3 m, Düsenlänge."),
    merged: t("Shell, lid, handle and grille form one block (regions listed); nozzle and brush are one wedge; the caster joins the shell.", "Schale, Deckel, Griff und Gitter bilden einen Block (Regionen gelistet); Düse und Bürste sind ein Keil; die Lenkrolle geht in der Schale auf."),
    omitted: t("Corrugation, hub, caster, seams, louvres, buttons.", "Wellung, Nabe, Lenkrolle, Fugen, Lamellen, Tasten."),
    thinElements: t("Hose and wand are 0.035-0.038 m = 0.18 mm in reality: thickened to 0.048 m (0.24 mm) to reach the 0.2 mm feature width; this is the only enlargement.", "Schlauch und Rohr sind real 0,035-0,038 m = 0,18 mm: auf 0,048 m (0,24 mm) verdickt, um die Merkmalsbreite 0,2 mm zu erreichen; das ist die einzige Vergrößerung."),
    hatching: t("None.", "Keine."),
    styles: t("Canister 0.5, hose 0.35, wand 0.8, nozzle 0.45; body pen 0.18 mm.", "Kessel 0,5, Schlauch 0,35, Rohr 0,8, Düse 0,45; Körperstift 0,18 mm."),
    overlays: t("The canister block lists shell, lid and exhaust, the nozzle block nozzle and brush; the thermal overlay paints them with area-weighted values.", "Der Kesselblock listet Schale, Deckel und Abluft, der Düsenblock Düse und Bürste; die Thermalansicht färbt sie mit flächengewichteten Werten."),
  },
};

const mk = (den, lo, hi, pens, elev) => rep(den, lo, hi, pens, [], elev, dec[den]);
const reps = {
  500: unavailable(500,
    t("A canister vacuum cleaner is 1.5 m x 0.78 m = 3 x 1.6 mm on paper at 1:500: wand (0.035 m = 0.07 mm), hose (0.038 m = 0.08 mm) and the nozzle (0.1 m = 0.2 mm) cannot be drawn without inflating them to six times their width, which would turn the appliance into a bar and a block.", "Ein Bodenstaubsauger misst 1,5 m x 0,78 m = 3 x 1,6 mm auf dem Papier bei 1:500: Rohr (0,035 m = 0,07 mm), Schlauch (0,038 m = 0,08 mm) und Düse (0,1 m = 0,2 mm) lassen sich nur auf das Sechsfache ihrer Breite aufblasen, was das Gerät zu einem Balken und einem Block machte."),
    t("Wand and hose (0.035-0.038 m = 0.07-0.08 mm)", "Rohr und Schlauch (0,035-0,038 m = 0,07-0,08 mm)"), 0.035,
    [{ kind: "scale", denominator: 200 }]),
  200: mk(200, 150, 350, { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 }, el200),
  100: mk(100, 75, 150, { body: 0.25, detail: 0.13, texture: null, glazing: null, ground: 0.35 }, el100),
  50: mk(50, 20, 75, { body: 0.35, detail: 0.18, texture: 0.13, glazing: null, ground: 0.5 }, el50),
};

const bone = (id, parent, title, x, y, joint, mass, length = 0.1) => ({ id, parent, title, x, y, angleDeg: 0, lengthM: length, joint, ...(mass ? { mass: { distribution: "point", source: "authored-estimate", ...mass } } : {}) });
const bones = [
  bone("body", null, t("Canister with lid and handle", "Kessel mit Deckel und Griff"), 0, 0, { type: "fixed" }, { kg: 3.1, comX: 0, comY: 0.14, reference: "en-60312" }, 0.5),
  bone("wheel", "body", t("Rear wheel pair", "Hinterradpaar"), -0.12, 0.06, { type: "revolute", minDeg: -3600, maxDeg: 3600 }, { kg: 0.2, comX: 0, comY: 0 }, 0.06),
  bone("caster", "body", t("Front caster", "Lenkrolle"), 0.18, 0.025, { type: "revolute", minDeg: -3600, maxDeg: 3600 }, { kg: 0.1, comX: 0, comY: 0 }, 0.025),
  ...hoseBones.map((id, i) => bone(id, i ? hoseBones[i - 1] : "body", t(`Hose segment ${i + 1}`, `Schlauchglied ${i + 1}`), i ? HOSE_L : INLET[0], i ? 0 : INLET[1], { type: "revolute", minDeg: -360, maxDeg: 360 }, { kg: 0.06, comX: HOSE_L / 2, comY: 0 }, HOSE_L)),
  bone("grip", hoseBones[HOSE_N - 1], t("Grip sleeve with air slide", "Griffhülse mit Nebenluftschieber"), HOSE_L, 0, { type: "revolute", minDeg: -360, maxDeg: 360 }, { kg: 0.15, comX: 0.07, comY: 0 }, 0.14),
  bone("wand", "grip", t("Telescopic wand", "Teleskoprohr"), 0, 0, { type: "fixed" }, { kg: 0.45, comX: WAND_L / 2, comY: 0 }, WAND_L),
  bone("nozzle", "wand", t("Floor nozzle", "Bodendüse"), WAND_L, 0, { type: "revolute", minDeg: -360, maxDeg: 360 }, { kg: 0.5, comX: 0.05, comY: -0.01 }, 0.28),
];

const anchors = [
  { id: "wheel-contact", bone: "wheel", x: 0, y: -0.06, kind: "contact", title: t("Rear wheels on the floor", "Hinterräder auf dem Boden"), support: "contact", normalDeg: 90, footprintLengthM: 0.03, footprintWidthM: 0.05, frictionCoefficient: 0.6 },
  { id: "caster-contact", bone: "caster", x: 0, y: -0.025, kind: "contact", title: t("Front caster on the floor", "Lenkrolle auf dem Boden"), support: "contact", normalDeg: 90, footprintLengthM: 0.02, footprintWidthM: 0.02, frictionCoefficient: 0.5 },
  { id: "nozzle-contact", bone: "nozzle", x: 0.05, y: -NOZ_H, kind: "contact", title: t("Floor nozzle on the floor", "Bodendüse auf dem Boden"), support: "contact", normalDeg: 90, footprintLengthM: 0.2, footprintWidthM: 0.28, frictionCoefficient: 0.4 },
  { id: "exhaust-port", bone: "body", x: -0.262, y: 0.14, kind: "emitter", title: t("Exhaust air behind the canister", "Abluft hinter dem Kessel") },
];

const refs = [
  { id: "en-60312", citation: "IEC 60312-1 / EN 60312-1 Vacuum cleaners for household use - Methods for measuring the performance (product class: canister vacuum cleaners, 4-6 kg)." },
  { id: "eu-666-2013", citation: "Commission Regulation (EU) No 666/2013 implementing Directive 2009/125/EC with regard to ecodesign requirements for vacuum cleaners (rated input power limit 900 W from 2017, 1600 W before; the 700 W of this asset are inside it)." },
  { id: "incropera-2018", citation: "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (emissivities, convection, jets)." },
  { id: "iso-10456-2007", citation: "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values (polymers, steel)." },
];

const mat1 = (id, title, kind, m, d) => [layer(id, title, kind, m, d, "authored-estimate", "iso-10456-2007")];
const regions = [
  region({ id: "body-shell", title: t("Canister shell (motor housing and bag chamber)", "Kesselschale (Motorraum und Beutelkammer)"), kind: "surface", model: "operating", material: plasticRed, areaM2: 0.42, L: 0.3, geometry: "vertical-plate", svf: 0.6, layers: mat1("shell", t("ABS shell 2.5 mm", "ABS-Schale 2,5 mm"), "polymer", plasticRed, 0.0025), operating: { key: "motor", heatW: 110 }, reference: "en-60312" }),
  region({ id: "lid", title: t("Lid and carry handle", "Deckel und Tragegriff"), kind: "surface", model: "operating", material: plasticRed, areaM2: 0.15, L: 0.4, geometry: "horizontal-plate-up", svf: 0.9, layers: mat1("lid", t("ABS lid 2.5 mm", "ABS-Deckel 2,5 mm"), "polymer", plasticRed, 0.0025), operating: { key: "motor", heatW: 40 }, reference: "en-60312" }),
  region({ id: "exhaust", title: t("Exhaust grille", "Abluftgitter"), kind: "heater", model: "operating", material: plasticDark, areaM2: 0.02, L: 0.1, geometry: "vertical-plate", svf: 0.5, layers: mat1("grille", t("ABS grille 2 mm", "ABS-Gitter 2 mm"), "polymer", plasticDark, 0.002), operating: { key: "motor", heatW: 30 }, reference: "en-60312" }),
  region({ id: "wheels", title: t("Wheels and caster", "Räder und Lenkrolle"), kind: "tyre", model: "passive", material: rubberM, areaM2: 0.05, L: 0.12, geometry: "cylinder", svf: 0.3, layers: mat1("tyre", t("Rubber tyre 6 mm", "Gummireifen 6 mm"), "polymer", rubberM, 0.006), reference: "iso-10456-2007" }),
  region({ id: "hose", title: t("Corrugated hose with cuff", "Gewellter Schlauch mit Muffe"), kind: "surface", model: "passive", material: hosePvc, areaM2: 0.14, L: 0.04, geometry: "cylinder", svf: 0.5, layers: mat1("hose", t("PVC hose wall 0.6 mm", "PVC-Schlauchwand 0,6 mm"), "polymer", hosePvc, 0.0006), reference: "iso-10456-2007" }),
  region({ id: "wand", title: t("Telescopic wand", "Teleskoprohr"), kind: "surface", model: "passive", material: steelTube, areaM2: 0.09, L: 0.035, geometry: "cylinder", svf: 0.5, layers: mat1("tube", t("Steel tube wall 0.6 mm", "Stahlrohrwand 0,6 mm"), "metal", steelTube, 0.0006), reference: "iso-10456-2007" }),
  region({ id: "grip", title: t("Grip sleeve", "Griffhülse"), kind: "surface", model: "passive", material: plasticDark, areaM2: 0.02, L: 0.05, geometry: "cylinder", svf: 0.5, layers: mat1("sleeve", t("ABS sleeve 3 mm", "ABS-Hülse 3 mm"), "polymer", plasticDark, 0.003), reference: "iso-10456-2007" }),
  region({ id: "nozzle", title: t("Floor nozzle housing", "Bodendüsengehäuse"), kind: "surface", model: "passive", material: plasticDark, areaM2: 0.09, L: 0.28, geometry: "horizontal-plate-up", svf: 0.8, layers: mat1("noz", t("ABS housing 2.5 mm", "ABS-Gehäuse 2,5 mm"), "polymer", plasticDark, 0.0025), reference: "iso-10456-2007" }),
  region({ id: "brush", title: t("Brush strip", "Bürstenleiste"), kind: "surface", model: "passive", material: bristle, areaM2: 0.01, L: 0.2, geometry: "horizontal-plate-down", svf: 0.1, layers: mat1("bristle", t("Nylon bristles 8 mm", "Nylonborsten 8 mm"), "polymer", bristle, 0.008), reference: "iso-10456-2007" }),
];

const emitters = [emitter({ id: "exhaust-air", kind: "vent", title: t("Warm exhaust air behind the canister", "Warme Abluft hinter dem Kessel"), anchor: "exhaust-port", dir: 180, d: 0.09, key: "motor", flow: 0.03, rise: 14.4, vapour: 0, reference: "en-60312" })];
const keyMotor = operatingKeyDef("motor", t("Suction motor", "Saugmotor"), [[0, "Off", "Aus"], [0.45, "Low suction (315 W)", "Schwache Saugstufe (315 W)"], [1, "Rated (700 W)", "Nennleistung (700 W)"]]);

const nozzleStepPoses = [
  ["parked", t("Parked", "Geparkt"), t("Switched off: the wand stands behind the canister with the nozzle on the floor and the hose arcs over to the grip.", "Ausgeschaltet: Das Rohr steht hinter dem Kessel, die Düse auf dem Boden, der Schlauch spannt sich im Bogen zum Griff."), -0.45, -102, 1, 0],
  ["running", t("Running, nozzle pushed out", "Läuft, Düse vorgeschoben"), t("Rated 700 W: the nozzle is pushed 1.05 m away from the canister at a 62 degree wand, the hose hangs in a loop and the exhaust air leaves behind the canister.", "Nennleistung 700 W: Die Düse ist 1,05 m vom Kessel vorgeschoben bei 62 Grad Rohrneigung, der Schlauch hängt in einer Schlaufe und die Abluft tritt hinter dem Kessel aus."), 1.05, -62, -1, 1],
  ["running-pulled", t("Running, nozzle pulled back", "Läuft, Düse zurückgezogen"), t("Rated 700 W at the other end of the stroke: the nozzle is 0.78 m from the canister and the wand stands steeper.", "Nennleistung 700 W am anderen Ende des Hubs: Die Düse ist 0,78 m vom Kessel entfernt und das Rohr steht steiler."), 0.78, -70, -1, 1],
];

const poses = nozzleStepPoses.map(([id, title, description, px, wandDeg, bulge, motor]) => {
  const c = chain(px, wandDeg, bulge);
  if (c.err > 0.002) throw new Error(`hose does not reach in ${id}: ${c.err}`);
  return {
    id, title, description, bones: c.bones, ground: "auto", contacts: ["wheel-contact", "caster-contact", "nozzle-contact"], loads: [],
    thermal: { operating: { motor }, regions: regionBindings(regionIds, {}, { brush: { exposedFraction: 0.3 }, wheels: { exposedFraction: 0.5, contact: { fraction: 0.05, tempC: 20 } } }), emitters: { "exhaust-air": motor } },
  };
});

const contactsFor = (d) => ["wheel-contact", "caster-contact", "nozzle-contact"].map((anchor) => ({ anchor, fromS: 0, toS: d }));
const clips = [
  { id: "floor-stroke", title: t("Vacuuming stroke", "Saugbewegung"), description: t("The nozzle is pushed out and pulled back every four seconds while the motor runs at 700 W.", "Die Düse wird alle vier Sekunden vorgeschoben und zurückgezogen, während der Motor mit 700 W läuft."), durationS: 4, fps: 15, loop: true, rootMotion: "none",
    keys: [{ t: 0, pose: "running", ease: "smooth" }, { t: 2, pose: "running-pulled", ease: "smooth" }],
    contacts: contactsFor(4), phases: [{ id: "push", fromS: 0, toS: 2, title: t("Push out", "Vorschieben") }, { id: "pull", fromS: 2, toS: 4, title: t("Pull back", "Zurückziehen") }],
    events: [{ t: 0, kind: "operating", title: t("Motor at rated power", "Motor auf Nennleistung"), target: "motor" }],
    tracks: [{ target: "operating:motor", ease: "step", keys: [{ t: 0, v: 1 }, { t: 4, v: 1 }] }, { target: "emitter:exhaust-air", ease: "step", keys: [{ t: 0, v: 1 }, { t: 4, v: 1 }] }] },
  { id: "run-and-park", title: t("Switch on, vacuum, switch off", "Einschalten, saugen, ausschalten"), description: t("From parked the motor starts, the nozzle goes out for ten minutes, then the motor is switched off and the cleaner is parked while the shell cools.", "Aus der Parkstellung läuft der Motor an, die Düse geht zehn Minuten hinaus, dann wird der Motor abgeschaltet und das Gerät geparkt, während die Schale abkühlt."), durationS: 1500, fps: 1, loop: false, rootMotion: "none",
    keys: [{ t: 0, pose: "parked", ease: "step" }, { t: 5, pose: "running", ease: "step" }, { t: 605, pose: "parked", ease: "step" }, { t: 1500, pose: "parked", ease: "step" }],
    contacts: contactsFor(1500),
    phases: [{ id: "parked", fromS: 0, toS: 5, title: t("Parked", "Geparkt") }, { id: "running", fromS: 5, toS: 605, title: t("Vacuuming", "Saugt") }, { id: "cool-down", fromS: 605, toS: 1500, title: t("Off, cooling", "Aus, kühlt ab") }],
    events: [{ t: 5, kind: "ignition", title: t("Motor on", "Motor an"), target: "motor" }, { t: 605, kind: "shutdown", title: t("Motor off", "Motor aus"), target: "motor" }],
    tracks: [{ target: "operating:motor", ease: "step", keys: [{ t: 0, v: 0 }, { t: 5, v: 1 }, { t: 605, v: 0 }] }, { target: "emitter:exhaust-air", ease: "step", keys: [{ t: 0, v: 0 }, { t: 5, v: 1 }, { t: 605, v: 0 }] }] },
];

export const vacuum = {
  id: "vacuum-cleaner", category: "appliances",
  title: t("Canister Vacuum Cleaner", "Bodenstaubsauger"),
  summary: t("A 700 W canister vacuum cleaner with hose, telescopic wand and floor nozzle, 5.1 kg: the motor releases 180 W into the canister shell, lid and grille and 520 W into warm exhaust air behind the canister; parked and running poses.", "Ein Bodenstaubsauger von 700 W mit Schlauch, Teleskoprohr und Bodendüse, 5,1 kg: Der Motor gibt 180 W an Kesselschale, Deckel und Gitter ab und 520 W an warme Abluft hinter dem Kessel; Posen geparkt und läuft."),
  tags: ["vacuum-cleaner", "household", "appliance", "internal-gain", "electric", "heat-source"],
  provenance: { author: "Semio staffage content (A4)", method: "handcrafted", created: "2026-11-01", references: refs,
    notes: { en: "Side view of a typical canister vacuum cleaner (no single product): canister 0.51 m x 0.30 m (class of EN 60312-1 test cleaners, 4-6 kg), rear wheel radius 0.06 m, ten-segment hose of 1.1 m, telescopic wand 0.78 m, floor nozzle 0.28 m long. The rated input of 700 W is below the 900 W limit of Regulation (EU) 666/2013. The whole 700 W ends up in the room: 110 W warm the shell, 40 W the lid, 30 W the grille and 520 W the exhaust air (0.030 m3/s at 14.4 K rise); the split is an authored estimate. Intake air and dust are not modelled.", de: "Seitenansicht eines typischen Bodenstaubsaugers (kein einzelnes Produkt): Kessel 0,51 m x 0,30 m (Klasse der Prüfgeräte nach EN 60312-1, 4-6 kg), Hinterradradius 0,06 m, zehngliedriger Schlauch von 1,1 m, Teleskoprohr 0,78 m, Bodendüse 0,28 m lang. Die Nennaufnahme von 700 W liegt unter der Grenze von 900 W der Verordnung (EU) 666/2013. Die gesamten 700 W landen im Raum: 110 W erwärmen die Schale, 40 W den Deckel, 30 W das Gitter und 520 W die Abluft (0,030 m3/s bei 14,4 K Anstieg); die Aufteilung ist ein Schätzwert. Ansaugluft und Staub sind nicht modelliert." } },
  dimensions: { heightM: 0.76, widthM: 1.5, depthM: 0.3, massKg: 5.1 },
  materials,
  rig: { kind: "mechanical", root: "body", bones },
  anchors,
  representations: reps,
  symbols: [],
  poses, clips,
  thermal: {
    environment: { airTempC: 20, meanRadiantTempC: 20, relativeHumidity: 0.5, windMS: 0, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325, skyTempC: 20, groundTempC: 20 },
    operating: [keyMotor], passengers: [], regions, emitters,
    assumptions: [
      t("The motor inside is not a region: its 700 W (including the 250 W of suction air power, which ends as heat too) leave through the shell (110 W), lid (40 W), grille (30 W) and the exhaust air (520 W). A room sees all of it as an internal gain.", "Der Motor im Inneren ist keine Region: Seine 700 W (einschließlich der 250 W Saugluftleistung, die ebenfalls als Wärme enden) verlassen das Gerät durch Schale (110 W), Deckel (40 W), Gitter (30 W) und die Abluft (520 W). Ein Raum erlebt alles davon als innere Last."),
      t("The suction level 0.45 scales motor heat and exhaust flow linearly.", "Die Saugstufe 0,45 skaliert Motorwärme und Abluftstrom linear."),
    ],
  },
  structure: { gravityMS2: 9.80665, dynamics: "quasi-static", supports: ["wheel-contact", "caster-contact", "nozzle-contact"], assumptions: [t("A rigid body rolling on two wheels, a caster and the nozzle; the hose is a flexible chain.", "Ein starrer Körper auf zwei Rädern, einer Rolle und der Düse; der Schlauch ist eine flexible Kette.")], clinical: false },
  presentation: { defaultPose: "running", defaultClip: "floor-stroke", defaultScale: 50, viewpoints: ["elevation"] },
  exports: ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"],
  completion: "authored",
};

export const vacuumManifest = { id: "vacuum-cleaner", category: "appliances", title: vacuum.title, heightM: 0.76, widthM: 1.5, poses: poses.map((p) => p.id), clips: clips.map((c) => c.id), representations: { 50: "authored", 100: "authored", 200: "authored", 500: "unavailable" }, symbols: [], analytics: ["thermal", "transport", "structural"], viewpoints: ["elevation"], exports: vacuum.exports, distinction: { en: "A 700 W canister vacuum cleaner with hose, wand and floor nozzle: shell, lid and grille warm up and 520 W of warm air leave behind the canister.", de: "Ein Bodenstaubsauger von 700 W mit Schlauch, Rohr und Bodendüse: Schale, Deckel und Gitter erwärmen sich, und 520 W warme Luft treten hinter dem Kessel aus." }, completion: "authored" };
