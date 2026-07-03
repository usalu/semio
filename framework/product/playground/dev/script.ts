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
	scanPlaygroundAppManifests,
	type PlaygroundHostKind,
} from "../../../../repo/lib/js/index.ts";

const REPO_ROOT = join(import.meta.dir, "../../../..");
const PLAYGROUND_MANIFESTS = scanPlaygroundAppManifests(REPO_ROOT);

const ENTRY_TO_HOST: Readonly<Record<string, PlaygroundHostKind>> = Object.fromEntries(
	PLAYGROUND_MANIFESTS.flatMap((entry) => {
		const host = (entry.hostKind ?? entry.kind) as PlaygroundHostKind;
		return [[entry.kind, host], ...(entry.aliases ?? []).map((alias) => [alias, host] as const)];
	}),
) as Readonly<Record<string, PlaygroundHostKind>>;

const PACKAGE_ROOT_BY_ENTRY: Readonly<Record<string, string>> = Object.fromEntries(
	PLAYGROUND_MANIFESTS.flatMap((entry) => [
		[entry.kind, entry.packageRoot],
		...(entry.aliases ?? []).map((alias) => [alias, entry.packageRoot] as const),
	]),
) as Readonly<Record<string, string>>;

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
		if (!PACKAGE_ROOT_BY_ENTRY[appEntry]) throw new Error(`unknown playground app: ${appEntry}`);
		const hostKind = hostKindForEntry(appEntry);
		const playEntryKind = appEntry;
		const env = {
			...playPollingEnv(),
			PLAYGROUND_APP: appEntry,
			PLAYGROUND_APP_KIND: playEntryKind,
			PLAYGROUND_PACKAGE_ROOT: PACKAGE_ROOT_BY_ENTRY[appEntry] ?? "",
		};
		runViteBunxDev(this.root, viteArgs, {
			portEnv: playgroundPortEnv(hostKind),
			defaultPort: playgroundDevPortString(hostKind),
			fixedPort: true,
			env,
			expectedPlayEntry: playEntryKind,
		});
	}
}

class BuildScript extends BundleScript {
	async run(segments: string[]): Promise<void> {
		const { app: appEntry, viteArgs } = resolveAppArg(segments);
		runBun(["run", "vite", "build", "--config", "vite.config.ts", ...viteArgs], this.root, {
			...playPollingEnv(),
			PLAYGROUND_APP: appEntry,
			PLAYGROUND_APP_KIND: appEntry,
			PLAYGROUND_PACKAGE_ROOT: PACKAGE_ROOT_BY_ENTRY[appEntry] ?? "",
		});
	}
}

class TestScript extends BundleScript {
	run(segments: string[]): void {
		runVitest(this.root, segments, "vitest.config.ts");
	}
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
