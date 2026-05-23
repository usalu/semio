import { readFileSync, writeFileSync } from "node:fs";

const NEW_TESTS = `if (import.meta.vitest) {
	const { describe, it, expect, beforeAll } = await import("vitest");
	const M = await import("@spatial/js-kernel-brepjs/testing");

	describe("@spatial/js-renderer-r3f interaction adapter", () => {
		it("keeps active spatial ground picks enabled when host geometry selection is disabled", () => {
			const snapshot = {
				state: "first_corner",
				spatialInteraction: {
					spatialGroundPick: true,
					pickDisabledStates: ["idle", "ready", "committed"],
					groundPointerMoveStates: ["first_corner"],
					heightDragStates: [],
					verticalRodStates: [],
					heightConfirmState: null,
				},
			} satisfies Pick<InteractionSnapshot, "state" | "spatialInteraction">;
			expect(interactionSpatialGroundPickPlaneEnabled(snapshot, true, [])).toBe(true);
			expect(interactionSpatialGroundPickPlaneEnabled(snapshot, true, ["vertex"])).toBe(false);
		});

		it("creates snap and selection metadata for geometry targets", () => {
			const targets = createSpatialPickTargets({
				schema: "spatial.model/v1",
				revision: 1,
				anchors: [],
				vertices: [{ id: "v0", position: [1, 2, 3] }],
				edges: [],
				wires: [],
				faces: [],
				shells: [],
				cells: [],
				cellComplexes: [],
				clusters: [],
			} as unknown as ModelJson);
			expect(targets).toEqual([{ kind: "objectVertex", geometryKind: "vertex", id: "v0", point: [1, 2, 3] }]);
			expect(createSpatialPickEvent("pointer.down", [9, 9, 9], targets[0]!, { shift: true })).toEqual({
				kind: "pointer.down",
				point: [9, 9, 9],
				modifiers: { shift: true },
				snap: { kind: "vertex", id: "v0", point: [1, 2, 3] },
				selection: { kind: "vertex", id: "v0" },
			});
		});

		it("adds extension view object picks when activeViewId is set", async () => {
			const model = new Model();
			applyModelDiff(model, M.boxModelDiff({ cornerA: [0, 0, 0], cornerB: [1, 1, 0], height: 1 }, cellRef("c0")));
			const views = ExtensionViewService.forKernel(new BrepjsKernel() as unknown as SpatialKernel);
			const activeViewId = qualifiedViewId("energy", "energy");
			await views.refresh(model, activeViewId);
			const editTargets = createSpatialPickTargets(model, views, null);
			const viewTargets = createSpatialPickTargets(model, views, activeViewId);
			expect(editTargets.some((t) => t.kind === "objectVertex")).toBe(true);
			expect(viewTargets.every((t) => t.kind === "object")).toBe(true);
			expect(viewTargets.length).toBeGreaterThan(0);
		});

		it("filterSpatialPickTargetsForActiveView scopes kernel vs view objects", () => {
			const targets: SpatialPickTarget[] = [
				{ kind: "objectVertex", geometryKind: "vertex", id: "v0", point: [0, 0, 0] },
				{ kind: "object", id: "energy.energy.hull", point: [0.5, 0.5, 0.5] },
			];
			expect(filterSpatialPickTargetsForActiveView(targets, null).map(spatialPickTargetKey)).toEqual(["objectVertex:v0"]);
			expect(filterSpatialPickTargetsForActiveView(targets, "energy.energy").map(spatialPickTargetKey)).toEqual([
				"object:energy.energy.hull",
			]);
		});

		it("resolveSpatialSceneVisibility switches edit wireframe vs committed view mesh", () => {
			expect(resolveSpatialSceneVisibility(null, { objectEdge: true, objectFace: true })).toEqual({
				showFactoryWireframe: true,
				showCommittedFaces: true,
				showCommittedEdges: true,
			});
			expect(resolveSpatialSceneVisibility("energy.energy", { object: true })).toEqual({
				showFactoryWireframe: false,
				showCommittedFaces: true,
				showCommittedEdges: true,
			});
		});

		it("defaultInteractionReplChromeState seeds geometry edit by default", () => {
			const chrome = defaultInteractionReplChromeState();
			expect(chrome.activeViewId).toBe(null);
			expect(chrome.filterKindToggles.objectVertex).toBe(true);
		});

		it("scopes displayed selection to activeViewId", () => {
			const renderer = [
				{ kind: "face", id: "f0", editable: true },
				{ kind: "object", id: "o0", editable: false },
			] satisfies readonly SelectionTarget[];
			expect(replDisplayedSelectionTargets(false, null, renderer, [])).toEqual([{ kind: "face", id: "f0", editable: true }]);
			expect(replDisplayedSelectionTargets(false, "energy.energy", renderer, [])).toEqual([
				{ kind: "object", id: "o0", editable: false },
			]);
		});

		it("merges picks within active view without clearing out-of-view targets", () => {
			const renderer: SelectionTarget[] = [
				{ kind: "wire", id: "w0", editable: true },
				{ kind: "object", id: "o0", editable: false },
			];
			expect(
				replMergeSelectionPickInView(false, null, renderer, [], [{ kind: "wire", id: "w1", editable: true }], {}),
			).toEqual([
				{ kind: "object", id: "o0", editable: false },
				{ kind: "wire", id: "w1", editable: true },
			]);
		});
	});
}
`;

const path = "c:/git/semio/spatial/js/renderer-r3f/index.tsx";
const lines = readFileSync(path, "utf8").split(/\r?\n/);
const start = lines.findIndex((l) => l.startsWith("if (import.meta.vitest)"));
if (start < 0) throw new Error("vitest block not found");
writeFileSync(path, [...lines.slice(0, start), NEW_TESTS.trimEnd()].join("\n"));
