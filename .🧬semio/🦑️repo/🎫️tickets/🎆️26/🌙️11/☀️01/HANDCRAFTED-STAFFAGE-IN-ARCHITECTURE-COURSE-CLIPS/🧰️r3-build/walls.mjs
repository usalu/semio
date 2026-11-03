import { REFS, baseAsset, hatch, lay, line, mat, mergeLayers, poly, rect, region, rep, sunClip, shadeClip, tx, uValue } from "./comp.mjs";

const brickHatch = hatch("cut-brick", "Cut solid brick", "Geschnittener Vollziegel", "lines", 45, 0.8);
const woolHatch = hatch("mineral-wool", "Mineral wool", "Mineralwolle", "cross", 45, 0.8);
const T = tx;
const A = (en, de) => [en, de];

const STACKS = {
  "solid-brick-wall-1960": { insulation: 0, key: "uninsulated" },
  "etics-brick-wall-12": { insulation: 0.12, key: "insulated-12" },
  "etics-brick-wall-30": { insulation: 0.3, key: "highly-insulated-30" },
};

const layersOf = (ins) => [
  { id: "plaster", d: 0.015, tone: 0.9, material: "lime-plaster", lambda: 0.7, rho: 1400, c: 1000, en: "Interior lime-cement plaster 15 mm", de: "Innenputz aus Kalkzement 15 mm", kind: "masonry" },
  { id: "brick", d: 0.365, tone: 0.55, material: "solid-brick", lambda: 0.75, rho: 1800, c: 1000, en: "Solid brick 365 mm (1.5 bricks)", de: "Vollziegel 365 mm (1,5 Stein)", kind: "masonry", hatch: "cut-brick" },
  ...(ins > 0 ? [{ id: "insulation", d: ins, tone: 0.8, material: "mineral-wool", lambda: 0.035, rho: 60, c: 1030, en: `ETICS mineral wool ${Math.round(ins * 1000)} mm`, de: `WDVS Mineralwolle ${Math.round(ins * 1000)} mm`, kind: "insulation", hatch: "mineral-wool" }] : []),
  { id: "render", d: 0.02, tone: 0.85, material: "mineral-render", lambda: 0.87, rho: 1700, c: 1000, en: "Mineral render 20 mm", de: "Mineralputz 20 mm", kind: "masonry" },
];

const dec = (den, merged, name) => ({
  silhouette: A(`A ${name} section 1.0 m high, interior on the left and exterior on the right, layers drawn at their real thickness.`, `Ein Schnitt der Wand ${name}, 1,0 m hoch, Innenraum links und außen rechts, Schichten in ihrer wirklichen Dicke.`),
  retained: A("Every layer that prints at this scale: the brick leaf, the insulation if any and the facing coat.", "Jede Schicht, die in diesem Maßstab druckbar ist: die Ziegelschale, die Dämmung falls vorhanden und die Außenschicht."),
  merged: A(merged ? "Layers thinner than 0.25 mm on paper join the thicker neighbour; the thermal region is one and keeps all layers." : "None.", merged ? "Schichten dünner als 0,25 mm auf Papier gehen in der dickeren Nachbarschicht auf; die thermische Region bleibt eine und behält alle Schichten." : "Keine."),
  omitted: A("The window reveal, the floor connection and the anchors of the render system.", "Die Fensterlaibung, der Deckenanschluss und die Dübel des Putzsystems."),
  thinElements: A("The 15 mm plaster is 0.3 mm at 1:50; the 20 mm render 0.4 mm. Below that they merge.", "Der 15-mm-Innenputz ist bei 1:50 0,3 mm; der 20-mm-Außenputz 0,4 mm. Darunter gehen sie auf."),
  hatching: A(den <= 100 ? "Brick at 45 degrees, mineral wool as a cross hatch (DIN ISO 128-50)." : "None.", den <= 100 ? "Ziegel unter 45 Grad, Mineralwolle als Kreuzschraffur (DIN ISO 128-50)." : "Keine."),
  styles: A("Body pen for the layer outlines, detail pen for the bed joints (1:50 only).", "Körperstift für die Schichtkonturen, Detailstift für die Lagerfugen (nur 1:50)."),
  overlays: A("One thermal region wall-field over all layers; the surface temperatures come from the scenario.", "Eine thermische Region wall-field über alle Schichten; die Oberflächentemperaturen kommen aus dem Szenario."),
});

export const wall = (id) => {
  const { insulation, key } = STACKS[id];
  const L = layersOf(insulation);
  const thick = L.reduce((s, l) => s + l.d, 0);
  const thermalLayers = L.map((l) => lay(l.id, l.en, l.de, l.kind, l.material, l.d, l.lambda, l.rho, l.c));
  const U = uValue(thermalLayers);
  const mass = Math.round(L.reduce((s, l) => s + l.d * l.rho, 0) * 10) / 10;
  const regions = [region({ id: "wall-field", title: ["Wall field, all layers", "Wandfeld, alle Schichten"], material: "solid-brick", areaM2: 1, length: 1, layers: thermalLayers })];
  const geometryAt = (den) => {
    const groups = mergeLayers(L, den, den === 100 ? 0.2 : 0.25);
    let x = 0;
    const shapes = [];
    for (const g of groups) {
      shapes.push(rect(g.id, "body", 1, g.tone, x, 0, g.d, 1, { region: "wall-field", material: g.material, ...(den <= 100 && g.hatch ? { hatch: g.hatch } : {}) }));
      x += g.d;
    }
    if (den === 50) {
      const bx = L[0].d;
      for (let k = 1; k < 13; k += 1) shapes.push(line(`joint-${k}`, "detail", 2, 0.3, [bx, k * 0.075], [bx + L[1].d, k * 0.075], { region: "wall-field", material: "solid-brick" }));
    }
    const plan = groups.map((g, i) => rect(g.id + "-plan", "body", 1, g.tone, groups.slice(0, i).reduce((a, h) => a + h.d, 0), -0.5, g.d, 1, { region: "wall-field", material: g.material, ...(den <= 100 && g.hatch ? { hatch: g.hatch } : {}) }));
    const front = [rect("face", "body", 1, groups[groups.length - 1].tone, -0.5, 0, 1, 1, { region: "wall-field", material: groups[groups.length - 1].material })];
    return { shapes, plan, front, merged: groups.length < L.length };
  };
  const reps = Object.fromEntries([50, 100, 200, 500].map((den) => {
    const { shapes, plan, front, merged } = geometryAt(den);
    const r = rep(den, den <= 100 ? (insulation > 0 ? [brickHatch, woolHatch] : [brickHatch]) : [], shapes, dec(den, merged, key));
    r.geometry = { elevation: shapes, front, plan };
    return [den, r];
  }));
  const name = insulation === 0 ? ["uninsulated 1960 solid brick wall", "ungedämmte Vollziegelwand von 1960"] : [`1960 solid brick wall with ${Math.round(insulation * 100)} cm ETICS`, `Vollziegelwand von 1960 mit ${Math.round(insulation * 100)} cm WDVS`];
  const uShown = U.toFixed(2);
  const c = {
    id,
    title: insulation === 0 ? ["Solid Brick Wall 1960", "Vollziegelwand 1960"] : [`Brick Wall with ${Math.round(insulation * 100)} cm ETICS`, `Ziegelwand mit ${Math.round(insulation * 100)} cm WDVS`],
    summary: [
      `A ${name[0]} in vertical section, 1 m high: 15 mm interior plaster, 365 mm solid brick${insulation ? `, ${Math.round(insulation * 1000)} mm mineral wool ETICS` : ""} and 20 mm mineral render, U = ${uShown} W/(m2 K) from interior 20 degC and R_si 0.13, R_se 0.04 m2K/W.`,
      `Eine ${name[1]} im Vertikalschnitt, 1 m hoch: 15 mm Innenputz, 365 mm Vollziegel${insulation ? `, ${Math.round(insulation * 1000)} mm Mineralwolle-WDVS` : ""} und 20 mm Mineralputz, U = ${uShown.replace(".", ",")} W/(m² K) bei Innenraum 20 °C und R_si 0,13, R_se 0,04 m²K/W.`,
    ],
    tags: ["component", "thermal", "wall", "build-up", "brick", ...(insulation ? ["etics", "insulation"] : ["uninsulated"])],
    references: [REFS.iso6946, REFS.iso10456, REFS.gebG, REFS.din1356, REFS.din128],
    notes: [
      "The wall is drawn in section as the heat-conduction lesson shows it: interior at the left, exterior at the right. One thermal region carries the layers from the inside out, so the solver gives the interior and exterior surface temperature of the scenario.",
      "Die Wand ist im Schnitt gezeichnet, wie ihn die Wärmeleitungs-Lektion zeigt: innen links, außen rechts. Eine thermische Region trägt die Schichten von innen nach außen, damit der Löser die innere und äußere Oberflächentemperatur des Szenarios liefert.",
    ],
    dimensions: { heightM: 1, widthM: Math.round(thick * 1000) / 1000, depthM: 1, massKg: mass },
    rootTitle: ["Wall strip, rigid", "Wandstreifen, starr"],
    materials: [
      mat("lime-plaster", "Lime-cement plaster", "Kalkzementputz", 0.93, { solarAbsorptivity: 0.5, densityKgM3: 1400, conductivityWmK: 0.7, specificHeatJKgK: 1000, source: "literature", reference: "en-iso-10456" }),
      mat("solid-brick", "Solid clay brick", "Vollziegel", 0.9, { solarAbsorptivity: 0.7, densityKgM3: 1800, conductivityWmK: 0.75, specificHeatJKgK: 1000, source: "literature", reference: "en-iso-10456" }),
      mat("mineral-render", "Mineral render", "Mineralputz", 0.9, { solarAbsorptivity: 0.4, densityKgM3: 1700, conductivityWmK: 0.87, specificHeatJKgK: 1000, source: "literature", reference: "en-iso-10456" }),
      ...(insulation ? [mat("mineral-wool", "Mineral wool board", "Mineralwolledämmplatte", 0.9, { solarAbsorptivity: 0.5, densityKgM3: 60, conductivityWmK: 0.035, specificHeatJKgK: 1030, source: "literature", reference: "en-iso-10456" })] : []),
    ],
    regions,
    representations: reps,
    poses: [
      { id: "winter-night", title: ["Winter night", "Winternacht"], description: ["Exterior face in the dark: no solar gain.", "Außenfläche im Dunkeln: kein Solargewinn."], sunlit: { default: 0 } },
      { id: "winter-sun", title: ["Winter sun", "Wintersonne"], description: ["Exterior face in full south sun.", "Außenfläche in voller Südsonne."], sunlit: { default: 1 } },
      { id: "half-shaded", title: ["Half shaded", "Halb beschattet"], description: ["Half of the exterior face in sun.", "Die Hälfte der Außenfläche in der Sonne."], sunlit: { default: 0.5 } },
    ],
    clips: [sunClip(["wall-field"], "winter-night", "winter-sun"), shadeClip(["wall-field"], "winter-sun")],
    assumptions: [
      T(`Winter reference case: air 0 degC, sky -12 degC, wind 3 m/s, 300 W/m2; interior 20 degC with R_si 0.13 m2K/W. U = ${uShown} W/(m2 K) over R_si + sum d/lambda + R_se (EN ISO 6946). The viewer may select any preset.`, `Winter-Referenzfall: Luft 0 °C, Himmel -12 °C, Wind 3 m/s, 300 W/m2; Innenraum 20 °C mit R_si 0,13 m2K/W. U = ${uShown.replace(".", ",")} W/(m² K) aus R_si + Summe d/lambda + R_se (EN ISO 6946). Der Betrachter kann jede Voreinstellung wählen.`),
      T("The wall is homogeneous: mortar joints, anchors and thermal bridges are not modelled.", "Die Wand ist homogen: Mörtelfugen, Dübel und Wärmebrücken sind nicht modelliert."),
    ],
    structure: [
      T("The 1 m strip is one rigid body on a fixed base; the self-weight acts at its centre. No imposed load is applied.", "Der 1-m-Streifen ist ein starrer Körper auf eingespanntem Fußpunkt; das Eigengewicht wirkt im Schwerpunkt. Es ist keine Nutzlast angesetzt."),
      T("Layer densities follow EN ISO 10456; the wall is not a load-bearing check.", "Die Rohdichten folgen EN ISO 10456; die Wand ist kein Tragfähigkeitsnachweis."),
    ],
  };
  return { asset: baseAsset(c), u: U, thick, mass };
};
