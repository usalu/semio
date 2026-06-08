// #region 🧲Header
/// <reference types="vite/client" />
/// <reference types="vitest/importMeta" />
/** @emoji 🧭 `@geometry/brep/js` — cad-free brepjs + OpenCascade kernel and contracts. */
// #endregion 🧲Header

export * from "./contracts.ts";
export * from "./mesh.ts";
export {
	BrepjsGeometryKernel,
	GeometryBrepPreviewKernel,
	brepjsGeometryKernel,
	geometryBrepPreviewKernel,
	ensureBrepWasmLoaded,
	vec3Add,
	vec3Sub,
	vec3Scale,
	vec3Dot,
	vec3Cross,
	vec3Length,
	vec3Distance,
	vec3Normalize,
	constrainMovePoint,
} from "./kernel.ts";

// #region 🧪Tests
if (import.meta.vitest) {
	const { beforeEach, describe, expect, it } = import.meta.vitest;
	const { BrepjsGeometryKernel, ensureBrepWasmLoaded } = await import("./kernel.ts");
	const { isRenderableMeshTransfer } = await import("./mesh.ts");

	describe("@geometry/brep/js", () => {
		const kernel = new BrepjsGeometryKernel();

		beforeEach(() => {
			kernel.resetForTest();
		});

		it("boxSync volume matches width×depth×height", async () => {
			await ensureBrepWasmLoaded();
			const solid = kernel.boxSync(2, 3, 4);
			expect(kernel.measureVolumeSync(solid)).toBeCloseTo(2 * 3 * 4, 1);
		});

		it("tessellateGeometry returns renderable mesh", async () => {
			await ensureBrepWasmLoaded();
			const solid = kernel.spherePrimSync(1);
			const mesh = await kernel.tessellateGeometry(solid, 0.05);
			expect(isRenderableMeshTransfer(mesh)).toBe(true);
		});

		it("fuseAllSync combines volumes", async () => {
			await ensureBrepWasmLoaded();
			const a = kernel.boxSync(1, 1, 1);
			const b = kernel.boxSync(1, 1, 1, [0.5, 0, 0]);
			const fused = kernel.fuseAllSync([a, b]);
			expect(kernel.measureVolumeSync(fused)).toBeGreaterThan(1);
		});

		it("line curve tessellates as edges", async () => {
			await ensureBrepWasmLoaded();
			const line = kernel.lineSync([0, 0, 0], [2, 0, 0]);
			const mesh = await kernel.tessellateGeometry(line, 0.05);
			expect(isRenderableMeshTransfer(mesh)).toBe(true);
			expect(mesh.edges.length).toBeGreaterThan(0);
		});

		it("fuseSync boolean registers fused solid", async () => {
			await ensureBrepWasmLoaded();
			const a = kernel.boxSync(1, 1, 1);
			const b = kernel.boxSync(1, 1, 1, [0.5, 0, 0]);
			const fused = kernel.fuseSync(a, b);
			expect(String(fused).startsWith("solid-")).toBe(true);
		});

		it("curvePointAt evaluates on line", async () => {
			await ensureBrepWasmLoaded();
			const line = kernel.lineSync([0, 0, 0], [4, 0, 0]);
			const pt = kernel.curvePointAtSync(line, 0.5);
			expect(pt[0]).toBeCloseTo(2, 1);
		});

		it("drawCircle registers previewable profile drawing", async () => {
			await ensureBrepWasmLoaded();
			const profile = kernel.drawCircleSync(1);
			expect(kernel.getGeometryKind(profile)).toBe("drawing");
			const mesh = await kernel.tessellateGeometry(profile, 0.05);
			expect(isRenderableMeshTransfer(mesh)).toBe(true);
			expect(mesh.edges.length).toBeGreaterThan(0);
		});

		it("sketch rectangle and extrude share centered footprint in preview", async () => {
			await ensureBrepWasmLoaded();
			const profile = kernel.sketchRectangleSync(4, 3);
			expect(kernel.getGeometryKind(profile)).toBe("drawing");
			const profileMesh = await kernel.tessellateGeometry(profile, 0.05);
			expect(isRenderableMeshTransfer(profileMesh)).toBe(true);
			expect(profileMesh.index.length).toBe(0);
			expect(profileMesh.edges.length).toBeGreaterThan(0);
			const profileBounds = kernel.getBoundsSync(profile);
			const solid = kernel.extrudeSync(profile, [0, 0, 1], 5);
			expect(kernel.getGeometryKind(solid)).toBe("solid");
			const solidMesh = await kernel.tessellateGeometry(solid, 0.05);
			expect(isRenderableMeshTransfer(solidMesh)).toBe(true);
			const solidBounds = kernel.getBoundsSync(solid);
			const prim = kernel.boxSync(4, 3, 5);
			const primBounds = kernel.getBoundsSync(prim);
			expect(profileBounds.min[0]).toBeCloseTo(solidBounds.min[0], 3);
			expect(profileBounds.max[0]).toBeCloseTo(solidBounds.max[0], 3);
			expect(profileBounds.min[1]).toBeCloseTo(solidBounds.min[1], 3);
			expect(profileBounds.max[1]).toBeCloseTo(solidBounds.max[1], 3);
			for (let axis = 0; axis < 3; axis += 1) {
				expect(solidBounds.min[axis]).toBeCloseTo(primBounds.min[axis]!, 5);
				expect(solidBounds.max[axis]).toBeCloseTo(primBounds.max[axis]!, 5);
			}
		});

		it("sketch and curve circle extrude tessellate in preview", async () => {
			await ensureBrepWasmLoaded();
			for (const profile of [kernel.sketchCircleSync(2), kernel.circleCurveSync(2), kernel.drawCircleSync(1)]) {
				const solid = kernel.extrudeSync(profile, [0, 0, 1], 5);
				expect(kernel.getGeometryKind(solid)).toBe("solid");
				const solidMesh = await kernel.tessellateGeometry(solid, 0.05);
				expect(isRenderableMeshTransfer(solidMesh)).toBe(true);
			}
		});

		it("makeExternalGear registers solid handle", async () => {
			await ensureBrepWasmLoaded();
			const gear = kernel.makeExternalGearSync(20, 3);
			expect(String(gear).startsWith("solid-")).toBe(true);
			expect(kernel.getGeometryKind(gear)).toBe("solid");
		});
	});
}
// #endregion 🧪Tests
