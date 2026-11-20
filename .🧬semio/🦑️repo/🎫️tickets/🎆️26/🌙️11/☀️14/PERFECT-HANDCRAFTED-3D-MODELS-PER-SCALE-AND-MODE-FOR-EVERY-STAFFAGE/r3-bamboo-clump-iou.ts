/** 🧪️ Silhouette IoU of the bamboo clump at one scale with groups of parts removed on both sides (2D shapes and 3D body solids), to find what costs the IoU. Usage: `bun r3-bamboo-clump-iou.ts <scale> <variant: all|no-leaves|no-branches|culms>` */
import { measureVolumes } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { load } from "./r3-lib.ts";

const { asset } = load("vegetation/bamboo-clump", "r3-bamboo-clump-iou.ts <scale> <variant>");
const scale = String(process.argv[2] ?? 50);
const variant = process.argv[3] ?? "all";
const isLeaf = (id: string) => /-leaves-|-l\d+$|apex\d+-l\d+$/.test(id);
const isBranch = (id: string) => /-branches$|^b\d+-\d+[ps](-t\d+)?$/.test(id);
const drop = (id: string) => (variant === "no-leaves" && isLeaf(id)) || (variant === "no-branches" && isBranch(id)) || (variant === "culms" && (isLeaf(id) || isBranch(id)));
const rep = asset.representations[scale];
for (const v of ["elevation", "front", "plan"]) rep.geometry[v] = rep.geometry[v].filter((s: { id: string }) => !drop(s.id));
const vol = asset.volumes[scale];
vol.body = vol.body.filter((s: { id: string }) => !drop(s.id));
const out = measureVolumes(asset as never, { scale: Number(scale) as never, rules: ["volumes-silhouette"] as never } as never, { signal: { aborted: false, throwIfAborted() {} }, progress: () => {} } as never) as { name: string; value: number }[];
console.log(variant, out.filter((m) => /^iou|^p95M/.test(m.name)).map((m) => `${m.name}=${m.value.toFixed(3)}`).join(" "));
