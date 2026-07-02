// #region 🧲Header
/** @emoji 🛝 Puzzle 3D playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import nakaginPuzzle3dFixtureJson from "../fixture/nakagin-capsule-tower.3d.json";
import concreteForestPuzzle3dFixtureJson from "../fixture/concrete-forest.3d.json";
import { parseFixture, type Fixture } from "../react/index.tsx";
import { Playground, type PlaygroundAppDefinition, type Platform } from "@semio-tech/framework-playground-core";
import {
  PUZZLE_3D_PLAY_APP_ID,
  PUZZLE_3D_PLAY_CONTROLLER_ID,
  buildPuzzle3dPlayRuntime,
  registerPuzzle3dPlayDeclarativeBodies,
} from "./index.ts";

export const PUZZLE_3D_PLAY_FIXTURE_NAKAGIN_ID = "nakagin";
export const PUZZLE_3D_PLAY_FIXTURE_CONCRETE_FOREST_ID = "concrete-forest";

export const PUZZLE_3D_PLAY_FIXTURE_OPTIONS = [
  { id: PUZZLE_3D_PLAY_FIXTURE_CONCRETE_FOREST_ID, label: "Concrete Forest" },
  { id: PUZZLE_3D_PLAY_FIXTURE_NAKAGIN_ID, label: "Nakagin capsule tower" },
] as const;

/** @emoji 🔒 Resolves a playground fixture slug (e.g. `concrete`) to a puzzle 3d fixture id. */
export function resolvePuzzle3dPlayFixtureSlug(slug: string): string | undefined {
  const aliases: Record<string, string> = { concrete: PUZZLE_3D_PLAY_FIXTURE_CONCRETE_FOREST_ID };
  const normalized = aliases[slug] ?? slug;
  return PUZZLE_3D_PLAY_FIXTURE_OPTIONS.some((row) => row.id === normalized) ? normalized : undefined;
}

export const PUZZLE_3D_PLAY_FIXTURE_JSON_BY_ID: Record<string, unknown> = {
  [PUZZLE_3D_PLAY_FIXTURE_NAKAGIN_ID]: nakaginPuzzle3dFixtureJson,
  [PUZZLE_3D_PLAY_FIXTURE_CONCRETE_FOREST_ID]: concreteForestPuzzle3dFixtureJson,
};

/** @emoji 🧪 Resolves imported puzzle 3d fixture JSON by catalog id. */
export function puzzle3dPlayFixtureJson(fixtureId: string = PUZZLE_3D_PLAY_FIXTURE_CONCRETE_FOREST_ID): unknown {
  return PUZZLE_3D_PLAY_FIXTURE_JSON_BY_ID[fixtureId] ?? concreteForestPuzzle3dFixtureJson;
}

/** @emoji 📄 Serializes a puzzle 3d fixture for Jack bridges. */
export function puzzle3dFixtureToJson(fixture: Fixture): string {
  return JSON.stringify(fixture);
}

/** @emoji 🃏 Normalizes a puzzle 3d fixture into board-shaped JSON for Jack queries. */
export function puzzle3dFixtureToJackBoardJson(fixtureOrJson: Fixture | string): string {
  const fixture =
    typeof fixtureOrJson === "string"
      ? (parseFixture(JSON.parse(fixtureOrJson) as unknown) ?? ({ objects: [] } as Fixture))
      : fixtureOrJson;
  return JSON.stringify({
    nodes: fixture.objects.map((object) => ({
      id: object.id,
      nodeKind: "Object",
      text: object.label ?? object.id,
    })),
    edges: [],
  });
}
/** @emoji 🛝 Puzzle 3D play harness as a single {@link Playground} instance. */
export class Playground3d extends Playground {
  readonly id = PUZZLE_3D_PLAY_APP_ID;
  readonly keybindings = [
    { key: "ctrl+a,meta+a", controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "selectAllSelection" },
    { key: "Delete", controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "deleteSelection" },
    { key: "Backspace", controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID, command: "deleteSelection" },
  ];

  createRuntime(): Platform {
    return buildPuzzle3dPlayRuntime();
  }

  registerBodies(): void {
    registerPuzzle3dPlayDeclarativeBodies();
  }

}

//#region 🔖PlaygroundAppDefinition
/** @emoji 🛝 Puzzle 3D playground app definition. */
export const puzzle3dPlayAppDefinition: PlaygroundAppDefinition = {
	id: PUZZLE_3D_PLAY_APP_ID,
	label: "Puzzle 3D",
	controllerId: PUZZLE_3D_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new Playground3d(),
	bootRenderer: async (pg) => {
		const { bootPuzzle3dPlay } = await import("@semio-tech/framework-playground-renderer-react/puzzle/3d");
		bootPuzzle3dPlay(pg);
	},
	devHost: {
		playEntryKind: "3d",
		resolveDedupe: ["react", "react-dom", "three", "@semio-tech/puzzle-3d-react"],
		optimizeDeps: { include: [
			"react",
			"react-dom",
			"react/jsx-runtime",
			"react/jsx-dev-runtime",
			"three",
			"@react-three/fiber",
			"@react-three/drei",
			"lucide-react",
			"@semio-tech/infinite-world-r3f",
			"@semio-tech/infinite-cavas-react-renderer",
			"@semio-tech/puzzle-3d-react",
		] },
	},
};
//#endregion 🔖PlaygroundAppDefinition
