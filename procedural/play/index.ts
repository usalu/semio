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
	type WindowBodyViewContext,
	type WindowEngagement,
} from "@framework/playground/core";
import { bootstrapElementsSurfaceChromeDocument } from "@ui/react";
import {
	PROCEDURAL_DEFAULT_FIXTURE,
	proceduralFixtureToJson,
	type CatalogueSection,
	type FlowReorganizeRequest,
} from "@procedural/react";

export const PROCEDURAL_PLAY_APP_ID = "procedural-play";
export const PROCEDURAL_PLAY_CONTROLLER_ID = "procedural-play";
export const PROCEDURAL_PLAY_SURFACE_ID = "procedural.play/v1";
export const PROCEDURAL_PLAY_BODY_KEY_MAIN = "procedural.play.main";
export const PROCEDURAL_PLAY_WINDOW_KIND_ID = "procedural-main";

export const PROCEDURAL_PLAY_DEFAULT_FIXTURE_JSON = proceduralFixtureToJson(PROCEDURAL_DEFAULT_FIXTURE);
export const PROCEDURAL_PLAY_LAYOUT = createStackLayout([PROCEDURAL_PLAY_WINDOW_KIND_ID], ["Procedural"]);

function proceduralPlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
	return { controllerId: PROCEDURAL_PLAY_CONTROLLER_ID, command, args };
}

/** @emoji 🎛 Procedural play shell controller. */
export class ProceduralPlayController extends Controller {
	readonly mainMode = new ModeRuntime("main", "Procedural", undefined);
	private fixtureJson = PROCEDURAL_PLAY_DEFAULT_FIXTURE_JSON;
	private previewText = "—";
	private catalogueSections: CatalogueSection[] = [];
	private catalogueRevision = 0;
	private extensionRevision = 0;
	private reorganizeEpoch = 0;
	private reorganizeOptionsJson = JSON.stringify({ layerSpacing: 120, siblingGap: 40, orientation: "leftRight" });

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

	getReorganize(): FlowReorganizeRequest {
		return { epoch: this.reorganizeEpoch, optionsJson: this.reorganizeOptionsJson };
	}

	private windowEngagement(): WindowEngagement {
		return {
			sessionActive: false,
			input: {
				id: "engagement-input",
				value: "",
				placeholder: "Procedural",
				onChange: proceduralPlayCmd("engagementInput"),
				onSubmit: proceduralPlayCmd("engagementSubmit"),
			},
			possibleEngagements: [],
			controls: [],
			status: [],
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
		if (command === "setFixtureJson") {
			const json = (args as { json?: string }).json;
			if (typeof json === "string") {
				this.fixtureJson = json;
				this.emit();
			}
			return;
		}
		if (command === "setPreviewText") {
			const text = (args as { text?: string }).text;
			if (typeof text === "string") {
				this.previewText = text;
				this.emit();
			}
			return;
		}
		if (command === "setCatalogueSections") {
			const sections = (args as { sections?: CatalogueSection[] }).sections;
			if (sections) {
				this.catalogueSections = sections;
				this.catalogueRevision += 1;
				this.emit();
			}
			return;
		}
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
