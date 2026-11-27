import { loadAsset, drawOptions, subjects } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/🧰️.ts";
import { resolveViews } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔭️multiview/🟦️.ts";
const asset = loadAsset("used-bentwood-dining-chairs")!;
const subject = subjects(asset, 1000).find((s) => s.id === "pose in-use")!;
const raw = new Map<string, Map<string, string>>();
for (const language of subject.languages)
  for (const mode of subject.modes) {
    const multi = resolveViews(asset, subject.query, { kind: "scale", denominator: 1000 }, mode, drawOptions(mode, 1000, false, language));
    for (const [view, d] of Object.entries(multi.views) as [string, any][]) {
      if (d.kind === "unavailable") continue;
      const m = new Map<string, string>();
      for (const layer of d.scene.layers) for (const item of layer.items) if (item.kind === "path" && item.id.startsWith("shape:")) m.set(item.id, JSON.stringify(item.commands));
      raw.set(`${language}|${view}|${mode}`, m);
      if (mode === "forces") console.log("[DEBUG] meta", language, view, JSON.stringify(d.scene.metadata.origin), d.scene.widthMm);
    }
  }
console.log("[DEBUG] languages", subject.languages, subject.modes);
for (const [key, en] of raw) {
  if (!key.startsWith("en|")) continue;
  const de = raw.get(key.replace("en|", "de|"))!;
  for (const k of new Set([...en.keys(), ...de.keys()])) if (en.get(k) !== de.get(k)) console.log("[DEBUG] diff", key, k, en.get(k)?.slice(0, 300), "|||", de.get(k)?.slice(0, 300));
}
