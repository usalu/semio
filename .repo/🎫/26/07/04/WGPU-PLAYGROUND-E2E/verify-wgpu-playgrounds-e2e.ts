#!/usr/bin/env bun
/** 🧪 Wgpu OS dev smoke: each plugin boots, renders non-empty canvas, no console errors. */

import { type Subprocess, spawn } from "bun";
import { chromium } from "playwright";
import { join } from "node:path";

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

async function canvasHasVisibleContent(page: import("playwright").Page): Promise<boolean> {
	const canvas = page.locator("#semio-wgpu-canvas");
	const box = await canvas.boundingBox();
	if (!box || box.width < 1 || box.height < 1) return false;
	const png = await canvas.screenshot({ type: "png" });
	if (png.length < 200) return false;
	let min = 255;
	let max = 0;
	for (let i = 100; i < Math.min(png.length, 4000); i++) {
		const value = png[i]!;
		if (value < min) min = value;
		if (value > max) max = value;
	}
	return max - min > 8;
}

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
	const page = await browser.newPage();
	try {
		page.on("pageerror", (error) => errors.push(error.message));
		page.on("console", (message) => {
			if (message.type() !== "error") return;
			const text = message.text();
			if (
				text.includes("boot failed") ||
				text.includes("atlas failed") ||
				text.includes("Failed to resolve import") ||
				text.includes("Uncaught")
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
		if (!painted) throw new Error("canvas screenshot has no visible content");
		if (errors.length > 0) throw new Error(errors.join(" | "));
		return `ok canvas painted plugin=${pluginId}`;
	} finally {
		await page.close();
	}
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
