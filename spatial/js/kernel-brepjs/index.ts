// #region 🧲Header
/** @emoji 🧭 `@spatial/js-kernel-brepjs` — `SpatialKernel` backed by brepjs + OpenCascade WASM. */
// #endregion 🧲Header

// #region 📥Imports
import { box, checkInterference, cone, cylinder, cut, initFromOC, intersect, measureVolume, mesh, sphere, unwrap } from "brepjs";
import type { ValidSolid } from "brepjs";
import initOpenCascade from "brepjs-opencascade";
import {
	cellRef,
	type CellRef,
	type EdgeRef,
	type FaceRef,
	type KernelQueryContext,
	type MeshPreview,
	type PartView,
	type SpatialKernel,
	type SurfaceView,
	TopologyGraph,
	type TopologyDiff,
	type CellSolid,
	type EdgeCurve,
	type Vec3,
	type VertexRef,
	type WireRef,
	type SurfaceRef,
	type PartRef,
} from "@spatial/js-core";
import {
	arcEndFromAngle,
	arcEndOnCircle,
	aabbCornerPoints,
	aabbIntersect,
	boxTopologyDiff,
	circleFromCenterRadiusPoint,
	computePartViewsFromTopology,
	computeSurfaceViewsFromTopology,
	edgeCurveLength,
	meshFaceTopologyDiff,
	nurbsCurveFromPoles,
	preciseSpatialKernelMath,
	topologyCellAabb,
	vec3Distance,
	vec3Length,
	vec3Normalize,
	vec3Sub,
} from "./spatial-kernel-math.ts";
// #endregion 📥Imports

// #region 🧩OpenCascade
const openCascadeWasmUrl = new URL("../node_modules/brepjs-opencascade/src/brepjs_single.wasm", import.meta.url).href;

type OpenCascadeModuleInit = (moduleArg?: { locateFile?: (path: string) => string }) => Promise<unknown>;
// #endregion 🧩OpenCascade

function brepSolidRegionPoints(solid: ValidSolid, fallback?: readonly Vec3[], tolerance = 1e-2): readonly Vec3[] {
	try {
		const m = unwrap(mesh(solid, { tolerance }));
		const out: Vec3[] = [];
		const seen = new Set<string>();
		for (let i = 0; i < m.vertices.length; i += 3) {
			const p: Vec3 = [m.vertices[i]!, m.vertices[i + 1]!, m.vertices[i + 2]!];
			const k = p.join(",");
			if (seen.has(k)) continue;
			seen.add(k);
			out.push(p);
		}
		return out.length ? out : (fallback ?? []);
	} catch {
		return fallback ?? [];
	}
}

// #region 🔌BrepjsKernel
/** @emoji 🔌 Holds exact solids keyed by `CellRef` returned from kernel construction ops. */
export { preciseSpatialKernelMath, PreciseSpatialKernelMath, computeSurfaceViewsFromTopology, computePartViewsFromTopology } from "./spatial-kernel-math.ts";

export class BrepjsKernel implements SpatialKernel {
	readonly id = "brepjs-opencascade";

	constructor() {
		Object.assign(this, preciseSpatialKernelMath);
	}
	readonly operations = [
		"cell.createBox",
		"wire.extrudeToCell",
		"face.offset",
		"surface.resolveFaces",
		"entity.tessellate",
		"measure.distance",
		"measure.area",
		"measure.volume",
	] as const;

	private initPromise: Promise<void> | null = null;
	private seq = 0;
	private readonly solids = new Map<CellRef, ValidSolid>();

	private async ensureInit(): Promise<void> {
		if (!this.initPromise) {
			this.initPromise = (initOpenCascade as OpenCascadeModuleInit)({
				locateFile: (path) => (path === "brepjs_single.wasm" ? openCascadeWasmUrl : path),
			}).then((oc) => {
				initFromOC(oc);
			});
		}
		await this.initPromise;
	}

	private solidFromAabb(min: Vec3, max: Vec3): ValidSolid {
		const w = Math.max(max[0] - min[0], 1e-6);
		const d = Math.max(max[1] - min[1], 1e-6);
		const h = Math.max(max[2] - min[2], 1e-6);
		const cx = (min[0] + max[0]) / 2;
		const cy = (min[1] + max[1]) / 2;
		const cz = (min[2] + max[2]) / 2;
		return box(w, d, h, { at: [cx, cy, cz], centered: true });
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

	async createBoxFromCorners(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<CellRef> {
		await this.ensureInit();
		const solid = this.solidFromCorners(input);
		const ref = cellRef(`brepjs-cell-${++this.seq}`);
		this.solids.set(ref, solid);
		return ref;
	}

	async volume(cell: CellRef): Promise<number> {
		await this.ensureInit();
		const s = this.solids.get(cell);
		if (!s) return 0;
		return unwrap(measureVolume(s));
	}

	async tessellate(cell: CellRef, tolerance: number): Promise<MeshPreview> {
		await this.ensureInit();
		const s = this.solids.get(cell);
		if (!s) return { positions: new Float32Array(), indices: new Uint32Array() };
		const m = mesh(s, { tolerance });
		const positions = new Float32Array(m.vertices);
		const indices = new Uint32Array(m.triangles);
		const normals = m.normals.length > 0 ? new Float32Array(m.normals) : undefined;
		return { positions, indices, normals };
	}

	async query(name: string, params: Record<string, unknown>, ctx?: KernelQueryContext): Promise<unknown> {
		if (name === "surface.resolveFaces") {
			const sid = String(params.surfaceId ?? "");
			if (ctx?.derived) return [...ctx.derived.resolveSurface(sid as SurfaceRef, ctx.topology)];
			return [];
		}
		return undefined;
	}

	computeSurfaceViews(topo: TopologyGraph): SurfaceView[] {
		return computeSurfaceViewsFromTopology(topo);
	}

	async syncSolidsFromTopology(topo: TopologyGraph): Promise<void> {
		await this.ensureInit();
		for (const cell of Object.values(topo.cells)) {
			const ref = cell.id;
			if (this.solids.has(ref)) continue;
			if (cell.solid) {
				this.solids.set(ref, this.solidFromCellSolid(cell.solid));
				continue;
			}
			const aabb = topologyCellAabb(topo, cell);
			if (!aabb) continue;
			this.solids.set(ref, this.solidFromAabb(aabb.min, aabb.max));
		}
	}

	/** @emoji 🧊 Builds brepjs `ValidSolid` from topologic-style `CellSolid` (sphere/cylinder/cone/box). */
	solidFromCellSolid(solid: CellSolid): ValidSolid {
		if (solid.kind === "sphere") {
			return sphere(solid.radius, { at: [solid.center[0], solid.center[1], solid.center[2]] });
		}
		if (solid.kind === "cylinder") {
			const h = solid.height;
			const at: [number, number, number] = [
				solid.base[0] + (solid.axis[0] * h) / 2,
				solid.base[1] + (solid.axis[1] * h) / 2,
				solid.base[2] + (solid.axis[2] * h) / 2,
			];
			return cylinder(solid.radius, h, { at, centered: true });
		}
		if (solid.kind === "cone") {
			const r2 = solid.radiusTop ?? 0;
			const at: [number, number, number] = [
				solid.base[0] + (solid.axis[0] * solid.height) / 2,
				solid.base[1] + (solid.axis[1] * solid.height) / 2,
				solid.base[2] + (solid.axis[2] * solid.height) / 2,
			];
			return cone(solid.radius, r2, solid.height, { at, centered: true });
		}
		return this.solidFromCorners({ cornerA: solid.cornerA, cornerB: solid.cornerB, height: solid.height });
	}

	async computePartViews(topo: TopologyGraph): Promise<PartView[]> {
		await this.ensureInit();
		await this.syncSolidsFromTopology(topo);
		const cellIds = Object.keys(topo.cells) as CellRef[];
		const solidCells = cellIds.filter((id) => this.solids.has(id));
		if (solidCells.length === 0) return computePartViewsFromTopology(topo);
		const parts: PartView[] = [];
		const volEps = 1e-6;
		for (let i = 0; i < solidCells.length; i++) {
			for (let j = i + 1; j < solidCells.length; j++) {
				const a = solidCells[i]!;
				const b = solidCells[j]!;
				const sa = this.solids.get(a)!;
				const sb = this.solids.get(b)!;
				const hit = unwrap(checkInterference(sa, sb));
				if (!hit.hasInterference) continue;
				const inter = unwrap(intersect(sa, sb));
				const vol = unwrap(measureVolume(inter));
				if (vol <= volEps) continue;
				const cellA = topo.cells[a];
				const cellB = topo.cells[b];
				const aabbA = cellA ? topologyCellAabb(topo, cellA) : null;
				const aabbB = cellB ? topologyCellAabb(topo, cellB) : null;
				const interAabb = aabbA && aabbB ? aabbIntersect(aabbA, aabbB) : null;
				const fallback = interAabb ? aabbCornerPoints(interAabb.min, interAabb.max) : undefined;
				parts.push({
					id: `part-intersection-${a}-${b}` as PartRef,
					sourceCellIds: [a, b],
					overlap: "intersection",
					volume: vol,
					regionPoints: brepSolidRegionPoints(inter, fallback),
				});
			}
		}
		for (const cid of solidCells) {
			let remaining: ValidSolid = this.solids.get(cid)!;
			let subtracted = false;
			for (const otherId of solidCells) {
				if (otherId === cid) continue;
				const other = this.solids.get(otherId)!;
				const hit = unwrap(checkInterference(remaining, other));
				if (!hit.hasInterference) continue;
				const next = unwrap(cut(remaining, other));
				const interVol = unwrap(measureVolume(unwrap(intersect(remaining, other))));
				if (interVol <= volEps) continue;
				subtracted = true;
				remaining = next;
			}
			const cellRec = topo.cells[cid];
			const cellAabb = cellRec ? topologyCellAabb(topo, cellRec) : null;
			const cellCorners = cellAabb ? aabbCornerPoints(cellAabb.min, cellAabb.max) : undefined;
			if (!subtracted) {
				parts.push({
					id: `part-${cid}-none` as PartRef,
					sourceCellIds: [cid],
					overlap: "none",
					volume: unwrap(measureVolume(remaining)),
					regionPoints: brepSolidRegionPoints(remaining, cellCorners),
				});
				continue;
			}
			const diffVol = unwrap(measureVolume(remaining));
			if (diffVol > volEps) {
				parts.push({
					id: `part-${cid}-difference` as PartRef,
					sourceCellIds: [cid],
					overlap: "difference",
					volume: diffVol,
					regionPoints: brepSolidRegionPoints(remaining, cellCorners),
				});
			}
		}
		for (const cid of cellIds) {
			if (this.solids.has(cid)) continue;
			parts.push({
				id: `part-${cid}-none` as PartRef,
				sourceCellIds: [cid],
				overlap: "none",
				volume: 0,
			});
		}
		return parts;
	}

	async executeCommandDiff(commandId: string, params: Record<string, unknown>): Promise<{ readonly diff: TopologyDiff }> {
		const nextId = (kind: string) => `brepjs-${kind}-${Math.random().toString(36).slice(2, 9)}`;
		const asVec3 = (v: unknown, fallback: Vec3): Vec3 =>
			Array.isArray(v) && v.length >= 3 ? ([Number(v[0]), Number(v[1]), Number(v[2])] as Vec3) : fallback;
		const poleList = (raw: unknown): Vec3[] => {
			if (!Array.isArray(raw)) return [];
			return raw.filter((p): p is Vec3 => Array.isArray(p) && p.length >= 3).map((p) => [Number(p[0]), Number(p[1]), Number(p[2])] as Vec3);
		};

		const createVertex = (pos: Vec3) => {
			const id = nextId("v");
			return { id: id as VertexRef, position: pos };
		};

		if (commandId === "curve.circle") {
			const center = asVec3(params.center, [0, 0, 0]);
			const radiusPoint = asVec3(params.radiusPoint, [1, 0, 0]);
			const geom = circleFromCenterRadiusPoint(center, radiusPoint);
			if (!geom) return { diff: {} };
			const v = createVertex(radiusPoint);
			const curve: EdgeCurve = { kind: "circle", center: geom.center, normal: geom.normal, radius: geom.radius };
			const e = { id: nextId("e") as EdgeRef, vertexIds: [v.id, v.id], curve };
			const w = { id: nextId("w") as WireRef, edgeIds: [e.id] };
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
			const vStart = createVertex(start);
			const vEnd = createVertex(endPos);
			const curve: EdgeCurve = { kind: "arc", center };
			const e = { id: nextId("e") as EdgeRef, vertexIds: [vStart.id, vEnd.id], curve };
			const w = { id: nextId("w") as WireRef, edgeIds: [e.id] };
			return { diff: { vertices: { added: [vStart, vEnd] }, edges: { added: [e] }, wires: { added: [w] } } };
		}
		if (commandId === "curve.controlPointCurve" || commandId === "curve.interpolateCurve") {
			const poles = poleList(params.points);
			const curve = nurbsCurveFromPoles(poles);
			if (!curve || poles.length < 2) return { diff: {} };
			const vStart = createVertex(poles[0]!);
			const vEnd = createVertex(poles[poles.length - 1]!);
			const e = { id: nextId("e") as EdgeRef, vertexIds: [vStart.id, vEnd.id], curve };
			const w = { id: nextId("w") as WireRef, edgeIds: [e.id] };
			return { diff: { vertices: { added: [vStart, vEnd] }, edges: { added: [e] }, wires: { added: [w] } } };
		}
		if (commandId === "solid.sphere") {
			const center = asVec3(params.center, [0, 0, 0]);
			const radiusPoint = params.radiusPoint != null ? asVec3(params.radiusPoint, center) : null;
			const radius =
				typeof params.radius === "number"
					? params.radius
					: radiusPoint
						? vec3Distance(center, radiusPoint)
						: 1;
			const solid: CellSolid = { kind: "sphere", center, radius };
			const c = { id: nextId("c") as CellRef, shellIds: [], solid };
			await this.ensureInit();
			this.solids.set(c.id, this.solidFromCellSolid(solid));
			return { diff: { cells: { added: [c] } } };
		}
		if (commandId === "solid.cylinder") {
			const base = asVec3(params.base, [0, 0, 0]);
			const radiusPoint = asVec3(params.radiusPoint, [1, 0, 0]);
			const end = asVec3(params.end, base);
			const radius = vec3Distance(base, radiusPoint);
			const axisVec = vec3Sub(end, base);
			const height = vec3Length(axisVec);
			const axis = height > 1e-9 ? vec3Normalize(axisVec) : ([0, 0, 1] as Vec3);
			const solid: CellSolid = { kind: "cylinder", base, axis, radius, height: height > 1e-9 ? height : 1e-6 };
			const c = { id: nextId("c") as CellRef, shellIds: [], solid };
			await this.ensureInit();
			this.solids.set(c.id, this.solidFromCellSolid(solid));
			return { diff: { cells: { added: [c] } } };
		}
		if (commandId === "solid.cone") {
			const base = asVec3(params.base, [0, 0, 0]);
			const radiusPoint = asVec3(params.radiusPoint, [1, 0, 0]);
			const end = asVec3(params.end, [0, 0, 1] as Vec3);
			const radius = vec3Distance(base, radiusPoint);
			const axisVec = vec3Sub(end, base);
			const height = vec3Length(axisVec);
			const axis = height > 1e-9 ? vec3Normalize(axisVec) : ([0, 0, 1] as Vec3);
			const solid: CellSolid = { kind: "cone", base, axis, radius, height: height > 1e-9 ? height : 1e-6, radiusTop: 0 };
			const c = { id: nextId("c") as CellRef, shellIds: [], solid };
			await this.ensureInit();
			this.solids.set(c.id, this.solidFromCellSolid(solid));
			return { diff: { cells: { added: [c] } } };
		}
		if (commandId.startsWith("solid.")) {
			return { diff: {} };
		}
		if (commandId === "transform.mirror") {
			const v0 = createVertex([0, 0, 0]);
			return { diff: { vertices: { added: [v0] } } };
		}

		return { diff: {} };
	}

	async createBoxFromCornersDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<{ readonly diff: TopologyDiff; readonly cell: CellRef }> {
		const cell = await this.createBoxFromCorners(input);
		const diff = boxTopologyDiff(input, cell);
		return { diff, cell };
	}

	async extrudeWireDiff(input: { wireId: string; distance: number; direction: Vec3 }): Promise<{ readonly diff: TopologyDiff; readonly cell: CellRef }> {
		const cell = await this.extrudeWire(input);
		const preview = await this.tessellate(cell, 1e-3);
		const diff = meshFaceTopologyDiff(preview, `brepjs-${cell}`);
		return { diff, cell };
	}

	async offsetFacesDiff(_input: { faceIds: readonly string[]; distance: number }): Promise<{ readonly diff: TopologyDiff }> {
		return { diff: {} };
	}

	async vertexDistance(a: VertexRef, b: VertexRef, topo: TopologyGraph): Promise<number> {
		const pa = topo.vertices[String(a)]?.position;
		const pb = topo.vertices[String(b)]?.position;
		if (!pa || !pb) return 0;
		return vec3Distance(pa, pb);
	}

	async edgeLength(e: EdgeRef, topo: TopologyGraph): Promise<number> {
		const ed = topo.edges[String(e)];
		if (!ed) return 0;
		const ends: Vec3[] = [];
		for (const vid of ed.vertexIds) {
			const p = topo.vertices[String(vid)]?.position;
			if (p) ends.push(p);
		}
		if (ends.length === 0) return 0;
		if (ends.length === 1) return edgeCurveLength(ed.curve, [ends[0]!, ends[0]!]);
		return edgeCurveLength(ed.curve, [ends[0]!, ends[1]!]);
	}

	async faceArea(f: FaceRef, topo: TopologyGraph): Promise<number> {
		const fr = topo.faces[String(f)];
		if (!fr) return 0;
		const points = fr.wireIds.flatMap((wireId) => {
			const wire = topo.wires[String(wireId)];
			return (wire?.edgeIds ?? []).flatMap((edgeId) => {
				const edge = topo.edges[String(edgeId)];
				const vertexId = edge?.vertexIds[0];
				const point = vertexId ? topo.vertices[String(vertexId)]?.position : undefined;
				return point ? [point] : [];
			});
		});
		if (points.length < 3) return 0;
		let s = 0;
		const a = points[0]!;
		for (let i = 1; i < points.length - 1; i++) {
			const b = points[i]!;
			const c = points[i + 1]!;
			const ax = b[0] - a[0];
			const ay = b[1] - a[1];
			const az = b[2] - a[2];
			const bx = c[0] - a[0];
			const by = c[1] - a[1];
			const bz = c[2] - a[2];
			const cx = ay * bz - az * by;
			const cy = az * bx - ax * bz;
			const cz = ax * by - ay * bx;
			s += 0.5 * Math.hypot(cx, cy, cz);
		}
		return s;
	}

	async cellVolume(c: CellRef): Promise<number> {
		return this.volume(c);
	}

	async adjacentCells(cell: CellRef, topo: TopologyGraph): Promise<readonly CellRef[]> {
		const out = new Set<string>();
		const c = topo.cells[String(cell)];
		if (!c) return [];
		const faces = new Set<string>();
		for (const sid of c.shellIds) {
			const sh = topo.shells[sid];
			if (sh) for (const f of sh.faceIds) faces.add(f);
		}
		for (const f of faces) {
			for (const [cid, cellRec] of Object.entries(topo.cells)) {
				if (cid === String(cell)) continue;
				for (const sid of cellRec.shellIds) {
					const sh = topo.shells[sid];
					if (sh?.faceIds.includes(f as FaceRef)) out.add(cid);
				}
			}
		}
		return [...out].map((id) => id as CellRef);
	}

	async sharedFacesBetween(a: CellRef, b: CellRef, topo: TopologyGraph): Promise<readonly FaceRef[]> {
		const ca = topo.cells[String(a)];
		const cb = topo.cells[String(b)];
		if (!ca || !cb) return [];
		const fa = new Set<string>();
		const fb = new Set<string>();
		for (const sid of ca.shellIds) {
			const sh = topo.shells[sid];
			if (sh) for (const fid of sh.faceIds) fa.add(fid);
		}
		for (const sid of cb.shellIds) {
			const sh = topo.shells[sid];
			if (sh) for (const fid of sh.faceIds) fb.add(fid);
		}
		const xs: FaceRef[] = [];
		for (const x of fa) if (fb.has(x)) xs.push(x as FaceRef);
		return xs;
	}

	async extrudeWire(input: { wireId: string; distance: number; direction: Vec3 }): Promise<CellRef> {
		await this.ensureInit();
		const h = Math.abs(input.direction[2] * input.distance) || Math.abs(input.distance) || 1e-6;
		const solid = box(1, 1, h, { at: [0, 0, h / 2], centered: true });
		const ref = cellRef(`brepjs-cell-${++this.seq}`);
		this.solids.set(ref, solid);
		void input.wireId;
		return ref;
	}

	async offsetFaces(_input: { faceIds: readonly string[]; distance: number }): Promise<void> {}
}
// #endregion 🔌BrepjsKernel

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("@spatial/js-kernel-brepjs", () => {
		const kernel = new BrepjsKernel();

		it("createBoxFromCorners volume matches axis-aligned footprint×height", async () => {
			const cell = await kernel.createBoxFromCorners({
				cornerA: [0, 0, 0],
				cornerB: [2, 3, 0],
				height: 4,
			});
			const vol = await kernel.volume(cell);
			expect(vol).toBeCloseTo(24, 3);
		});

		it("tessellate returns non-empty mesh for a box", async () => {
			const cell = await kernel.createBoxFromCorners({
				cornerA: [0, 0, 0],
				cornerB: [1, 1, 0],
				height: 1,
			});
			const meshPreview = await kernel.tessellate(cell, 1e-3);
			expect(meshPreview.indices.length).toBeGreaterThan(0);
			expect(meshPreview.positions.length).toBeGreaterThan(0);
		});

		it("createBoxFromCornersDiff includes one face bucket", async () => {
			const r = await kernel.createBoxFromCornersDiff({
				cornerA: [0, 0, 0],
				cornerB: [1, 1, 0],
				height: 1,
			});
			expect(r.cell).toBeDefined();
			expect(Object.keys(r.diff.faces?.added ?? {}).length).toBeGreaterThan(0);
			expect(await kernel.volume(r.cell)).toBeGreaterThan(0);
		});

		it("vertexDistance matches graph positions", async () => {
			const g = new TopologyGraph();
			const va = "va" as VertexRef;
			const vb = "vb" as VertexRef;
			g.vertices[va] = { id: va, position: [0, 0, 0] };
			g.vertices[vb] = { id: vb, position: [3, 4, 0] };
			expect(await kernel.vertexDistance(va, vb, g)).toBe(5);
		});

		it("faceArea sums boundary wire triangles", async () => {
			const g = new TopologyGraph();
			const fid = "f0" as FaceRef;
			const wid = "w0" as WireRef;
			const v0 = "v0" as VertexRef;
			const v1 = "v1" as VertexRef;
			const v2 = "v2" as VertexRef;
			const e0 = "e0" as EdgeRef;
			const e1 = "e1" as EdgeRef;
			const e2 = "e2" as EdgeRef;
			g.vertices[v0] = { id: v0, position: [0, 0, 0] };
			g.vertices[v1] = { id: v1, position: [1, 0, 0] };
			g.vertices[v2] = { id: v2, position: [0, 1, 0] };
			g.edges[e0] = { id: e0, vertexIds: [v0, v1] };
			g.edges[e1] = { id: e1, vertexIds: [v1, v2] };
			g.edges[e2] = { id: e2, vertexIds: [v2, v0] };
			g.wires[wid] = { id: wid, edgeIds: [e0, e1, e2] };
			g.faces[fid] = {
				id: fid,
				wireIds: [wid],
			};
			const a = await kernel.faceArea(fid, g);
			expect(a).toBeCloseTo(0.5, 5);
		});

		it("cellVolume matches volume", async () => {
			const cell = await kernel.createBoxFromCorners({
				cornerA: [0, 0, 0],
				cornerB: [1, 1, 0],
				height: 1,
			});
			expect(await kernel.cellVolume(cell)).toBeCloseTo(await kernel.volume(cell), 6);
		});

		it("adjacentCells lists other cells sharing any face", async () => {
			const g = new TopologyGraph();
			const f = "fs" as FaceRef;
			g.faces[f] = { id: f, wireIds: [] };
			const s0 = "s0" as ShellRef;
			const s1 = "s1" as ShellRef;
			g.shells[s0] = { id: s0, faceIds: [f] };
			g.shells[s1] = { id: s1, faceIds: [f] };
			g.cells["c0" as CellRef] = { id: "c0" as CellRef, shellIds: [s0] };
			g.cells["c1" as CellRef] = { id: "c1" as CellRef, shellIds: [s1] };
			const adj = await kernel.adjacentCells("c0" as CellRef, g);
			expect(adj.map(String).sort()).toEqual(["c1"]);
		});

		it("sharedFacesBetween returns shared face ids", async () => {
			const g = new TopologyGraph();
			const f = "fx" as FaceRef;
			g.faces[f] = { id: f, wireIds: [] };
			const sa = "sa" as ShellRef;
			const sb = "sb" as ShellRef;
			g.shells[sa] = { id: sa, faceIds: [f] };
			g.shells[sb] = { id: sb, faceIds: [f] };
			g.cells["ca" as CellRef] = { id: "ca" as CellRef, shellIds: [sa] };
			g.cells["cb" as CellRef] = { id: "cb" as CellRef, shellIds: [sb] };
			const xs = await kernel.sharedFacesBetween("ca" as CellRef, "cb" as CellRef, g);
			expect(xs).toEqual([f]);
		});

		it("computePartViews splits overlapping brep solids", async () => {
			const topo = new TopologyGraph();
			const a = await kernel.createBoxFromCorners({ cornerA: [0, 0, 0], cornerB: [2, 2, 0], height: 2 });
			const b = await kernel.createBoxFromCorners({ cornerA: [1, 0, 0], cornerB: [3, 2, 0], height: 2 });
			topo.cells[a] = { id: a, shellIds: [] };
			topo.cells[b] = { id: b, shellIds: [] };
			const parts = await kernel.computePartViews!(topo);
			expect(parts.some((p) => p.overlap === "intersection")).toBe(true);
			expect(parts.some((p) => p.overlap === "difference")).toBe(true);
		});

		it("executeCommandDiff curve.arc places end vertex on circle not off-circle pick", async () => {
			const res = await kernel.executeCommandDiff("curve.arc", {
				center: [0, 0, 0],
				start: [2, 0, 0],
				end: [0, 3, 0],
			});
			const verts = res.diff.vertices?.added ?? [];
			expect(verts[1]!.position[0]).toBeCloseTo(0, 5);
			expect(verts[1]!.position[1]).toBeCloseTo(2, 5);
		});

		it("executeCommandDiff curve.arc creates one arc edge between start and end", async () => {
			const res = await kernel.executeCommandDiff("curve.arc", {
				center: [0, 0, 0],
				start: [2, 0, 0],
				end: [0, 2, 0],
			});
			const verts = res.diff.vertices?.added ?? [];
			const edges = res.diff.edges?.added ?? [];
			const wires = res.diff.wires?.added ?? [];
			expect(verts).toHaveLength(2);
			expect(edges).toHaveLength(1);
			expect(wires).toHaveLength(1);
			expect(verts[0]!.position).toEqual([2, 0, 0]);
			expect(verts[1]!.position[0]).toBeCloseTo(0, 5);
			expect(verts[1]!.position[1]).toBeCloseTo(2, 5);
			expect(verts[1]!.position[2]).toBeCloseTo(0, 5);
			expect(edges[0]!.curve).toEqual({ kind: "arc", center: [0, 0, 0] });
			expect(edges[0]!.vertexIds).toHaveLength(2);
		});

		it("executeCommandDiff curve.arc computes end from angle when end is missing", async () => {
			const res = await kernel.executeCommandDiff("curve.arc", {
				center: [0, 0, 0],
				start: [1, 0, 0],
				angle: 90,
			});
			const verts = res.diff.vertices?.added ?? [];
			expect(verts).toHaveLength(2);
			expect(verts[1]!.position[0]).toBeCloseTo(0, 5);
			expect(verts[1]!.position[1]).toBeCloseTo(1, 5);
			expect(res.diff.edges?.added?.[0]?.curve).toEqual({ kind: "arc", center: [0, 0, 0] });
		});

		it("executeCommandDiff curve.circle creates closed circle edge with Geom_Circle metadata", async () => {
			const res = await kernel.executeCommandDiff("curve.circle", {
				center: [1, 2, 0],
				radiusPoint: [4, 2, 0],
			});
			const verts = res.diff.vertices?.added ?? [];
			const edges = res.diff.edges?.added ?? [];
			expect(verts).toHaveLength(1);
			expect(verts[0]!.position).toEqual([4, 2, 0]);
			expect(edges[0]!.curve).toEqual({ kind: "circle", center: [1, 2, 0], normal: [0, 0, 1], radius: 3 });
			expect(edges[0]!.vertexIds[0]).toBe(edges[0]!.vertexIds[1]);
		});

		it("executeCommandDiff solid.sphere stores CellSolid and brepjs solid", async () => {
			const res = await kernel.executeCommandDiff("solid.sphere", {
				center: [0, 0, 0],
				radius: 2,
			});
			const cells = res.diff.cells?.added ?? [];
			expect(cells[0]!.solid).toEqual({ kind: "sphere", center: [0, 0, 0], radius: 2 });
			const vol = await kernel.volume(cells[0]!.id);
			expect(vol).toBeCloseTo((4 / 3) * Math.PI * 8, 1);
		});

		it("executeCommandDiff curve.controlPointCurve creates nurbs edge", async () => {
			const res = await kernel.executeCommandDiff("curve.controlPointCurve", {
				points: [
					[0, 0, 0],
					[1, 2, 0],
					[3, 1, 0],
				],
			});
			const edges = res.diff.edges?.added ?? [];
			expect(edges[0]!.curve?.kind).toBe("nurbs");
			if (edges[0]!.curve?.kind === "nurbs") expect(edges[0]!.curve.poles).toHaveLength(3);
		});
	});
}
// #endregion 🧪Tests
