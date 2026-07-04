import type { PluginManifest, UiNode, ViewState } from "./types.ts";

export type PluginWasmHandle = {
	readonly pluginId: string;
	readonly manifest: PluginManifest;
	readonly createApp: (appId: string) => Promise<number>;
	readonly destroyApp: (instanceId: number) => Promise<void>;
	readonly handleCommand: (instanceId: number, commandJson: string, viewState: ViewState) => Promise<string[]>;
	readonly render: (instanceId: number, bodyKey: string, viewState: ViewState) => Promise<UiNode>;
	readonly dispose: () => void;
};

export type PluginRegistryEntry = {
	readonly pluginId: string;
	readonly moduleUrl: string;
};

export const DEFAULT_PLUGIN_REGISTRY: readonly PluginRegistryEntry[] = [
	{ pluginId: "draw", moduleUrl: "/plugin-modules/draw/draw_plugin.js" },
];

export async function loadPluginModule(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
	const module = (await import(/* @vite-ignore */ moduleUrl)) as {
		default?: () => Promise<void> | void;
		semio_plugin_manifest?: () => string;
		semio_plugin_create_app?: (appId: string) => number;
		semio_plugin_destroy_app?: (instanceId: number) => void;
		semio_plugin_handle_command?: (instanceId: number, commandJson: string, viewStateJson: string) => string;
		semio_plugin_render?: (instanceId: number, bodyKey: string, viewStateJson: string) => string;
	};
	if (module.default) await module.default();
	if (!module.semio_plugin_manifest) {
		throw new Error(`[DEBUG] plugin ${pluginId} missing semio_plugin_manifest export`);
	}
	const manifest = JSON.parse(module.semio_plugin_manifest()) as PluginManifest;
	return {
		pluginId,
		manifest,
		async createApp(appId: string) {
			const create = module.semio_plugin_create_app;
			if (!create) throw new Error(`plugin ${pluginId} missing create_app`);
			return create(appId);
		},
		async destroyApp(instanceId: number) {
			module.semio_plugin_destroy_app?.(instanceId);
		},
		async handleCommand(instanceId: number, commandJson: string, viewState: ViewState) {
			const handle = module.semio_plugin_handle_command;
			if (!handle) return [];
			const raw = handle(instanceId, commandJson, JSON.stringify(viewState));
			return JSON.parse(raw) as string[];
		},
		async render(instanceId: number, bodyKey: string, viewState: ViewState) {
			const render = module.semio_plugin_render;
			if (!render) throw new Error(`plugin ${pluginId} missing render`);
			return JSON.parse(render(instanceId, bodyKey, JSON.stringify(viewState))) as UiNode;
		},
		dispose() {},
	};
}

export async function loadPluginWasm(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
	return loadPluginModule(pluginId, moduleUrl);
}
