/** 📏️ Round 3 tram: the product's silhouette numbers of the candidate document `🗑️generated/r3-tram/🔣️.json` with some 2D shapes or body solids left out, to find what moves a number (`bun r3-tram-measure.ts 50 elevation [-shape id,...] [-solid id-prefix,...]`). */
import { readFileSync } from "node:fs";
import { join } from "node:path";

import * as volumes from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { GENERATED, type Json } from "./volumes-lib.ts";

const [scale, view] = [Number(process.argv[2] ?? 50), process.argv[3] ?? "elevation"];
const flag = (name: string): string[] => (process.argv.indexOf(name) < 0 ? [] : process.argv[process.argv.indexOf(name) + 1]!.split(","));
const asset = JSON.parse(readFileSync(join(GENERATED, "r3-tram", "🔣️.json"), "utf8")).asset as Json;
const shapes = flag("-shape");
const solids = flag("-solid");
const rep = asset.representations[String(scale)].geometry;
rep[view] = rep[view].filter((s: Json) => !shapes.includes(s.id));
const v = asset.volumes[String(scale)];
v.body = v.body.filter((s: Json) => !solids.some((p) => String(s.id).startsWith(p)));
const list = (volumes as Json).measureVolumes(asset, { scale, rules: ["volumes-silhouette"] }) as Json[];
for (const m of list) if (String(m.name).endsWith(view)) console.log(`[DEBUG] ${m.name}=${Math.round(m.value * 1e4) / 1e4}`);
