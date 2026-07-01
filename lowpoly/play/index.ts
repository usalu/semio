// #region 🧲Header
/** @emoji 🔷 Lowpoly play — low-poly mesh editing playground. */
// #endregion 🧲Header

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	Platform,
	Playground,
	WindowKindRuntime,
	buildPuzzle3dWindowBody,
	createPlayAppRuntime,
	createDefaultLayout,
	createProductPlaygroundPlatform,
	enforcePlaygroundWindowEngagementInput,
	registerWindowBody,
	FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
	FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
	FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
	uiDeclarativeSectionsToTree,
	uiInspectorGroupsToTree,
	uiInspectorReadonlyField,
	type AppTools,
	type CommandDescriptor,
	type ToolLeaf,
	type UiInspectorFieldGroup,
	type UiNode,
	type UiTreeItemNode,
	type WindowBodyViewContext,
	type WindowEngagement,
	type WindowMeasure,
	toolCollection,
} from "@semio-tech/framework-playground-core";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import {
	DEFAULT_LOWPOLY_SELECTION,
	LOWPOLY_FIXTURE_SCHEMA,
	type LowpolyFixtureV1,
	type LowpolySelectionModeV1,
	lowpolyFixtureToJson,
	parseLowpolyFixtureJson,
} from "@semio-tech/lowpoly-core";
import type { LowpolyTransformTool } from "@semio-tech/lowpoly-react";

export const LOWPOLY_PLAY_APP_ID = "lowpoly-play";
export const LOWPOLY_PLAY_CONTROLLER_ID = "lowpoly-play";
export const LOWPOLY_PLAY_BODY_KEY_MAIN = "lowpoly.play.main";
export const LOWPOLY_PLAY_SURFACE_ID = "lowpoly.play/v1";
export const LOWPOLY_PLAY_WINDOW_KIND_ID = "lowpoly-main";
export const LOWPOLY_PLAY_HIERARCHY_TAB_ID = "framework.panel.hierarchy";
export const LOWPOLY_PLAY_CATALOGUE_TAB_ID = "framework.panel.catalogue";
export const LOWPOLY_PLAY_INSPECTION_TAB_ID = "framework.panel.inspection";

const EMPTY_FIXTURE: LowpolyFixtureV1 = {
	schema: LOWPOLY_FIXTURE_SCHEMA,
	objects: [],
	activeObjectId: "",
	selection: DEFAULT_LOWPOLY_SELECTION,
};

export const LOWPOLY_PLAY_DEFAULT_FIXTURE_JSON = lowpolyFixtureToJson(EMPTY_FIXTURE);
export const LOWPOLY_PLAY_LAYOUT = createDefaultLayout([LOWPOLY_PLAY_WINDOW_KIND_ID], "row", [100], ["Lowpoly"]);

export type LowpolyEditTool =
	| "extrude"
	| "inset"
	| "bevel"
	| "loop_cut"
	| "merge"
	| "dissolve"
	| "subdivide"
	| "triangulate"
	| "mirror"
	| "decimate";

function lowpolyPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
	return { controllerId: LOWPOLY_PLAY_CONTROLLER_ID, command, args };
}

/** @emoji 🧰 Lowpoly play footer toolbar. */
export function buildLowpolyPlayToolbarTools(
	controllerId: string,
	selectionMode: LowpolySelectionModeV1,
	transformTool: LowpolyTransformTool,
): AppTools {
	const modeToggle = (id: string, label: string, mode: LowpolySelectionModeV1): ToolLeaf => ({
		id,
		kind: "toggle",
		label,
		iconId: mode === "vertex" ? "circle" : mode === "edge" ? "minus" : mode === "face" ? "square" : "box",
		pressed: selectionMode === mode,
		controllerId,
		command: "setSelectionMode",
		args: { mode },
	});
	const transformToggle = (id: string, label: string, tool: LowpolyTransformTool): ToolLeaf => ({
		id,
		kind: "toggle",
		label,
		iconId: tool === "move" ? "move" : tool === "rotate" ? "rotate-cw" : "maximize-2",
		pressed: transformTool === tool,
		controllerId,
		command: "setTransformTool",
		args: { tool },
	});
	return [
		toolCollection("selection", "mouse-pointer", [
			modeToggle("lowpoly.mode.object", "Object", "object"),
			modeToggle("lowpoly.mode.vertex", "Vertex", "vertex"),
			modeToggle("lowpoly.mode.edge", "Edge", "edge"),
			modeToggle("lowpoly.mode.face", "Face", "face"),
		]),
		toolCollection("transform", "move", [
			transformToggle("lowpoly.transform.move", "Move", "move"),
			transformToggle("lowpoly.transform.rotate", "Rotate", "rotate"),
			transformToggle("lowpoly.transform.scale", "Scale", "scale"),
		]),
		toolCollection("edit", "pen-tool", [
			{ kind: "button", id: "lowpoly.extrude", label: "Extrude", iconId: "box", controllerId, command: "extrude" },
			{ kind: "button", id: "lowpoly.inset", label: "Inset", iconId: "square", controllerId, command: "inset" },
			{ kind: "button", id: "lowpoly.bevel", label: "Bevel", iconId: "git-branch", controllerId, command: "bevel" },
			{ kind: "button", id: "lowpoly.loop_cut", label: "Loop Cut", iconId: "git-commit", controllerId, command: "loopCut" },
			{ kind: "button", id: "lowpoly.merge", label: "Merge", iconId: "git-merge", controllerId, command: "merge" },
			{ kind: "button", id: "lowpoly.dissolve", label: "Dissolve", iconId: "eraser", controllerId, command: "dissolve" },
			{ kind: "button", id: "lowpoly.subdivide", label: "Subdivide", iconId: "grid-3x3", controllerId, command: "subdivide" },
			{ kind: "button", id: "lowpoly.triangulate", label: "Triangulate", iconId: "triangle", controllerId, command: "triangulate" },
			{ kind: "button", id: "lowpoly.mirror", label: "Mirror", iconId: "flip-horizontal", controllerId, command: "mirror" },
			{ kind: "button", id: "lowpoly.decimate", label: "Decimate", iconId: "minimize-2", controllerId, command: "decimate" },
		]),
		toolCollection("options", "settings", [
			{ kind: "button", id: "lowpoly.snap", label: "Snap", iconId: "magnet", controllerId, command: "snap" },
			{ kind: "button", id: "lowpoly.smooth", label: "Smooth", iconId: "sun", controllerId, command: "toggleSmooth" },
		]),
	];
}

//#region Panels

export function buildLowpolyPlayHierarchyTree(fixtureJson: string, selectedObjectIndex: number | null): UiNode {
	const fixture = parseLowpolyFixtureJson(fixtureJson);
	if (!fixture) {
		return {
			type: "tree",
			sections: [{ id: "lowpoly-hierarchy.invalid", label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, defaultOpen: true, items: [{ id: "lowpoly-hierarchy.invalid.msg", label: "Invalid fixture" }] }],
		};
	}
	const items: UiTreeItemNode[] = fixture.objects.map((obj, index) => ({
		id: `lowpoly-hierarchy.obj.${obj.id}`,
		label: obj.name,
		description: obj.id,
		command: lowpolyPlayCmd("selectObject", { objectId: obj.id, index }),
	}));
	return {
		type: "tree",
		sections: [
			{
				id: "lowpoly-hierarchy.objects",
				label: "Objects",
				defaultOpen: true,
				items: items.length ? items : [{ id: "lowpoly-hierarchy.empty", label: "(none)" }],
			},
		],
		selectedIds: selectedObjectIndex != null ? [`lowpoly-hierarchy.obj.${fixture.objects[selectedObjectIndex]?.id}`] : [],
	};
}

export function buildLowpolyPlayCatalogueTree(): UiNode {
	const primitives = [
		{ kind: "box", label: "Cube" },
		{ kind: "plane", label: "Plane" },
		{ kind: "cylinder", label: "Cylinder" },
		{ kind: "cone", label: "Cone" },
		{ kind: "ico_sphere", label: "Ico Sphere" },
	];
	return {
		type: "tree",
		sections: [
			{
				id: "lowpoly-catalogue.primitives",
				label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
				defaultOpen: true,
				items: primitives.map((p) => ({
					id: `lowpoly-catalogue.${p.kind}`,
					label: p.label,
					description: p.kind,
					command: lowpolyPlayCmd("addPrimitive", { kind: p.kind }),
				})),
			},
		],
	};
}

export function buildLowpolyPlayInspectorTree(fixtureJson: string, toolParams: Record<string, number>): UiNode {
	const fixture = parseLowpolyFixtureJson(fixtureJson);
	if (!fixture?.objects.length) {
		return uiDeclarativeSectionsToTree([
			{ type: "section", id: "lowpoly-inspector.empty", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "Add or select an object." }] },
		]);
	}
	const active = fixture.objects.find((o) => o.id === fixture.activeObjectId) ?? fixture.objects[0];
	const groups: UiInspectorFieldGroup[] = [
		{
			id: "lowpoly-inspector.object",
			label: "Object",
			fields: [
				uiInspectorReadonlyField("lowpoly-inspector.name", "Name", active?.name ?? ""),
				uiInspectorReadonlyField("lowpoly-inspector.mode", "Selection", fixture.selection.mode),
			],
		},
		{
			id: "lowpoly-inspector.tool",
			label: "Tool Params",
			fields: [
				{
					type: "field",
					id: "lowpoly-inspector.extrude",
					label: "Extrude distance",
					child: {
						type: "input",
						id: "lowpoly-inspector.extrude.input",
						inputKind: "number",
						value: String(toolParams.extrudeDistance ?? 0.25),
						onChange: lowpolyPlayCmd("setToolParam", { field: "extrudeDistance" }),
					},
				},
				{
					type: "field",
					id: "lowpoly-inspector.inset",
					label: "Inset amount",
					child: {
						type: "input",
						id: "lowpoly-inspector.inset.input",
						inputKind: "number",
						value: String(toolParams.insetAmount ?? 0.1),
						onChange: lowpolyPlayCmd("setToolParam", { field: "insetAmount" }),
					},
				},
				{
					type: "field",
					id: "lowpoly-inspector.decimate",
					label: "Decimate ratio",
					child: {
						type: "input",
						id: "lowpoly-inspector.decimate.input",
						inputKind: "number",
						value: String(toolParams.decimateRatio ?? 0.5),
						onChange: lowpolyPlayCmd("setToolParam", { field: "decimateRatio" }),
					},
				},
			],
		},
	];
	return uiInspectorGroupsToTree(groups);
}

//#endregion Panels

/** @emoji 🎮 Lowpoly play controller. */
export class LowpolyPlayController extends Controller {
	readonly mainMode = new ModeRuntime("main", "Edit", undefined);
	private fixtureJson = LOWPOLY_PLAY_DEFAULT_FIXTURE_JSON;
	private selectionMode: LowpolySelectionModeV1 = "object";
	private selectedIds: number[] = [];
	private transformTool: LowpolyTransformTool = "move";
	private toolParams: Record<string, number> = {
		extrudeDistance: 0.25,
		insetAmount: 0.1,
		bevelAmount: 0.05,
		bevelSegments: 1,
		loopCuts: 1,
		decimateRatio: 0.5,
		snapGrid: 0.25,
		mirrorAxis: 0,
	};
	private smoothShading = false;
	private meshCommandEpoch = 0;
	private pendingMeshCommand: string | null = null;
	private interactionRevision = 0;
	private readonly snapshotListeners = new Set<() => void>();

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(LOWPOLY_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.rebuildShellMode();
	}

	getFixtureJson(): string {
		return this.fixtureJson;
	}

	getSelectionMode(): LowpolySelectionModeV1 {
		return this.selectionMode;
	}

	getSelectedIds(): readonly number[] {
		return this.selectedIds;
	}

	getTransformTool(): LowpolyTransformTool {
		return this.transformTool;
	}

	getToolParams(): Readonly<Record<string, number>> {
		return this.toolParams;
	}

	getPendingMeshCommand(): string | null {
		return this.pendingMeshCommand;
	}

	clearPendingMeshCommand(): void {
		this.pendingMeshCommand = null;
	}

	getMeshCommandEpoch(): number {
		return this.meshCommandEpoch;
	}

	getSmoothShading(): boolean {
		return this.smoothShading;
	}

	getInteractionRevision(): number {
		return this.interactionRevision;
	}

	subscribeSnapshot(listener: () => void): () => void {
		this.snapshotListeners.add(listener);
		return () => this.snapshotListeners.delete(listener);
	}

	private notifySnapshot(): void {
		for (const listener of this.snapshotListeners) {
			listener();
		}
	}

	private commitFixture(json: string): void {
		if (json === this.fixtureJson) return;
		this.fixtureJson = json;
		this.interactionRevision += 1;
		this.notifySnapshot();
		this.emit();
	}

	private bumpMeshCommand(): void {
		this.meshCommandEpoch += 1;
		this.interactionRevision += 1;
		this.notifySnapshot();
		this.emit();
	}

	private windowEngagement(): WindowEngagement {
		return {
			sessionActive: false,
			input: {
				id: "lowpoly-engagement",
				value: "",
				placeholder: "extrude, inset, mirror, decimate",
				onChange: lowpolyPlayCmd("engagementInput"),
				onSubmit: lowpolyPlayCmd("engagementSubmit"),
			},
			possibleEngagements: [
				{ id: "lowpoly.eng.extrude", label: "Extrude", command: lowpolyPlayCmd("extrude") },
				{ id: "lowpoly.eng.triangulate", label: "Triangulate", command: lowpolyPlayCmd("triangulate") },
			],
			controls: [],
			status: [{ id: "lowpoly-status", text: `${this.selectionMode} · ${this.transformTool}` }],
		};
	}

	private rebuildShellMode(): void {
		this.mainMode.tools = buildLowpolyPlayToolbarTools(LOWPOLY_PLAY_CONTROLLER_ID, this.selectionMode, this.transformTool);
		this.mainMode.windowKinds = [
			new WindowKindRuntime(LOWPOLY_PLAY_WINDOW_KIND_ID, "Lowpoly", LOWPOLY_PLAY_BODY_KEY_MAIN, undefined, [], this.windowEngagement()),
		];
		for (const windowKind of this.mainMode.windowKinds) {
			enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Lowpoly play window "${windowKind.id}"`);
		}
	}

	override run(command: string, args?: unknown): void {
		if (command === "setFixtureJson") {
			const json = (args as { json?: string }).json;
			if (typeof json === "string") this.commitFixture(json);
			return;
		}
		if (command === "setSelectionMode") {
			const mode = (args as { mode?: LowpolySelectionModeV1 }).mode;
			if (mode) {
				this.selectionMode = mode;
				this.rebuildShellMode();
				this.interactionRevision += 1;
				this.notifySnapshot();
				this.emit();
			}
			return;
		}
		if (command === "setSelection") {
			const mode = (args as { mode?: LowpolySelectionModeV1 }).mode;
			const ids = (args as { ids?: number[] }).ids;
			if (mode) this.selectionMode = mode;
			if (Array.isArray(ids)) this.selectedIds = [...ids];
			this.interactionRevision += 1;
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "setTransformTool") {
			const tool = (args as { tool?: LowpolyTransformTool }).tool;
			if (tool) {
				this.transformTool = tool;
				this.rebuildShellMode();
				this.emit();
			}
			return;
		}
		if (command === "setToolParam") {
			const field = (args as { field?: string; value?: number }).field;
			const value = (args as { value?: number }).value;
			if (field && typeof value === "number") {
				this.toolParams = { ...this.toolParams, [field]: value };
				this.interactionRevision += 1;
				this.notifySnapshot();
				this.emit();
			}
			return;
		}
		if (command === "selectObject") {
			const objectId = (args as { objectId?: string; index?: number }).objectId;
			const index = (args as { index?: number }).index;
			if (typeof objectId === "string") {
				const fixture = parseLowpolyFixtureJson(this.fixtureJson);
				if (fixture) {
					const next = {
						...fixture,
						activeObjectId: objectId,
						selection: { mode: "object" as const, ids: typeof index === "number" ? [index] : [] },
					};
					this.selectionMode = "object";
					this.selectedIds = typeof index === "number" ? [index] : [];
					this.commitFixture(lowpolyFixtureToJson(next));
				}
			}
			return;
		}
		if (command === "addPrimitive") {
			const kind = (args as { kind?: string }).kind ?? "box";
			this.pendingMeshCommand = `addPrimitive:${kind}`;
			this.bumpMeshCommand();
			return;
		}
		const meshCommands = ["extrude", "inset", "bevel", "loopCut", "merge", "dissolve", "subdivide", "triangulate", "mirror", "decimate", "snap", "toggleSmooth"];
		if (meshCommands.includes(command)) {
			this.pendingMeshCommand = command;
			this.bumpMeshCommand();
			return;
		}
		if (command === "engagementSubmit") {
			const value = ((args as { value?: string }).value ?? "").trim().toLowerCase();
			if (value) this.run(value);
		}
	}
}

function buildLowpolyPlayMainDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
	return buildPuzzle3dWindowBody(LOWPOLY_PLAY_SURFACE_ID, LOWPOLY_PLAY_CONTROLLER_ID, LOWPOLY_PLAY_WINDOW_KIND_ID);
}

export function registerLowpolyPlayDeclarativeBodies(): void {
	registerWindowBody(LOWPOLY_PLAY_BODY_KEY_MAIN, buildLowpolyPlayMainDeclarativeBody);
}

export function buildLowpolyPlayAppRuntime(controller: LowpolyPlayController): AppRuntime {
	return createPlayAppRuntime(LOWPOLY_PLAY_APP_ID, "Lowpoly", controller, LOWPOLY_PLAY_LAYOUT, controller.mainMode);
}

/** @emoji 🛝 Lowpoly playground app. */
export class PlaygroundLowpoly extends Playground {
	readonly id = LOWPOLY_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new LowpolyPlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildLowpolyPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerLowpolyPlayDeclarativeBodies();
	}
}

export { lowpolyPlayCmd };

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("LowpolyPlayController", () => {
		it("default fixture json is valid schema", () => {
			expect(LOWPOLY_PLAY_DEFAULT_FIXTURE_JSON).toContain(LOWPOLY_FIXTURE_SCHEMA);
		});
		it("mesh command bumps epoch", () => {
			const bus = new CommandBus();
			const ctrl = new LowpolyPlayController(bus, () => {});
			const before = ctrl.getMeshCommandEpoch();
			ctrl.run("extrude");
			expect(ctrl.getMeshCommandEpoch()).toBeGreaterThan(before);
		});
	});
}

if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest && import.meta.env.PUZZLE_PLAY_ENTRY === "lowpoly") {
	bootstrapElementsSurfaceChromeDocument("system");
	void (async () => {
		await import("./globals.css");
		const { bootLowpolyPlay } = await import("@semio-tech/framework-playground-renderer-react/lowpoly");
		bootLowpolyPlay(new PlaygroundLowpoly());
	})();
}
