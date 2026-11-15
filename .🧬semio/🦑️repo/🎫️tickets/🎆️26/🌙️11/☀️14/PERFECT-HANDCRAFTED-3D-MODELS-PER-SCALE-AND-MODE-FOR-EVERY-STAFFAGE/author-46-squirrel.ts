/** 🦊 Author 46: red-squirrel body, thermal extras, structure and decisions per scale, built bone by bone from the scale's own drawings (author-25-quad.ts builder, hand-set ears, eyes, nose, belly patch, S-curved tail). */
import { openCtx, saveLists, flag, has, drawn, ellipsoidOf, capsuleOf, boxOf, type Json } from "./kit-lib.ts";
import { autoParts, buildStructure, type Cfg } from "./author-25-quad.ts";
import { bboxLocal, part } from "./author-25-lib.ts";
import { centreOnLofts, clampJets, fixPlume, shiftUndrawn } from "./author-46-lib.ts";
import { decisions, type T } from "./author-25-text.ts";

const args = process.argv.slice(2);
const scale = Number(flag(args, "--scale"));
const c = openCtx("animals/red-squirrel", scale);
const SIDE: Record<string, string> = { f: "far", n: "near" };
const name = (bone: string): string | undefined => {
  const m = /^(arm|forearm|hand|thigh|shank|foot)-([fn])$/.exec(bone);
  if (m !== null) return `${SIDE[m[2]!]}-${m[1]}`;
  return ({ "spine-r": "rump", "spine-f": "chest", neck: "neck", head: "head", "tail-1": "tail-1", "tail-2": "tail-2" } as Record<string, string>)[bone];
};
const cfg: Cfg = {
  material: "fur-red",
  name,
  e: (b) => (/^spine/.test(b) ? 2.3 : /^(thigh)/.test(b) ? 2.5 : /^tail/.test(b) ? 2.1 : /^(neck|head)$/.test(b) ? 2.3 : /^(hand|foot)/.test(b) ? 2.2 : 2.4),
  tone: (b) => (/^tail/.test(b) ? 0.45 : /-f$/.test(b) ? 0.5 : /-n$/.test(b) ? 0.35 : b === "spine-r" ? 0.38 : b === "neck" ? 0.38 : 0.4),
  elevOnly: { head: ["head"], "spine-f": ["chest"] },
  elevExclude: /^(belly-patch)$/,
  latExclude: /^(belly-patch|ear|ear-far)$/,
  twin: {},
  bevel: { "spine-r": 0.9, "spine-f": 0.9, neck: 0.9 },
};
const body: Json[] = autoParts(c, cfg).map((p) => {
  const s = part(c, p);
  if (/-(hand|foot)$/.test(p.id)) return { ...s, material: "bare-skin", tone: /far/.test(p.id) ? (/hand/.test(p.id) ? 0.65 : 0.7) : /hand/.test(p.id) ? 0.5 : 0.55 };
  if (/^tail/.test(p.id)) return { ...s, material: "tail-fur" };
  return s;
});
const has2d = (id: string): boolean => drawn(c, "elevation").some((x) => x.id === id);
const eb = has2d("ear") ? bboxLocal(c, "head", "ear") : null;
if (eb !== null) for (const w of [-0.027, 0.027]) body.push(ellipsoidOf({ id: w < 0 ? "near-ear" : "far-ear", bone: "head", material: "fur-red", tone: 0.3 }, [(eb[0] + eb[1]) / 2, (eb[2] + eb[3]) / 2, w], [Math.max(0.0128, (eb[1] - eb[0]) / 2), Math.max(0.0128, (eb[3] - eb[2]) / 2), Math.max(0.0058, 0.8 * c.feature)]));
const hb = has2d("head") ? bboxLocal(c, "head", "head") : null;
if (hb !== null && scale <= 50) for (const w of [-0.022, 0.022]) body.push(ellipsoidOf({ id: w < 0 ? "near-eye" : "far-eye", bone: "head", material: "bare-skin", tone: 0.04, role: "detail" }, [hb[1] - 0.026, hb[2] + 0.64 * (hb[3] - hb[2]), w], [0.0078, 0.0078, 0.0078]));
if (hb !== null) body.push(ellipsoidOf({ id: "nose", bone: "head", material: "bare-skin", tone: 0.12, role: "detail" }, [hb[1] + 0.002, hb[2] + 0.45 * (hb[3] - hb[2]), 0], [Math.max(0.0075, 0.51 * c.feature), Math.max(0.0075, 0.51 * c.feature), Math.max(0.0085, 0.56 * c.feature)]));
const bp = has2d("belly-patch") ? bboxLocal(c, "spine-f", "belly-patch") : null;
if (bp !== null) body.push({ ...ellipsoidOf({ id: "belly-patch", bone: "spine-f", material: "fur-white", tone: 0.05 }, [(bp[0] + bp[1]) / 2, bp[2] + 0.002, 0], [(bp[1] - bp[0]) / 2, 0.0045, 0.02]), texture: true });
const REGION = (s: Json): string | undefined => {
  if (/-eye$/.test(s.id)) return undefined;
  if (s.id === "nose") return "nose";
  if (s.id === "belly-patch") return "fur-belly";
  if (/-ear$/.test(s.id)) return "ears";
  const b: string = s.bone;
  if (/^(spine-r|spine-f|neck)$/.test(b)) return "fur-back";
  if (b === "head") return "head-fur";
  if (/^tail/.test(b)) return "tail-fur";
  if (/^(arm|forearm)-/.test(b)) return "arms-fur";
  if (/^hand-/.test(b)) return "hands-skin";
  if (/^(thigh|shank)-/.test(b)) return "legs-fur";
  if (/^foot-/.test(b)) return "feet-skin";
  return undefined;
};
const R: Record<string, [number, number]> = {
  "spine-r": [0.03, 0.028], "spine-f": [0.028, 0.026], neck: [0.016, 0.014], head: [0.016, 0.01], "tail-1": [0.02, 0.022], "tail-2": [0.02, 0.016],
  arm: [0.008, 0.007], forearm: [0.006, 0.005], hand: [0.004, 0.004], thigh: [0.016, 0.01], shank: [0.008, 0.006], foot: [0.005, 0.004],
};
const structure = (): Json[] => centreOnLofts(c, shiftUndrawn(c, buildStructure(c, { material: "fur-red", radius: (b) => R[b] ?? R[b.replace(/-[fn]$/, "")] ?? [0.006, 0.005], lateral: (b) => (/-f$/.test(b) ? 0.036 : /-n$/.test(b) ? -0.036 : 0) }), { ear: 0.9, tail: 0.9, ...(JSON.parse(process.env.FRAC ?? "{}")) }), /^never$/);
const extras = (): Json[] => {
  const th: Json[] = (c.vol.thermal as Json[]).filter((x) => !/eye/.test(x.id) && !/eye/.test(x.of ?? "")).map((x) => ((x.tags ?? []).includes("breath-nose") ? { ...x, region: "head-fur" } : x));
  const have = new Set(th.map((x) => x.region));
  const out: Json[] = [];
  const bodyAll: Json[] = c.vol.body;
  const own = (id: string): Json | undefined => bodyAll.find((b) => b.id === id);
  if (!have.has("ears")) for (const id of ["near-ear", "far-ear"]) if (own(id) !== undefined) out.push({ id: `th-${id}`, of: id, region: "ears" });
  const bones: Json[] = c.raw.rig.bones;
  const len = (b: string): number => bones.find((x) => x.id === b)!.lengthM;
  const rr = Math.max(0.0085, 0.57 * c.feature);
  const sides = [["n", -0.036], ["f", 0.036]] as const;
  for (const [reg, bone, kind] of [["arms-fur", "arm", "capsule"], ["hands-skin", "hand", "ellipsoid"], ["legs-fur", "thigh", "capsule"], ["feet-skin", "foot", "ellipsoid"]] as const) {
    if (have.has(reg)) continue;
    for (const [s, w] of sides) {
      const b = `${bone}-${s}`;
      const base = { id: `th-${reg}-${s}`, bone: b, material: reg.endsWith("skin") ? "bare-skin" : "fur-red", tone: reg.endsWith("skin") ? 0.55 : 0.4 };
      out.push({ ...(kind === "capsule" ? capsuleOf(base, [rr, 0, w], [len(b) - rr, 0, w], rr, rr) : ellipsoidOf(base, [0.5 * len(b), 0, w], [Math.max(reg === "feet-skin" ? 0.017 : 0.0095, 0.5 * c.feature), rr, rr])), region: reg });
    }
  }
  const all = [...th, ...out];
  clampJets(all, bodyAll);
  fixPlume(c, all, bodyAll, 0.05);
  return all;
};
const OWN: Record<number, T> = {
  50: { en: "Hand-set parts at 1:50: the sitting trunk is a haunch loft along the rump bone and a chest loft along the upright spine bone with the back and belly lines of the elevation, the S-curved tail is two lofts that follow its two bones (14 cm wide, bushy), arms and legs are lofts along their six bones each side with bare-skin hands and feet, the two ears are flat tufted ellipsoids, the eyes are 16 mm balls inside the head sides, the nose is a dark bare-skin ellipsoid at the muzzle tip and the white belly patch is a flush lens skin on the chest front (texture, not in the silhouette).", de: "Von Hand gesetzte Teile bei 1:50: der sitzende Rumpf ist ein Keulenloft entlang der Hüft-Bone und ein Brustloft entlang der aufrechten Wirbelsäulen-Bone mit Rücken- und Bauchlinie des Aufrisses, der S-förmige Schwanz besteht aus zwei Lofts entlang seiner beiden Bones (14 cm breit, buschig), Arme und Beine sind Lofts entlang ihrer sechs Bones je Seite mit Händen und Füßen aus nackter Haut, die beiden Ohren sind flache Pinselellipsoide, die Augen sind 16-mm-Kugeln innerhalb der Kopfseiten, die Nase ist ein dunkles Ellipsoid aus nackter Haut an der Schnauzenspitze und der weiße Bauchfleck ist eine bündige Linsenhaut an der Brustvorderseite (Textur, nicht in der Silhouette)." },
  100: { en: "Hand-set parts at 1:100: arms, hands, legs and feet are not drawn at this scale (they sit in the sitting trunk silhouette), so the model has the rump, chest, neck, head, two ear ellipsoids, nose and the two tail lofts; the limb bones only exist in the structure list.", de: "Von Hand gesetzte Teile bei 1:100: Arme, Hände, Beine und Füße sind in diesem Maßstab nicht gezeichnet (sie liegen in der Silhouette des sitzenden Rumpfes), das Modell besteht daher aus Keule, Brust, Hals, Kopf, zwei Ohrellipsoiden, Nase und den beiden Schwanzlofts; die Gliedmaßen-Bones gibt es nur in der Tragwerkliste." },
};
if (has(args, "--decisions")) saveLists(c, { decisions: decisions(scale as 50, { en: "red squirrel", de: "Eichhörnchen", trunk: { en: "rounded sitting-trunk sections", de: "gerundete Schnitte des sitzenden Rumpfes" }, limb: { en: "round limb sections", de: "runde Gliedmaßenschnitte" }, head: { en: "oval head sections", de: "ovale Kopfschnitte" }, extra: [{ en: "eyes and nose", de: "Augen und Nase" }] }, { body: OWN[scale]! }) });
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
