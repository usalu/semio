/** Author 25: dairy-cow body lists per scale from the scale's own drawings (see author-25-quad.ts). */
import { openCtx, saveLists, flag, has, drawn, ellipsoidOf, capsuleOf, type Json } from "./kit-lib.ts";
import { buildBody, buildStructure, surfaceW, type Cfg } from "./author-25-quad.ts";
import { bboxLocal, thirdRadius } from "./author-25-lib.ts";
import { decisions, type T } from "./author-25-text.ts";

const args = process.argv.slice(2);
const scale = Number(flag(args, "--scale"));
const c = openCtx("animals/dairy-cow", scale);
const SIDE: Record<string, string> = { f: "far", n: "near" };
const BASE: Record<string, string> = { thigh: "thigh", shank: "gaskin", metatarsus: "hind-cannon", hoof: "hoof", scapula: "scapula", humerus: "humerus", forearm: "forearm", cannon: "fore-cannon" };
const name = (bone: string): string | undefined => {
  const m = /^(thigh|shank|metatarsus|hoof|scapula|humerus|forearm|cannon)-([fh])([fn])$/.exec(bone);
  if (m !== null) return `${SIDE[m[3]!]}-${m[1] === "hoof" ? (m[2] === "h" ? "hind-hoof" : "fore-hoof") : BASE[m[1]!]}`;
  const ear = /^ear-([fn])$/.exec(bone);
  if (ear !== null) return `${SIDE[ear[1]!]}-ear`;
  return ({ lumbar: "hindbody", thorax: "chest", neck: "neck", head: "head", jaw: "jaw", udder: "udder", "tail-1": "tail-1", "tail-2": "tail-2", "tail-3": "tail-3" } as Record<string, string>)[bone];
};
const cfg: Cfg = {
  material: "white-hair",
  name,
  e: (b) => (/^(lumbar|thorax)$/.test(b) ? Number(process.env.ET ?? 3) : /^(thigh|scapula|humerus)/.test(b) ? 2.8 : /^(tail|ear)/.test(b) ? 2.2 : /^(neck|head|jaw|udder)$/.test(b) ? 2.4 : 2.5),
  tone: (b) => (/^ear-f$|-[fh]f$/.test(b) ? 0.7 : /^tail/.test(b) ? 0.5 : 0.85),
  elevExclude: /teat|muzzle|blaze|patch|rumen|eye/,
  latFallback: { "cannon-fn": "forearm-fn", "cannon-ff": "forearm-ff", "metatarsus-hn": "shank-hn", "metatarsus-hf": "shank-hf", "hoof-fn": "forearm-fn", "hoof-ff": "forearm-ff", "hoof-hn": "shank-hn", "hoof-hf": "shank-hf", "tail-1": "tail-2", "tail-3": "tail-2" },
  twin: { "ear-f": "ear-n", "tail-1": "tail-2", "tail-3": "tail-2" },
  latScale: { thorax: 1.045 },
  fallbackHalf: { jaw: 0.06 },
  latTaper: scale >= 0 ? {} : { "thigh-hf": { taper: [0.7, 0.95], inward: [0.75, 1] }, "thigh-hn": { taper: [0.7, 0.95], inward: [0.75, 1] } },
  tolFactor: scale >= 200 ? 0.2 : 0.45,
  bevel: { lumbar: 1, thorax: 1, neck: 1 },
  ears: { "ear-f": { roll: -22, thick: Math.max(0.018, 0.6 * c.feature), scale: 0.5 }, "ear-n": { roll: 22, thick: Math.max(0.018, 0.6 * c.feature), scale: 0.5 } },
};
const body: Json[] = buildBody(c, cfg);
const has2d = (id: string): boolean => drawn(c, "elevation").some((x) => x.id === id);
const trunkOf = (bone: string): Json => body.find((s) => s.bone === bone)!;
const mat = (m: string, tone: number, role = "body"): { id: string; bone: string; material: string; tone: number; role: string } => ({ id: "", bone: "", material: m, tone, role });
for (const sid of ["patch-rump", "patch-barrel", "patch-chest", "rumen-flank"]) {
  if (!has2d(sid)) continue;
  const shape = drawn(c, "elevation").find((x) => x.id === sid)!;
  const bone = shape.bone;
  const b = bboxLocal(c, bone, sid)!;
  const uc = (b[0] + b[1]) / 2;
  const vc = (b[2] + b[3]) / 2;
  let w = Infinity;
  for (let k = -3; k <= 3; k++) w = Math.min(w, Math.abs(surfaceW(trunkOf(bone), uc, vc + (k / 3) * ((b[3] - b[2]) / 2) * 0.8)));
  const t = 0.006;
  body.push({ ...ellipsoidOf({ ...mat(sid === "rumen-flank" ? "white-hair" : "black-hair", sid === "rumen-flank" ? 0.6 : 0.15), id: sid, bone }, [uc, vc, -(w - 0.004)], [(b[1] - b[0]) / 2, (b[3] - b[2]) / 2, t]), texture: true });
}
if (has2d("udder")) {
  for (const [sid, lat] of [["teat-front", 0.06], ["teat-rear", 0.06]] as const) {
    const b = bboxLocal(c, "udder", sid);
    if (b === null) continue;
    for (const side of [-1, 1]) body.push(ellipsoidOf({ ...mat("bare-skin", 0.4), id: `${sid}-${side < 0 ? "near" : "far"}`, bone: "udder" }, [(b[0] + b[1]) / 2, (b[2] + b[3]) / 2, side * lat], [(b[1] - b[0]) / 2, (b[3] - b[2]) / 2, (b[1] - b[0]) / 2]));
  }
}
const muz = has2d("muzzle") ? bboxLocal(c, "head", "muzzle") : null;
if (muz !== null) body.push(ellipsoidOf({ ...mat("bare-skin", 0.25), id: "muzzle", bone: "head" }, [(muz[0] + muz[1]) / 2, (muz[2] + muz[3]) / 2, 0], [0.85 * (muz[1] - muz[0]) / 2, 0.85 * (muz[3] - muz[2]) / 2, thirdRadius(0.85 * (muz[1] - muz[0]) / 2, 0.85 * (muz[3] - muz[2]) / 2, 0.036, 0.045)]));
const eye = has2d("eye") ? bboxLocal(c, "head", "eye") : null;
if (eye !== null) for (const w of [0.105, -0.105]) body.push(ellipsoidOf({ ...mat("black-hair", 0.04, "detail"), id: w > 0 ? "far-eye" : "near-eye", bone: "head" }, [(eye[0] + eye[1]) / 2, (eye[2] + eye[3]) / 2, w], [Math.max(0.0089, 0.52 * c.feature), Math.max(0.0089, 0.52 * c.feature), Math.max(0.0089, 0.52 * c.feature)]));
if (scale === 500) body.push(capsuleOf({ ...mat("white-hair", 0.7), id: "ear-span", bone: "ear-n" }, [0.005, 0, -0.28], [0.005, 0, 0.28], 0.05, 0.05));
const REGION = (s: Json): string | undefined => {
  if (/^(patch-)/.test(s.id)) return "coat-patches";
  if (s.id === "rumen-flank") return "flank-rumen";
  if (s.id === "muzzle") return "muzzle";
  if (/-eye$/.test(s.id)) return "eyes";
  if (/^teat/.test(s.id)) return "udder";
    const b: string = s.bone;
  if (/^(lumbar|thorax)$/.test(b)) return "coat-trunk";
  if (/^(neck|head|jaw)$/.test(b)) return "coat-head-neck";
  if (/^ear-/.test(b)) return "ears";
  if (b === "udder") return "udder";
  if (/^(scapula|humerus|forearm)/.test(b)) return "coat-forelimbs";
  if (/^(thigh|shank)/.test(b)) return "coat-hindlimbs";
  if (/^(metatarsus|cannon)/.test(b)) return "lower-legs";
  if (/^hoof/.test(b)) return "hooves";
  if (/^tail/.test(b)) return "coat-tail";
  return undefined;
};
const R: Record<string, [number, number]> = {
  pelvis: [0.07, 0.065], lumbar: [0.065, 0.065], thorax: [0.09, 0.075], neck: [0.07, 0.06], head: [0.045, 0.03], jaw: [0.03, 0.025], "ear-n": [0.012, 0.01], "ear-f": [0.012, 0.01], udder: [0.05, 0.04],
  "tail-1": [0.022, 0.018], "tail-2": [0.018, 0.014], "tail-3": [0.014, 0.01],
  thigh: [0.05, 0.036], shank: [0.032, 0.024], metatarsus: [0.022, 0.018], hoof: [0.02, 0.016], scapula: [0.04, 0.036], humerus: [0.04, 0.03], forearm: [0.03, 0.024], cannon: [0.022, 0.018],
};
const structure = (): Json[] => buildStructure(c, { diagonal: /^ear-/, centreV: /^(hoof|metatarsus|cannon)/, material: "white-hair", length: { udder: 0.7, ...(scale >= 500 ? { "shank-hf": 0.62, "shank-hn": 0.62, "forearm-ff": 0.62, "forearm-fn": 0.62 } : {}) }, radius: (b) => R[b] ?? R[b.replace(/-[fh][fn]$/, "")] ?? [0.02, 0.02], lateral: (b) => (/^ear-/.test(b) ? undefined : /^(thigh|shank|metatarsus|hoof)-[fh]f$/.test(b) ? 0.22 : /^(thigh|shank|metatarsus|hoof)-[fh]n$/.test(b) ? -0.22 : /-[fh]f$/.test(b) ? 0.2 : /-[fh]n$/.test(b) ? -0.2 : 0) });
const extras = (): Json[] => {
  const keep = (x: Json): boolean => (!["muzzle", "eyes"].includes(x.region) || /^(muzzle|.*-eye)$/.test(x.of ?? "")) && (!["muzzle", "eyes", "ears", "hooves", "coat-patches", "flank-rumen", "udder", "lower-legs"].includes(x.region) || x.of === undefined || (c.vol.body as Json[]).some((b) => b.id === x.of && (/^(patch|rumen|muzzle|teat)/.test(b.id) || b.id === x.of)));
  const th: Json[] = (c.vol.thermal as Json[]).filter(keep);
  const have = new Set(th.map((x) => x.region));
  const out: Json[] = [];
  const r = Math.max(0.0153, 0.51 * c.feature);
  const bodyAll: Json[] = c.vol.body;
  const trunk = bodyAll.find((b) => b.bone === "lumbar")!;
  for (const reg of ["coat-patches", "flank-rumen"]) if (!have.has(reg)) out.push({ id: `th-${reg}`, of: trunk.id, region: reg });
  if (!have.has("muzzle")) out.push({ ...ellipsoidOf({ id: "th-muzzle", bone: "head", material: "bare-skin", tone: 0.25 }, [0.45, -0.01, 0], [Math.max(0.045, 1.5 * r), Math.max(0.04, r), Math.max(0.04, r)]), region: "muzzle" });
  if (!have.has("eyes")) for (const w of [0.09, -0.09]) out.push({ ...ellipsoidOf({ id: `th-eye-${w > 0 ? "f" : "n"}`, bone: "head", material: "black-hair", tone: 0.04 }, [0.3, 0.1, w], [Math.max(0.0089, r), Math.max(0.0089, r), Math.max(0.0089, r)]), region: "eyes" });
  if (!have.has("ears")) for (const side of ["n", "f"] as const) out.push({ ...ellipsoidOf({ id: `th-ear-${side}`, bone: `ear-${side}`, material: "white-hair", tone: 0.7 }, [0.1, 0, (side === "f" ? 1 : -1) * 0.19], [Math.max(0.1, r * 3), r, r]), region: "ears" });
  if (!have.has("hooves")) for (const side of ["n", "f"] as const) for (const hind of [true, false]) out.push({ ...ellipsoidOf({ id: `th-hoof-${hind ? "h" : "f"}${side}`, bone: `hoof-${hind ? "h" : "f"}${side}`, material: "claw-horn", tone: 0.2 }, [0.07, 0, (side === "f" ? 1 : -1) * (hind ? 0.22 : 0.2)], [0.075, Math.max(0.045, r), Math.max(0.045, r)]), region: "hooves" });
  if (!have.has("udder")) out.push({ id: "th-udder", of: (bodyAll.find((b) => b.bone === "udder") as Json).id, region: "udder" });
  return [...th, ...out];
};
const OWN: Record<number, T> = {
  50: { en: "Hand-set parts at 1:50: the black patches (rump, barrel, chest) and the rumen flank are thin lens skins flush on the near side of the trunk, the four teats are ellipsoids under the udder at the lateral offsets the front and plan drawings give, the muzzle is a bare-skin ellipsoid, the eyes are 40 mm balls, the ears are flaps that follow the drawn diagonal ear (out and back from the head, rolled 22 degrees, 36 mm thick) and their structure members run obliquely inside them.", de: "Von Hand gesetzte Teile bei 1:50: die schwarzen Flecken (Kruppe, Rumpf, Brust) und die Pansenflanke sind dünne Linsenhäute bündig auf der nahen Rumpfseite, die vier Zitzen sind Ellipsoide unter dem Euter an den seitlichen Abständen aus Vorderansicht und Grundriss, das Flotzmaul ist ein Ellipsoid aus nackter Haut, die Augen sind 40-mm-Kugeln, die Ohren sind Lappen entlang des gezeichneten diagonalen Ohrs (vom Kopf nach außen und hinten, um 22 Grad gerollt, 36 mm dick), ihre Strukturglieder verlaufen schräg darin; das Flotzmaul ist ein Ellipsoid, dessen ganze Oberfläche nahe der angegebenen Fläche liegt." },
  100: { en: "Hand-set parts at 1:100: patches, flank, teats, muzzle, eyes and diagonal flap ears as at 1:50 with this scale's drawings and the 15 mm minimum; the blaze is not a solid (flat marking).", de: "Von Hand gesetzte Teile bei 1:100: Flecken, Flanke, Zitzen, Flotzmaul, Augen und diagonale Ohrlappen wie bei 1:50 mit den Zeichnungen dieses Maßstabs und dem Minimum von 15 mm; die Blesse ist kein Körper (flache Markierung)." },
  200: { en: "Hand-set parts at 1:200: the drawing is one mass per bone, so every bone has one loft; no patches, teats or eyes in the body; muzzle, eyes, hooves and ears carry their own thermal solids where the body has none.", de: "Von Hand gesetzte Teile bei 1:200: die Zeichnung hat eine Masse je Bone, also ein Loft je Bone; keine Flecken, Zitzen oder Augen im Körper; Flotzmaul, Augen, Hufe und Ohren haben eigene Thermikkörper, wo der Körper keine hat." },
  500: { en: "Hand-set parts at 1:500: legs follow the open leg strokes of the drawing at 125 mm (the real leg thickness), thigh, trunk masses, neck, head and udder are single lofts, the tail is three lofts (root and tip outlines added from the 1:200 tail masses, the middle outline was redrawn from it earlier); the ears are one 100 mm bar across the head as drawn; muzzle, eyes and hooves only exist in the thermal list.", de: "Von Hand gesetzte Teile bei 1:500: die Beine folgen den offenen Beinstrichen der Zeichnung mit 125 mm (der wirklichen Beindicke), Oberschenkel, Rumpfmassen, Hals, Kopf und Euter sind einzelne Lofts, die Rute besteht aus drei Lofts (Wurzel- und Spitzenumriss aus den Rutenmassen von 1:200 ergänzt, der mittlere Umriss wurde früher daraus neu gezeichnet); die Ohren sind ein 100-mm-Balken quer über den Kopf wie gezeichnet; Flotzmaul, Augen und Hufe gibt es nur in der Thermikliste." },
};
const dec = () => decisions(scale as 50, { en: "dairy cow", de: "Milchkuh", trunk: { en: "rounded barrel sections", de: "gerundete Rumpfschnitte" }, limb: { en: "round limb sections", de: "runde Gliedmaßenschnitte" }, head: { en: "oval head sections", de: "ovale Kopfschnitte" }, extra: [{ en: "patches, teats, muzzle and eyes", de: "Flecken, Zitzen, Flotzmaul und Augen" }] }, { body: OWN[scale]! });
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
