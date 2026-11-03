import { existsSync, readFileSync, writeFileSync } from "node:fs";
const L = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog";
type S = Record<string, unknown>;
const r4 = (n: number) => +n.toFixed(4);
const rect = (id: string, bone: string, role: string, z: number, filled: boolean, x: number, y: number, w: number, h: number, extra: S = {}): S => ({ id, bone, role, z, filled, tone: filled ? (role === "body" ? 0.85 : 0.6) : 0.5, kind: "rect", x: r4(x), y: r4(y), w: r4(w), h: r4(h), ...extra });
const poly = (id: string, bone: string, role: string, z: number, filled: boolean, closed: boolean, pts: [number, number][], extra: S = {}): S => ({ id, bone, role, z, filled, tone: filled ? 0.85 : 0.5, kind: "poly", points: pts.map(([a, b]) => [r4(a), r4(b)]), closed, ...extra });
const ell = (id: string, bone: string, role: string, z: number, filled: boolean, cx: number, cy: number, rx: number, ry = rx): S => ({ id, bone, role, z, filled, tone: filled ? 0.85 : 0.5, kind: "ellipse", cx: r4(cx), cy: r4(cy), rx: r4(rx), ry: r4(ry) });
const bar = (id: string, bone: string, role: string, z: number, x0: number, y0: number, x1: number, y1: number, w: number, filled = true): S => {
  const dx = x1 - x0, dy = y1 - y0, l = Math.hypot(dx, dy), nx = (-dy / l) * (w / 2), ny = (dx / l) * (w / 2);
  return poly(id, bone, role, z, filled, true, [[x0 + nx, y0 + ny], [x1 + nx, y1 + ny], [x1 - nx, y1 - ny], [x0 - nx, y0 - ny]]);
};
type Sym = { id: string; title: { en: string; de: string }; description: { en: string; de: string }; note: { en: string; de: string }; geometry: { elevation: S[]; front: S[]; plan: S[] }; denominators?: number[] };
const G = "ground";
const trueNote = (extra: { en: string; de: string }) => ({
  en: `Model metres at real size in the frame of the asset (x ahead, lateral positive to the left, y up): the side view is seen from the right, the front view from ahead, the top view from above; all three show the default pose. ${extra.en}`,
  de: `Modellmeter in wahrer Größe im Koordinatensystem des Objekts (x nach vorn, seitlich positiv nach links, y nach oben): die Seitenansicht blickt von rechts, die Vorderansicht von vorn, die Draufsicht von oben; alle drei zeigen die Standardhaltung. ${extra.de}`,
});

const stool: Sym = {
  id: "stool-plan",
  title: { en: "Stool symbol, true extent", de: "Hockersymbol, wahre Ausdehnung" },
  description: { en: "The bar stool as a simple pictogram in all three views: round seat of 0.36 m diameter on four splayed legs with a foot ring, leg footprint 0.42 m.", de: "Der Barhocker als einfaches Piktogramm in allen drei Ansichten: runder Sitz von 0,36 m Durchmesser auf vier gespreizten Beinen mit Fußring, Standfläche 0,42 m." },
  note: trueNote({ en: "Seat radius 0.18 m, leg footprint radius 0.21 m; the swivel does not change the round outline.", de: "Sitzradius 0,18 m, Standflächenradius 0,21 m; die Drehung ändert den runden Umriss nicht." }),
  geometry: {
    elevation: [rect("seat", G, "body", 3, true, -0.18, 0.82, 0.36, 0.05, { r: 0.01 }), bar("leg-near", G, "body", 2, -0.16, 0.82, -0.2, 0, 0.025), bar("leg-far", G, "body", 2, 0.16, 0.82, 0.2, 0, 0.025), poly("foot-ring", G, "detail", 2.5, false, false, [[-0.19, 0.28], [0.19, 0.28]])],
    front: [rect("seat", G, "body", 3, true, -0.18, 0.82, 0.36, 0.05, { r: 0.01 }), bar("leg-left", G, "body", 2, -0.16, 0.82, -0.2, 0, 0.025), bar("leg-right", G, "body", 2, 0.16, 0.82, 0.2, 0, 0.025), poly("foot-ring", G, "detail", 2.5, false, false, [[-0.19, 0.28], [0.19, 0.28]])],
    plan: [ell("plan-seat", G, "body", 2, true, 0, 0, 0.18), ell("plan-ring", G, "detail", 1, false, 0, 0, 0.212)],
  },
};

const shelf: Sym = {
  id: "shelf-plan",
  title: { en: "Shelf unit symbol, true extent", de: "Regalsymbol, wahre Ausdehnung" },
  description: { en: "The shelf unit in all three views: carcass 0.80 x 0.30 x 1.80 m on a plinth, five shelves at 0.36 m steps of 0.30 m, the back panel on the left side.", de: "Das Regal in allen drei Ansichten: Korpus 0,80 x 0,30 x 1,80 m auf einem Sockel, fünf Böden im Abstand von 0,30 m, die Rückwand auf der linken Seite." },
  note: trueNote({ en: "The back panel lies on the left (+z) of the unit, the open front faces right (-z).", de: "Die Rückwand liegt links (+z) am Möbel, die offene Front zeigt nach rechts (-z)." }),
  geometry: {
    elevation: [
      rect("plinth", G, "body", 1, true, 0.018, 0, 0.764, 0.075),
      rect("side-l", G, "body", 2, true, 0, 0.06, 0.022, 1.74),
      rect("side-r", G, "body", 2, true, 0.778, 0.06, 0.022, 1.74),
      rect("top", G, "body", 2, true, 0, 1.778, 0.8, 0.022),
      rect("bottom", G, "body", 2, true, 0, 0.06, 0.8, 0.022),
      rect("back", G, "detail", 1.5, false, 0.022, 0.082, 0.756, 1.696),
      ...[0.36, 0.66, 0.96, 1.26, 1.56].map((y, i) => rect(`shelf-${i + 1}`, G, "body", 2.5, true, 0.014, y, 0.772, 0.022)),
    ],
    front: [
      rect("side", G, "body", 2, false, -0.15, 0.06, 0.3, 1.74),
      rect("plinth", G, "body", 1, true, -0.12, 0, 0.25, 0.075),
      rect("back", G, "detail", 2.5, true, 0.132, 0.07, 0.018, 1.708),
      ...[0.36, 0.66, 0.96, 1.26, 1.56].map((y, i) => rect(`shelf-${i + 1}`, G, "detail", 2.2, false, -0.145, y, 0.277, 0.022)),
    ],
    plan: [rect("plan-body", G, "body", 1, true, 0, -0.15, 0.8, 0.3), poly("plan-front", G, "detail", 2, false, false, [[0.022, -0.128], [0.778, -0.128]]), poly("plan-back", G, "detail", 2, false, false, [[0.022, 0.132], [0.778, 0.132]])],
  },
};

const table: Sym = {
  id: "table-plan",
  title: { en: "Table symbol, true extent", de: "Tischsymbol, wahre Ausdehnung" },
  description: { en: "The extending dining table closed in all three views: top 1.60 x 0.90 m at 0.75 m, aprons, four legs, centre rail and the 0.60 m leaf outline.", de: "Der Ausziehtisch geschlossen in allen drei Ansichten: Platte 1,60 x 0,90 m auf 0,75 m, Zargen, vier Beine, Mittelschiene und der Umriss der 0,60-m-Einlegeplatte." },
  note: trueNote({ en: "The leaf outline marks the 0.60 m extension at x 0.50 to 1.10 m.", de: "Der Umriss der Einlegeplatte markiert die 0,60-m-Verlängerung bei x 0,50 bis 1,10 m." }),
  geometry: {
    elevation: [rect("top", G, "body", 3, true, 0, 0.72, 1.6, 0.03), rect("apron", G, "detail", 2, false, 0.1, 0.64, 1.4, 0.08), rect("leg-l", G, "body", 2.5, true, 0.1, 0, 0.07, 0.72), rect("leg-r", G, "body", 2.5, true, 1.43, 0, 0.07, 0.72), poly("rail", G, "detail", 2, false, false, [[0.3, 0.66], [1.3, 0.66]]), poly("leaf", G, "detail", 3.5, false, false, [[0.5, 0.72], [0.5, 0.69], [1.1, 0.69], [1.1, 0.72]])],
    front: [rect("top", G, "body", 3, true, -0.45, 0.72, 0.9, 0.03), rect("apron", G, "detail", 2, false, -0.38, 0.64, 0.76, 0.08), rect("leg-left", G, "body", 2.5, true, 0.34, 0, 0.07, 0.64), rect("leg-right", G, "body", 2.5, true, -0.41, 0, 0.07, 0.64)],
    plan: [rect("plan-top", G, "body", 1, true, 0, -0.45, 1.6, 0.9), poly("plan-leaf", G, "detail", 2, false, true, [[0.5, -0.45], [1.1, -0.45], [1.1, 0.45], [0.5, 0.45]]), poly("plank-a", G, "detail", 2, false, false, [[0.02, -0.15], [1.58, -0.15]]), poly("plank-b", G, "detail", 2, false, false, [[0.02, 0.15], [1.58, 0.15]])],
  },
};

const deck: Sym = {
  id: "chair-plan",
  title: { en: "Chair symbol, true extent", de: "Stuhlsymbol, wahre Ausdehnung" },
  description: { en: "The unfolded deck chair in all three views: rear and front frame legs, the seat arms with the front rail, the canvas sling and the top rail.", de: "Der aufgeklappte Liegestuhl in allen drei Ansichten: hintere und vordere Rahmenbeine, die Sitzarme mit Frontholm, die Leinwand und der obere Holm." },
  note: trueNote({ en: "The sling hangs between the top rail at the back and the front rail of the seat; the chair faces ahead (+x).", de: "Die Bespannung hängt zwischen dem oberen Holm hinten und dem Frontholm des Sitzes; der Stuhl blickt nach vorn (+x)." }),
  geometry: {
    elevation: [bar("rear", "rear", "body", 2, 0.02, 0.025, 0.488, 0.93, 0.045), bar("front", "front", "body", 2.1, 0.675, 0.025, 0.348, 0.636, 0.045), bar("seat", "seat", "body", 2.2, 0.355, 0.64, 0.762, 0.405, 0.045), bar("canvas-a", "canvas-a", "detail", 1.5, 0.323, 0.347, 0.491, 0.923, 0.01), poly("canvas-b", "canvas-b", "detail", 1.5, false, false, [[0.323, 0.371], [0.77, 0.371]])].map((s) => ({ ...s, bone: G })),
    front: [rect("rear-r", G, "body", 2, true, -0.292, 0.011, 0.045, 0.914), rect("rear-l", G, "body", 2, true, 0.247, 0.011, 0.045, 0.914), rect("front-r", G, "body", 2.1, true, -0.258, 0.011, 0.045, 0.633), rect("front-l", G, "body", 2.1, true, 0.212, 0.011, 0.045, 0.633), rect("seat-r", G, "body", 2.2, true, -0.203, 0.384, 0.045, 0.268), rect("seat-l", G, "body", 2.2, true, 0.158, 0.384, 0.045, 0.268), rect("top-rail", G, "body", 2.5, true, -0.27, 0.91, 0.54, 0.025), rect("canvas", G, "detail", 1.5, false, -0.2, 0.347, 0.4, 0.576)],
    plan: [rect("rear-l", G, "body", 2, true, -0.008, 0.247, 0.511, 0.045), rect("rear-r", G, "body", 2, true, -0.008, -0.292, 0.511, 0.045), rect("front-l", G, "body", 2.1, true, 0.329, 0.212, 0.365, 0.045), rect("front-r", G, "body", 2.1, true, 0.329, -0.258, 0.365, 0.045), rect("seat-l", G, "body", 2.2, true, 0.338, 0.158, 0.435, 0.045), rect("seat-r", G, "body", 2.2, true, 0.338, -0.203, 0.435, 0.045), rect("top-rail", G, "body", 2.5, true, 0.478, -0.27, 0.025, 0.54), rect("front-rail", G, "body", 2.5, true, 0.758, -0.18, 0.025, 0.36), rect("canvas", G, "detail", 1.5, false, 0.323, -0.2, 0.447, 0.4)],
  },
};
for (const v of ["elevation"] as const) deck.geometry[v] = deck.geometry[v].map((s) => ({ ...s, bone: G }));
for (const v of ["front", "plan"] as const) deck.geometry[v] = deck.geometry[v].map((s) => ({ ...s, bone: G }));

const picnic: Sym = {
  id: "picnic-plan",
  title: { en: "Picnic table symbol, true extent", de: "Picknicktisch-Symbol, wahre Ausdehnung" },
  description: { en: "The picnic table with deployed benches in all three views: top 0.70 x 1.80 m at 0.75 m, two benches at 0.44 m, two A-frame trestles with a cross beam.", de: "Der Picknicktisch mit ausgeklappten Bänken in allen drei Ansichten: Platte 0,70 x 1,80 m auf 0,75 m, zwei Bänke auf 0,44 m, zwei A-Böcke mit Querbalken." },
  note: trueNote({ en: "x runs across the table, the 1.80 m length runs laterally; benches in the deployed position.", de: "x läuft quer zum Tisch, die Länge von 1,80 m verläuft seitlich; Bänke ausgeklappt." }),
  geometry: {
    elevation: [rect("top", G, "body", 3, true, -0.35, 0.71, 0.7, 0.04), rect("top-beam", G, "detail", 2.5, false, -0.3, 0.665, 0.6, 0.045), poly("leg-l", G, "body", 2, true, true, [[-0.76, 0], [-0.71, 0], [-0.175, 0.665], [-0.225, 0.665]]), poly("leg-r", G, "body", 2, true, true, [[0.76, 0], [0.71, 0], [0.175, 0.665], [0.225, 0.665]]), rect("beam", G, "body", 2.2, true, -0.48, 0.355, 0.96, 0.045), rect("bench-l", G, "body", 2.6, true, -0.76, 0.4, 0.36, 0.04), rect("bench-r", G, "body", 2.6, true, 0.4, 0.4, 0.36, 0.04), rect("hinge-l", G, "detail", 2.7, false, -0.5, 0.34, 0.08, 0.1), rect("hinge-r", G, "detail", 2.7, false, 0.42, 0.34, 0.08, 0.1)],
    front: [rect("top", G, "body", 3, true, -0.9, 0.71, 1.8, 0.04), rect("bench", G, "body", 2.6, true, -0.85, 0.4, 1.7, 0.04), rect("trestle-l", G, "body", 2, true, 0.777, 0, 0.046, 0.665), rect("trestle-r", G, "body", 2, true, -0.823, 0, 0.046, 0.665), rect("beam-l", G, "detail", 2.2, false, 0.777, 0.355, 0.046, 0.045), rect("beam-r", G, "detail", 2.2, false, -0.823, 0.355, 0.046, 0.045), rect("hinge-l", G, "detail", 2.7, false, 0.67, 0.34, 0.1, 0.1), rect("hinge-r", G, "detail", 2.7, false, -0.77, 0.34, 0.1, 0.1)],
    plan: [rect("plan-bench-r", G, "body", 1, true, 0.4, -0.85, 0.36, 1.7), rect("plan-bench-l", G, "body", 1, true, -0.76, -0.85, 0.36, 1.7), rect("plan-top", G, "body", 2, true, -0.35, -0.9, 0.7, 1.8), rect("leg-l-0", G, "detail", 1.5, false, -0.72, 0.777, 0.53, 0.046), rect("leg-r-0", G, "detail", 1.5, false, 0.19, 0.777, 0.53, 0.046), rect("leg-l-1", G, "detail", 1.5, false, -0.72, -0.823, 0.53, 0.046), rect("leg-r-1", G, "detail", 1.5, false, 0.19, -0.823, 0.53, 0.046)],
  },
};

const bed: Sym = {
  id: "bed-plan",
  title: { en: "Bed symbol, true extent", de: "Bettsymbol, wahre Ausdehnung" },
  description: { en: "The single bed flat in all three views: head board 0.95 m, foot board 0.55 m, side rails, slatted base, 0.21 m mattress; pillow and cover edge are conventional marks.", de: "Das Einzelbett flach in allen drei Ansichten: Kopfteil 0,95 m, Fußteil 0,55 m, Seitenholme, Lattenrost, 0,21-m-Matratze; Kissen und Deckenkante sind Zeichen in wahrer Größe." },
  note: trueNote({ en: "x along the bed with the head at 0; pillow 0.40 x 0.66 m and the turned-down cover edge at x 0.70 m are conventional marks at real size.", de: "x entlang des Betts, Kopf bei 0; Kissen 0,40 x 0,66 m und die umgeschlagene Deckenkante bei x 0,70 m sind Zeichen in wahrer Größe." }),
  geometry: {
    elevation: [rect("head", G, "body", 2, true, 0, 0, 0.06, 0.95), rect("foot", G, "body", 2, true, 2.06, 0, 0.06, 0.55), rect("rail", G, "body", 2.2, true, 0.06, 0.14, 2, 0.16), rect("leg-mid", G, "body", 1.5, true, 1.03, 0, 0.06, 0.16), rect("slats", G, "detail", 2.4, false, 0.06, 0.263, 2, 0.022), rect("mattress", G, "body", 2.6, true, 0.055, 0.275, 2.01, 0.21), poly("cover-edge", G, "detail", 3, false, false, [[0.7, 0.485], [0.7, 0.275]]), rect("pillow", G, "detail", 3, false, 0.12, 0.485, 0.4, 0.07, { r: 0.03 })],
    front: [rect("head", G, "body", 3, false, -0.45, 0, 0.9, 0.95), rect("foot-post-r", G, "detail", 2, false, -0.45, 0, 0.06, 0.55), rect("foot-post-l", G, "detail", 2, false, 0.39, 0, 0.06, 0.55), rect("foot-rail", G, "detail", 2, false, -0.45, 0.47, 0.9, 0.08), rect("rail-end", G, "detail", 2.2, false, -0.45, 0.14, 0.9, 0.16), rect("mattress", G, "detail", 2.6, false, -0.41, 0.275, 0.82, 0.21)],
    plan: [rect("plan-frame", G, "body", 1, true, 0, -0.45, 2.12, 0.9), rect("plan-mattress", G, "detail", 2, false, 0.055, -0.41, 2.01, 0.82), rect("plan-pillow", G, "detail", 3, false, 0.12, -0.33, 0.4, 0.66, { r: 0.03 }), poly("plan-turn", G, "detail", 3, false, false, [[0.7, -0.41], [0.7, 0.41]])],
  },
};

const sofa: Sym = {
  id: "sofa-plan",
  title: { en: "Sofa symbol, true extent", de: "Sofasymbol, wahre Ausdehnung" },
  description: { en: "The three-seat sofa in all three views: 2.00 x 0.90 x 0.85 m with two arms, three seat cushions, three back cushions and a skirt on four feet; the back stands on the left side.", de: "Das Dreisitzer-Sofa in allen drei Ansichten: 2,00 x 0,90 x 0,85 m mit zwei Armlehnen, drei Sitzkissen, drei Rückenkissen und einer Zarge auf vier Füßen; die Rückenlehne steht auf der linken Seite." },
  note: trueNote({ en: "The back is on the left (+z), the seats face right (-z). Not for 1:500.", de: "Die Lehne steht links (+z), die Sitze blicken nach rechts (-z). Nicht für 1:500." }),
  geometry: {
    elevation: [rect("arm-l", G, "body", 3, true, 0, 0.1, 0.2, 0.52), rect("arm-r", G, "body", 3, true, 1.8, 0.1, 0.2, 0.52), rect("skirt", G, "body", 1, true, 0.15, 0.1, 1.7, 0.14), rect("foot-l", G, "body", 1, true, 0.085, 0, 0.07, 0.12), rect("foot-r", G, "body", 1, true, 1.845, 0, 0.07, 0.12), ...[0.19, 0.733, 1.267].map((x, i) => rect(`seat-${i + 1}`, G, "body", 2, true, x, 0.2, 0.543, 0.23)), ...[0.19, 0.733, 1.267].map((x, i) => rect(`back-${i + 1}`, G, "body", 1.5, false, x, 0.43, 0.543, 0.42))],
    front: [rect("arm", G, "body", 3, true, -0.45, 0.1, 0.9, 0.52), rect("skirt", G, "body", 1, true, -0.43, 0.1, 0.86, 0.14), rect("foot-l", G, "body", 1, true, 0.325, 0, 0.07, 0.12), rect("foot-r", G, "body", 1, true, -0.395, 0, 0.07, 0.12), rect("back", G, "body", 1.5, false, 0.19, 0.2, 0.26, 0.65), rect("seat", G, "detail", 2, false, -0.42, 0.2, 0.62, 0.23)],
    plan: [rect("arm-l", G, "body", 3, true, 0, -0.45, 0.2, 0.9), rect("arm-r", G, "body", 3, true, 1.8, -0.45, 0.2, 0.9), ...[0.19, 0.733, 1.267].map((x, i) => rect(`seat-${i + 1}`, G, "body", 2, true, x, -0.42, 0.543, 0.62)), ...[0.19, 0.733, 1.267].map((x, i) => rect(`back-${i + 1}`, G, "body", 2.5, true, x, 0.19, 0.543, 0.26))],
  },
};

const bike: Sym = {
  id: "plan-true",
  title: { en: "Hoop symbol, true extent", de: "Bügelsymbol, wahre Ausdehnung" },
  description: { en: "The Sheffield hoop in all three views at real size: a 0.75 m wide, 0.80 m high round-bar hoop of 48.3 mm tube with its cross bar; seen from above a 0.75 x 0.0483 m bar with the two leg centres marked; the bikes park along the long side.", de: "Der Sheffield-Bügel in allen drei Ansichten in wahrer Größe: ein 0,75 m breiter, 0,80 m hoher Rohrbügel aus 48,3-mm-Rohr mit Querstange; von oben ein Balken 0,75 x 0,0483 m mit den zwei markierten Beinmitten; die Räder parken entlang der Längsseite." },
  note: trueNote({ en: "Tube diameter 0.0483 m; pen widths are paper millimetres.", de: "Rohrdurchmesser 0,0483 m; Stiftbreiten sind Papiermillimeter." }),
  geometry: {
    elevation: [
      { id: "hoop-outer", bone: "stand", role: "body", z: 3, filled: false, tone: 0.5, kind: "path", d: "M -0.375 0 L -0.375 0.55 C -0.375 0.72 -0.25 0.8 0 0.8 C 0.25 0.8 0.375 0.72 0.375 0.55 L 0.375 0" },
      { id: "hoop-inner", bone: "stand", role: "body", z: 3, filled: false, tone: 0.5, kind: "path", d: "M -0.3267 0 L -0.3267 0.55 C -0.3267 0.69 -0.22 0.7517 0 0.7517 C 0.22 0.7517 0.3267 0.69 0.3267 0.55 L 0.3267 0" },
      rect("cross-bar", "stand", "detail", 2.5, false, -0.3267, 0.37, 0.6534, 0.0483),
    ],
    front: [rect("hoop-tube", "stand", "body", 3, true, -0.02415, 0, 0.0483, 0.8, { r: 0.02415 }), poly("cross-bar", "stand", "detail", 3.5, false, false, [[-0.02415, 0.37], [0.02415, 0.37]])],
    plan: [rect("hoop-plan", "stand", "body", 3, true, -0.375, -0.02415, 0.75, 0.0483), ell("leg-plan-l", "stand", "body", 4, true, -0.3508, 0, 0.013283), ell("leg-plan-r", "stand", "body", 4, true, 0.3508, 0, 0.013283)],
  },
};

const bikeMark: Sym = {
  id: "plan-mark",
  title: { en: "Hoop sign", de: "Bügelzeichen" },
  description: { en: "A conventional sign for one stand at small scale: in the top view a 4.4 mm bar with a dot at each end, in the side view an inverted U of 4.4 mm, in the front view a 3.5 mm post; the row of marks shows the parking capacity.", de: "Ein Planzeichen für einen Bügel im kleinen Maßstab: in der Draufsicht ein 4,4-mm-Balken mit einem Punkt an jedem Ende, in der Seitenansicht ein umgekehrtes U von 4,4 mm, in der Vorderansicht ein 3,5-mm-Pfosten; die Zeichenreihe zeigt die Stellplatzzahl." },
  note: { en: "Conventional symbol, not to scale: nominal paper sign in paper millimetres, its 4.4 mm length carries no dimension (the real hoop is 0.75 m = 1.5 mm at 1:500); insertion point at the bar centre on the ground.", de: "Konventionelles Zeichen, nicht maßstäblich: nominelles Papierzeichen in Papiermillimetern, seine Länge von 4,4 mm trägt kein Maß (der reale Bügel ist 0,75 m = 1,5 mm bei 1:500); Einfügepunkt in der Balkenmitte am Boden." },
  geometry: {
    elevation: [poly("hoop-sign", "stand", "body", 3, false, false, [[-2.2, 0], [-2.2, 2.6], [-1.5, 3.5], [1.5, 3.5], [2.2, 2.6], [2.2, 0]]), ell("foot-l", "stand", "body", 4, true, -2.2, 0, 0.45), ell("foot-r", "stand", "body", 4, true, 2.2, 0, 0.45)],
    front: [poly("post-sign", "stand", "body", 3, false, false, [[0, 0], [0, 3.5]]), ell("foot", "stand", "body", 4, true, 0, 0, 0.45)],
    plan: [poly("bar", "stand", "body", 3, false, false, [[-2.2, 0], [2.2, 0]]), ell("foot-l", "stand", "body", 4, true, -2.2, 0, 0.45), ell("foot-r", "stand", "body", 4, true, 2.2, 0, 0.45)],
  },
};

const bollard: Sym = {
  id: "plan-true",
  title: { en: "Bollard symbol, true extent", de: "Pollersymbol, wahre Ausdehnung" },
  description: { en: "The retractable bollard in all three views at real size: the 0.20 m shaft with reflector, beacon and cap, the 0.30 m sleeve and 0.40 m frame ring in the ground, the ram and the lateral power unit below grade; from above the shaft disc inside the sleeve and frame rings.", de: "Der versenkbare Poller in allen drei Ansichten in wahrer Größe: der 0,20-m-Schaft mit Reflektor, Leuchte und Kappe, die 0,30-m-Hülse und der 0,40-m-Rahmenring im Boden, der Stößel und das seitliche Antriebsaggregat unter Terrain; von oben die Schaftscheibe in Hülse und Rahmenring." },
  note: trueNote({ en: "The shaft follows the raised or lowered pose; the power unit lies below grade on the left (+z) of the sleeve and is drawn as hidden below the ground line.", de: "Der Schaft folgt der gehobenen oder gesenkten Haltung; das Antriebsaggregat liegt unter Terrain links (+z) der Hülse und ist als verdeckt unter der Bodenlinie gezeichnet." }),
  geometry: {
    elevation: [
      rect("shaft", "sleeve", "body", 3, true, -0.1, -0.5, 0.2, 0.91), rect("reflector", "sleeve", "detail", 3.2, false, -0.1, 0.41, 0.2, 0.11), rect("beacon", "sleeve", "detail", 3.2, false, -0.1, 0.52, 0.2, 0.06), rect("cap", "sleeve", "body", 3.3, true, -0.1, 0.58, 0.2, 0.07),
      rect("sleeve", "sleeve", "body", 2, false, -0.15, -1.25, 0.3, 1.25), rect("frame-l", "sleeve", "body", 2.2, true, -0.2, -0.07, 0.055, 0.085), rect("frame-r", "sleeve", "body", 2.2, true, 0.145, -0.07, 0.055, 0.085),
      rect("power-unit", "sleeve", "detail", 1, false, 0.15, -0.95, 0.21, 0.4), rect("ram", "sleeve", "detail", 1, false, -0.06, -1.25, 0.12, 0.55), poly("hose", "sleeve", "detail", 1.2, false, false, [[0.06, -0.7], [0.15, -0.7]]),
    ],
    front: [
      rect("shaft", "sleeve", "body", 3, true, -0.1, -0.5, 0.2, 0.91), rect("reflector", "sleeve", "detail", 3.2, false, -0.1, 0.41, 0.2, 0.11), rect("beacon", "sleeve", "detail", 3.2, false, -0.1, 0.52, 0.2, 0.06), rect("cap", "sleeve", "body", 3.3, true, -0.1, 0.58, 0.2, 0.07),
      rect("sleeve", "sleeve", "body", 2, false, -0.15, -1.25, 0.3, 1.25), rect("frame-l", "sleeve", "body", 2.2, true, -0.2, -0.07, 0.055, 0.085), rect("frame-r", "sleeve", "body", 2.2, true, 0.145, -0.07, 0.055, 0.085),
      rect("power-unit", "sleeve", "detail", 1, false, -0.01, -0.95, 0.21, 0.4), rect("ram", "sleeve", "detail", 1, false, -0.06, -1.25, 0.12, 0.55),
    ],
    plan: [ell("frame-plan", "sleeve", "detail", 2.4, false, 0, 0, 0.2), ell("sleeve-plan", "sleeve", "detail", 2.5, false, 0, 0, 0.15), ell("shaft-plan", "sleeve", "body", 3, true, 0, 0, 0.1), rect("power-unit", "sleeve", "detail", 1, false, 0.15, -0.01, 0.21, 0.21), rect("motor", "sleeve", "detail", 1.2, false, 0.22, 0.025, 0.07, 0.14)],
  },
};

const bollardDot: Sym = {
  id: "plan-dot",
  title: { en: "Bollard sign", de: "Pollerzeichen" },
  description: { en: "A conventional sign for one bollard: in the top view a filled 0.9 mm dot; in the side and front view a 0.9 mm post of 2.5 mm with a cap; at the bollard spacing of 1.2 to 1.5 m a row of dots reads as a line of control.", de: "Ein Planzeichen für einen Poller: in der Draufsicht ein gefüllter Punkt von 0,9 mm; in der Seiten- und Vorderansicht ein 0,9-mm-Pfosten von 2,5 mm mit Kappe; beim Pollerabstand von 1,2 bis 1,5 m liest sich eine Punktreihe als Sperrlinie." },
  note: { en: "Conventional symbol, not to scale: the dot marks the real position of the shaft axis (insertion point at the origin on the ground); its 0.9 mm diameter and 2.5 mm height are paper sizes and carry no dimension (the real shaft is 0.2 m = 0.4 mm at 1:500).", de: "Konventionelles Zeichen, nicht maßstäblich: der Punkt markiert die reale Lage der Schaftachse (Einfügepunkt im Ursprung am Boden); sein Durchmesser von 0,9 mm und seine Höhe von 2,5 mm sind Papiermaße und tragen kein Maß (der reale Schaft ist 0,2 m = 0,4 mm bei 1:500)." },
  geometry: {
    elevation: [rect("post", "sleeve", "body", 3, true, -0.45, 0, 0.9, 2.2), ell("cap", "sleeve", "body", 3.1, true, 0, 2.2, 0.45)],
    front: [rect("post", "sleeve", "body", 3, true, -0.45, 0, 0.9, 2.2), ell("cap", "sleeve", "body", 3.1, true, 0, 2.2, 0.45)],
    plan: [ell("dot", "sleeve", "body", 3, true, 0, 0, 0.45)],
  },
};

const jobs: [string, string, Sym][] = [
  ["🛋️furniture", "bar-stool", stool], ["🛋️furniture", "bookshelf", shelf], ["🛋️furniture", "dining-table", table], ["🛋️furniture", "folding-deck-chair", deck], ["🛋️furniture", "picnic-table", picnic], ["🛋️furniture", "single-bed", bed], ["🛋️furniture", "three-seat-sofa", sofa],
  ["🪑️objects", "bicycle-stand", bike], ["🪑️objects", "bicycle-stand", bikeMark], ["🪑️objects", "retractable-bollard", bollard], ["🪑️objects", "retractable-bollard", bollardDot],
];

const matchEnd = (t: string, open: number): number => {
  let depth = 0, str = false;
  for (let i = open; i < t.length; i++) {
    const c = t[i]!;
    if (str) { if (c === "\\") i++; else if (c === '"') str = false; continue; }
    if (c === '"') str = true;
    else if (c === "[") depth++;
    else if (c === "]" && --depth === 0) return i;
  }
  throw new Error("unbalanced " + t.slice(open, open + 60));
};
const files = new Map<string, string>();
for (const [cat, slug, sym] of jobs) {
  const f = `${L}/${cat}/${slug}/🔣️.json`;
  if (!existsSync(f)) throw new Error(f);
  let text = files.get(f) ?? readFileSync(f, "utf8");
  const m = /\n( *)"symbols": \[/.exec(text)!;
  const unit = (/\n( +)"\$schema"/.exec(text)?.[1] ?? "  ").length;
  const key = m.index + m[0].length - 1;
  const end = matchEnd(text, key);
  const list = JSON.parse(text.slice(key, end + 1)) as any[];
  const i = list.findIndex((s) => s.id === sym.id);
  if (i < 0) throw new Error(`${slug} ${sym.id}`);
  const old = list[i];
  list[i] = { ...old, title: sym.title, description: sym.description, geometry: sym.geometry, dimensionalNote: sym.note };
  const body = JSON.stringify(list, null, unit).split("\n").map((l, n) => (n === 0 ? l : m[1] + l)).join("\n");
  text = text.slice(0, key) + body + text.slice(end + 1);
  files.set(f, text);
}
for (const [f, t] of files) writeFileSync(f, t);
console.log("wrote", files.size, "files");
