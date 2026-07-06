import { createElement, type ReactElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";
import { Canvas2dHost } from "./components/canvas-2d-host.tsx";
import { NodeGraphHost } from "./components/node-graph-host.tsx";
import { RasterHost } from "./components/raster-host.tsx";
import { TableHost } from "./components/table-host.tsx";
import { TextEditorHost } from "./components/text-editor-host.tsx";
import { World3dHost } from "./components/world-3d-host.tsx";
import { appHierarchyLabel, appWindowHierarchyLabel } from "./os-shell.tsx";
import { interpretUiNode } from "./ui-interpreter.tsx";
import type { UiNode } from "./os-shell.tsx";

const noopCommand = () => {};

describe("framework plugin runtime", () => {
	it("loads plugin modules through framework-core", async () => {
		const { loadPluginModule } = await import("@semio-tech/framework-core");
		const handle = await loadPluginModule("mock", "data:application/javascript,export function semio_plugin_manifest(){return JSON.stringify({pluginId:'mock',label:'Mock',version:'0',apps:[],programs:[],examples:[]})}");
		expect(handle.manifest.pluginId).toBe("mock");
	});
});

describe("framework renderer types", () => {
	it("formats canonical app hierarchy for chrome and window tabs", () => {
		const app = {
			id: "puzzle3d-play",
			label: "Puzzle 3D",
			hierarchy: ["semio", "puzzle", "3d"],
			controllerId: "puzzle3d-play",
			modes: [],
			windowKinds: [],
			panelTabs: [],
			keybindings: [],
		};
		expect(appHierarchyLabel(app.hierarchy)).toBe("semio · puzzle · 3d");
		expect(appWindowHierarchyLabel(app, "Puzzle 3D")).toBe("semio · puzzle · 3d");
		expect(appWindowHierarchyLabel(app, "Perspective")).toBe("semio · puzzle · 3d · perspective");
	});

	it("accepts component scene nodes", () => {
		const node: UiNode = {
			type: "componentScene",
			surfaceId: "draw.play.composite",
			controllerId: "draw-play",
			componentKind: "canvas-2d",
			canvas2d: {
				cameraX: 0,
				cameraY: 0,
				zoom: 1,
				layersJson: "[]",
			},
		};
		expect(node.componentKind).toBe("canvas-2d");
	});
});

describe("framework renderer hosts", () => {
	it("renders node graph host from media graph scene json", () => {
		const markup = renderToStaticMarkup(
			createElement(NodeGraphHost, {
				node: {
					type: "componentScene",
					surfaceId: "s.play.media-graph",
					controllerId: "s-play",
					componentKind: "node-graph",
					nodeGraph: {
						nodesJson: JSON.stringify([
							{
								id: "node-a",
								instanceId: "app-a",
								label: "Draw",
								x: 10,
								y: 20,
								inputs: [{ id: "in", resourceKind: "2d.drawing" }],
								outputs: [{ id: "out", resourceKind: "2d.drawing" }],
							},
						]),
						edgesJson: "[]",
						viewportJson: '{"x":0,"y":0,"zoom":1}',
					},
				},
				onCommand: noopCommand,
			}),
		);
		expect(markup).toContain("semio-node-graph-host");
	});

	it("renders editable node graph host with find items", () => {
		const markup = renderToStaticMarkup(
			createElement(NodeGraphHost, {
				node: {
					type: "componentScene",
					surfaceId: "s.play.media-graph",
					controllerId: "s-play",
					componentKind: "node-graph",
					nodeGraph: {
						nodesJson: JSON.stringify([
							{
								id: "node-a",
								instanceId: "app-a",
								label: "Draw",
								x: 10,
								y: 20,
								inputs: [{ id: "in", resourceKind: "2d.drawing" }],
								outputs: [{ id: "out", resourceKind: "2d.drawing" }],
							},
						]),
						edgesJson: "[]",
						viewportJson: '{"x":0,"y":0,"zoom":1}',
						editable: true,
						findItemsJson: JSON.stringify([{ id: "app-a", label: "Draw", category: "Media graph" }]),
					},
				},
				onCommand: noopCommand,
			}),
		);
		expect(markup).toContain("semio-node-graph-host");
	});

	it("renders canvas 2d host with infinite canvas session", () => {
		const markup = renderToStaticMarkup(
			createElement(Canvas2dHost, {
				node: {
					type: "componentScene",
					surfaceId: "draw.play.canvas",
					controllerId: "draw-play",
					componentKind: "canvas-2d",
					canvas2d: {
						cameraX: 0,
						cameraY: 0,
						zoom: 1,
						layersJson: JSON.stringify([{ id: "layer-1", name: "Layer 1", x: 0, y: 0, width: 120, height: 80 }]),
					},
				},
				onCommand: noopCommand,
			}),
		);
		expect(markup).toContain("semio-canvas-2d-host");
	});

	it("renders world 3d empty state without mounting r3f canvas", () => {
		const markup = renderToStaticMarkup(
			createElement(World3dHost, {
				node: {
					type: "componentScene",
					surfaceId: "puzzle.play.world",
					controllerId: "puzzle-play",
					componentKind: "world-3d",
				},
				onCommand: noopCommand,
			}),
		);
		expect(markup).toContain("semio-world-3d-empty");
	});

	it("accepts extended world 3d scene fields", () => {
		const node: UiNode = {
			type: "componentScene",
			surfaceId: "lowpoly.play.main",
			controllerId: "lowpoly-play",
			componentKind: "world-3d",
			world3d: {
				cameraJson: "{}",
				meshesJson: "[]",
				instancesJson: "[]",
				selectionJson: "{}",
			},
		};
		expect(node.world3d?.meshesJson).toBe("[]");
		expect(node.world3d?.selectionJson).toBe("{}");
	});

	it("renders text editor host", () => {
		const markup = renderToStaticMarkup(
			createElement(TextEditorHost, {
				node: {
					type: "componentScene",
					surfaceId: "writer.play.editor",
					controllerId: "writer-play",
					componentKind: "text-editor",
					textEditor: {
						buffer: "hello",
						language: "jack",
						tokensJson: JSON.stringify([{ class: "ident", start: 0, end: 5 }]),
					},
				},
				onCommand: noopCommand,
			}),
		);
		expect(markup).toContain("semio-text-editor-host");
		expect(markup).toContain("hello");
	});

	it("renders table host with ui-react table", () => {
		const markup = renderToStaticMarkup(
			createElement(TableHost, {
				node: {
					type: "componentScene",
					surfaceId: "s.play.catalogue",
					controllerId: "s-play",
					componentKind: "table",
					table: {
						columnsJson: JSON.stringify([{ id: "label", label: "Label" }]),
						rowsJson: JSON.stringify([{ label: "Draw" }]),
					},
				},
				onCommand: noopCommand,
			}),
		);
		expect(markup).toContain("semio-table-host");
		expect(markup).toContain("Draw");
	});

	it("renders raster host from base64 pixels", () => {
		const markup = renderToStaticMarkup(
			createElement(RasterHost, {
				node: {
					type: "componentScene",
					surfaceId: "raster.play.viewport",
					controllerId: "raster-play",
					componentKind: "raster",
					raster: {
						width: 2,
						height: 2,
						pixelsBase64: "iVBORw0KGgoAAAANSUhEUgAAAAIAAAACCAYAAABytg0kAAAAEklEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==",
					},
				},
				onCommand: noopCommand,
			}),
		);
		expect(markup).toContain("semio-raster-host");
		expect(markup).toContain("data:image/png;base64,");
	});

	it("interprets virtual file system component scenes", () => {
		const markup = renderToStaticMarkup(
			interpretUiNode(
				{
					type: "componentScene",
					surfaceId: "s.play.media-vfs",
					controllerId: "s-play",
					componentKind: "virtualFileSystem",
					virtualFileSystem: {
						schemaJson: JSON.stringify({
							fileNodeKinds: {
								instance: { id: "instance", name: "Instance", descriptors: [] },
							},
							descriptorKinds: {},
							descriptorColumnIds: [],
						}),
						rowsJson: JSON.stringify([
							{
								id: "row-1",
								fileNodeKindId: "instance",
								name: "Draw",
								path: "/draw",
								level: 0,
							},
						]),
					},
				},
				{ onCommand: noopCommand },
			) as ReactElement,
		);
		expect(markup).toContain("Draw");
	});
});
