import { load, save, clone, text } from "./lib.ts";

{
  const doc = load("external-venetian-blind");
  const a = doc.asset;
  const src = a.representations["200"].geometry;
  const keep = (id: string) => !/^slat-\d+$/.test(id) || Number(id.slice(5)) % 4 === 0;
  const out: any = {};
  for (const view of ["elevation", "front", "plan"]) {
    out[view] = clone(src[view])
      .filter((s: any) => keep(s.id) && s.role !== "texture")
      .map((s: any) => {
        const o = { ...s };
        if (o.role === "detail") o.role = "body";
        if (/^guide-/.test(o.id) && o.kind === "rect") {
          const edge = o.x > 0 ? o.x + o.w : o.x;
          o.w = 0.06;
          o.x = o.x > 0 ? edge - 0.06 : edge;
        }
        if (o.penMm !== undefined) o.penMm = 0.18;
        return o;
      });
  }
  a.representations["500"] = {
    status: "authored",
    denominator: 500,
    validRange: { minDenominator: 350, maxDenominator: 1000 },
    pens: { body: 0.18, detail: null, texture: null, glazing: 0.1, ground: 0.18 },
    hatches: [],
    geometry: out,
    decisions: {
      silhouette: text(
        "A 5.5 mm tall mark: the cut facade bands, the headrail box as a block and five lamella lines (every fourth lamella) hanging from it with the bottom rail. The side view is the vertical section; the front view is the blind seen from outside with the lamellae as extruded lines across the 1.2 m width; the plan view is the wall head and the box, with the lamellae hidden under it.",
        "Ein 5,5 mm hohes Zeichen: die geschnittenen Fassadenbänder, die Kastenblende als Block und fünf Lamellenlinien (jede vierte Lamelle) daran mit der Unterschiene. Die Seitenansicht ist der Vertikalschnitt; die Vorderansicht ist die Jalousie von außen mit den Lamellen als extrudierte Linien über die 1,2 m Breite; die Draufsicht ist der Wandkopf mit dem Kasten, die Lamellen bleiben darunter verdeckt.",
      ),
      retained: text(
        "Headrail box, bottom rail, the wall bands and lamellae 0, 4, 8, 12 and 16, each bound to its own bone, so lift and tilt move them in every view.",
        "Kastenblende, Unterschiene, die Wandbänder und die Lamellen 0, 4, 8, 12 und 16, je an ihrer eigenen Bone, sodass Hub und Kippen sie in jeder Ansicht bewegen.",
      ),
      merged: text(
        "The 17 lamellae of 0.08 m pitch (0.16 mm at 1:500) are thinned to every fourth, a 0.32 m pitch of 0.64 mm: a simplified lamella hatch whose line count is a convention, not the real number.",
        "Die 17 Lamellen mit 0,08 m Teilung (0,16 mm bei 1:500) sind auf jede vierte ausgedünnt, 0,32 m Teilung und 0,64 mm: eine vereinfachte Lamellenschraffur, deren Linienzahl eine Konvention und nicht die wirkliche Zahl ist.",
      ),
      omitted: text(
        "The glazing line, the two guide bars and the lamellae in between: the lamella thickness of 12 mm is 0.024 mm and cannot be printed.",
        "Die Verglasungslinie, die zwei Führungsschienen und die Lamellen dazwischen: die Lamellendicke von 12 mm sind 0,024 mm und nicht druckbar.",
      ),
      thinElements: text(
        "Every lamella is a body line of 0.13 mm; no filled strip thinner than 0.2 mm is drawn.",
        "Jede Lamelle ist eine Körperlinie von 0,13 mm; kein gefüllter Streifen unter 0,2 mm wird gezeichnet.",
      ),
      hatching: text("None; the lamellae are lines.", "Keine; die Lamellen sind Linien."),
      styles: text(
        "Wall bands and box are filled blocks; lamellae and bottom rail are body lines.",
        "Wandbänder und Kasten sind gefüllte Blöcke; Lamellen und Unterschiene sind Körperlinien.",
      ),
      overlays: text(
        "The lamella angle reads from the line slope in the side view and from the line spacing in the front view; the tilt of the five lines carries the shading.",
        "Der Lamellenwinkel liest sich an der Linienneigung in der Seitenansicht und am Linienabstand in der Vorderansicht; die Neigung der fünf Linien trägt die Verschattung.",
      ),
    },
  };
  save("external-venetian-blind", doc);
}

{
  const doc = load("hot-water-cylinder");
  const a = doc.asset;
  const src = a.representations["200"].geometry;
  const out: any = {};
  for (const view of ["elevation", "front", "plan"]) {
    out[view] = clone(src[view]).map((s: any) => {
      const o = { ...s };
      if (o.id === "feet") o.h = 0.12;
      if (["side-l", "side-r", "hot-out"].includes(o.id) && o.kind === "rect") {
        const S = 0.11;
        const cx = o.x + o.w / 2;
        const cy = o.y + o.h / 2;
        o.x = o.id === "side-l" && view !== "front" ? o.x : o.id === "side-r" && view !== "front" ? o.x + o.w - S : cx - S / 2;
        o.y = o.id === "hot-out" && view !== "plan" ? o.y + o.h - S : cy - S / 2;
        o.w = S;
        o.h = S;
      }
      return o;
    });
  }
  a.representations["500"] = {
    status: "authored",
    denominator: 500,
    validRange: { minDenominator: 350, maxDenominator: 1000 },
    pens: { body: 0.18, detail: null, texture: null, glazing: null, ground: 0.25 },
    hatches: [],
    geometry: out,
    decisions: {
      silhouette: text(
        "A 1.2 mm by 3.0 mm mark: a tall block of two stacked jacket halves on a foot bar with a dome. Front view: the same stack across 0.6 m; top view: the 0.6 m dome disc with the three stubs.",
        "Ein Zeichen von 1,2 mm mal 3,0 mm: ein hoher Block aus zwei übereinanderliegenden Mantelhälften auf einem Fußbalken mit Wölbung. Vorderansicht: derselbe Stapel über 0,6 m; Draufsicht: die 0,6 m große Wölbungsscheibe mit den drei Stutzen.",
      ),
      retained: text(
        "The foot bar, the two jacket halves (two stratification regions each), the dome and the three stubs, each bound to its bone.",
        "Der Fußbalken, die zwei Mantelhälften (je zwei Schichtungsregionen), die Wölbung und die drei Stutzen, je an ihrer Bone.",
      ),
      merged: text(
        "The four stratification quarters form two halves of 0.66 m; the stubs are enlarged to 0.11 m so that each is 0.22 mm, a conventional enlargement overlapping the jacket and keeping the overall width of 0.66 m.",
        "Die vier Schichtungsviertel bilden zwei Hälften von 0,66 m; die Stutzen sind auf 0,11 m vergrößert, je 0,22 mm, eine konventionelle Vergrößerung, die den Mantel überlappt und die Gesamtbreite von 0,66 m hält.",
      ),
      omitted: text(
        "Dial, needle, label, seams, the base ring and the remaining side stubs of 0.03 m (0.06 mm).",
        "Zifferblatt, Nadel, Beschriftung, Nähte, der Fußring und die übrigen seitlichen Stutzen von 0,03 m (0,06 mm).",
      ),
      thinElements: text(
        "The foot bar is enlarged to 0.1 m (0.2 mm); no other element is thinner than 0.2 mm.",
        "Der Fußbalken ist auf 0,1 m (0,2 mm) vergrößert; kein anderes Element ist dünner als 0,2 mm.",
      ),
      hatching: text("None.", "Keine."),
      styles: text("Filled blocks only, tone by region.", "Nur gefüllte Blöcke, Ton nach Region."),
      overlays: text(
        "The two halves carry the charge level in thermal views; the quarters are two regions each.",
        "Die zwei Hälften tragen den Ladezustand in den Thermalansichten; die Viertel sind je zwei Regionen.",
      ),
    },
  };
  save("hot-water-cylinder", doc);
}
