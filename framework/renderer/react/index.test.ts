import { createElement, type ReactElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";
import { Canvas2dHost } from "./components/canvas-2d-host.tsx";
import { NodeGraphHost } from "./components/node-graph-host.tsx";
import { RasterHost } from "./components/raster-host.tsx";
import { TableHost } from "./components/table-host.tsx";
import { TextEditorHost } from "./components/text-editor-host.tsx";
import { World3dHost } from "./components/world-3d-host.tsx";
import { appDocumentLabel, appWindowDocumentLabel, buildToolbarRibbonSegments, selectSpawnedToolNodes, sortToolNodes, spawnedWindowChromeForKind, ToolTree } from "./os-shell.tsx";
import { interpretUiNode } from "./ui-interpreter.tsx";
import type { UiNode } from "./os-shell.tsx";

const noopCommand = () => {};

describe("framework plugin runtime", () => {
	it("loads plugin modules through framework-core", async () => {
		const { loadPluginModule } = await import("@semio-tech/framework-core");
		const handle = await loadPluginModule("mock", "data:application/javascript,export function semio_plugin_manifest(){return JSON.stringify({pluginId:'mock',label:'Mock',version:'0',apps:[],programs:[],examples:[]})}");
		expect(handle.manifest.pluginId).toBe("mock");
	});

	it("extracts patch ops from CommandResult plugin responses", async () => {
		const { patchOpsFromCommandResponse } = await import("@semio-tech/framework-core");
		const legacy = patchOpsFromCommandResponse(
			JSON.stringify([JSON.stringify({ op: "setDocument", document: { id: "legacy" } })]),
		);
		expect(legacy).toEqual([JSON.stringify({ op: "setDocument", document: { id: "legacy" } })]);
		const commandResult = patchOpsFromCommandResponse(
			JSON.stringify({
				output: null,
				operations: [{ diff: { payload: { op: "setDocument", document: { id: "forest" } } } }],
				inverseGroup: { commandId: "setActiveExample:1:0", operations: [] },
			}),
		);
		expect(commandResult).toEqual([JSON.stringify({ op: "setDocument", document: { id: "forest" } })]);
	});
});

describe("framework renderer types", () => {
	it("formats canonical app document for chrome and window tabs", () => {
		const app = {
			id: "puzzle3d-play",
			label: "Puzzle 3D",
			document: ["semio", "puzzle", "3d"],
			controllerId: "puzzle3d-play",
			modes: [],
			windowKinds: [],
			panelTabs: [],
			keybindings: [],
		};
		expect(appDocumentLabel(app.document)).toBe("semio · puzzle · 3d");
		expect(appWindowDocumentLabel(app, "Puzzle 3D")).toBe("semio · puzzle · 3d");
		expect(appWindowDocumentLabel(app, "Perspective")).toBe("semio · puzzle · 3d · perspective");
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

describe("framework external slots", () => {
	it("resolves external slots through contributor plugins", async () => {
		const { resolveExternalSlots } = await import("@semio-tech/framework-core");
		const handle = {
			pluginId: "forms-module-procedural",
			manifest: { pluginId: "forms-module-procedural", label: "Module", version: "0", apps: [], programs: [], examples: [] },
			createApp: async () => 7,
			destroyApp: async () => {},
			handleCommand: async () => [],
			render: async () => ({ type: "text", value: "fallback" }),
			renderWithDocument: async (_instanceId: number, bodyKey: string) => ({
				type: "text",
				value: `resolved:${bodyKey}`,
			}),
			tools: async () => [],
			windowEngagements: async () => ({}),
			windowMeasures: async () => ({}),
			dispose: () => {},
		};
		const resolved = await resolveExternalSlots(
			{
				type: "externalSlot",
				pluginId: "forms-module-procedural",
				appId: "forms-module-procedural",
				bodyKey: "preview",
				paramsJson: "{}",
			},
			{
				plugins: new Map([["forms-module-procedural", handle]]),
				contributorInstances: new Map(),
				viewState: {},
			},
		);
		expect(resolved).toEqual({ type: "text", value: "resolved:preview" });
	});

	it("renders external slot fallback text when unresolved", () => {
		const markup = renderToStaticMarkup(
			interpretUiNode(
				{
					type: "externalSlot",
					pluginId: "missing-module",
					appId: "missing-module",
					bodyKey: "preview",
					paramsJson: "{}",
				},
				{ onCommand: noopCommand },
			),
		);
		expect(markup).toContain("Extension unavailable: missing-module");
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
			surfaceId: "puzzle.3d.play.viewport",
			controllerId: "puzzle3d-play",
			componentKind: "world-3d",
			world3d: {
				cameraJson: "{}",
				meshesJson: "[]",
				instancesJson: "[]",
				selectionJson: "{}",
				vorticesJson: "[]",
				attractionsJson: "[]",
				targetVolumesJson: "[]",
				referencesJson: "[]",
				brushPreviewJson: undefined,
				interactionJson: "{\"activeTool\":\"select\"}",
			},
		};
		expect(node.world3d?.meshesJson).toBe("[]");
		expect(node.world3d?.vorticesJson).toBe("[]");
		expect(node.world3d?.interactionJson).toContain("select");
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

describe("spawned window chrome", () => {
	const app = {
		id: "cad-play",
		label: "CAD",
		document: ["semio", "cad"],
		controllerId: "cad-play",
		defaultModeId: "edit",
		modes: [
			{
				id: "edit",
				label: "Edit",
				tools: [{ id: "static-tool", kind: "button" as const, iconId: "save", controllerId: "cad-play", command: "save" }],
			},
		],
		windowKinds: [
			{
				id: "cad-window-shape",
				label: "Shape",
				bodyKey: "shape",
				engagement: {
					input: {
						id: "engagement-input",
						placeholder: "Command",
						onChange: { controllerId: "cad-play", command: "engagementInput" },
					},
					possibleEngagements: [{ id: "box", label: "Box", command: { controllerId: "cad-play", command: "startBox" } }],
				},
				measures: [{ id: "render-mode", kind: "select" as const, label: "Render Mode", value: "shaded", items: [], onChange: { controllerId: "cad-play", command: "setRenderMode" } }],
			},
		],
		panelTabs: [],
		keybindings: [],
	};

	it("prefers dynamic spawned tools over static mode tools", () => {
		const dynamic = [{ id: "dynamic-tool", kind: "button" as const, iconId: "box", controllerId: "cad-play", command: "box" }];
		expect(selectSpawnedToolNodes(dynamic, app, "edit")).toEqual(dynamic);
		expect(selectSpawnedToolNodes([], app, "edit")).toEqual(app.modes[0]!.tools);
	});

	it("builds spawned engagement and measures chrome from plugin contributions", () => {
		const kind = app.windowKinds[0]!;
		const engagements = {
			[kind.id]: {
				input: {
					id: "engagement-input",
					value: "Box",
					placeholder: "Command",
					onChange: { controllerId: "cad-play", command: "engagementInput" },
				},
				possibleEngagements: [{ id: "box", label: "Box", detail: "b", command: { controllerId: "cad-play", command: "startBox" } }],
			},
		};
		const measures = { [kind.id]: kind.measures ?? [] };
		const chrome = spawnedWindowChromeForKind(kind, engagements, measures, noopCommand);
		expect(chrome.engagement?.input?.value).toBe("Box");
		expect(chrome.engagement?.possibleEngagements?.[0]?.label).toBe("Box");
		const measuresMarkup = renderToStaticMarkup(chrome.measures as ReactElement);
		expect(measuresMarkup).toContain("Render Mode");
	});
});

describe("toolbar ribbon", () => {
	it("sorts tool nodes by order", () => {
		const sorted = sortToolNodes([
			{ id: "b", kind: "button", iconId: "box", order: 2, controllerId: "x", command: "b" },
			{ id: "a", kind: "button", iconId: "box", order: 1, controllerId: "x", command: "a" },
		]);
		expect(sorted.map((node) => node.id)).toEqual(["a", "b"]);
	});

	it("builds picker segments for sibling nested collections", () => {
		const segments = buildToolbarRibbonSegments(
			[
				{
					id: "view",
					kind: "collection",
					iconId: "eye",
					children: [
						{
							id: "view-tools",
							kind: "collection",
							iconId: "zoom-in",
							children: [{ id: "zoom-in", kind: "button", iconId: "zoom-in", controllerId: "x", command: "zoomIn" }],
						},
					],
				},
				{
					id: "construct",
					kind: "collection",
					iconId: "box",
					children: [
						{
							id: "construct-tools",
							kind: "collection",
							iconId: "box",
							children: [{ id: "box", kind: "button", iconId: "box", controllerId: "x", command: "box" }],
						},
					],
				},
			],
			["construct"],
		);
		expect(segments[0]).toMatchObject({ kind: "picker", depth: 0 });
		expect(segments.some((segment) => segment.kind === "tools" && segment.items.some((item) => item.id === "box"))).toBe(true);
	});

	it("flattens leaf-only sibling collections into separate tool zones", () => {
		const segments = buildToolbarRibbonSegments(
			[
				{
					id: "view",
					kind: "collection",
					iconId: "eye",
					children: [{ id: "zoom-in", kind: "button", iconId: "zoom-in", controllerId: "x", command: "zoomIn" }],
				},
				{
					id: "save",
					kind: "collection",
					iconId: "save",
					children: [{ id: "export", kind: "button", iconId: "download", controllerId: "x", command: "export" }],
				},
			],
			[],
		);
		expect(segments.every((segment) => segment.kind === "tools")).toBe(true);
		expect(segments).toHaveLength(2);
	});

	it("renders ribbon toolbar with picker and batched toggles", () => {
		const markup = renderToStaticMarkup(
			createElement(ToolTree, {
				tools: [
					{
						id: "view",
						kind: "collection",
						iconId: "eye",
						children: [
							{
								id: "view-tools",
								kind: "collection",
								iconId: "eye",
								children: [
									{ id: "show-edges", kind: "toggle", iconId: "box", pressed: true, controllerId: "x", command: "edges" },
									{ id: "show-faces", kind: "toggle", iconId: "square", pressed: false, controllerId: "x", command: "faces" },
								],
							},
						],
					},
					{
						id: "construct",
						kind: "collection",
						iconId: "box",
						children: [
							{
								id: "construct-tools",
								kind: "collection",
								iconId: "box",
								children: [{ id: "box", kind: "button", iconId: "box", controllerId: "x", command: "box" }],
							},
						],
					},
				],
				onCommand: noopCommand,
			}),
		);
		expect(markup).toContain('id="ui.toolbar"');
		expect(markup).toContain('data-slot="toggle-group"');
	});
});
