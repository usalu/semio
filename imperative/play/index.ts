// #region 🧲Header
/** @emoji ⚙️ Imperative play — step-list editor playground. */
// #endregion 🧲Header

import {
	AppRuntime,
	CommandBus,
	Controller,
	Platform,
	Playground,
	createPlayAppRuntime,
	createProductPlaygroundPlatform,
	createStackLayout,
	registerWindowBody,
	type CommandDescriptor,
	type WindowBodyViewContext,
} from "@semio-tech/framework-playground-core";
import { bootstrapElementsSurfaceChromeDocument } from "@semio-tech/ui-react";
import { DEFAULT_IMPERATIVE_DOCUMENT, imperativeDocumentToJson, type ImperativeDocumentV1 } from "@semio-tech/imperative-core";
import { ImperativeEditor } from "@semio-tech/imperative-react";

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
	private documentJson = IMPERATIVE_PLAY_DEFAULT_DOCUMENT_JSON;

	getDocumentJson(): string {
		return this.documentJson;
	}

	setDocumentJson(json: string): void {
		this.documentJson = json;
		this.notify();
	}

	override run(command: string, args?: Record<string, unknown>): void {
		if (command === "setDocumentJson" && typeof args?.json === "string") {
			this.setDocumentJson(args.json);
		}
	}
}

function buildImperativePlayMainBody(ctx: WindowBodyViewContext): ReturnType<typeof ImperativeEditor> {
	const ctrl = ctx.runtime.controller<ImperativePlayController>(IMPERATIVE_PLAY_CONTROLLER_ID);
	return (
		<ImperativeEditor
			className="h-full min-h-0"
			documentJson={ctrl.getDocumentJson()}
			onDocumentChange={(json) => ctrl.setDocumentJson(json)}
		/>
	);
}

export function registerImperativePlayDeclarativeBodies(): void {
	registerWindowBody(IMPERATIVE_PLAY_BODY_KEY_MAIN, buildImperativePlayMainBody);
}

function buildImperativePlayAppRuntime(ctrl: ImperativePlayController): AppRuntime {
	return createPlayAppRuntime({
		appId: IMPERATIVE_PLAY_APP_ID,
		title: "Imperative",
		layout: IMPERATIVE_PLAY_LAYOUT,
		windowKinds: [{ id: IMPERATIVE_PLAY_WINDOW_KIND_ID, label: "Imperative", bodyKey: IMPERATIVE_PLAY_BODY_KEY_MAIN }],
		surfaceHosts: [{ id: IMPERATIVE_PLAY_SURFACE_ID, type: "imperative" }],
	});
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
		it("stores document json", () => {
			const bus = new CommandBus();
			const ctrl = new ImperativePlayController(bus, () => {});
			ctrl.run("setDocumentJson", { json: IMPERATIVE_PLAY_DEFAULT_DOCUMENT_JSON });
			expect(ctrl.getDocumentJson()).toContain("imperative.document/v1");
		});
	});
}

if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest && import.meta.env.PUZZLE_PLAY_ENTRY === "imperative") {
	bootstrapElementsSurfaceChromeDocument("system");
	void (async () => {
		await import("./globals.css");
		const { bootImperativePlay } = await import("@semio-tech/framework-playground-renderer-react/imperative");
		bootImperativePlay(new PlaygroundImperative());
	})();
}
