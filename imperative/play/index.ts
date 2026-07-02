// #region 🧲Header
/** @emoji ⚙️ Imperative play — step-list editor playground. */
// #endregion 🧲Header

import {
	AppRuntime,
	CommandBus,
	Controller,
	ModeRuntime,
	Platform,
	Playground,
	WindowKindRuntime,
	buildImperativeWindowBody,
	createPlayAppRuntime,
	createProductPlaygroundPlatform,
	createStackLayout,
	registerWindowBody,
	type CommandDescriptor,
	type UiNode,
	type WindowBodyViewContext,
} from "@semio-tech/framework-playground-core";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import { DEFAULT_IMPERATIVE_DOCUMENT, imperativeDocumentToJson } from "@semio-tech/imperative-core";

export const IMPERATIVE_PLAY_APP_ID = "imperative-play";
export const IMPERATIVE_PLAY_CONTROLLER_ID = "imperative-play";
export const IMPERATIVE_PLAY_SURFACE_ID = "imperative.play/v1";
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

export function registerImperativePlayDeclarativeBodies(): void {
	registerWindowBody(IMPERATIVE_PLAY_BODY_KEY_MAIN, buildImperativePlayMainDeclarativeBody);
}

export function buildImperativePlayAppRuntime(controller: ImperativePlayController): AppRuntime {
	return createPlayAppRuntime(IMPERATIVE_PLAY_APP_ID, "Imperative", controller, IMPERATIVE_PLAY_LAYOUT, controller.mainMode);
}

/** @emoji 🛝 Imperative playground app. */
export class PlaygroundImperative extends Playground {
	readonly id = IMPERATIVE_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new ImperativePlayController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildImperativePlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerImperativePlayDeclarativeBodies();
	}
}

export { imperativePlayCmd };

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("ImperativePlayController", () => {
		it("default document json is valid", () => {
			expect(IMPERATIVE_PLAY_DEFAULT_DOCUMENT_JSON).toContain("imperative.document/v1");
			expect(IMPERATIVE_PLAY_DEFAULT_DOCUMENT_JSON).toContain("step-1");
		});
	});
}

//#region 🔖SExtension
import { baselineSingleAppPlatformDefinition, type PlatformDefinition } from "@semio-tech/framework-platform-core";

/** @emoji 🧩 S program definition for imperative. */
export function buildImperativeProgramDefinition(): PlatformDefinition {
	return baselineSingleAppPlatformDefinition("imperative", "Imperative", "imperative", "Imperative", IMPERATIVE_PLAY_CONTROLLER_ID);
}
//#endregion 🔖SExtension

if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest && import.meta.env.PUZZLE_PLAY_ENTRY === "imperative") {
	bootstrapElementsSurfaceChromeDocument("system");
	void (async () => {
		await import("./globals.css");
		const { bootImperativePlay } = await import("@semio-tech/framework-playground-renderer-react/imperative");
		bootImperativePlay(new PlaygroundImperative());
	})();
}
