/** 🔬️ Probe: how the flow of a still pose and of a looping clip changes between frames (streams, tracers), with and without turbulence. */
import { loadCatalogFromFiles, DEFAULT_CATALOG_ROOT } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { defaultDepictionOptions, depict, transportOf } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
import { FLOW_LAYERS } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔗️contracts/🟦️.ts";

const catalog = await loadCatalogFromFiles(DEFAULT_CATALOG_ROOT);
const kettle = catalog.assets.find((a) => a.id === "electric-kettle")!;
const adult = catalog.assets.find((a) => a.id === "standing-adult")!;
const base = (scale: number) => ({ ...defaultDepictionOptions("thermal", "en", scale, "elevation"), flow: FLOW_LAYERS, flowDevelopS: 14 });
const sig = (f: ReturnType<typeof transportOf>) => JSON.stringify(f.flow!.streams.map((s) => s.points.map((p) => [p.x.toFixed(6), p.y.toFixed(6), p.z.toFixed(6)])));
for (const calm of [false, true]) {
  const options = { ...base(20), ...(calm ? { flowConditions: { turbulence: { intensity: 0, floorMS: 0, lagrangianTimeS: 1 } } } : {}) };
  const a = transportOf({ asset: kettle, query: { kind: "pose", pose: "boiling", timeS: 0 } }, options);
  const b = transportOf({ asset: kettle, query: { kind: "pose", pose: "boiling", timeS: 0.5 } }, options);
  console.log("kettle calm", calm, "t", a.flow!.timeS, b.flow!.timeS, "streams", a.flow!.streams.length, b.flow!.streams.length, "same streams", sig(a) === sig(b), "puffs", a.flow!.puffs.length, b.flow!.puffs.length);
}
const options = base(50);
const q = (timeS: number) => ({ kind: "clip", clip: "idle-breathing", timeS }) as const;
const selector = { kind: "scale", scale: { denominator: 50 } } as const;
const t0 = performance.now();
const scenes = [0, 2.5, 5].map((t) => depict(evaluate(adult, q(t), selector), { ...options, representation: selector }, {}));
console.log("adult ms", Math.round(performance.now() - t0));
const items = (i: number) => scenes[i]!.layers.flatMap((l) => l.items);
const flowOf = (i: number) => JSON.stringify(items(i).filter((x) => x.id.startsWith("flow-")));
const kinds = (i: number, p: string) => JSON.stringify(items(i).filter((x) => x.id.startsWith(p)));
console.log("loop closes", flowOf(0) === flowOf(2), "mid differs", flowOf(0) !== flowOf(1), "sizes", scenes.map((s) => [s.widthMm, s.heightMm, s.metadata.origin.xMm, s.metadata.origin.yMm].map((v) => v.toFixed(3)).join("/")));
for (const p of ["flow-stream", "flow-arrow", "flow-tracer", "flow-vapour", "flow-fog", "flow-path"]) console.log(p, kinds(0, p) === kinds(2, p), items(0).filter((x) => x.id.startsWith(p)).length, items(2).filter((x) => x.id.startsWith(p)).length);
const ta = transportOf({ asset: adult, query: q(0) }, options), tb = transportOf({ asset: adult, query: q(5) }, options);
console.log("field times", ta.flow!.timeS, tb.flow!.timeS, "same streams", sig(ta) === sig(tb));
