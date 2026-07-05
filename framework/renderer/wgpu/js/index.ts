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

const ICON_IDS = [
	"search",
	"panel-left",
	"panel-right",
	"chevron-left",
	"chevron-right",
	"chevron-up",
	"chevron-down",
	"sun",
	"moon",
	"rotate-ccw",
	"rotate-cw",
	"save",
	"home",
	"settings",
	"x",
	"plus",
	"minus",
] as const;

const ICON_SIZE = 24;
const ATLAS_COLS = 8;

async function rasterizeIcon(id: string): Promise<ImageData | null> {
	const response = await fetch(`/asset/icon/${id}.svg`);
	if (!response.ok) return null;
	const svg = await response.text();
	const blob = new Blob([svg], { type: "image/svg+xml" });
	const url = URL.createObjectURL(blob);
	try {
		const image = await new Promise<HTMLImageElement>((resolve, reject) => {
			const img = new Image();
			img.onload = () => resolve(img);
			img.onerror = reject;
			img.src = url;
		});
		const canvas = document.createElement("canvas");
		canvas.width = ICON_SIZE;
		canvas.height = ICON_SIZE;
		const ctx = canvas.getContext("2d");
		if (!ctx) return null;
		ctx.clearRect(0, 0, ICON_SIZE, ICON_SIZE);
		ctx.drawImage(image, 0, 0, ICON_SIZE, ICON_SIZE);
		return ctx.getImageData(0, 0, ICON_SIZE, ICON_SIZE);
	} finally {
		URL.revokeObjectURL(url);
	}
}

export async function buildIconAtlas(): Promise<{
	width: number;
	height: number;
	pixels: Uint8Array;
	entries: Record<string, [number, number, number, number]>;
}> {
	const loaded = await Promise.all(ICON_IDS.map(async (id) => ({ id, image: await rasterizeIcon(id) })));
	const rows = Math.ceil(loaded.length / ATLAS_COLS);
	const width = ATLAS_COLS * ICON_SIZE;
	const height = rows * ICON_SIZE;
	const pixels = new Uint8Array(width * height * 4);
	const entries: Record<string, [number, number, number, number]> = {};
	for (const [index, item] of loaded.entries()) {
		if (!item.image) continue;
		const col = index % ATLAS_COLS;
		const row = Math.floor(index / ATLAS_COLS);
		const ox = col * ICON_SIZE;
		const oy = row * ICON_SIZE;
		for (let y = 0; y < ICON_SIZE; y++) {
			for (let x = 0; x < ICON_SIZE; x++) {
				const src = (y * ICON_SIZE + x) * 4;
				const dst = ((oy + y) * width + (ox + x)) * 4;
				pixels[dst] = item.image.data[src] ?? 0;
				pixels[dst + 1] = item.image.data[src + 1] ?? 0;
				pixels[dst + 2] = item.image.data[src + 2] ?? 0;
				pixels[dst + 3] = item.image.data[src + 3] ?? 0;
			}
		}
		entries[item.id] = [ox / width, oy / height, (ox + ICON_SIZE) / width, (oy + ICON_SIZE) / height];
	}
	return { width, height, pixels, entries };
}

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
		uploadIconAtlas?: (width: number, height: number, pixels: Uint8Array, entriesJson: string) => void;
	};
	if (rendererModule.default) await rendererModule.default();
	if (!rendererModule.semioRendererBoot) {
		throw new Error("[DEBUG] wgpu renderer module missing semioRendererBoot");
	}
	await rendererModule.semioRendererBoot(canvas, handles, options.plugin ?? "s");
	const iconAtlas = await buildIconAtlas();
	if (rendererModule.uploadIconAtlas) {
		rendererModule.uploadIconAtlas(
			iconAtlas.width,
			iconAtlas.height,
			iconAtlas.pixels,
			JSON.stringify(iconAtlas.entries),
		);
	}
}
