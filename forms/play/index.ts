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
	createStackLayout,
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
import { bootstrapElementsSurfaceChromeDocument, treeReorderDragController, type TreeDataItem, type TreeDragAndDropController, type TreeDropPosition } from "@semio-tech/ui-react";
import {
	QUESTION_KIND_CATALOGUE,
	applyFormEditOp,
	createFormId,
	defaultQuestionForKind,
	findQuestionLocation,
	formSpecToJson,
	type FormQuestion,
	type FormQuestionKind,
	type FormSpec,
	type FormValues,
} from "@semio-tech/forms-core";
import { FORMS_QUESTION_DRAG_MIME, defaultFormSpec, formSpecFromJson, formsQuestionPaletteTreeDragController } from "@semio-tech/forms-react";
import { FORMS_PLAY_FIXTURE_DEFAULT_ID, resolveFormsPlayFixtureSlug } from "./fixture-slugs.ts";

export const FORMS_PLAY_APP_ID = "forms-play";
export const FORMS_PLAY_CONTROLLER_ID = "forms-play";
export const FORMS_PLAY_SURFACE_ID_BUILDER = "forms.play.builder/v1";
export const FORMS_PLAY_SURFACE_ID_PREVIEW = "forms.play.preview/v1";
export const FORMS_PLAY_SURFACE_ID_TRY = "forms.play.try/v1";
export const FORMS_PLAY_BODY_KEY_BUILDER = "forms.play.builder";
export const FORMS_PLAY_BODY_KEY_PREVIEW = "forms.play.preview";
export const FORMS_PLAY_BODY_KEY_TRY = "forms.play.try";
export const FORMS_PLAY_WINDOW_KIND_BUILDER = "forms-builder";
export const FORMS_PLAY_WINDOW_KIND_PREVIEW = "forms-preview";
export const FORMS_PLAY_WINDOW_KIND_TRY = "forms-try";
export const FORMS_PLAY_EDIT_MODE_ID = "main";
export const FORMS_PLAY_TRY_MODE_ID = "try";
export const FORMS_PLAY_HIERARCHY_TAB_ID = "framework.panel.hierarchy";
export const FORMS_PLAY_CATALOGUE_TAB_ID = "framework.panel.catalogue";
export const FORMS_PLAY_INSPECTION_TAB_ID = "framework.panel.inspection";

export const FORMS_PLAY_LAYOUT = createDefaultLayout(
	[FORMS_PLAY_WINDOW_KIND_BUILDER, FORMS_PLAY_WINDOW_KIND_PREVIEW],
	"row",
	[62, 38],
	["Builder", "Preview"],
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
	{ id: FORMS_PLAY_FIXTURE_DEFAULT_ID, label: "Default Contact" },
	...Object.keys(FORMS_PLAY_FILE_FIXTURE_JSON_BY_ID)
		.sort()
		.map((id) => ({ id, label: formsFixtureLabelFromId(id) })),
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

/** @emoji 🖱️ Side-panel hierarchy drag: reorder questions and accept catalogue drops. */
export function createFormsPlayHierarchyTreeDragController(getController: () => FormsPlayController | undefined): TreeDragAndDropController {
	const reorder = treeReorderDragController({
		resolveParentId: (item) => {
			if (item.id.startsWith("step:")) return item.id;
			const spec = getController()?.getSpec();
			if (!spec) return undefined;
			const location = findQuestionLocation(spec, item.id);
			return location ? formsPlayStepTreeId(location.stepId) : undefined;
		},
		onMove: (move) => {
			getController()?.run("moveQuestion", {
				questionId: move.itemId,
				toStepId: move.toParentId.startsWith("step:") ? move.toParentId.slice(5) : move.toParentId,
				targetId: move.toParentId,
				position: move.position,
			});
		},
	});
	return {
		...reorder,
		handleDrop: (context) => {
			const cataloguePayload = context.data[FORMS_QUESTION_DRAG_MIME];
			if (cataloguePayload && context.targetKind === "item") {
				try {
					const payload = JSON.parse(cataloguePayload) as { kind?: FormQuestionKind };
					if (payload.kind) {
						getController()?.run("dropQuestionKind", {
							kind: payload.kind,
							targetId: (context.target as TreeDataItem).id,
							dropPosition: context.dropPosition,
						});
					}
				} catch {
					/* ignore malformed catalogue payload */
				}
				return;
			}
			reorder.handleDrop?.(context);
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
	const kindItems: UiTreeItemNode[] = QUESTION_KIND_CATALOGUE.map((entry) => ({
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

function formsPlayInspectorFields(question: FormQuestion): UiNode[] {
	const fields: UiNode[] = [
		{
			type: "field",
			id: "forms-play-inspector.label",
			label: "Label",
			child: {
				type: "input",
				id: "forms-play-inspector.label.input",
				inputKind: "text",
				value: question.label,
				commit: "blur",
				onChange: formsPlayCmd("patchQuestion", { questionId: question.id, field: "label" }),
			},
		},
		{
			type: "field",
			id: "forms-play-inspector.kind",
			label: "Kind",
			child: { type: "text", value: question.kind },
		},
		{
			type: "field",
			id: "forms-play-inspector.required",
			label: "Required",
			child: {
				type: "toggle",
				id: "forms-play-inspector.required.toggle",
				pressed: Boolean(question.required),
				text: question.required ? "Required" : "Optional",
				onChange: formsPlayCmd("patchQuestion", { questionId: question.id, field: "required" }),
			},
		},
	];
	if (question.kind === "text" || question.kind === "longText") {
		fields.push({
			type: "field",
			id: "forms-play-inspector.placeholder",
			label: "Placeholder",
			child: {
				type: "input",
				id: "forms-play-inspector.placeholder.input",
				inputKind: "text",
				value: question.placeholder ?? "",
				commit: "blur",
				onChange: formsPlayCmd("patchQuestion", { questionId: question.id, field: "placeholder" }),
			},
		});
	}
	if (question.kind === "slider" || question.kind === "number") {
		fields.push(
			{
				type: "field",
				id: "forms-play-inspector.min",
				label: "Min",
				child: {
					type: "input",
					id: "forms-play-inspector.min.input",
					inputKind: "number",
					value: String(question.min ?? 0),
					commit: "blur",
					onChange: formsPlayCmd("patchQuestion", { questionId: question.id, field: "min" }),
				},
			},
			{
				type: "field",
				id: "forms-play-inspector.max",
				label: "Max",
				child: {
					type: "input",
					id: "forms-play-inspector.max.input",
					inputKind: "number",
					value: String(question.max ?? 100),
					commit: "blur",
					onChange: formsPlayCmd("patchQuestion", { questionId: question.id, field: "max" }),
				},
			},
		);
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
	readonly mainMode = new ModeRuntime(FORMS_PLAY_EDIT_MODE_ID, "Edit", undefined);
	readonly tryMode = new ModeRuntime(FORMS_PLAY_TRY_MODE_ID, "Try", undefined);
	private spec: FormSpec = defaultFormSpec();
	private selectedIds: string[] = [];
	private tryValues: FormValues = {};
	private interactionRevision = 0;
	private readonly snapshotListeners = new Set<() => void>();

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(FORMS_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.rebuildShellMode();
		this.rebuildTryMode();
	}

	getSpec(): FormSpec {
		return this.spec;
	}

	getSpecJson(): string {
		return formSpecToJson(this.spec);
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
		this.spec = spec;
		this.tryValues = {};
		this.notifySnapshot();
		this.emit();
	}

	private rebuildTryMode(): void {
		this.tryMode.windowKinds = [new WindowKindRuntime(FORMS_PLAY_WINDOW_KIND_TRY, "Try", FORMS_PLAY_BODY_KEY_TRY)];
		this.tryMode.defaultLayout = createStackLayout([FORMS_PLAY_WINDOW_KIND_TRY], ["Try"]);
	}

	getFixtureCatalog(): PlaygroundFixtureCatalog {
		const activeFixtureId = playgroundResolvedFixtureId(this.spec.id, FORMS_PLAY_FIXTURE_DEFAULT_ID);
		return {
			activeFixtureId,
			options: FORMS_PLAY_FIXTURE_OPTIONS,
			locked: isPlaygroundFixtureLocked(),
		};
	}

	private rebuildShellMode(): void {
		this.mainMode.tools = buildFormsPlayToolbarTools(this.id);
		this.mainMode.windowKinds = [
			new WindowKindRuntime(FORMS_PLAY_WINDOW_KIND_BUILDER, "Builder", FORMS_PLAY_BODY_KEY_BUILDER),
			new WindowKindRuntime(FORMS_PLAY_WINDOW_KIND_PREVIEW, "Preview", FORMS_PLAY_BODY_KEY_PREVIEW),
		];
	}

	override run(command: string, args?: unknown): void {
		if (command === "setActiveFixture") {
			const fixtureId = (args as { fixtureId?: string }).fixtureId;
			if (typeof fixtureId !== "string") return;
			if (isPlaygroundNoFixtureId(fixtureId)) {
				this.setSpec(defaultFormSpec());
				return;
			}
			const json = FORMS_PLAY_FILE_FIXTURE_JSON_BY_ID[fixtureId];
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
				applyFormEditOp(this.spec, {
					op: "addStep",
					step: { id: stepId, title: `Step ${this.spec.steps.length + 1}`, questions: [] },
				}),
			);
			return;
		}
		if (command === "addQuestion") {
			const kind = ((args as { kind?: FormQuestionKind }).kind ?? "text") as FormQuestionKind;
			const stepId = (args as { stepId?: string }).stepId ?? this.spec.steps[0]?.id;
			if (!stepId) return;
			this.setSpec(
				applyFormEditOp(this.spec, {
					op: "addQuestion",
					stepId,
					question: defaultQuestionForKind(kind, createFormId("q")),
				}),
			);
			return;
		}
		if (command === "dropQuestionKind") {
			const kind = (args as { kind?: FormQuestionKind }).kind;
			const targetId = (args as { targetId?: string }).targetId;
			const dropPosition = (args as { dropPosition?: TreeDropPosition }).dropPosition;
			if (!kind || !targetId) return;
			const stepId = resolveStepIdFromTreeTarget(this.spec, targetId);
			if (!stepId) return;
			const index = resolveQuestionInsertIndex(this.spec, stepId, targetId, dropPosition);
			this.setSpec(
				applyFormEditOp(this.spec, {
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
			const source = findQuestionLocation(this.spec, questionId);
			if (!source) return;
			const index = resolveQuestionInsertIndex(this.spec, toStepId, targetId, position);
			this.setSpec(
				applyFormEditOp(this.spec, {
					op: "moveQuestion",
					questionId,
					fromStepId: source.stepId,
					toStepId,
					index: index ?? 0,
				}),
			);
			return;
		}
		if (command === "patchQuestion") {
			const questionId = (args as { questionId?: string }).questionId;
			const field = (args as { field?: string }).field;
			const patch = args as { value?: unknown; pressed?: boolean };
			const rawValue = patch.value ?? patch.pressed;
			if (!questionId || !field) return;
			const location = findQuestionLocation(this.spec, questionId);
			if (!location) return;
			let value: unknown = rawValue;
			if (field === "required") value = Boolean(patch.pressed ?? rawValue);
			if (field === "min" || field === "max") value = Number(rawValue);
			const question = { ...location.question, [field]: value } as FormQuestion;
			const steps = this.spec.steps.map((step) =>
				step.id === location.stepId
					? { ...step, questions: step.questions.map((entry) => (entry.id === questionId ? question : entry)) }
					: step,
			);
			this.setSpec({ ...this.spec, steps });
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

function buildFormsPlayBuilderBody(_ctx: unknown): UiNode {
	return buildFormsWindowBody(FORMS_PLAY_SURFACE_ID_BUILDER, FORMS_PLAY_CONTROLLER_ID, "builder");
}

function buildFormsPlayPreviewBody(_ctx: unknown): UiNode {
	return buildFormsWindowBody(FORMS_PLAY_SURFACE_ID_PREVIEW, FORMS_PLAY_CONTROLLER_ID, "preview");
}

export function registerFormsPlayDeclarativeBodies(): void {
	registerWindowBody(FORMS_PLAY_BODY_KEY_BUILDER, buildFormsPlayBuilderBody);
	registerWindowBody(FORMS_PLAY_BODY_KEY_PREVIEW, buildFormsPlayPreviewBody);
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
			ctrl.run("setActiveFixture", { fixtureId: "onboarding" });
			expect(ctrl.getSpec().steps.length).toBeGreaterThan(1);
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
