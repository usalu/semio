import { readFileSync, writeFileSync } from "node:fs";

let s = readFileSync("spatial/js/core/index.ts", "utf8");

s = s.replaceAll("return { diff: boxTopologyDiff(", "return { diff: ctx.preview.boxTopologyDiff(");
s = s.replaceAll("Math.min(", "ctx.preview.min2(");
s = s.replaceAll("Math.max(", "ctx.preview.max2(");
s = s.replaceAll("Math.abs(", "ctx.preview.abs(");
s = s.replaceAll("Math.hypot(", "ctx.preview.vec3Length(ctx.preview.vec3Sub(");

// Fix broken hypot replacement - revert and do properly
s = readFileSync("spatial/js/core/index.ts", "utf8");
s = s.replace("return { diff: boxTopologyDiff(", "return { diff: ctx.preview.boxTopologyDiff(");

const patchRun = (id: string, prelude: string) => {
	const needle = `id: "${id}",\n\t\trun: (params) => {`;
	if (!s.includes(needle)) return;
	s = s.replace(needle, `id: "${id}",\n\t\trun: (params, ctx) => {\n\t\t\t${prelude}`);
};

patchRun("box.aabbFromDiagonalCorners", "const pr = ctx.preview;");
patchRun("box.tripletRubber", "const pr = ctx.preview;");
patchRun("box.tripletCommit", "const pr = ctx.preview;");
patchRun("box.snapSquareFootprint", "const pr = ctx.preview;");
patchRun("box.setCubeHeightFromFootprint", "const pr = ctx.preview;");
patchRun("box.rubberCornerFromCenter", "const pr = ctx.preview;");
patchRun("box.rubberSquareFromCenter", "const pr = ctx.preview;");
patchRun("box.verticalFinalizeFootprint", "const pr = ctx.preview;");
patchRun("box.initPeakAboveOrigin", "const pr = ctx.preview;");
patchRun("box.peakFromOriginZ", "const pr = ctx.preview;");
patchRun("box.verticalRubberCorner", "const pr = ctx.preview;");
patchRun("box.cornerFromLengthWidth", "const pr = ctx.preview;");

s = s.replaceAll("Math.min(", "pr.min2(");
s = s.replaceAll("Math.max(", "pr.max2(");
s = s.replaceAll("Math.abs(", "pr.abs(");

// min2 with 3 args - fix triplet cases manually
s = s.replaceAll("pr.min2(p0[0], p1[0], P[0])", "pr.minN([p0[0], p1[0], P[0]])");
s = s.replaceAll("pr.min2(p0[1], p1[1], P[1])", "pr.minN([p0[1], p1[1], P[1]])");
s = s.replaceAll("pr.max2(p0[0], p1[0], P[0])", "pr.maxN([p0[0], p1[0], P[0]])");
s = s.replaceAll("pr.max2(p0[1], p1[1], P[1])", "pr.maxN([p0[1], p1[1], P[1]])");

writeFileSync("spatial/js/core/index.ts", s);
console.log("actions patched");
