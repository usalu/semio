import { load, save, rep } from "./lib.ts";
const d = load("solar-thermal-collector");
const r200 = rep(d, 200);
const pick = (v: string, id: string) => structuredClone(r200.geometry[v].find((s: any) => s.id === id));
const all = ["housing", "glazing", "absorber", "header-lower", "header-upper"];
const box = (id: string, bone: string, extra: any) => ({ id, bone, role: "body", filled: true, ...extra });
d.asset.representations["500"] = {
  status: "authored",
  denominator: 500,
  validRange: { minDenominator: 350, maxDenominator: 1000 },
  pens: { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.25 },
  hatches: [],
  geometry: {
    elevation: ["base-rail", "front-strut", "rear-strut", "collector-box"].map((id) => pick("elevation", id)),
    front: [
      { kind: "rect", x: -0.425, y: 0, w: 0.85, h: 0.18, ...box("front-foot-block", "stand", { z: 2.7, tone: 0.5, region: "stand", material: "galvanised-steel" }) },
      { kind: "rect", x: -0.5, y: 0.18, w: 1, h: 1.22907, ...box("front-box", "housing", { z: 3.2, tone: 0.4, regions: all, material: "anodised-aluminium" }) },
    ],
    plan: [{ kind: "rect", x: -0.84783, y: -0.5, w: 1.69566, h: 1, ...box("plan-box", "housing", { z: 3.2, tone: 0.4, regions: all, material: "anodised-aluminium" }) }],
  },
  decisions: {
    silhouette: { en: "A 3.4 mm by 2 mm by 2.8 mm collector: one merged tilted slab on a base rail and two struts in the side view, one merged block on a foot block in the front view and one merged rectangle in the top view.", de: "Ein 3,4 mm mal 2 mm mal 2,8 mm grosser Kollektor: eine zusammengeführte geneigte Platte auf Fussschiene und zwei Stützen in der Seitenansicht, ein zusammengeführter Block auf einem Fussblock in der Vorderansicht und ein zusammengeführtes Rechteck in der Draufsicht." },
    retained: { en: "Slab, base rail and both struts in the side view; foot block and slab face in the front view; the full footprint in the top view. The tilt and the three real dimensions of 1.70 m, 1.00 m and 1.41 m are kept.", de: "Platte, Fussschiene und beide Stützen in der Seitenansicht; Fussblock und Plattenfläche in der Vorderansicht; die volle Grundfläche in der Draufsicht. Die Neigung und die drei wahren Masse 1,70 m, 1,00 m und 1,41 m bleiben erhalten." },
    merged: { en: "Housing, glass, absorber and both headers are one form listing five regions; the two foot pads of the front view are one block.", de: "Wanne, Glas, Absorber und beide Sammler sind eine Form mit fünf Regionen; die beiden Fussplatten der Vorderansicht sind ein Block." },
    omitted: { en: "Glass field, absorber outline, headers, insulation, risers and all section detail.", de: "Glasfeld, Absorberumriss, Sammler, Dämmung, Steigrohre und alle Schnittdetails." },
    thinElements: { en: "Struts are 0.08 m (0.16 mm) and the rail 0.08 m high; they are the thinnest drawn parts.", de: "Die Stützen sind 0,08 m (0,16 mm) und die Schiene 0,08 m hoch; es sind die dünnsten gezeichneten Teile." },
    hatching: { en: "None.", de: "Keine." },
    styles: { en: "Body pen 0.13 mm, ground 0.25 mm; darker tones carry the housing, lighter tones the thin parts.", de: "Körperstift 0,13 mm, Boden 0,25 mm; dunklere Töne tragen die Wanne, hellere Töne die dünnen Teile." },
    overlays: { en: "The slab lists five regions as one merged form in every view.", de: "Die Platte führt in jeder Ansicht fünf Regionen als eine zusammengeführte Form." },
  },
};
save("solar-thermal-collector", d);
const s = load("scissor-lift");
const u = rep(s, 500);
u.reason = {
  en: "At 1:500 the stowed scissor stack is five stages of 0.11 m (0.22 mm) and the raised scissors need ten crossing arms of 0.12 m (0.24 mm) with guard rails of 0.04 to 0.05 m tubes (0.1 mm): the articulated rig cannot be drawn physically without arms closer than the 0.15 mm gap limit, so no scaled drawing of the moving scissors exists at this scale. The 4.4 mm true-extent silhouette of the stowed lift is offered as a symbol in all three views.",
  de: "Bei 1:500 ist das eingefahrene Scherenpaket fünf Stufen von je 0,11 m (0,22 mm), und die gehobene Schere braucht zehn kreuzende Arme von 0,12 m (0,24 mm) mit Geländerrohren von 0,04 bis 0,05 m (0,1 mm): das gegliederte Rig lässt sich nicht massstäblich zeichnen, ohne dass Arme näher als die Grenzlücke von 0,15 mm liegen, daher gibt es bei diesem Massstab keine massstäbliche Zeichnung der bewegten Schere. Die wahre Umrissform des eingefahrenen Hubwagens mit 4,4 mm wird als Symbol in allen drei Ansichten angeboten.",
};
save("scissor-lift", s);
