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
	type PlaygroundFixtureCatalog,
	type PlaygroundFixtureHost,
	type ToolLeaf,
	toolCollection,
	type UiNode,
	type UiSectionNode,
} from "@semio-tech/framework-playground-core";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import { applyFormEditOp, createFormId, defaultQuestionForKind, formSpecToJson, parseFormSpec, type FormQuestionKind, type FormSpec } from "@semio-tech/forms-core";
import { defaultFormSpec, formSpecFromJson } from "@semio-tech/forms-react";
import { FORMS_PLAY_FIXTURE_DEFAULT_ID, resolveFormsPlayFixtureSlug } from "./fixture-slugs.ts";

export const FORMS_PLAY_APP_ID = "forms-play";
export const FORMS_PLAY_CONTROLLER_ID = "forms-play";
export const FORMS_PLAY_SURFACE_ID_BUILDER = "forms.play.builder/v1";
export const FORMS_PLAY_SURFACE_ID_PREVIEW = "forms.play.preview/v1";
export const FORMS_PLAY_BODY_KEY_BUILDER = "forms.play.builder";
export const FORMS_PLAY_BODY_KEY_PREVIEW = "forms.play.preview";
export const FORMS_PLAY_WINDOW_KIND_BUILDER = "forms-builder";
export const FORMS_PLAY_WINDOW_KIND_PREVIEW = "forms-preview";
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

export function buildFormsPlayHierarchyTree(spec: FormSpec, selectedIds: readonly string[]): UiNode {
	return uiDeclarativeSectionsToTree([
		{
			type: "section",
			id: "forms-play-hierarchy",
			label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
			children: spec.steps.flatMap((step) => [
				{ type: "text", value: step.title },
				...step.questions.map((question) => ({
					type: "field" as const,
					id: question.id,
					label: question.label,
					child: { type: "text" as const, value: question.kind },
				})),
			]),
		},
	]);
}

export function buildFormsPlayCatalogueTree(): UiNode {
	return uiDeclarativeSectionsToTree([
		{
			type: "section",
			id: "forms-play-catalogue",
			label: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
			children: [
				{ type: "text", value: "Drag question kinds from the Builder catalogue panel." },
				{ type: "button", id: "forms.add-step", label: "Add Step", command: { controllerId: FORMS_PLAY_CONTROLLER_ID, command: "addStep" } },
			],
		},
	]);
}

export function buildFormsPlayInspectorTree(spec: FormSpec, selectedIds: readonly string[]): UiNode {
	const questionId = selectedIds[0];
	const question = spec.steps.flatMap((step) => step.questions).find((entry) => entry.id === questionId);
	return uiDeclarativeSectionsToTree([
		{
			type: "section",
			id: "forms-play-inspector",
			label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
			children: question
				? [
						{ type: "field", id: "label", label: "Label", child: { type: "text", value: question.label } },
						{ type: "field", id: "kind", label: "Kind", child: { type: "text", value: question.kind } },
					]
				: [{ type: "text", value: "Select a question in the Builder window." }],
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
	readonly mainMode = new ModeRuntime("main", "Edit", undefined);
	private spec: FormSpec = defaultFormSpec();
	private selectedIds: string[] = [];
	private interactionRevision = 0;
	private readonly snapshotListeners = new Set<() => void>();

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(FORMS_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.rebuildShellMode();
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
		this.notifySnapshot();
		this.emit();
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
			const stepId = this.spec.steps[0]?.id;
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
