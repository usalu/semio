/** Author 25: horse body lists per scale from the scale's own drawings (see author-25-quad.ts). */
import { openCtx, saveLists, flag, has, drawn, ellipsoidOf, type Json } from "./kit-lib.ts";
import { buildBody, buildStructure, type Cfg } from "./author-25-quad.ts";
import { bboxLocal, part, thirdRadius } from "./author-25-lib.ts";
import { decisions, type T } from "./author-25-text.ts";

const args = process.argv.slice(2);
const scale = Number(flag(args, "--scale"));
const c = openCtx("animals/horse", scale);
const SIDE: Record<string, string> = { f: "far", n: "near" };
const BASE: Record<string, [string, string]> = { thigh: ["thigh", "thigh"], shank: ["gaskin", "gaskin"], cannon: ["hind-cannon", "fore-cannon"], pastern: ["hind-pastern", "fore-pastern"], hoof: ["hind-hoof", "fore-hoof"], scapula: ["scapula", "scapula"], humerus: ["humerus", "humerus"], radius: ["radius", "radius"] };
const name = (bone: string): string | undefined => {
  const m = /^(thigh|shank|cannon|pastern|hoof|scapula|humerus|radius)-([fh])([fn])$/.exec(bone);
  if (m !== null) return `${SIDE[m[3]!]}-${BASE[m[1]!]![m[2] === "h" ? 0 : 1]}`;
  const ear = /^ear-([fn])$/.exec(bone);
  if (ear !== null) return `${SIDE[ear[1]!]}-ear`;
  return ({ pelvis: "pelvis", lumbar: "hindbody", thorax: "chest", "neck-1": "neck-1", "neck-2": "neck-2", "neck-3": "neck-3", head: "head", jaw: "jaw", "tail-1": "tail-1", "tail-2": "tail-2", "tail-3": "tail-3", "tail-4": "tail-4" } as Record<string, string>)[bone];
};
const cfg: Cfg = {
  material: "short-coat",
  name,
  e: (b) => (/^(lumbar|thorax)$/.test(b) ? 3 : /^(thigh|scapula|humerus)/.test(b) ? 2.8 : /^(tail|ear)/.test(b) ? 2.2 : /^(neck|head|jaw)/.test(b) ? 2.4 : 2.5),
  tone: (b) => (/^ear-f$|-[fh]f$/.test(b) ? 0.5 : /^tail/.test(b) ? 0.2 : 0.62),
  elevExclude: /mane|forelock|muzzle|eye|hair/,
  latExclude: /mane|forelock|muzzle|eye|hair/,
  latFallback: Object.fromEntries(["hf", "hn", "ff", "fn"].flatMap((k) => [[`pastern-${k}`, `cannon-${k}`], [`hoof-${k}`, `cannon-${k}`]])),
  latScale: { thorax: 1.03, pelvis: 1.03 },
  twin: { pelvis: "lumbar", "ear-f": "ear-n", "tail-1": "tail-2", "tail-3": "tail-2", "tail-4": "tail-2", "neck-3": "neck-2" },
  latTaper: scale >= 0 ? {} : { "thigh-hf": { taper: [0.85, 1], inward: [0.85, 1] }, "thigh-hn": { taper: [0.85, 1], inward: [0.85, 1] } },
  fallbackHalf: { jaw: 0.05 },
  tolFactor: scale >= 200 ? 0.3 : 0.45,
  bevel: { lumbar: 1, thorax: 1, pelvis: 1 },
  ears: { "ear-f": { roll: -20, thick: Math.max(0.012, 0.6 * c.feature), scale: 0.5 }, "ear-n": { roll: 20, thick: Math.max(0.012, 0.6 * c.feature), scale: 0.5 } },
  tweak: (b, sc, p) => (b === "pelvis" ? { ...p, uRange: [-0.3, 0.2] } : b === "lumbar" ? { ...p, uRange: [0, 0.4] } : p),
};
const body: Json[] = buildBody(c, cfg);
const els = drawn(c, "elevation");
const has2d = (id: string): boolean => els.some((x) => x.id === id);
for (const e of els.filter((x) => /^(mane|forelock)/.test(x.id))) {
  body.push(part(c, { id: e.id, bone: e.bone, material: "mane-tail-hair", tone: 0.18, elev: [e.id], lat: { half: 0.022 }, e: 2.2 }));
}
const muz = has2d("muzzle") ? bboxLocal(c, "head", "muzzle") : null;
if (muz !== null) body.push(ellipsoidOf({ id: "muzzle", bone: "head", material: "bare-skin", tone: 0.2 }, [(muz[0] + muz[1]) / 2, (muz[2] + muz[3]) / 2, 0], [(muz[1] - muz[0]) / 2, (muz[3] - muz[2]) / 2, thirdRadius((muz[1] - muz[0]) / 2, (muz[3] - muz[2]) / 2, 0.0143)]));
const eye = has2d("eye") ? bboxLocal(c, "head", "eye") : null;
if (eye !== null) for (const w of [0.085, -0.085]) body.push(ellipsoidOf({ id: w > 0 ? "far-eye" : "near-eye", bone: "head", material: "eye-surface", tone: 0.04, role: "detail" }, [(eye[0] + eye[1]) / 2, (eye[2] + eye[3]) / 2, w], [Math.max(0.0108, 0.52 * c.feature), Math.max(0.0108, 0.52 * c.feature), Math.max(0.0108, 0.52 * c.feature)]));
const REGION = (s: Json): string | undefined => {
  if (/^(mane|forelock)/.test(s.id)) return "mane";
  if (s.id === "muzzle") return "muzzle-skin";
  if (/-eye$/.test(s.id)) return "eyes";
  const b: string = s.bone;
  if (/^(lumbar|thorax|neck)/.test(b)) return "coat-trunk";
  if (/^(head|jaw)$/.test(b)) return "coat-head";
  if (/^ear-/.test(b)) return "ears";
  if (/^hoof/.test(b)) return "hooves";
  if (/^(scapula|humerus|radius)|^(cannon|pastern)-f/.test(b)) return "coat-forelimbs";
  if (/^(thigh|shank)|^(cannon|pastern)-h/.test(b)) return "coat-hindlimbs";
  if (/^tail/.test(b)) return "tail";
  return undefined;
};
const R: Record<string, [number, number]> = {
  pelvis: [0.08, 0.07], lumbar: [0.07, 0.07], thorax: [0.09, 0.08], "neck-1": [0.075, 0.065], "neck-2": [0.06, 0.05], "neck-3": [0.05, 0.045], head: [0.05, 0.03], jaw: [0.025, 0.02], "ear-n": [0.01, 0.008], "ear-f": [0.01, 0.008],
  "tail-1": [0.02, 0.018], "tail-2": [0.018, 0.015], "tail-3": [0.014, 0.012], "tail-4": [0.012, 0.01],
  thigh: [0.055, 0.04], shank: [0.036, 0.026], cannon: [0.02, 0.016], pastern: [0.017, 0.015], hoof: [0.02, 0.016], scapula: [0.04, 0.036], humerus: [0.045, 0.035], radius: [0.03, 0.022],
};
const structure = (): Json[] => buildStructure(c, { material: "short-coat", length: { "ear-n": 0.8, "ear-f": 0.8 }, radius: (b) => R[b] ?? R[b.replace(/-[fh][fn]$/, "")] ?? [0.02, 0.02], lateral: (b) => {
    const m = /^(pastern|hoof)-([fh][fn])$/.exec(b);
    const leg = m === null || (c.vol.body as Json[]).some((x) => x.bone === b) ? undefined : (c.vol.body as Json[]).find((x) => x.bone === `cannon-${m[2]}` && x.kind === "loft");
    return leg === undefined ? undefined : (leg.sections as Json[]).reduce((n: number, q: Json) => n + q.at[2], 0) / leg.sections.length;
  } });
const extras = (): Json[] => {
  const keepOf = (x: Json): boolean => (!["muzzle-skin", "eyes"].includes(x.region) || /^(muzzle|.*-eye)$/.test(x.of ?? "")) && (!["muzzle-skin", "eyes", "ears", "hooves", "mane"].includes(x.region) || x.of === undefined || (c.vol.body as Json[]).some((b) => b.id === x.of && /^(mane|forelock|muzzle|hoof|far-hind-hoof|near-hind-hoof)/.test(b.id) || b.bone?.startsWith("hoof") || b.bone?.startsWith("ear")));
  const th: Json[] = (c.vol.thermal as Json[]).filter(keepOf);
  const have = new Set(th.map((x) => x.region));
  const out: Json[] = [];
  const r = Math.max(0.0153, 0.51 * c.feature);
  if (!have.has("muzzle-skin")) out.push({ ...ellipsoidOf({ id: "th-muzzle", bone: "head", material: "bare-skin", tone: 0.2 }, [0.52, -0.02, 0], [Math.max(0.04, 1.5 * r), Math.max(0.03, r), Math.max(0.035, r)]), region: "muzzle-skin" });
  if (!have.has("eyes")) for (const w of [0.07, -0.07]) out.push({ ...ellipsoidOf({ id: `th-eye-${w > 0 ? "f" : "n"}`, bone: "head", material: "eye-surface", tone: 0.04 }, [0.3, 0.07, w], [Math.max(0.0108, r), Math.max(0.0108, r), Math.max(0.0108, r)]), region: "eyes" });
  if (!have.has("ears")) for (const side of ["n", "f"] as const) out.push({ ...ellipsoidOf({ id: `th-ear-${side}`, bone: `ear-${side}`, material: "short-coat", tone: 0.5 }, [0.07, 0, (side === "f" ? 1 : -1) * 0.06], [Math.max(0.08, r * 3), r, r]), region: "ears" });
  if (!have.has("hooves")) for (const side of ["n", "f"] as const) for (const hind of [true, false]) out.push({ ...ellipsoidOf({ id: `th-hoof-${hind ? "h" : "f"}${side}`, bone: `hoof-${hind ? "h" : "f"}${side}`, material: "hoof-horn", tone: 0.2 }, [0.045, 0, (side === "f" ? 1 : -1) * 0.1], [Math.max(0.05, r * 1.5), Math.max(0.045, r), Math.max(0.045, r)]), region: "hooves" });
  if (!have.has("mane")) out.push({ ...ellipsoidOf({ id: "th-mane", bone: "neck-2", material: "mane-tail-hair", tone: 0.18 }, [0.14, 0.14, 0], [0.14, Math.max(0.05, 2 * r), Math.max(0.03, r)]), region: "mane" });
  if (!have.has("tail")) out.push({ ...ellipsoidOf({ id: "th-tail", bone: "tail-2", material: "mane-tail-hair", tone: 0.18 }, [0.2, 0, 0], [0.2, Math.max(0.04, r), Math.max(0.03, r)]), region: "tail" });
  return [...th, ...out];
};
const OWN: Record<number, T> = {
  50: { en: "Hand-set parts at 1:50: the mane and the forelock are thin sheets (44 mm thick) standing on the crest line of the three neck bones, the muzzle is a bare-skin ellipsoid whose whole surface equals the declared muzzle skin, the eyes are 22 mm balls whose surfaces equal the declared eye skin, the ears are flaps rolled 20 degrees outwards, the thighs and the pelvis follow the drawn widths without narrowing, the lumbar bone takes its width from the loin capsule added to the plan drawing (the plan had a waist between pelvis and chest where the elevation has a straight back); the four tail bones carry the tail hair as separate lofts.", de: "Von Hand gesetzte Teile bei 1:50: Mähne und Schopf sind dünne Bleche (44 mm dick) auf der Kammlinie der drei Halsknochen, das Maul ist ein Ellipsoid aus nackter Haut, dessen ganze Oberfläche der angegebenen Maulhaut entspricht, die Augen sind 22-mm-Kugeln, deren Oberfläche der angegebenen Augenhaut entspricht, die Ohren sind um 20 Grad nach außen gerollte Lappen, Schenkel und Becken folgen den gezeichneten Breiten ohne Verjüngung, der Lendenknochen nimmt seine Breite von der Lendenkapsel, die dem Grundriss hinzugefügt wurde (der Grundriss hatte eine Taille zwischen Becken und Brust, wo der Aufriss einen geraden Rücken hat); die vier Schweifknochen tragen das Schweifhaar als eigene Lofts." },
  100: { en: "Hand-set parts at 1:100: mane sheets, forelock, muzzle, eyes and ears as at 1:50 with the 15 mm minimum of this scale; pastern and hoof stay separate bones as drawn, undrawn legs take the lateral position of the cannon.", de: "Von Hand gesetzte Teile bei 1:100: Mähnenbleche, Schopf, Maul, Augen und Ohren wie bei 1:50 mit dem Minimum von 15 mm dieses Maßstabs; Fessel und Huf bleiben getrennte Knochen wie gezeichnet, ungezeichnete Beine nehmen die seitliche Lage des Röhrbeins." },
  200: { en: "Hand-set parts at 1:200: the mane is one sheet on the middle neck bone, forelock, muzzle and eyes are no longer drawn as solids of the body (thermal solids carry the regions), the ears are drawn on the near side only so the far ear follows its twin.", de: "Von Hand gesetzte Teile bei 1:200: die Mähne ist ein Blech am mittleren Halsknochen, Schopf, Maul und Augen sind im Körper nicht mehr als Körper gezeichnet (Thermikkörper tragen die Regionen), die Ohren sind nur auf der nahen Seite gezeichnet, das ferne folgt seinem Zwilling." },
  500: { en: "Hand-set parts at 1:500: the generalised horse: trunk in two masses, neck in two wedges, head, the tail in two pieces, each leg in upper, gaskin and lower parts; the lower legs, tail and neck wedge outlines were redrawn from the 1:200 outlines because their coordinates were not in the bone frames; mane, forelock, muzzle, eyes and ears only exist in the thermal list.", de: "Von Hand gesetzte Teile bei 1:500: das verallgemeinerte Pferd: Rumpf in zwei Massen, Hals in zwei Keilen, Kopf, Schweif in zwei Stücken, jedes Bein in Ober-, Unter- und Unterteil; die Umrisse der Unterbeine, des Schweifs und des Halskeils wurden aus den Umrissen von 1:200 neu gezeichnet, weil ihre Koordinaten nicht in den Knochenrahmen lagen; Mähne, Schopf, Maul, Augen und Ohren gibt es nur in der Thermikliste." },
};
const dec = () => decisions(scale as 50, { en: "horse", de: "Pferd", trunk: { en: "rounded barrel sections", de: "gerundete Rumpfschnitte" }, limb: { en: "round limb sections", de: "runde Gliedmaßenschnitte" }, head: { en: "oval head sections", de: "ovale Kopfschnitte" }, extra: [{ en: "mane, forelock, muzzle and eyes", de: "Mähne, Schopf, Maul und Augen" }] }, { body: OWN[scale]! });
if (has(args, "--decisions")) saveLists(c, { decisions: dec() });
else if (has(args, "--extras")) saveLists(c, { thermal: extras() });
else if (has(args, "--structure")) saveLists(c, { structure: structure() });
else if (has(args, "--map")) {
  const m = new Map<string, string[]>();
  for (const s of body) {
    const r = REGION(s);
    if (r !== undefined) m.set(r, [...(m.get(r) ?? []), s.id]);
  }
  console.log([...m.entries()].map(([r, ids]) => `${r}=${ids.join("+")}`).join(","));
} else if (has(args, "--write")) saveLists(c, { body });
else console.log(JSON.stringify(body.map((s) => [s.id, s.bone, s.sections?.length])));
