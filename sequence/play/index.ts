// #region 🧲Header
/** @emoji 📜 Sequence play — execution-flow canvas playground. */
// #endregion 🧲Header

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	Platform,
	Playground,
	WindowKindRuntime,
	buildSequenceWindowBody,
	createPlayAppRuntime,
	createProductPlaygroundPlatform,
	createStackLayout,
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
	DEFAULT_IMPERATIVE_CATALOGUE,
	type ImperativeCatalogueItem,
} from "@semio-tech/imperative-core";
import {
	DAG_LOD_MODE_AUTOMATIC,
	dagLodAutomaticSelectLabel,
	dagPlayLodTierMenuLabel,
	dagPlayLodTiers,
	isDagDrawLodKind,
	type DagDrawLodKind,
	type DagLodModeKind,
	type DagReorganizeRequest,
} from "@semio-tech/dag-react";
import {
	DEFAULT_SEQUENCE_FIXTURE,
	parseSequenceFixtureJson,
	sequenceFixtureToJson,
	type SequenceFixtureV1,
	type SequenceStepV1,
} from "@semio-tech/sequence-core";

export const SEQUENCE_PLAY_APP_ID = "sequence-play";
export const SEQUENCE_PLAY_CONTROLLER_ID = "sequence-play";
export const SEQUENCE_PLAY_SURFACE_ID = "sequence.play/v1";
export const SEQUENCE_PLAY_BODY_KEY_MAIN = "sequence.play.main";
export const SEQUENCE_PLAY_WINDOW_KIND_ID = "sequence-main";
export const SEQUENCE_PLAY_DEFAULT_FIXTURE_JSON = sequenceFixtureToJson(DEFAULT_SEQUENCE_FIXTURE);
export const SEQUENCE_PLAY_LAYOUT = createStackLayout([SEQUENCE_PLAY_WINDOW_KIND_ID], ["Sequence"]);
export const SEQUENCE_PLAY_HIERARCHY_TAB_ID = "framework.panel.hierarchy";
export const SEQUENCE_PLAY_CATALOGUE_TAB_ID = "framework.panel.catalogue";
export const SEQUENCE_PLAY_INSPECTION_TAB_ID = "framework.panel.inspection";

export const SEQUENCE_ENGAGEMENT_REORGANIZE_ID = "sequence.tool.reorganize";
export const SEQUENCE_ENGAGEMENT_RUN_ID = "sequence.tool.run";
export const SEQUENCE_ENGAGEMENT_ORIENTATION_LR_ID = "sequence.layout.leftRight";
export const SEQUENCE_ENGAGEMENT_ORIENTATION_TB_ID = "sequence.layout.topBottom";

export type SequenceLayoutOrientation = "leftRight" | "topBottom";

export interface SequenceRunRequest {
	readonly epoch: number;
}

const DEFAULT_LAYER_SPACING = 120;
const DEFAULT_SIBLING_GAP = 40;

function sequencePlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
	return { controllerId: SEQUENCE_PLAY_CONTROLLER_ID, command, args };
}

function buildSequenceLayoutOptionsJson(layerSpacing: number, siblingGap: number, orientation: SequenceLayoutOrientation): string {
	return JSON.stringify({ layerSpacing, siblingGap, orientation });
}

/** @emoji 🧰 Sequence play footer toolbar. */
export function buildSequencePlayToolbarTools(controllerId: string, orientation: SequenceLayoutOrientation): AppTools {
	const layoutToggle = (id: string, label: string, value: SequenceLayoutOrientation): ToolLeaf => ({
		id,
		kind: "toggle",
		label,
		iconId: value === "leftRight" ? "arrow-right" : "arrow-down",
		pressed: orientation === value,
		controllerId,
		command: "setOrientation",
		args: { orientation: value },
	});
	return [
		toolCollection("execution", "play", [
			{ kind: "button", id: "sequence.run", label: "Run", iconId: "play", controllerId, command: "run" },
		]),
		toolCollection("layout", "layout-grid", [
			{ kind: "button", id: "sequence.reorganize", label: "Reorganize", iconId: "refresh-cw", controllerId, command: "reorganize" },
			layoutToggle("sequence.orientation.lr", "Left to right", "leftRight"),
			layoutToggle("sequence.orientation.tb", "Top to bottom", "topBottom"),
		]),
	];
}

// #region 🔖SequencePlayPanels
export function parseSequencePlayFixtureJson(json: string): SequenceFixtureV1 | null {
	return parseSequenceFixtureJson(json);
}

export function buildSequencePlayHierarchyTree(fixtureJson: string, selectedStepIds: readonly string[]): UiNode {
	const fixture = parseSequencePlayFixtureJson(fixtureJson);
	if (!fixture) {
		return {
			type: "tree",
			sections: [
				{
					id: "sequence-play-hierarchy.invalid",
					label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
					defaultOpen: true,
					items: [{ id: "sequence-play-hierarchy.invalid.msg", label: "Invalid sequence fixture" }],
				},
			],
		};
	}
	const stepItems: UiTreeItemNode[] = fixture.steps.map((step) => ({
		id: `sequence-play-hierarchy.step.${step.id}`,
		label: step.kind,
		description: step.id,
		command: sequencePlayCmd("setSelection", { ids: [step.id] }),
	}));
	const edgeItems: UiTreeItemNode[] = fixture.edges.map((edge) => ({
		id: `sequence-play-hierarchy.edge.${edge.id}`,
		label: `${edge.from} → ${edge.to}`,
		description: "click to disconnect",
		command: sequencePlayCmd("disconnectSteps", { from: edge.from, to: edge.to }),
	}));
	return {
		type: "tree",
		sections: [
			{
				id: "sequence-play-hierarchy.steps",
				label: "Steps",
				defaultOpen: true,
				items: stepItems.length ? stepItems : [{ id: "sequence-play-hierarchy.steps.empty", label: "(none)" }],
			},
			{
				id: "sequence-play-hierarchy.edges",
				label: "Edges",
				defaultOpen: false,
				items: edgeItems.length ? edgeItems : [{ id: "sequence-play-hierarchy.edges.empty", label: "(none)" }],
			},
		],
		selectedIds: selectedStepIds.map((id) => `sequence-play-hierarchy.step.${id}`),
	};
}

export function buildSequencePlayCatalogueTree(): UiNode {
	return {
		type: "tree",
		sections: DEFAULT_IMPERATIVE_CATALOGUE.sections.map((section) => ({
			id: `sequence-play-catalogue.${section.id}`,
			label: section.title,
			defaultOpen: true,
			items: section.items.map((item: ImperativeCatalogueItem) => ({
				id: `sequence-play-catalogue.kind.${item.kind}`,
				label: item.name,
				description: item.kind,
				command: sequencePlayCmd("addStep", { kind: item.kind }),
			})),
		})),
	};
}

function sequencePlayInspectorPatch(stepIds: readonly string[], field: string) {
	return sequencePlayCmd("patchSequenceSteps", { stepIds, field });
}

function sequencePlayInspectorNumberField(stepIds: readonly string[], fieldId: string, label: string, values: readonly number[], field: string): UiNode {
	const uniform = values.every((value) => value === values[0]);
	return {
		type: "field",
		id: fieldId,
		label,
		child: {
			type: "input",
			id: `${fieldId}.input`,
			inputKind: "number",
			value: uniform ? String(values[0] ?? "") : "",
			placeholder: uniform ? undefined : "—",
			onChange: sequencePlayInspectorPatch(stepIds, field),
		},
	};
}

function sequencePlayInspectorTextField(stepIds: readonly string[], fieldId: string, label: string, values: readonly string[], field: string): UiNode {
	const uniform = values.every((value) => value === values[0]);
	return {
		type: "field",
		id: fieldId,
		label,
		child: {
			type: "input",
			id: `${fieldId}.input`,
			inputKind: "text",
			value: uniform ? (values[0] ?? "") : "",
			placeholder: uniform ? undefined : "—",
			commit: "blur",
			onChange: sequencePlayInspectorPatch(stepIds, field),
		},
	};
}

function sequencePlayInspectorParamGroup(steps: readonly SequenceStepV1[]): UiInspectorFieldGroup | null {
	if (!steps.length) return null;
	const stepIds = steps.map((step) => step.id);
	const kind = steps[0]?.kind;
	if (!kind || steps.some((step) => step.kind !== kind)) return null;
	const item = DEFAULT_IMPERATIVE_CATALOGUE.sections.flatMap((section) => section.items).find((entry) => entry.kind === kind);
	if (!item?.inputs.length) return null;
	const fields: UiNode[] = item.inputs.map((input) => {
		const values = steps.map((step) => step.params[input.name]);
		const fieldId = `sequence-play-inspector.${kind}.${input.name}`;
		if (input.code === "N") {
			return sequencePlayInspectorNumberField(
				stepIds,
				fieldId,
				input.name,
				values.map((value) => (typeof value === "number" ? value : Number(value ?? 0))),
				input.name,
			);
		}
		return sequencePlayInspectorTextField(
			stepIds,
			fieldId,
			input.name,
			values.map((value) => String(value ?? "")),
			input.name,
		);
	});
	return { id: `sequence-play-inspector.params.${kind}`, label: kind, fields };
}

function sequencePlayInspectorBaseGroup(steps: readonly SequenceStepV1[]): UiInspectorFieldGroup {
	const stepIds = steps.map((step) => step.id);
	const fields: UiNode[] = [];
	if (stepIds.length === 1) {
		fields.push(uiInspectorReadonlyField("sequence-play-inspector.id", "Id", stepIds[0] ?? ""));
	} else {
		fields.push(uiInspectorReadonlyField("sequence-play-inspector.id", "Id", `${stepIds.length} selected`));
	}
	fields.push(
		uiInspectorReadonlyField("sequence-play-inspector.kind", "Kind", steps[0]?.kind ?? ""),
		{
			type: "field",
			id: "sequence-play-inspector.remove",
			label: "Actions",
			child: {
				type: "button",
				id: "sequence-play-inspector.remove.button",
				label: "Remove step",
				onClick: sequencePlayCmd("removeSequenceStep", { stepId: stepIds[0] }),
			},
		},
	);
	return { id: "sequence-play-inspector.base", label: "Step", fields };
}

export function buildSequencePlayInspectorTree(fixtureJson: string, selectedStepIds: readonly string[]): UiNode {
	const fixture = parseSequencePlayFixtureJson(fixtureJson);
	if (!fixture) {
		return uiDeclarativeSectionsToTree([
			{ type: "section", id: "sequence-play-inspector.invalid", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "Invalid sequence fixture" }] },
		]);
	}
	if (!selectedStepIds.length) {
		return uiDeclarativeSectionsToTree([
			{
				type: "section",
				id: "sequence-play-inspector.empty",
				label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
				children: [{ type: "text", value: "Select a step in the hierarchy." }],
			},
		]);
	}
	const steps = selectedStepIds
		.map((id) => fixture.steps.find((step) => step.id === id))
		.filter((step): step is SequenceStepV1 => Boolean(step));
	if (!steps.length) {
		return uiDeclarativeSectionsToTree([
			{ type: "section", id: "sequence-play-inspector.missing", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "Step not found" }] },
		]);
	}
	const groups: UiInspectorFieldGroup[] = [];
	const paramGroup = sequencePlayInspectorParamGroup(steps);
	if (paramGroup) groups.push(paramGroup);
	groups.push(sequencePlayInspectorBaseGroup(steps));
	return uiInspectorGroupsToTree(groups);
}
// #endregion 🔖SequencePlayPanels

/** @emoji 🎮 Sequence play controller. */
export class SequencePlayController extends Controller {
	readonly mainMode = new ModeRuntime("main", "Edit", undefined);
	private fixtureJson = SEQUENCE_PLAY_DEFAULT_FIXTURE_JSON;
	private engagementInput = "";
	private layerSpacing = DEFAULT_LAYER_SPACING;
	private siblingGap = DEFAULT_SIBLING_GAP;
	private orientation: SequenceLayoutOrientation = "leftRight";
	private reorganizeEpoch = 0;
	private reorganizeOptionsJson = buildSequenceLayoutOptionsJson(DEFAULT_LAYER_SPACING, DEFAULT_SIBLING_GAP, "leftRight");
	private runEpoch = 0;
	private lodMode: DagLodModeKind = DAG_LOD_MODE_AUTOMATIC;
	private lodModeByInstance: Record<string, DagLodModeKind> = {};
	private effectiveLod: DagDrawLodKind = "normal";
	private selectedStepIds: string[] = [];
	private interactionRevision = 0;
	private readonly snapshotListeners = new Set<() => void>();

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(SEQUENCE_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.rebuildShellMode();
	}

	getFixtureJson(): string {
		return this.fixtureJson;
	}

	getReorganize(): DagReorganizeRequest {
		return { epoch: this.reorganizeEpoch, optionsJson: this.reorganizeOptionsJson };
	}

	getRunRequest(): SequenceRunRequest {
		return { epoch: this.runEpoch };
	}

	lodModeForScope(scopeId: string): DagLodModeKind {
		return this.lodModeByInstance[scopeId] ?? this.lodMode;
	}

	getSelectedStepIds(): readonly string[] {
		return this.selectedStepIds;
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

	private commitFixture(next: SequenceFixtureV1): void {
		const json = sequenceFixtureToJson(next);
		if (json === this.fixtureJson) return;
		this.fixtureJson = json;
		this.interactionRevision += 1;
		this.notifySnapshot();
		this.emit();
	}

	private applyFixtureJson(json: string): void {
		const parsed = parseSequencePlayFixtureJson(json);
		if (!parsed || sequenceFixtureToJson(parsed) === this.fixtureJson) return;
		this.fixtureJson = sequenceFixtureToJson(parsed);
		this.interactionRevision += 1;
		this.notifySnapshot();
		this.emit();
	}

	private nextStepId(fixture: SequenceFixtureV1): string {
		let serial = 1;
		while (fixture.steps.some((step) => step.id === `step-${serial}`)) {
			serial += 1;
		}
		return `step-${serial}`;
	}

	private addStep(kind: string): void {
		const fixture = parseSequencePlayFixtureJson(this.fixtureJson);
		if (!fixture) return;
		const id = this.nextStepId(fixture);
		const x = 40 + fixture.steps.length * 280;
		this.commitFixture({
			...fixture,
			steps: [...fixture.steps, { id, kind, params: {}, x, y: 40 }],
		});
		this.selectedStepIds = [id];
	}

	private removeSequenceStep(stepId: string): void {
		const fixture = parseSequencePlayFixtureJson(this.fixtureJson);
		if (!fixture) return;
		this.selectedStepIds = this.selectedStepIds.filter((id) => id !== stepId);
		this.commitFixture({
			...fixture,
			steps: fixture.steps.filter((step) => step.id !== stepId),
			edges: fixture.edges.filter((edge) => edge.from !== stepId && edge.to !== stepId),
		});
	}

	private disconnectSteps(from: string, to: string): void {
		const fixture = parseSequencePlayFixtureJson(this.fixtureJson);
		if (!fixture) return;
		this.commitFixture({
			...fixture,
			edges: fixture.edges.filter((edge) => !(edge.from === from && edge.to === to)),
		});
	}

	private patchSequenceSteps(stepIds: readonly string[], field: string, value: unknown): void {
		const fixture = parseSequencePlayFixtureJson(this.fixtureJson);
		if (!fixture || !stepIds.length) return;
		this.commitFixture({
			...fixture,
			steps: fixture.steps.map((step) =>
				stepIds.includes(step.id) ? { ...step, params: { ...step.params, [field]: value } } : step,
			),
		});
	}

	private lodMeasure(scopeId: string): WindowMeasure {
		return {
			kind: "select",
			id: `${scopeId}-lod`,
			label: "LOD",
			value: this.lodModeForScope(scopeId),
			items: [
				{ id: "automatic", value: DAG_LOD_MODE_AUTOMATIC, label: dagLodAutomaticSelectLabel(this.effectiveLod) },
				...dagPlayLodTiers().map((tier) => ({ id: tier, value: tier, label: dagPlayLodTierMenuLabel(tier) })),
			],
			onChange: { controllerId: SEQUENCE_PLAY_CONTROLLER_ID, command: "setLodMode", args: { instanceId: scopeId } },
		};
	}

	private windowMeasures(): readonly WindowMeasure[] {
		return [this.lodMeasure(SEQUENCE_PLAY_WINDOW_KIND_ID)];
	}

	private syncReorganizeOptionsJson(): void {
		this.reorganizeOptionsJson = buildSequenceLayoutOptionsJson(this.layerSpacing, this.siblingGap, this.orientation);
	}

	private triggerReorganize(): void {
		this.syncReorganizeOptionsJson();
		this.reorganizeEpoch += 1;
		this.rebuildShellMode();
		this.emit();
	}

	private triggerRun(): void {
		this.runEpoch += 1;
		this.emit();
	}

	private windowEngagement(): WindowEngagement {
		return {
			sessionActive: false,
			input: {
				id: "engagement-input",
				value: this.engagementInput,
				placeholder: "Run, reorganize, lr, tb",
				onChange: sequencePlayCmd("engagementInput"),
				onSubmit: sequencePlayCmd("engagementSubmit"),
			},
			possibleEngagements: [
				{ id: SEQUENCE_ENGAGEMENT_RUN_ID, label: "Run", command: sequencePlayCmd("run") },
				{ id: SEQUENCE_ENGAGEMENT_REORGANIZE_ID, label: "Reorganize", command: sequencePlayCmd("reorganize") },
				{ id: SEQUENCE_ENGAGEMENT_ORIENTATION_LR_ID, label: "Left to Right", command: sequencePlayCmd("setOrientation", { orientation: "leftRight" }) },
				{ id: SEQUENCE_ENGAGEMENT_ORIENTATION_TB_ID, label: "Top to Bottom", command: sequencePlayCmd("setOrientation", { orientation: "topBottom" }) },
			],
			controls: [
				{
					kind: "slider",
					id: "sequence-layer-spacing",
					label: "Layer spacing",
					value: this.layerSpacing,
					min: 40,
					max: 320,
					step: 10,
					onChange: sequencePlayCmd("setSpacing", { field: "layerSpacing" }),
				},
				{
					kind: "slider",
					id: "sequence-sibling-gap",
					label: "Sibling gap",
					value: this.siblingGap,
					min: 10,
					max: 160,
					step: 5,
					onChange: sequencePlayCmd("setSpacing", { field: "siblingGap" }),
				},
			],
			status: [{ id: "sequence-layout-orientation", text: this.orientation === "leftRight" ? "Left to right" : "Top to bottom" }],
		};
	}

	private rebuildShellMode(): void {
		this.mainMode.tools = buildSequencePlayToolbarTools(SEQUENCE_PLAY_CONTROLLER_ID, this.orientation);
		this.mainMode.windowKinds = [
			new WindowKindRuntime(SEQUENCE_PLAY_WINDOW_KIND_ID, "Sequence", SEQUENCE_PLAY_BODY_KEY_MAIN, undefined, this.windowMeasures(), this.windowEngagement()),
		];
		for (const windowKind of this.mainMode.windowKinds) {
			enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Sequence play window "${windowKind.id}"`);
		}
	}

	private applyEngagement(value: string): void {
		const trimmed = value.trim().toLowerCase();
		if (!trimmed) return;
		if (trimmed === "run") {
			this.triggerRun();
			return;
		}
		if (trimmed === "reorganize" || trimmed === "layout") {
			this.triggerReorganize();
			return;
		}
		if (trimmed === "lr" || trimmed === "left" || trimmed === "left to right") {
			this.orientation = "leftRight";
			this.syncReorganizeOptionsJson();
			this.rebuildShellMode();
			this.emit();
			return;
		}
		if (trimmed === "tb" || trimmed === "top" || trimmed === "top to bottom") {
			this.orientation = "topBottom";
			this.syncReorganizeOptionsJson();
			this.rebuildShellMode();
			this.emit();
			return;
		}
		this.engagementInput = "";
		this.rebuildShellMode();
		this.emit();
	}

	override run(command: string, args?: unknown): void {
		if (command === "engagementInput") {
			const value = (args as { value?: string }).value;
			if (typeof value === "string" && value !== this.engagementInput) {
				this.engagementInput = value;
				this.rebuildShellMode();
				this.emit();
			}
			return;
		}
		if (command === "engagementSubmit") {
			const value = (args as { value?: string }).value ?? this.engagementInput;
			this.applyEngagement(value);
			return;
		}
		if (command === "setSpacing") {
			const field = (args as { field?: string; value?: number }).field;
			const value = (args as { value?: number }).value;
			if (typeof value !== "number") return;
			if (field === "layerSpacing") this.layerSpacing = value;
			else if (field === "siblingGap") this.siblingGap = value;
			else return;
			this.syncReorganizeOptionsJson();
			this.rebuildShellMode();
			this.emit();
			return;
		}
		if (command === "setOrientation") {
			const orientation = (args as { orientation?: SequenceLayoutOrientation }).orientation;
			if (orientation !== "leftRight" && orientation !== "topBottom") return;
			this.orientation = orientation;
			this.syncReorganizeOptionsJson();
			this.rebuildShellMode();
			this.emit();
			return;
		}
		if (command === "reorganize") {
			this.triggerReorganize();
			return;
		}
		if (command === "run") {
			this.triggerRun();
			return;
		}
		if (command === "setFixtureJson") {
			const json = (args as { json?: string }).json;
			if (typeof json === "string") {
				this.applyFixtureJson(json);
			}
			return;
		}
		if (command === "addStep") {
			const kind = (args as { kind?: string }).kind;
			if (typeof kind === "string") {
				this.addStep(kind);
			}
			return;
		}
		if (command === "disconnectSteps") {
			const from = (args as { from?: string }).from;
			const to = (args as { to?: string }).to;
			if (typeof from === "string" && typeof to === "string") {
				this.disconnectSteps(from, to);
			}
			return;
		}
		if (command === "removeSequenceStep") {
			const stepId = (args as { stepId?: string }).stepId;
			if (typeof stepId === "string") {
				this.removeSequenceStep(stepId);
			}
			return;
		}
		if (command === "patchSequenceSteps") {
			const stepIds = (Array.isArray((args as { stepIds?: string[] }).stepIds) ? (args as { stepIds?: string[] }).stepIds : []).map(String).filter(Boolean);
			const field = (args as { field?: string }).field;
			const value = (args as { value?: unknown }).value ?? (args as { pressed?: boolean }).pressed;
			if (!stepIds.length || typeof field !== "string") return;
			this.patchSequenceSteps(stepIds, field, value);
			return;
		}
		if (command === "setSelection") {
			const ids = (args as { ids?: string[] }).ids;
			if (!Array.isArray(ids)) return;
			const next = [...new Set(ids.filter((id) => typeof id === "string"))];
			if (JSON.stringify(next) === JSON.stringify(this.selectedStepIds)) return;
			this.selectedStepIds = next;
			this.interactionRevision += 1;
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "setLodMode") {
			const { value, instanceId } = args as { value?: string; instanceId?: string };
			const scopeId = instanceId ?? SEQUENCE_PLAY_WINDOW_KIND_ID;
			if (typeof value !== "string") return;
			if (value !== DAG_LOD_MODE_AUTOMATIC && !isDagDrawLodKind(value)) return;
			this.lodModeByInstance = { ...this.lodModeByInstance, [scopeId]: value as DagLodModeKind };
			if (scopeId === SEQUENCE_PLAY_WINDOW_KIND_ID) {
				this.lodMode = value as DagLodModeKind;
			}
			this.rebuildShellMode();
			this.emit();
			return;
		}
		if (command === "setEffectiveLod") {
			const { lod, instanceId } = args as { lod?: DagDrawLodKind; instanceId?: string };
			const scopeId = instanceId ?? SEQUENCE_PLAY_WINDOW_KIND_ID;
			if (!lod || !isDagDrawLodKind(lod)) return;
			if (scopeId !== SEQUENCE_PLAY_WINDOW_KIND_ID) return;
			if (this.effectiveLod === lod) return;
			this.effectiveLod = lod;
			this.rebuildShellMode();
			this.emit();
			return;
		}
	}
}

function buildSequencePlayMainDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
	return buildSequenceWindowBody(SEQUENCE_PLAY_SURFACE_ID, SEQUENCE_PLAY_CONTROLLER_ID, SEQUENCE_PLAY_WINDOW_KIND_ID);
}

export function registerSequencePlayDeclarativeBodies(): void {
	registerWindowBody(SEQUENCE_PLAY_BODY_KEY_MAIN, buildSequencePlayMainDeclarativeBody);
}

export function buildSequencePlayAppRuntime(controller: SequencePlayController): AppRuntime {
	return createPlayAppRuntime(SEQUENCE_PLAY_APP_ID, "Sequence", controller, SEQUENCE_PLAY_LAYOUT, controller.mainMode);
}

/** @emoji 🛝 Sequence playground app. */
export class PlaygroundSequence extends Playground {
	readonly id = SEQUENCE_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new SequencePlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildSequencePlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerSequencePlayDeclarativeBodies();
	}
}

export { sequencePlayCmd };

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("SequencePlayController", () => {
		it("default fixture json is valid", () => {
			expect(SEQUENCE_PLAY_DEFAULT_FIXTURE_JSON).toContain("sequence.fixture/v1");
			expect(SEQUENCE_PLAY_DEFAULT_FIXTURE_JSON).toContain("step-2");
		});
		it("addStep and run bump interaction state", () => {
			const bus = new CommandBus();
			const ctrl = new SequencePlayController(bus, () => {});
			ctrl.run("addStep", { kind: "wait.delay" });
			expect(ctrl.getFixtureJson()).toContain("wait.delay");
			const before = ctrl.getRunRequest().epoch;
			ctrl.run("run");
			expect(ctrl.getRunRequest().epoch).toBeGreaterThan(before);
		});
		it("disconnectSteps removes edge from fixture", () => {
			const bus = new CommandBus();
			const ctrl = new SequencePlayController(bus, () => {});
			ctrl.run("disconnectSteps", { from: "step-1", to: "step-2" });
			expect(ctrl.getFixtureJson()).not.toContain('"from":"step-1","to":"step-2"');
		});
	});
}

if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest && import.meta.env.PUZZLE_PLAY_ENTRY === "sequence") {
	bootstrapElementsSurfaceChromeDocument("system");
	void (async () => {
		await import("./globals.css");
		const { bootSequencePlay } = await import("@semio-tech/framework-playground-renderer-react/sequence");
		bootSequencePlay(new PlaygroundSequence());
	})();
}
