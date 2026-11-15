import { Author, save, type V3 } from "./author-01-kit.ts";

const SLUG = "ivy-on-trellis";
const TIMBER = "timber-c24", STEEL = "steel-s235", LEAF = "ivy-leaf", STEM = "ivy-stem";

const box2 = (a: Author, view: "elevation" | "front" | "plan", ids: string | string[]) => a.box2(view, ids);

function body(scale: 50 | 100 | 200 | 500): Author {
  const a = new Author(SLUG, scale);
  a.into("body");
  const f = a.feature;
  if (scale === 500) {
    const e = box2(a, "elevation", "screen"), p = box2(a, "plan", "screen");
    a.box("screen", "panel", [(e.minX + e.maxX) / 2, (e.minY + e.maxY) / 2, (p.minY + p.maxY) / 2], [e.maxX - e.minX - 0.2, e.maxY - e.minY, Math.max(p.maxY - p.minY, f * 1.2)], a.mat(LEAF, 0.5));
    for (const x of [-0.9, 0.9]) {
      const sd = x < 0 ? "l" : "r";
      a.box(`post-${sd}`, `post-${sd}`, [x, 1.05, 0], [0.1, 2.1, Math.max(p.maxY - p.minY, f * 1.2)], a.mat(TIMBER, 0.5));
      a.box(`shoe-${sd}`, `post-${sd}`, [x, 0.14, 0], [0.12, 0.28, Math.max(p.maxY - p.minY, f * 1.2)], a.mat(STEEL, 0.3));
    }
    for (const s of ["l", "r"]) {
      const id = `crown-${s}`;
      const ee = box2(a, "elevation", id), ff = box2(a, "front", id);
      const c: V3 = [(ee.minX + ee.maxX) / 2, (ee.minY + ee.maxY) / 2, (ff.minX + ff.maxX) / 2];
      a.ell(id, id, c, [(ee.maxX - ee.minX) / 2, (ee.maxY - ee.minY) / 2, Math.max((ff.maxX - ff.minX) / 2, f * 0.6)], a.mat(LEAF, 0.5), { tiltDeg: s === "l" ? 12 : -10 });
    }
    return a;
  }
  for (const s of ["l", "r"]) {
    const p = box2(a, "elevation", `post-${s}`), pz = box2(a, "plan", `post-${s}`), sh = box2(a, "elevation", `shoe-${s}`), sz = box2(a, "plan", `shoe-${s}`);
    a.box(`post-${s}`, `post-${s}`, [(p.minX + p.maxX) / 2, (p.minY + p.maxY) / 2, (pz.minY + pz.maxY) / 2], [p.maxX - p.minX, p.maxY - p.minY, pz.maxY - pz.minY], a.meta(`post-${s}`), { cornerRadius: scale === 50 ? 0.006 : 0 });
    a.box(`shoe-${s}`, `post-${s}`, [(sh.minX + sh.maxX) / 2, (sh.minY + sh.maxY) / 2, (sz.minY + sz.maxY) / 2], [sh.maxX - sh.minX, sh.maxY - sh.minY, sz.maxY - sz.minY], a.meta(`shoe-${s}`), { cornerRadius: scale === 50 ? 0.01 : 0 });
  }
  const rt = box2(a, "plan", "rail-top");
  for (const r of ["rail-top", "rail-bottom"]) {
    const e = box2(a, "elevation", r);
    a.box(r, "panel", [(e.minX + e.maxX) / 2, (e.minY + e.maxY) / 2, (rt.minY + rt.maxY) / 2], [e.maxX - e.minX, e.maxY - e.minY, rt.maxY - rt.minY], a.meta(r));
  }
  if (scale === 50) {
    for (const r of ["batten-h-0", "batten-h-1"]) {
      const e = box2(a, "elevation", r);
      a.box(r, "panel", [(e.minX + e.maxX) / 2, (e.minY + e.maxY) / 2, 0], [e.maxX - e.minX, e.maxY - e.minY, 0.05], a.meta(r));
    }
    const e = box2(a, "elevation", "batten-v-0");
    a.box("batten-v", "panel", [(e.minX + e.maxX) / 2, (e.minY + e.maxY) / 2, 0], [e.maxX - e.minX, e.maxY - e.minY, 0.05], a.meta("batten-v-0"), { extra: { repeat: { count: 8, translation: [0.2, 0, 0] } } });
    for (let n = 1; n <= 4; n++) {
      const o = a.origin(`stem-${n}`), top = a.tip(`stem-${n}`);
      const mid: V3 = [o[0] + 0.03 * (n % 2 ? 1 : -1), (o[1] + top[1]) / 2, 0.03 * (n % 2 ? -1 : 1)];
      a.cap(`stem-${n}-lo`, `stem-${n}`, [o[0], o[1] + 0.05, 0.02], mid, 0.014, 0.012, a.meta(`mat-${n}-lo-0`, { material: STEM, role: "body", tone: 0.3 }));
      a.cap(`stem-${n}-hi`, `stem-${n}`, mid, [top[0] + 0.02, top[1] - 0.1, 0.02], 0.012, 0.009, a.meta(`mat-${n}-lo-0`, { material: STEM, role: "body", tone: 0.3 }));
    }
  }
  if (scale === 100) {
    const seen = new Set<string>();
    for (const sh of a.st.elevation) {
      const id = sh.shape.id as string;
      if (!id.startsWith("batten-") || seen.has(id)) continue;
      seen.add(id);
      const pts = sh.rings.flat();
      const xs = pts.map((p) => p.x), ys = pts.map((p) => p.y);
      const w = Math.max(Math.max(...xs) - Math.min(...xs), 0.05), h = Math.max(Math.max(...ys) - Math.min(...ys), 0.05);
      a.box(id, "panel", [(Math.max(...xs) + Math.min(...xs)) / 2, (Math.max(...ys) + Math.min(...ys)) / 2, 0], [w, h, 0.05], a.mat(TIMBER, 0.5, "detail"));
    }
  }
  for (let n = 1; n <= 4; n++) for (const h of ["lo", "hi"]) {
    const bone = `fol-${n}-${h}`;
    const E = a.shapesOf("elevation", bone), F = a.shapesOf("front", bone);
    a.loftFromViews(`mat-${n}-${h}`, bone, { elev: E, front: F, n: { 50: 9, 100: 7, 200: 5 }[scale], exp: 2.6, pinch: 0.45, m: a.mat(LEAF, 0.5) });
  }
  return a;
}



function thermal(a: Author) {
  const sc = a.scale, f = a.feature;
  if (sc === 500) {
    a.thermalRegion("leaves-upper", (s) => s.bone.startsWith("crown"), "ref");
    a.thermalRegion("leaves-lower", (s) => s.id === "screen", "ref");
    a.thermalRegion("trellis-timber", (s) => s.material === TIMBER, "ref");
    a.thermalRegion("post-shoes", (s) => s.material === STEEL, "ref");
  } else {
    a.thermalRegion("leaves-upper", (s) => s.material === LEAF && /fol-\d-hi/.test(s.bone), "ref");
    a.thermalRegion("leaves-lower", (s) => s.material === LEAF && /fol-\d-lo/.test(s.bone), "ref");
    a.thermalRegion("trellis-timber", (s) => s.material === TIMBER, "ref");
    a.thermalRegion("post-shoes", (s) => s.material === STEEL, "ref");
  }
  a.into("thermal", { region: "stems" });
  if (sc <= 100) a.fitArea("stems", 0.45, (k) => { for (const n of [1, 2, 3, 4]) { const o = a.origin(`stem-${n}`), t = a.tip(`stem-${n}`); a.cap(`th-stem-${n}`, `stem-${n}`, [o[0] + 0.02, o[1] + (sc === 50 ? 0.2 : 0.3), 0.0], [t[0] + 0.02, t[1] - (sc === 50 ? 0.2 : 0.3), 0.0], 0.02 * k, 0.015 * k, a.mat(STEM, 0.3)); } }, 0.2, 3);
  else a.fitArea("stems", 0.45, (k) => { for (const n of [1, 2, 3, 4]) { const o = a.origin(`stem-${n}`); a.cap(`th-stem-${n}`, `stem-${n}`, [o[0] + 0.02, o[1] + 0.4, 0.0], [o[0] + 0.02, o[1] + 0.4 + 1.2 * k, 0.0], f * 0.6, f * 0.6, a.mat(STEM, 0.3)); } }, 0.05, 1);
  a.emitter("transpiration-upper", { maxD: 0.3, region: "leaves-upper", flat: "z" });
  a.emitter("transpiration-lower", { maxD: 0.3, region: "leaves-lower", flat: "z" });
}

const supportsOf = (a: Author) => { for (const s of ["l", "r"]) a.support(`base-support-${s}`, "block", { size: [0.11, Math.max(0.06, a.feature * 1.1), 0.11], centre: [s === "l" ? -0.9 : 0.9, -Math.max(0.06, a.feature * 1.1) / 2, 0], material: STEEL }); };

function structure(a: Author) {
  const sc = a.scale, f = a.feature;
  const r = Math.max(0.02, f * 0.6);
  a.memberWorld("member-root-plate", "root-plate", [-0.85, 0.385, 0], [0.85, 0.385, 0], Math.max(r, f * 0.55));
  for (const s of ["l", "r"]) { const o = a.origin(`crown-${s}`); a.memberWorld(`member-crown-${s}`, `crown-${s}`, [o[0] - 0.05, 1.965, 0], [o[0] + 0.05, 1.965, 0], Math.max(0.012, f * 0.55)); }
  if (sc === 50) for (let n = 1; n <= 4; n++) for (const h of ["lo", "hi"]) a.sref(`member-stem-${n}-${h}`, `stem-${n}-${h}`, `stem-${n}`, "member");
  if (sc >= 100) for (let n = 1; n <= 4; n++) { const o = a.origin(`stem-${n}`); a.memberWorld(`member-stem-${n}`, `stem-${n}`, [o[0], 1.0, 0], [o[0], 1.75, 0], Math.max(0.012 * (sc / 50) ** 0.5, f * 0.55)); }
  a.autoMembers((b) => (b.section ? [0.045, 0.045] : /^fol|crown/.test(b.id) ? [0.012 * (sc / 50) ** 0.5, 0.008 * (sc / 50) ** 0.5] : /stem/.test(b.id) ? [0.014 * (sc / 50) ** 0.5, 0.01 * (sc / 50) ** 0.5] : [0.03, 0.03]), { t0: 0.1, t1: 0.9 });
  if (sc === 50) a.autoJoints((b) => 0.02 * (sc / 50) ** 0.5, { only: (b) => /^fol/.test(b.id) });
  supportsOf(a);
  a.fitMembers(20);
}

const NOTES: Record<number, [string, string][]> = {
  50: [["Body: posts, steel shoes, rails and the horizontal battens are timber and steel boxes of the drawn sizes (the trellis is genuinely prismatic), the eight vertical battens are one box with a 0.2 m repeat; the four climbing stems are two-piece capsules with a slight sideways drift; each of the eight leaf mats is a loft whose half widths follow its own elevation and front outlines, so the ivy reads as a cushion in front of the trellis rather than a box.", "Körper: Pfosten, Stahlschuhe, Riegel und waagrechte Latten sind Holz- und Stahlquader in den gezeichneten Maßen (das Rankgitter ist wirklich prismatisch), die acht senkrechten Latten sind ein Quader mit 0,2 m Wiederholung; die vier Kletterstämme sind zweiteilige Kapseln mit leichtem Seitenversatz; jede der acht Blattmatten ist ein Loft, dessen Halbbreiten ihren eigenen Aufriss- und Frontkonturen folgen, sodass der Efeu als Polster vor dem Gitter erscheint statt als Quader."],
   ["Thermal: leaf mats as scaled shells fitted to the authored areas, the hidden stems as thin capsules inside the mats, trellis timber and steel shoes as references to the body solids, emitters as flat discs.", "Thermisch: Blattmatten als verkleinerte Schalen auf die angegebenen Flächen abgestimmt, die verdeckten Triebe als dünne Kapseln in den Matten, Rankgitterholz und Stahlschuhe als Verweise auf die Körper, Emitter als flache Scheiben."],
   ["Structure: every bone of the structural view has a member capsule inside the body (posts 4.5 cm, panel, stems and mat axes 1 to 1.4 cm), joint balls at the revolute mat bones, support blocks under both post feet.", "Tragwerk: jeder Knochen der Tragwerksansicht hat eine Stabkapsel im Körper (Pfosten 4,5 cm, Feld, Triebe und Mattenachsen 1 bis 1,4 cm), Gelenkkugeln an den drehbaren Mattenknochen, Auflagerblöcke unter beiden Pfostenfüßen."]],
  100: [["Body modelled anew against the 1:100 drawings: the battens are now individual outline boxes, the stems disappear (they are hidden behind the mats), the mats are lofts with seven sections.", "Körper neu gegen die 1:100-Zeichnungen modelliert: die Latten sind nun einzelne Konturquader, die Triebe entfallen (sie liegen hinter den Matten), die Matten sind Lofts mit sieben Schnitten."],
   ["Thermal: as 1:50 with coarser shells; stems stay as hidden capsules.", "Thermisch: wie bei 1:50 mit gröberen Schalen; die Triebe bleiben als verdeckte Kapseln."],
   ["Structure: member capsules with sections grown by the square root of the scale ratio, joint balls kept, supports.", "Tragwerk: Stabkapseln mit Querschnitten, vergrößert um die Wurzel des Maßstabsverhältnisses, Gelenkkugeln beibehalten, Auflager."]],
  200: [["Body modelled anew against the 1:200 drawings: posts, shoes, two rails and eight mat lofts with five sections; battens and stems are below the 30 mm feature or hidden.", "Körper neu gegen die 1:200-Zeichnungen modelliert: Pfosten, Schuhe, zwei Riegel und acht Mattenlofts mit fünf Schnitten; Latten und Triebe liegen unter dem Merkmal von 30 mm oder sind verdeckt."],
   ["Thermal: scaled mat shells, hidden stem capsules, references for timber and shoes.", "Thermisch: verkleinerte Mattenschalen, verdeckte Triebkapseln, Verweise für Holz und Schuhe."],
   ["Structure: member capsules and supports; joints are below the feature and omitted.", "Tragwerk: Stabkapseln und Auflager; Gelenke liegen unter dem Merkmal und entfallen."]],
  500: [["Body modelled anew against the 1:500 drawings: the posts, rails and mats merge into one screen box (as drawn) and two crown ellipsoids at the stem tops.", "Körper neu gegen die 1:500-Zeichnungen modelliert: Pfosten, Riegel und Matten verschmelzen zu einem Schirmquader (wie gezeichnet) und zwei Kronenellipsoiden an den Triebspitzen."],
   ["Thermal: crown shells, a doubled lower sheet for the lower leaf area, hidden stem capsules, timber and shoes referenced from the screen where possible.", "Thermisch: Kronenschalen, ein doppeltes Bahnblech für die untere Blattfläche, verdeckte Triebkapseln."],
   ["Structure: member capsules at the minimum feature of 75 mm, supports.", "Tragwerk: Stabkapseln im Mindestmerkmal von 75 mm, Auflager."]],
};

function build(scale: 50 | 100 | 200 | 500): Author {
  const a = body(scale);
  thermal(a);
  structure(a);
  for (const [en, de] of NOTES[scale]!) a.note(en, de);
  return a;
}
export { build };

export { body };
void TIMBER; void STEEL; void LEAF; void STEM;
