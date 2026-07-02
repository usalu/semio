// #region 🧲Header
/** @emoji 🛝 GIS map playground harness — fixtures and dev-host wiring only. */
// #endregion 🧲Header

import {
	Platform,
	Playground,
	PLAYGROUND_NO_FIXTURE_ID,
	createProductPlaygroundPlatform,
	type PlaygroundAppDefinition,
} from "@semio-tech/framework-playground-core";
import reuseMapFixtureJson from "../fixture/reuse.map.gis.json";
import {
	GIS_MAP_PLAY_APP_ID,
	GIS_MAP_PLAY_BODY_KEY_MAIN,
	GIS_MAP_PLAY_CONTROLLER_ID,
	GIS_MAP_PLAY_FIXTURE_REUSE_ID,
	MapPlayController,
	buildMapPlayAppRuntime,
	buildMapPlayInspectorTree,
	gisMapFixtureToDescriptor,
	parseGisMapFixtureV1,
	registerMapPlayDeclarativeBodies,
	type GisMapFixtureV1,
	type GisMapPlayFixtureHostConfig,
} from "./index.ts";

export const GIS_MAP_PLAY_FIXTURE_OPTIONS = [{ id: GIS_MAP_PLAY_FIXTURE_REUSE_ID, label: "Reuse map" }] as const;

export const GIS_MAP_PLAY_DEFAULT_FIXTURE: GisMapFixtureV1 =
	parseGisMapFixtureV1(reuseMapFixtureJson as unknown) ?? (reuseMapFixtureJson as GisMapFixtureV1);

/** @emoji 📂 Builds GIS map playground fixture host config. */
export function createGisMapPlayFixtureHost(): GisMapPlayFixtureHostConfig {
	return {
		defaultFixture: GIS_MAP_PLAY_DEFAULT_FIXTURE,
		reuseFixtureId: GIS_MAP_PLAY_FIXTURE_REUSE_ID,
		options: [...GIS_MAP_PLAY_FIXTURE_OPTIONS],
	};
}

/** @emoji 🛝 Map playground app. */
export class PlaygroundMap extends Playground {
	readonly id = GIS_MAP_PLAY_APP_ID;

	createRuntime(): Platform {
		const runtime = createProductPlaygroundPlatform(this.id);
		const ctrl = new MapPlayController(runtime.commandBus, () => runtime.notify(), createGisMapPlayFixtureHost());
		runtime.addApp(buildMapPlayAppRuntime(ctrl));
		return runtime;
	}

	registerBodies(): void {
		registerMapPlayDeclarativeBodies();
	}
}

//#region 🔖PlaygroundAppDefinition
/** @emoji 🛝 Map playground app definition. */
export const gis2dPlayAppDefinition: PlaygroundAppDefinition = {
	id: GIS_MAP_PLAY_APP_ID,
	label: "Map",
	controllerId: GIS_MAP_PLAY_CONTROLLER_ID,
	modes: [{ id: "edit", label: "Edit" }],
	defaultModeId: "edit",
	createPlayground: () => new PlaygroundMap(),
	bootRenderer: async (pg) => {
		const { bootMapPlay } = await import("@semio-tech/framework-playground-renderer-react/gis/2d");
		bootMapPlay(pg);
	},
	devHost: {
		playEntryKind: "gis-2d",
		resolveDedupe: ["react", "react-dom", "@semio-tech/gis-2d-react", "three"],
		watchIgnored: ["../rs/lib.rs", "../rs/target/**", "../rs/Cargo.toml", "../rs/script.ts"],
		optimizeDeps: { include: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime", "@semio-tech/infinite-cavas-react-renderer", "@semio-tech/gis-2d-react"] },
	},
};
//#endregion 🔖PlaygroundAppDefinition

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("MapPlayController fixtures", () => {
		it("loads the reuse fixture by default", () => {
			const runtime = new Platform({ id: "test" });
			const ctrl = new MapPlayController(runtime.commandBus, () => runtime.notify(), createGisMapPlayFixtureHost());
			expect(ctrl.getFixtureCatalog().activeFixtureId).toBe(GIS_MAP_PLAY_FIXTURE_REUSE_ID);
			expect(ctrl.getSnapshot().activeFixture?.schema).toBe("gis.map.fixture/v1");
			expect(ctrl.getSnapshot().activeFixture?.positions.length).toBeGreaterThan(0);
			expect(ctrl.getSnapshot().activeFixture?.routes.length).toBeGreaterThan(0);
		});

		it("clears fixture overlays when No fixture is selected", () => {
			const runtime = new Platform({ id: "test" });
			const ctrl = new MapPlayController(runtime.commandBus, () => runtime.notify(), createGisMapPlayFixtureHost());
			ctrl.run("setActiveFixture", { fixtureId: PLAYGROUND_NO_FIXTURE_ID });
			expect(ctrl.getSnapshot().activeFixture).toBeNull();
		});

		it("maps fixture positions into a map descriptor", () => {
			const descriptor = gisMapFixtureToDescriptor(GIS_MAP_PLAY_DEFAULT_FIXTURE);
			expect(descriptor.positions[0]?.sourceUrl).toBeTruthy();
			expect(descriptor.routes.length).toBe(GIS_MAP_PLAY_DEFAULT_FIXTURE.routes.length);
		});

		it("buildMapPlayInspectorTree batches position fields for multi-select", () => {
			const fixture = GIS_MAP_PLAY_DEFAULT_FIXTURE;
			const positionIds = fixture.positions.slice(0, 2).map((row) => row.id);
			const tree = buildMapPlayInspectorTree(fixture, positionIds, []);
			expect(tree.type).toBe("tree");
			const latField = tree.sections[0]?.items.find((item) => item.id === "gis-map-play-inspector.position.lat");
			expect(latField?.control?.type).toBe("input");
			expect(latField?.control?.onChange?.command).toBe("patchPositions");
			expect(latField?.control?.onChange?.args).toMatchObject({ positionIds, field: "lat" });
		});

		it("patchPositions updates every selected position", () => {
			const runtime = new Platform({ id: "test" });
			const ctrl = new MapPlayController(runtime.commandBus, () => runtime.notify(), createGisMapPlayFixtureHost());
			const fixture = ctrl.getActiveFixture();
			expect(fixture).not.toBeNull();
			const positionIds = fixture!.positions.slice(0, 2).map((row) => row.id);
			ctrl.run("patchPositions", { positionIds, field: "label", value: "batch-label" });
			const updated = ctrl.getActiveFixture()!;
			for (const positionId of positionIds) {
				expect(updated.positions.find((row) => row.id === positionId)?.label).toBe("batch-label");
			}
		});
	});
}
// #endregion 🧪Tests
