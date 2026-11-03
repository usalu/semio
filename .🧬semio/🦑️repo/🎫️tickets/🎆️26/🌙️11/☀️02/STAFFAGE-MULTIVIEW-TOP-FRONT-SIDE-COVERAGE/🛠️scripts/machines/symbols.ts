import { load, save } from "./lib.ts";

type Opt = { role?: string; z?: number; tone?: number; filled?: boolean; pen?: number };
type Maker = ReturnType<typeof maker>;

function maker(bone: string, view: string) {
  let z = 1;
  const common = (id: string, o: Opt) => ({ id: `${view}-${id}`, bone, role: o.role ?? "body", z: o.z ?? z++, filled: o.filled ?? true, tone: o.tone ?? 0.5, ...(o.pen ? { penMm: o.pen } : {}) });
  return {
    rect: (id: string, x: number, y: number, w: number, h: number, o: Opt = {}) => ({ kind: "rect", x, y, w, h, ...common(id, o) }),
    poly: (id: string, points: number[][], closed = true, o: Opt = {}) => ({ kind: "poly", points, closed, ...common(id, { ...o, filled: closed ? o.filled ?? true : false }) }),
    ellipse: (id: string, cx: number, cy: number, rx: number, ry: number, o: Opt = {}) => ({ kind: "ellipse", cx, cy, rx, ry, ...common(id, o) }),
  };
}

type Views = { elevation: any[]; front: any[]; plan: any[] };
type Def = { id: string; asset: string; title: [string, string]; description: [string, string]; note: [string, string]; views: (m: (v: string) => Maker) => Views };

const U = { filled: false };
const defs: Def[] = [
  {
    asset: "city-bicycle", id: "bicycle-plan",
    title: ["Bicycle symbol", "Fahrradzeichen"],
    description: ["True-extent symbol of the bicycle in side, front and top view: both wheels, frame triangle, fork, stem and handlebar, saddle, rack and rear lamp; every view carries the real length, width and height.", "Zeichen des Fahrrads in wahrer Ausdehnung in Seiten-, Vorder- und Draufsicht: beide Räder, Rahmendreieck, Gabel, Vorbau und Lenker, Sattel, Gepäckträger und Rücklicht; jede Ansicht trägt die wahre Länge, Breite und Höhe."],
    note: ["Model metres: length 1.87 m, handlebar width 0.62 m, height 1.09 m, tyre width 0.06 m; side, front and top view.", "Modellmeter: Länge 1,87 m, Lenkerbreite 0,62 m, Höhe 1,09 m, Reifenbreite 0,06 m; Seiten-, Vorder- und Draufsicht."],
    views: (m) => {
      const e = m("elevation"), f = m("front"), p = m("plan");
      return {
        elevation: [
          e.ellipse("wheel-r", -0.575, 0.345, 0.345, 0.345, { ...U, tone: 0.15 }),
          e.ellipse("wheel-f", 0.565, 0.345, 0.345, 0.345, { ...U, tone: 0.15 }),
          e.poly("frame", [[-0.575, 0.345], [-0.1, 0.3], [0.35, 0.8], [-0.28, 0.8], [-0.575, 0.345]], false, { tone: 0.4 }),
          e.poly("seat-tube", [[-0.1, 0.3], [-0.35, 0.98]], false, { tone: 0.4 }),
          e.poly("fork", [[0.35, 0.8], [0.565, 0.345]], false, { tone: 0.4 }),
          e.poly("stem", [[0.35, 0.8], [0.3, 0.95], [0.04, 1.09]], false, { tone: 0.4 }),
          e.rect("saddle", -0.49, 0.98, 0.3, 0.08, { tone: 0.3 }),
          e.poly("rack", [[-0.84, 0.8], [-0.3, 0.8]], false, { tone: 0.4 }),
          e.rect("lamp", -0.97, 0.15, 0.08, 0.1, { tone: 0.2 }),
        ],
        front: [
          f.rect("wheels", -0.03, 0, 0.06, 0.69, { tone: 0.15 }),
          f.rect("frame-column", -0.03, 0.23, 0.06, 0.76, { tone: 0.4 }),
          f.rect("saddle", -0.09, 0.98, 0.18, 0.06, { tone: 0.3 }),
          f.rect("fork", -0.05, 0.34, 0.1, 0.47, { tone: 0.4 }),
          f.rect("steer-column", -0.02, 0.78, 0.04, 0.31, { tone: 0.4 }),
          f.rect("handlebar", -0.31, 1.03, 0.62, 0.05, { tone: 0.4 }),
        ],
        plan: [
          p.rect("wheel-r", -0.92, -0.03, 0.69, 0.06, { tone: 0.15 }),
          p.rect("wheel-f", 0.22, -0.03, 0.69, 0.06, { tone: 0.15 }),
          p.poly("frame", [[-0.58, 0], [0.35, 0]], false, { tone: 0.4 }),
          p.rect("rack", -0.97, -0.09, 0.78, 0.18, { tone: 0.3 }),
          p.ellipse("saddle", -0.34, 0, 0.15, 0.07, { tone: 0.3 }),
          p.poly("steer", [[0.35, 0], [0.04, 0]], false, { tone: 0.4 }),
          p.poly("handlebar", [[0.04, -0.31], [0.04, 0.31]], false, { tone: 0.4 }),
          p.poly("pedals", [[-0.1, -0.2], [-0.1, 0.2]], false, { tone: 0.4 }),
        ],
      };
    },
  },
  {
    asset: "delivery-van", id: "van-plan",
    title: ["Delivery van symbol", "Lieferwagenzeichen"],
    description: ["True-extent symbol of the van in side, front and top view: body outline, cab glazing, wheels, mirrors and the sliding door on the right-hand side.", "Zeichen des Lieferwagens in wahrer Ausdehnung in Seiten-, Vorder- und Draufsicht: Karosserieumriss, Kabinenverglasung, Räder, Spiegel und die Schiebetür auf der rechten Seite."],
    note: ["Model metres: length 5.34 m, width over the mirrors 1.99 m, height 1.99 m; side, front and top view.", "Modellmeter: Länge 5,34 m, Breite über die Spiegel 1,99 m, Höhe 1,99 m; Seiten-, Vorder- und Draufsicht."],
    views: (m) => {
      const e = m("elevation"), f = m("front"), p = m("plan");
      return {
        elevation: [
          e.poly("body", [[-2.67, 0.36], [-2.67, 1.99], [1.15, 1.99], [1.85, 1.35], [2.67, 1.2], [2.67, 0.36]], true, { tone: 0.6 }),
          e.ellipse("wheel-f", 1.7, 0.355, 0.36, 0.355, { tone: 0.1 }),
          e.ellipse("wheel-r", -1.6, 0.355, 0.36, 0.355, { tone: 0.1 }),
          e.poly("windscreen", [[1.14, 1.13], [1.8, 1.13], [1.14, 1.93]], true, { ...U, tone: 0.9 }),
          e.rect("door-window", 0.45, 1.2, 1.15, 0.58, { ...U, tone: 0.9 }),
          e.rect("slider", -0.8, 0.36, 1.05, 1.44, { ...U, tone: 0.4 }),
        ],
        front: [
          f.rect("body", -0.91, 0.36, 1.82, 1.63, { tone: 0.6 }),
          f.rect("tyre-l", 0.74, 0, 0.2, 0.71, { tone: 0.1 }),
          f.rect("tyre-r", -0.94, 0, 0.2, 0.71, { tone: 0.1 }),
          f.rect("windscreen", -0.84, 1.13, 1.68, 0.8, { ...U, tone: 0.9 }),
          f.rect("mirror-l", 0.91, 1.19, 0.085, 0.22, { tone: 0.3 }),
          f.rect("mirror-r", -0.995, 1.19, 0.085, 0.22, { tone: 0.3 }),
        ],
        plan: [
          p.poly("body", [[-2.67, -0.91], [2.3, -0.91], [2.67, -0.75], [2.67, 0.75], [2.3, 0.91], [-2.67, 0.91]], true, { tone: 0.6 }),
          p.rect("tyre-fl", 1.34, 0.74, 0.72, 0.2, { tone: 0.1 }),
          p.rect("tyre-fr", 1.34, -0.94, 0.72, 0.2, { tone: 0.1 }),
          p.rect("tyre-rl", -1.96, 0.74, 0.72, 0.2, { tone: 0.1 }),
          p.rect("tyre-rr", -1.96, -0.94, 0.72, 0.2, { tone: 0.1 }),
          p.rect("windscreen", 1.14, -0.84, 0.66, 1.68, { ...U, tone: 0.9 }),
          p.rect("slider", -0.8, -0.91, 1.05, 0.05, { tone: 0.4 }),
          p.poly("rear-doors", [[-2.67, -0.88], [-2.67, 0.88]], false, { tone: 0.4 }),
          p.rect("mirror-l", 1.91, 0.91, 0.14, 0.085, { tone: 0.3 }),
          p.rect("mirror-r", 1.91, -0.995, 0.14, 0.085, { tone: 0.3 }),
        ],
      };
    },
  },
  {
    asset: "electric-kick-scooter", id: "kick-scooter-plan",
    title: ["E-scooter symbol", "E-Scooter-Zeichen"],
    description: ["True-extent symbol of the e-scooter in side, front and top view: deck, both wheels, steering fork and stem, handlebar.", "Zeichen des E-Scooters in wahrer Ausdehnung in Seiten-, Vorder- und Draufsicht: Trittbrett, beide Räder, Gabel und Lenkstange, Lenker."],
    note: ["Model metres: length 1.09 m, handlebar width 0.43 m, height 1.11 m; side, front and top view.", "Modellmeter: Länge 1,09 m, Lenkerbreite 0,43 m, Höhe 1,11 m; Seiten-, Vorder- und Draufsicht."],
    views: (m) => {
      const e = m("elevation"), f = m("front"), p = m("plan");
      return {
        elevation: [
          e.ellipse("wheel-f", 0.43, 0.11, 0.11, 0.11, { tone: 0.1 }),
          e.ellipse("wheel-r", -0.43, 0.11, 0.11, 0.11, { tone: 0.1 }),
          e.rect("deck", -0.41, 0.1, 0.79, 0.05, { tone: 0.4 }),
          e.poly("fork", [[0.43, 0.11], [0.4, 0.3], [0.29, 1.05]], false, { tone: 0.4 }),
          e.rect("grip", 0.25, 1.05, 0.07, 0.06, { tone: 0.4 }),
        ],
        front: [
          f.rect("wheel", -0.02, 0, 0.04, 0.22, { tone: 0.1 }),
          f.rect("deck", -0.09, 0.1, 0.18, 0.05, { tone: 0.4 }),
          f.rect("stem", -0.03, 0.28, 0.06, 0.83, { tone: 0.4 }),
          f.rect("bars", -0.215, 1.05, 0.43, 0.06, { tone: 0.4 }),
        ],
        plan: [
          p.rect("wheel-f", 0.32, -0.02, 0.22, 0.04, { tone: 0.1 }),
          p.rect("wheel-r", -0.54, -0.02, 0.22, 0.04, { tone: 0.1 }),
          p.rect("deck", -0.41, -0.09, 0.79, 0.18, { tone: 0.4 }),
          p.poly("stem", [[0.27, 0], [0.43, 0]], false, { tone: 0.4 }),
          p.rect("bars", 0.26, -0.215, 0.06, 0.43, { tone: 0.4 }),
        ],
      };
    },
  },
  {
    asset: "heat-pump-outdoor-unit", id: "site-plan-footprint",
    title: ["Outdoor unit symbol", "Außengerätezeichen"],
    description: ["True-extent symbol of the 0.46 x 1.10 m outdoor unit in side, front and top view: casing, lid, the fan opening on the discharge face (offset to the left) and the discharge vane.", "Zeichen des 0,46 x 1,10 m grossen Aussengeräts in wahrer Ausdehnung in Seiten-, Vorder- und Draufsicht: Gehäuse, Deckel, die Ventilatoröffnung auf der Ausblasseite (nach links versetzt) und die Ausblaslamelle."],
    note: ["Model metres: casing 0.46 m deep x 1.10 m wide x 1.41 m high with the vane reaching 0.48 m ahead; side, front and top view.", "Modellmeter: Gehäuse 0,46 m tief x 1,10 m breit x 1,41 m hoch, die Lamelle ragt 0,48 m nach vorn; Seiten-, Vorder- und Draufsicht."],
    views: (m) => {
      const e = m("elevation"), f = m("front"), p = m("plan");
      return {
        elevation: [
          e.rect("casing", -0.23, 0, 0.46, 1.41, { tone: 0.6 }),
          e.rect("lid", -0.23, 1.4, 0.46, 0.05, { tone: 0.5 }),
          e.rect("fan-guard", 0.21, 0.45, 0.05, 0.7, { role: "detail", tone: 0.5 }),
          e.rect("vane", 0.23, 0.72, 0.25, 0.11, { tone: 0.4 }),
        ],
        front: [
          f.rect("casing", -0.55, 0, 1.1, 1.41, { tone: 0.6 }),
          f.rect("lid", -0.55, 1.4, 1.1, 0.05, { tone: 0.5 }),
          f.ellipse("fan-opening", 0.12, 0.8, 0.35, 0.35, { role: "detail", ...U, tone: 0.5 }),
          f.ellipse("fan-hub", 0.12, 0.8, 0.06, 0.06, { role: "detail", tone: 0.4 }),
          f.rect("vane", -0.24, 0.72, 0.72, 0.11, { tone: 0.4 }),
        ],
        plan: [
          p.rect("casing", -0.23, -0.55, 0.46, 1.1, { tone: 0.6 }),
          p.rect("fan-guard", 0.23, -0.23, 0.03, 0.7, { role: "detail", tone: 0.5 }),
          p.rect("vane", 0.23, -0.24, 0.25, 0.72, { tone: 0.4 }),
        ],
      };
    },
  },
  {
    asset: "motor-scooter", id: "scooter-plan",
    title: ["Motor scooter symbol", "Motorrollerzeichen"],
    description: ["True-extent symbol of the motor scooter in side, front and top view: body, seat, both wheels, steering fork, cowl and handlebar.", "Zeichen des Motorrollers in wahrer Ausdehnung in Seiten-, Vorder- und Draufsicht: Karosserie, Sitz, beide Räder, Gabel, Lenkerverkleidung und Lenker."],
    note: ["Model metres: length 1.85 m, handlebar width 0.74 m, height 1.15 m; side, front and top view.", "Modellmeter: Länge 1,85 m, Lenkerbreite 0,74 m, Höhe 1,15 m; Seiten-, Vorder- und Draufsicht."],
    views: (m) => {
      const e = m("elevation"), f = m("front"), p = m("plan");
      return {
        elevation: [
          e.ellipse("wheel-f", 0.67, 0.23, 0.23, 0.23, { tone: 0.1 }),
          e.ellipse("wheel-r", -0.67, 0.23, 0.23, 0.23, { tone: 0.1 }),
          e.poly("body", [[-0.94, 0.4], [-0.9, 0.95], [-0.45, 1.0], [-0.2, 0.87], [0, 0.3], [0.45, 0.26], [0.56, 0.6], [0.4, 0.9], [-0.2, 0.7]], true, { tone: 0.5 }),
          e.rect("seat", -0.82, 0.67, 0.6, 0.2, { tone: 0.3 }),
          e.poly("fork", [[0.67, 0.23], [0.42, 0.98]], false, { tone: 0.4 }),
          e.poly("cowl", [[0.33, 0.9], [0.55, 0.9], [0.5, 1.15], [0.36, 1.15]], true, { tone: 0.4 }),
        ],
        front: [
          f.rect("wheel", -0.06, 0, 0.12, 0.46, { tone: 0.1 }),
          f.rect("body", -0.21, 0.26, 0.42, 0.74, { tone: 0.5 }),
          f.rect("fork", -0.09, 0.23, 0.18, 0.75, { tone: 0.4 }),
          f.rect("cowl-bar", -0.37, 0.93, 0.74, 0.22, { tone: 0.4 }),
        ],
        plan: [
          p.rect("wheel-r", -0.9, -0.06, 0.46, 0.12, { tone: 0.1 }),
          p.rect("wheel-f", 0.44, -0.07, 0.46, 0.14, { tone: 0.1 }),
          p.poly("body", [[-0.94, -0.17], [-0.5, -0.2], [0.3, -0.2], [0.56, -0.1], [0.56, 0.1], [0.3, 0.2], [-0.5, 0.2], [-0.94, 0.17]], true, { tone: 0.5 }),
          p.rect("seat", -0.8, -0.15, 0.58, 0.3, { tone: 0.3 }),
          p.rect("handlebar", 0.37, -0.37, 0.06, 0.74, { tone: 0.4 }),
        ],
      };
    },
  },
  {
    asset: "scissor-lift", id: "site-plan-footprint",
    title: ["Scissor lift symbol", "Scherenarbeitsbühnenzeichen"],
    description: ["True-extent symbol of the stowed scissor lift in side, front and top view: chassis, tyres, folded scissor stack, platform with guard rails.", "Zeichen der eingefahrenen Scherenarbeitsbühne in wahrer Ausdehnung in Seiten-, Vorder- und Draufsicht: Fahrgestell, Reifen, zusammengefaltetes Scherenpaket, Plattform mit Geländer."],
    note: ["Model metres: footprint 1.85 x 0.81 m, stowed height with guard rails 2.20 m; the deck extension is retracted; side, front and top view.", "Modellmeter: Grundfläche 1,85 x 0,81 m, eingefahrene Höhe mit Geländer 2,20 m; die Plattformverlängerung ist eingefahren; Seiten-, Vorder- und Draufsicht."],
    views: (m) => {
      const e = m("elevation"), f = m("front"), p = m("plan");
      return {
        elevation: [
          e.rect("chassis", -0.925, 0.1, 1.85, 0.35, { tone: 0.6 }),
          e.ellipse("tyre-f", 0.55, 0.27, 0.27, 0.27, { tone: 0.1 }),
          e.ellipse("tyre-r", -0.55, 0.27, 0.27, 0.27, { tone: 0.1 }),
          e.rect("stack", -0.92, 0.45, 1.84, 0.55, { role: "detail", ...U, tone: 0.4 }),
          e.rect("deck", -0.925, 1, 1.85, 0.1, { tone: 0.5 }),
          e.rect("toe-board", -0.925, 1.1, 1.85, 0.2, { role: "detail", ...U, tone: 0.4 }),
          e.rect("mid-rail", -0.925, 1.62, 1.85, 0.07, { role: "detail", tone: 0.4 }),
          e.rect("top-rail", -0.925, 2.13, 1.85, 0.07, { role: "detail", tone: 0.4 }),
          e.rect("post-r", -0.925, 1.1, 0.07, 1.1, { role: "detail", tone: 0.4 }),
          e.rect("post-f", 0.85, 1.1, 0.075, 1.1, { role: "detail", tone: 0.4 }),
        ],
        front: [
          f.rect("chassis", -0.37, 0.1, 0.74, 0.35, { tone: 0.6 }),
          f.rect("tyre-l", 0.24, 0, 0.17, 0.54, { tone: 0.1 }),
          f.rect("tyre-r", -0.41, 0, 0.17, 0.54, { tone: 0.1 }),
          f.rect("stack", -0.39, 0.45, 0.78, 0.55, { role: "detail", ...U, tone: 0.4 }),
          f.rect("deck", -0.37, 1, 0.74, 0.1, { tone: 0.5 }),
          f.rect("toe-board", -0.37, 1.1, 0.74, 0.15, { role: "detail", ...U, tone: 0.4 }),
          f.rect("mid-rail", -0.37, 1.62, 0.74, 0.07, { role: "detail", tone: 0.4 }),
          f.rect("top-rail", -0.37, 2.13, 0.74, 0.07, { role: "detail", tone: 0.4 }),
          f.rect("post-l", 0.3, 1.1, 0.07, 1.1, { role: "detail", tone: 0.4 }),
          f.rect("post-r", -0.37, 1.1, 0.07, 1.1, { role: "detail", tone: 0.4 }),
        ],
        plan: [
          p.rect("chassis", -0.925, -0.405, 1.85, 0.81, { tone: 0.6 }),
          p.rect("tyre-fl", 0.28, 0.24, 0.54, 0.165, { tone: 0.1 }),
          p.rect("tyre-fr", 0.28, -0.405, 0.54, 0.165, { tone: 0.1 }),
          p.rect("tyre-rl", -0.82, 0.24, 0.54, 0.165, { tone: 0.1 }),
          p.rect("tyre-rr", -0.82, -0.405, 0.54, 0.165, { tone: 0.1 }),
          p.rect("deck", -0.925, -0.37, 1.85, 0.74, { role: "detail", ...U, tone: 0.5 }),
          p.rect("rail-l", -0.925, 0.3, 1.85, 0.07, { role: "detail", tone: 0.4 }),
          p.rect("rail-r", -0.925, -0.37, 1.85, 0.07, { role: "detail", tone: 0.4 }),
          p.rect("rail-f", 0.85, -0.37, 0.075, 0.74, { role: "detail", tone: 0.4 }),
          p.rect("rail-b", -0.925, -0.37, 0.075, 0.74, { role: "detail", tone: 0.4 }),
        ],
      };
    },
  },
  {
    asset: "tower-crane", id: "site-plan-reach",
    title: ["Tower crane symbol", "Turmdrehkranzeichen"],
    description: ["True-extent symbol of the tower crane in side, front and top view: ballasted foundation, mast, slewing platform, tower head, jib with trolley, counter-jib with counterweight, winch house, cab and the pendant ties.", "Zeichen des Turmdrehkrans in wahrer Ausdehnung in Seiten-, Vorder- und Draufsicht: ballastiertes Fundament, Turm, Drehbühne, Turmspitze, Ausleger mit Laufkatze, Gegenausleger mit Gegengewicht, Windenhaus, Kabine und Abspannungen."],
    note: ["Model metres: jib 55 m, counter-jib 16.5 m, mast 2 x 2 m, foundation 7 x 3.5 m, top of the tower head 48.6 m; the jib points ahead; side, front and top view; the swept reach circle is not part of the symbol.", "Modellmeter: Ausleger 55 m, Gegenausleger 16,5 m, Turm 2 x 2 m, Fundament 7 x 3,5 m, Oberkante der Turmspitze 48,6 m; der Ausleger zeigt nach vorn; Seiten-, Vorder- und Draufsicht; der überstrichene Reichweitenkreis gehört nicht zum Zeichen."],
    views: (m) => {
      const e = m("elevation"), f = m("front"), p = m("plan");
      return {
        elevation: [
          e.rect("ballast", -3.5, 0, 7, 1, { tone: 0.6 }),
          e.rect("mast", -1, 1, 2, 39, { tone: 0.45 }),
          e.rect("slew-platform", -2.4, 40, 5.8, 1.2, { tone: 0.5 }),
          e.rect("tower-head", -1.2, 41.2, 2.4, 7.4, { tone: 0.45 }),
          e.rect("jib", 0, 43.6, 55, 1.6, { tone: 0.35 }),
          e.rect("counter-jib", -16.5, 43.7, 16.5, 1.4, { tone: 0.35 }),
          e.rect("counterweight", -16.1, 41.5, 2.4, 2.2, { tone: 0.6 }),
          e.rect("cab", 1.4, 41.2, 1.9, 2.2, { role: "detail", tone: 0.4 }),
          e.rect("winch-house", -10, 45.15, 4, 1.8, { tone: 0.5 }),
          e.poly("pendant-fore", [[0, 48.6], [50, 45.2]], false, { role: "detail", tone: 0.4 }),
          e.poly("pendant-back", [[0, 48.6], [-15.6, 45.1]], false, { role: "detail", tone: 0.4 }),
          e.rect("trolley", 49.3, 43.05, 1.4, 0.55, { tone: 0.5 }),
        ],
        front: [
          f.rect("ballast", -1.75, 0, 3.5, 1, { tone: 0.6 }),
          f.rect("mast", -1, 1, 2, 39, { tone: 0.45 }),
          f.rect("slew-platform", -1.2, 40.5, 2.4, 0.7, { tone: 0.5 }),
          f.rect("tower-head", -0.9, 41.2, 1.8, 7.4, { tone: 0.45 }),
          f.rect("jib", -0.8, 43.6, 1.6, 1.6, { tone: 0.35 }),
          f.rect("counterweight", -1.2, 41.5, 2.4, 2.2, { tone: 0.6 }),
          f.rect("cab", -1, 41.2, 1, 2.2, { role: "detail", tone: 0.4 }),
          f.rect("winch-house", -0.7, 45.15, 1.4, 1.8, { tone: 0.5 }),
          f.rect("trolley", -1, 43.05, 2, 0.55, { tone: 0.5 }),
        ],
        plan: [
          p.rect("ballast", -3.5, -1.75, 7, 3.5, { tone: 0.6 }),
          p.rect("mast", -1, -1, 2, 2, { tone: 0.45 }),
          p.rect("slew-platform", -2.4, -1.2, 5.8, 2.4, { tone: 0.5 }),
          p.rect("tower-head", -1.2, -0.9, 2.4, 1.8, { tone: 0.45 }),
          p.rect("jib", 0, -0.8, 55, 1.6, { tone: 0.35 }),
          p.rect("counter-jib", -16.5, -0.7, 16.5, 1.4, { tone: 0.35 }),
          p.rect("counterweight", -16.1, -1.2, 2.4, 2.4, { tone: 0.6 }),
          p.rect("cab", 1.4, -1, 1.9, 1, { role: "detail", tone: 0.4 }),
          p.rect("winch-house", -10, -0.7, 4, 1.4, { tone: 0.5 }),
          p.rect("trolley", 49.3, -1, 1.4, 2, { tone: 0.5 }),
        ],
      };
    },
  },
  {
    asset: "tram", id: "tram-plan",
    title: ["Tram symbol", "Strassenbahnzeichen"],
    description: ["True-extent symbol of the tram in side, front and top view: end module, bellows gangway, middle module, wheelsets, windows, the right-hand doors and the pantograph.", "Zeichen der Strassenbahn in wahrer Ausdehnung in Seiten-, Vorder- und Draufsicht: Endmodul, Faltenbalg, Mittelmodul, Radsätze, Fenster, die Türen auf der rechten Seite und der Stromabnehmer."],
    note: ["Model metres: length 15.45 m, width 2.40 m, height with the raised pantograph 5.0 m, cut at the rear of the middle module; side, front and top view.", "Modellmeter: Länge 15,45 m, Breite 2,40 m, Höhe mit gehobenem Stromabnehmer 5,0 m, am Heck des Mittelmoduls abgeschnitten; Seiten-, Vorder- und Draufsicht."],
    views: (m) => {
      const e = m("elevation"), f = m("front"), p = m("plan");
      return {
        elevation: [
          e.rect("shell-end", 0.45, 0.28, 9, 3.12, { tone: 0.6 }),
          e.rect("shell-mid", -6, 0.28, 5.55, 3.12, { tone: 0.6 }),
          e.rect("gangway", -0.45, 0.45, 0.9, 2.8, { ...U, tone: 0.2 }),
          e.ellipse("wheel-r1", 1, 0.3, 0.3, 0.3, { tone: 0.1 }),
          e.ellipse("wheel-r2", 2.8, 0.3, 0.3, 0.3, { tone: 0.1 }),
          e.ellipse("wheel-f1", 6.3, 0.3, 0.3, 0.3, { tone: 0.1 }),
          e.ellipse("wheel-f2", 8.1, 0.3, 0.3, 0.3, { tone: 0.1 }),
          e.rect("window-end", 0.9, 1.25, 7.55, 1.5, { ...U, tone: 0.9 }),
          e.rect("window-mid", -5.85, 1.25, 5.15, 1.5, { ...U, tone: 0.9 }),
          e.rect("door-a", 3.5, 0.28, 1.3, 2.42, { ...U, tone: 0.4 }),
          e.rect("door-b", -4, 0.28, 1.3, 2.42, { ...U, tone: 0.4 }),
          e.poly("pantograph", [[2.6, 3.4], [4.36, 4.26], [2.6, 4.97]], false, { tone: 0.3 }),
          e.rect("pan-head", 2.29, 4.9, 0.62, 0.12, { tone: 0.3 }),
        ],
        front: [
          f.rect("shell", -1.2, 0.28, 2.4, 3.12, { tone: 0.6 }),
          f.rect("windscreen", -1.05, 1.25, 2.1, 1.8, { ...U, tone: 0.9 }),
          f.rect("wheel-l", 0.65, 0, 0.13, 0.6, { tone: 0.1 }),
          f.rect("wheel-r", -0.78, 0, 0.13, 0.6, { tone: 0.1 }),
          f.rect("pan-arm", -0.06, 3.4, 0.12, 1.6, { tone: 0.3 }),
          f.rect("pan-head", -0.6, 4.9, 1.2, 0.12, { tone: 0.3 }),
        ],
        plan: [
          p.poly("shell-end", [[0.45, -1.2], [8.8, -1.2], [9.45, -0.7], [9.45, 0.7], [8.8, 1.2], [0.45, 1.2]], true, { tone: 0.6 }),
          p.rect("shell-mid", -6, -1.2, 5.55, 2.4, { tone: 0.6 }),
          p.rect("gangway", -0.45, -1, 0.9, 2, { tone: 0.2 }),
          p.rect("door-a", 3.5, -1.2, 1.3, 0.1, { tone: 0.4 }),
          p.rect("door-b", -4, -1.2, 1.3, 0.1, { tone: 0.4 }),
          p.rect("pan-head", 2.29, -0.6, 0.62, 1.2, { ...U, tone: 0.3 }),
          p.poly("pan-arm", [[2.54, 0], [4.42, 0]], false, { tone: 0.3 }),
        ],
      };
    },
  },
  {
    asset: "wheel-loader", id: "articulated-steering-plan",
    title: ["Wheel loader symbol", "Radladerzeichen"],
    description: ["True-extent symbol of the wheel loader driving straight, in side, front and top view: rear frame with counterweight and engine bay, front frame, cab, articulation joint, four tyres, lift arms and the bucket.", "Zeichen des geradeaus fahrenden Radladers in wahrer Ausdehnung in Seiten-, Vorder- und Draufsicht: Hinterwagen mit Gegengewicht und Motorraum, Vorderwagen, Kabine, Knickgelenk, vier Reifen, Hubarme und Schaufel."],
    note: ["Model metres: length with the bucket on the ground 8.9 m, bucket width 2.95 m, height 3.4 m, tyres 1.75 m diameter and 0.6 m wide; side, front and top view.", "Modellmeter: Länge mit der Schaufel am Boden 8,9 m, Schaufelbreite 2,95 m, Höhe 3,4 m, Reifen 1,75 m Durchmesser und 0,6 m breit; Seiten-, Vorder- und Draufsicht."],
    views: (m) => {
      const e = m("elevation"), f = m("front"), p = m("plan");
      return {
        elevation: [
          e.ellipse("tyre-r", -1.695, 0.875, 0.875, 0.875, { tone: 0.15 }),
          e.ellipse("tyre-f", 1.705, 0.875, 0.875, 0.875, { tone: 0.15 }),
          e.ellipse("rim-r", -1.695, 0.875, 0.5, 0.5, { role: "detail", tone: 0.4 }),
          e.ellipse("rim-f", 1.705, 0.875, 0.5, 0.5, { role: "detail", tone: 0.4 }),
          e.rect("counterweight", -3.5, 0.62, 0.9, 1.78, { tone: 0.6 }),
          e.rect("hood", -3, 0.62, 2.1, 1.78, { tone: 0.5 }),
          e.rect("front-frame", -0.9, 0.62, 3.25, 1.58, { tone: 0.5 }),
          e.rect("cab", -0.9, 1.45, 1.9, 1.95, { tone: 0.45 }),
          e.rect("cab-glass", 0.2, 2.1, 0.58, 1.1, { role: "detail", ...U, tone: 0.9 }),
          e.rect("exhaust-stack", -1.32, 2.3, 0.18, 0.85, { role: "detail", tone: 0.3 }),
          e.poly("lift-arm", [[0.76, 1.1], [0.9, 0.86], [3.4, 1.1], [3.3, 1.5]], true, { tone: 0.4 }),
          e.poly("bucket", [[3.15, 1.7], [3.4, 0.43], [5.4, 0.43], [5.3, 1.1], [4, 1.76]], true, { tone: 0.35 }),
        ],
        front: [
          f.rect("counterweight", -1.1, 0.62, 2.2, 1.78, { tone: 0.6 }),
          f.rect("hood", -1, 0.62, 2, 1.78, { tone: 0.5 }),
          f.rect("front-frame", -0.9, 0.62, 1.8, 0.58, { tone: 0.5 }),
          f.rect("cab", -0.8, 1.45, 1.6, 1.95, { tone: 0.45 }),
          f.rect("cab-glass", -0.65, 2.1, 1.3, 1.1, { role: "detail", ...U, tone: 0.9 }),
          f.rect("exhaust-stack", -0.99, 2.4, 0.18, 0.75, { role: "detail", tone: 0.3 }),
          f.rect("tyre-l", 0.8, 0, 0.6, 1.75, { tone: 0.15 }),
          f.rect("tyre-r", -1.4, 0, 0.6, 1.75, { tone: 0.15 }),
          f.rect("lift-arm-l", 0.9, 0.86, 0.3, 1.52, { role: "detail", tone: 0.4 }),
          f.rect("lift-arm-r", -1.2, 0.86, 0.3, 1.52, { role: "detail", tone: 0.4 }),
          f.rect("bucket", -1.475, 0.43, 2.95, 1.33, { tone: 0.35 }),
        ],
        plan: [
          p.rect("counterweight", -3.5, -1.1, 0.9, 2.2, { tone: 0.6 }),
          p.rect("hood", -3, -1, 2.1, 2, { tone: 0.5 }),
          p.rect("front-frame", -0.9, -0.9, 3.25, 1.8, { tone: 0.5 }),
          p.rect("cab", -0.9, -0.8, 1.9, 1.6, { tone: 0.45 }),
          p.rect("exhaust-stack", -1.32, -0.99, 0.18, 0.18, { role: "detail", tone: 0.3 }),
          p.rect("tyre-fl", 0.82, 0.8, 1.75, 0.6, { tone: 0.15 }),
          p.rect("tyre-fr", 0.82, -1.4, 1.75, 0.6, { tone: 0.15 }),
          p.rect("tyre-rl", -2.58, 0.8, 1.75, 0.6, { tone: 0.15 }),
          p.rect("tyre-rr", -2.58, -1.4, 1.75, 0.6, { tone: 0.15 }),
          p.rect("lift-arm-l", 0.76, 0.9, 3.12, 0.3, { role: "detail", tone: 0.4 }),
          p.rect("lift-arm-r", 0.76, -1.2, 3.12, 0.3, { role: "detail", tone: 0.4 }),
          p.rect("bucket", 3.15, -1.475, 2.25, 2.95, { tone: 0.35 }),
          p.ellipse("articulation-joint", 0, 0, 0.25, 0.25, { role: "detail", tone: 0.1 }),
        ],
      };
    },
  },
];

for (const def of defs) {
  const doc = load(def.asset);
  const symbol = doc.asset.symbols.find((s: any) => s.id === def.id);
  const bone = doc.asset.rig.root;
  const views = def.views((v) => maker(bone, v));
  symbol.title = { en: def.title[0], de: def.title[1] };
  symbol.description = { en: def.description[0], de: def.description[1] };
  symbol.dimensionalNote = { en: def.note[0], de: def.note[1] };
  symbol.geometry = views;
  save(def.asset, doc);
  console.log("symbol", def.asset, def.id, bone);
}
