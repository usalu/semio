// #region 🧲Header
/** @emoji 🛝 WIRES playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import { Playground2d } from "@semio-tech/puzzle-2d-core";
import { WIRES_PLAY_APP_ID, WIRES_PLAY_CONTROLLER_ID } from "./index.ts";
import type { PlaygroundAppDefinition } from "@semio-tech/framework-playground-core";

export { Playground2d as PlaygroundWires };

//#region 🔖PlaygroundAppDefinition
/** @emoji 🛝 Wires playground app definition. */
export const wiresPlayAppDefinition: PlaygroundAppDefinition = {
	id: WIRES_PLAY_APP_ID,
	label: "Wires",
	controllerId: WIRES_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new Playground2d(),
	bootRenderer: async (pg) => {
		const { bootWiresPlay } = await import("@semio-tech/framework-playground-renderer-react/reasoning/wires");
		bootWiresPlay(pg);
	},
	devHost: {
		playEntryKind: "wires",
		resolveDedupe: ["react", "react-dom", "three", "@semio-tech/puzzle-2d-react", "@semio-tech/reasoning-mindmap-wires-react"],
		watchIgnored: ["../../../../puzzle/2d/rs/lib.rs",
    "../../../../puzzle/2d/rs/target/**",
    "../../../../puzzle/2d/rs/Cargo.toml",
    "../../../../puzzle/2d/rs/script.ts",],
		optimizeDeps: { include: [
      "react",
      "react-dom",
      "react/jsx-runtime",
      "react/jsx-dev-runtime",
      "three",
      "@react-three/fiber",
      "@react-three/drei",
      "lucide-react",
      "@semio-tech/infinite-cavas-react-renderer",
      "@semio-tech/puzzle-2d-react",
      "@semio-tech/reasoning-mindmap-react",
    ] },
	},
};
//#endregion 🔖PlaygroundAppDefinition
