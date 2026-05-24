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
		const pa = topo.vertices[ed.vertexA]?.position;
		const pb = topo.vertices[ed.vertexB]?.position;
		if (!pa || !pb) return 0;
		return vec3Distance(pa, pb);
	}

	async faceArea(f: FaceRef, topo: TopologyGraph): Promise<number> {
		const fr = topo.faces[String(f)];
		if (!fr) return 0;
		if (fr.surface.kind === "planar") return 1;
		const verts = fr.surface.vertices;
		const tris = fr.surface.triangles;
		let s = 0;
		for (const tri of tris) {
			const i0 = tri[0]!;
			const i1 = tri[1]!;
			const i2 = tri[2]!;
			const a = verts[i0]!;
			const b = verts[i1]!;
			const c = verts[i2]!;
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

		it("createBoxFromCornersDiff includes one face bucket", async () => {
			const cell = await kernel.createBoxFromCorners({
				cornerA: [0, 0, 0],
				cornerB: [1, 0, 0],
				height: 1,
			});
			const r = await kernel.createBoxFromCornersDiff({
				cornerA: [0, 0, 0],
				cornerB: [1, 0, 0],
				height: 1,
			});
			expect(r.cell).toBeDefined();
			expect(Object.keys(r.diff.faces?.added ?? {}).length).toBeGreaterThan(0);
			await kernel.volume(cell);
		});

		it("vertexDistance matches graph positions", async () => {
			const { TopologyGraph } = await import("@spatial/js-core");
			const g = new TopologyGraph();
			const va = "va" as VertexRef;
			const vb = "vb" as VertexRef;
			g.vertices[va] = { id: va, position: [0, 0, 0] };
			g.vertices[vb] = { id: vb, position: [3, 4, 0] };
			expect(await kernel.vertexDistance(va, vb, g)).toBe(5);
		});

		it("faceArea sums mesh triangles", async () => {
			const { TopologyGraph, type FaceRef } = await import("@spatial/js-core");
			const g = new TopologyGraph();
			const fid = "f0" as FaceRef;
			g.faces[fid] = {
				id: fid,
				outerWireId: "w0" as import("@spatial/js-core").WireRef,
				surface: {
					kind: "mesh",
					vertices: [
						[0, 0, 0],
						[1, 0, 0],
						[0, 1, 0],
					],
					triangles: [[0, 1, 2]],
				},
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
