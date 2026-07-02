export * from "./internal.ts";
export * from "./playground.ts";

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";
import { rasterPlayAppDefinition } from "./playground.ts";

/** @emoji 🧩 S program definition for raster. */
export function buildRasterProgramDefinition(): PlatformDefinition {
	const app = rasterPlayAppDefinition;
	return {
		id: "raster",
		name: "Raster",
		apiVersion: "1",
		apps: [{ id: "raster", label: app.label, controllerId: app.controllerId, modes: app.modes, defaultModeId: app.defaultModeId }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("parseRasterDocument", () => {
		it("parses minimal document", () => {
			const doc = parseRasterDocument({
				schema: "raster.document",
				id: "test",
				camera: { x: 0, y: 0, zoom: 1 },
				layers: [{ kind: "pixel", id: "a", name: "A", visible: true, opacity: 1, blendMode: "multiply", transform: { x: 0, y: 0, scaleX: 1, scaleY: 1, rotation: 0 } }],
			});
			expect(doc.layers[0]?.blendMode).toBe("multiply");
		});

		it("rejects wrong schema", () => {
			expect(() => parseRasterDocument({ schema: "other" })).toThrow();
		});

		it("round-trips assets and filters", () => {
			const raw = {
				schema: "raster.document",
				id: "semio",
				camera: { x: 0, y: 0, zoom: 1 },
				assets: { emblem: { mime: "image/png", data: "aGVsbG8=" } },
				layers: [
					{
						kind: "pixel",
						id: "logo",
						name: "Logo",
						visible: true,
						opacity: 1,
						blendMode: "normal",
						imageKey: "emblem",
						filters: [{ kind: "gaussianBlur", radius: 8 }],
						transform: defaultRasterTransform(),
					},
				],
			};
			const doc = parseRasterDocument(raw);
			const restored = parseRasterDocument(JSON.parse(rasterDocumentToExportJson(doc)));
			expect(restored.assets?.emblem?.data).toBe("aGVsbG8=");
			expect(restored.layers[0]?.kind === "pixel" && restored.layers[0].filters?.[0]?.kind).toBe("gaussianBlur");
		});
	});

	describe("applyRasterEditOp", () => {
		it("toggles visibility", () => {
			const doc = defaultRasterDocument();
			const layerId = doc.layers[0]!.id;
			const next = applyRasterEditOp(doc, { op: "setLayerVisible", layerId, visible: false });
			expect(next.layers[0]?.visible).toBe(false);
		});

		it("reorders layers", () => {
			const doc = parseRasterDocument({
				schema: "raster.document",
				id: "t",
				camera: { x: 0, y: 0, zoom: 1 },
				layers: [
					{ kind: "pixel", id: "a", name: "A", visible: true, opacity: 1, blendMode: "normal", transform: defaultRasterTransform() },
					{ kind: "pixel", id: "b", name: "B", visible: true, opacity: 1, blendMode: "normal", transform: defaultRasterTransform() },
				],
			});
			const next = applyRasterEditOp(doc, { op: "reorderLayer", layerId: "a", index: 1 });
			expect(next.layers.map((layer) => layer.id)).toEqual(["b", "a"]);
		});

		it("duplicates a layer", () => {
			const doc = defaultRasterDocument();
			const layerId = doc.layers[0]!.id;
			const next = applyRasterEditOp(doc, { op: "duplicateLayer", layerId });
			expect(next.layers).toHaveLength(2);
			expect(next.layers[1]?.name).toContain("copy");
		});
	});

	describe("resolveRasterMarqueeLayerHits", () => {
		it("selects layers inside a full marquee", () => {
			const doc = parseRasterDocument({
				schema: "raster.document",
				id: "test",
				camera: { x: 0, y: 0, zoom: 1 },
				layers: [
					{
						kind: "pixel",
						id: "a",
						name: "A",
						visible: true,
						opacity: 1,
						blendMode: "normal",
						width: 100,
						height: 100,
						transform: { x: 0, y: 0, scaleX: 1, scaleY: 1, rotation: 0 },
					},
				],
			});
			const hits = resolveRasterMarqueeLayerHits(doc, doc.camera, { width: 800, height: 600 }, { x: 350, y: 250, width: 100, height: 100 }, false);
			expect(hits).toEqual(["a"]);
		});
	});

	describe("rasterDocumentToSyncJson", () => {
		it("omits camera so zoom does not re-sync compositor layers", () => {
			const doc = defaultRasterDocument("t");
			const zoomed = applyRasterEditOp(doc, { op: "setCamera", camera: { x: 12, y: -4, zoom: 2.5 } });
			expect(rasterDocumentToSyncJson(doc)).toBe(rasterDocumentToSyncJson(zoomed));
			expect(zoomed.camera.zoom).toBe(2.5);
		});
	});

	describe("rasterCameraEqual", () => {
		it("compares camera tuples", () => {
			expect(rasterCameraEqual({ x: 0, y: 0, zoom: 1 }, { x: 0, y: 0, zoom: 1 })).toBe(true);
			expect(rasterCameraEqual({ x: 0, y: 0, zoom: 1 }, { x: 0, y: 0, zoom: 2 })).toBe(false);
		});
	});

	describe("rasterNavigatorFitCamera", () => {
		it("fits visible pixel layers into the navigator viewport", () => {
			const doc = parseRasterDocument({
				schema: "raster.document",
				id: "t",
				camera: { x: 0, y: 0, zoom: 1 },
				layers: [
					{
						kind: "pixel",
						id: "a",
						name: "A",
						visible: true,
						opacity: 1,
						blendMode: "normal",
						width: 200,
						height: 100,
						transform: { x: 0, y: 0, scaleX: 1, scaleY: 1, rotation: 0 },
					},
				],
			});
			const fit = rasterNavigatorFitCamera(doc, { width: 400, height: 200 }, 0);
			expect(fit.zoom).toBeGreaterThan(1);
			expect(fit.x).toBe(0);
			expect(fit.y).toBe(0);
		});
	});

	describe("rasterWheelCamera", () => {
		it("zooms toward the cursor", () => {
			const camera = { x: 0, y: 0, zoom: 1 };
			const viewport = { width: 400, height: 300 };
			const zoomedIn = rasterWheelCamera(camera, viewport, { x: 200, y: 150 }, -100);
			expect(zoomedIn.zoom).toBeGreaterThan(camera.zoom);
			const zoomedOut = rasterWheelCamera(camera, viewport, { x: 200, y: 150 }, 100);
			expect(zoomedOut.zoom).toBeLessThan(camera.zoom);
		});
	});

	describe("rasterNavigatorViewportOverlay", () => {
		it("maps the composite viewport into navigator screen space", () => {
			const contentCamera = { x: 0, y: 0, zoom: 2 };
			const contentViewport = { width: 800, height: 600 };
			const navigatorCamera = { x: 0, y: 0, zoom: 0.5 };
			const navigatorViewport = { width: 200, height: 150 };
			const overlay = rasterNavigatorViewportOverlay(contentCamera, contentViewport, navigatorCamera, navigatorViewport);
			expect(overlay.width).toBeGreaterThan(0);
			expect(overlay.height).toBeGreaterThan(0);
		});
	});

	describe("rasterPlaySelectionIdsFromTreeRowIds", () => {
		it("maps hierarchy and mask tree rows to layer ids", () => {
			const doc = parseRasterDocument({
				schema: "raster.document",
				id: "t",
				camera: { x: 0, y: 0, zoom: 1 },
				layers: [
					{
						kind: "pixel",
						id: "logo",
						name: "Logo",
						visible: true,
						opacity: 1,
						blendMode: "normal",
						transform: defaultRasterTransform(),
						mask: { enabled: true, width: 64, height: 64 },
					},
				],
			});
			const layerRow = rasterPlayLayersTreeRowId(doc.layers[0]!);
			const maskRow = rasterPlayMaskTreeRowId("logo");
			expect(rasterPlaySelectionIdsFromTreeRowIds(doc, [layerRow])).toEqual(["logo"]);
			expect(rasterPlaySelectionIdsFromTreeRowIds(doc, [maskRow])).toEqual(["logo"]);
			expect(rasterPlayTreeRowIdsForSelectionIds(doc, ["logo"])).toEqual([layerRow]);
			expect(rasterPlayMaskTreeRowIdsForSelectionIds(doc, ["logo"])).toEqual([maskRow]);
		});
	});

	describe("rasterPlayLayersTreeHighlightedIdsForKind", () => {
		it("highlights all layers sharing a blend mode", () => {
			const doc = parseRasterDocument({
				schema: "raster.document",
				id: "t",
				camera: { x: 0, y: 0, zoom: 1 },
				layers: [
					{ kind: "pixel", id: "a", name: "A", visible: true, opacity: 1, blendMode: "screen", transform: defaultRasterTransform() },
					{ kind: "pixel", id: "b", name: "B", visible: true, opacity: 1, blendMode: "screen", transform: defaultRasterTransform() },
					{ kind: "pixel", id: "c", name: "C", visible: true, opacity: 1, blendMode: "normal", transform: defaultRasterTransform() },
				],
			});
			const ids = rasterPlayLayersTreeHighlightedIdsForKind(doc, { domain: "blendMode", kindId: "screen" });
			expect(ids).toHaveLength(2);
		});
	});

	describe("createRasterAppVcsHandler", () => {
		it("materializes inline raster documents", () => {
			const doc = defaultRasterDocument("inline");
			const projection = createRasterAppVcsHandler().materializeProjection({ inline: rasterDocumentToJson(doc) });
			expect(projection.id).toBe("inline");
		});
	});
}
// #endregion 🧪Tests
