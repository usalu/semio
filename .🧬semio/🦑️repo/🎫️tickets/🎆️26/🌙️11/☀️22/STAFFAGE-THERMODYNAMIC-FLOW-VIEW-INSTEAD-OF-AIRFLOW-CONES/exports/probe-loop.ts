/** 🔬️ Probe: does the frame after the last of a looping clip equal the first (numerically)? Lists what differs. */
import { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { defaultDepictionOptions, depict } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
import { FLOW_LAYERS } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔗️contracts/🟦️.ts";

const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const adult = catalog.assets.find((a) => a.id === "standing-adult")!;
const selector = { kind: "scale", scale: { denominator: 50 } } as const;
const options = { ...defaultDepictionOptions("thermal", "en", 50, "elevation"), flow: FLOW_LAYERS, flowDevelopS: 14, representation: selector };
const [a, b] = [0, 5].map((timeS) => depict(evaluate(adult, { kind: "clip", clip: "idle-breathing", timeS }, selector), options, {}));
const flowItems = (scene: typeof a) => scene!.layers.flatMap((l) => l.items).filter((i) => i.id.startsWith("flow-")).map((i) => ({ ...i, id: i.id.startsWith("flow-path:") ? "flow-path" : i.id.replace(/^(flow-tracer(?:-head)?):\d+$/u, "$1") })).map((i) => ({ i, key: `${i.id.startsWith("flow-tracer") ? i.id : ""}|${i.kind === "path" && i.id.startsWith("flow-tracer") && i.commands[0]!.op !== "Z" ? `${(i.commands[0]!.x - scene!.metadata.origin.xMm).toFixed(2)}|${(i.commands[0]!.y - scene!.metadata.origin.yMm).toFixed(2)}` : ""}` })).sort((p, q) => (p.key < q.key ? -1 : p.key > q.key ? 1 : 0)).map((e) => e.i);
const A = flowItems(a), B = flowItems(b);
const o = [a!.metadata.origin, b!.metadata.origin];
let worst = 0, paint = 0; const odd: string[] = [];
console.log("flow items", A.length, B.length, "paths", A.filter((i) => i.id === "flow-path").length, B.filter((i) => i.id === "flow-path").length);
A.forEach((x, n) => {
  const y = B[n];
  if (y === undefined || x.id !== y.id || x.kind !== "path" || y.kind !== "path" || x.commands.length !== y.commands.length) { if (odd.length < 8) odd.push(`${x.id}|${y?.id}`); return; }
  x.commands.forEach((c, i) => { const d = y.commands[i]!; if (c.op !== "Z" && d.op !== "Z") worst = Math.max(worst, Math.abs(c.x - o[0]!.xMm - d.x + o[1]!.xMm), Math.abs(c.y - o[0]!.yMm - d.y + o[1]!.yMm)); });
  if (x.stroke?.color !== y.stroke?.color || x.fill?.color !== y.fill?.color) { paint++; if (odd.length < 8) odd.push(`${x.id} ${x.stroke?.color}/${y.stroke?.color} ${x.fill?.color}/${y.fill?.color}`); }
  worst = Math.max(worst, Math.abs((x.stroke?.opacity ?? 1) - (y.stroke?.opacity ?? 1)), Math.abs((x.fill?.opacity ?? 1) - (y.fill?.opacity ?? 1)), Math.abs((x.stroke?.widthMm ?? 0) - (y.stroke?.widthMm ?? 0)));
});
console.log("worst", worst, "paint", paint, odd);
