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
	UI_INSPECTOR_MIXED_PLACEHOLDER,
	uiInspectorGroupsToTree,
	uiInspectorMixedNumber,
	uiInspectorMixedText,
	uiInspectorMixedToggle,
	uiInspectorReadonlyField,
	type UiInspectorFieldGroup,
	type AppTools,
	type CommandDescriptor,
	type PlaygroundFixtureCatalog,
	type PlaygroundFixtureHost,
	type ToolLeaf,
	toolCollection,
	type UiNode,
	type UiTreeItemNode,
	type UiTreeNode,
	type WindowMeasure,
	type WindowEngagement,
	enforcePlaygroundWindowEngagementInput,
} from "@semio-tech/framework-playground-core";
import { DocumentVcsStore, createDocumentVcsEnvelope, recordProjectionChange } from "@semio-tech/vcs-core";
import { bootstrapElementsSurfaceChromeDocument, type TreeDataItem, type TreeDragAndDropController, type TreeDropPosition } from "@semio-tech/ui-react";
import {
	applyFormEditOp,
	backwardsFormEditOp,
	createFormId,
	defaultFormSpec,
	defaultQuestionForKind,
	diffFormEditOp,
	findQuestionLocation,
	formSpecToJson,
	formsExtensionHost,
	isExtensionFormQuestion,
	questionKindContribution,
	type FormEditOp,
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

function formsPlayInspectorPatch(questionIds: readonly string[], field: string) {
	return formsPlayCmd("patchQuestions", { questionIds, field });
}

function formsPlayInspectorInput(
	id: string,
	label: string,
	questionIds: readonly string[],
	field: string,
	values: readonly string[],
	inputKind: "text" | "number" = "text",
): UiNode {
	const mixed = inputKind === "number" ? uiInspectorMixedNumber(values.map(Number)) : uiInspectorMixedText(values);
	return {
		type: "field",
		id,
		label,
		child: {
			type: "input",
			id: `${id}.input`,
			inputKind,
			value: inputKind === "number" ? (mixed.uniform ? String(mixed.value) : "") : mixed.value,
			placeholder: inputKind === "number" ? (mixed.uniform ? undefined : UI_INSPECTOR_MIXED_PLACEHOLDER) : mixed.placeholder,
			commit: "blur",
			onChange: formsPlayInspectorPatch(questionIds, field),
		},
	};
}

function formsPlayInspectorToggle(id: string, label: string, questionIds: readonly string[], field: string, values: readonly boolean[]): UiNode {
	const mixed = uiInspectorMixedToggle(values);
	return {
		type: "field",
		id,
		label,
		child: {
			type: "toggle",
			id: `${id}.toggle`,
			iconId: "check",
			pressed: mixed.pressed,
			text: mixed.uniform ? (mixed.pressed ? "Yes" : "No") : UI_INSPECTOR_MIXED_PLACEHOLDER,
			onChange: formsPlayInspectorPatch(questionIds, field),
		},
	};
}

function formsPlayInspectorOptionFields(questions: readonly (FormQuestion & { readonly options: readonly FormSelectOption[] })[]): UiNode[] {
	const questionIds = questions.map((entry) => entry.id);
	const fields: UiNode[] = [
		{
			type: "button",
			id: `forms-play-inspector.${questionIds[0] ?? "batch"}.add-option`,
			iconId: "plus",
			label: "Add Option",
			command: formsPlayCmd("addQuestionOption", { questionId: questionIds[0] }),
		},
	];
	if (questionIds.length !== 1) {
		fields.unshift(uiInspectorReadonlyField("forms-play-inspector.options-hint", "Options", `${questionIds.length} questions — edit individually`));
		return fields;
	}
	const question = questions[0]!;
	for (const [index, option] of question.options.entries()) {
		fields.push(
			formsPlayInspectorInput(`forms-play-inspector.${question.id}.option.${index}.value`, `Option ${index + 1} Value`, questionIds, `option:${index}:value`, [option.value]),
			formsPlayInspectorInput(`forms-play-inspector.${question.id}.option.${index}.label`, `Option ${index + 1} Label`, questionIds, `option:${index}:label`, [option.label]),
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

function formsPlayInspectorVectorFields(questions: readonly (FormQuestion & { readonly fields: readonly FormVectorField[] })[]): UiNode[] {
	const questionIds = questions.map((entry) => entry.id);
	const fields: UiNode[] = [
		{
			type: "button",
			id: `forms-play-inspector.${questionIds[0] ?? "batch"}.add-field`,
			iconId: "plus",
			label: "Add Field",
			command: formsPlayCmd("addVectorField", { questionId: questionIds[0] }),
		},
	];
	if (questionIds.length !== 1) {
		fields.unshift(uiInspectorReadonlyField("forms-play-inspector.vector-hint", "Vector Fields", `${questionIds.length} questions — edit individually`));
		return fields;
	}
	const question = questions[0]!;
	for (const [index, field] of question.fields.entries()) {
		fields.push(
			formsPlayInspectorInput(`forms-play-inspector.${question.id}.field.${index}.key`, `Field ${index + 1} Key`, questionIds, `vectorField:${index}:key`, [field.key]),
			formsPlayInspectorInput(
				`forms-play-inspector.${question.id}.field.${index}.label`,
				`Field ${index + 1} Label`,
				questionIds,
				`vectorField:${index}:label`,
				[field.label ?? ""],
			),
			formsPlayInspectorInput(
				`forms-play-inspector.${question.id}.field.${index}.value`,
				`Field ${index + 1} Default`,
				questionIds,
				`vectorField:${index}:value`,
				[String(field.value ?? 0)],
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

function formsPlayInspectorKindGroup(kind: string, questions: readonly FormQuestion[]): UiInspectorFieldGroup | null {
	if (!questions.length) return null;
	const questionIds = questions.map((entry) => entry.id);
	const fields: UiNode[] = [];
	const question = questions[0]!;
	if (kind === "text" || kind === "longText") {
		fields.push(
			formsPlayInspectorInput("forms-play-inspector.placeholder", "Placeholder", questionIds, "placeholder", questions.map((entry) => (entry.kind === kind ? (entry.placeholder ?? "") : ""))),
			formsPlayInspectorInput("forms-play-inspector.default", "Default", questionIds, "default", questions.map((entry) => (entry.kind === kind ? String(entry.default ?? "") : ""))),
		);
	}
	if (kind === "number" || kind === "slider") {
		fields.push(
			formsPlayInspectorInput("forms-play-inspector.min", "Min", questionIds, "min", questions.map((entry) => (entry.kind === kind ? String(entry.min ?? 0) : "0")), "number"),
			formsPlayInspectorInput("forms-play-inspector.max", "Max", questionIds, "max", questions.map((entry) => (entry.kind === kind ? String(entry.max ?? 100) : "100")), "number"),
			formsPlayInspectorInput("forms-play-inspector.step", "Step", questionIds, "step", questions.map((entry) => (entry.kind === kind ? String(entry.step ?? 1) : "1")), "number"),
			formsPlayInspectorInput("forms-play-inspector.default", "Default", questionIds, "default", questions.map((entry) => (entry.kind === kind ? String(entry.default ?? 0) : "0")), "number"),
		);
	}
	if (kind === "slider") {
		fields.push(formsPlayInspectorInput("forms-play-inspector.unit", "Unit", questionIds, "unit", questions.map((entry) => (entry.kind === "slider" ? (entry.unit ?? "") : ""))));
	}
	if (kind === "boolean") {
		fields.push(formsPlayInspectorToggle("forms-play-inspector.default", "Default", questionIds, "default", questions.map((entry) => (entry.kind === "boolean" ? Boolean(entry.default) : false))));
	}
	if (kind === "single" || kind === "multi") {
		const optionQuestions = questions.filter((entry): entry is FormQuestion & { readonly options: readonly FormSelectOption[] } => entry.kind === kind);
		fields.push(...formsPlayInspectorOptionFields(optionQuestions));
		fields.push(
			formsPlayInspectorInput(
				"forms-play-inspector.default",
				"Default",
				questionIds,
				"default",
				questions.map((entry) =>
					entry.kind === "multi" ? (entry.default ?? []).join(",") : entry.kind === "single" ? String(entry.default ?? "") : "",
				),
			),
		);
	}
	if (kind === "date" || kind === "color") {
		fields.push(formsPlayInspectorInput("forms-play-inspector.default", "Default", questionIds, "default", questions.map((entry) => (entry.kind === kind ? String(entry.default ?? "") : ""))));
	}
	if (kind === "vector") {
		const vectorQuestions = questions.filter((entry): entry is FormQuestion & { readonly fields: readonly FormVectorField[] } => entry.kind === "vector");
		fields.push(
			formsPlayInspectorInput("forms-play-inspector.schema", "Schema", questionIds, "schema", questions.map((entry) => (entry.kind === "vector" ? (entry.schema ?? "") : ""))),
			formsPlayInspectorInput("forms-play-inspector.step", "Step", questionIds, "step", questions.map((entry) => (entry.kind === "vector" ? String(entry.step ?? 0.1) : "0.1")), "number"),
			...formsPlayInspectorVectorFields(vectorQuestions),
		);
	}
	if (kind === "note") {
		fields.push(formsPlayInspectorInput("forms-play-inspector.text", "Text", questionIds, "text", questions.map((entry) => (entry.kind === "note" ? entry.text : ""))));
	}
	if (kind === "image") {
		fields.push(formsPlayInspectorInput("forms-play-inspector.src", "Source URL", questionIds, "src", questions.map((entry) => (entry.kind === "image" ? (entry.src ?? "") : ""))));
	}
	if (kind === "file") {
		fields.push(formsPlayInspectorInput("forms-play-inspector.accept", "Accept", questionIds, "accept", questions.map((entry) => (entry.kind === "file" ? (entry.accept ?? "") : ""))));
	}
	if (isExtensionFormQuestion(question)) {
		const contribution = questionKindContribution(question);
		const fixtureSlug = question.fixtureSlug ?? contribution?.controls?.fixtureSlug ?? contribution?.edit?.fixtureSlug ?? contribution?.preview?.fixtureSlug;
		if (fixtureSlug) {
			fields.push(uiInspectorReadonlyField("forms-play-inspector.fixtureSlug", "Flow Fixture", fixtureSlug));
		}
		if (contribution?.edit?.surface === "flow3d" || contribution?.preview?.surface === "flow3d") {
			fields.push(uiInspectorReadonlyField("forms-play-inspector.editSurface", "Edit Surface", "flow3d (params + preview in Edit window)"));
		}
	}
	if (!fields.length) return null;
	const label = kind.charAt(0).toUpperCase() + kind.slice(1);
	return { id: `forms-play-inspector.kind.${kind}`, label, fields };
}

function formsPlayInspectorBaseGroup(questions: readonly FormQuestion[]): UiInspectorFieldGroup {
	const questionIds = questions.map((entry) => entry.id);
	const labels = questions.map((entry) => entry.label);
	const kinds = questions.map((entry) => entry.kind);
	const descriptions = questions.map((entry) => entry.description ?? "");
	const required = questions.map((entry) => Boolean(entry.required));
	const kindMixed = uiInspectorMixedText(kinds);
	return {
		id: "forms-play-inspector.base",
		label: "Question",
		fields: [
			formsPlayInspectorInput("forms-play-inspector.label", "Label", questionIds, "label", labels),
			uiInspectorReadonlyField(
				"forms-play-inspector.kind",
				"Kind",
				kindMixed.uniform ? (kinds[0] ?? "") : (kindMixed.placeholder ?? UI_INSPECTOR_MIXED_PLACEHOLDER),
			),
			uiInspectorReadonlyField("forms-play-inspector.id", "Id", questionIds.length === 1 ? (questionIds[0] ?? "") : `${questionIds.length} selected`),
			formsPlayInspectorInput("forms-play-inspector.description", "Description", questionIds, "description", descriptions),
			formsPlayInspectorToggle("forms-play-inspector.required", "Required", questionIds, "required", required),
		],
	};
}

/** @emoji 🔍 Details inspection: editable properties for the selected question. */
export function buildFormsPlayInspectorTree(spec: FormSpec, selectedIds: readonly string[]): UiNode {
	const questions = selectedIds
		.map((questionId) => findQuestionLocation(spec, questionId)?.question)
		.filter((question): question is FormQuestion => Boolean(question));
	if (!questions.length) {
		return uiDeclarativeSectionsToTree([
			{
				type: "section",
				id: "forms-play-inspector.empty",
				label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
				children: [{ type: "text", value: selectedIds.length ? "Question not found." : "Select a question in the hierarchy." }],
			},
		]);
	}
	const groups: UiInspectorFieldGroup[] = [];
	const kinds = [...new Set(questions.map((entry) => entry.kind))];
	for (const kind of kinds) {
		const kindQuestions = questions.filter((entry) => entry.kind === kind);
		const kindGroup = formsPlayInspectorKindGroup(kind, kindQuestions);
		if (kindGroup) groups.push(kindGroup);
	}
	groups.push(formsPlayInspectorBaseGroup(questions));
	return uiInspectorGroupsToTree(groups);
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
	private readonly docStore = new DocumentVcsStore<FormSpec, FormEditOp>({
		envelope: createDocumentVcsEnvelope("forms.form/v1", "forms-play", defaultFormSpec()),
		applyOp: applyFormEditOp,
		backwardsOp: backwardsFormEditOp,
		diffOp: diffFormEditOp,
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

	private applySpecEdit(op: FormEditOp): void {
		recordProjectionChange(this.docStore, [op]);
		this.tryValues = {};
		this.rebuildShellMode();
		this.notifySnapshot();
		this.emit();
	}

	private commitDocument(next: FormSpec): void {
		this.applySpecEdit({ op: "setDocument", document: next });
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

	getDocumentVcsStore(): DocumentVcsStore<FormSpec, FormEditOp> {
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
		this.applySpecEdit({ op: "setDocument", document: spec });
	}

	private editMeasures(): readonly WindowMeasure[] {
		const spec = this.projection();
		return [
			{
				kind: "slider",
				id: "forms-edit-steps",
				label: "Steps",
				value: spec.steps.length,
				min: 1,
				max: Math.max(spec.steps.length, 1),
				step: 1,
				onChange: formsPlayCmd("addStep"),
			},
		];
	}

	private tryMeasures(): readonly WindowMeasure[] {
		const spec = this.projection();
		const questionCount = spec.steps.reduce((count, step) => count + step.questions.length, 0);
		return [
			{
				kind: "slider",
				id: "forms-try-questions",
				label: "Questions",
				value: questionCount,
				min: 0,
				max: Math.max(questionCount, 1),
				step: 1,
				onChange: formsPlayCmd("exportFixture"),
			},
		];
	}

	private editEngagement(): WindowEngagement {
		return {
			sessionActive: false,
			input: {
				id: "forms-edit-engagement",
				value: "",
				placeholder: "Add step",
				onChange: formsPlayCmd("editEngagementInput"),
				onSubmit: formsPlayCmd("addStep"),
			},
			possibleEngagements: [
				{ id: "forms-add-step", label: "Add step", command: formsPlayCmd("addStep") },
				{ id: "forms-add-question", label: "Add question", command: formsPlayCmd("addQuestion", { kind: "text" }) },
			],
		};
	}

	private tryEngagement(): WindowEngagement {
		return {
			sessionActive: false,
			input: {
				id: "forms-try-engagement",
				value: "",
				placeholder: "Export JSON",
				onChange: formsPlayCmd("tryEngagementInput"),
				onSubmit: formsPlayCmd("exportFixture"),
			},
			status: [{ id: "forms-try-values", text: `${Object.keys(this.tryValues).length} answered` }],
		};
	}

	private rebuildShellMode(): void {
		this.mainMode.tools = buildFormsPlayToolbarTools(this.id);
		this.mainMode.windowKinds = [
			new WindowKindRuntime(FORMS_PLAY_WINDOW_KIND_EDIT, "Edit", FORMS_PLAY_BODY_KEY_EDIT, undefined, this.editMeasures(), this.editEngagement()),
			new WindowKindRuntime(FORMS_PLAY_WINDOW_KIND_TRY, "Try", FORMS_PLAY_BODY_KEY_TRY, undefined, this.tryMeasures(), this.tryEngagement()),
		];
		for (const windowKind of this.mainMode.windowKinds) {
			enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Forms play window "${windowKind.id}"`);
		}
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
			this.applySpecEdit({
				op: "updateQuestion",
				stepId: location.stepId,
				question: { ...question, options } as FormQuestion,
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
			this.applySpecEdit({
				op: "updateQuestion",
				stepId: location.stepId,
				question: { ...question, fields: vectorFields } as FormQuestion,
			});
			return;
		}
		let value: unknown = rawValue;
		if (field === "options" || field === "fields") {
			const nextQuestion = { ...question, [field]: rawValue } as FormQuestion;
			this.applySpecEdit({ op: "updateQuestion", stepId: location.stepId, question: nextQuestion });
			return;
		}
		if (field === "required") value = Boolean(rawValue);
		else if (field === "default" && question.kind === "boolean") value = Boolean(rawValue);
		else if (field === "default" && question.kind === "multi" && Array.isArray(rawValue)) value = rawValue;
		else if (field === "default" && question.kind === "multi") value = String(rawValue ?? "").split(",").map((entry) => entry.trim()).filter(Boolean);
		else if (field === "default" && question.kind === "vector" && Array.isArray(rawValue)) value = rawValue;
		else if (field === "min" || field === "max" || field === "step" || (field === "default" && (question.kind === "number" || question.kind === "slider"))) value = Number(rawValue);
		const nextQuestion = { ...question, [field]: value } as FormQuestion;
		this.applySpecEdit({ op: "updateQuestion", stepId: location.stepId, question: nextQuestion });
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
			this.applySpecEdit({
				op: "addStep",
				step: { id: stepId, title: `Step ${this.projection().steps.length + 1}`, questions: [] },
			});
			return;
		}
		if (command === "addQuestion") {
			const kind = (args as { kind?: string }).kind ?? "text";
			const stepId = (args as { stepId?: string }).stepId ?? this.projection().steps[0]?.id;
			if (!stepId) return;
			this.applySpecEdit({
				op: "addQuestion",
				stepId,
				question: defaultQuestionForKind(kind, createFormId("q")),
			});
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
			this.applySpecEdit({
				op: "addQuestion",
				stepId,
				index,
				question: defaultQuestionForKind(kind, createFormId("q")),
			});
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
			this.applySpecEdit({
				op: "moveQuestion",
				questionId,
				fromStepId: source.stepId,
				toStepId,
				index: index ?? 0,
			});
			return;
		}
		if (command === "moveStep") {
			const stepId = (args as { stepId?: string }).stepId;
			const index = (args as { index?: number }).index;
			if (!stepId || typeof index !== "number") return;
			this.applySpecEdit({ op: "moveStep", stepId, index });
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
		if (command === "patchQuestions") {
			const questionIds = (Array.isArray((args as { questionIds?: string[] }).questionIds) ? (args as { questionIds?: string[] }).questionIds : []).map(String).filter(Boolean);
			const field = (args as { field?: string }).field;
			const patch = args as { value?: unknown; pressed?: boolean };
			const rawValue = patch.value ?? patch.pressed;
			if (!questionIds.length || !field) return;
			for (const questionId of questionIds) {
				this.patchQuestionField(questionId, field, rawValue);
			}
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
				this.rebuildShellMode();
				this.notifySnapshot();
				this.emit();
			}
			return;
		}
		if (command === "resetTry") {
			this.tryValues = {};
			this.rebuildShellMode();
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

		it("batch-patches shared fields across multiple questions", () => {
			const bus = new CommandBus();
			const ctrl = new FormsPlayController(bus, () => {});
			const stepId = ctrl.getSpec().steps[0]?.id;
			if (!stepId) return;
			ctrl.run("addQuestion", { kind: "text", stepId });
			ctrl.run("addQuestion", { kind: "text", stepId });
			const questions = ctrl.getSpec().steps[0]?.questions.filter((entry) => entry.kind === "text").slice(-2) ?? [];
			if (questions.length < 2) return;
			ctrl.run("patchQuestions", { questionIds: questions.map((entry) => entry.id), field: "required", pressed: true });
			for (const question of questions) {
				expect(findQuestionLocation(ctrl.getSpec(), question.id)?.question.required).toBe(true);
			}
		});

		it("orders inspector sections kind-specific before base", () => {
			const spec = defaultFormSpec();
			const question = spec.steps[0]?.questions[0];
			if (!question) return;
			const tree = buildFormsPlayInspectorTree(spec, [question.id]);
			const labels = (tree.type === "tree" ? tree.sections : []).map((section) => section.label);
			expect(labels.indexOf("Text")).toBeLessThan(labels.indexOf("Question"));
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
