// #region 🧲Header
/** @emoji 🧊 `@semio-tech/framework-renderer-wgpu` — raw wgpu WASM renderer boot for declarative Rust plugin UI trees. */
// #endregion 🧲Header

import { loadPluginModule, pluginHandleForBridge } from "@semio-tech/framework-core";

export type FrameworkOsWgpuBootOptions = {
	readonly rootId?: string;
	readonly plugin?: string;
	readonly plugins?: readonly { readonly pluginId: string; readonly moduleUrl: string }[];
	readonly rendererModuleUrl?: string;
};

const DEFAULT_RENDERER_MODULE_URL = "/renderer-modules/wgpu/semio_framework_renderer_wgpu.js";

export async function bootFrameworkOsWgpu(options: FrameworkOsWgpuBootOptions = {}): Promise<void> {
	const root = document.getElementById(options.rootId ?? "root");
	if (!root) throw new Error("missing #root");
	root.replaceChildren();
	const canvas = document.createElement("canvas");
	canvas.id = "semio-wgpu-canvas";
	canvas.style.display = "block";
	canvas.style.width = "100%";
	canvas.style.height = "100vh";
	canvas.style.touchAction = "none";
	root.append(canvas);

	const pluginEntries = options.plugins ?? [];
	const handles = await Promise.all(
		pluginEntries.map(async (entry) => ({
			pluginId: entry.pluginId,
			handle: pluginHandleForBridge(await loadPluginModule(entry.pluginId, entry.moduleUrl)),
		})),
	);

	const rendererUrl = options.rendererModuleUrl ?? DEFAULT_RENDERER_MODULE_URL;
	const rendererModule = (await import(/* @vite-ignore */ rendererUrl)) as {
		default?: (input?: WebAssembly.Module | BufferSource | Response) => Promise<void>;
		semioRendererBoot?: (
			canvas: HTMLCanvasElement,
			plugins: { pluginId: string; handle: ReturnType<typeof pluginHandleForBridge> }[],
			pluginFilter: string,
		) => Promise<void>;
	};
	if (rendererModule.default) await rendererModule.default();
	if (!rendererModule.semioRendererBoot) {
		throw new Error("[DEBUG] wgpu renderer module missing semioRendererBoot");
	}
	await rendererModule.semioRendererBoot(canvas, handles, options.plugin ?? "s");
}
