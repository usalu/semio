/** 🗒️ Per-scale decision texts: the base figure's decisions rewritten for the new person (replacements in both languages), extended by the prop and limb sentences of each scale. */
import { SCALES, type Any, type Scale, type Text } from "./lib.ts";

export type Edits = {
  /** 🔁️ Pairs of literal replacements applied to every decision text, English and German. */
  replace: readonly (readonly [string, string])[];
  /** ➕️ Sentences appended per scale and representation field. */
  append: Partial<Record<Scale, Partial<Record<string, Text>>>>;
  /** 🧊️ Sentences appended to the volume decisions per scale (body, thermal, structure in order). */
  volume: Partial<Record<Scale, readonly Text[]>>;
  /** ✍️ Fields replaced outright per scale. */
  set?: Partial<Record<Scale, Partial<Record<string, Text>>>>;
};

/** 🖋️ Applies the edits to every representation and volume of the asset. */
export function applyDecisions(asset: Any, edits: Edits): void {
  const swap = (t: Text): Text => {
    let { en, de } = t;
    for (const [a, b] of edits.replace) { en = en.split(a).join(b); de = de.split(a).join(b); }
    return { en, de };
  };
  for (const scale of SCALES) {
    const rep = asset.representations[scale];
    if (rep.status === "authored") {
      for (const field of Object.keys(rep.decisions)) rep.decisions[field] = swap(rep.decisions[field]);
      for (const [field, text] of Object.entries(edits.set?.[scale] ?? {})) rep.decisions[field] = text;
      for (const [field, text] of Object.entries(edits.append[scale] ?? {})) {
        const old = rep.decisions[field] ?? { en: "", de: "" };
        rep.decisions[field] = { en: `${old.en} ${text!.en}`.trim(), de: `${old.de} ${text!.de}`.trim() };
      }
    }
    const vol = asset.volumes[scale];
    if (vol.status !== "authored") continue;
    vol.decisions = vol.decisions.map(swap);
    (edits.volume[scale] ?? []).forEach((text, i) => {
      if (i < vol.decisions.length) vol.decisions[i] = { en: `${vol.decisions[i].en} ${text.en}`, de: `${vol.decisions[i].de} ${text.de}` };
      else vol.decisions.push(text);
    });
  }
}
