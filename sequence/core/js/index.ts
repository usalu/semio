// #region 🧲Header
/** @emoji 📜 Sequence play app — execution-flow canvas editor. */
// #endregion 🧲Header


export * from "./internal.ts";

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	WindowKindRuntime,
	buildSequenceWindowBody,
	createPlayAppRuntime,
	createDefaultLayout,
	createJackPlayWindowEngagement,
	enforcePlaygroundWindowEngagementInput,
	registerWindowBody,
	buildWriterWindowBody,
	JackHoverBridge,
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
  createPlaygroundApp,
  createProductPlaygroundPlatform,
} from "@semio-tech/framework-playground-core";
import {
	type EffectLogEntry,
	type ImperativeCatalogueItem,
	type RunResult,
	imperativeExtensionHost,
} from "@semio-tech/imperative-core";
import { sequenceStepCatalogueItemDragData } from "@semio-tech/sequence-react";
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
	type SequenceFixture,
	type SequenceStep,
} from "./internal.ts";
import { createWriterDocument, type WriterDocument } from "@semio-tech/writer-core";
import { runJackOnBoardFixture } from "@semio-tech/graph-dsl-core";

export const SEQUENCE_PLAY_APP_ID = "sequence-play";
export const SEQUENCE_PLAY_CONTROLLER_ID = "sequence-play";
export const SEQUENCE_PLAY_BODY_KEY_MAIN = "sequence.play.main";
export const SEQUENCE_PLAY_BODY_KEY_SCRIPT = "sequence.play.script";
export const SEQUENCE_PLAY_SURFACE_ID = "sequence.play";
export const SEQUENCE_PLAY_SCRIPT_SURFACE_ID = "sequence.play.script";
export const SEQUENCE_PLAY_WINDOW_KIND_ID = "sequence-main";
export const SEQUENCE_PLAY_SCRIPT_WINDOW_KIND_ID = "sequence-script";
export const SEQUENCE_PLAY_WINDOW_KIND_JACK = "sequence-jack";
export const SEQUENCE_PLAY_WINDOW_KIND_COMPILED_DAG = "sequence-compiled-dag";
export const SEQUENCE_PLAY_SURFACE_ID_COMPILED_DAG = "sequence.play.compiled-dag";
export const SEQUENCE_PLAY_BODY_KEY_COMPILED_DAG = "sequence.play.compiled-dag";
export const SEQUENCE_PLAY_SURFACE_ID_JACK = "sequence.play.jack";
export const SEQUENCE_PLAY_BODY_KEY_JACK = "sequence.play.jack";
export const SEQUENCE_PLAY_DEFAULT_JACK_QUERY = "MATCH (n:step) RETURN n.name";
export const SEQUENCE_PLAY_DEFAULT_FIXTURE_JSON = sequenceFixtureToJson(DEFAULT_SEQUENCE_FIXTURE);
export const SEQUENCE_PLAY_LAYOUT = createDefaultLayout(
	[SEQUENCE_PLAY_WINDOW_KIND_ID, SEQUENCE_PLAY_SCRIPT_WINDOW_KIND_ID, SEQUENCE_PLAY_WINDOW_KIND_JACK, SEQUENCE_PLAY_WINDOW_KIND_COMPILED_DAG],
	"row",
	[40, 20, 20, 20],
	["Sequence", "Compiled Script", "Jack", "Compiled DAG"],
);
export const SEQUENCE_PLAY_HIERARCHY_TAB_ID = "framework.panel.hierarchy";
export const SEQUENCE_PLAY_CATALOGUE_TAB_ID = "framework.panel.catalogue";
export const SEQUENCE_PLAY_INSPECTION_TAB_ID = "framework.panel.inspection";

export const SEQUENCE_ENGAGEMENT_REORGANIZE_ID = "sequence.tool.reorganize";
export const SEQUENCE_ENGAGEMENT_RUN_ID = "sequence.tool.run";
export const SEQUENCE_ENGAGEMENT_STOP_ID = "sequence.tool.stop";
export const SEQUENCE_ENGAGEMENT_ORIENTATION_LR_ID = "sequence.layout.leftRight";
export const SEQUENCE_ENGAGEMENT_ORIENTATION_TB_ID = "sequence.layout.topBottom";

export type SequenceLayoutOrientation = "leftRight" | "topBottom";

export interface SequenceRunRequest {
	readonly epoch: number;
}

export interface SequenceRunStopRequest {
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
			{ kind: "button", id: "sequence.stop", label: "Stop", iconId: "square", controllerId, command: "stop" },
		]),
		toolCollection("layout", "layout-grid", [
			{ kind: "button", id: "sequence.reorganize", label: "Reorganize", iconId: "refresh-cw", controllerId, command: "reorganize" },
			layoutToggle("sequence.orientation.lr", "Left to right", "leftRight"),
			layoutToggle("sequence.orientation.tb", "Top to bottom", "topBottom"),
		]),
	];
}

// #region 🔖SequencePlayPanels
export function parseSequencePlayFixtureJson(json: string): SequenceFixture | null {
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
	const catalogue = imperativeExtensionHost.getCatalogue();
	return {
		type: "tree",
		sections: catalogue.sections.map((section) => ({
			id: `sequence-play-catalogue.${section.id}`,
			label: section.title,
			defaultOpen: true,
			items: section.items.map((item: ImperativeCatalogueItem) => ({
				id: `sequence-play-catalogue.kind.${item.kind}`,
				label: item.name,
				description: item.kind,
				draggable: true,
				dragData: sequenceStepCatalogueItemDragData(item),
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

function sequencePlayInspectorParamGroup(steps: readonly SequenceStep[]): UiInspectorFieldGroup | null {
	if (!steps.length) return null;
	const stepIds = steps.map((step) => step.id);
	const kind = steps[0]?.kind;
	if (!kind || steps.some((step) => step.kind !== kind)) return null;
	const item = imperativeExtensionHost
		.getCatalogue()
		.sections.flatMap((section) => section.items)
		.find((entry) => entry.kind === kind);
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

function sequencePlayInspectorBaseGroup(steps: readonly SequenceStep[]): UiInspectorFieldGroup {
	const stepIds = steps.map((step) => step.id);
	const fields: UiNode[] = [];
	if (stepIds.length === 1) {
		fields.push(uiInspectorReadonlyField("sequence-play-inspector.id", "Id", stepIds[0] ?? ""));
	} else {
		fields.push(uiInspectorReadonlyField("sequence-play-inspector.id", "Id", `${stepIds.length} selected`));
	}
	if (stepIds.length === 1 && steps[0]?.kind.startsWith("control.")) {
		fields.push({
			type: "field",
			id: "sequence-play-inspector.collapse",
			label: "Bodies",
			child: {
				type: "button",
				id: "sequence-play-inspector.collapse.button",
				label: steps[0].collapsed ? "Expand bodies" : "Collapse bodies",
				onClick: sequencePlayCmd("toggleStepCollapsed", { stepId: stepIds[0] }),
			},
		});
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

export function buildSequencePlayInspectorTree(
	fixtureJson: string,
	selectedStepIds: readonly string[],
	effectLog: readonly EffectLogEntry[] = [],
): UiNode {
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
		.filter((step): step is SequenceStep => Boolean(step));
	if (!steps.length) {
		return uiDeclarativeSectionsToTree([
			{ type: "section", id: "sequence-play-inspector.missing", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, children: [{ type: "text", value: "Step not found" }] },
		]);
	}
	const groups: UiInspectorFieldGroup[] = [];
	const paramGroup = sequencePlayInspectorParamGroup(steps);
	if (paramGroup) groups.push(paramGroup);
	groups.push(sequencePlayInspectorBaseGroup(steps));
	const runLogFields: UiNode[] =
		effectLog.length === 0
			? [{ type: "text", value: "Run to see effects." }]
			: effectLog.map((entry, index) => ({
					type: "text" as const,
					value: entry.error ? `${entry.kind} · ${entry.error}` : entry.kind,
				}));
	groups.push({ id: "sequence-play-inspector.run-log", label: "Run Log", fields: runLogFields });
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
	private runStopEpoch = 0;
	private lodMode: DagLodModeKind = DAG_LOD_MODE_AUTOMATIC;
	private lodModeByInstance: Record<string, DagLodModeKind> = {};
	private effectiveLod: DagDrawLodKind = "normal";
	private compiledText = "";
	private compiledWireLiteral = "";
	private effectLog: EffectLogEntry[] = [];
	private interactionRevision = 0;
	private readonly snapshotListeners = new Set<() => void>();
	private readonly jackBridge = new JackHoverBridge();
	private jackEngagementInput = "";

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(SEQUENCE_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.jackBridge.setJackQueryText(SEQUENCE_PLAY_DEFAULT_JACK_QUERY);
		this.jackBridge.setFixtureJson(this.getFixtureJson());
		this.jackBridge.bindPointerFocus(this.pointerFocus);
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

	getRunStopRequest(): SequenceRunStopRequest {
		return { epoch: this.runStopEpoch };
	}

	lodModeForScope(scopeId: string): DagLodModeKind {
		return this.lodModeByInstance[scopeId] ?? this.lodMode;
	}

	getSelectedStepIds(): readonly string[] {
		return this.pointerFocus.getSnapshot().selection;
	}

	private setSelectedStepIds(ids: readonly string[]): void {
		const next = [...new Set(ids.filter((id) => typeof id === "string"))];
		if (JSON.stringify(next) === JSON.stringify(this.getSelectedStepIds())) return;
		this.pointerFocus.setSelection(next);
		this.interactionRevision += 1;
		this.notifySnapshot();
		this.emit();
	}

	getCompiledText(): string {
		return this.compiledText;
	}

	getCompiledWireLiteral(): string {
		return this.compiledWireLiteral;
	}

	getWriterDocumentCompiledDag(): WriterDocument {
		return createWriterDocument({ id: "sequence-compiled-dag", languageId: "wire", text: this.compiledWireLiteral });
	}

	getEffectLog(): readonly EffectLogEntry[] {
		return this.effectLog;
	}

	getExtensionRevision(): number {
		return imperativeExtensionHost.getRevision();
	}

	getInteractionRevision(): number {
		return this.interactionRevision;
	}

	subscribeSnapshot(listener: () => void): () => void {
		this.snapshotListeners.add(listener);
		const unsubJack = this.jackBridge.subscribe(listener);
		return () => {
			this.snapshotListeners.delete(listener);
			unsubJack();
		};
	}

	getJackQueryText(): string {
		return this.jackBridge.getJackQueryText();
	}

	getWriterDocumentJack(): WriterDocument {
		return createWriterDocument({ id: "sequence-jack", languageId: "jack", text: this.jackBridge.getJackQueryText() });
	}

	getJackHoverOccurrences(): readonly { readonly start: number; readonly end: number }[] {
		return this.jackBridge.getJackHoverOccurrences();
	}

	getJackSelectOccurrences(): readonly { readonly start: number; readonly end: number }[] {
		return this.jackBridge.getJackSelectOccurrences();
	}

	getHoverEpoch(): number {
		return this.jackBridge.getHoverEpoch();
	}

	getSelectEpoch(): number {
		return this.jackBridge.getSelectEpoch();
	}

	getGraphHighlightedNodeIds(): readonly string[] {
		return this.jackBridge.getGraphHoveredNodeIds();
	}

	private notifySnapshot(): void {
		for (const listener of this.snapshotListeners) {
			listener();
		}
	}

	private commitFixture(next: SequenceFixture): void {
		const json = sequenceFixtureToJson(next);
		if (json === this.fixtureJson) return;
		this.fixtureJson = json;
		this.jackBridge.setFixtureJson(this.fixtureJson);
		this.interactionRevision += 1;
		this.notifySnapshot();
		this.emit();
	}

	private applyFixtureJson(json: string): void {
		const parsed = parseSequencePlayFixtureJson(json);
		if (!parsed || sequenceFixtureToJson(parsed) === this.fixtureJson) return;
		this.fixtureJson = sequenceFixtureToJson(parsed);
		this.jackBridge.setFixtureJson(this.fixtureJson);
		this.interactionRevision += 1;
		this.notifySnapshot();
		this.emit();
	}

	private setCompiledText(text: string): void {
		if (text === this.compiledText) return;
		this.compiledText = text;
		this.interactionRevision += 1;
		this.notifySnapshot();
		this.emit();
	}

	private setCompiledWireLiteral(text: string): void {
		if (text === this.compiledWireLiteral) return;
		this.compiledWireLiteral = text;
		this.interactionRevision += 1;
		this.notifySnapshot();
		this.emit();
	}

	private setRunResult(result: RunResult): void {
		this.effectLog = [...result.effects];
		this.interactionRevision += 1;
		this.notifySnapshot();
		this.emit();
	}

	private nextStepId(fixture: SequenceFixture): string {
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
		this.setSelectedStepIds([id]);
	}

	private collectRemoveStepIds(fixture: SequenceFixture, stepId: string): string[] {
		const ids = [stepId];
		const step = fixture.steps.find((entry) => entry.id === stepId);
		if (step?.kind.startsWith("control.")) {
			for (const member of fixture.steps) {
				if (member.slot?.owner === stepId) ids.push(member.id);
			}
		}
		return ids;
	}

	private removeSequenceStep(stepId: string): void {
		const fixture = parseSequencePlayFixtureJson(this.fixtureJson);
		if (!fixture) return;
		const removeIds = new Set(this.collectRemoveStepIds(fixture, stepId));
		this.setSelectedStepIds(this.getSelectedStepIds().filter((id) => !removeIds.has(id)));
		this.commitFixture({
			...fixture,
			steps: fixture.steps.filter((step) => !removeIds.has(step.id)),
			edges: fixture.edges.filter((edge) => !removeIds.has(edge.from) && !removeIds.has(edge.to)),
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

	private triggerStop(): void {
		this.runStopEpoch += 1;
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
				{ id: SEQUENCE_ENGAGEMENT_STOP_ID, label: "Stop", command: sequencePlayCmd("stop") },
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

	private scriptWindowEngagement(): WindowEngagement {
		return {
			sessionActive: false,
			input: {
				id: "sequence-script-engagement",
				value: "",
				placeholder: "Compiled script is read-only",
				onChange: sequencePlayCmd("engagementInput"),
				onSubmit: sequencePlayCmd("engagementSubmit"),
			},
			possibleEngagements: [],
			controls: [],
			status: [{ id: "sequence-script-status", text: this.compiledText ? "Compiled" : "Empty" }],
		};
	}

	private jackEngagement(): WindowEngagement {
		return createJackPlayWindowEngagement(SEQUENCE_PLAY_WINDOW_KIND_JACK, SEQUENCE_PLAY_CONTROLLER_ID, this.jackEngagementInput);
	}

	private compiledDagWindowEngagement(): WindowEngagement {
		return {
			sessionActive: false,
			input: {
				id: "sequence-compiled-dag-engagement",
				value: "",
				placeholder: "Compiled DAG is read-only",
				onChange: sequencePlayCmd("engagementInput"),
				onSubmit: sequencePlayCmd("engagementSubmit"),
			},
			possibleEngagements: [],
			controls: [],
			status: [{ id: "sequence-compiled-dag-status", text: this.compiledWireLiteral ? "Compiled" : "Empty" }],
		};
	}

	private rebuildShellMode(): void {
		this.mainMode.tools = buildSequencePlayToolbarTools(SEQUENCE_PLAY_CONTROLLER_ID, this.orientation);
		this.mainMode.windowKinds = [
			new WindowKindRuntime(SEQUENCE_PLAY_WINDOW_KIND_ID, "Sequence", SEQUENCE_PLAY_BODY_KEY_MAIN, undefined, this.windowMeasures(), this.windowEngagement()),
			new WindowKindRuntime(
				SEQUENCE_PLAY_SCRIPT_WINDOW_KIND_ID,
				"Compiled Script",
				SEQUENCE_PLAY_BODY_KEY_SCRIPT,
				undefined,
				[],
				this.scriptWindowEngagement(),
			),
			new WindowKindRuntime(SEQUENCE_PLAY_WINDOW_KIND_JACK, "Jack", SEQUENCE_PLAY_BODY_KEY_JACK, undefined, undefined, this.jackEngagement()),
			new WindowKindRuntime(SEQUENCE_PLAY_WINDOW_KIND_COMPILED_DAG, "Compiled DAG", SEQUENCE_PLAY_BODY_KEY_COMPILED_DAG, undefined, [], this.compiledDagWindowEngagement()),
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
		if (command === "jackEngagementInput") {
			const value = (args as { value?: string }).value;
			if (typeof value === "string" && value !== this.jackEngagementInput) {
				this.jackEngagementInput = value;
				this.rebuildShellMode();
				this.emit();
			}
			return;
		}
		if (command === "setJackQuery") {
			const text = (args as { text?: string }).text;
			if (typeof text === "string") {
				this.jackBridge.setJackQueryText(text);
				this.notifySnapshot();
				this.emit();
			}
			return;
		}
		if (command === "setJackHover") {
			this.jackBridge.setJackHover((args as { offset?: number | null }).offset ?? null);
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "setJackSelect") {
			this.jackBridge.setJackSelect((args as { start: number; end: number } | null) ?? null);
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "setGraphHover") {
			this.jackBridge.setGraphHover((args as { id?: string | null }).id ?? null);
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "setGraphSelect") {
			const ids = (args as { ids?: readonly string[] }).ids ?? [];
			this.jackBridge.setGraphSelect(ids);
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "runJackQuery") {
			runJackOnBoardFixture(this.getFixtureJson(), this.jackBridge.getJackQueryText());
			this.notifySnapshot();
			this.emit();
			return;
		}
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
		if (command === "stop") {
			this.triggerStop();
			return;
		}
		if (command === "toggleStepCollapsed") {
			const stepId = (args as { stepId?: string }).stepId;
			if (typeof stepId !== "string") return;
			const fixture = parseSequencePlayFixtureJson(this.fixtureJson);
			if (!fixture) return;
			this.commitFixture({
				...fixture,
				steps: fixture.steps.map((step) =>
					step.id === stepId && step.kind.startsWith("control.")
						? { ...step, collapsed: !step.collapsed }
						: step,
				),
			});
			return;
		}
		if (command === "setFixtureJson") {
			const json = (args as { json?: string }).json;
			if (typeof json === "string") {
				this.applyFixtureJson(json);
			}
			return;
		}
		if (command === "setCompiledText") {
			const text = (args as { text?: string }).text;
			if (typeof text === "string") {
				this.setCompiledText(text);
			}
			return;
		}
		if (command === "setCompiledWireLiteral") {
			const text = (args as { text?: string }).text;
			if (typeof text === "string") {
				this.setCompiledWireLiteral(text);
			}
			return;
		}
		if (command === "setRunResult") {
			const result = (args as { result?: RunResult }).result;
			if (result && Array.isArray(result.effects)) {
				this.setRunResult(result);
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
			this.setSelectedStepIds(ids);
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

function buildSequencePlayScriptDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
	return buildWriterWindowBody(SEQUENCE_PLAY_SCRIPT_SURFACE_ID, SEQUENCE_PLAY_CONTROLLER_ID, SEQUENCE_PLAY_SCRIPT_WINDOW_KIND_ID);
}

function buildSequencePlayJackDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
	return buildWriterWindowBody(SEQUENCE_PLAY_SURFACE_ID_JACK, SEQUENCE_PLAY_CONTROLLER_ID, SEQUENCE_PLAY_WINDOW_KIND_JACK);
}

function buildSequencePlayCompiledDagDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
	return buildWriterWindowBody(SEQUENCE_PLAY_SURFACE_ID_COMPILED_DAG, SEQUENCE_PLAY_CONTROLLER_ID, SEQUENCE_PLAY_WINDOW_KIND_COMPILED_DAG);
}

export function registerSequencePlayDeclarativeBodies(): void {
	registerWindowBody(SEQUENCE_PLAY_BODY_KEY_MAIN, buildSequencePlayMainDeclarativeBody);
	registerWindowBody(SEQUENCE_PLAY_BODY_KEY_SCRIPT, buildSequencePlayScriptDeclarativeBody);
	registerWindowBody(SEQUENCE_PLAY_BODY_KEY_JACK, buildSequencePlayJackDeclarativeBody);
	registerWindowBody(SEQUENCE_PLAY_BODY_KEY_COMPILED_DAG, buildSequencePlayCompiledDagDeclarativeBody);
}

export function buildSequencePlayAppRuntime(controller: SequencePlayController): AppRuntime {
	return createPlayAppRuntime(SEQUENCE_PLAY_APP_ID, "Sequence", controller, SEQUENCE_PLAY_LAYOUT, controller.mainMode);
}

export { sequencePlayCmd };

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("SequencePlayController", () => {
		it("default fixture json is valid", () => {
			expect(SEQUENCE_PLAY_DEFAULT_FIXTURE_JSON).toContain("sequence.fixture");
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
		it("layout exposes sequence, compiled script, and jack windows", () => {
			expect(SEQUENCE_PLAY_LAYOUT.root.kind).toBe("row");
			expect(SEQUENCE_PLAY_SCRIPT_WINDOW_KIND_ID).toBe("sequence-script");
			expect(SEQUENCE_PLAY_WINDOW_KIND_JACK).toBe("sequence-jack");
		});
	});
}

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";

/** @emoji 🧩 S program definition for sequence. */
export function buildSequenceProgramDefinition(): PlatformDefinition {
	return {
		id: "sequence",
		name: "Sequence",
		apiVersion: "1",
		apps: [{ id: "sequence", label: "Sequence", controllerId: SEQUENCE_PLAY_CONTROLLER_ID, modes: [{ id: "edit", label: "Edit" }], defaultModeId: "edit" }],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension

//#region 🔖Play

/** @emoji 🛝 Sequence playground app. */


export const sequencePlayAppDefinition = createPlaygroundApp({
	id: SEQUENCE_PLAY_APP_ID,
	label: "Sequence",
	controllerId: "sequence-play",
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	devHost: {
		playEntryKind: "sequence",
		resolveDedupe: ["react", "react-dom", "@semio-tech/sequence-react"],
		watchIgnored: ["../core/lib.rs", "../../imperative/**", "../core/target/**", "../core/pkg/**"],
		optimizeDeps: { include: ["react", "react-dom"] },
	},
	createRuntime: () => {
		const runtime = createProductPlaygroundPlatform(SEQUENCE_PLAY_APP_ID);
			const ctrl = new SequencePlayController(runtime.commandBus, () => runtime.notify());
			runtime.addApp(buildSequencePlayAppRuntime(ctrl));
			return runtime;
	},
	registerBodies: () => {
		registerSequencePlayDeclarativeBodies();
	},
	bootRenderer: async (pg) => {
		const { bootSequencePlay } = await import("@semio-tech/framework-playground-renderer-react/sequence");
		bootSequencePlay(pg);
	},
});
//#endregion 🔖Play
