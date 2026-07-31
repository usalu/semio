import { BrepjsGeometryKernel, ensureBrepWasmLoaded } from "../../../../../../geometry/brep/js/index.ts";
import { evaluateBrepFlowKind } from "../../../../../../procedural/3d/react/index.tsx";

await ensureBrepWasmLoaded();
const kernel = new BrepjsGeometryKernel();

const line = evaluateBrepFlowKind("brep.curve.line", JSON.stringify({ start: [0, 0, 0], end: [4, 0, 0] }), kernel);
const lineGeo = JSON.parse(line).geometry as string;
const divide = evaluateBrepFlowKind("brep.eval.divideCurve", JSON.stringify({ geometry: lineGeo, count: 3 }), kernel);
const divideParsed = JSON.parse(divide) as { list?: Record<string, { x: number }> };
console.log("[DEBUG] divideCurve", divideParsed.list?.["0"]?.x, divideParsed.list?.["2"]?.x);

const reparam = evaluateBrepFlowKind("brep.curve.reparametrize", JSON.stringify({ geometry: lineGeo, samples: 8 }), kernel);
console.log("[DEBUG] reparametrize", JSON.parse(reparam).geometry);

const randomWasm = await import("../../../../../../flow/modules/math/pkg/flow_module_math.js");
await randomWasm.default?.();
const seeded = randomWasm.evaluate("math.random", JSON.stringify({ seed: 42, min: 0, max: 1 }));
const seeded2 = randomWasm.evaluate("math.random", JSON.stringify({ seed: 42, min: 0, max: 1 }));
console.log("[DEBUG] math.random seeded", seeded, seeded === seeded2);

const range = (await import("../../../../../../flow/modules/list/pkg/flow_module_list.js")).evaluate("list.range", JSON.stringify({ start: 0, step: 2, count: 3 }));
console.log("[DEBUG] list.range", range);
