import { specificOsDevBuildInventoryV1 } from "/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/🧩️catalog/📦️packages/🟦️typescript/🏗️builder/🟦️.ts";
const inv = specificOsDevBuildInventoryV1();
console.log(inv.defaultVariant, inv.playgrounds.map((p: any) => `${p.variant}|${p.pluginId}|${(p.aliases ?? []).join(",")}`).join("\n"));
