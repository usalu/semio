#!/usr/bin/env bun
/** 🧪 Wgpu OS dev smoke: each plugin boots, renders non-empty canvas, no console errors. */

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

const bunExe = Bun.which("bun") ?? "bun";
const port = process.env.S_OS_PORT ?? "7199";
const baseUrl = `http://127.0.0.1:${port}/`;
const bootTimeoutMs = 240_000;
const onlyPlugin = process.argv.find((arg, index) => process.argv[index - 1] === "--plugin");
const targets = onlyPlugin ? plugins.filter((id) => id === onlyPlugin) : [...plugins];

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

const BG_R = 13;
const BG_G = 13;
const BG_B = 15;
const BG_TOLERANCE = 6;
const CHROME_LUMA_MIN = 72;
const CHROME_PIXEL_MIN_RATIO = 0.001;
const CHROME_MAX_LUMA_MIN = 80;

type PaintStats = {
	readonly nonBackgroundRatio: number;
	readonly navbarChromeRatio: number;
	readonly footerChromeRatio: number;
	readonly navbarMaxLuma: number;
	readonly footerMaxLuma: number;
};

function analyzePngBuffer(png: Buffer): PaintStats {
	const { data, width: w, height: h } = PNG.sync.read(png);
	if (w < 8 || h < 8) throw new Error("screenshot too small");
	const isBg = (r: number, g: number, b: number) =>
		Math.abs(r - BG_R) <= BG_TOLERANCE &&
		Math.abs(g - BG_G) <= BG_TOLERANCE &&
		Math.abs(b - BG_B) <= BG_TOLERANCE;
	const luma = (r: number, g: number, b: number) => 0.299 * r + 0.587 * g + 0.114 * b;
	let nonBg = 0;
	const total = w * h;
	const stripStats = (y0: number, y1: number, x0 = 0, x1 = w) => {
		let nonBgStrip = 0;
		let chromeStrip = 0;
		let maxLuma = 0;
		let count = 0;
		for (let y = y0; y < y1; y++) {
			for (let x = x0; x < x1; x++) {
				const i = (y * w + x) * 4;
				const r = data[i]!;
				const g = data[i + 1]!;
				const b = data[i + 2]!;
				const l = luma(r, g, b);
				count += 1;
				maxLuma = Math.max(maxLuma, l);
				if (!isBg(r, g, b)) nonBgStrip += 1;
				if (l >= CHROME_LUMA_MIN) chromeStrip += 1;
			}
		}
		return {
			nonBgRatio: count > 0 ? nonBgStrip / count : 0,
			chromeRatio: count > 0 ? chromeStrip / count : 0,
			maxLuma,
		};
	};
	for (let i = 0; i < data.length; i += 4) {
		if (!isBg(data[i]!, data[i + 1]!, data[i + 2]!)) nonBg += 1;
	}
	const navbarH = Math.max(8, Math.floor(h * 0.06));
	const footerH = Math.max(8, Math.floor(h * 0.05));
	const navbar = stripStats(0, navbarH);
	const footer = stripStats(h - footerH, h);
	return {
		nonBackgroundRatio: nonBg / total,
		navbarChromeRatio: navbar.chromeRatio,
		footerChromeRatio: footer.chromeRatio,
		navbarMaxLuma: navbar.maxLuma,
		footerMaxLuma: footer.maxLuma,
	};
}

async function canvasPaintStats(page: import("playwright").Page): Promise<PaintStats> {
	const png = await page.locator("#semio-wgpu-canvas").screenshot({ type: "png" });
	return analyzePngBuffer(Buffer.from(png));
}

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
	return {
		bodyNonBgRatio: bodyTotal > 0 ? bodyNonBg / bodyTotal : 0,
		maxLuma,
	};
}

const pluginBodyMinRatio: Partial<Record<(typeof plugins)[number], number>> = {
	cad: 0.008,
	puzzle3d: 0.006,
	puzzle5d: 0.006,
	flow: 0.003,
	draw: 0.003,
	gis2d: 0.004,
	lowpoly: 0.006,
};

async function assertBodyContent(page: import("playwright").Page, pluginId: string): Promise<void> {
	const png = Buffer.from(await page.locator("#semio-wgpu-canvas").screenshot({ type: "png" }));
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

async function canvasHasVisibleContent(page: import("playwright").Page): Promise<boolean> {
	const stats = await canvasPaintStats(page);
	if (stats.nonBackgroundRatio < 0.01) return false;
	if (stats.navbarChromeRatio < CHROME_PIXEL_MIN_RATIO || stats.navbarMaxLuma < CHROME_MAX_LUMA_MIN) return false;
	if (stats.footerChromeRatio < CHROME_PIXEL_MIN_RATIO || stats.footerMaxLuma < CHROME_MAX_LUMA_MIN) return false;
	return true;
}

async function capturePaintStats(page: import("playwright").Page, selector: string): Promise<PaintStats | null> {
	const locator = page.locator(selector);
	if ((await locator.count()) === 0) return null;
	const png = await locator.screenshot({ type: "png" });
	return analyzePngBuffer(Buffer.from(png));
}

const tier1Plugins = new Set([
	"cad", "gis2d", "puzzle2d", "puzzle3d", "puzzle5d", "flow", "procedural2d", "procedural3d",
	"trinity", "trinity-rewrite", "sequence",
]);

async function waitForWgpuBoot(page: import("playwright").Page): Promise<void> {
	await new Promise<void>((resolve, reject) => {
		const timeout = setTimeout(() => reject(new Error("wgpu boot timeout")), bootTimeoutMs);
		const onConsole = (message: import("playwright").ConsoleMessage) => {
			if (message.text().includes("[DEBUG] wgpu renderer booted")) {
				clearTimeout(timeout);
				page.off("console", onConsole);
				resolve();
			}
		};
		page.on("console", onConsole);
	});
}

async function smokePlugin(
	browser: import("playwright").Browser,
	pluginId: string,
): Promise<string> {
	const errors: string[] = [];
	const warnings: string[] = [];
	const page = await browser.newPage();
	try {
		page.on("pageerror", (error) => errors.push(error.message));
		page.on("console", (message) => {
			const text = message.text();
			if (message.type() === "warning" && !text.includes("[DEBUG]")) {
				warnings.push(text);
			}
			if (message.type() !== "error") return;
			if (
				text.includes("boot failed") ||
				text.includes("atlas failed") ||
				text.includes("Failed to resolve import") ||
				text.includes("Uncaught") ||
				text.includes("WebGPU")
			) {
				errors.push(text);
			}
		});
		await page.goto(`${baseUrl}?plugin=${encodeURIComponent(pluginId)}`, {
			waitUntil: "domcontentloaded",
			timeout: bootTimeoutMs,
		});
		await page.waitForSelector("#semio-wgpu-canvas", { timeout: bootTimeoutMs });
		await waitForWgpuBoot(page);
		await page.waitForTimeout(1200);
		const painted = await canvasHasVisibleContent(page);
		if (!painted) {
			const stats = await canvasPaintStats(page).catch(() => null);
			throw new Error(
				stats
					? `canvas paint check failed ratio=${stats.nonBackgroundRatio.toFixed(4)} navbarChrome=${stats.navbarChromeRatio.toFixed(4)} footerChrome=${stats.footerChromeRatio.toFixed(4)} navbarMaxL=${stats.navbarMaxLuma.toFixed(1)} footerMaxL=${stats.footerMaxLuma.toFixed(1)}`
					: "canvas screenshot has no visible content",
			);
		}
		if (tier1Plugins.has(pluginId)) {
			await assertBodyContent(page, pluginId);
			await commandPaletteSmoke(page);
		}
		if (pluginId === "s" || pluginId === "flow") {
			await interactionSmoke(page, pluginId);
		}
		if (pluginId === "s" || pluginId === "draw" || pluginId === "flow") {
			await chromeParitySmoke(page, pluginId);
		}
		if (warnings.length > 0) throw new Error(`console warnings: ${warnings.join(" | ")}`);
		if (errors.length > 0) throw new Error(errors.join(" | "));
		const shotPath = join(import.meta.dir, `screenshot-${pluginId}.png`);
		await page.locator("#semio-wgpu-canvas").screenshot({ path: shotPath });
		return `ok canvas painted plugin=${pluginId}`;
	} finally {
		await page.close();
	}
}

async function chromeParitySmoke(page: import("playwright").Page, pluginId: string): Promise<void> {
	const canvas = page.locator("#semio-wgpu-canvas");
	const box = await canvas.boundingBox();
	if (!box) throw new Error("canvas missing for chrome parity smoke");
	await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
	await page.waitForTimeout(350);
	let painted = await canvasHasVisibleContent(page);
	if (!painted) throw new Error("command palette broke canvas paint");
	await page.keyboard.press("Escape");
	await page.waitForTimeout(150);
	if (pluginId === "flow" || pluginId === "draw") {
		await page.keyboard.press(process.platform === "darwin" ? "Meta+f" : "Control+f");
		await page.waitForTimeout(350);
		painted = await canvasHasVisibleContent(page);
		if (!painted) throw new Error("find palette broke canvas paint");
		await page.keyboard.press("Escape");
	}
	const click = async (rx: number, ry: number) => {
		await page.mouse.click(box.x + box.width * rx, box.y + box.height * ry);
		await page.waitForTimeout(250);
	};
	await click(0.78, 0.08);
	if (pluginId === "draw") {
		await click(0.92, 0.12);
	}
	if (pluginId === "s") {
		await click(0.06, 0.12);
		await click(0.92, 0.12);
	}
	painted = await canvasHasVisibleContent(page);
	if (!painted) throw new Error("window chrome rails broke canvas paint");
}

async function interactionSmoke(page: import("playwright").Page, pluginId: string): Promise<void> {
	const canvas = page.locator("#semio-wgpu-canvas");
	const box = await canvas.boundingBox();
	if (!box) throw new Error("canvas missing for interaction smoke");
	const click = async (rx: number, ry: number) => {
		await page.mouse.click(box.x + box.width * rx, box.y + box.height * ry);
		await page.waitForTimeout(200);
	};
	await click(0.72, 0.04);
	await click(0.68, 0.04);
	if (pluginId === "flow") {
		await click(0.5, 0.55);
		await click(0.62, 0.48);
	}
	const painted = await canvasHasVisibleContent(page);
	if (!painted) throw new Error("canvas empty after interaction smoke");
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
				S_OS_PORT: port,
				SEMIO_RENDERER: "wgpu",
				SEMIO_PLUGIN: "s",
			},
		});
		await waitForDev(baseUrl);
	}

	const browser = await chromium.launch({
		headless: true,
		args: ["--enable-unsafe-webgpu", "--use-angle=swiftshader", "--enable-features=Vulkan"],
	});
	let failed = 0;
	try {
		for (const pluginId of targets) {
			process.stdout.write(`WGPUTEST ${pluginId}... `);
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

	const logPath = join(import.meta.dir, "verify-wgpu-playgrounds-e2e.log");
	await Bun.write(logPath, `${logLines.join("\n")}\n`);
	console.log(`\nlog: ${logPath}`);
	console.log(`\n${targets.length - failed}/${targets.length} passed`);
	process.exit(failed > 0 ? 1 : 0);
} finally {
	devProc?.kill();
	await devProc?.exited;
}
