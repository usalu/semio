// #region 🧲Header
/** @emoji 🛝 CAD playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import { Platform, Playground, createProductPlaygroundPlatform, type PlaygroundAppDefinition } from "@semio-tech/framework-playground-core";
import geometryConcreteForestLeft from "../../../asset/play/hexagonal-cut-concrete-forest-left.model.json";
import geometryConcreteForestRight from "../../../asset/play/hexagonal-cut-concrete-forest-right.model.json";
import { defaultModelDefinitionId } from "@semio-tech/cad-js-core";
import {
	CAD_PLAY_APP_ID,
	CAD_PLAY_CONTROLLER_ID,
	CadPlayShellController,
	buildCadPlayAppRuntime,
	registerCadPlayDeclarativeBodies,
} from "./index.ts";
import { CAD_PLAY_SHAPE_ASSET_IDS, resolveCadPlayFixtureSlug } from "./fixture-slugs.ts";

export { CAD_PLAY_SHAPE_ASSET_IDS, resolveCadPlayFixtureSlug } from "./fixture-slugs.ts";

/** @emoji 🧩 Shape fixture assets for CAD play navbar. */
export const CAD_PLAY_SHAPE_ASSETS = [
	{ id: "concrete-forest-left", key: "c", label: "Concrete forest (left)", json: { modelSpace: geometryConcreteForestLeft, activeModelDefinitionId: defaultModelDefinitionId() } as Record<string, unknown> },
	{ id: "concrete-forest-right", key: "d", label: "Concrete forest (right)", json: geometryConcreteForestRight as Record<string, unknown> },
] as const;

/** @emoji 🛝 CAD playground app. */
export class PlaygroundCad extends Playground {
	readonly id = CAD_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		registerCadPlayDeclarativeBodies();
		const controller = new CadPlayShellController(runtime.commandBus, () => runtime.notify());
		runtime.addApp(buildCadPlayAppRuntime(controller));
		return runtime;
	}

	registerBodies(): void {
		registerCadPlayDeclarativeBodies();
	}
}

//#region 🔖PlaygroundAppDefinition

/** @emoji 🛝 CAD playground app definition. */
export const cadPlayAppDefinition: PlaygroundAppDefinition = {
	id: CAD_PLAY_APP_ID,
	label: "CAD",
	controllerId: CAD_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new PlaygroundCad(),
	bootRenderer: async (pg) => {
		const { bootCadPlay } = await import("@semio-tech/framework-playground-renderer-react/cad");
		bootCadPlay(pg);
	},
	devHost: {
		playEntryKind: "cad",
		resolveDedupe: ["react", "react-dom", "three"],
		optimizeDeps: {
			include: [
				"react",
				"react-dom",
				"react/jsx-runtime",
				"react/jsx-dev-runtime",
				"three",
				"@react-three/fiber",
				"@react-three/drei",
				"@semio-tech/infinite-world-r3f",
				"brepjs",
				"brepjs-opencascade",
				"golden-layout",
				"lucide-react",
				"chevrotain",
			],
		},
	},
};
//#endregion 🔖PlaygroundAppDefinition
