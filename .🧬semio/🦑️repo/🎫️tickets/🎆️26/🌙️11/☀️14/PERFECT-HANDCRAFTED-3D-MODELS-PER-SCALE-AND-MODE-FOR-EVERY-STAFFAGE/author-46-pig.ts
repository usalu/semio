/** 🐖 Author 46: pig body, thermal extras, structure and decisions per scale, built bone by bone from the scale's own drawings (author-25-quad.ts builder, hand-set cloven hooves, snout disc, eyes, curled tail). */
import { openCtx, saveLists, flag, has, drawn, ellipsoidOf, capsuleOf, boxOf, type Json } from "./kit-lib.ts";
import { autoParts, buildStructure, type Cfg } from "./author-25-quad.ts";
import { bboxLocal, part } from "./author-25-lib.ts";
import { centreOnLofts, clampJets, fixPlume, shiftUndrawn } from "./author-46-lib.ts";
import { decisions, type T } from "./author-25-text.ts";

const args = process.argv.slice(2);
const scale = Number(flag(args, "--scale"));
const c = openCtx("animals/pig", scale);
const SIDE: Record<string, string> = { f: "far", n: "near" };
const BASE: Record<string, string> = { thigh: "ham", shank: "gaskin", metatarsus: "hind-cannon", hoof: "hoof", scapula: "shoulder", humerus: "humerus", forearm: "forearm", metacarpus: "fore-cannon" };
const name = (bone: string): string | undefined => {
  const m = /^(thigh|shank|metatarsus|hoof|scapula|humerus|forearm|metacarpus)-([fh])([fn])$/.exec(bone);
  if (m !== null) return `${SIDE[m[3]!]}-${m[1] === "hoof" ? (m[2] === "h" ? "hind-hoof" : "fore-hoof") : BASE[m[1]!]}`;
  const ear = /^ear-([fn])$/.exec(bone);
  if (ear !== null) return `${SIDE[ear[1]!]}-ear`;
  return ({ lumbar: "hindbody", thorax: "chest", neck: "neck", head: "head", jaw: "jaw", "tail-1": "tail-root", "tail-2": "tail-curl" } as Record<string, string>)[bone];
};
const cfg: Cfg = {
  material: "pig-skin",
  name,
  e: (b) => (/^(lumbar|thorax)$/.test(b) ? 2.2 : /^(thigh|scapula|humerus)/.test(b) ? 2.7 : /^(tail|ear)/.test(b) ? 2.2 : /^(neck|head|jaw)$/.test(b) ? 2.2 : /^hoof/.test(b) ? 2.2 : 2.4),
  tone: (b) => (/^ear-n$/.test(b) ? 0.45 : /^hoof/.test(b) ? 0.2 : /-[fh]f$|^ear-f$/.test(b) ? 0.38 : 0.52),
  elevOnly: { head: ["head"] },
  latBone: {},
  twin: { "ear-f": "ear-n" },
  latTaper: { "thigh-hf": { taper: [0.75, 0.95], inward: [0.8, 1] }, "thigh-hn": { taper: [0.75, 0.95], inward: [0.8, 1] } },
  bevel: { lumbar: 0.92, thorax: 0.92, neck: 0.92 },
  tweak: (b, _sc, p) => (b === "tail-1" ? { ...p, lat: { half: Math.max(0.016, 0.51 * c.feature) } } : p),
  ears: { "ear-f": { roll: -35, thick: Math.max(0.0042, 0.56 * c.feature), scale: 0.55 }, "ear-n": { roll: 35, thick: Math.max(0.0042, 0.56 * c.feature), scale: 0.55 } },
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
const sn = has2d("snout") ? bboxLocal(c, "head", "snout") : null;
if (sn !== null) body.push(ellipsoidOf({ id: "snout", bone: "head", material: "pig-skin", tone: 0.8 }, [sn[1] - 0.019, (sn[2] + sn[3]) / 2, 0], [0.024, (sn[3] - sn[2]) / 2, 0.048]));
const eb = has2d("eye") ? bboxLocal(c, "head", "eye") : null;
if (eb !== null) for (const w of [0.074, -0.074]) body.push(ellipsoidOf({ id: w > 0 ? "far-eye" : "near-eye", bone: "head", material: "pig-skin", tone: 0.04, role: "detail" }, [(eb[0] + eb[1]) / 2, (eb[2] + eb[3]) / 2, w], [0.0075, 0.0075, 0.0075]));
const REGION = (s: Json): string | undefined => {
  if (s.id === "snout") return "snout";
  if (/-eye$/.test(s.id)) return undefined;
  const b: string = s.bone;
  if (/^(lumbar|thorax|neck|pelvis)$/.test(b)) return "skin-trunk";
  if (/^(head|jaw)$/.test(b)) return "skin-head";
  if (/^ear-/.test(b)) return "ears";
  if (/^tail/.test(b)) return "tail";
  if (/^hoof/.test(b)) return "hooves";
  if (/^(scapula|humerus|forearm|metacarpus|thigh|shank|metatarsus)/.test(b)) return "skin-legs";
  return undefined;
};
const R: Record<string, [number, number]> = {
  pelvis: [0.06, 0.055], lumbar: [0.055, 0.055], thorax: [0.065, 0.06], neck: [0.06, 0.05], head: [0.045, 0.025], jaw: [0.012, 0.01], "ear-n": [0.004, 0.004], "ear-f": [0.004, 0.004],
  "tail-1": [0.008, 0.006], "tail-2": [0.007, 0.006],
  thigh: [0.045, 0.03], shank: [0.022, 0.016], metatarsus: [0.014, 0.011], hoof: [0.012, 0.01], scapula: [0.035, 0.03], humerus: [0.03, 0.022], forearm: [0.018, 0.014], metacarpus: [0.013, 0.011],
};
const structure = (): Json[] => centreOnLofts(c, shiftUndrawn(c, buildStructure(c, { material: "pig-skin", radius: (b) => R[b] ?? R[b.replace(/-[fh][fn]$/, "")] ?? [0.01, 0.01], length: { "ear-n": 0.7, "ear-f": 0.7 }, lateral: (b) => (/^ear-f$/.test(b) ? (scale >= 500 ? 0 : 0.1) : /^ear-n$/.test(b) ? (scale >= 500 ? 0 : -0.1) : /-hf$/.test(b) ? 0.13 : /-hn$/.test(b) ? -0.13 : /-ff$/.test(b) ? 0.1 : /-fn$/.test(b) ? -0.1 : 0) }), { ear: 0.2, tail: 0.9 }));
const extras = (): Json[] => {
  const ownBody = new Set((c.vol.body as Json[]).filter((b) => b.id === "snout").map((b) => b.id));
  const th: Json[] = (c.vol.thermal as Json[]).filter((x) => !/-eye$/.test(x.of ?? "")).filter((x) => !["snout"].includes(x.region) || (x.of !== undefined && ownBody.has(x.of)));
  const have = new Set(th.map((x) => x.region));
  const out: Json[] = [];
  const r = Math.max(0.0153, 0.51 * c.feature);
  const bodyAll: Json[] = c.vol.body;
  if (!have.has("snout")) {
    const hb = bboxLocal(c, "head", drawn(c, "elevation", "head").find((x) => x.role === "body")!.id)!;
    out.push({ ...ellipsoidOf({ id: "th-snout", bone: "head", material: "pig-skin", tone: 0.8 }, [hb[1] - 0.5 * r, (hb[2] + hb[3]) / 2 - 0.2 * (hb[3] - hb[2]) / 2, 0], [r * 1.2, Math.max(r, 0.03), Math.max(r, 0.045)]), region: "snout" });
  }
  if (!have.has("ears")) {
    if (scale >= 500) out.push({ ...boxOf({ id: "th-ear-span", bone: "ear-n", material: "pig-skin", tone: 0.45 }, [0.08, 0, 0], [0.0765, 0.0765, 0.2]), region: "ears" });
    else for (const side of ["n", "f"] as const) out.push({ ...ellipsoidOf({ id: `th-ear-${side}`, bone: `ear-${side}`, material: "pig-skin", tone: 0.45 }, [0.08, 0, (side === "f" ? 1 : -1) * 0.1], [Math.max(0.08, 2 * r), r, r]), region: "ears" });
  }
  if (!have.has("hooves")) for (const side of ["n", "f"] as const) for (const hind of [true, false]) {
    const bone = hind ? `metatarsus-h${side}` : `metacarpus-f${side}`;
    const foot = bodyAll.find((x) => x.bone === bone)!;
    const last = (foot.sections as Json[])[(foot.sections as Json[]).length - 1];
    out.push({ ...(scale >= 500 ? boxOf({ id: `th-hoof-${hind ? "h" : "f"}${side}`, bone, material: "horn", tone: 0.2 }, [last.at[0] - 0.4 * r, last.at[1], last.at[2]], [0.0765, 0.0765, 0.0765]) : capsuleOf({ id: `th-hoof-${hind ? "h" : "f"}${side}`, bone, material: "horn", tone: 0.2 }, [last.at[0] - 1.3 * r, last.at[1], last.at[2]], [last.at[0] - 0.5 * r, last.at[1], last.at[2]], r, r)), region: "hooves" });
  }
  if (!have.has("tail")) out.push({ ...ellipsoidOf({ id: "th-tail", bone: "tail-1", material: "pig-skin", tone: 0.5 }, [0.04, 0, 0], [0.05, r, r]), region: "tail" });
  const all = [...th, ...out];
  clampJets(all, bodyAll);
  fixPlume(c, all, bodyAll, 0.2);
  return all;
};
const OWN: Record<number, T> = {
  50: { en: "Hand-set parts at 1:50: every hoof is cloven into two toes (two lofts per hoof, each about 37 mm wide), the ears are flaps rolled 35 degrees out of the sagittal plane, the snout is a flattened disc of bare skin at the tip of the head loft, the eyes are 15 mm balls in the head sides, the hams narrow towards the hip, the curled tail is its own loft cut from the drawn curl, and the trunk sections are rounded barrels with a spine ridge and a belly line from the elevation.", de: "Von Hand gesetzte Teile bei 1:50: jeder Huf ist in zwei Zehen gespalten (zwei Lofts je Huf, je etwa 37 mm breit), die Ohren sind um 24 Grad aus der Mittelebene gerollte Lappen, der Rüssel ist eine abgeflachte Scheibe nackter Haut an der Spitze des Kopflofts, die Augen sind 15-mm-Kugeln in den Kopfseiten, die Schinken verjüngen sich zur Hüfte, der Ringelschwanz ist ein eigener Loft aus der gezeichneten Locke, und die Rumpfschnitte sind runde Fässer mit Rückenlinie und Bauchlinie aus dem Aufriss." },
  100: { en: "Hand-set parts at 1:100: hooves are single lofts (the cleft is below 15 mm), ears, snout disc and eyes as at 1:50 with this scale's minimum; shoulder, humerus and forearm stay separate bones as drawn.", de: "Von Hand gesetzte Teile bei 1:100: die Hufe sind einzelne Lofts (die Spalte liegt unter 15 mm), Ohren, Rüsselscheibe und Augen wie bei 1:50 mit dem Minimum dieses Maßstabs; Schulter, Oberarm und Unterarm bleiben getrennte Bones wie gezeichnet." },
  200: { en: "Hand-set parts at 1:200: one loft per drawn bone, the trunk masses follow the merged 1:200 hindbody, chest and neck outlines; the snout and ears have their own thermal solids where the body has none.", de: "Von Hand gesetzte Teile bei 1:200: ein Loft je gezeichneter Bone, die Rumpfmassen folgen den zusammengefassten Umrissen von Hinterkörper, Brust und Hals bei 1:200; Rüssel und Ohren haben eigene Thermikkörper, wo der Körper keine hat." },
  500: { en: "Hand-set parts at 1:500: the elevation of this scale drew no legs and no far limbs, so the four near legs and the four far legs were redrawn at the root as closed outlines from the 1:200 shank, cannon and hoof outlines (see the report); legs are at least 75 mm thick, trunk, neck and head are single masses; ears, snout and hooves exist only in the thermal list.", de: "Von Hand gesetzte Teile bei 1:500: der Aufriss dieses Maßstabs zeichnete weder die vier Beine noch die fernen Gliedmaßen, daher wurden die vier nahen und die vier fernen Beine an der Wurzel als geschlossene Umrisse aus den Umrissen von Unterschenkel, Mittelfuß und Huf bei 1:200 neu gezeichnet (siehe Bericht); die Beine sind mindestens 75 mm dick, Rumpf, Hals und Kopf sind einzelne Massen; Ohren, Rüssel und Hufe gibt es nur in der Thermikliste." },
};
if (has(args, "--decisions")) saveLists(c, { decisions: decisions(scale as 50, { en: "pig", de: "Schwein", trunk: { en: "rounded barrel sections", de: "gerundete Fassschnitte" }, limb: { en: "round limb sections", de: "runde Gliedmaßenschnitte" }, head: { en: "oval head sections", de: "ovale Kopfschnitte" }, extra: [{ en: "cloven hooves and eyes", de: "gespaltene Hufe und Augen" }] }, { body: OWN[scale]! }) });
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
