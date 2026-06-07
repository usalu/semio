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

		it("createBoxFromCorners volume matches footprint×height", async () => {
			await ensureBrepWasmLoaded();
			const solid = await kernel.createBoxFromCorners({
				cornerA: [0, 0, 0],
				cornerB: [2, 3, 0],
				height: 4,
			});
			const vol = await kernel.volume(solid);
			expect(vol).toBeCloseTo(2 * 3 * 4, 1);
		});

		it("tessellate returns renderable mesh", async () => {
			await ensureBrepWasmLoaded();
			const solid = await kernel.createSphere([0, 0, 0], 1);
			const mesh = await kernel.tessellate(solid, 0.05);
			expect(isRenderableMeshTransfer(mesh)).toBe(true);
		});

		it("fuseSolids combines volumes", async () => {
			await ensureBrepWasmLoaded();
			const a = await kernel.createBoxFromCorners({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 });
			const b = await kernel.createBoxFromCorners({ cornerA: [0.5, 0, 0], cornerB: [1.5, 1, 0], height: 1 });
			const fused = await kernel.fuseSolids([a, b]);
			const vol = await kernel.volume(fused);
			expect(vol).toBeGreaterThan(1);
		});
	});
}
// #endregion 🧪Tests
