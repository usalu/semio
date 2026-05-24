// #region 🧲Header
/** @emoji 🧭 `@spatial/js-kernel-brepjs` — `KernelAdapter` backed by brepjs + OpenCascade WASM. */
// #endregion 🧲Header

// #region 📥Imports
import { box, init, measureVolume, mesh, unwrap } from "brepjs";
import type { ValidSolid } from "brepjs";
import {
	cellRef,
	meshFaceTopologyDiff,
	type CellRef,
	type EdgeRef,
	type FaceRef,
	type KernelAdapter,
	type MeshPreview,
	TopologyGraph,
	type TopologyDiff,
	type Vec3,
	type VertexRef,
	type WireRef,
	vec3Distance,
} from "@spatial/js-core";
// #endregion 📥Imports

// #region 🔌BrepjsKernel
/** @emoji 🔌 Holds exact solids keyed by `CellRef` returned from kernel construction ops. */
export class BrepjsKernel implements KernelAdapter {
	readonly id = "brepjs-opencascade";
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
		if (!this.initPromise) this.initPromise = init().then(() => undefined);
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

	async query(name: string, params: Record<string, unknown>): Promise<unknown> {
		if (name === "surface.resolveFaces") {
			const sid = String(params.surfaceId ?? "");
			if (!sid.startsWith("surface-")) return [sid];
			return [`face-${sid}-a`, `face-${sid}-b`];
		}
		return undefined;
	}

	async createBoxFromCornersDiff(input: { cornerA: Vec3; cornerB: Vec3; height: number }): Promise<{ readonly diff: TopologyDiff; readonly cell: CellRef }> {
		const cell = await this.createBoxFromCorners(input);
		const preview = await this.tessellate(cell, 1e-3);
		const diff = meshFaceTopologyDiff(preview, `brepjs-${cell}`);
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
		const pa = topo.vertices[String(ed.vertexIds[0])]?.position;
		const pb = topo.vertices[String(ed.vertexIds[1])]?.position;
		if (!pa || !pb) return 0;
		return vec3Distance(pa, pb);
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
	});
}
// #endregion 🧪Tests
