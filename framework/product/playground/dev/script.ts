#!/usr/bin/env bun
/** 🧭 `@semio-tech/framework-playground-dev` task router. */
import { join } from "node:path";
import {
	BundleScript,
	ScriptRouter,
	playPollingEnv,
	playgroundDevPortString,
	playgroundPortEnv,
	runBun,
	runBundleScriptMain,
	runVitest,
	runViteBunxDev,
	type PlaygroundHostKind,
} from "../../../../repo/lib/js/index.ts";
import { loadPlaygroundApp } from "@semio-tech/framework-playground-core/app-registry";

const ENTRY_TO_HOST: Readonly<Record<string, PlaygroundHostKind>> = {
	"2d": "puzzle-2d",
	"3d": "puzzle-3d",
	"5d": "puzzle-5d",
	"gis-2d": "gis-2d",
	wires: "wires",
};

const PACKAGE_ROOT_BY_ENTRY: Readonly<Record<string, string>> = {
	draw: "draw",
	writer: "writer",
	raster: "raster",
	forms: "forms",
	flow: "flow",
	dag: "mathematical/graph/port/directed/dag",
	imperative: "imperative",
	sequence: "sequence",
	layout: "layout",
	lowpoly: "lowpoly",
	"procedural-2d": "procedural/2d",
	"procedural-3d": "procedural/3d",
	shooting: "shooting",
	s: "s",
	vcs: "vcs",
	"gis-2d": "gis/2d",
	wires: "reasoning/mindmap/wires",
	"trinity-jack": "trinity/jack/host-core",
	"trinity-rewrite": "trinity/rewrite",
	presentation: "framework/product/presentation",
	cad: "cad/js/renderer",
	"2d": "puzzle/2d",
	"3d": "puzzle/3d",
	"5d": "puzzle/5d",
};

function resolveAppArg(segments: string[]): { readonly app: string; readonly viteArgs: string[] } {
	const flag = segments.findIndex((segment) => segment === "--app");
	if (flag >= 0 && segments[flag + 1]) {
		const app = segments[flag + 1]!;
		const viteArgs = [...segments.slice(0, flag), ...segments.slice(flag + 2)];
		return { app, viteArgs };
	}
	const app = segments[0] ?? process.env.PLAYGROUND_APP ?? "draw";
	return { app, viteArgs: segments[0] === app ? segments.slice(1) : segments };
}

function hostKindForEntry(entry: string): PlaygroundHostKind {
	return ENTRY_TO_HOST[entry] ?? (entry as PlaygroundHostKind);
}

class DevScript extends BundleScript {
	async run(segments: string[]): Promise<void> {
		const { app: appEntry, viteArgs } = resolveAppArg(segments);
		const app = await loadPlaygroundApp(appEntry);
		if (!app) throw new Error(`unknown playground app: ${appEntry}`);
		const prebuild = app.devHost?.prebuild;
		if (prebuild) await prebuild(this.root);
		const hostKind = hostKindForEntry(appEntry);
		const env = {
			...playPollingEnv(),
			PLAYGROUND_APP: appEntry,
			PUZZLE_PLAY_ENTRY: app.devHost?.playEntryKind ?? appEntry,
			PLAYGROUND_PACKAGE_ROOT: PACKAGE_ROOT_BY_ENTRY[appEntry] ?? "",
		};
		runViteBunxDev(this.root, viteArgs, {
			portEnv: playgroundPortEnv(hostKind),
			defaultPort: playgroundDevPortString(hostKind),
			fixedPort: true,
			env,
		});
	}
}

class BuildScript extends BundleScript {
	async run(segments: string[]): Promise<void> {
		const { app: appEntry, viteArgs } = resolveAppArg(segments);
		runBun(["run", "vite", "build", "--config", "vite.config.ts", ...viteArgs], this.root, {
			...playPollingEnv(),
			PLAYGROUND_APP: appEntry,
			PUZZLE_PLAY_ENTRY: appEntry,
			PLAYGROUND_PACKAGE_ROOT: PACKAGE_ROOT_BY_ENTRY[appEntry] ?? "",
		});
	}
}

class TestScript extends BundleScript {
	run(segments: string[]): void {
		runVitest(this.root, segments);
	}
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
