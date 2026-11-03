import { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { defaultDepictionOptions } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
import { resolveSheet } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/📑️sheet/🟦️.ts";
const c = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const a = c.assets.find((x) => x.id === "skateboarder")!;
for (const d of [100, 200, 500]) {
  const { view, viewpoint, scale, representation, ...rest } = defaultDepictionOptions("dark-line", "en", d);
  const { sheet } = resolveSheet(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", denominator: d }, "dark-line", rest as never, "third", "en");
  const items = sheet.layers.flatMap((l) => l.layers ?? []).concat(sheet.layers.flatMap((l) => l.items.map((i) => ({ layer: l.id, i })) as never));
  const xs = (pred: (id: string, layer: string) => boolean) => sheet.layers.flatMap((l) => l.items.filter((i) => i.kind === "path" && pred(i.id, l.id)).flatMap((i: any) => i.commands.filter((c: any) => "x" in c).map((c: any) => c.x as number)));
  const g = xs((id) => id.startsWith("front:ground"));
  const f = xs((id, l) => id.startsWith("front:") && !["ground", "annotations"].includes(l));
  console.log(d, "ground", Math.min(...g).toFixed(3), Math.max(...g).toFixed(3), "figure", Math.min(...f).toFixed(3), Math.max(...f).toFixed(3), "k", sheet.metadata.mmPerMetre);
}
