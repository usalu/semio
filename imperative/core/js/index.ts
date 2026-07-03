// #region 🧲Header
/** @emoji ⚙️ Imperative play app — step-list editor. */
// #endregion 🧲Header

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	WindowKindRuntime,
	buildImperativeWindowBody,
	createPlayAppRuntime,
	createPlaygroundApp,
	createStackLayout,
	registerWindowBody,
	type CommandDescriptor,
	type UiNode,
	type WindowBodyViewContext,
} from "@semio-tech/framework-playground-core";
import { DEFAULT_IMPERATIVE_DOCUMENT, imperativeDocumentToJson } from "./internal.ts";

export * from "./internal.ts";

export const IMPERATIVE_PLAY_APP_ID = "imperative-play";
export const IMPERATIVE_PLAY_CONTROLLER_ID = "imperative-play";
export const IMPERATIVE_PLAY_SURFACE_ID = "imperative.play";
export const IMPERATIVE_PLAY_BODY_KEY_MAIN = "imperative.play.main";
export const IMPERATIVE_PLAY_WINDOW_KIND_ID = "imperative-main";
export const IMPERATIVE_PLAY_DEFAULT_DOCUMENT_JSON = imperativeDocumentToJson(DEFAULT_IMPERATIVE_DOCUMENT);
export const IMPERATIVE_PLAY_LAYOUT = createStackLayout([IMPERATIVE_PLAY_WINDOW_KIND_ID], ["Imperative"]);

function imperativePlayCmd(command: string, args?: Record<string, unknown>): CommandDescriptor {
	return { controllerId: IMPERATIVE_PLAY_CONTROLLER_ID, command, args };
}

/** @emoji 🎮 Imperative play controller. */
export class ImperativePlayController extends Controller {
	readonly mainMode = new ModeRuntime("main", "Edit", undefined);
	private documentJson = IMPERATIVE_PLAY_DEFAULT_DOCUMENT_JSON;

	constructor(commandBus: CommandBus, hostNotify: () => void) {
		super(IMPERATIVE_PLAY_CONTROLLER_ID, commandBus, hostNotify);
		this.mainMode.windowKinds = [
			new WindowKindRuntime(IMPERATIVE_PLAY_WINDOW_KIND_ID, "Imperative", IMPERATIVE_PLAY_BODY_KEY_MAIN),
		];
	}

	getDocumentJson(): string {
		return this.documentJson;
	}

	setDocumentJson(json: string): void {
		this.documentJson = json;
		this.emit();
	}

	override run(command: string, args?: Record<string, unknown>): void {
		if (command === "setDocumentJson" && typeof args?.json === "string") {
			this.setDocumentJson(args.json);
		}
	}
}

function buildImperativePlayMainDeclarativeBody(_ctx: WindowBodyViewContext): UiNode {
	return buildImperativeWindowBody(IMPERATIVE_PLAY_SURFACE_ID, IMPERATIVE_PLAY_CONTROLLER_ID, IMPERATIVE_PLAY_WINDOW_KIND_ID);
}

/** @emoji 🧩 Registers imperative play window bodies. */
export const imperativePlayWindowBodies: Readonly<Record<string, (ctx: import("@semio-tech/framework-platform-core").WindowBodyViewContext) => UiNode>> = {
	[IMPERATIVE_PLAY_BODY_KEY_MAIN]: buildImperativePlayMainDeclarativeBody,
};

export function registerImperativePlayDeclarativeBodies(): void {
	for (const [key, build] of Object.entries(imperativePlayWindowBodies)) registerWindowBody(key, build);
}

/** @emoji 🛝 Builds imperative play {@link AppRuntime}. */
export function buildImperativePlayAppRuntime(controller: ImperativePlayController): AppRuntime {
	return createPlayAppRuntime(IMPERATIVE_PLAY_APP_ID, "Imperative", controller, IMPERATIVE_PLAY_LAYOUT, controller.mainMode);
}

export { imperativePlayCmd };

//#region 🔖SExtension
import type { PlatformDefinition } from "@semio-tech/framework-platform-core";

/** @emoji 🧩 S program definition for imperative. */
export function buildImperativeProgramDefinition(): PlatformDefinition {
	return {
		id: "imperative",
		name: "Imperative",
		apiVersion: "1",
		apps: [
			{
				id: "imperative",
				label: "Imperative",
				controllerId: IMPERATIVE_PLAY_CONTROLLER_ID,
				modes: [{ id: "edit", label: "Edit" }],
				defaultModeId: "edit",
			},
		],
		createPlatformApi: () => ({}),
	};
}
//#endregion 🔖SExtension

//#region 🔖OsProgram
import { mergeOsProgramDefinition, osBaselineResource, registerAppVcsHandler } from "@semio-tech/framework-os-core";
import type { OsProgramContribution } from "@semio-tech/framework-platform-core";
import { createImperativeAppVcsHandler } from "./internal.ts";

const imperativeProgramContributionResources = {
		"imperative": { ...osBaselineResource("computation.imperative", "imperative.document", "imperative"), parameterFields: [{ fieldPath: "/camera/zoom", label: "Camera zoom", type: "numeric" }] },
	};

/** @emoji 🧩 OS program contribution for imperative. */
export const imperativeProgramContribution: OsProgramContribution = {
	programId: "imperative",
	register() {
		mergeOsProgramDefinition("imperative", buildImperativeProgramDefinition(), imperativeProgramContributionResources);
		registerAppVcsHandler(createImperativeAppVcsHandler());
	},
};
//#endregion 🔖OsProgram


if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("ImperativePlayController", () => {
		it("default document json is valid", () => {
			expect(IMPERATIVE_PLAY_DEFAULT_DOCUMENT_JSON).toContain("imperative.document");
			expect(IMPERATIVE_PLAY_DEFAULT_DOCUMENT_JSON).toContain("step-1");
		});
	});
}

//#region 🔖Play

/** @emoji 🛝 Imperative playground app. */

export const imperativePlayAppDefinition = createPlaygroundApp({
	id: IMPERATIVE_PLAY_APP_ID,
	label: "Imperative",
	controllerId: "imperative-play",
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	devHost: {
		playEntryKind: "imperative",
		resolveDedupe: ["react", "react-dom", "@semio-tech/imperative-react"],
		watchIgnored: ["../core/lib.rs", "../engine/**", "../module/**", "../core/target/**", "../core/pkg/**"],
		optimizeDeps: { include: ["react", "react-dom"] },
	},
	runtimeBootstrap: {
		createController: (bus, notify) => new ImperativePlayController(bus, notify),
		buildAppRuntime: buildImperativePlayAppRuntime,
	},
});
//#endregion 🔖Play
