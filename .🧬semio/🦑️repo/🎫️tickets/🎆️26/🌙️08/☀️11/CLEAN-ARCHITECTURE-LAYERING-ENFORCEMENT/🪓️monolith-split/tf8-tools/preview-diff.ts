const E = "/Users/ueli/Documents/semio/✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine";
const G = process.argv[2]!;
const oldM: any = await import(`${G}/tf8-diff/prev-old.ts`);
const newM: any = { ...(await import(`${E}/🧮️preview/🟦️.ts`)) };
for (const child of ["🔷️brep-records", "📏️vec3", "🌀️curve-sampling", "🔺️face-geometry", "📦️aabb", "⚓️anchor-placement", "🧊️preview-model-diff", "🎚️preview-params"]) {
  try { Object.assign(newM, await import(`${E}/🧮️preview/${child}/🟦️.ts`)); } catch (e) { if (!String((e as Error).message).includes("Cannot find module")) throw e; }
}
const spatial: any = await import(`${E}/🗺️spatial/🟦️.ts`);
const modelMod: any = await import(`${E}/📐️geometry/🧊️model/🟦️.ts`);
let seed = 123456789;
const rnd = () => { seed = (seed * 1664525 + 1013904223) >>> 0; return seed / 4294967296; };
const v3 = (): [number, number, number] => [rnd() * 20 - 10, rnd() * 20 - 10, rnd() * 20 - 10];
const unit = () => { const v = v3(); const l = Math.hypot(...v) || 1; return v.map((x) => x / l) as [number, number, number]; };
const box = () => { const a = v3(); return { min: a, max: [a[0] + rnd() * 5, a[1] + rnd() * 5, a[2] + rnd() * 5] as [number, number, number] }; };
const results: { name: string; ok: boolean; detail?: string }[] = [];
const thrown: string[] = [];
const ser = (v: unknown) => JSON.stringify(v, (_k, x) => (typeof x === "number" && !Number.isFinite(x) ? String(x) : x instanceof Map ? [...x] : x));
const check = (name: string, f: (m: any) => unknown) => {
  let a: string, b: string;
  try { a = ser(f(oldM)); } catch (e) { a = "THROW " + String((e as Error).message); }
  try { b = ser(f(newM)); } catch (e) { b = "THROW " + String((e as Error).message); }
  if (a.startsWith("THROW")) thrown.push(name + ": " + a.slice(0, 90));
  results.push({ name, ok: a === b, detail: a === b ? undefined : `${a.slice(0, 120)} != ${b.slice(0, 120)}` });
};
const names: Record<string, (m: any) => unknown> = {};
for (let i = 0; i < 40; i++) {
  const a = v3(), b = v3(), c = v3(), n = unit(), s = rnd() * 3 + 0.1, ang = rnd() * 360;
  const bx = box(), by = box(), bz = box(), bw = box(), ax2 = unit(), pick = v3();
  const pts = [a, b, c, v3(), v3()];
  const cases: [string, (m: any) => unknown][] = [
    ["vec3Add", (m) => m.vec3Add(a, b)], ["vec3Sub", (m) => m.vec3Sub(a, b)], ["vec3Scale", (m) => m.vec3Scale(a, s)], ["vec3Dot", (m) => m.vec3Dot(a, b)], ["vec3Cross", (m) => m.vec3Cross(a, b)], ["vec3Length", (m) => m.vec3Length(a)], ["vec3Distance", (m) => m.vec3Distance(a, b)], ["vec3Normalize", (m) => m.vec3Normalize(a)],
    ["constrainMovePoint", (m) => [m.constrainMovePoint(a, b, "x"), m.constrainMovePoint(a, b, "plane", n), m.constrainMovePoint(a, b, "none")]],
    ["arcPlaneFrame", (m) => m.arcPlaneFrame(a, b, c)], ["arcSweepRadians", (m) => { const f = m.arcPlaneFrame(a, b, c); return f ? m.arcSweepRadians(f, c) : null; }], ["arcSamplePoints", (m) => m.arcSamplePoints(a, b, c, 8)],
    ["arcFrameFromRadiusPoint", (m) => m.arcFrameFromRadiusPoint(a, b)], ["arcEndOnCircle", (m) => m.arcEndOnCircle(a, b, c)], ["arcEndFromAngle", (m) => m.arcEndFromAngle(a, b, ang)],
    ["circleSamplePoints", (m) => m.circleSamplePoints(a, n, s, 12)], ["ellipseSamplePoints", (m) => m.ellipseSamplePoints(a, n, ax2, s, s / 2, 12)],
    ["nurbsDisplaySamplePoints", (m) => m.nurbsDisplaySamplePoints(pts, 5)], ["polylineLength", (m) => m.polylineLength(pts)],
    ["edgeCurveLength", (m) => [m.edgeCurveLength(undefined, [a, b]), m.edgeCurveLength({ kind: "circle", center: a, normal: n, radius: s } as any, [a, b]), m.edgeCurveLength({ kind: "ellipse", center: a, normal: n, majorAxis: ax2, majorRadius: s + 1, minorRadius: s } as any, [a, b]), m.edgeCurveLength(m.nurbsCurveFromPoles(pts), [a, b]), m.edgeCurveLength({ kind: "arc", center: a, normal: n, radius: s } as any, [b, c])]],
    ["edgeSamplePoints", (m) => m.edgeSamplePoints({ v1: { id: "v1", position: a }, v2: { id: "v2", position: b } }, { id: "e1", vertexIds: ["v1", "v2"] }, 6)],
    ["circleFromCenterRadiusPoint", (m) => m.circleFromCenterRadiusPoint(a, b)], ["nurbsCurveFromPoles", (m) => [m.nurbsCurveFromPoles(pts), m.nurbsCurveFromPoles(pts, true), m.nurbsCurveFromPoles([a])]],
    ["faceNormalFromPoints", (m) => m.faceNormalFromPoints([a, b, c])],
    ["solidPrimitiveAabb", (m) => [m.solidPrimitiveAabb({ kind: "box", cornerA: a, cornerB: b, height: s } as any), m.solidPrimitiveAabb({ kind: "sphere", center: a, radius: s } as any), m.solidPrimitiveAabb({ kind: "cylinder", base: a, axis: n, height: s, radius: s / 2 } as any), m.solidPrimitiveAabb({ kind: "cone", base: a, axis: n, height: s, radius: s / 2, radiusTop: s / 3 } as any)]],
    ["aabbCornerPoints", (m) => m.aabbCornerPoints(bx.min, bx.max)], ["aabbIntersect", (m) => m.aabbIntersect(bx, by)], ["aabbVolume", (m) => m.aabbVolume(bx)],
    ["aabbOverlapUnionVolume", (m) => m.aabbOverlapUnionVolume(bx, [by, bz, bw])], ["aabbDifferencePieces", (m) => m.aabbDifferencePieces(bx, [by, bz])],
    ["readVec3", (m) => [m.readVec3(a), m.readVec3("x"), m.readVec3([1, 2])]], ["computeBoxPreviewLayout", (m) => m.computeBoxPreviewLayout(a, b, s)], ["aabbFromPoints", (m) => [m.aabbFromPoints(pts), m.aabbFromPoints(pts, 1), m.aabbFromPoints([])]],
    ["transformPointsForPreviewKind", (m) => ["move", "scale", "rotate", "mirror", "other"].map((k) => { const f = m.transformPointsForPreviewKind(k, { pick, delta: a, vector: a, factor: s, scale: [s, s, s], origin: b, axis: n, angle: ang, anchor: b, target: c, from: b, to: c, normal: n, point: a, base: a, rotation: ang, angleDeg: ang }); return pts.map((p) => f(p)); })],
    ["facePlaneGroupKey", (m) => m.facePlaneGroupKey(n, a)], ["projectPointOnScalarAxis", (m) => m.projectPointOnScalarAxis(a, n, b)], ["scalarTopOnAxis", (m) => m.scalarTopOnAxis(a, n, s, 1.5)], ["clampPointAlongDirection", (m) => m.clampPointAlongDirection(a, b, s)],
    ["meshFaceModelDiff", (m) => m.meshFaceModelDiff({ position: new Float32Array([...a, ...b, ...c]), normal: new Float32Array([0, 0, 1, 0, 0, 1, 0, 0, 1]), index: new Uint32Array([0, 1, 2]) } as any, "t" + i)],
    ["boxModelDiff", (m) => m.boxModelDiff({ cornerA: a, cornerB: b, height: s }, "solid-1")],
  ];
  for (const [name, f] of cases) check(`${name}#${i}`, f);
}
const buildModel = (m: any, i: number) => {
  const model = new modelMod.Model();
  const a = v3(), b = [a[0] + 2 + rnd(), a[1] + 2 + rnd(), a[2]] as [number, number, number];
  spatial.applyModelDiff(model, m.boxModelDiff({ cornerA: a, cornerB: b, height: 1 + rnd() }, "solid-" + i));
  return model;
};
for (let i = 0; i < 10; i++) {
  const sd = seed;
  const seedAt = () => { seed = sd; };
  const probe = (name: string, f: (m: any, model: any) => unknown) => check(`${name}@${i}`, (m) => { seedAt(); const model = buildModel(m, i); return f(m, model); });
  probe("modelObjectAabb", (m, model) => Object.values(model.solids).map((s) => m.modelObjectAabb(model, s)));
  probe("derivedSolidPoints", (m, model) => Object.values(model.solids).map((s) => (m.PreciseSpatialKernelMath, m.solidFaceIds(model, (s as any).id))));
  probe("faceCentroid/normal/points", (m, model) => Object.values(model.faces).map((f) => [m.faceCentroid(model, f), m.faceNormal(model, f), m.preciseSpatialKernelMath.facePoints(model, f), m.derivedFacePoints(model, f)]));
  probe("fuseSolidsToExternalFaces", (m, model) => m.fuseSolidsToExternalFaces(model, Object.keys(model.solids)));
  probe("evaluateAnchorPosition", (m, model) => Object.values(model.vertices).slice(0, 3).map((v: any) => m.evaluateAnchorPosition(model, { id: "anc", position: [9, 9, 9], attachment: { kind: "vertex", id: v.id } } as any)));
  probe("anchorPlacementFromEntity", (m, model) => [
    ...Object.values(model.vertices).slice(0, 2).map((v: any) => m.anchorPlacementFromEntity(model, "vertex", v.id, [1, 2, 3])),
    ...Object.values(model.edges).slice(0, 2).map((e: any) => m.anchorPlacementFromEntity(model, "edge", e.id, [1, 2, 3])),
    ...Object.values(model.faces).slice(0, 3).map((f: any) => m.anchorPlacementFromEntity(model, "face", f.id, [0.5, 1.5, 2.5])),
    ...Object.values(model.solids).slice(0, 1).map((s: any) => m.anchorPlacementFromEntity(model, "solid", s.id, [0.5, 1.5, 2.5])),
  ]);
  probe("evaluateAnchorPosition surfaces", (m, model) => [
    ...Object.values(model.faces).slice(0, 3).map((f: any) => m.evaluateAnchorPosition(model, { id: "a", position: [9, 9, 9], attachment: { kind: "face", id: f.id, u: 0.3, v: 0.6 } } as any)),
    ...Object.values(model.solids).slice(0, 1).map((s: any) => m.evaluateAnchorPosition(model, { id: "a", position: [9, 9, 9], attachment: { kind: "solid", id: s.id, u: 0.3, v: 0.6, w: 0.2 } } as any)),
    ...Object.values(model.edges).slice(0, 2).map((e: any) => m.evaluateAnchorPosition(model, { id: "a", position: [9, 9, 9], attachment: { kind: "edge", id: e.id, t: 0.4 } } as any)),
  ]);
}
const methods = ["vec3Add", "vec3Sub", "vec3Scale", "vec3Dot", "vec3Cross", "vec3Length", "vec3Distance", "vec3Normalize", "arcPlaneFrame", "arcSweepRadians", "arcSamplePoints", "arcFrameFromRadiusPoint", "arcEndOnCircle", "arcEndFromAngle", "circleSamplePoints", "ellipseSamplePoints", "nurbsDisplaySamplePoints", "polylineLength", "edgeCurveLength", "edgeSamplePoints", "circleFromCenterRadiusPoint", "nurbsCurveFromPoles", "aabbFromPoints", "aabbCornerPoints", "aabbIntersect", "solidPrimitiveAabb", "modelObjectAabb", "boxModelDiff", "meshFaceModelDiff", "evaluateAnchorPosition", "anchorPlacementFromEntity", "computeBoxPreviewLayout", "transformPointsForPreviewKind", "constrainMovePoint", "facePoints", "faceCentroid", "faceNormal", "solidFaceIds", "fuseSolidsToExternalFaces", "facePlaneGroupKey", "projectPointOnScalarAxis", "scalarTopOnAxis", "clampPointAlongDirection", "abs", "min2", "max2", "minN", "maxN", "hypot3", "atan2", "cos", "sin", "randomTag"];
const oldKeys = Object.keys(oldM.preciseSpatialKernelMath).sort().join(","), newKeys = Object.keys(newM.preciseSpatialKernelMath).sort().join(",");
results.push({ name: "class members identical", ok: oldKeys === newKeys, detail: oldKeys === newKeys ? undefined : `${oldKeys} != ${newKeys}` });
const exportsOld = Object.keys(oldM).sort(), exportsNew = Object.keys(newM).sort();
const exportsMissing = exportsOld.filter((k) => !exportsNew.includes(k));
results.push({ name: "parent exports (informational: moved to children)", ok: true, detail: `missing from parent ${exportsMissing.length}` });
const bad = results.filter((r) => !r.ok);
console.log(JSON.stringify({ total: results.length, mismatches: bad.length, oldThrew: thrown.length, thrownSample: [...new Set(thrown.map((t) => t.replace(/#\d+|@\d+/u, "")))].slice(0, 12), sample: bad.slice(0, 10), methods: methods.length }, null, 1));
process.exit(bad.length ? 1 : 0);
