// #region 🧲Header
/** @emoji 🔧 Procedural play harness on `@framework/playground/core`. */
// #endregion 🧲Header

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	Platform,
	Playground,
	WindowKindRuntime,
	buildFlowWindowBody,
	createStackLayout,
	enforcePlaygroundWindowEngagementInput,
	registerWindowBody,
	type CommandDescriptor,
	type UiNode,
	type UiTreeItemNode,
	type UiTreeSectionNode,
	type WindowBodyViewContext,
	type WindowEngagement,
} from "@framework/playground/core";
import { bootstrapElementsSurfaceChromeDocument } from "@ui/react";
import {
	flowPlayCatalogueItemDragData,
	type CatalogueItem,
	type CatalogueSection,
	type FlowExtensionEntry,
	type FlowReorganizeRequest,
} from "@flow/react";
import {
	PROCEDURAL_DEFAULT_FIXTURE,
	proceduralExtensionHost,
	proceduralFixtureToJson,
	type FlowFixtureV1,
} from "@procedural/react";

export const PROCEDURAL_PLAY_APP_ID = "procedural-play";
export const PROCEDURAL_PLAY_CONTROLLER_ID = "procedural-play";
export const PROCEDURAL_PLAY_SURFACE_ID = "procedural.play/v1";
export const PROCEDURAL_PLAY_BODY_KEY_MAIN = "procedural.play.main";
export const PROCEDURAL_PLAY_WINDOW_KIND_ID = "procedural-main";

export const PROCEDURAL_PLAY_DEFAULT_FIXTURE: FlowFixtureV1 = PROCEDURAL_DEFAULT_FIXTURE;
export const PROCEDURAL_PLAY_DEFAULT_FIXTURE_JSON = proceduralFixtureToJson(PROCEDURAL_DEFAULT_FIXTURE);
export const PROCEDURAL_PLAY_LAYOUT = createStackLayout([PROCEDURAL_PLAY_WINDOW_KIND_ID], ["Procedural"]);
export const PROCEDURAL_PLAY_KINDS_TAB_ID = "procedural-play-kinds";
export const PROCEDURAL_PLAY_EXTENSIONS_TAB_ID = "procedural-play-extensions";

export type ProceduralLayoutOrientation = "leftRight" | "topBottom";

const DEFAULT_LAYER_SPACING = 120;
const DEFAULT_SIBLING_GAP = 40;

function proceduralPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
	return { controllerId: PROCEDURAL_PLAY_CONTROLLER_ID, command, args };
}

function buildProceduralLayoutOptionsJson(layerSpacing: number, siblingGap: number, orientation: ProceduralLayoutOrientation): string {
	return JSON.stringify({ layerSpacing, siblingGap, orientation });
}

/** @emoji 🧩 Workbench extensions tab: installed modules with enable/disable toggles. */
export function buildProceduralPlayExtensionsTree(entries: readonly FlowExtensionEntry[]): UiNode {
	if (!entries.length) {
		return {
			type: "tree",
			sections: [
				{
					id: "procedural-play-extensions.empty",
					label: "Extensions",
					defaultOpen: true,
					items: [{ id: "procedural-play-extensions.empty.msg", label: "Loading extensions…" }],
				},
			],
		};
	}
	return {
		type: "tree",
		sections: [
			{
				id: "procedural-play-extensions.installed",
				label: "Installed",
				defaultOpen: true,
				items: entries.map((entry) => ({
					id: `procedural-play-extensions.${entry.id}`,
					label: entry.manifest.name,
					description: `${entry.manifest.version} · ${entry.active ? "enabled" : "disabled"} · ${entry.manifest.contributes.neuronKinds.length} kinds · ${entry.manifest.contributes.commands.length} commands`,
					command: proceduralPlayCmd("toggleExtension", { id: entry.id, enabled: !entry.active }),
				})),
			},
			{
				id: "procedural-play-extensions.commands",
				label: "Commands",
				defaultOpen: true,
				items: proceduralExtensionHost.activeCommands().map((command) => ({
					id: `procedural-play-extensions.command.${command.id}`,
					label: command.title,
					description: command.id,
					command: proceduralPlayCmd("runExtensionCommand", { commandId: command.id }),
				})),
			},
		],
	};
}

function proceduralPlayKindsTreeItem(sectionId: string, index: number, item: CatalogueItem): UiTreeItemNode {
	return {
		id: `procedural-play-kinds.${sectionId}.${index}.${item.neuronKind ?? item.kind}`,
		label: item.name,
		description: item.summary,
		draggable: true,
		dragData: flowPlayCatalogueItemDragData(item),
	};
}

/** @emoji 🏷️ Workbench catalogue tab: module sections plus Inputs and Outputs. */
export function buildProceduralPlayKindsTree(sections: readonly CatalogueSection[]): UiNode {
	if (!sections.length) {
		return {
			type: "tree",
			sections: [
				{
					id: "procedural-play-kinds.empty",
					label: "Catalogue",
					defaultOpen: true,
					items: [{ id: "procedural-play-kinds.empty.msg", label: "Loading catalogue…" }],
				},
			],
		};
	}
	const treeSections: UiTreeSectionNode[] = sections.map((section) => ({
		id: `procedural-play-kinds.${section.id}`,
		label: section.title,
		defaultOpen: true,
		items: section.items.map((item, index) => proceduralPlayKindsTreeItem(section.id, index, item)),
	}));
	return { type: "tree", sections: treeSections };
}

/** @emoji 🎛 Procedural play shell controller. */
export class ProceduralPlayController extends Controller {
	readonly mainMode = new ModeRuntime("main", "Procedural", undefined);
	private fixtureJson = PROCEDURAL_PLAY_DEFAULT_FIXTURE_JSON;
	private previewText = "—";
	private catalogueSections: CatalogueSection[] = [];
	private catalogueRevision = 0;
	private readonly snapshotListeners = new Set<() => void>();
	private engagementInput = "";
	private layerSpacing = DEFAULT_LAYER_SPACING;
	private siblingGap = DEFAULT_SIBLING_GAP;
	private orientation: ProceduralLayoutOrientation = "leftRight";
	private reorganizeEpoch = 0;
	private reorganizeOptionsJson = buildProceduralLayoutOptionsJson(DEFAULT_LAYER_SPACING, DEFAULT_SIBLING_GAP, "leftRight");
	private extensionRevision = 0;

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(PROCEDURAL_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.rebuildShellMode();
	}

	getFixtureJson(): string {
		return this.fixtureJson;
	}

	getPreviewText(): string {
		return this.previewText;
	}

	getCatalogueSections(): readonly CatalogueSection[] {
		return this.catalogueSections;
	}

	getCatalogueRevision(): number {
		return this.catalogueRevision;
	}

	getExtensionRevision(): number {
		return this.extensionRevision;
	}

	getExtensionEntries(): readonly FlowExtensionEntry[] {
		return proceduralExtensionHost.listEntries();
	}

	/** @emoji 🔔 Subscribes to catalogue updates for workbench kinds panel refresh. */
	subscribeSnapshot(listener: () => void): () => void {
		this.snapshotListeners.add(listener);
		return () => this.snapshotListeners.delete(listener);
	}

	private notifySnapshot(): void {
		for (const listener of this.snapshotListeners) {
			listener();
		}
	}

	getReorganize(): FlowReorganizeRequest {
		return { epoch: this.reorganizeEpoch, optionsJson: this.reorganizeOptionsJson };
	}

	private syncReorganizeOptionsJson(): void {
		this.reorganizeOptionsJson = buildProceduralLayoutOptionsJson(this.layerSpacing, this.siblingGap, this.orientation);
	}

	private triggerReorganize(): void {
		this.syncReorganizeOptionsJson();
		this.reorganizeEpoch += 1;
		this.rebuildShellMode();
		this.emit();
	}

	private windowEngagement(): WindowEngagement {
		return {
			sessionActive: false,
			input: {
				id: "engagement-input",
				value: this.engagementInput,
				placeholder: "Reorganize, lr, tb",
				onChange: proceduralPlayCmd("engagementInput"),
				onSubmit: proceduralPlayCmd("engagementSubmit"),
			},
			possibleEngagements: [
				{ id: "procedural.tool.reorganize", label: "Reorganize", command: proceduralPlayCmd("reorganize") },
				{ id: "procedural.layout.leftRight", label: "Left to Right", command: proceduralPlayCmd("setOrientation", { orientation: "leftRight" }) },
				{ id: "procedural.layout.topBottom", label: "Top to Bottom", command: proceduralPlayCmd("setOrientation", { orientation: "topBottom" }) },
			],
			controls: [
				{
					kind: "slider",
					id: "procedural-layer-spacing",
					label: "Layer spacing",
					value: this.layerSpacing,
					min: 40,
					max: 320,
					step: 10,
					onChange: proceduralPlayCmd("setSpacing", { field: "layerSpacing" }),
				},
				{
					kind: "slider",
					id: "procedural-sibling-gap",
					label: "Sibling gap",
					value: this.siblingGap,
					min: 10,
					max: 160,
					step: 5,
					onChange: proceduralPlayCmd("setSpacing", { field: "siblingGap" }),
				},
			],
			status: [{ id: "procedural-layout-orientation", text: this.orientation === "leftRight" ? "Left to right" : "Top to bottom" }],
		};
	}

	private rebuildShellMode(): void {
		this.mainMode.windowKinds = [
			new WindowKindRuntime(PROCEDURAL_PLAY_WINDOW_KIND_ID, "Procedural", PROCEDURAL_PLAY_BODY_KEY_MAIN, undefined, [], this.windowEngagement()),
		];
		for (const windowKind of this.mainMode.windowKinds) {
			enforcePlaygroundWindowEngagementInput(windowKind.engagement, `Procedural play window "${windowKind.id}"`);
		}
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
			const orientation = (args as { orientation?: ProceduralLayoutOrientation }).orientation;
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
		if (command === "setFixtureJson") {
			const json = (args as { json?: string }).json;
			if (typeof json === "string" && json !== this.fixtureJson) {
				this.fixtureJson = json;
				this.emit();
			}
			return;
		}
		if (command === "setPreviewText") {
			const text = (args as { text?: string }).text;
			if (typeof text === "string" && text !== this.previewText) {
				this.previewText = text;
				this.emit();
			}
			return;
		}
		if (command === "setCatalogueSections") {
			const sections = (args as { sections?: CatalogueSection[] }).sections;
			if (Array.isArray(sections)) {
				this.catalogueSections = sections;
				this.catalogueRevision += 1;
				this.notifySnapshot();
				this.emit();
			}
			return;
		}
		if (command === "toggleExtension") {
			const id = (args as { id?: string }).id;
			const enabled = (args as { enabled?: boolean }).enabled;
			if (typeof id !== "string" || typeof enabled !== "boolean") return;
			void proceduralExtensionHost.setActive(id, enabled).then(() => {
				this.extensionRevision += 1;
				this.notifySnapshot();
				this.emit();
			});
			return;
		}
		if (command === "runExtensionCommand") {
			const commandId = (args as { commandId?: string }).commandId;
			if (typeof commandId !== "string") return;
			const result = proceduralExtensionHost.executeCommand(commandId);
			console.log(`[DEBUG] procedural extension command ${commandId}: ${result}`);
			this.emit();
			return;
		}
	}

	private applyEngagement(value: string): void {
		const trimmed = value.trim().toLowerCase();
		if (!trimmed) return;
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

}

export function registerProceduralPlayDeclarativeBodies(): void {
	registerWindowBody(PROCEDURAL_PLAY_BODY_KEY_MAIN, (_ctx: WindowBodyViewContext) => buildFlowWindowBody(PROCEDURAL_PLAY_SURFACE_ID));
}

export function buildProceduralPlayAppRuntime(controller: ProceduralPlayController): AppRuntime {
	const app = new AppRuntime(PROCEDURAL_PLAY_APP_ID, "Procedural", undefined, controller, PROCEDURAL_PLAY_LAYOUT, []);
	app.defaultModeId = controller.mainMode.id;
	app.addMode(controller.mainMode);
	return app;
}

/** @emoji 🛝 Procedural playground app. */
export class PlaygroundProcedural extends Playground {
	readonly id = PROCEDURAL_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = new Platform({ id: this.id });
		const ctrl = new ProceduralPlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildProceduralPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerProceduralPlayDeclarativeBodies();
	}
}

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("@procedural/play", () => {
		it("exports default fixture json", () => {
			expect(PROCEDURAL_PLAY_DEFAULT_FIXTURE_JSON).toContain("flow.fixture/v1");
		});

		it("controller stores fixture json", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			ctrl.run("setFixtureJson", { json: '{"schema":"flow.fixture/v1"}' });
			expect(ctrl.getFixtureJson()).toContain("flow.fixture/v1");
		});

		it("kinds tree marks catalogue rows draggable", () => {
			const tree = buildProceduralPlayKindsTree([
				{
					id: "brep",
					title: "Brep",
					items: [{ kind: "neuron", neuronKind: "brep.box", name: "Box", summary: "Box solid" }],
				},
			]);
			expect(tree.type).toBe("tree");
			const item = tree.sections?.[0]?.items?.[0];
			expect(item?.draggable).toBe(true);
			expect(item?.dragData).toBeDefined();
		});

		it("catalogue revision bumps when sections arrive", () => {
			const bus = new CommandBus();
			const ctrl = new ProceduralPlayController(bus, () => {});
			expect(ctrl.getCatalogueRevision()).toBe(0);
			ctrl.run("setCatalogueSections", {
				sections: [{ id: "brep", title: "Brep", items: [{ kind: "neuron", neuronKind: "brep.box", name: "Box", summary: "Box" }] }],
			});
			expect(ctrl.getCatalogueRevision()).toBe(1);
		});

		it("extensions tree lists installed modules", () => {
			const tree = buildProceduralPlayExtensionsTree([
				{
					id: "brep",
					active: true,
					manifest: {
						schema: "flow.module/v1",
						id: "brep",
						name: "Brep",
						version: "0.1.0",
						activationEvents: ["onStartup"],
						contributes: {
							neuronKinds: [{ id: "brep.box", module: "brep", name: "Box", summary: "Box", inputs: [], outputs: ["brep"] }],
							widgets: [],
							commands: [],
							settings: [],
						},
					},
				},
			]);
			const labels = tree.sections?.flatMap((section) => section.items?.map((item) => item.label) ?? []) ?? [];
			expect(labels).toContain("Brep");
		});
	});
}
// #endregion 🧪Tests

// #region 🔖Boot
if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest && import.meta.env.PUZZLE_PLAY_ENTRY === "procedural") {
	bootstrapElementsSurfaceChromeDocument("system");
	void (async () => {
		await import("./globals.css");
		const { bootProceduralPlay } = await import("@framework/playground/renderer/react/procedural");
		bootProceduralPlay(new PlaygroundProcedural());
	})();
}
// #endregion 🔖Boot
