import type { PluginManifest, ToolNode, UiNode, ViewState } from "./types.ts";
import {
	DEFAULT_PLUGIN_REGISTRY,
	loadPluginModule as loadCorePluginModule,
	loadPluginWasm as loadCorePluginWasm,
	type PluginRegistryEntry,
	type PluginWasmHandle as CorePluginWasmHandle,
} from "@semio-tech/framework-core";

export type PluginWasmHandle = {
	readonly pluginId: string;
	readonly manifest: PluginManifest;
	readonly createApp: (appId: string) => Promise<number>;
	readonly destroyApp: (instanceId: number) => Promise<void>;
	readonly handleCommand: (instanceId: number, commandJson: string, viewState: ViewState) => Promise<string[]>;
	readonly render: (instanceId: number, bodyKey: string, viewState: ViewState) => Promise<UiNode>;
	readonly tools: (instanceId: number, viewState: ViewState) => Promise<readonly ToolNode[]>;
	readonly dispose: () => void;
};

export type { PluginRegistryEntry };
export { DEFAULT_PLUGIN_REGISTRY };

export async function loadPluginModule(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
	return adaptPluginHandle(await loadCorePluginModule(pluginId, moduleUrl));
}

export async function loadPluginWasm(pluginId: string, moduleUrl: string): Promise<PluginWasmHandle> {
	return adaptPluginHandle(await loadCorePluginWasm(pluginId, moduleUrl));
}

function adaptPluginHandle(handle: CorePluginWasmHandle): PluginWasmHandle {
	return {
		pluginId: handle.pluginId,
		manifest: handle.manifest as unknown as PluginManifest,
		createApp: (appId) => handle.createApp(appId),
		destroyApp: (instanceId) => handle.destroyApp(instanceId),
		handleCommand: (instanceId, commandJson, viewState) =>
			handle.handleCommand(instanceId, commandJson, viewState),
		render: async (instanceId, bodyKey, viewState) =>
			(await handle.render(instanceId, bodyKey, viewState)) as unknown as UiNode,
		tools: async (instanceId, viewState) =>
			(await handle.tools(instanceId, viewState)) as unknown as ToolNode[],
		dispose: () => handle.dispose(),
	};
}
