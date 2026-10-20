/** ⚙️ The brepjs WebAssembly engine: the OpenCascade-backed implementation of every spatial kernel operation.
 *
 * @see ../🟦️.ts — the brepjs kernel module, kept ONLY as a vitest differential oracle
 */
import { box, bsplineApprox, circle, cone, curveEndPoint, curveStartPoint, cut, cylinder, face, intersect, loft, sweep as sweepAlongSpine, initializeOwnedOpenCascade, isOk, isSolid, isValidSolid, line, measureArea, measureDistance, measureLength, measureVolume, offsetFace, fuseAll, sphere, threePointArc, unwrap, vertex as brepVertex, wireLoop, type OwnedBrepSolid as ValidSolid, type OwnedBrepWire as Wire } from "@semio-tech/cad-engine-ts/brepjs";
import { isEmptyModelDiff, type ModelDiff } from "@semio-tech/s-spatial-kernel-ts/spatial";
import { Model } from "@semio-tech/s-spatial-kernel-ts/geometry/model";
import { emptyMeshTransfer, kernelGeometry, type EdgeCurve, type MeshTransfer, type Vec3 } from "@semio-tech/framework-3d-js";
import { boxModelDiff } from "@semio-tech/s-spatial-kernel-ts/preview";
import { arcEndFromAngle, arcEndOnCircle, arcSamplePoints, circleFromCenterRadiusPoint, edgeCurveLength, nurbsCurveFromPoles, nurbsDisplaySamplePoints } from "@semio-tech/s-spatial-kernel-ts/preview/curve-sampling";
import { geom, type EdgeRecord, type EdgeRef, type FaceRecord, type FaceRef, type SolidPrimitive, type SolidRecord, type SolidRef, type VertexRef, type WireRef } from "@semio-tech/s-spatial-kernel-ts/preview/brep-records";
import { vec3Add, vec3Cross, vec3Distance, vec3Length, vec3Normalize, vec3Scale, vec3Sub } from "@semio-tech/s-spatial-kernel-ts/preview/vec3";
import { createOpenCascadeLocateFile, MESH_TRANSFER_CACHE_MAX } from "../🧩️opencascade-and-scratch/🟦️.ts";
import { cloneMeshTransfer, meshTransferFromBrep } from "../🧱️entity-maps/🟦️.ts";
import { defaultSweepProfileWire, edgeIdsFromPicks, extrudeModelWire, geomEdgeToBrepEdge, geomWireToBrepWire, geomWireToOrientedFace, geomWireToOrientedFaceLoose, mergeSolidAdds, picksFromRaw, picksFromValue, selectionPicksFromParams, solidRefsFromSelectionRaw, wireIdsFromKeyedRaw, wireIdsFromSelectionPicks } from "../🔗️wires-and-selection/🟦️.ts";
import { cornerConnectorDiff, editExplodeDiff, editJoinDiff, editSplitDiff, editTrimDiff } from "../✂️edit-diffs/🟦️.ts";
import { deriveValidSolidFromRecordOrPrimitive, extrusionDistanceFromParams } from "../🧊️solid-topology-and-fixture/🟦️.ts";

// #region 🔌️BrepjsWasmEngine
/** 🔌️ WASM-side engine: exact solids keyed by `SolidRef` (runs in worker or local fallback). */
export class BrepjsWasmEngine {
  readonly operations: readonly string[] = ["solid.createBox", "wire.extrudeToSolid", "face.offset", "entity.tessellate", "measure.distance", "measure.area", "measure.volume"];

  private initPromise: Promise<void> | null = null;
  private seq = 0;
  private readonly solids = new Map<SolidRef, ValidSolid>();
  private solidsModelKey: string | null = null;

  /** 🧪️ Clears solids cache (vitest shared kernel). */
  resetDerivedPipeline(): void {
    this.solids.clear();
    this.meshCache.clear();
    this.solidsModelKey = null;
  }

  private modelDerivedKey(model: Model): string {
    const solids = (Object.keys(geom(model).solids) as SolidRef[]).sort().join(",");
    const vertexDigest = Object.values(geom(model).vertices)
      .map((v) => `${v.id}:${v.position.map((n) => n.toFixed(4)).join(",")}`)
      .sort()
      .join("|");
    return `${model.revision}:${solids}:${vertexDigest}`;
  }

  async ensureInit(): Promise<void> {
    if (!this.initPromise) {
      this.initPromise = createOpenCascadeLocateFile().then(initializeOwnedOpenCascade);
    }
    await this.initPromise;
  }

  private solidFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): ValidSolid {
    const ax = Math.min(input.cornerA[0], input.cornerB[0]);
    const ay = Math.min(input.cornerA[1], input.cornerB[1]);
    const bx = Math.max(input.cornerA[0], input.cornerB[0]);
    const by = Math.max(input.cornerA[1], input.cornerB[1]);
    const w = bx - ax;
    const d = by - ay;
    const h = input.height;
    const minZ = Math.min(input.cornerA[2], input.cornerB[2]);
    const cx = (ax + bx) / 2;
    const cy = (ay + by) / 2;
    return box(w, d, h, { at: [cx, cy, minZ + h / 2], centered: true });
  }

  async createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<SolidRef> {
    await this.ensureInit();
    const solid = this.solidFromCorners(input);
    const ref = kernelGeometry.solidRef(`brepjs-solid-${++this.seq}`);
    this.solids.set(ref, solid);
    return ref;
  }

  async volume(solid: SolidRef): Promise<number> {
    await this.ensureInit();
    const s = this.solids.get(solid);
    if (!s) return 0;
    return unwrap(measureVolume(s));
  }

  private readonly meshCache = new Map<string, MeshTransfer>();

  private meshCacheKey(solid: SolidRef, tolerance: number, model?: Model): string {
    return model ? `${String(solid)}:${tolerance}:r${model.revision}` : `${String(solid)}:${tolerance}`;
  }

  disposeSolid(solid: SolidRef): void {
    const prefix = `${String(solid)}:`;
    for (const key of [...this.meshCache.keys()]) {
      if (key.startsWith(prefix)) this.meshCache.delete(key);
    }
    this.solids.delete(solid);
  }

  async tessellate(solid: SolidRef, tolerance: number, model?: Model): Promise<MeshTransfer> {
    await this.ensureInit();
    if (model) await this.syncSolidsFromModel(model);
    const s = this.solids.get(solid);
    if (!s) return emptyMeshTransfer();
    const key = this.meshCacheKey(solid, tolerance, model);
    const cached = this.meshCache.get(key);
    if (cached) return cloneMeshTransfer(cached);
    const solidRecord = model ? geom(model).solids[solid] : undefined;
    const transfer = meshTransferFromBrep(s, tolerance, { solidRef: solid, model, solidRecord });
    if (this.meshCache.size >= MESH_TRANSFER_CACHE_MAX) {
      const first = this.meshCache.keys().next().value;
      if (first) this.meshCache.delete(first);
    }
    this.meshCache.set(key, cloneMeshTransfer(transfer));
    return transfer;
  }

  /** 🧊️ Brep for one shape source solid (topology or primitive; no fused-hull metadata). */
  private brepForShapeSourceSolid(model: Model, solidId: SolidRef): ValidSolid | null {
    const rec = geom(model).solids[solidId];
    if (!rec) return null;
    const cached = this.solids.get(rec.id);
    if (cached) return cached;
    return deriveValidSolidFromRecordOrPrimitive(model, rec, (p) => this.solidFromSolidPrimitive(p));
  }

  /** 🧊️ Boolean-union brep for energy hull rows tagged with `fuseSourceSolidIds` metadata. */
  private fusedHullBrepFromMetadata(model: Model, hullId: SolidRef): ValidSolid | null {
    const meta = model.metadata.get(String(hullId));
    const raw = meta?.fuseSourceSolidIds;
    if (!Array.isArray(raw) || raw.length === 0) return null;
    const shapes: ValidSolid[] = [];
    for (const sid of raw) {
      const brep = this.brepForShapeSourceSolid(model, sid as SolidRef);
      if (brep) shapes.push(brep);
    }
    if (shapes.length === 0) return null;
    if (shapes.length === 1) return shapes[0]!;
    const fused = fuseAll(shapes);
    return isOk(fused) ? fused.value : null;
  }

  /** 🧊️ Authoritative brep for a solid: fused hull metadata, shell topology, else analytic primitive. */
  solidForSolidRecord(model: Model, solid: SolidRecord): ValidSolid | null {
    const cached = this.solids.get(solid.id);
    if (cached) return cached;
    const fusedHull = this.fusedHullBrepFromMetadata(model, solid.id);
    if (fusedHull) {
      this.solids.set(solid.id, fusedHull);
      return fusedHull;
    }
    return deriveValidSolidFromRecordOrPrimitive(model, solid, (p) => this.solidFromSolidPrimitive(p));
  }

  /** 🧊️ Resolves solid refs to live `ValidSolid` breps for boolean operands (fused hull, shell topology, or primitive). */
  validSolidsFromRefs(model: Model, refs: readonly SolidRef[]): ValidSolid[] {
    const out: ValidSolid[] = [];
    for (const ref of refs) {
      const rec = geom(model).solids[ref];
      if (!rec) continue;
      const brep = this.solidForSolidRecord(model, rec);
      if (brep) out.push(brep);
    }
    return out;
  }

  async syncSolidsFromModel(model: Model): Promise<void> {
    await this.ensureInit();
    const modelKey = this.modelDerivedKey(model);
    if (this.solidsModelKey === modelKey && this.solids.size > 0) return;
    const kernelBreps = new Map<SolidRef, ValidSolid>();
    for (const cell of Object.values(geom(model).solids)) {
      if (cell.shellIds.length > 0) continue;
      const cached = this.solids.get(cell.id);
      if (cached) kernelBreps.set(cell.id, cached);
    }
    this.solids.clear();
    this.meshCache.clear();
    for (const cell of Object.values(geom(model).solids)) {
      const brep = this.solidForSolidRecord(model, cell);
      if (brep) this.solids.set(cell.id, brep);
    }
    for (const [id, brep] of kernelBreps) {
      if (!this.solids.has(id)) this.solids.set(id, brep);
    }
    this.solidsModelKey = modelKey;
  }

  /** 🧊️ Builds brepjs `ValidSolid` from `SolidPrimitive` (sphere/cylinder/cone/box). */
  solidFromSolidPrimitive(solid: SolidPrimitive): ValidSolid {
    if (solid.kind === "sphere") {
      return sphere(solid.radius, { at: [solid.center[0], solid.center[1], solid.center[2]] });
    }
    if (solid.kind === "cylinder") {
      const h = Math.max(solid.height, 1e-6);
      const ax = solid.axis;
      const axLen = Math.hypot(ax[0], ax[1], ax[2]);
      const axis: Vec3 = axLen > 1e-12 ? [ax[0] / axLen, ax[1] / axLen, ax[2] / axLen] : [0, 0, 1];
      return cylinder(solid.radius, h, {
        at: solid.base,
        axis,
        centered: false,
      });
    }
    if (solid.kind === "cone") {
      const r2 = solid.radiusTop ?? 0;
      const h = Math.max(solid.height, 1e-6);
      const ax = solid.axis;
      const axLen = Math.hypot(ax[0], ax[1], ax[2]);
      const axis: Vec3 = axLen > 1e-12 ? [ax[0] / axLen, ax[1] / axLen, ax[2] / axLen] : [0, 0, 1];
      return cone(solid.radius, r2, h, {
        at: solid.base,
        axis,
        centered: false,
      });
    }
    return this.solidFromCorners({ cornerA: solid.cornerA, cornerB: solid.cornerB, height: solid.height });
  }

  async executeCommandDiff(commandId: string, params: Record<string, unknown>): Promise<{ readonly diff: ModelDiff }> {
    const nextId = (kind: string) => `brepjs-${kind}-${Math.random().toString(36).slice(2, 9)}`;
    const asVec3 = (v: unknown, fallback: Vec3): Vec3 => (Array.isArray(v) && v.length >= 3 ? ([Number(v[0]), Number(v[1]), Number(v[2])] as Vec3) : fallback);
    const poleList = (raw: unknown): Vec3[] => {
      if (!Array.isArray(raw)) return [];
      const out: Vec3[] = [];
      for (const p of raw) {
        if (!Array.isArray(p) || p.length < 3) continue;
        out.push([Number(p[0]), Number(p[1]), Number(p[2])]);
      }
      return out;
    };

    const createVertex = (pos: Vec3) => {
      const id = nextId("v");
      return { id: id as VertexRef, position: pos };
    };

    if (commandId === "curve.line") {
      const p0 = asVec3(params.p0, [0, 0, 0]);
      const p1 = asVec3(params.p1, [1, 0, 0]);
      const v0 = createVertex(p0);
      const v1 = createVertex(p1);
      const e = { id: nextId("e") as EdgeRef, vertexIds: [v0.id, v1.id], curve: { kind: "line" as const } };
      const w = { id: nextId("w") as WireRef, edgeIds: [e.id] };
      return { diff: { vertices: { added: [v0, v1] }, edges: { added: [e] }, wires: { added: [w] } } };
    }
    if (commandId === "curve.polyline") {
      const pts = poleList(params.points);
      if (pts.length < 2) return { diff: {} };
      const verts = pts.map((p) => createVertex(p));
      const edges: EdgeRecord[] = [];
      for (let i = 0; i < verts.length - 1; i++) {
        edges.push({ id: nextId("e") as EdgeRef, vertexIds: [verts[i]!.id, verts[i + 1]!.id], curve: { kind: "line" } });
      }
      const w = { id: nextId("w") as WireRef, edgeIds: edges.map((e) => e.id) };
      return { diff: { vertices: { added: verts }, edges: { added: edges }, wires: { added: [w] } } };
    }
    if (commandId === "curve.circle") {
      const center = asVec3(params.center, [0, 0, 0]);
      const radiusPoint = asVec3(params.radiusPoint, [1, 0, 0]);
      const geom = circleFromCenterRadiusPoint(center, radiusPoint);
      if (!geom) return { diff: {} };
      const brepEdge = circle(geom.radius, { at: geom.center, axis: geom.normal });
      const v = createVertex(radiusPoint);
      const curve: EdgeCurve = { kind: "circle", center: geom.center, normal: geom.normal, radius: geom.radius };
      const e = { id: nextId("e") as EdgeRef, vertexIds: [v.id, v.id], curve };
      const w = { id: nextId("w") as WireRef, edgeIds: [e.id] };
      void brepEdge;
      return { diff: { vertices: { added: [v] }, edges: { added: [e] }, wires: { added: [w] } } };
    }
    if (commandId === "curve.arc") {
      const center = asVec3(params.center, [0, 0, 0]);
      const start = params.start != null ? asVec3(params.start, [1, 0, 0]) : ([1, 0, 0] as Vec3);
      const endRaw = params.end != null ? asVec3(params.end, start) : null;
      const angle = typeof params.angle === "number" ? params.angle : null;
      let endPos: Vec3;
      if (endRaw) endPos = arcEndOnCircle(center, start, endRaw);
      else if (angle !== null) endPos = arcEndFromAngle(center, start, angle) ?? start;
      else endPos = start;
      const mid = arcSamplePoints(center, start, endPos, 4)[2] ?? endPos;
      const brepEdge = threePointArc(start, mid, endPos);
      const vStart = createVertex(curveStartPoint(brepEdge));
      const vEnd = createVertex(curveEndPoint(brepEdge));
      const curve: EdgeCurve = { kind: "arc", center };
      const e = { id: nextId("e") as EdgeRef, vertexIds: [vStart.id, vEnd.id], curve };
      const w = { id: nextId("w") as WireRef, edgeIds: [e.id] };
      return { diff: { vertices: { added: [vStart, vEnd] }, edges: { added: [e] }, wires: { added: [w] } } };
    }
    if (commandId === "curve.controlPointCurve" || commandId === "curve.interpolateCurve") {
      const poles = poleList(params.points);
      if (poles.length < 2) return { diff: {} };
      const through = commandId === "curve.interpolateCurve";
      const fitPoints = through ? [...nurbsDisplaySamplePoints(poles, 16)] : poles;
      const brepResult = bsplineApprox(fitPoints);
      if (!isOk(brepResult)) return { diff: {} };
      const curve = nurbsCurveFromPoles(poles, through);
      if (!curve) return { diff: {} };
      const vStart = createVertex(curveStartPoint(brepResult.value));
      const vEnd = createVertex(curveEndPoint(brepResult.value));
      const e = { id: nextId("e") as EdgeRef, vertexIds: [vStart.id, vEnd.id], curve };
      const w = { id: nextId("w") as WireRef, edgeIds: [e.id] };
      return { diff: { vertices: { added: [vStart, vEnd] }, edges: { added: [e] }, wires: { added: [w] } } };
    }
    if (commandId === "solid.sphere") {
      const center = asVec3(params.center, [0, 0, 0]);
      const radiusPoint = params.radiusPoint != null ? asVec3(params.radiusPoint, center) : null;
      const radius = typeof params.radius === "number" ? params.radius : radiusPoint ? vec3Distance(center, radiusPoint) : 1;
      const solid: SolidPrimitive = { kind: "sphere", center, radius };
      const c = { id: nextId("c") as SolidRef, shellIds: [], solid };
      await this.ensureInit();
      this.solids.set(c.id, this.solidFromSolidPrimitive(solid));
      return { diff: { solids: { added: [c] } } };
    }
    if (commandId === "solid.cylinder") {
      const base = asVec3(params.base, [0, 0, 0]);
      const radiusPoint = asVec3(params.radiusPoint, [1, 0, 0]);
      const end = asVec3(params.end, base);
      const radius = vec3Distance(base, radiusPoint);
      const axisVec = vec3Sub(end, base);
      const height = vec3Length(axisVec);
      const axis = height > 1e-9 ? vec3Normalize(axisVec) : ([0, 0, 1] as Vec3);
      const solid: SolidPrimitive = { kind: "cylinder", base, axis, radius, height: height > 1e-9 ? height : 1e-6 };
      const c = { id: nextId("c") as SolidRef, shellIds: [], solid };
      await this.ensureInit();
      this.solids.set(c.id, this.solidFromSolidPrimitive(solid));
      return { diff: { solids: { added: [c] } } };
    }
    if (commandId === "solid.cone") {
      const base = asVec3(params.base, [0, 0, 0]);
      const radiusPoint = asVec3(params.radiusPoint, [1, 0, 0]);
      const end = asVec3(params.end, [0, 0, 1] as Vec3);
      const radius = vec3Distance(base, radiusPoint);
      const axisVec = vec3Sub(end, base);
      const height = vec3Length(axisVec);
      const axis = height > 1e-9 ? vec3Normalize(axisVec) : ([0, 0, 1] as Vec3);
      const solid: SolidPrimitive = { kind: "cone", base, axis, radius, height: height > 1e-9 ? height : 1e-6, radiusTop: 0 };
      const c = { id: nextId("c") as SolidRef, shellIds: [], solid };
      await this.ensureInit();
      this.solids.set(c.id, this.solidFromSolidPrimitive(solid));
      return { diff: { solids: { added: [c] } } };
    }
    const fuseShapes = (shapes: readonly ValidSolid[]): ValidSolid | null => {
      if (shapes.length === 0) return null;
      if (shapes.length === 1) return shapes[0]!;
      const fused = fuseAll(shapes as ValidSolid[]);
      return isOk(fused) ? fused.value : null;
    };
    if (commandId === "solid.booleanUnion") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      await this.ensureInit();
      const refs = solidRefsFromSelectionRaw(model, params.targets);
      if (refs.length < 2) return { diff: {} };
      const shapes = this.validSolidsFromRefs(model, refs);
      if (shapes.length < 2) return { diff: {} };
      const fused = fuseAll(shapes);
      if (!isOk(fused)) return { diff: {} };
      const ref = kernelGeometry.solidRef(nextId("union"));
      this.solids.set(ref, fused.value);
      return { diff: { solids: { added: [{ id: ref, shellIds: [] }], removed: refs } } };
    }
    if (commandId === "solid.booleanDifference") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      await this.ensureInit();
      const baseRefs = solidRefsFromSelectionRaw(model, params.baseObjects);
      const toolRefs = solidRefsFromSelectionRaw(model, params.cutterObjects);
      if (!baseRefs.length || !toolRefs.length) return { diff: {} };
      const baseShape = fuseShapes(this.validSolidsFromRefs(model, baseRefs));
      const toolShapes = this.validSolidsFromRefs(model, toolRefs);
      if (!baseShape || !toolShapes.length) return { diff: {} };
      let result = baseShape;
      for (const tool of toolShapes) {
        const cutResult = cut(result, tool);
        if (!isOk(cutResult)) return { diff: {} };
        result = cutResult.value;
      }
      const ref = kernelGeometry.solidRef(nextId("diff"));
      this.solids.set(ref, result);
      return { diff: { solids: { added: [{ id: ref, shellIds: [] }], removed: [...baseRefs, ...toolRefs] } } };
    }
    if (commandId === "solid.booleanIntersection") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      await this.ensureInit();
      const firstRefs = solidRefsFromSelectionRaw(model, params.firstSet);
      const secondRefs = solidRefsFromSelectionRaw(model, params.secondSet);
      if (!firstRefs.length || !secondRefs.length) return { diff: {} };
      const firstShape = fuseShapes(this.validSolidsFromRefs(model, firstRefs));
      const secondShape = fuseShapes(this.validSolidsFromRefs(model, secondRefs));
      if (!firstShape || !secondShape) return { diff: {} };
      const isected = intersect(firstShape, secondShape);
      if (!isOk(isected)) return { diff: {} };
      const ref = kernelGeometry.solidRef(nextId("isect"));
      this.solids.set(ref, isected.value);
      return { diff: { solids: { added: [{ id: ref, shellIds: [] }], removed: [...firstRefs, ...secondRefs] } } };
    }
    if (commandId.startsWith("solid.")) {
      return { diff: {} };
    }
    if (commandId === "transform.mirror") {
      const v0 = createVertex([0, 0, 0]);
      return { diff: { vertices: { added: [v0] } } };
    }
    if (commandId.endsWith("From2PointsAndHeight")) {
      const p0 = asVec3(params.pointA, asVec3(params.p0, [0, 0, 0]));
      const p1 = asVec3(params.pointB, asVec3(params.p1, [1, 1, 0]));
      const height = typeof params.height === "number" && Number.isFinite(params.height) ? params.height : 2.7;
      const cornerA: Vec3 = [Math.min(p0[0], p1[0]), Math.min(p0[1], p1[1]), Math.min(p0[2], p1[2])];
      const cornerB: Vec3 = [Math.max(p0[0], p1[0]), Math.max(p0[1], p1[1]), Math.max(p0[2], p1[2])];
      return this.createBoxFromCornersDiff({ cornerA, cornerB, height });
    }
    if (commandId.endsWith("FromCurveAndHeight")) {
      const wireId = String(params.wireId ?? "");
      const distance = typeof params.height === "number" && Number.isFinite(params.height) ? params.height : 2.7;
      const model = params.model instanceof Model ? params.model : null;
      if (wireId && model) return this.extrudeWireDiff({ wireId, distance, direction: [0, 0, 1], model });
      return { diff: {} };
    }
    if (commandId.endsWith("FromSurface")) {
      const faceId = String(params.faceId ?? "");
      const model = params.model instanceof Model ? params.model : null;
      if (faceId && model) return this.offsetFacesDiff({ faceIds: [faceId], distance: 0.01, model });
      return { diff: {} };
    }
    if (commandId === "surface.extrudeCrv") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      const distance = extrusionDistanceFromParams(params);
      if (!(distance > 1e-9)) return { diff: {} };
      const direction = asVec3(params.direction, [0, 0, 1]);
      const wireIds = wireIdsFromSelectionPicks(model, selectionPicksFromParams(params));
      if (!wireIds.length) return { diff: {} };
      const diffs: ModelDiff[] = [];
      for (const wireId of wireIds) {
        const row = await this.extrudeWireDiff({ wireId, distance, direction, model });
        if (!isEmptyModelDiff(row.diff)) diffs.push(row.diff);
      }
      return { diff: mergeSolidAdds(diffs) };
    }
    if (commandId === "surface.plane") {
      const cornerA = asVec3(params.cornerA, [0, 0, 0]);
      const cornerB = asVec3(params.cornerB, [1, 0, 0]);
      const along = vec3Sub(cornerB, cornerA);
      const side = vec3Length(along) > 1e-9 ? vec3Length(along) : 1;
      const dir = vec3Length(along) > 1e-9 ? vec3Normalize(along) : ([1, 0, 0] as Vec3);
      let perp = vec3Cross(dir, [0, 0, 1]);
      if (vec3Length(perp) < 1e-9) perp = vec3Cross(dir, [0, 1, 0]);
      perp = vec3Normalize(perp);
      const p0 = cornerA;
      const p1 = vec3Add(cornerA, vec3Scale(dir, side));
      const p2 = vec3Add(p1, vec3Scale(perp, side));
      const p3 = vec3Add(cornerA, vec3Scale(perp, side));
      const corners = [p0, p1, p2, p3];
      await this.ensureInit();
      const brepEdges = corners.map((p, i) => line(p, corners[(i + 1) % 4]!));
      const loopResult = wireLoop(brepEdges);
      if (!isOk(loopResult)) return { diff: {} };
      const faceResult = face(loopResult.value as Parameters<typeof face>[0]);
      if (!isOk(faceResult)) return { diff: {} };
      const verts = corners.map((p) => createVertex(p));
      const edges: EdgeRecord[] = verts.map((v, i) => ({ id: nextId("e") as EdgeRef, vertexIds: [v.id, verts[(i + 1) % 4]!.id], curve: { kind: "line" as const } }));
      const w = { id: nextId("w") as WireRef, edgeIds: edges.map((e) => e.id) };
      const normal = vec3Normalize(vec3Cross(dir, perp));
      const f: FaceRecord = { id: nextId("f") as FaceRef, wireIds: [w.id], surface: { kind: "plane", origin: p0, normal } };
      return { diff: { vertices: { added: verts }, edges: { added: edges }, wires: { added: [w] }, faces: { added: [f] } } };
    }
    if (commandId === "surface.loft" || commandId === "surface.networkSrf") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      const wireIds = wireIdsFromSelectionPicks(model, picksFromRaw(params.curves));
      if (!wireIds.length) return { diff: {} };
      await this.ensureInit();
      if (wireIds.length === 1) {
        const planar = geomWireToOrientedFaceLoose(model, wireIds[0]!);
        if (!planar) return { diff: {} };
        const f: FaceRecord = { id: nextId("f") as FaceRef, wireIds: [wireIds[0]!] };
        return { diff: { faces: { added: [f] } } };
      }
      const brepWires: Wire[] = [];
      for (const wid of wireIds) {
        const bw = geomWireToBrepWire(model, wid);
        if (!bw) return { diff: {} };
        brepWires.push(bw);
      }
      const lofted = loft(brepWires as Parameters<typeof loft>[0], { ruled: true });
      if (!isOk(lofted)) return { diff: {} };
      const f: FaceRecord = { id: nextId("f") as FaceRef, wireIds: [...wireIds] };
      return { diff: { faces: { added: [f] } } };
    }
    if (commandId === "surface.sweep1" || commandId === "surface.sweep2") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      const railIds = wireIdsFromKeyedRaw(model, params.targets);
      if (!railIds.length) return { diff: {} };
      const sectionIds = wireIdsFromSelectionPicks(model, picksFromRaw(params.sections));
      await this.ensureInit();
      const railWire = geomWireToBrepWire(model, railIds[0]!);
      if (!railWire) return { diff: {} };
      const profileWire = sectionIds.length ? geomWireToBrepWire(model, sectionIds[0]!) : defaultSweepProfileWire(model, railIds[0]!);
      if (!profileWire) return { diff: {} };
      const swept = sweepAlongSpine(profileWire as Parameters<typeof sweepAlongSpine>[0], railWire as Parameters<typeof sweepAlongSpine>[1]);
      if (!isOk(swept)) return { diff: {} };
      const boundaryWireIds = sectionIds.length ? [railIds[0]!, sectionIds[0]!] : [railIds[0]!];
      const f: FaceRecord = { id: nextId("f") as FaceRef, wireIds: boundaryWireIds };
      return { diff: { faces: { added: [f] } } };
    }
    if (commandId === "edit.join") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      return { diff: editJoinDiff(model, params) };
    }
    if (commandId === "edit.explode") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      return { diff: editExplodeDiff(model, params) };
    }
    if (commandId === "edit.split") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      return { diff: editSplitDiff(model, params) };
    }
    if (commandId === "edit.trim") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      return { diff: editTrimDiff(model, params) };
    }
    if (commandId === "edit.chamfer" || commandId === "edit.fillet") {
      const model = params.model instanceof Model ? params.model : null;
      if (!model) return { diff: {} };
      const keyed = params.targets as { readonly firstCurve?: unknown; readonly secondCurve?: unknown } | undefined;
      const idA = edgeIdsFromPicks(model, picksFromValue(keyed?.firstCurve))[0] ?? null;
      const idB = edgeIdsFromPicks(model, picksFromValue(keyed?.secondCurve))[0] ?? null;
      if (!idA || !idB) return { diff: {} };
      return { diff: cornerConnectorDiff(model, idA, idB, commandId === "edit.chamfer" ? "chamfer" : "fillet") };
    }

    return { diff: {} };
  }

  async createBoxFromCornersDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<{ readonly diff: ModelDiff; readonly solid: SolidRef }> {
    const solid = await this.createBoxFromCorners(input);
    const diff = boxModelDiff(input, solid);
    return { diff, solid };
  }

  async extrudeWireDiff(input: { wireId: string; distance: number; direction: Vec3; model: Model }): Promise<{ readonly diff: ModelDiff; readonly solid: SolidRef }> {
    const solid = await this.extrudeWire(input);
    return { diff: { solids: { added: [{ id: solid, shellIds: [] }] } }, solid };
  }

  async offsetFacesDiff(input: { faceIds: readonly string[]; distance: number; model: Model }): Promise<{ readonly diff: ModelDiff }> {
    await this.ensureInit();
    const fid = input.faceIds[0];
    if (!fid) return { diff: {} };
    const fr = geom(input.model).faces[fid];
    const wireId = fr?.wireIds[0];
    if (!wireId) return { diff: {} };
    const planar = geomWireToOrientedFace(input.model, wireId);
    if (!planar) return { diff: {} };
    const offset = offsetFace(planar, input.distance);
    if (!isOk(offset)) return { diff: {} };
    const offsetSolid = offset.value;
    if (!isSolid(offsetSolid) || !isValidSolid(offsetSolid)) return { diff: {} };
    const ref = kernelGeometry.solidRef(`brepjs-offset-${++this.seq}`);
    this.solids.set(ref, offsetSolid);
    return { diff: { solids: { added: [{ id: ref, shellIds: [] }] } } };
  }

  async vertexDistance(a: VertexRef, b: VertexRef, model: Model): Promise<number> {
    await this.ensureInit();
    const pa = geom(model).vertices[String(a)]?.position;
    const pb = geom(model).vertices[String(b)]?.position;
    if (!pa || !pb) return 0;
    return unwrap(measureDistance(brepVertex(pa), brepVertex(pb)));
  }

  async edgeLength(e: EdgeRef, model: Model): Promise<number> {
    await this.ensureInit();
    const ed = geom(model).edges[String(e)];
    if (!ed) return 0;
    const brepEdge = geomEdgeToBrepEdge(model, ed);
    if (brepEdge) return unwrap(measureLength(brepEdge));
    const ends: Vec3[] = [];
    for (const vid of ed.vertexIds) {
      const p = geom(model).vertices[String(vid)]?.position;
      if (p) ends.push(p);
    }
    if (ends.length < 2) return 0;
    return edgeCurveLength(ed.curve, [ends[0]!, ends[1]!]);
  }

  async faceArea(f: FaceRef, model: Model): Promise<number> {
    await this.ensureInit();
    const fr = geom(model).faces[String(f)];
    const wireId = fr?.wireIds[0];
    if (!wireId) return 0;
    const planar = geomWireToOrientedFace(model, wireId);
    if (planar) return unwrap(measureArea(planar));
    return 0;
  }

  async solidVolume(c: SolidRef): Promise<number> {
    return this.volume(c);
  }

  async adjacentSolids(cell: SolidRef, model: Model): Promise<readonly SolidRef[]> {
    const out = new Set<string>();
    const c = geom(model).solids[String(cell)];
    if (!c) return [];
    const faces = new Set<string>();
    for (const sid of c.shellIds) {
      const sh = geom(model).shells[sid];
      if (sh) for (const f of sh.faceIds) faces.add(f);
    }
    for (const f of faces) {
      for (const [cid, cellRec] of Object.entries(geom(model).solids)) {
        if (cid === String(cell)) continue;
        for (const sid of cellRec.shellIds) {
          const sh = geom(model).shells[sid];
          if (sh?.faceIds.includes(f as FaceRef)) out.add(cid);
        }
      }
    }
    return [...out].map((id) => id as SolidRef);
  }

  async sharedFacesBetween(a: SolidRef, b: SolidRef, model: Model): Promise<readonly FaceRef[]> {
    const ca = geom(model).solids[String(a)];
    const cb = geom(model).solids[String(b)];
    if (!ca || !cb) return [];
    const fa = new Set<string>();
    const fb = new Set<string>();
    for (const sid of ca.shellIds) {
      const sh = geom(model).shells[sid];
      if (sh) for (const fid of sh.faceIds) fa.add(fid);
    }
    for (const sid of cb.shellIds) {
      const sh = geom(model).shells[sid];
      if (sh) for (const fid of sh.faceIds) fb.add(fid);
    }
    const xs: FaceRef[] = [];
    for (const x of fa) if (fb.has(x)) xs.push(x as FaceRef);
    return xs;
  }

  async extrudeWire(input: { wireId: string; distance: number; direction: Vec3; model: Model }): Promise<SolidRef> {
    await this.ensureInit();
    const solid = extrudeModelWire(input.model, input.wireId, input.direction, input.distance);
    if (!solid) throw new Error(`Cannot extrude wire ${input.wireId}`);
    const ref = kernelGeometry.solidRef(`brepjs-solid-${++this.seq}`);
    this.solids.set(ref, solid);
    return ref;
  }

  async offsetFaces(input: { faceIds: readonly string[]; distance: number; model: Model }): Promise<void> {
    await this.offsetFacesDiff(input);
  }

}
// #endregion 🔌️BrepjsWasmEngine
