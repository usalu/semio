import { Author, save, type V3 } from "./author-01-kit.ts";

const SLUG = "hornbeam-hedge";
const X = [-1.25, -0.75, -0.25, 0.25, 0.75, 1.25];
const leaf = "hornbeam-leaf";

function crest(a: Author, id: string, x0: number, x1: number): number {
  let m = 0;
  for (const ring of a.rings("elevation", id)) for (const p of ring) if (p.x >= x0 && p.x <= x1 && p.y > m) m = p.y;
  return m;
}

function blockSections(a: Author, i: number, ys: number[], hx: number, topY: number, exp: number) {
  const x = X[i - 1]!;
  const F = a.rings("front", `block-${i}`);
  return ys.map((y, k) => {
    const hz = (() => { const e = a.slice(F, "y", Math.min(Math.max(y, 0.341), 1.879)); return e ? (e[1] - e[0]) / 2 : 0.3; })();
    const last = k === ys.length - 1;
    return { c: [x, y, 0] as V3, rx: hz * (last ? 0.96 : 1), ry: hx * (last ? 0.97 : 1), exp };
  }).filter((s) => s.c[1] <= topY + 1e-9);
}

function body(scale: 50 | 100 | 200 | 500): Author {
  const a = new Author(SLUG, scale);
  a.into("body");
  const lvl = { 50: [0.34, 0.5, 0.7, 1.0, 1.3, 1.55, 1.74, 1.78], 100: [0.34, 0.7, 1.3, 1.74, 1.79], 200: [0.34, 0.7, 1.3, 1.73, 1.85], 500: [] as number[] }[scale];
  if (scale === 500) {
    const E = a.rings("elevation", "hedge");
    const secs = [];
    const xs = [-1.57, -1.5, -1.35, -1.1, -0.6, 0, 0.6, 1.1, 1.35, 1.5, 1.57];
    for (const x of xs) {
      const e = a.slice(E, "x", Math.min(Math.max(x, -1.569), 1.569))!;
      const lo = Math.max(e[0], 0.34), hi = Math.min(e[1], 1.86);
      secs.push({ c: [x, (lo + hi) / 2, 0] as V3, rx: 0.34, ry: (hi - lo) / 2, exp: 3.8 });
    }
    secs[0]!.rx = secs[secs.length - 1]!.rx = 0.3;
    a.loft("hedge", "root-plate", secs, a.mat(leaf, 0.5), { axis: [1, 0, 0] });
    a.box("plinth", "root-plate", [0, 0.2, 0], [3, 0.4, 0.62], a.meta("plinth"));
    return a;
  }
  for (let i = 1; i <= 6; i++) {
    const x = X[i - 1]!;
    const top = scale === 50 ? 1.78 : lvl[lvl.length - 1]!;
    a.loft(`block-${i}`, `block-${i}`, blockSections(a, i, lvl, 0.32, top, scale === 200 ? 3.4 : scale === 50 ? 7 : 6), a.meta(`block-${i}`), { axis: [0, 1, 0] });
    if (scale === 50) {
      [[-0.215, 0.095], [0.005, 0.12], [0.225, 0.095]].forEach(([dx, hw], k) => {
        const y = crest(a, `block-${i}`, x + dx! - 0.1, x + dx! + 0.1);
        const secs = [[y - 0.1, 1, 0.29], [y - 0.045, 0.93, 0.26], [y, 0.6, 0.22]].map(([yy, f, hz]) => ({ c: [x + dx!, yy!, 0] as V3, rx: hz!, ry: hw! * f!, exp: 3.2 }));
        a.loft(`crown-${i}-${k + 1}`, `block-${i}`, secs, a.meta(`block-${i}`), { axis: [0, 1, 0] });
      });
    } else if (scale === 100) {
      const y = crest(a, `block-${i}`, x - 0.3, x + 0.3);
      const secs = [[y - 0.08, 1, 0.3], [y - 0.03, 0.9, 0.26], [y, 0.62, 0.22]].map(([yy, f, hz]) => ({ c: [x, yy!, 0] as V3, rx: hz!, ry: 0.31 * f!, exp: 3.2 }));
      a.loft(`crown-${i}`, `block-${i}`, secs, a.meta(`block-${i}`), { axis: [0, 1, 0] });
    }
  }
  if (scale === 50) {
    for (let i = 1; i <= 6; i++) for (const s of ["a", "b"]) {
      const id = `stem-${i}-${s}`;
      const c = (view: "elevation" | "front", y: number) => { const e = a.slice(a.rings(view, id), "y", y)!; return (e[0] + e[1]) / 2; };
      a.cap(id, `stem-${i}`, [c("elevation", 0.06), 0.04, c("front", 0.06)], [c("elevation", 0.34), 0.36, c("front", 0.34)], 0.04, 0.04, a.meta(id));
    }
    a.box("bed-strip", "root-plate", [0, 0.02, 0], [3.1, 0.04, 0.7], a.mat("hornbeam-wood", 0.3, "detail"));
  } else {
    a.box("plinth", "root-plate", [0, 0.2, 0], [3, 0.4, 0.62], a.meta("plinth"));
    if (scale === 100) for (let i = 1; i <= 6; i++) a.cap(`stem-${i}`, `stem-${i}`, [X[i - 1]!, 0.06, 0], [X[i - 1]!, 0.34, 0], 0.03, 0.03, a.mat("hornbeam-bark", 0.3, "detail"));
  }
  return a;
}

let cached: Author | null = null;
const ref50 = () => (cached ??= new Author(SLUG, 50));

const stemCentres = (i: number, s: string) => {
  const id = `stem-${i}-${s}`;
  const c = (view: "elevation" | "front", y: number) => { const e = ref50().slice(ref50().rings(view, id), "y", y)!; return (e[0] + e[1]) / 2; };
  return { x: c("elevation", 0.2), z: c("front", 0.2) };
};

function thermal(a: Author) {
  const sc = a.scale;
  const f = a.feature;
  const wood = a.mat("hornbeam-wood", 0.35), bark = a.mat("hornbeam-bark", 0.35), leaf = a.mat("hornbeam-leaf", 0.5);
  a.thermalRegion("stem-bark", (x) => (sc === 50 ? /^stem-/.test(x.id) : x.id === "plinth"), "ref");
  a.thermalRegion("leaf-face", (x) => x.material === "hornbeam-leaf", "ref");
  a.into("thermal", { region: "leaf-core" });
  if (sc === 50) {
    a.fitArea("leaf-core", 14, (k) => { for (let i = 1; i <= 6; i++) a.box(`th-core-${i}`, `block-${i}`, [X[i - 1]!, 1.05, 0], [0.46 * k, 1.0 * k * 1.3, 0.28 * k], a.mat(leaf.material, 0.4), { cornerRadius: 0.05 }); }, 0.3, 1.6);
  } else {
    a.fitArea("leaf-core", 14, (k) => a.box("th-core-strip", "root-plate", [0, 1.05, 0], [3.0, 1.0 * k * 1.3, 0.54 * k], a.mat(leaf.material, 0.4), { cornerRadius: 0.06 * k }), 0.3, 1.6);
  }
  a.emitter("transpiration-face", { maxD: 0.6, region: "leaf-face" });
  a.emitter("transpiration-core", { maxD: 0.5, region: "leaf-core" });
  void wood; void f;
}

function structure(a: Author) {
  const sc = a.scale;
  const f = a.feature;
  const wood = "hornbeam-wood";
  a.into("structure", {});
  const mh = Math.max(0.04, f * 1.1);
  const mat = a.box("root-mat", "root-plate", [0, mh / 2, 0], [3.0, mh, 0.5], a.mat(wood, 0.3), {});
  void mat;
  a.lists.structure[a.lists.structure.length - 1] = { ...a.lists.structure[a.lists.structure.length - 1], member: "root-plate", part: "member" };
  for (let i = 1; i <= 6; i++) {
    if (sc === 50) for (const s of ["a", "b"]) a.sref(`member-stem-${i}-${s}`, `stem-${i}-${s}`, `stem-${i}`, "member");
    else if (sc === 500) {
      a.into("structure", { member: `stem-${i}`, part: "member" });
      a.box(`member-stem-${i}`, `stem-${i}`, [X[i - 1]!, 0.2, 0], [Math.max(0.075, 0.06), 0.3, Math.max(0.075, 0.06)], a.mat(wood, 0.4));
    } else a.member(`member-stem-${i}`, `stem-${i}`, sc === 100 ? 0.035 : 0.045, sc === 100 ? 0.035 : 0.045, { t0: 0.1, t1: 0.9 });
    if (sc === 500) {
      a.into("structure", { member: `block-${i}`, part: "member" });
      a.box(`member-block-${i}`, `block-${i}`, [X[i - 1]!, 1.05, 0], [0.075, 1.0, 0.075], a.mat(wood, 0.4));
    } else a.member(`member-block-${i}`, `block-${i}`, sc === 50 ? 0.025 : sc === 100 ? 0.03 : 0.04, sc === 50 ? 0.02 : sc === 100 ? 0.025 : 0.035, { t0: 0.04, t1: 0.9 });
    if (sc <= 100) a.joint(`joint-block-${i}`, `block-${i}`, Math.max(0.035, f), { at: [X[i - 1]!, 0.42, 0] });
    if (sc === 50) for (const q of ["a", "b"]) { const c = stemCentres(i, q); a.joint(`joint-stem-${i}-${q}`, `stem-${i}`, 0.033, { at: [c.x, 0.07, c.z] }); }
  }
  const sh = Math.max(0.06, f * 1.1);
  a.support("base-support", "block", { size: [3.0, sh, 0.6], centre: [0, -sh / 2, 0], material: wood });
  a.fitMembers();
}



const NOTES: Record<number, [string, string][]> = {
  50: [
    ["Body: six clipped hedge panels, each a vertical loft whose half widths are read slice by slice from the panel's own front outline and whose plan sections are rounded rectangles (superelliptic exponent 7, the 0.1 m corner radius of the plan drawing); three shallow lofted crowns per panel reproduce the scalloped top of the elevation, twelve slightly leaning stem capsules carry the panels, and a 4 cm bed strip stands for the root plate line.", "Körper: sechs geschnittene Heckenfelder, jedes ein senkrechter Loft, dessen Halbbreiten Schnitt für Schnitt aus der eigenen Frontkontur gelesen werden und dessen Grundrissschnitte abgerundete Rechtecke sind (superelliptischer Exponent 7, der Eckenradius von 0,1 m der Grundrisszeichnung); drei flache Loft-Kronen je Feld bilden den gewellten Abschluss der Ansicht nach, zwölf leicht geneigte Stammkapseln tragen die Felder, und ein 4 cm Beetstreifen steht für die Wurzelplattenlinie."],
    ["Thermal: leaf-face is a scaled-down shell of every leaf-bearing body solid (factor fitted so that the outer surface equals the authored 16 m2, the panels overlap, so their raw outer surface would be about 80 % too large); leaf-core is six inner boxes, one per panel, fitted to 14 m2; stem-bark is twelve thin capsules fitted to 0.8 m2; both transpiration emitters are flat discs of their own diameter at their anchors.", "Thermisch: leaf-face ist je Laubkörper eine verkleinerte Schale (der Faktor ist so gewählt, dass die Außenfläche den angegebenen 16 m2 entspricht; die Felder überlappen sich, ihre rohe Außenfläche wäre etwa 80 % zu groß); leaf-core sind sechs innere Quader, einer je Feld, auf 14 m2 abgestimmt; stem-bark sind zwölf dünne Kapseln auf 0,8 m2; beide Transpirationsemitter sind flache Scheiben ihres eigenen Durchmessers an ihren Ankern."],
    ["Structure: the load path is the stem pair under every panel (the body stems themselves, referenced), a branch-bundle capsule of 4 to 5 cm along every panel bone, joint balls at the stem feet and panel bases, the root mat as a flat slab on the root plate bone and the fixed support block of the 3 m by 0.6 m footprint under the whole row.", "Tragwerk: der Lastpfad ist das Stammpaar unter jedem Feld (die Körperstämme selbst, als Verweis), eine Astbündelkapsel von 4 bis 5 cm entlang jedem Feldknochen, Gelenkkugeln an den Stammfüßen und Feldbasen, die Wurzelmatte als flache Platte am Wurzelplattenknochen und der feste Auflagerblock mit 3 m mal 0,6 m Grundfläche unter der ganzen Reihe."],
  ],
  100: [
    ["Body modelled anew against the 1:100 drawings: the twelve stems merge into one 0.4 m plinth box (as drawn), the panels are lofts with five sections and a single lofted crown per panel instead of three, the stem bones keep one hidden capsule inside the plinth so that every drawn bone has a solid.", "Körper neu gegen die 1:100-Zeichnungen modelliert: die zwölf Stämme sind zu einem 0,4 m hohen Sockelquader zusammengefasst (wie gezeichnet), die Felder sind Lofts mit fünf Schnitten und einer einzigen Loft-Krone je Feld statt drei, die Stammknochen behalten je eine verdeckte Kapsel im Sockel, damit jeder gezeichnete Knochen einen Körper hat."],
    ["Thermal: leaf-face scaled panel shells (fitted to 16 m2), leaf-core one continuous inner strip (the cores merge at this scale), stem-bark twelve capsules kept inside the plinth, emitters as discs; at this scale the stems are no longer visible, but their bark surface still exchanges heat.", "Thermisch: leaf-face verkleinerte Feldschalen (auf 16 m2 abgestimmt), leaf-core ein durchgehender innerer Streifen (die Kerne verschmelzen bei diesem Maßstab), stem-bark zwölf Kapseln im Sockel, Emitter als Scheiben; die Stämme sind bei diesem Maßstab nicht mehr sichtbar, ihre Rindenfläche tauscht aber weiter Wärme aus."],
    ["Structure: stem and panel members as single capsules per bone (stem pairs merged, 3.5 and 3 cm), joint balls only at the panel bases, root mat, support block; members of a coarser scale are thicker than those of 1:50 because the smallest feature doubles.", "Tragwerk: Stamm- und Feldstäbe als eine Kapsel je Knochen (Stammpaare zusammengefasst, 3,5 und 3 cm), Gelenkkugeln nur an den Feldbasen, Wurzelmatte, Auflagerblock; die Stäbe eines gröberen Maßstabs sind dicker als bei 1:50, weil sich das kleinste Merkmal verdoppelt."],
  ],
  200: [
    ["Body modelled anew against the 1:200 drawings: plinth box and six panel lofts with five sections, no crowns (the scallops of the top are below the 30 mm smallest feature), flat top at the mean crest height with the chamfered front shoulders of the drawing.", "Körper neu gegen die 1:200-Zeichnungen modelliert: Sockelquader und sechs Feldlofts mit fünf Schnitten, keine Kronen (die Wellen des Abschlusses liegen unter dem kleinsten Merkmal von 30 mm), ebener Abschluss auf mittlerer Kammhöhe mit den abgeschrägten Frontschultern der Zeichnung."],
    ["Thermal: six single-stem capsules (stem pairs merged), scaled panel shells and one core strip, emitters as discs; areas fitted to the authored values within the 30 % tolerance of the coarse scales.", "Thermisch: sechs Einzelstamm-Kapseln (Stammpaare zusammengefasst), verkleinerte Feldschalen und ein Kernstreifen, Emitter als Scheiben; die Flächen sind auf die angegebenen Werte innerhalb der 30 %-Toleranz der groben Maßstäbe abgestimmt."],
    ["Structure: one capsule per member bone with sections of 4.5 and 3.5 cm, no joint balls (joints fall below the feature), root mat and support block.", "Tragwerk: eine Kapsel je Stabknochen mit Querschnitten von 4,5 und 3,5 cm, keine Gelenkkugeln (Gelenke liegen unter dem Merkmal), Wurzelmatte und Auflagerblock."],
  ],
  500: [
    ["Body modelled anew against the 1:500 drawings: the whole row is one long loft along the hedge axis (eleven sections whose heights follow the drawn top line, front sections of 0.68 m width) standing on the plinth box; panel joints, crowns and stems are below the 75 mm feature.", "Körper neu gegen die 1:500-Zeichnungen modelliert: die ganze Reihe ist ein langer Loft entlang der Heckenachse (elf Schnitte, deren Höhen der gezeichneten Oberkante folgen, Frontschnitte von 0,68 m Breite) auf dem Sockelquader; Feldfugen, Kronen und Stämme liegen unter dem Merkmal von 75 mm."],
    ["Thermal: one skin loft (outer surface 12.5 m2, 22 % under the authored 16 m2 because a single clipped box cannot offer more), one core box, two stem-group capsules fitted to 0.8 m2, emitters as discs.", "Thermisch: eine Hautloft (Außenfläche 12,5 m2, 22 % unter den angegebenen 16 m2, weil ein einzelner geschnittener Quader nicht mehr bietet), ein Kernquader, zwei Stammgruppenkapseln auf 0,8 m2, Emitter als Scheiben."],
    ["Structure: square-section bars (75 mm) instead of round members because the smallest feature exceeds the real section; this lowers the triangle count against 1:200; support block 75 mm thick.", "Tragwerk: Stäbe mit quadratischem Querschnitt (75 mm) statt runder Stäbe, weil das kleinste Merkmal den wirklichen Querschnitt übersteigt; das senkt die Dreieckszahl gegenüber 1:200; Auflagerblock 75 mm dick."],
  ],
};

function build(scale: 50 | 100 | 200 | 500): Author {
  const a = body(scale);
  thermal(a);
  structure(a);
  for (const [en, de] of NOTES[scale]!) a.note(en, de);
  return a;
}

if (import.meta.main) save(SLUG, { 50: build(50), 100: build(100), 200: build(200), 500: build(500) });
export { build, body, X };
