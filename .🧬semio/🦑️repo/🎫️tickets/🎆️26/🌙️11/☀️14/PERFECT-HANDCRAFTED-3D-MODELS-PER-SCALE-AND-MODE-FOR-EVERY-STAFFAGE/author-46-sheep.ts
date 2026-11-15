/** 🐑 Author 46: sheep body, thermal extras, structure and decisions per scale, built bone by bone from the scale's own drawings (author-25-quad.ts builder, hand-set cloven hooves, eyes, flat ears, woolly tail). */
import { openCtx, saveLists, flag, has, drawn, ellipsoidOf, capsuleOf, boxOf, type Json } from "./kit-lib.ts";
import { autoParts, buildStructure, type Cfg } from "./author-25-quad.ts";
import { bboxLocal, part } from "./author-25-lib.ts";
import { centreOnLofts, clampJets, fixPlume, shiftUndrawn } from "./author-46-lib.ts";
import { decisions, type T } from "./author-25-text.ts";

const args = process.argv.slice(2);
const scale = Number(flag(args, "--scale"));
const c = openCtx("animals/sheep", scale);
const SIDE: Record<string, string> = { f: "far", n: "near" };
const BASE: Record<string, string> = { thigh: "thigh", shank: "gaskin", metatarsus: "hind-cannon", hoof: "hoof", scapula: "shoulder", humerus: "humerus", forearm: "forearm", metacarpus: "fore-cannon" };
const name = (bone: string): string | undefined => {
  const m = /^(thigh|shank|metatarsus|hoof|scapula|humerus|forearm|metacarpus)-([fh])([fn])$/.exec(bone);
  if (m !== null) return `${SIDE[m[3]!]}-${m[1] === "hoof" ? (m[2] === "h" ? "hind-hoof" : "fore-hoof") : BASE[m[1]!]}`;
  const ear = /^ear-([fn])$/.exec(bone);
  if (ear !== null) return `${SIDE[ear[1]!]}-ear`;
  return ({ lumbar: "hindbody", thorax: "chest", neck: "neck", head: "head", jaw: "jaw", "tail-1": "tail-root", "tail-2": "tail-tip" } as Record<string, string>)[bone];
};
const cfg: Cfg = {
  material: "fleece",
  name,
  e: (b) => (/^(lumbar|thorax)$/.test(b) ? 2.2 : /^(thigh|scapula|humerus)/.test(b) ? 2.7 : /^(tail|ear)/.test(b) ? 2.2 : /^(neck|head|jaw)$/.test(b) ? 2.2 : /^hoof/.test(b) ? 2.2 : 2.4),
  tone: (b) => (/^ear-n$/.test(b) ? 0.6 : /^hoof/.test(b) ? 0.55 : /-[fh]f$|^ear-f$/.test(b) ? 0.55 : 0.72),
  elevOnly: { head: ["head"] },
  elevExclude: /^(fold-|mouth-line)/,
  latBone: {},
  twin: { "ear-f": "ear-n" },
  latTaper: { "thigh-hf": { taper: [0.75, 0.95], inward: [0.8, 1] }, "thigh-hn": { taper: [0.75, 0.95], inward: [0.8, 1] } },
  bevel: { lumbar: 0.92, thorax: 0.92, neck: 0.92 },
  tweak: (b, _sc, p) => (b === "tail-1" ? { ...p, lat: { half: Math.max(0.032, 0.51 * c.feature), taper: 0.7 } } : b === "tail-2" ? { ...p, lat: { half: Math.max(0.021, 0.51 * c.feature), taper: 0.7 } } : p),
  ears: { "ear-f": { roll: -68, thick: Math.max(0.0042, 0.56 * c.feature), scale: 0.5 }, "ear-n": { roll: 68, thick: Math.max(0.0042, 0.56 * c.feature), scale: 0.5 } },
};
const parts = autoParts(c, cfg);
const body: Json[] = [];
for (const p of parts) {
  if (/^hoof-/.test(p.bone) && scale <= 100 && "view" in p.lat) {
    const hw = scale === 50 ? 0.0185 : 0.0;
    if (hw > 0) {
      for (const [k, sgn] of [["a", 1], ["b", -1]] as const) body.push(part(c, { ...p, id: `${p.id}-toe-${k}`, lat: { ...p.lat, scale: 0.48, offset: sgn * hw } }));
      continue;
    }
  }
  body.push(part(c, p));
}
const has2d = (id: string): boolean => drawn(c, "elevation").some((x) => x.id === id);
const eb = has2d("eye") ? bboxLocal(c, "head", "eye") : null;
if (eb !== null) for (const w of [0.042, -0.042]) body.push(ellipsoidOf({ id: w > 0 ? "far-eye" : "near-eye", bone: "head", material: "bare-skin", tone: 0.04, role: "detail" }, [(eb[0] + eb[1]) / 2, (eb[2] + eb[3]) / 2, w], [0.0085, 0.0085, 0.0085]));
for (const [bone, id, len, w] of [["thigh-hn", "near-thigh", 0.21, -0.1], ["humerus-fn", "near-humerus", 0.17, -0.09]] as const) {
  if (scale !== 50 || body.some((s) => s.bone === bone)) continue;
  body.push(ellipsoidOf({ id, bone, material: "fleece", tone: 0.72 }, [len * 0.5, 0, w], [len * 0.5, 0.05, 0.05]));
}
const REGION = (s: Json): string | undefined => {
  if (/-eye$/.test(s.id)) return undefined;
  const b: string = s.bone;
  if (/^(lumbar|thorax|pelvis)$/.test(b)) return "fleece-trunk";
  if (b === "neck") return "fleece-neck";
  if (/^(head|jaw)$/.test(b)) return "face";
  if (/^ear-/.test(b)) return "ears";
  if (/^tail/.test(b)) return "tail";
  if (/^hoof/.test(b)) return "hooves";
  if (/^(metacarpus|metatarsus)/.test(b)) return "lower-legs";
  if (/^(scapula|humerus|forearm|thigh|shank)/.test(b)) return "wool-upper-legs";
  return undefined;
};
for (const s of body) {
  const r = REGION(s);
  if (r === "ears" || s.id.endsWith("-eye")) s.material = "bare-skin";
  else if (r === "hooves") s.material = "hoof-horn";
  else if (r === "face" || r === "lower-legs") s.material = "short-hair";
}
const R: Record<string, [number, number]> = {
  pelvis: [0.05, 0.045], lumbar: [0.05, 0.045], thorax: [0.055, 0.05], neck: [0.04, 0.035], head: [0.028, 0.02], jaw: [0.012, 0.01], "ear-n": [0.004, 0.004], "ear-f": [0.004, 0.004],
  "tail-1": [0.014, 0.012], "tail-2": [0.012, 0.01],
  thigh: [0.045, 0.03], shank: [0.02, 0.015], metatarsus: [0.012, 0.01], hoof: [0.01, 0.009], scapula: [0.035, 0.03], humerus: [0.028, 0.02], forearm: [0.016, 0.013], metacarpus: [0.011, 0.01],
};
const structure = (): Json[] => centreOnLofts(c, shiftUndrawn(c, buildStructure(c, { material: "fleece", radius: (b) => R[b] ?? R[b.replace(/-[fh][fn]$/, "")] ?? [0.01, 0.01], length: { "ear-n": 0.6, "ear-f": 0.6 }, lateral: (b) => (/^ear-f$/.test(b) ? (scale >= 500 ? 0 : 0.0875) : /^ear-n$/.test(b) ? (scale >= 500 ? 0 : -0.0875) : /-hf$/.test(b) ? 0.13 : /-hn$/.test(b) ? -0.13 : /-ff$/.test(b) ? 0.1 : /-fn$/.test(b) ? -0.1 : 0) })));
const extras = (): Json[] => {
  const th: Json[] = (c.vol.thermal as Json[]).filter((x) => !/-eye$/.test(x.of ?? ""));
  const have = new Set(th.map((x) => x.region));
  const out: Json[] = [];
  const r = Math.max(0.0153, 0.51 * c.feature);
  const bodyAll: Json[] = c.vol.body;
  if (!have.has("ears")) {
    if (scale >= 500) out.push({ ...boxOf({ id: "th-ear-span", bone: "ear-n", material: "bare-skin", tone: 0.6 }, [0.05, 0, 0], [0.0765, 0.0765, 0.2]), region: "ears" });
    else for (const side of ["n", "f"] as const) out.push({ ...ellipsoidOf({ id: `th-ear-${side}`, bone: `ear-${side}`, material: "bare-skin", tone: 0.6 }, [0.05, 0, (side === "f" ? 1 : -1) * 0.09], [Math.max(0.05, 2 * r), r, r]), region: "ears" });
  }
  if (!have.has("hooves")) for (const side of ["n", "f"] as const) for (const hind of [true, false]) {
    const bone = hind ? `metatarsus-h${side}` : `metacarpus-f${side}`;
    const foot = bodyAll.find((x) => x.bone === bone)!;
    const last = (foot.sections as Json[])[(foot.sections as Json[]).length - 1];
    const at = [last.at[0] - 0.4 * r, last.at[1], last.at[2]];
    out.push({ ...(scale >= 500 ? boxOf({ id: `th-hoof-${hind ? "h" : "f"}${side}`, bone, material: "hoof-horn", tone: 0.55 }, at, [0.0765, 0.0765, 0.0765]) : capsuleOf({ id: `th-hoof-${hind ? "h" : "f"}${side}`, bone, material: "hoof-horn", tone: 0.55 }, [at[0]! - 0.9 * r, at[1]!, at[2]!], at as never, r, r)), region: "hooves" });
  }
  if (!have.has("tail")) out.push({ ...ellipsoidOf({ id: "th-tail", bone: "tail-1", material: "fleece", tone: 0.66 }, [0.05, 0, 0], [0.06, r, r]), region: "tail" });
  const all = [...th, ...out];
  clampJets(all, bodyAll);
  fixPlume(c, all, bodyAll, 0.2);
  return all;
};
const OWN: Record<number, T> = {
  50: { en: "Hand-set parts at 1:50: every hoof is cloven into two toes (two lofts per hoof), the ears are flat flaps rolled 68 degrees so that they stand out sideways as drawn in plan and front, the eyes are 17 mm balls in the head sides, the thighs narrow towards the hip, the wool tail hangs from the rump end (the tail root bone was moved from inside the rump to its back surface at the root) and the trunk sections are round woolly barrels from the elevation back and belly lines.", de: "Von Hand gesetzte Teile bei 1:50: jeder Huf ist in zwei Zehen gespalten (zwei Lofts je Huf), die Ohren sind flache, um 68 Grad gerollte Lappen, die wie in Grundriss und Vorderansicht gezeichnet seitlich abstehen, die Augen sind 17-mm-Kugeln in den Kopfseiten, die Schenkel verjüngen sich zur Hüfte, der Wollschwanz hängt am Ende der Kruppe (die Schwanzwurzel-Bone wurde an der Wurzel aus dem Inneren der Kruppe an ihre Rückfläche versetzt) und die Rumpfschnitte sind runde Wollfässer aus Rücken- und Bauchlinie des Aufrisses." },
  100: { en: "Hand-set parts at 1:100: hooves are single lofts (the cleft is below 15 mm), the ears and eyes as at 1:50 with this scale's minimum, shoulder, upper arm and forearm stay separate bones as drawn, the wool tail is one loft.", de: "Von Hand gesetzte Teile bei 1:100: die Hufe sind einzelne Lofts (die Spalte liegt unter 15 mm), Ohren und Augen wie bei 1:50 mit dem Minimum dieses Maßstabs, Schulter, Oberarm und Unterarm bleiben getrennte Bones wie gezeichnet, der Wollschwanz ist ein Loft." },
  200: { en: "Hand-set parts at 1:200: one loft per drawn bone, one ear pair from the single drawn ear and its far twin, the trunk masses follow the merged hindbody, chest and neck outlines; the ears and tail carry their own thermal solids where the body has none.", de: "Von Hand gesetzte Teile bei 1:200: ein Loft je gezeichneter Bone, ein Ohrenpaar aus dem einen gezeichneten Ohr und seinem fernen Zwilling, die Rumpfmassen folgen den zusammengefassten Umrissen von Hinterkörper, Brust und Hals; Ohren und Schwanz haben eigene Thermikkörper, wo der Körper keine hat." },
  500: { en: "Hand-set parts at 1:500: the elevation of this scale drew only a few leg strokes and no far legs, so the four near legs and the four far legs were redrawn at the root as closed outlines from the 1:200 shank, cannon and hoof outlines (see the report); legs are at least 75 mm thick, trunk, neck and head are single masses; ears, tail and hooves exist only in the thermal list.", de: "Von Hand gesetzte Teile bei 1:500: der Aufriss dieses Maßstabs zeichnete nur wenige Beinstriche und keine fernen Beine, daher wurden die vier nahen und die vier fernen Beine an der Wurzel als geschlossene Umrisse aus den Umrissen von Unterschenkel, Mittelfuß und Huf bei 1:200 neu gezeichnet (siehe Bericht); die Beine sind mindestens 75 mm dick, Rumpf, Hals und Kopf sind einzelne Massen; Ohren, Schwanz und Hufe gibt es nur in der Thermikliste." },
};
if (has(args, "--decisions")) saveLists(c, { decisions: decisions(scale as 50, { en: "sheep", de: "Schaf", trunk: { en: "rounded woolly barrel sections", de: "gerundete Wollfassschnitte" }, limb: { en: "round limb sections", de: "runde Gliedmaßenschnitte" }, head: { en: "narrow oval head sections", de: "schmale ovale Kopfschnitte" }, extra: [{ en: "cloven hooves and eyes", de: "gespaltene Hufe und Augen" }] }, { body: OWN[scale]! }) });
else if (has(args, "--extras")) {
  const add = extras();
  saveLists(c, { thermal: add });
  console.log(`thermal solids: ${add.length}`);
} else if (has(args, "--structure")) saveLists(c, { structure: structure() });
else if (has(args, "--map")) {
  const m = new Map<string, string[]>();
  for (const s of body) {
    const r = REGION(s);
    if (r !== undefined) m.set(r, [...(m.get(r) ?? []), s.id]);
  }
  console.log([...m.entries()].map(([r, ids]) => `${r}=${ids.join("+")}`).join(","));
} else if (has(args, "--write")) saveLists(c, { body });
else console.log(JSON.stringify(body.map((s) => [s.id, s.bone, s.sections?.length])));
