// #region 🧲Header
/** @emoji 🧊 Trunk boot glue — loads wasm plugins and starts the wgpu renderer. */
// #endregion 🧲Header

import { PLUGIN_TARGETS } from "../../../plugin/registry/generated/plugins.ts";

declare const DEFAULT_PLUGIN_FILTER: string;

await new Promise<void>((resolve) => {
	if (document.readyState === "loading") {
		document.addEventListener("DOMContentLoaded", () => resolve(), { once: true });
	} else {
		resolve();
	}
});

async function loadPluginModule(pluginId: string, moduleUrl: string) {
	const module = (await import(/* @vite-ignore */ moduleUrl)) as {
		default?: () => Promise<void> | void;
		semio_plugin_manifest?: () => string;
		semio_plugin_create_app?: (appId: string) => number;
		semio_plugin_destroy_app?: (instanceId: number) => void;
		semio_plugin_handle_command?: (instanceId: number, commandJson: string, viewStateJson: string) => string;
		semio_plugin_render?: (instanceId: number, bodyKey: string, viewStateJson: string) => string;
		semio_plugin_tools?: (instanceId: number, viewStateJson: string) => string;
		semio_plugin_window_engagements?: (instanceId: number, viewStateJson: string) => string;
		semio_plugin_window_measures?: (instanceId: number, viewStateJson: string) => string;
	};
	if (module.default) await module.default();
	if (!module.semio_plugin_manifest) {
		throw new Error(`[DEBUG] plugin ${pluginId} missing semio_plugin_manifest export`);
	}
	const manifest = JSON.parse(module.semio_plugin_manifest());
	return {
		pluginId,
		manifest,
		createApp: async (appId: string) => {
			const create = module.semio_plugin_create_app;
			if (!create) throw new Error(`plugin ${pluginId} missing create_app`);
			return create(appId);
		},
		destroyApp: async (instanceId: number) => {
			module.semio_plugin_destroy_app?.(instanceId);
		},
		handleCommand: async (instanceId: number, commandJson: string, viewState: unknown) => {
			const handle = module.semio_plugin_handle_command;
			if (!handle) return [];
			return JSON.parse(handle(instanceId, commandJson, JSON.stringify(viewState)));
		},
		render: async (instanceId: number, bodyKey: string, viewState: unknown) => {
			const render = module.semio_plugin_render;
			if (!render) throw new Error(`plugin ${pluginId} missing render`);
			return JSON.parse(render(instanceId, bodyKey, JSON.stringify(viewState)));
		},
		tools: async (instanceId: number, viewState: unknown) => {
			const tools = module.semio_plugin_tools;
			if (!tools) return [];
			return JSON.parse(tools(instanceId, JSON.stringify(viewState)));
		},
		windowEngagements: async (instanceId: number, viewState: unknown) => {
			const engagements = module.semio_plugin_window_engagements;
			if (!engagements) return {};
			return JSON.parse(engagements(instanceId, JSON.stringify(viewState)));
		},
		windowMeasures: async (instanceId: number, viewState: unknown) => {
			const measures = module.semio_plugin_window_measures;
			if (!measures) return {};
			return JSON.parse(measures(instanceId, JSON.stringify(viewState)));
		},
	};
}

function pluginHandleForBridge(handle: Awaited<ReturnType<typeof loadPluginModule>>) {
	return {
		manifest: () => JSON.stringify(handle.manifest),
		createApp: (appId: string) => handle.createApp(appId),
		destroyApp: (instanceId: number) => handle.destroyApp(instanceId),
		handleCommand: (instanceId: number, commandJson: string, viewStateJson: string) =>
			handle.handleCommand(instanceId, commandJson, JSON.parse(viewStateJson)).then((ops) => JSON.stringify(ops)),
		render: (instanceId: number, bodyKey: string, viewStateJson: string) =>
			handle.render(instanceId, bodyKey, JSON.parse(viewStateJson)).then((node) => JSON.stringify(node)),
		tools: (instanceId: number, viewStateJson: string) =>
			handle.tools(instanceId, JSON.parse(viewStateJson)).then((nodes) => JSON.stringify(nodes)),
		windowEngagements: (instanceId: number, viewStateJson: string) =>
			handle.windowEngagements(instanceId, JSON.parse(viewStateJson)).then((engagements) => JSON.stringify(engagements)),
		windowMeasures: (instanceId: number, viewStateJson: string) =>
			handle.windowMeasures(instanceId, JSON.parse(viewStateJson)).then((measures) => JSON.stringify(measures)),
	};
}

const pluginFromUrl = new URLSearchParams(location.search).get("plugin");
const pluginFilter = pluginFromUrl ?? DEFAULT_PLUGIN_FILTER;
const studioMode = pluginFilter === "s";
const pluginTargets = studioMode
	? PLUGIN_TARGETS
	: PLUGIN_TARGETS.filter((entry) => entry.pluginId === pluginFilter);

async function pluginModuleAvailable(moduleUrl: string): Promise<boolean> {
	try {
		const response = await fetch(moduleUrl, { method: "HEAD" });
		return response.ok;
	} catch {
		return false;
	}
}

const availableTargets: (typeof PLUGIN_TARGETS)[number][] = [];
for (const entry of pluginTargets) {
	if (await pluginModuleAvailable(entry.moduleUrl)) {
		availableTargets.push(entry);
	}
}
if (availableTargets.length === 0) {
	throw new Error(`[DEBUG] no wasm plugin modules found for filter ${pluginFilter}`);
}

const handles = await Promise.all(
	availableTargets.map(async (entry) => ({
		pluginId: entry.pluginId,
		handle: pluginHandleForBridge(await loadPluginModule(entry.pluginId, entry.moduleUrl)),
	})),
);

const bindings = await new Promise<Record<string, unknown>>((resolve, reject) => {
	const host = window as { wasmBindings?: Record<string, unknown> };
	const finish = () => {
		if (!host.wasmBindings) {
			reject(new Error("[DEBUG] trunk wasm bindings missing"));
			return;
		}
		resolve(host.wasmBindings);
	};
	if (host.wasmBindings) {
		finish();
		return;
	}
	const timeout = window.setTimeout(() => reject(new Error("[DEBUG] trunk wasm bindings timeout")), 30000);
	const done = () => {
		window.clearTimeout(timeout);
		window.clearInterval(poll);
		finish();
	};
	window.addEventListener("TrunkApplicationStarted", done, { once: true });
	const poll = window.setInterval(() => {
		if (host.wasmBindings) done();
	}, 50);
});

if (!bindings.semioRendererBoot) throw new Error("[DEBUG] missing semioRendererBoot");
await (bindings.semioRendererBoot as (handles: typeof handles, pluginFilter: string) => Promise<void>)(
	handles,
	pluginFilter,
);
