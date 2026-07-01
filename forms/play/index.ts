// #region 🧲Header
/** @emoji 📋 Forms play harness on `@semio-tech/framework-playground-core`. */
// #endregion 🧲Header

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	Platform,
	Playground,
	WindowKindRuntime,
	buildFormsWindowBody,
	createDefaultLayout,
	createPlayAppRuntime,
	createProductPlaygroundPlatform,
	isPlaygroundFixtureLocked,
	isPlaygroundNoFixtureId,
	PLAYGROUND_NO_FIXTURE_ID,
	playgroundResolvedFixtureId,
	registerWindowBody,
	FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
	FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
	FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
	uiDeclarativeSectionsToTree,
	type AppTools,
	type CommandDescriptor,
	type PlaygroundFixtureCatalog,
	type PlaygroundFixtureHost,
	type ToolLeaf,
	toolCollection,
	type UiNode,
	type UiTreeItemNode,
	type UiTreeNode,
} from "@semio-tech/framework-playground-core";
import {
	DocumentVcsStore,
	applyJsonReplaceOp,
	createDocumentVcsEnvelope,
	recordJsonProjectionChange,
	type JsonReplaceOp,
} from "@semio-tech/framework-core";
import { bootstrapElementsSurfaceChromeDocument, type TreeDataItem, type TreeDragAndDropController, type TreeDropPosition } from "@semio-tech/ui-react";
import {
	applyFormEditOp,
	createFormId,
	defaultFormSpec,
	defaultQuestionForKind,
	findQuestionLocation,
	formSpecToJson,
	formsExtensionHost,
	isExtensionFormQuestion,
	questionKindContribution,
	type FormQuestion,
	type FormSelectOption,
	type FormSpec,
	type FormValues,
	type FormVectorField,
} from "@semio-tech/forms-core";
import { FORMS_QUESTION_DRAG_MIME, abortFormsQuestionPaletteDrag, formSpecFromJson, formsQuestionPaletteTreeDragController } from "@semio-tech/forms-react";
import { FORMS_PLAY_FIXTURE_DEFAULT_ID, resolveFormsPlayFixtureSlug } from "./fixture-slugs.ts";

export const FORMS_PLAY_APP_ID = "forms-play";
export const FORMS_PLAY_CONTROLLER_ID = "forms-play";
export const FORMS_PLAY_SURFACE_ID_EDIT = "forms.play.edit/v1";
export const FORMS_PLAY_SURFACE_ID_TRY = "forms.play.try/v1";
export const FORMS_PLAY_BODY_KEY_EDIT = "forms.play.edit";
export const FORMS_PLAY_BODY_KEY_TRY = "forms.play.try";
export const FORMS_PLAY_WINDOW_KIND_EDIT = "forms-edit";
export const FORMS_PLAY_WINDOW_KIND_TRY = "forms-try";
export const FORMS_PLAY_HIERARCHY_TAB_ID = "framework.panel.hierarchy";
export const FORMS_PLAY_CATALOGUE_TAB_ID = "framework.panel.catalogue";
export const FORMS_PLAY_INSPECTION_TAB_ID = "framework.panel.inspection";

export const FORMS_PLAY_LAYOUT = createDefaultLayout(
	[FORMS_PLAY_WINDOW_KIND_EDIT, FORMS_PLAY_WINDOW_KIND_TRY],
	"row",
	[58, 42],
	["Edit", "Try"],
);

export { FORMS_PLAY_FIXTURE_DEFAULT_ID, resolveFormsPlayFixtureSlug };

const formsFixtureModules = import.meta.glob("../fixture/*.forms.json", { eager: true }) as Record<string, { default: unknown }>;

function formsFixtureIdFromGlobPath(globPath: string): string {
	const base = globPath.split("/").pop() ?? globPath;
	return base.replace(/\.forms\.json$/, "");
}

function formsFixtureLabelFromId(id: string): string {
	return id
		.split("-")
		.filter(Boolean)
		.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
		.join(" ");
}

const FORMS_PLAY_FILE_FIXTURE_JSON_BY_ID: Record<string, string> = Object.fromEntries(
	Object.entries(formsFixtureModules).map(([path, mod]) => {
		const id = formsFixtureIdFromGlobPath(path);
		const json = typeof mod.default === "string" ? mod.default : JSON.stringify(mod.default);
		return [id, json];
	}),
);

export const FORMS_PLAY_FIXTURE_OPTIONS: ReadonlyArray<{ readonly id: string; readonly label: string }> = [
	...Object.keys(FORMS_PLAY_FILE_FIXTURE_JSON_BY_ID)
		.sort()
		.map((id) => ({ id: id === "building-component" ? FORMS_PLAY_FIXTURE_DEFAULT_ID : id, label: formsFixtureLabelFromId(id) })),
];

function formsPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
	return { controllerId: FORMS_PLAY_CONTROLLER_ID, command, args };
}

function formsPlayStepTreeId(stepId: string): string {
	return `step:${stepId}`;
}

function resolveStepIdFromTreeTarget(spec: FormSpec, targetId: string): string | null {
	if (targetId.startsWith("step:")) return targetId.slice(5);
	const location = findQuestionLocation(spec, targetId);
	return location?.stepId ?? spec.steps[0]?.id ?? null;
}

function resolveQuestionInsertIndex(spec: FormSpec, stepId: string, targetId: string, position: TreeDropPosition | undefined): number | undefined {
	const step = spec.steps.find((entry) => entry.id === stepId);
	if (!step) return undefined;
	if (targetId.startsWith("step:")) {
		return position === "before" ? 0 : step.questions.length;
	}
	const targetIndex = step.questions.findIndex((question) => question.id === targetId);
	if (targetIndex < 0) return step.questions.length;
	if (position === "before") return targetIndex;
	if (position === "after") return targetIndex + 1;
	return step.questions.length;
}

/** @emoji 📍 Resolves a hierarchy tree row under client coordinates for palette drops. */
export function resolveFormsPlayDropTargetFromPoint(clientX: number, clientY: number): { targetId: string; dropPosition: TreeDropPosition } | null {
	if (typeof document === "undefined") return null;
	const element = document.elementFromPoint(clientX, clientY);
	if (!element) return null;
	const row = element.closest('[data-slot="tree-item-row"]') as HTMLElement | null;
	if (!row?.id || row.id.startsWith("forms-play-catalogue.")) return null;
	const rect = row.getBoundingClientRect();
	const y = clientY - rect.top;
	let dropPosition: TreeDropPosition = "inside";
	if (y < rect.height * 0.25) dropPosition = "before";
	else if (y > rect.height * 0.75) dropPosition = "after";
	return { targetId: row.id, dropPosition };
}

/** @emoji 📍 Default drop target when releasing over the builder preview. */
export function resolveFormsPlayDefaultDropTarget(spec: FormSpec, selectedIds: readonly string[]): { targetId: string; dropPosition: TreeDropPosition } {
	const selectedId = selectedIds[0];
	if (selectedId) {
		const location = findQuestionLocation(spec, selectedId);
		if (location) return { targetId: location.question.id, dropPosition: "after" };
		if (selectedId.startsWith("step:")) return { targetId: selectedId, dropPosition: "inside" };
	}
	const stepId = spec.steps[0]?.id;
	return { targetId: formsPlayStepTreeId(stepId ?? "step-1"), dropPosition: "inside" };
}

/** @emoji 📥 Commits a catalogue question drop at pointer coordinates. */
export function commitFormsPlayQuestionDropAtClient(
	ctrl: FormsPlayController | undefined,
	clientX: number,
	clientY: number,
	kind: string,
): boolean {
	if (!ctrl) return false;
	const treeTarget = resolveFormsPlayDropTargetFromPoint(clientX, clientY);
	const target = treeTarget ?? resolveFormsPlayDefaultDropTarget(ctrl.getSpec(), ctrl.getSelectedIds());
	ctrl.run("dropQuestionKind", { kind, targetId: target.targetId, dropPosition: target.dropPosition });
	return true;
}

function formsPlayParseCatalogueDrop(data: Record<string, string>): string | null {
	const cataloguePayload = data[FORMS_QUESTION_DRAG_MIME];
	if (!cataloguePayload) return null;
	try {
		const payload = JSON.parse(cataloguePayload) as { kind?: string };
		return payload.kind ?? null;
	} catch {
		return null;
	}
}

function formsPlayCommitCatalogueDrop(
	getController: () => FormsPlayController | undefined,
	targetId: string,
	dropPosition: TreeDropPosition | undefined,
	kind: string,
): void {
	getController()?.run("dropQuestionKind", { kind, targetId, dropPosition });
}

function formsPlayIsHierarchyMetaRow(id: string): boolean {
	return id.startsWith("forms-play-catalogue.") || id.startsWith("forms-play-hierarchy.");
}

function resolveStepInsertIndex(spec: FormSpec, targetId: string, position: TreeDropPosition | undefined): number | undefined {
	if (targetId === "forms-play-hierarchy.steps" || targetId === "forms-play-hierarchy.empty") {
		return spec.steps.length;
	}
	if (!targetId.startsWith("step:")) return undefined;
	const stepId = targetId.slice(5);
	const targetIndex = spec.steps.findIndex((step) => step.id === stepId);
	if (targetIndex < 0) return undefined;
	if (position === "before") return targetIndex;
	if (position === "after" || position === "inside") return targetIndex + 1;
	return targetIndex;
}

/** @emoji 🖱️ Side-panel hierarchy drag: reorder steps and questions; accept catalogue drops only. */
export function createFormsPlayHierarchyTreeDragController(getController: () => FormsPlayController | undefined): TreeDragAndDropController {
	return {
		onDragStart: () => {
			abortFormsQuestionPaletteDrag();
		},
		handleDrop: ({ target, targetKind, data, sourceItems, dropPosition }) => {
			const catalogueKind = formsPlayParseCatalogueDrop(data);
			if (catalogueKind) {
				if (targetKind === "item") {
					formsPlayCommitCatalogueDrop(getController, (target as TreeDataItem).id, dropPosition, catalogueKind);
				} else if (targetKind === "section") {
					const spec = getController()?.getSpec();
					const stepId = spec?.steps[0]?.id;
					if (stepId) {
						formsPlayCommitCatalogueDrop(getController, formsPlayStepTreeId(stepId), "inside", catalogueKind);
					}
				}
				return;
			}
			const sourceItem = sourceItems[0];
			if (!sourceItem || formsPlayIsHierarchyMetaRow(sourceItem.id)) return;
			const spec = getController()?.getSpec();
			if (!spec) return;
			if (sourceItem.id.startsWith("step:")) {
				if (targetKind === "section") {
					getController()?.run("moveStep", { stepId: sourceItem.id.slice(5), index: spec.steps.length });
					return;
				}
				if (targetKind !== "item") return;
				const targetItem = target as TreeDataItem;
				if (formsPlayIsHierarchyMetaRow(targetItem.id)) return;
				const index = resolveStepInsertIndex(spec, targetItem.id, dropPosition);
				if (index === undefined) return;
				getController()?.run("moveStep", { stepId: sourceItem.id.slice(5), index });
				return;
			}
			if (targetKind !== "item") return;
			const targetItem = target as TreeDataItem;
			if (formsPlayIsHierarchyMetaRow(targetItem.id)) return;
			const toStepId = resolveStepIdFromTreeTarget(spec, targetItem.id);
			if (!toStepId) return;
			getController()?.run("moveQuestion", {
				questionId: sourceItem.id,
				toStepId,
				targetId: targetItem.id,
				position: dropPosition ?? "inside",
			});
		},
	};
}

/** @emoji 🌳 Workbench hierarchy: steps and questions from the live form spec. */
export function buildFormsPlayHierarchyTree(spec: FormSpec, selectedIds: readonly string[]): UiTreeNode {
	const stepItems: UiTreeItemNode[] = spec.steps.map((step) => ({
		id: formsPlayStepTreeId(step.id),
		label: step.title,
		description: `${step.questions.length} questions`,
		defaultOpen: true,
		draggable: true,
		command: formsPlayCmd("setSelection", { ids: [] }),
		items: step.questions.map((question) => ({
			id: question.id,
			label: question.label,
			description: question.kind,
			draggable: true,
			command: formsPlayCmd("setSelection", { ids: [question.id] }),
		})),
	}));
	return {
		type: "tree",
		sections: [
			{
				id: "forms-play-hierarchy.steps",
				label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
				defaultOpen: true,
				items: stepItems.length ? stepItems : [{ id: "forms-play-hierarchy.empty", label: "(no steps)" }],
			},
		],
		selectedIds: [...selectedIds],
		selectionChange: formsPlayCmd("setSelection"),
	};
}

/** @emoji 📚 Workbench catalogue: draggable question kinds and quick actions. */
export function buildFormsPlayCatalogueTree(): UiTreeNode {
	const kindItems: UiTreeItemNode[] = formsExtensionHost.catalogueEntries().map((entry) => ({
		id: `forms-play-catalogue.${entry.kind}`,
		label: entry.label,
		description: entry.kind,
		icon: entry.iconId,
		draggable: true,
		dragData: { [FORMS_QUESTION_DRAG_MIME]: JSON.stringify({ kind: entry.kind }) },
	}));
	return {
		type: "tree",
		sections: [
			{
				id: "forms-play-catalogue.kinds",
				label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
				defaultOpen: true,
				items: kindItems,
			},
			{
				id: "forms-play-catalogue.actions",
				label: "Actions",
				defaultOpen: true,
				items: [
					{ id: "forms-play-catalogue.add-step", label: "Add Step", command: formsPlayCmd("addStep") },
					{ id: "forms-play-catalogue.add-question", label: "Add Text Question", command: formsPlayCmd("addQuestion", { kind: "text" }) },
				],
			},
		],
	};
}

function formsPlayInspectorInput(
	id: string,
	label: string,
	questionId: string,
	field: string,
	value: string,
	inputKind: "text" | "number" = "text",
): UiNode {
	return {
		type: "field",
		id,
		label,
		child: {
			type: "input",
			id: `${id}.input`,
			inputKind,
			value,
			commit: "blur",
			onChange: formsPlayCmd("patchQuestion", { questionId, field }),
		},
	};
}

function formsPlayInspectorToggle(id: string, label: string, questionId: string, field: string, pressed: boolean): UiNode {
	return {
		type: "field",
		id,
		label,
		child: {
			type: "toggle",
			id: `${id}.toggle`,
			iconId: "check",
			pressed,
			text: pressed ? "Yes" : "No",
			onChange: formsPlayCmd("patchQuestion", { questionId, field }),
		},
	};
}

function formsPlayInspectorOptionFields(question: FormQuestion & { readonly options: readonly FormSelectOption[] }): UiNode[] {
	const fields: UiNode[] = [
		{
			type: "button",
			id: `forms-play-inspector.${question.id}.add-option`,
			iconId: "plus",
			label: "Add Option",
			command: formsPlayCmd("addQuestionOption", { questionId: question.id }),
		},
	];
	for (const [index, option] of question.options.entries()) {
		fields.push(
			formsPlayInspectorInput(`forms-play-inspector.${question.id}.option.${index}.value`, `Option ${index + 1} Value`, question.id, `option:${index}:value`, option.value),
			formsPlayInspectorInput(`forms-play-inspector.${question.id}.option.${index}.label`, `Option ${index + 1} Label`, question.id, `option:${index}:label`, option.label),
			{
				type: "button",
				id: `forms-play-inspector.${question.id}.option.${index}.remove`,
				iconId: "trash-2",
				label: `Remove Option ${index + 1}`,
				command: formsPlayCmd("removeQuestionOption", { questionId: question.id, index }),
			},
		);
	}
	return fields;
}

function formsPlayInspectorVectorFields(question: FormQuestion & { readonly fields: readonly FormVectorField[] }): UiNode[] {
	const fields: UiNode[] = [
		{
			type: "button",
			id: `forms-play-inspector.${question.id}.add-field`,
			iconId: "plus",
			label: "Add Field",
			command: formsPlayCmd("addVectorField", { questionId: question.id }),
		},
	];
	for (const [index, field] of question.fields.entries()) {
		fields.push(
			formsPlayInspectorInput(`forms-play-inspector.${question.id}.field.${index}.key`, `Field ${index + 1} Key`, question.id, `vectorField:${index}:key`, field.key),
			formsPlayInspectorInput(
				`forms-play-inspector.${question.id}.field.${index}.label`,
				`Field ${index + 1} Label`,
				question.id,
				`vectorField:${index}:label`,
				field.label ?? "",
			),
			formsPlayInspectorInput(
				`forms-play-inspector.${question.id}.field.${index}.value`,
				`Field ${index + 1} Default`,
				question.id,
				`vectorField:${index}:value`,
				String(field.value ?? 0),
				"number",
			),
			{
				type: "button",
				id: `forms-play-inspector.${question.id}.field.${index}.remove`,
				iconId: "trash-2",
				label: `Remove Field ${index + 1}`,
				command: formsPlayCmd("removeVectorField", { questionId: question.id, index }),
			},
		);
	}
	return fields;
}

function formsPlayInspectorFields(question: FormQuestion): UiNode[] {
	const fields: UiNode[] = [
		formsPlayInspectorInput("forms-play-inspector.label", "Label", question.id, "label", question.label),
		{
			type: "field",
			id: "forms-play-inspector.kind",
			label: "Kind",
			child: { type: "text", value: question.kind },
		},
		formsPlayInspectorInput("forms-play-inspector.description", "Description", question.id, "description", question.description ?? ""),
		formsPlayInspectorToggle("forms-play-inspector.required", "Required", question.id, "required", Boolean(question.required)),
	];
	if (question.kind === "text" || question.kind === "longText") {
		fields.push(
			formsPlayInspectorInput("forms-play-inspector.placeholder", "Placeholder", question.id, "placeholder", question.placeholder ?? ""),
			formsPlayInspectorInput("forms-play-inspector.default", "Default", question.id, "default", String(question.default ?? "")),
		);
	}
	if (question.kind === "number" || question.kind === "slider") {
		fields.push(
			formsPlayInspectorInput("forms-play-inspector.min", "Min", question.id, "min", String(question.min ?? 0), "number"),
			formsPlayInspectorInput("forms-play-inspector.max", "Max", question.id, "max", String(question.max ?? 100), "number"),
			formsPlayInspectorInput("forms-play-inspector.step", "Step", question.id, "step", String(question.step ?? 1), "number"),
			formsPlayInspectorInput("forms-play-inspector.default", "Default", question.id, "default", String(question.default ?? 0), "number"),
		);
	}
	if (question.kind === "slider") {
		fields.push(formsPlayInspectorInput("forms-play-inspector.unit", "Unit", question.id, "unit", question.unit ?? ""));
	}
	if (question.kind === "boolean") {
		fields.push(formsPlayInspectorToggle("forms-play-inspector.default", "Default", question.id, "default", Boolean(question.default)));
	}
	if (question.kind === "single" || question.kind === "multi") {
		fields.push(...formsPlayInspectorOptionFields(question));
		fields.push(
			formsPlayInspectorInput(
				"forms-play-inspector.default",
				"Default",
				question.id,
				"default",
				question.kind === "multi" ? (question.default ?? []).join(",") : String(question.default ?? ""),
			),
		);
	}
	if (question.kind === "date" || question.kind === "color") {
		fields.push(formsPlayInspectorInput("forms-play-inspector.default", "Default", question.id, "default", String(question.default ?? "")));
	}
	if (question.kind === "vector") {
		fields.push(
			formsPlayInspectorInput("forms-play-inspector.schema", "Schema", question.id, "schema", question.schema ?? ""),
			formsPlayInspectorInput("forms-play-inspector.step", "Step", question.id, "step", String(question.step ?? 0.1), "number"),
			...formsPlayInspectorVectorFields(question),
		);
	}
	if (question.kind === "note") {
		fields.push(formsPlayInspectorInput("forms-play-inspector.text", "Text", question.id, "text", question.text));
	}
	if (question.kind === "image") {
		fields.push(formsPlayInspectorInput("forms-play-inspector.src", "Source URL", question.id, "src", question.src ?? ""));
	}
	if (question.kind === "file") {
		fields.push(formsPlayInspectorInput("forms-play-inspector.accept", "Accept", question.id, "accept", question.accept ?? ""));
	}
	if (isExtensionFormQuestion(question)) {
		const contribution = questionKindContribution(question);
		const fixtureSlug = question.fixtureSlug ?? contribution?.controls?.fixtureSlug ?? contribution?.edit?.fixtureSlug ?? contribution?.preview?.fixtureSlug;
		if (fixtureSlug) {
			fields.push({
				type: "field",
				id: "forms-play-inspector.fixtureSlug",
				label: "Flow Fixture",
				child: { type: "text", value: fixtureSlug },
			});
		}
		if (contribution?.edit?.surface === "flow3d" || contribution?.preview?.surface === "flow3d") {
			fields.push({
				type: "field",
				id: "forms-play-inspector.editSurface",
				label: "Edit Surface",
				child: { type: "text", value: "flow3d (params + preview in Edit window)" },
			});
		}
	}
	return fields;
}

/** @emoji 🔍 Details inspection: editable properties for the selected question. */
export function buildFormsPlayInspectorTree(spec: FormSpec, selectedIds: readonly string[]): UiNode {
	const questionId = selectedIds[0];
	if (!questionId) {
		return uiDeclarativeSectionsToTree([
			{
				type: "section",
				id: "forms-play-inspector.empty",
				label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
				children: [{ type: "text", value: "Select a question in the hierarchy." }],
			},
		]);
	}
	const location = findQuestionLocation(spec, questionId);
	if (!location) {
		return uiDeclarativeSectionsToTree([
			{
				type: "section",
				id: "forms-play-inspector.missing",
				label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
				children: [{ type: "text", value: "Question not found." }],
			},
		]);
	}
	return uiDeclarativeSectionsToTree([
		{
			type: "section",
			id: "forms-play-inspector.question",
			label: location.question.label,
			children: formsPlayInspectorFields(location.question),
		},
	]);
}

export function buildFormsPlayToolbarTools(controllerId: string): AppTools {
	const button = (id: string, label: string, command: string, args?: Record<string, unknown>): ToolLeaf => ({
		kind: "button",
		id,
		label,
		controllerId,
		command,
		args,
	});
	return [
		toolCollection("actions", "ui.toolbar.parent.actions", [
			button("forms.add-step", "Add Step", "addStep"),
			button("forms.add-question", "Add Question", "addQuestion", { kind: "text" }),
			button("forms.export", "Export JSON", "exportFixture"),
		]),
	];
}

/** @emoji 🎛 Forms play shell controller. */
export class FormsPlayController extends Controller implements PlaygroundFixtureHost {
	readonly mainMode = new ModeRuntime("main", "Forms", undefined);
	private readonly docStore = new DocumentVcsStore<FormSpec, JsonReplaceOp<FormSpec>>({
		envelope: createDocumentVcsEnvelope("forms.form/v1", "forms-play", defaultFormSpec()),
		applyOp: applyJsonReplaceOp,
	});
	private selectedIds: string[] = [];
	private tryValues: FormValues = {};
	private interactionRevision = 0;
	private extensionRevision = 0;
	private readonly snapshotListeners = new Set<() => void>();

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(FORMS_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		const json = FORMS_PLAY_FILE_FIXTURE_JSON_BY_ID["building-component"];
		if (json) this.replaceDocument(formSpecFromJson(json));
		formsExtensionHost.subscribe(() => {
			this.extensionRevision += 1;
			this.notifySnapshot();
			this.emit();
		});
		void formsExtensionHost.activateDefaults();
		this.rebuildShellMode();
	}

	private projection(): FormSpec {
		return this.docStore.projection();
	}

	private commitDocument(next: FormSpec): void {
		recordJsonProjectionChange(this.docStore, next);
		this.tryValues = {};
		this.notifySnapshot();
		this.emit();
	}

	replaceDocument(spec: FormSpec): void {
		this.commitDocument(spec);
	}

	getSpec(): FormSpec {
		return this.projection();
	}

	getSpecJson(): string {
		return formSpecToJson(this.projection());
	}

	getDocumentVcsStore(): DocumentVcsStore<FormSpec, JsonReplaceOp<FormSpec>> {
		return this.docStore;
	}

	getSelectedIds(): readonly string[] {
		return this.selectedIds;
	}

	getTryValues(): FormValues {
		return this.tryValues;
	}

	getInteractionRevision(): number {
		return this.interactionRevision;
	}

	getExtensionRevision(): number {
		return this.extensionRevision;
	}

	subscribeSnapshot(listener: () => void): () => void {
		this.snapshotListeners.add(listener);
		return () => this.snapshotListeners.delete(listener);
	}

	private notifySnapshot(): void {
		this.interactionRevision += 1;
		for (const listener of this.snapshotListeners) {
			listener();
		}
	}

	private setSpec(spec: FormSpec): void {
		this.commitDocument(spec);
	}

	private rebuildShellMode(): void {
		this.mainMode.tools = buildFormsPlayToolbarTools(this.id);
		this.mainMode.windowKinds = [
			new WindowKindRuntime(FORMS_PLAY_WINDOW_KIND_EDIT, "Edit", FORMS_PLAY_BODY_KEY_EDIT),
			new WindowKindRuntime(FORMS_PLAY_WINDOW_KIND_TRY, "Try", FORMS_PLAY_BODY_KEY_TRY),
		];
	}

	private patchQuestionField(questionId: string, field: string, rawValue: unknown): void {
		const location = findQuestionLocation(this.projection(), questionId);
		if (!location) return;
		const question = location.question;
		if (field.startsWith("option:")) {
			const [, indexRaw, part] = field.split(":");
			const index = Number(indexRaw);
			if (!Number.isFinite(index) || (question.kind !== "single" && question.kind !== "multi")) return;
			const options = [...question.options];
			const entry = options[index];
			if (!entry) return;
			options[index] = { ...entry, [part!]: String(rawValue ?? "") };
			this.setSpec({
				...this.projection(),
				steps: this.projection().steps.map((step) =>
					step.id === location.stepId
						? { ...step, questions: step.questions.map((entry) => (entry.id === questionId ? ({ ...question, options } as FormQuestion) : entry)) }
						: step,
				),
			});
			return;
		}
		if (field.startsWith("vectorField:")) {
			const [, indexRaw, part] = field.split(":");
			const index = Number(indexRaw);
			if (!Number.isFinite(index) || question.kind !== "vector") return;
			const vectorFields = [...question.fields];
			const entry = vectorFields[index];
			if (!entry) return;
			const nextValue = part === "value" ? Number(rawValue) : String(rawValue ?? "");
			vectorFields[index] = { ...entry, [part!]: nextValue };
			this.setSpec({
				...this.projection(),
				steps: this.projection().steps.map((step) =>
					step.id === location.stepId
						? { ...step, questions: step.questions.map((entry) => (entry.id === questionId ? ({ ...question, fields: vectorFields } as FormQuestion) : entry)) }
						: step,
				),
			});
			return;
		}
		let value: unknown = rawValue;
		if (field === "options" || field === "fields") {
			const nextQuestion = { ...question, [field]: rawValue } as FormQuestion;
			this.setSpec({
				...this.projection(),
				steps: this.projection().steps.map((step) =>
					step.id === location.stepId ? { ...step, questions: step.questions.map((entry) => (entry.id === questionId ? nextQuestion : entry)) } : step,
				),
			});
			return;
		}
		if (field === "required") value = Boolean(rawValue);
		else if (field === "default" && question.kind === "boolean") value = Boolean(rawValue);
		else if (field === "default" && question.kind === "multi" && Array.isArray(rawValue)) value = rawValue;
		else if (field === "default" && question.kind === "multi") value = String(rawValue ?? "").split(",").map((entry) => entry.trim()).filter(Boolean);
		else if (field === "default" && question.kind === "vector" && Array.isArray(rawValue)) value = rawValue;
		else if (field === "min" || field === "max" || field === "step" || (field === "default" && (question.kind === "number" || question.kind === "slider"))) value = Number(rawValue);
		const nextQuestion = { ...question, [field]: value } as FormQuestion;
		this.setSpec({
			...this.projection(),
			steps: this.projection().steps.map((step) =>
				step.id === location.stepId ? { ...step, questions: step.questions.map((entry) => (entry.id === questionId ? nextQuestion : entry)) } : step,
			),
		});
	}

	getFixtureCatalog(): PlaygroundFixtureCatalog {
		const activeFixtureId = playgroundResolvedFixtureId(this.projection().id, FORMS_PLAY_FIXTURE_DEFAULT_ID);
		return {
			activeFixtureId,
			options: FORMS_PLAY_FIXTURE_OPTIONS,
			locked: isPlaygroundFixtureLocked(),
		};
	}

	override run(command: string, args?: unknown): void {
		if (command === "setActiveFixture") {
			const fixtureId = (args as { fixtureId?: string }).fixtureId;
			if (typeof fixtureId !== "string") return;
			if (isPlaygroundNoFixtureId(fixtureId)) {
				this.setSpec(defaultFormSpec());
				return;
			}
			const resolvedId = resolveFormsPlayFixtureSlug(fixtureId) ?? fixtureId;
			const json = FORMS_PLAY_FILE_FIXTURE_JSON_BY_ID[resolvedId];
			if (json) this.setSpec(formSpecFromJson(json));
			return;
		}
		if (command === "setSelection") {
			const ids = (args as { ids?: string[] }).ids;
			if (Array.isArray(ids)) {
				this.selectedIds = ids;
				this.notifySnapshot();
				this.emit();
			}
			return;
		}
		if (command === "setSpecJson") {
			const json = (args as { json?: string }).json;
			if (typeof json === "string") {
				this.setSpec(formSpecFromJson(json));
			}
			return;
		}
		if (command === "addStep") {
			const stepId = createFormId("step");
			this.setSpec(
				applyFormEditOp(this.projection(), {
					op: "addStep",
					step: { id: stepId, title: `Step ${this.projection().steps.length + 1}`, questions: [] },
				}),
			);
			return;
		}
		if (command === "addQuestion") {
			const kind = (args as { kind?: string }).kind ?? "text";
			const stepId = (args as { stepId?: string }).stepId ?? this.projection().steps[0]?.id;
			if (!stepId) return;
			this.setSpec(
				applyFormEditOp(this.projection(), {
					op: "addQuestion",
					stepId,
					question: defaultQuestionForKind(kind, createFormId("q")),
				}),
			);
			return;
		}
		if (command === "dropQuestionKind") {
			const kind = (args as { kind?: string }).kind;
			const targetId = (args as { targetId?: string }).targetId;
			const dropPosition = (args as { dropPosition?: TreeDropPosition }).dropPosition;
			if (!kind || !targetId) return;
			const stepId = resolveStepIdFromTreeTarget(this.projection(), targetId);
			if (!stepId) return;
			const index = resolveQuestionInsertIndex(this.projection(), stepId, targetId, dropPosition);
			this.setSpec(
				applyFormEditOp(this.projection(), {
					op: "addQuestion",
					stepId,
					index,
					question: defaultQuestionForKind(kind, createFormId("q")),
				}),
			);
			return;
		}
		if (command === "moveQuestion") {
			const questionId = (args as { questionId?: string }).questionId;
			const toStepId = (args as { toStepId?: string }).toStepId;
			const targetId = (args as { targetId?: string }).targetId ?? questionId;
			const position = (args as { position?: TreeDropPosition }).position ?? "inside";
			if (!questionId || !toStepId) return;
			const source = findQuestionLocation(this.projection(), questionId);
			if (!source) return;
			const index = resolveQuestionInsertIndex(this.projection(), toStepId, targetId, position);
			this.setSpec(
				applyFormEditOp(this.projection(), {
					op: "moveQuestion",
					questionId,
					fromStepId: source.stepId,
					toStepId,
					index: index ?? 0,
				}),
			);
			return;
		}
		if (command === "moveStep") {
			const stepId = (args as { stepId?: string }).stepId;
			const index = (args as { index?: number }).index;
			if (!stepId || typeof index !== "number") return;
			this.setSpec(applyFormEditOp(this.projection(), { op: "moveStep", stepId, index }));
			return;
		}
		if (command === "patchQuestion") {
			const questionId = (args as { questionId?: string }).questionId;
			const field = (args as { field?: string }).field;
			const patch = args as { value?: unknown; pressed?: boolean };
			const rawValue = patch.value ?? patch.pressed;
			if (!questionId || !field) return;
			this.patchQuestionField(questionId, field, rawValue);
			return;
		}
		if (command === "addQuestionOption") {
			const questionId = (args as { questionId?: string }).questionId;
			if (!questionId) return;
			const location = findQuestionLocation(this.projection(), questionId);
			if (!location || (location.question.kind !== "single" && location.question.kind !== "multi")) return;
			const options = [...location.question.options, { value: createFormId("opt"), label: "New Option" }];
			this.patchQuestionField(questionId, "options", options);
			return;
		}
		if (command === "removeQuestionOption") {
			const questionId = (args as { questionId?: string }).questionId;
			const index = (args as { index?: number }).index;
			if (!questionId || typeof index !== "number") return;
			const location = findQuestionLocation(this.projection(), questionId);
			if (!location || (location.question.kind !== "single" && location.question.kind !== "multi")) return;
			const options = location.question.options.filter((_, entryIndex) => entryIndex !== index);
			if (options.length === 0) return;
			this.patchQuestionField(questionId, "options", options);
			return;
		}
		if (command === "addVectorField") {
			const questionId = (args as { questionId?: string }).questionId;
			if (!questionId) return;
			const location = findQuestionLocation(this.projection(), questionId);
			if (!location || location.question.kind !== "vector") return;
			const fields = [...location.question.fields, { key: createFormId("axis"), label: "Axis", value: 0 }];
			this.patchQuestionField(questionId, "fields", fields);
			return;
		}
		if (command === "removeVectorField") {
			const questionId = (args as { questionId?: string }).questionId;
			const index = (args as { index?: number }).index;
			if (!questionId || typeof index !== "number") return;
			const location = findQuestionLocation(this.projection(), questionId);
			if (!location || location.question.kind !== "vector") return;
			const fields = location.question.fields.filter((_, entryIndex) => entryIndex !== index);
			if (fields.length === 0) return;
			this.patchQuestionField(questionId, "fields", fields);
			return;
		}
		if (command === "setTryValues") {
			const values = (args as { values?: FormValues }).values;
			if (values && typeof values === "object") {
				this.tryValues = values;
				this.notifySnapshot();
				this.emit();
			}
			return;
		}
		if (command === "resetTry") {
			this.tryValues = {};
			this.notifySnapshot();
			this.emit();
			return;
		}
		if (command === "exportFixture") {
			console.log("[DEBUG] forms export", this.getSpecJson());
			return;
		}
	}
}

function buildFormsPlayEditBody(_ctx: unknown): UiNode {
	return buildFormsWindowBody(FORMS_PLAY_SURFACE_ID_EDIT, FORMS_PLAY_CONTROLLER_ID, "builder");
}

function buildFormsPlayTryBody(_ctx: unknown): UiNode {
	return buildFormsWindowBody(FORMS_PLAY_SURFACE_ID_TRY, FORMS_PLAY_CONTROLLER_ID, "preview");
}

export function registerFormsPlayDeclarativeBodies(): void {
	registerWindowBody(FORMS_PLAY_BODY_KEY_EDIT, buildFormsPlayEditBody);
	registerWindowBody(FORMS_PLAY_BODY_KEY_TRY, buildFormsPlayTryBody);
}

export function buildFormsPlayAppRuntime(controller: FormsPlayController): AppRuntime {
	return createPlayAppRuntime(FORMS_PLAY_APP_ID, "Forms", controller, FORMS_PLAY_LAYOUT, controller.mainMode);
}

export class PlaygroundForms extends Playground {
	readonly id = FORMS_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new FormsPlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildFormsPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerFormsPlayDeclarativeBodies();
	}
}

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("@semio-tech/forms-play", () => {
		it("exports builder and preview layout", () => {
			expect(FORMS_PLAY_LAYOUT.root.kind).toBe("row");
		});

		it("controller loads file fixtures", () => {
			const bus = new CommandBus();
			const ctrl = new FormsPlayController(bus, () => {});
			ctrl.run("setActiveFixture", { fixtureId: "building-component" });
			expect(ctrl.getSpec().id).toBe("building-component");
			expect(ctrl.getSpec().steps.flatMap((step) => step.questions).some((question) => question.kind === "buildingComponent")).toBe(true);
		});

		it("catalogue includes extension question kinds", () => {
			const catalogue = buildFormsPlayCatalogueTree();
			expect(catalogue.sections?.[0]?.items?.some((item) => item.description === "buildingComponent")).toBe(true);
		});

		it("adds steps and questions", () => {
			const bus = new CommandBus();
			const ctrl = new FormsPlayController(bus, () => {});
			const initial = ctrl.getSpec().steps.length;
			ctrl.run("addStep");
			expect(ctrl.getSpec().steps.length).toBe(initial + 1);
		});

		it("builds hierarchy and catalogue side-panel trees", () => {
			const spec = defaultFormSpec();
			const hierarchy = buildFormsPlayHierarchyTree(spec, []);
			const catalogue = buildFormsPlayCatalogueTree();
			expect(hierarchy.type).toBe("tree");
			expect(catalogue.type).toBe("tree");
			expect(hierarchy.sections?.[0]?.items?.[0]?.items?.length).toBeGreaterThan(0);
			expect(catalogue.sections?.[0]?.items?.some((item) => item.draggable)).toBe(true);
		});

		it("uses edit and try window kinds in a single mode", () => {
			const bus = new CommandBus();
			const ctrl = new FormsPlayController(bus, () => {});
			expect(ctrl.mainMode.windowKinds.map((kind) => kind.id)).toEqual([FORMS_PLAY_WINDOW_KIND_EDIT, FORMS_PLAY_WINDOW_KIND_TRY]);
			const app = buildFormsPlayAppRuntime(ctrl);
			expect(app.modes).toHaveLength(1);
		});

		it("adds and removes select options", () => {
			const bus = new CommandBus();
			const ctrl = new FormsPlayController(bus, () => {});
			ctrl.run("addQuestion", { kind: "single", stepId: ctrl.getSpec().steps[0]?.id });
			const question = ctrl.getSpec().steps[0]?.questions.at(-1);
			expect(question?.kind).toBe("single");
			if (!question || question.kind !== "single") return;
			const initial = question.options.length;
			ctrl.run("addQuestionOption", { questionId: question.id });
			const updated = findQuestionLocation(ctrl.getSpec(), question.id)?.question;
			expect(updated?.kind).toBe("single");
			if (updated?.kind === "single") expect(updated.options.length).toBe(initial + 1);
		});

		it("stores try values separately from the spec", () => {
			const bus = new CommandBus();
			const ctrl = new FormsPlayController(bus, () => {});
			ctrl.run("setTryValues", { values: { "q-text": "Ada" } });
			expect(ctrl.getTryValues()["q-text"]).toBe("Ada");
			ctrl.run("addStep");
			expect(ctrl.getTryValues()).toEqual({});
		});

		it("resolves default and point drop targets", () => {
			const spec = defaultFormSpec();
			const fallback = resolveFormsPlayDefaultDropTarget(spec, []);
			expect(fallback.targetId.startsWith("step:")).toBe(true);
			expect(resolveFormsPlayDropTargetFromPoint(0, 0)).toBeNull();
		});

		it("hierarchy drag reorders questions without catalogue payload", () => {
			const bus = new CommandBus();
			const ctrl = new FormsPlayController(bus, () => {});
			const drag = createFormsPlayHierarchyTreeDragController(() => ctrl);
			const stepId = ctrl.getSpec().steps[0]?.id;
			if (!stepId) return;
			ctrl.run("addQuestion", { kind: "text", stepId });
			ctrl.run("addQuestion", { kind: "boolean", stepId });
			const questions = ctrl.getSpec().steps[0]?.questions ?? [];
			const first = questions[0];
			const second = questions[1];
			if (!first || !second) return;
			const beforeIds = questions.map((question) => question.id);
			drag.handleDrop?.({
				target: { id: second.id, label: second.label },
				targetKind: "item",
				data: {},
				sourceItems: [{ id: first.id, label: first.label }],
				section: { id: "forms-play-hierarchy.steps", label: "Hierarchy", items: [] },
				dropPosition: "after",
			});
			const afterIds = ctrl.getSpec().steps[0]?.questions.map((question) => question.id) ?? [];
			expect(afterIds).not.toEqual(beforeIds);
			expect(afterIds.indexOf(first.id)).toBeGreaterThan(afterIds.indexOf(second.id));
		});

		it("hierarchy drag reorders steps without catalogue payload", () => {
			const bus = new CommandBus();
			const ctrl = new FormsPlayController(bus, () => {});
			ctrl.run("setActiveFixture", { fixtureId: "building-component" });
			const drag = createFormsPlayHierarchyTreeDragController(() => ctrl);
			const firstStep = ctrl.getSpec().steps[0];
			const secondStep = ctrl.getSpec().steps[1];
			if (!firstStep || !secondStep) return;
			drag.handleDrop?.({
				target: { id: formsPlayStepTreeId(secondStep.id), label: secondStep.title },
				targetKind: "item",
				data: {},
				sourceItems: [{ id: formsPlayStepTreeId(firstStep.id), label: firstStep.title }],
				section: { id: "forms-play-hierarchy.steps", label: "Hierarchy", items: [] },
				dropPosition: "after",
			});
			expect(ctrl.getSpec().steps.map((step) => step.id)).toEqual([secondStep.id, firstStep.id]);
		});

		it("hierarchy drag accepts catalogue payload as add question", () => {
			const bus = new CommandBus();
			const ctrl = new FormsPlayController(bus, () => {});
			const stepId = ctrl.getSpec().steps[0]?.id;
			if (!stepId) return;
			const initial = ctrl.getSpec().steps[0]?.questions.length ?? 0;
			const drag = createFormsPlayHierarchyTreeDragController(() => ctrl);
			drag.handleDrop?.({
				target: { id: formsPlayStepTreeId(stepId), label: "Step" },
				targetKind: "item",
				data: { [FORMS_QUESTION_DRAG_MIME]: JSON.stringify({ kind: "note" }) },
				sourceItems: [{ id: "forms-play-catalogue.note", label: "Note" }],
				section: { id: "forms-play-hierarchy.steps", label: "Hierarchy", items: [] },
				dropPosition: "inside",
			});
			expect(ctrl.getSpec().steps[0]?.questions.length).toBe(initial + 1);
			expect(ctrl.getSpec().steps[0]?.questions.at(-1)?.kind).toBe("note");
		});
	});
}
// #endregion 🧪Tests

// #region 🔖Boot
if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest && import.meta.env.PUZZLE_PLAY_ENTRY === "forms") {
	bootstrapElementsSurfaceChromeDocument("system");
	void (async () => {
		await import("./globals.css");
		const { bootFormsPlay } = await import("@semio-tech/framework-playground-renderer-react/forms");
		bootFormsPlay(new PlaygroundForms());
	})();
}
// #endregion 🔖Boot
