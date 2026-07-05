#!/usr/bin/env bun
/** 🧪 React OS dev smoke: each plugin boots, shell chrome visible, no console errors. */

import { type Subprocess, spawn } from "bun";
import { chromium } from "playwright";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "../../../../../..");
const plugins = [
	"draw", "note", "writer", "raster", "forms", "vcs", "flow", "dag", "imperative", "sequence",
	"layout", "puzzle2d", "gis2d", "procedural2d", "reasoning-wires", "cad", "puzzle3d", "puzzle5d",
	"shooting", "lowpoly", "procedural3d", "trinity", "trinity-rewrite", "s", "presentation",
] as const;

const world3dPlugins = new Set(["cad", "puzzle3d", "puzzle5d", "shooting", "lowpoly", "procedural3d"]);

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
		page.on("pageerror", (error) => errors.push(error.message));
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
		if (world3dPlugins.has(pluginId)) {
			const worldReady = await world3dCanvasReady(page);
			if (!worldReady) throw new Error("world-3d canvas missing or too small");
		}
		if (pluginId === "s" || pluginId === "flow") {
			await interactionSmoke(page, pluginId);
		}
		if (warnings.length > 0) throw new Error(`console warnings: ${warnings.join(" | ")}`);
		if (errors.length > 0) throw new Error(errors.join(" | "));
		const shotPath = join(import.meta.dir, `screenshot-react-${pluginId}.png`);
		await page.screenshot({ path: shotPath, fullPage: true });
		return `ok shell visible plugin=${pluginId}`;
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
				S_OS_PORT: port,
				SEMIO_RENDERER: "react",
				SEMIO_PLUGIN: "s",
			},
		});
		await waitForDev(baseUrl);
	}

	const browser = await chromium.launch({ headless: true });
	let failed = 0;
	try {
		for (const pluginId of targets) {
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
