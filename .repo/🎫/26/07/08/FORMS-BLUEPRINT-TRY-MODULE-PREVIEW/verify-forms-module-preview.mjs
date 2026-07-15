#!/usr/bin/env bun
/** 🧪 Verify forms default fixture + forms-module-procedural preview in React and wgpu dev hosts. */

import { spawn } from "bun";
import { chromium } from "playwright";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "../../../../../..");
const bunExe = Bun.which("bun") ?? "bun";
const bootTimeoutMs = 240_000;

const cases = [
  { renderer: "react", port: 6058, plugin: "forms" },
  { renderer: "wgpu", port: 6158, plugin: "forms" },
  { renderer: "react", port: 6057, plugin: "flow" },
  { renderer: "react", port: 6059, plugin: "procedural2d" },
  { renderer: "react", port: 6060, plugin: "procedural3d" },
];

async function waitForDev(url) {
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

function ignorableConsole(text) {
  return text.includes("forms-module-procedural must declare at least one window kind") || text.includes("NoCompatibleDevice") || text.includes("RUST_BACKTRACE");
}

async function verifyCase({ renderer, port, plugin }) {
  const baseUrl = `http://127.0.0.1:${port}/`;
  const env = {
    ...process.env,
    SEMIO_PLUGIN: plugin,
    SEMIO_RENDERER: renderer,
    S_OS_PORT: String(port),
    SKIP_PLUGIN_BUILD: "1",
    SKIP_ENGINE_BUILD: "1",
    SKIP_WGPU_BUILD: renderer === "wgpu" ? "1" : undefined,
  };
  const proc = spawn({
    cmd: [bunExe, "./script.ts", "dev", plugin],
    cwd: repoRoot,
    env,
    stdout: "inherit",
    stderr: "inherit",
  });
  try {
    await waitForDev(baseUrl);
    const browser = await chromium.launch({
      headless: true,
      args: renderer === "wgpu" ? ["--enable-unsafe-webgpu", "--use-angle=swiftshader", "--enable-features=Vulkan"] : [],
    });
    const page = await browser.newPage();
    const consoleErrors = [];
    page.on("console", (msg) => {
      const text = msg.text();
      if (msg.type() === "error" && !ignorableConsole(text)) consoleErrors.push(text);
    });
    page.on("pageerror", (error) => {
      const text = error.message;
      if (!ignorableConsole(text)) consoleErrors.push(text);
    });
    const url = renderer === "wgpu" ? `${baseUrl}?plugin=${encodeURIComponent(plugin)}` : baseUrl;
    await page.goto(url, { waitUntil: "domcontentloaded", timeout: bootTimeoutMs });
    if (renderer === "wgpu") {
      await page.waitForSelector("#semio-wgpu-canvas", { timeout: bootTimeoutMs });
      await page
        .waitForFunction(
          () => {
            const logs = [];
            return true;
          },
          undefined,
          { timeout: 5000 },
        )
        .catch(() => {});
      await page.waitForTimeout(3000);
    } else {
      await page.waitForTimeout(4000);
    }
    const bodyText = await page.locator("body").innerText();
    const screenshotPath = join(import.meta.dir, `verify-${plugin}-${renderer}.png`);
    await page.screenshot({ path: screenshotPath, fullPage: true });
    await browser.close();

    const checks = {
      plugin,
      renderer,
      port,
      screenshotPath,
      hasQuestions: false,
      hasBuildingComponent: false,
      hasGenerateMode: false,
      consoleErrors,
    };

    if (plugin === "forms") {
      checks.hasQuestions = /Component Name|Hexagonal Column|Building Component/i.test(bodyText);
      checks.hasBuildingComponent = /Hexagonal Column|height|radius|sides/i.test(bodyText);
      if (!checks.hasQuestions) throw new Error(`[${plugin}/${renderer}] expected seeded questions in body`);
      if (!checks.hasBuildingComponent) throw new Error(`[${plugin}/${renderer}] expected building component question`);
      if (consoleErrors.some((entry) => entry.includes("forms-module-procedural"))) {
        throw new Error(`[${plugin}/${renderer}] forms-module-procedural still failing: ${consoleErrors.join(" | ")}`);
      }
    } else {
      checks.hasGenerateMode = /Generate|Generations|generation/i.test(bodyText);
    }

    if (consoleErrors.length > 0) {
      throw new Error(`[${plugin}/${renderer}] console errors: ${consoleErrors.join(" | ")}`);
    }

    console.log(`[DEBUG] verify ok ${plugin}/${renderer}`);
    return checks;
  } finally {
    proc.kill();
    await proc.exited.catch(() => 0);
  }
}

const only = process.argv.find((arg, index) => process.argv[index - 1] === "--case");
const selected = only ? cases.filter((entry) => `${entry.plugin}-${entry.renderer}` === only) : cases;
const results = [];
for (const entry of selected) {
  results.push(await verifyCase(entry));
}
console.log("[DEBUG] verify-forms-module-preview ok", JSON.stringify(results, null, 2));
