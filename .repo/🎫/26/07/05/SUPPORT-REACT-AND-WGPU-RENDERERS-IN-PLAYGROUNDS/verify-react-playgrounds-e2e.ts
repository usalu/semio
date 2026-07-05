#!/usr/bin/env bun
/** 🧪 React OS dev functional smoke: each plugin boots, chrome visible, canvas has real content, no console errors. */

import { type Subprocess, spawn } from "bun";
import { chromium } from "playwright";
import { join } from "node:path";
import { PNG } from "pngjs";

const repoRoot = join(import.meta.dir, "../../../../../..");
const plugins = [
	"draw", "note", "writer", "raster", "forms", "vcs", "flow", "dag", "imperative", "sequence",
	"layout", "puzzle2d", "gis2d", "procedural2d", "reasoning-wires", "cad", "puzzle3d", "puzzle5d",
	"shooting", "lowpoly", "procedural3d", "trinity", "trinity-rewrite", "s", "presentation",
] as const;

const world3dPlugins = new Set(["cad", "puzzle3d", "puzzle5d", "shooting", "lowpoly", "procedural3d"]);
const canvas2dPlugins = new Set([
	"draw", "note", "layout", "puzzle2d", "gis2d", "procedural2d", "reasoning-wires", "presentation",
]);
const graphPlugins = new Set(["flow", "dag", "sequence", "trinity", "trinity-rewrite"]);

const bunExe = Bun.which("bun") ?? "bun";
const port = process.env.S_OS_PORT ?? "7199";
const baseUrl = `http://127.0.0.1:${port}/`;
const bootTimeoutMs = 240_000;
const onlyPlugin = process.argv.find((arg, index) => process.argv[index - 1] === "--plugin");
const targets = onlyPlugin ? plugins.filter((id) => id === onlyPlugin) : [...plugins];

type PaintStats = {
	readonly nonBackgroundRatio: number;
	readonly maxLuma: number;
};

function analyzePngBuffer(png: Buffer): PaintStats {
	const { data, width: w, height: h } = PNG.sync.read(png);
	if (w < 8 || h < 8) throw new Error("screenshot too small");
	const bgR = 13;
	const bgG = 13;
	const bgB = 15;
	const tolerance = 8;
	const isBg = (r: number, g: number, b: number) =>
		Math.abs(r - bgR) <= tolerance && Math.abs(g - bgG) <= tolerance && Math.abs(b - bgB) <= tolerance;
	const luma = (r: number, g: number, b: number) => 0.299 * r + 0.587 * g + 0.114 * b;
	let nonBg = 0;
	let maxLuma = 0;
	for (let i = 0; i < data.length; i += 4) {
		const r = data[i]!;
		const g = data[i + 1]!;
		const b = data[i + 2]!;
		maxLuma = Math.max(maxLuma, luma(r, g, b));
		if (!isBg(r, g, b)) nonBg += 1;
	}
	return { nonBackgroundRatio: nonBg / (w * h), maxLuma };
}

const tier1Plugins = new Set([
	"cad", "gis2d", "puzzle2d", "puzzle3d", "puzzle5d", "flow", "procedural2d", "procedural3d",
	"trinity", "trinity-rewrite", "sequence",
]);

const BODY_MIN_RATIO = 0.004;
const BODY_MIN_LUMA = 28;

type BodyPaintStats = { readonly bodyNonBgRatio: number; readonly maxLuma: number };

function analyzeBodyRegion(png: Buffer): BodyPaintStats {
	const { data, width: w, height: h } = PNG.sync.read(png);
	const y0 = Math.floor(h * 0.08);
	const y1 = Math.floor(h * 0.92);
	const x0 = Math.floor(w * 0.06);
	const x1 = Math.floor(w * 0.94);
	const bgR = 13;
	const bgG = 13;
	const bgB = 15;
	const tolerance = 8;
	const luma = (r: number, g: number, b: number) => 0.299 * r + 0.587 * g + 0.114 * b;
	let bodyNonBg = 0;
	let bodyTotal = 0;
	let maxLuma = 0;
	for (let y = y0; y < y1; y++) {
		for (let x = x0; x < x1; x++) {
			const i = (y * w + x) * 4;
			const r = data[i]!;
			const g = data[i + 1]!;
			const b = data[i + 2]!;
			maxLuma = Math.max(maxLuma, luma(r, g, b));
			bodyTotal += 1;
			if (
				Math.abs(r - bgR) > tolerance ||
				Math.abs(g - bgG) > tolerance ||
				Math.abs(b - bgB) > tolerance
			) {
				bodyNonBg += 1;
			}
		}
	}
	return { bodyNonBgRatio: bodyTotal > 0 ? bodyNonBg / bodyTotal : 0, maxLuma };
}

const pluginBodyMinRatio: Partial<Record<(typeof plugins)[number], number>> = {
	cad: 0.008,
	puzzle3d: 0.006,
	puzzle5d: 0.006,
	flow: 0.003,
	draw: 0.003,
	gis2d: 0.004,
	lowpoly: 0.006,
	procedural2d: 0.003,
};

async function assertTier1BodyContent(page: import("playwright").Page, pluginId: string): Promise<void> {
	const selector = world3dPlugins.has(pluginId)
		? ".semio-world-3d-host canvas"
		: canvas2dPlugins.has(pluginId)
			? "canvas"
			: graphPlugins.has(pluginId)
				? '[data-component-kind="node-graph"], canvas, svg'
				: "#root";
	const locator = page.locator(selector).first();
	if ((await locator.count()) === 0) throw new Error(`tier1 body surface missing for ${pluginId}`);
	const png = Buffer.from(await locator.screenshot({ type: "png" }));
	const bodyStats = analyzeBodyRegion(png);
	const minRatio = pluginBodyMinRatio[pluginId as (typeof plugins)[number]] ?? BODY_MIN_RATIO;
	if (bodyStats.bodyNonBgRatio < minRatio) {
		throw new Error(`body content too sparse ratio=${bodyStats.bodyNonBgRatio.toFixed(4)} min=${minRatio}`);
	}
	if (bodyStats.maxLuma < BODY_MIN_LUMA) {
		throw new Error(`body lacks visible contrast maxLuma=${bodyStats.maxLuma.toFixed(1)}`);
	}
}

async function commandPaletteSmoke(page: import("playwright").Page): Promise<void> {
	await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
	await page.waitForTimeout(300);
	await page.keyboard.press("Escape");
}

async function capturePaintStats(page: import("playwright").Page, selector: string): Promise<PaintStats | null> {
	const locator = page.locator(selector).first();
	if ((await locator.count()) === 0) return null;
	const png = Buffer.from(await locator.screenshot({ type: "png" }));
	return analyzePngBuffer(png);
}

async function assertFunctionalContent(page: import("playwright").Page, pluginId: string): Promise<void> {
	if (world3dPlugins.has(pluginId)) {
		const stats = await capturePaintStats(page, ".semio-world-3d-host canvas");
		if (!stats) throw new Error("world-3d canvas missing");
		if (stats.nonBackgroundRatio < 0.005) throw new Error("world-3d canvas appears blank");
		if (stats.maxLuma < 40) throw new Error("world-3d canvas lacks visible geometry");
		return;
	}
	if (canvas2dPlugins.has(pluginId)) {
		const stats = await capturePaintStats(page, "canvas");
		if (!stats) throw new Error("canvas-2d surface missing");
		if (stats.nonBackgroundRatio < 0.003) throw new Error("canvas-2d appears blank");
		return;
	}
	if (graphPlugins.has(pluginId)) {
		const nodeGraph = page.locator('[data-component-kind="node-graph"], canvas, svg').first();
		if ((await nodeGraph.count()) === 0) throw new Error("graph surface missing");
		return;
	}
	if (pluginId === "raster") {
		const stats = await capturePaintStats(page, "canvas");
		if (!stats || stats.nonBackgroundRatio < 0.002) throw new Error("raster composite appears empty");
		return;
	}
	if (pluginId === "writer" || pluginId === "forms" || pluginId === "vcs" || pluginId === "imperative") {
		const body = page.locator('[data-slot="window-content"], .semio-text-editor-host, table').first();
		if ((await body.count()) === 0) throw new Error("main body surface missing");
	}
}

async function assertPanelTabs(page: import("playwright").Page): Promise<void> {
	const tabs = page.locator('[data-slot="side-panel-tabs"], [data-slot="mobile-panel-tabs"]').first();
	if ((await tabs.count()) > 0) return;
	const tabButton = page.locator('[data-slot="side-panel-tab-button"], [data-slot="mobile-panel-tab-button"]').first();
	if ((await tabButton.count()) > 0) return;
	const panelContent = page.locator('[data-slot="side-panel-content"], [data-slot="mobile-panel-content"]').first();
	if ((await panelContent.count()) === 0) throw new Error("side panel missing");
}

async function waitForDev(url: string): Promise<void> {
	const deadline = Date.now() + bootTimeoutMs;
	while (Date.now() < deadline) {
		try {
			const response = await fetch(url);
			if (response.ok) return;
		} catch {}
		await Bun.sleep(500);
	}
	throw new Error(`dev server not ready at ${url}`);
}

async function waitForReactShell(page: import("playwright").Page): Promise<void> {
	await page.waitForSelector('[data-slot="navbar"]', { timeout: bootTimeoutMs });
	await page.waitForSelector('[data-slot="footer"]', { timeout: bootTimeoutMs });
	await page.waitForFunction(() => {
		const alert = document.querySelector('[role="alert"]');
		return alert == null;
	}, { timeout: bootTimeoutMs });
	await page.waitForTimeout(1200);
}

async function shellChromeVisible(page: import("playwright").Page): Promise<boolean> {
	const navbar = page.locator('[data-slot="navbar"]');
	const footer = page.locator('[data-slot="footer"]');
	const appName = page.locator('[data-slot="app-name"]');
	if (!(await navbar.isVisible())) return false;
	if (!(await footer.isVisible())) return false;
	const name = (await appName.textContent())?.trim() ?? "";
	return name.length > 0 && !name.toLowerCase().includes("loading");
}

async function world3dCanvasReady(page: import("playwright").Page): Promise<boolean> {
	const canvas = page.locator(".semio-world-3d-host canvas");
	if ((await canvas.count()) === 0) return false;
	const box = await canvas.first().boundingBox();
	return box != null && box.width > 8 && box.height > 8;
}

function isBenignPageError(text: string): boolean {
	if (text.includes("NoCompatibleDevice")) return true;
	if (text.includes("No available adapters")) return true;
	return false;
}

function isActionableConsoleError(text: string): boolean {
	if (text.includes("[DEBUG]")) return false;
	if (text.includes("boot failed")) return true;
	if (text.includes("Failed to resolve import")) return true;
	if (text.includes("Uncaught")) return true;
	if (text.includes("World LOD context missing")) return true;
	if (text.includes("same key")) return true;
	if (text.includes("WebGLRenderer")) return true;
	return false;
}

function isActionableConsoleWarning(text: string): boolean {
	if (text.includes("[DEBUG]")) return false;
	if (text.includes("GL Driver Message")) return false;
	if (text.includes("WebGL-")) return false;
	if (text.includes("same key")) return true;
	return false;
}

async function smokePlugin(
	browser: import("playwright").Browser,
	pluginId: string,
): Promise<string> {
	const errors: string[] = [];
	const warnings: string[] = [];
	const page = await browser.newPage();
	try {
		page.on("pageerror", (error) => {
			if (!isBenignPageError(error.message)) errors.push(error.message);
		});
		page.on("console", (message) => {
			const text = message.text();
			if (message.type() === "warning" && isActionableConsoleWarning(text)) {
				warnings.push(text);
			}
			if (message.type() === "error" && isActionableConsoleError(text)) {
				errors.push(text);
			}
		});
		await page.goto(`${baseUrl}?plugin=${encodeURIComponent(pluginId)}`, {
			waitUntil: "domcontentloaded",
			timeout: bootTimeoutMs,
		});
		await waitForReactShell(page);
		const chromeVisible = await shellChromeVisible(page);
		if (!chromeVisible) throw new Error("shell chrome not visible");
		await assertPanelTabs(page);
		if (world3dPlugins.has(pluginId)) {
			const worldReady = await world3dCanvasReady(page);
			if (!worldReady) throw new Error("world-3d canvas missing or too small");
		}
		await assertFunctionalContent(page, pluginId);
		if (tier1Plugins.has(pluginId)) {
			await assertTier1BodyContent(page, pluginId);
			await commandPaletteSmoke(page);
		}
		if (pluginId === "s" || pluginId === "flow" || pluginId === "puzzle3d") {
			await interactionSmoke(page, pluginId);
		}
		if (warnings.length > 0) throw new Error(`console warnings: ${warnings.join(" | ")}`);
		if (errors.length > 0) throw new Error(errors.join(" | "));
		const shotPath = join(import.meta.dir, `screenshot-react-${pluginId}.png`);
		await page.screenshot({ path: shotPath, fullPage: false });
		return `ok functional plugin=${pluginId}`;
	} finally {
		await page.close();
	}
}

async function interactionSmoke(page: import("playwright").Page, pluginId: string): Promise<void> {
	const root = page.locator("#root");
	const box = await root.boundingBox();
	if (!box) throw new Error("root missing for interaction smoke");
	const click = async (rx: number, ry: number) => {
		await page.mouse.click(box.x + box.width * rx, box.y + box.height * ry);
		await page.waitForTimeout(200);
	};
	await click(0.92, 0.04);
	await click(0.88, 0.04);
	if (pluginId === "flow") {
		await click(0.5, 0.55);
		await click(0.62, 0.48);
	}
	const chromeVisible = await shellChromeVisible(page);
	if (!chromeVisible) throw new Error("shell chrome lost after interaction smoke");
}

let devProc: Subprocess | null = null;
const logLines: string[] = [];
const useRunningServer = process.env.SKIP_DEV === "1";

try {
	if (!useRunningServer) {
		devProc = spawn({
			cmd: [bunExe, "nx", "run", "@semio-tech/framework-os-dev:dev"],
			cwd: repoRoot,
			stdout: "pipe",
			stderr: "pipe",
			env: {
				...process.env,
				SKIP_PLUGIN_BUILD: process.env.SKIP_PLUGIN_BUILD ?? "1",
				SKIP_ENGINE_BUILD: process.env.SKIP_ENGINE_BUILD ?? "1",
				S_OS_PORT: port,
				SEMIO_RENDERER: "react",
				SEMIO_PLUGIN: "s",
			},
		});
		await waitForDev(baseUrl);
	}

	let browser = await chromium.launch({ headless: true });
	let failed = 0;
	try {
		for (const [index, pluginId] of targets.entries()) {
			if (index > 0 && index % 8 === 0) {
				await browser.close();
				browser = await chromium.launch({ headless: true });
			}
			process.stdout.write(`REACTTEST ${pluginId}... `);
			try {
				const line = await smokePlugin(browser, pluginId);
				console.log(line);
				logLines.push(`${pluginId}: ${line}`);
			} catch (error) {
				failed += 1;
				const message = error instanceof Error ? error.message : String(error);
				console.log(`FAIL ${message}`);
				logLines.push(`${pluginId}: FAIL ${message}`);
			}
		}
	} finally {
		await browser.close();
	}

	const logPath = join(import.meta.dir, "verify-react-playgrounds-e2e.log");
	await Bun.write(logPath, `${logLines.join("\n")}\n`);
	console.log(`\nlog: ${logPath}`);
	console.log(`\n${targets.length - failed}/${targets.length} passed`);
	process.exit(failed > 0 ? 1 : 0);
} finally {
	devProc?.kill();
	await devProc?.exited;
}
