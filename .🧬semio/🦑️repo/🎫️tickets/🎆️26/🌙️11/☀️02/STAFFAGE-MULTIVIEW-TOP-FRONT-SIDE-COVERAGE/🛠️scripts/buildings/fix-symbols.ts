import { load, save, shapes as rawShapes, sign, poly, rect, text, clone, worldOf } from "./lib.ts";
let CURRENT: any = null;
const shapes = (a: any, scale: number, view: string): any[] => rawShapes(a, scale, view);
const W = (a: any, view: string, list: any[]): any[] => worldOf(a, view, list);

const ALL = ["cantilever-balcony", "curtain-wall-bay", "external-venetian-blind", "glass-balustrade", "glulam-beam", "masonry-wall-with-lintel", "panel-radiator", "sectional-garage-door", "sliding-door", "spiral-stair", "stair-flight", "steel-column", "tied-canopy", "tilt-window", "wheelchair-ramp", "ceiling-diffuser", "circulation-pump", "decentralised-hrv-fan", "desktop-monitor", "duct-fan", "electric-fan-heater", "electric-kettle", "electricity-meter", "gas-boiler", "gas-hob", "gas-meter", "hot-water-cylinder"];

for (const id of ALL) {
  const doc = load(id);
  const reps = doc.asset.representations;
  for (const k of Object.keys(reps)) {
    const r = reps[k];
    if (r.status === "unavailable" && r.cause === undefined) {
      const { status, denominator, ...rest } = r;
      reps[k] = { status, denominator, cause: "below-printable-feature", ...rest };
    }
  }
  save(id, doc);
}

const setSymbol = (doc: any, symbolId: string, patch: (s: any) => void) => patch(doc.asset.symbols.find((s: any) => s.id === symbolId));

{
  const doc = load("glass-balustrade");
  const a = doc.asset;
  setSymbol(doc, "balustrade-plan", (s) => {
    s.title = text("Glass balustrade, conventional sign", "Glasbrüstung, Zeichen");
    s.description = text(
      "The conventional sign of a glass balustrade panel in all three views at real size: in plan the tube handrail as a narrow bar over the glass line with the floor shoe as an outline; in the front view the panel face with shoe, glass and handrail bar; in the side view the section across the panel with the shoe block, the glass line and the handrail disc.",
      "Das Zeichen eines Glasbrüstungsfeldes in allen drei Ansichten in wahrer Größe: im Grundriss der Rohrhandlauf als schmaler Balken über der Glaslinie mit dem Bodenschuh als Umriss; in der Vorderansicht die Feldfläche mit Schuh, Glas und Handlaufbalken; in der Seitenansicht der Schnitt quer zum Feld mit Schuhblock, Glaslinie und Handlaufscheibe.",
    );
    s.dimensionalNote = text(
      "Real size in every view: panel length 2.00 m, height 1.10 m, shoe 0.30 m deep; the handrail tube is 53 mm. At 1:500 the sign is 4 mm long and the handrail is drawn by its pen.",
      "Echte Größe in jeder Ansicht: Feldlänge 2,00 m, Höhe 1,10 m, Schuh 0,30 m tief; das Handlaufrohr misst 53 mm. Bei 1:500 ist das Zeichen 4 mm lang und der Handlauf entsteht durch die Federbreite.",
    );
    s.geometry.elevation = W(a, "elevation", sign(shapes(a, 200, "elevation"), { unfill: /shoe-block/ }));
    s.geometry.front = W(a, "front", sign(shapes(a, 200, "front"), { unfill: /shoe-front|glass-front/ }));
    s.geometry.plan = [
      ...W(a, "plan", sign(shapes(a, 200, "plan"), { unfill: /shoe-plan/ })),
      poly("glass-line", "panel", "glazing", [[0, -1], [0, 1]], false, { region: "panel-glass" }),
    ];
  });
  save("glass-balustrade", doc);
}

{
  const doc = load("sliding-door");
  const a = doc.asset;
  const front200 = a.representations["200"].geometry.front;
  if (!front200.some((s: any) => s.id === "leaf-behind")) front200.push(clone(a.representations["100"].geometry.front.find((s: any) => s.id === "leaf-behind")));
  setSymbol(doc, "sliding-door-plan", (s) => {
    s.title = text("Sliding door, conventional sign", "Schiebetür, Zeichen");
    s.description = text(
      "The conventional sign of a sliding door in all three views at real size: in plan the leaf as a thin glass rectangle in its track, the jambs as filled squares and an arrow in the sliding direction (DIN 1356-1); in the side view the facade of jambs, head track and leaf with the same arrow; in the front view the door edge-on with the leaf behind the jamb.",
      "Das Zeichen einer Schiebetür in allen drei Ansichten in wahrer Größe: im Grundriss der Flügel als dünnes Glasrechteck in seiner Schiene, die Pfosten als gefüllte Quadrate und ein Pfeil in Schieberichtung (DIN 1356-1); in der Seitenansicht die Fassade aus Pfosten, Kopfschiene und Flügel mit demselben Pfeil; in der Vorderansicht die Tür auf der Kante mit dem Flügel hinter dem Pfosten.",
    );
    s.dimensionalNote = text(
      "Real size: 3.10 m between the outer jamb faces by 0.10 m deep and 2.15 m high; the leaf is 1.00 m long and 50 mm thick. The arrow marks the sliding direction and has no dimension.",
      "Echte Größe: 3,10 m über die äußeren Pfostenkanten bei 0,10 m Tiefe und 2,15 m Höhe; der Flügel ist 1,00 m lang und 50 mm dick. Der Pfeil markiert die Schieberichtung und hat keine Abmessung.",
    );
    s.geometry.elevation = [
      ...W(a, "elevation", sign(shapes(a, 200, "elevation"))),
      poly("slide-arrow", "track", "detail", [[2.05, 1], [2.9, 1]]),
      poly("slide-arrow-head", "track", "detail", [[2.82, 1.05], [2.9, 1], [2.82, 0.95]]),
    ];
    s.geometry.front = W(a, "front", sign(front200.map((x: any) => clone(x))));
    s.geometry.plan = [
      ...W(a, "plan", sign(shapes(a, 200, "plan"))),
      poly("slide-arrow", "track", "detail", [[2.05, 0], [2.9, 0]]),
      poly("slide-arrow-head", "track", "detail", [[2.82, 0.04], [2.9, 0], [2.82, -0.04]]),
    ];
  });
  save("sliding-door", doc);
}

{
  const doc = load("steel-column");
  const a = doc.asset;
  a.dimensions.depthM = 0.26;
  for (const r of Object.values<any>(a.representations))
    if (r.status === "authored")
      for (const sh of r.geometry.front) if (sh.id === "splice-mark" && sh.points.some((p: number[]) => Math.abs(p[0]) > 0.13)) sh.points = sh.points.map((p: number[]) => [Math.sign(p[0]) * 0.13, p[1]]);
  setSymbol(doc, "section-plan", (s) => {
    s.title = text("HEB 200 column, conventional sign", "Stütze HEB 200, Zeichen");
    s.description = text(
      "The conventional sign of a steel column in all three views at real size: in plan the rolled H-section cut above the base plate, hatched at 1:50 and solid black at 1:100 and 1:200, with the base plate below as an outline (DIN 1356-1); in the side and front view the shaft with base plate and cap plate.",
      "Das Zeichen einer Stahlstütze in allen drei Ansichten in wahrer Größe: im Grundriss das gewalzte H-Profil über der Fußplatte geschnitten, bei 1:50 schraffiert und bei 1:100 und 1:200 schwarz, mit der Fußplatte darunter als Umriss (DIN 1356-1); in Seiten- und Vorderansicht der Schaft mit Fuß- und Kopfplatte.",
    );
    s.dimensionalNote = text(
      "Real size: HEB 200 of 200 x 200 mm with 15 mm flanges and a 9 mm web, on a 300 x 260 mm base plate, 3.00 m high; the fillets of the rolled section are not drawn. At 1:200 the cut section is 1 mm square.",
      "Echte Größe: HEB 200 mit 200 x 200 mm, 15 mm Flanschen und 9 mm Steg auf einer Fußplatte von 300 x 260 mm, 3,00 m hoch; die Hohlkehlen des Walzprofils sind nicht gezeichnet. Bei 1:200 ist der Schnitt 1 mm groß.",
    );
    s.geometry.elevation = W(a, "elevation", sign(shapes(a, 200, "elevation")));
    s.geometry.front = W(a, "front", sign(shapes(a, 200, "front")));
    s.geometry.plan = [...s.geometry.plan.filter((x: any) => x.id !== "base-plate-outline"), poly("base-plate-outline", "column", "detail", [[-0.15, -0.13], [0.15, -0.13], [0.15, 0.13], [-0.15, 0.13]], true, { region: "base-plate" })];
  });
  save("steel-column", doc);
}

{
  const doc = load("tilt-window");
  const outline = [[-0.15, 0], [0.15, 0], [0.15, 1.25], [-0.15, 1.25]];
  const common = {
    elevation: (arrow: boolean) => [
      poly("window-outline", "sill", "body", outline, true, { region: "frame-timber" }),
      poly("sash-line", "sill", "glazing", [[0, 0.05], [0, 1.25]], false, { region: "pane-glass" }),
      ...(arrow ? [poly("tilt-arrow", "sill", "detail", [[0, 1.15], [-0.12, 1.15]]), poly("tilt-arrow-head", "sill", "detail", [[-0.08, 1.19], [-0.12, 1.15], [-0.08, 1.11]])] : []),
    ],
    front: (lines: boolean) => [
      rect("window-outline", "sill", "body", -0.45, 0, 0.9, 1.25, false, { region: "frame-timber" }),
      rect("sash-outline", "sill", "body", -0.4, 0.05, 0.8, 1.2, false, { region: "frame-timber" }),
      ...(lines ? [poly("tilt-line-a", "sill", "detail", [[-0.4, 1.25], [0, 0.05]]), poly("tilt-line-b", "sill", "detail", [[0.4, 1.25], [0, 0.05]])] : []),
    ],
    plan: (lines: boolean) => [
      rect("jamb-left", "sill", "body", -0.15, 0.4, 0.3, 0.05, true, { region: "frame-timber" }),
      rect("jamb-right", "sill", "body", -0.15, -0.45, 0.3, 0.05, true, { region: "frame-timber" }),
      poly("glass-line", "sill", "glazing", [[0, -0.4], [0, 0.4]], false, { region: "pane-glass" }),
      ...(lines
        ? [poly("frame-interior", "sill", "detail", [[-0.06, -0.4], [-0.06, 0.4]], false, { region: "frame-sill" }), poly("frame-exterior", "sill", "detail", [[0.06, -0.4], [0.06, 0.4]], false, { region: "frame-sill" })]
        : [rect("opening-outline", "sill", "detail", -0.15, -0.45, 0.3, 0.9, false)]),
    ],
  };
  setSymbol(doc, "window-plan", (s) => {
    s.title = text("Window, conventional sign in plan", "Fenster, Zeichen im Grundriss");
    s.description = text(
      "The conventional plan sign of a window in a wall opening, with its side and front view at real size: in plan the wall cut at the reveals and three lines across the opening for the frames and the glass (DIN 1356-1), the room side to the rear (-x); in the side view the section across the frame with the sash line; in the front view the facade outline of opening and sash.",
      "Das Grundrisszeichen eines Fensters in einer Wandöffnung mit seiner Seiten- und Vorderansicht in wahrer Größe: im Grundriss die Wand an den Laibungen geschnitten und drei Linien quer über die Öffnung für die Rahmen und das Glas (DIN 1356-1), die Raumseite hinten (-x); in der Seitenansicht der Schnitt quer zum Rahmen mit der Flügellinie; in der Vorderansicht der Fassadenumriss von Öffnung und Flügel.",
    );
    s.dimensionalNote = text(
      "Real size: opening 0.90 m wide and 1.25 m high in a 0.30 m deep reveal; the three lines are 60 mm apart, which is 0.12 mm at 1:500 and merges into one line.",
      "Echte Größe: Öffnung 0,90 m breit und 1,25 m hoch in einer 0,30 m tiefen Laibung; die drei Linien liegen 60 mm auseinander, bei 1:500 sind das 0,12 mm und sie verschmelzen zu einer Linie.",
    );
    s.geometry = { elevation: common.elevation(false), front: common.front(false), plan: common.plan(true) };
  });
  setSymbol(doc, "tilt-opening-sign", (s) => {
    s.title = text("Tilt sign, conventional sign in the facade view", "Kippzeichen in der Fassadenansicht");
    s.description = text(
      "The facade-view opening sign of a bottom-hinged tilt sash, with its side and top view at real size: in the front view two lines from the head corners of the sash that meet at the hinge side (DIN 1356-1); in the side view the section with an arrow towards the room (-x); in plan the reveals and the glass line.",
      "Das Öffnungszeichen eines unten angeschlagenen Kippflügels in der Fassadenansicht, mit Seiten- und Draufsicht in wahrer Größe: in der Vorderansicht zwei Linien von den oberen Ecken des Flügels, die sich auf der Bandseite treffen (DIN 1356-1); in der Seitenansicht der Schnitt mit einem Pfeil zum Raum (-x); im Grundriss die Laibungen und die Glaslinie.",
    );
    s.dimensionalNote = text(
      "Real size: opening 0.90 x 1.25 m in the facade plane, sash 0.80 x 1.20 m, reveal 0.30 m deep; the lines and the arrow carry only the opening direction and have no dimension.",
      "Echte Größe: Öffnung 0,90 x 1,25 m in der Fassadenebene, Flügel 0,80 x 1,20 m, Laibung 0,30 m tief; die Linien und der Pfeil tragen nur die Öffnungsrichtung und haben keine Abmessung.",
    );
    s.geometry = { elevation: common.elevation(true), front: common.front(true), plan: common.plan(false) };
  });
  save("tilt-window", doc);
}

{
  const doc = load("spiral-stair");
  const a = doc.asset;
  setSymbol(doc, "spiral-stair-plan", (s) => {
    s.title = text("Spiral stair, conventional sign", "Wendeltreppe, Zeichen");
    s.description = text(
      "The conventional sign of a spiral stair in all three views at real size: in plan 15 pie treads around the column, the walking line at 0.6 m radius and the balustrade circle (DIN 1356-1, DIN 18065); in the side and front view the column, base plate, the 15 treads as outlines and the helical handrail.",
      "Das Zeichen einer Wendeltreppe in allen drei Ansichten in wahrer Größe: im Grundriss 15 Kreisringstufen um die Säule, die Lauflinie bei 0,6 m Radius und der Geländerkreis (DIN 1356-1, DIN 18065); in Seiten- und Vorderansicht Säule, Fußplatte, die 15 Stufen als Umriss und der wendelförmige Handlauf.",
    );
    s.dimensionalNote = text(
      "Model metres at real size in every view: the 1.6 m diameter, the 22.5 degree tread angle and the 3.73 m height are true.",
      "Modellmeter in wahrer Größe in jeder Ansicht: der Durchmesser 1,6 m, der Stufenwinkel 22,5 Grad und die Höhe 3,73 m sind echt.",
    );
    s.geometry.elevation = W(a, "elevation", sign(shapes(a, 200, "elevation"), { unfill: /^tread-/ }));
    s.geometry.front = W(a, "front", sign(shapes(a, 200, "front"), { unfill: /^tread-/ }));
  });
  save("spiral-stair", doc);
}

{
  const doc = load("stair-flight");
  const a = doc.asset;
  setSymbol(doc, "flight-plan", (s) => {
    s.title = text("Stair flight, conventional sign", "Treppenlauf, Zeichen");
    s.description = text(
      "The conventional sign of a straight flight in all three views at real size: in plan outline, one line per riser, the stringer as an outline and the walking line with a start dot and an arrow pointing up (DIN 1356-1); in the side view the stringer and the twelve treads as outlines; in the front view the stringer and the treads seen from the foot of the flight.",
      "Das Zeichen eines geraden Laufs in allen drei Ansichten in wahrer Größe: im Grundriss Umriss, eine Linie je Steigung, der Wangenbalken als Umriss und die Lauflinie mit Startpunkt und Pfeil nach oben (DIN 1356-1); in der Seitenansicht der Wangenbalken und die zwölf Stufen als Umriss; in der Vorderansicht Wangenbalken und Stufen vom Lauffuß aus gesehen.",
    );
    s.dimensionalNote = text(
      "Real size: 3.60 m run (12 goings of 0.30 m) by 1.20 m width and 2.90 m high, the stringer reaching 3.855 m; the cut line at 1 m above the floor is not drawn. At 1:200 the sign is 19 x 6 mm.",
      "Echte Größe: 3,60 m Lauflänge (12 Auftritte zu 0,30 m) bei 1,20 m Breite und 2,90 m Höhe, der Wangenbalken reicht bis 3,855 m; die Schnittlinie 1 m über dem Boden ist nicht gezeichnet. Bei 1:200 ist das Zeichen 19 x 6 mm groß.",
    );
    s.geometry.elevation = W(a, "elevation", sign(shapes(a, 200, "elevation"), { unfill: /^tread-board/ }));
    s.geometry.front = W(a, "front", sign(shapes(a, 200, "front"), { unfill: /^tread-/ }));
    const edges = Array.from({ length: 12 }, (_, i) => poly(`tread-edge-${String(i + 1).padStart(2, "0")}`, "stringer", "detail", [[-0.03 + 0.3 * i, -0.6], [-0.03 + 0.3 * i, 0.6]], false, { region: "tread-top" }));
    s.geometry.plan = [
      poly("flight-outline", "stringer", "body", [[-0.03, -0.6], [3.6, -0.6], [3.6, 0.6], [-0.03, 0.6]], true, { region: "tread-top" }),
      poly("stringer-outline", "stringer", "body", [[0.12, -0.08], [3.855, -0.08], [3.855, 0.08], [0.12, 0.08]], true, { region: "stringer-steel" }),
      ...edges.slice(1),
      poly("walking-line", "stringer", "detail", [[0.12, 0], [3.45, 0]]),
      poly("walking-arrow", "stringer", "detail", [[3.2, 0.1], [3.45, 0], [3.2, -0.1]]),
      { id: "walking-start", bone: "stringer", role: "detail", z: 1, filled: true, tone: 0.5, kind: "ellipse", cx: 0.12, cy: 0, rx: 0.04, ry: 0.04 },
    ];
  });
  save("stair-flight", doc);
}
console.log("done");
