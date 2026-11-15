/** Author 25: labrador-dog body lists per scale from the scale's own drawings (see author-25-quad.ts). */
import { openCtx, saveLists, flag, has, type Json } from "./kit-lib.ts";
import { buildBody, buildStructure, type Cfg } from "./author-25-quad.ts";
import { bboxLocal, bottomAt, part, thirdRadius } from "./author-25-lib.ts";
import { drawn, ellipsoidOf } from "./kit-lib.ts";
import { decisions, type T } from "./author-25-text.ts";

const args = process.argv.slice(2);
const scale = Number(flag(args, "--scale"));
const c = openCtx("animals/labrador-dog", scale);
const SIDE: Record<string, string> = { f: "far", n: "near" };
const BASE: Record<string, string> = { thigh: "thigh", shank: "shank", metatarsus: "hock", scapula: "scapula", humerus: "humerus", forearm: "forearm", metacarpus: "wrist" };
const name = (bone: string): string | undefined => {
  const m = /^(thigh|shank|metatarsus|scapula|humerus|forearm|metacarpus|paw)-([fh])([fn])$/.exec(bone);
  if (m !== null) return `${SIDE[m[3]!]}-${m[1] === "paw" ? (m[2] === "h" ? "hind-paw" : "forepaw") : BASE[m[1]!]}`;
  const ear = /^ear-([fn])$/.exec(bone);
  if (ear !== null) return `${SIDE[ear[1]!]}-ear`;
  const direct: Record<string, string> = { lumbar: "hindbody", thorax: "chest", neck: "neck", head: "head", jaw: "jaw", "tail-1": "tail-root", "tail-2": "tail-mid", "tail-3": "tail-tip" };
  return direct[bone];
};
const cfg: Cfg = {
  material: "short-fur",
  name,
  e: (b) => (/^(lumbar|thorax)$/.test(b) ? 3 : /^(thigh|scapula|humerus)/.test(b) ? 2.8 : /^(tail|ear)/.test(b) ? 2.2 : /^(neck|head|jaw)$/.test(b) ? 2.4 : 2.5),
  tone: (b) => (/^ear-f$|-[fh]f$/.test(b) ? 0.55 : /^ear-n$/.test(b) ? 0.6 : /^tail/.test(b) ? 0.66 : 0.72),
  elevOnly: { head: ["head"], jaw: ["jaw"] },
  latBone: { head: "head-core" },
  twin: { "ear-f": "ear-n", "tail-1": "tail-2", "tail-3": "tail-2" },
  latTaper: { "thigh-hf": { taper: [0.7, 0.95], inward: [0.75, 1] }, "thigh-hn": { taper: [0.7, 0.95], inward: [0.75, 1] } },
  bevel: { lumbar: 0.9, thorax: 0.9, neck: 0.9 },
  ears: { "ear-f": { roll: -22, thick: Math.max(0.013, 0.6 * c.feature), scale: 0.42 }, "ear-n": { roll: 22, thick: Math.max(0.013, 0.6 * c.feature), scale: 0.42 } },
  skip: ["head-core"],
  tolFactor: scale >= 200 ? 0.2 : 0.45,
  tweak: (b, sc, p) => (sc >= 200 && /^tail-[123]$/.test(b) ? { ...p, lat: { ...(p.lat as object), bone: "tail-2" } as never, uRange: b === "tail-1" ? [0, 0.15] : b === "tail-2" ? [0, 0.14] : [0, 0.13] } : p),
};
const body: Json[] = buildBody(c, cfg);
body.push(part(c, { id: "skull", bone: "head-core", material: "short-fur", tone: 0.5, elev: ["head"], elevBone: "head", lat: { view: "plan", bone: "head-core", scale: 0.85 }, vScale: 0.8, e: 2.4 }));
const has2d = (id: string): boolean => drawn(c, "elevation").some((x) => x.id === id);
if (has2d("tongue")) body.push(part(c, { id: "tongue", bone: "jaw", material: "tongue-mucosa", tone: 0.42, elev: ["tongue"], lat: { half: 0.021 }, e: 2.2 }));
const nose = bboxLocal(c, "head", "nose-leather");
if (nose !== null) body.push(ellipsoidOf({ id: "nose-leather", bone: "head", material: "bare-skin", tone: 0.08 }, [nose[1] - 0.017, (nose[2] + nose[3]) / 2, 0], [0.0165, 0.0165, thirdRadius(0.0165, 0.0165, 0.0039)]));
if (has2d("eye")) for (const w of [0.04, -0.04]) body.push(ellipsoidOf({ id: w > 0 ? "far-eye" : "near-eye", bone: "head", material: "bare-skin", tone: 0.04, role: "detail" }, [(bboxLocal(c, "head", "eye")![0] + bboxLocal(c, "head", "eye")![1]) / 2, (bboxLocal(c, "head", "eye")![2] + bboxLocal(c, "head", "eye")![3]) / 2, w], [Math.max(0.0065, 0.6 * c.feature), Math.max(0.0065, 0.6 * c.feature), Math.max(0.0065, 0.6 * c.feature)]));
if (has2d("near-hind-pad")) {
  for (const side of ["n", "f"] as const) {
    for (const hind of [true, false]) {
      const paw = body.find((s) => s.bone === `paw-${hind ? "h" : "f"}${side}`)!;
      const u = hind ? 0.0439 : 0.0365;
      body.push({ ...ellipsoidOf({ id: `${SIDE[side]}-${hind ? "hind" : "fore"}-pad`, bone: paw.bone, material: "bare-skin", tone: 0.12, role: "detail" }, [u, bottomAt(paw, u), (side === "f" ? 1 : -1) * 0.078], [hind ? 0.036 : 0.038, thirdRadius(hind ? 0.036 : 0.038, 0.02, 0.0047, 0.51 * c.feature), 0.02]) });
    }
  }
}
const REGION = (s: Json): string | undefined => {
  if (s.id === "nose-leather") return "nose-skin";
  if (s.id === "tongue") return "tongue";
  if (/-pad$/.test(s.id)) return "pads";
  if (/-eye$/.test(s.id)) return undefined;
  const b: string = s.bone;
  if (/^(lumbar|thorax|neck)$/.test(b)) return "coat-trunk";
  if (/^(head|jaw)$/.test(b)) return "coat-head";
  if (/^ear-/.test(b)) return "ears";
  if (/^(scapula|humerus|forearm|metacarpus|paw-f)/.test(b)) return "coat-forelimbs";
  if (/^(thigh|shank|metatarsus|paw-h)/.test(b)) return "coat-hindlimbs";
  if (/^tail/.test(b)) return "coat-tail";
  return undefined;
};
const R: Record<string, [number, number]> = {
  pelvis: [0.042, 0.04], lumbar: [0.04, 0.042], thorax: [0.048, 0.04], neck: [0.032, 0.028], head: [0.034, 0.016], "head-core": [0.01, 0.01], jaw: [0.016, 0.012], "ear-n": [0.007, 0.006], "ear-f": [0.007, 0.006],
  "tail-1": [0.016, 0.013], "tail-2": [0.013, 0.01], "tail-3": [0.01, 0.007],
  thigh: [0.024, 0.018], shank: [0.016, 0.011], metatarsus: [0.01, 0.008], paw: [0.009, 0.007], scapula: [0.02, 0.018], humerus: [0.02, 0.015], forearm: [0.014, 0.01], metacarpus: [0.009, 0.007],
};
const structure = (): Json[] =>
  buildStructure(c, {
    material: "short-fur",
    radius: (b) => R[b] ?? R[b.replace(/-[fh][fn]$/, "")] ?? [0.01, 0.01],
    lateral: (b) => (/^ear-f$/.test(b) ? 0.07 : /^ear-n$/.test(b) ? -0.07 : /-[fh]f$/.test(b) ? 0.085 : /-[fh]n$/.test(b) ? -0.085 : 0),
  });
const extras = (): Json[] => {
  const nonOwn = (x: Json): boolean => ["ears"].includes(x.region) && x.of !== undefined && !body.some((b) => b.id === x.of);
  const ownBody = new Set((c.vol.body as Json[]).filter((b) => /nose-leather|tongue|-pad$/.test(b.id)).map((b) => b.id));
  const th: Json[] = (c.vol.thermal as Json[]).filter((x) => !["nose-skin", "tongue", "pads"].includes(x.region) || (x.of !== undefined && ownBody.has(x.of)));
  c.vol.thermal = th;
  const have = new Set(th.map((x) => x.region));
  const out: Json[] = [];
  const r = Math.max(0.0153, 0.51 * c.feature);
  const bodyAll: Json[] = c.vol.body;
  if (!have.has("nose-skin")) {
    const hb = bboxLocal(c, "head", drawn(c, "elevation", "head").find((x) => x.role === "body")!.id)!;
    out.push({ ...ellipsoidOf({ id: "th-nose-skin", bone: "head", material: "bare-skin", tone: 0.08 }, [hb[1] - 0.2 * r, (hb[2] + hb[3]) / 2 - 0.25 * (hb[3] - hb[2]) / 2, 0], [r * 1.4, r, r * 1.2]), region: "nose-skin" });
  }
  if (!have.has("ears")) for (const side of ["n", "f"] as const) out.push({ ...ellipsoidOf({ id: `th-ear-${side}`, bone: `ear-${side}`, material: "short-fur", tone: 0.55 }, [0.05, 0, (side === "f" ? 1 : -1) * 0.07], [Math.max(0.05, r * 2), r, r]), region: "ears" });
  if (!have.has("tongue")) out.push({ ...ellipsoidOf({ id: "th-tongue", bone: "jaw", material: "tongue-mucosa", tone: 0.42 }, [0.12 - 0.5 * r, 0.0, 0], [Math.max(0.05, r * 2.5), r, Math.max(0.02, r)]), region: "tongue" });
  if (!have.has("pads")) {
    for (const side of ["n", "f"] as const) for (const hind of [true, false]) {
      const bone = hind ? `metatarsus-h${side}` : `metacarpus-f${side}`;
      const foot = bodyAll.find((x) => x.bone === bone)!;
      const last = (foot.sections as Json[])[(foot.sections as Json[]).length - 1];
      const u = last.at[0] - (c.scale >= 500 ? 0.85 : 0.65) * r;
      out.push({ ...ellipsoidOf({ id: `th-${SIDE[side]}-${hind ? "hind" : "fore"}-pad`, bone, material: "bare-skin", tone: 0.12 }, [u, last.at[1] + 0.02, (side === "f" ? 1 : -1) * 0.078], [r, thirdRadius(r, r, 0.0047, r), r]), region: "pads" });
    }
  }
  return out;
};
const OWN: Record<number, T> = {
  50: { en: "Hand-set parts at 1:50: the ears are flaps rolled 22 degrees out of the sagittal plane (26 mm thick), the hind thighs narrow towards the hip, the four flat pads (whole surface equal to the declared pad area) sit under the toes of each paw, the tongue is 42 mm wide inside the closed mouth, the nose leather is a 33 x 33 x 40 mm ellipsoid at the muzzle tip whose whole surface equals the declared nose skin, the eyes are 13 mm balls, and the skull solid on the head axis bone lies inside the head loft so that every drawn bone has a solid.", de: "Von Hand gesetzte Teile bei 1:50: die Ohren sind um 22 Grad aus der Mittelebene gerollte Lappen (26 mm dick), die Hinterschenkel verjüngen sich zur Hüfte, die vier flachen Ballen (Gesamtoberfläche gleich der angegebenen Ballenfläche) sitzen unter den Zehen jeder Pfote, die Zunge ist 42 mm breit im geschlossenen Maul, das Nasenleder ist ein Ellipsoid von 33 x 33 x 40 mm an der Schnauzenspitze, dessen ganze Oberfläche der angegebenen Nasenhaut entspricht, die Augen sind 13-mm-Kugeln, und der Schädelkörper an der Kopfachsen-Bone liegt im Kopfloft, damit jede gezeichnete Bone einen Körper hat." },
  100: { en: "Hand-set parts at 1:100: ears, nose, tongue and pads as at 1:50 but with 18 mm eyes this scale needs; shoulder, upper arm, forearm and wrist stay separate bones as drawn.", de: "Von Hand gesetzte Teile bei 1:100: Ohren, Nase, Zunge und Ballen wie bei 1:50, aber mit 18-mm-Augen, die dieser Maßstab braucht; Schulter, Oberarm, Unterarm und Handwurzel bleiben getrennte Bones wie gezeichnet." },
  200: { en: "Hand-set parts at 1:200: no jaw tongue, eye or pad solids in the body (not drawn), only the near ear is drawn in elevation so the far ear follows its twin, tail root and tip are cut out of the one drawn tail polygon.", de: "Von Hand gesetzte Teile bei 1:200: keine Zungen-, Augen- oder Ballenkörper im Körper (nicht gezeichnet), im Aufriss ist nur das nahe Ohr gezeichnet, das ferne folgt seinem Zwilling, Rutenwurzel und -spitze sind aus dem einen gezeichneten Rutenpolygon geschnitten." },
  500: { en: "Hand-set parts at 1:500: leg lofts follow the redrawn thigh, leg and foot outlines; legs are 75 mm thick, the tail is cut into three bones from the single tail outline; ears, nose, tongue and pads exist only in the thermal list as 75 mm solids.", de: "Von Hand gesetzte Teile bei 1:500: die Beinlofts folgen den neu gezeichneten Umrissen von Oberschenkel, Unterschenkel und Fuß; die Beine sind 75 mm dick, die Rute ist aus dem einen Rutenumriss in drei Bones geschnitten; Ohren, Nase, Zunge und Ballen gibt es nur als 75-mm-Körper in der Thermikliste." },
};
if (has(args, "--decisions")) saveLists(c, { decisions: decisions(scale as 50, { en: "labrador", de: "Labrador", trunk: { en: "rounded trunk sections", de: "gerundete Rumpfschnitte" }, limb: { en: "round limb sections", de: "runde Gliedmaßenschnitte" }, head: { en: "oval head sections", de: "ovale Kopfschnitte" }, extra: [{ en: "eyes, nose leather, tongue and pads", de: "Augen, Nasenleder, Zunge und Ballen" }] }, { body: OWN[scale]! }) });
else if (has(args, "--extras")) {
  const add = extras();
  saveLists(c, { thermal: [...(c.vol.thermal as Json[]), ...add] });
  console.log(`thermal extras: ${add.map((x) => x.id).join(", ")}`);
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
