/** 🦊 Author 46: red-fox body, thermal extras, structure and decisions per scale, built bone by bone from the scale's own drawings (author-25-quad.ts builder, hand-set eyes and nose, upright ears, three-part brush). */
import { openCtx, saveLists, flag, has, drawn, ellipsoidOf, capsuleOf, boxOf, type Json } from "./kit-lib.ts";
import { autoParts, buildStructure, type Cfg } from "./author-25-quad.ts";
import { bboxLocal, part } from "./author-25-lib.ts";
import { centreOnLofts, clampJets, fixPlume, shiftUndrawn } from "./author-46-lib.ts";
import { decisions, type T } from "./author-25-text.ts";

const args = process.argv.slice(2);
const scale = Number(flag(args, "--scale"));
const c = openCtx("animals/red-fox", scale);
const SIDE: Record<string, string> = { f: "far", n: "near" };
const BASE: Record<string, string> = { thigh: "thigh", shank: "gaskin", metatarsus: "hind-cannon", paw: "paw", scapula: "shoulder", humerus: "humerus", forearm: "forearm", metacarpus: "fore-cannon" };
const name = (bone: string): string | undefined => {
  const m = /^(thigh|shank|metatarsus|paw|scapula|humerus|forearm|metacarpus)-([fh])([fn])$/.exec(bone);
  if (m !== null) return `${SIDE[m[3]!]}-${m[1] === "paw" ? (m[2] === "h" ? "hind-paw" : "fore-paw") : BASE[m[1]!]}`;
  const ear = /^ear-([fn])$/.exec(bone);
  if (ear !== null) return `${SIDE[ear[1]!]}-ear`;
  return ({ lumbar: "hindbody", thorax: "chest", neck: "neck", head: "head", jaw: "jaw", "tail-1": "brush-root", "tail-2": "brush-mid", "tail-3": "brush-tip" } as Record<string, string>)[bone];
};
const cfg: Cfg = {
  material: "red-fur",
  name,
  e: (b) => (/^(lumbar|thorax)$/.test(b) ? 2.2 : /^(thigh|scapula|humerus)/.test(b) ? 2.7 : /^(tail|ear)/.test(b) ? 2.2 : /^(neck|head|jaw)$/.test(b) ? 2.2 : /^paw/.test(b) ? 2.2 : 2.4),
  tone: (b) => (/^ear-n$/.test(b) ? 0.85 : /^ear-f$/.test(b) ? 0.75 : /^tail-3$/.test(b) ? 0.3 : /^tail/.test(b) ? 0.62 : /^(shank|metatarsus|paw|forearm|metacarpus)-[fh]n$/.test(b) ? 0.85 : /^(shank|metatarsus|paw|forearm|metacarpus)-[fh]f$/.test(b) ? 0.7 : 0.7),
  elevOnly: { head: ["head"], jaw: ["jaw"] },
  elevExclude: /^(eye|nose|mouth-line)/,
  latBone: {},
  twin: { "ear-f": "ear-n" },
  latTaper: { "thigh-hf": { taper: [0.75, 0.95], inward: [0.8, 1] }, "thigh-hn": { taper: [0.75, 0.95], inward: [0.8, 1] } },
  bevel: { lumbar: 0.92, thorax: 0.92, neck: 0.92 },
};
const body: Json[] = autoParts(c, cfg).map((p) => part(c, p));
const has2d = (id: string): boolean => drawn(c, "elevation").some((x) => x.id === id);
const eb = has2d("eye") ? bboxLocal(c, "head", "eye") : null;
const er = Math.max(0.0042, 0.56 * c.feature);
if (eb !== null) for (const w of [0.026, -0.026]) body.push(ellipsoidOf({ id: w > 0 ? "far-eye" : "near-eye", bone: "head", material: "dark-fur", tone: 0.04, role: "detail" }, [(eb[0] + eb[1]) / 2, (eb[2] + eb[3]) / 2, w], [Math.max(0.0105, er), Math.max(0.0095, er), Math.max(0.0095, er)]));
const nb = has2d("nose") ? bboxLocal(c, "head", "nose") : null;
if (nb !== null) body.push(ellipsoidOf({ id: "nose", bone: "head", material: "dark-fur", tone: 0.04, role: "detail" }, [(nb[0] + nb[1]) / 2, (nb[2] + nb[3]) / 2, 0], [Math.max(0.0115, (nb[1] - nb[0]) / 2), Math.max(0.01, (nb[3] - nb[2]) / 2), 0.0115]));
const REGION = (s: Json): string | undefined => {
  if (/-eye$/.test(s.id) || s.id === "nose") return undefined;
  const b: string = s.bone;
  if (/^(lumbar|thorax|neck|pelvis)$/.test(b)) return "coat-trunk";
  if (/^(head|jaw)$/.test(b)) return "face";
  if (/^ear-/.test(b)) return "ears";
  if (/^tail/.test(b)) return "brush";
  if (/^paw/.test(b)) return "paws";
  if (/^(scapula|humerus|forearm|metacarpus|thigh|shank|metatarsus)/.test(b)) return "coat-legs";
  return undefined;
};
const R: Record<string, [number, number]> = {
  pelvis: [0.03, 0.028], lumbar: [0.03, 0.03], thorax: [0.04, 0.035], neck: [0.03, 0.025], head: [0.025, 0.014], jaw: [0.01, 0.008], "ear-n": [0.006, 0.004], "ear-f": [0.006, 0.004],
  "tail-1": [0.03, 0.03], "tail-2": [0.03, 0.025], "tail-3": [0.02, 0.012],
  thigh: [0.03, 0.02], shank: [0.012, 0.009], metatarsus: [0.008, 0.007], paw: [0.007, 0.006], scapula: [0.02, 0.018], humerus: [0.015, 0.012], forearm: [0.01, 0.008], metacarpus: [0.007, 0.006],
};
const structure = (): Json[] => centreOnLofts(c, shiftUndrawn(c, buildStructure(c, { material: "red-fur", radius: (b) => R[b] ?? R[b.replace(/-[fh][fn]$/, "")] ?? [0.01, 0.01], lateral: (b) => (/^ear-f$/.test(b) ? (scale >= 200 ? 0 : 0.033) : /^ear-n$/.test(b) ? (scale >= 200 ? 0 : -0.033) : /-hf$/.test(b) ? 0.045 : /-hn$/.test(b) ? -0.045 : /-ff$/.test(b) ? 0.04 : /-fn$/.test(b) ? -0.04 : 0) }), { ear: 0.9, tail: 0.9 }));
const extras = (): Json[] => {
  const th: Json[] = (c.vol.thermal as Json[]).filter((x) => !/-eye$|^nose$/.test(x.of ?? ""));
  const have = new Set(th.map((x) => x.region));
  const out: Json[] = [];
  const r = Math.max(0.0153, 0.51 * c.feature);
  const bodyAll: Json[] = c.vol.body;
  if (!have.has("ears")) for (const side of ["n", "f"] as const) out.push({ ...ellipsoidOf({ id: `th-ear-${side}`, bone: `ear-${side}`, material: "dark-fur", tone: 0.8 }, [0.04, 0, (side === "f" ? 1 : -1) * 0.033], [Math.max(0.04, 2 * r), r, r]), region: "ears" });
  if (!have.has("paws")) for (const side of ["n", "f"] as const) for (const hind of [true, false]) {
    const bone = hind ? `metatarsus-h${side}` : `metacarpus-f${side}`;
    const foot = bodyAll.find((x) => x.bone === bone)!;
    const last = (foot.sections as Json[])[(foot.sections as Json[]).length - 1];
    out.push({ ...ellipsoidOf({ id: `th-paw-${hind ? "h" : "f"}${side}`, bone, material: "dark-fur", tone: 0.8 }, [last.at[0] - 0.3 * r, last.at[1], last.at[2]], [Math.max(0.03, 1.2 * r), r, r]), region: "paws" });
  }
  if (!have.has("brush")) out.push({ ...ellipsoidOf({ id: "th-brush", bone: "tail-1", material: "red-fur", tone: 0.62 }, [0.1, 0, 0], [0.15, r, r]), region: "brush" });
  const all = [...th, ...out];
  clampJets(all, bodyAll);
  fixPlume(c, all, bodyAll, 0.07);
  return all;
};
const OWN: Record<number, T> = {
  50: { en: "Hand-set parts at 1:50: the ears are upright cone lofts along their bones (lateral extent from the front and plan drawings, 44 to 54 mm), the eyes are 21 mm balls and the nose a dark 23 mm ellipsoid in the head sides and at the muzzle tip, the thighs narrow towards the hip, the brush is three lofts (root, mid, white-tipped tip) along its three bones and the trunk sections are narrow oval barrels with the spine and belly lines of the elevation.", de: "Von Hand gesetzte Teile bei 1:50: die Ohren sind aufrechte Kegellofts entlang ihrer Bones (seitliche Ausdehnung aus Vorderansicht und Grundriss, 44 bis 54 mm), die Augen sind 21-mm-Kugeln und die Nase ein dunkles 23-mm-Ellipsoid in den Kopfseiten und an der Schnauzenspitze, die Schenkel verjüngen sich zur Hüfte, die Lunte besteht aus drei Lofts (Wurzel, Mitte, Spitze mit weißer Spitze) entlang ihrer drei Bones und die Rumpfschnitte sind schmale ovale Fässer mit Rücken- und Bauchlinie des Aufrisses." },
  100: { en: "Hand-set parts at 1:100: upright ears, eyes and nose as at 1:50 with this scale's minimum; the brush is three lofts as drawn (brush, brush-2, tip), shoulder, upper arm and forearm stay separate bones.", de: "Von Hand gesetzte Teile bei 1:100: aufrechte Ohren, Augen und Nase wie bei 1:50 mit dem Minimum dieses Maßstabs; die Lunte besteht aus drei Lofts wie gezeichnet (Lunte, Lunte-2, Spitze), Schulter, Oberarm und Unterarm bleiben getrennte Bones." },
  200: { en: "Hand-set parts at 1:200: one loft per drawn bone, one ear from the single drawn ear and its far twin, the three brush lofts from the three drawn brush shapes; ears, paws and the brush carry their own thermal solids where the body has none.", de: "Von Hand gesetzte Teile bei 1:200: ein Loft je gezeichneter Bone, ein Ohr aus dem einen gezeichneten Ohr und seinem fernen Zwilling, die drei Luntenlofts aus den drei gezeichneten Luntenformen; Ohren, Pfoten und Lunte haben eigene Thermikkörper, wo der Körper keine hat." },
};
if (has(args, "--decisions")) saveLists(c, { decisions: decisions(scale as 50, { en: "red fox", de: "Rotfuchs", trunk: { en: "narrow oval barrel sections", de: "schmale ovale Fassschnitte" }, limb: { en: "slender round limb sections", de: "schlanke runde Gliedmaßenschnitte" }, head: { en: "narrow wedge head sections", de: "schmale keilförmige Kopfschnitte" }, extra: [{ en: "eyes and nose", de: "Augen und Nase" }] }, { body: OWN[scale]! }) });
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
