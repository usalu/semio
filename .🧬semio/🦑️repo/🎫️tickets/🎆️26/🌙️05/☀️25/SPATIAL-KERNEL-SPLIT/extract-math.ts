import { readFileSync, writeFileSync } from "node:fs";

const lines = readFileSync("spatial/js/core/index.ts", "utf8").split(/\r?\n/);
const sl = (a: number, b: number) => lines.slice(a - 1, b).join("\n");
const header = `/** @emoji 🧮️ Precise spatial math for \`SpatialPreviewKernel\` / \`SpatialKernel\`. */
import type {
	ArcPlaneFrame,
	AnchorAttachment,
	AnchorRecord,
	CellRecord,
	CellRef,
	CellSolid,
	EdgeCurve,
	EdgeRecord,
	FaceRecord,
	FaceRef,
	MeshPreview,
	PartRef,
	PartView,
	ShellRef,
	SurfaceRef,
	SurfaceView,
	TopologyDiff,
	TopologyGraph,
	VertexRecord,
	VertexRef,
	WireRecord,
	WireRef,
	Vec3,
} from "@spatial/js-core";

`;
const body = [sl(47, 357), sl(1438, 1707), sl(1710, 1838), sl(2771, 2877), sl(2664, 3236)].join("\n\n");
writeFileSync("spatial/js/kernel-brepjs/spatial-kernel-math.ts", header + body);
console.log("wrote", (header + body).split("\n").length, "lines");
