// #region 🧲Header
/** @emoji 🔌 Canonical dev/test ports and env vars for playground Vite hosts (launch.json + nx + scripts). */
// #endregion 🧲Header

//#region 🔖PlaygroundDevPorts
export type PlaygroundHostKind =
	| "storybook"
	| "compose"
	| "puzzle-2d"
	| "puzzle-3d"
	| "puzzle-5d"
	| "wires"
	| "flow"
	| "dag"
	| "procedural-3d"
	| "procedural-2d"
	| "shooting"
	| "cad"
	| "gis-map"
	| "projektetage"
	| "presentation";

type PlaygroundPortSpec = {
	readonly dev: number;
	readonly test?: number;
	readonly env: string;
};

/** Dev ports 6012–6020 are puzzle/graph/cad plays; 6040+ are map/presentation; test ports sit in 6027–6052. */
export const PLAYGROUND_PORTS: Record<PlaygroundHostKind, PlaygroundPortSpec> = {
	storybook: { dev: 6010, env: "STORYBOOK_PORT" },
	compose: { dev: 4000, env: "COMPOSE_PLAY_PORT" },
	"puzzle-2d": { dev: 6012, test: 6027, env: "PUZZLE_2D_PLAY_PORT" },
	"puzzle-3d": { dev: 6013, test: 6028, env: "PUZZLE_3D_PLAY_PORT" },
	"puzzle-5d": { dev: 6014, test: 6035, env: "PUZZLE_5D_PLAY_PORT" },
	wires: { dev: 6015, env: "WIRES_PLAY_PORT" },
	flow: { dev: 6016, test: 6029, env: "FLOW_PLAY_PORT" },
	dag: { dev: 6017, test: 6030, env: "DAG_PLAY_PORT" },
	"procedural-3d": { dev: 6018, test: 6031, env: "PROCEDURAL_3D_PLAY_PORT" },
	"procedural-2d": { dev: 6021, test: 6033, env: "PROCEDURAL_2D_PLAY_PORT" },
	shooting: { dev: 6019, test: 6032, env: "SHOOTING_PLAY_PORT" },
	cad: { dev: 6020, test: 6041, env: "CAD_JS_RENDERER_PLAY_PORT" },
	"gis-map": { dev: 6040, env: "GIS_MAP_PLAY_PORT" },
	projektetage: { dev: 6050, env: "PRAESENTATION_PROJEKTETAGE_PORT" },
	presentation: { dev: 6051, test: 6052, env: "PRESENTATION_PLAY_PORT" },
};

/** @emoji 🔌 Local dev port for a playground host. */
export function playgroundDevPort(kind: PlaygroundHostKind): number {
	return PLAYGROUND_PORTS[kind].dev;
}

/** @emoji 🔌 String dev port (vite `--port`, nx `env`). */
export function playgroundDevPortString(kind: PlaygroundHostKind): string {
	return String(playgroundDevPort(kind));
}

/** @emoji 🧪 Vitest/playwright port when set; otherwise `undefined`. */
export function playgroundTestPort(kind: PlaygroundHostKind): number | undefined {
	return PLAYGROUND_PORTS[kind].test;
}

/** @emoji 🧪 String test port for nx `env` / playwright. */
export function playgroundTestPortString(kind: PlaygroundHostKind): string | undefined {
	const port = playgroundTestPort(kind);
	return port === undefined ? undefined : String(port);
}

/** @emoji 🔌 Process env var holding the dev port override. */
export function playgroundPortEnv(kind: PlaygroundHostKind): string {
	return PLAYGROUND_PORTS[kind].env;
}

/** @emoji 🚧 Every assigned playground dev + test port (for strict binding). */
export function allPlaygroundReservedPorts(): ReadonlySet<number> {
	const ports = new Set<number>();
	for (const spec of Object.values(PLAYGROUND_PORTS)) {
		ports.add(spec.dev);
		if (spec.test !== undefined) ports.add(spec.test);
	}
	return ports;
}

/** @emoji 🌐 Subset used by iframe static-site embed URLs (`compose`, `cad`, puzzle dims). */
export const PLAYGROUND_EMBED_SITE_DEV_PORTS = {
	compose: playgroundDevPortString("compose"),
	cad: playgroundDevPortString("cad"),
	"2d": playgroundDevPortString("puzzle-2d"),
	"3d": playgroundDevPortString("puzzle-3d"),
	"5d": playgroundDevPortString("puzzle-5d"),
} as const;

export type PlaygroundEmbedSiteKind = keyof typeof PLAYGROUND_EMBED_SITE_DEV_PORTS;

/** @emoji 🔒 Process env var locking a playground to one fixture (hides navbar dropdown). */
export const PLAYGROUND_LOCKED_FIXTURE_ENV = "PLAYGROUND_LOCKED_FIXTURE_ID";

/** @emoji 🔒 Locked fixture id from process env, if any. */
export function playgroundLockedFixtureIdFromEnv(env: NodeJS.ProcessEnv = process.env): string | undefined {
	const raw = env[PLAYGROUND_LOCKED_FIXTURE_ENV]?.trim();
	return raw || undefined;
}

/** @emoji 🔌 Vite `define` entries for playground play bundles. */
export function playgroundPlayViteDefine(extra: Record<string, string> = {}): Record<string, string> {
	return {
		"import.meta.env.PLAYGROUND_LOCKED_FIXTURE_ID": JSON.stringify(playgroundLockedFixtureIdFromEnv() ?? ""),
		"import.meta.vitest": "undefined",
		...extra,
	};
}
//#endregion 🔖PlaygroundDevPorts

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("playgroundDevPorts", () => {
		it("assigns a unique port per dev and test slot", () => {
			const seen = new Set<number>();
			for (const spec of Object.values(PLAYGROUND_PORTS)) {
				expect(seen.has(spec.dev)).toBe(false);
				seen.add(spec.dev);
				if (spec.test !== undefined) {
					expect(seen.has(spec.test)).toBe(false);
					seen.add(spec.test);
				}
			}
		});

		it("keeps cad and dag on distinct dev ports", () => {
			expect(playgroundDevPort("cad")).toBe(6020);
			expect(playgroundDevPort("dag")).toBe(6017);
		});

		it("playgroundPlayViteDefine embeds locked fixture env", () => {
			const prev = process.env[PLAYGROUND_LOCKED_FIXTURE_ENV];
			try {
				delete process.env[PLAYGROUND_LOCKED_FIXTURE_ENV];
				expect(playgroundPlayViteDefine()["import.meta.env.PLAYGROUND_LOCKED_FIXTURE_ID"]).toBe('""');
				process.env[PLAYGROUND_LOCKED_FIXTURE_ENV] = "concrete-forest";
				expect(playgroundPlayViteDefine()["import.meta.env.PLAYGROUND_LOCKED_FIXTURE_ID"]).toBe('"concrete-forest"');
			} finally {
				if (prev === undefined) delete process.env[PLAYGROUND_LOCKED_FIXTURE_ENV];
				else process.env[PLAYGROUND_LOCKED_FIXTURE_ENV] = prev;
			}
		});
	});
}
