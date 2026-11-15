/** 🔬️ Union volume band numbers of two scales. Usage: bun author-32-band.ts cat/slug fine coarse [--drop id,id] */
import { bandVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts";
import { entryOf, loadSource, merged, scaleMeshes, featureM } from "./volumes-lib.ts";
const a = process.argv.slice(2);
const [spec, f, c] = a;
const drop = (a.includes("--drop") ? a[a.indexOf("--drop") + 1]! : "").split(",");
const { entry, asset } = entryOf(spec!);
const m = merged(asset, loadSource(entry)!);
const body = (s: string) => ((scaleMeshes(m, Number(s), m.volumes[s]) as any).body as any[]).map((x) => ({ ...x, positions: x.world, bone: x.bone }));
const only = a.includes("--only") ? a[a.indexOf("--only") + 1]!.split(",") : null;
const fine = body(f!).filter((x) => !only || only.includes(x.id)), coarse = body(c!).filter((x) => !drop.includes(x.id) && (!only || only.includes(x.id)));
const feat = featureM(Number(c), asset);
const band = bandVolumes(fine as any, coarse as any, 1 * feat);
console.log(JSON.stringify(band), "excess", Math.max(0, band.eroded - band.coarse, band.coarse - band.dilated) / band.fine, "feature", feat);
