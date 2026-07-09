import { readFileSync, writeFileSync } from "node:fs";

let s = readFileSync("spatial/js/core/index.ts", "utf8");

if (!s.includes("@spatial/js-kernel-brepjs")) {
  s = s.replace(
    "if (import.meta.vitest) {",
    `if (import.meta.vitest) {
	import { BrepjsKernel, computePartViewsFromTopology, computeSurfaceViewsFromTopology, preciseSpatialKernelMath } from "@spatial/js-kernel-brepjs";
	const M = preciseSpatialKernelMath;`,
  );
}

s = s.replaceAll("vec3Distance(", "M.vec3Distance(");
s = s.replaceAll("boxTopologyDiff(", "M.boxTopologyDiff(");
s = s.replaceAll("edgeSamplePoints(", "M.edgeSamplePoints(");
s = s.replaceAll("arcEndOnCircle(", "M.arcEndOnCircle(");
s = s.replaceAll("implements SpatialKernel", "implements Partial<SpatialKernel>");
s = s.replaceAll("class AnchorKernel implements Partial<SpatialKernel>", "class AnchorKernel extends BrepjsKernel");
s = s.replaceAll("class CommandKernel implements Partial<SpatialKernel>", "class CommandKernel extends BrepjsKernel");
s = s.replaceAll("class ArcKernel implements Partial<SpatialKernel>", "class ArcKernel extends BrepjsKernel");
s = s.replaceAll("class StubKernel implements Partial<SpatialKernel>", "class StubKernel extends BrepjsKernel");
s = s.replaceAll("class MeasKernel implements Partial<SpatialKernel>", "class MeasKernel extends BrepjsKernel");
s = s.replaceAll("class AreaKernel implements Partial<SpatialKernel>", "class AreaKernel extends BrepjsKernel");
s = s.replaceAll("class RecordingStubKernel implements Partial<SpatialKernel>", "class RecordingStubKernel extends BrepjsKernel");

s = s.replace(
  `const DEFAULT_KERNEL: SpatialKernel = {
			id: "default-test",
			operations: [] as const,
			async createBoxFromCorners() {
				return cellRef("c");
			},
			async volume() {
				return 0;
			},
			async tessellate() {
				return { positions: new Float32Array(), indices: new Uint32Array() };
			},
		};`,
  "const DEFAULT_KERNEL = new BrepjsKernel();",
);

writeFileSync("spatial/js/core/index.ts", s);
writeFileSync("spatial/js/core/package.json", readFileSync("spatial/js/core/package.json", "utf8").replace('"devDependencies": {', '"devDependencies": {\n    "@spatial/js-kernel-brepjs": "workspace:*",'));
console.log("tests patched");
