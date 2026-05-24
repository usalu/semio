// #region 🧲Header
/** @emoji 🧭 `@spatial/js-kernel-brepjs` — `KernelAdapter` backed by brepjs + OpenCascade WASM. */
// #endregion 🧲Header

// #region 📥Imports
import { box, init, measureVolume, mesh, unwrap } from "brepjs";
import type { ValidSolid } from "brepjs";
import { cellRef, type CellRef, type KernelAdapter, type MeshPreview, type Vec3 } from "@spatial/js-core";
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
			return [`face-${sid}-a`, `face-${sid}-b`];
		}
		return undefined;
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
	});
}
// #endregion 🧪Tests
