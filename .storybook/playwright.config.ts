// #region 🧲Header
// 💻 .storybook/playwright.config.ts
// Specs: Run Playwright smoke coverage against the root monorepo Storybook dev server.
// Summary: Configures Playwright for Storybook end-to-end verification across aggregated workspace stories.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { defineConfig, devices } from "@playwright/test";

const storybookDir = resolve(fileURLToPath(import.meta.url), "..");
const repoRootPath = resolve(storybookDir, "..");
const storybookPort = process.env.STORYBOOK_PORT ?? "65010";
function withTrailingSlash(url: string): string {
	return url.endsWith("/") ? url : `${url}/`;
}
/** Trailing `/` so `page.goto("iframe.html")` stays under the static server root (`dev.script.ts storybook-static` serves `storybook-static/`). */
const baseURL = withTrailingSlash(
	process.env.PLAYWRIGHT_BASE_URL ?? `http://127.0.0.1:${storybookPort}/storybook-static`,
);
const webServerUrl = new URL("index.html", baseURL).href;

export default defineConfig({
	testDir: storybookDir,
	testMatch: ["monorepo.spec.ts"],
	fullyParallel: false,
	forbidOnly: !!process.env.CI,
	retries: process.env.CI ? 2 : 0,
	timeout: 300000,
	expect: { timeout: 120_000 },
	workers: 1,
	reporter: [["list"]],
	use: {
		baseURL,
		trace: "on-first-retry",
	},
	projects: [
		{
			name: "chromium",
			use: {
				...devices["Desktop Chrome"],
				launchOptions: {
					args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader"],
				},
			},
		},
	],
});