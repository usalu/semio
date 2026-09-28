/** 🧊️ S19 one-off live probe: opens procedural generation3d (and, with `--flow`, the flow editor) in the served `s` shell
 * from Home via the command palette, lets the seated example evaluate, and records every console line, the contributions
 * closure the host pushed (`__semioOsInstalledContributions`), the window texts and two screenshots.
 * With `--export` it then presses the Actions rail `exportDocument` (default format) and records the download: its size and,
 * for glTF, the third-party-free structural witness (`asset.version`, mesh/accessor counts).
 * usage: bun s19-gen3d-probe.ts <outDir> [--serve <url>] [--port <n>] [--wait <ms>] [--app generation3d|flow] [--export]
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import { ensureParityPlaywrightBrowsersPath } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts";
import { awaitBeacon, click, clickUncovered, dismissIntroduction, seatLocale, unfoldActionsRail } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";
import { ensureDevServe } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts";
import type { Page } from "playwright";

const APPS = {
  generation3d: { pluginId: "procedural", appId: "s.procedural.generation3d@1/*#editor", query: "generation3d" },
  flow: { pluginId: "flow", appId: "s.flow.flow@1/*#editor", query: "flow" },
} as const;

const arg = (name: string): string | undefined => (process.argv.includes(name) ? process.argv[process.argv.indexOf(name) + 1] : undefined);

const windowIds = (page: Page): Promise<string[]> => page.evaluate(() => [...document.querySelectorAll('[data-slot="mode-dock-tab"]')].map((element) => element.getAttribute("data-window-id") ?? "").filter(Boolean));

async function openProgram(page: Page, pluginId: string, appId: string, query: string): Promise<{ opened: string[]; detail: string }> {
  const before = await windowIds(page);
  await dismissIntroduction(page);
  await page.evaluate(() => {
    if (document.activeElement instanceof HTMLElement) document.activeElement.blur();
    document.body.focus();
  });
  await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 15_000 }).catch(() => undefined);
  if ((await input.count()) === 0) return { opened: [], detail: "command palette never opened" };
  await input.fill(query);
  await page.waitForTimeout(1_200);
  for (const id of [`spawn.${pluginId}.${appId}`, `spawn.${pluginId}`]) {
    const item = page.locator(`[data-slot="command-item"][data-command-item-id="${id}"]`).first();
    await item.waitFor({ state: "visible", timeout: 8_000 }).catch(() => undefined);
    if ((await item.count()) === 0) continue;
    await item.click({ timeout: 8_000 }).catch(() => item.click({ force: true }).catch(() => undefined));
    const deadline = Date.now() + 120_000;
    while (Date.now() < deadline) {
      const fresh = (await windowIds(page)).filter((windowId) => !before.includes(windowId));
      if (fresh.length > 0) return { opened: fresh, detail: id };
      await page.waitForTimeout(300);
    }
    return { opened: [], detail: `${id} pressed, no new window` };
  }
  return { opened: [], detail: `no spawn row for ${pluginId}/${appId}` };
}

async function main(): Promise<void> {
  const outDir = process.argv[2];
  if (!outDir) throw new Error("usage: bun s19-gen3d-probe.ts <outDir> [--serve <url>] [--port <n>] [--wait <ms>] [--app generation3d|flow]");
  mkdirSync(outDir, { recursive: true });
  const app = APPS[(arg("--app") ?? "generation3d") as keyof typeof APPS];
  const waitMs = Number(arg("--wait") ?? 60_000);
  const repoRoot = "/Users/ueli/Documents/semio";
  const serve = arg("--serve") ? { url: arg("--serve")!, reused: true, stop: async () => {} } : await ensureDevServe({ repoRoot, port: Number(arg("--port") ?? 6610), onProgress: (_status, line) => console.log(line) });
  ensureParityPlaywrightBrowsersPath();
  const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
  const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
  const lines: string[] = [];
  page.on("console", (message) => lines.push(`${new Date().toISOString().slice(11, 23)} ${message.type()}: ${message.text()}`.slice(0, 2_000)));
  page.on("pageerror", (error) => lines.push(`${new Date().toISOString().slice(11, 23)} pageerror: ${String(error)}`.slice(0, 2_000)));
  page.on("worker", (worker) => {
    const name = worker.url().split("/").pop() ?? "worker";
    worker.on("console", (message) => lines.push(`${new Date().toISOString().slice(11, 23)} [worker ${name}] ${message.type()}: ${message.text()}`.slice(0, 2_000)));
  });
  const report: Record<string, unknown> = { serve: serve.url, reused: serve.reused, app, started: new Date().toISOString() };
  try {
    await page.goto(serve.url, { waitUntil: "commit", timeout: 300_000 });
    report.beacon = await awaitBeacon(page, Date.now() + 300_000);
    await dismissIntroduction(page);
    await page.waitForTimeout(3_000);
    report.seated = await seatLocale(page, "en");
    await page.keyboard.press("Escape").catch(() => undefined);
    const opened = await openProgram(page, app.pluginId, app.appId, app.query);
    report.opened = opened;
    await page.waitForTimeout(Math.min(20_000, waitMs));
    await page.screenshot({ path: join(outDir, "early.png") }).catch(() => undefined);
    await page.waitForTimeout(Math.max(0, waitMs - 20_000));
    await page.screenshot({ path: join(outDir, "late.png") }).catch(() => undefined);
    report.installed = await page.evaluate(() => {
      const witness = (window as unknown as { __semioOsInstalledContributions?: (instanceId?: number) => string | null }).__semioOsInstalledContributions;
      return witness ? witness() : null;
    });
    if (process.argv.includes("--export")) {
      const downloads: import("playwright").Download[] = [];
      page.on("download", (download) => downloads.push(download));
      const rowSelector = '[data-slot="window-action-pane"] [id="action.exportDocument"]';
      const executeSelector = '[data-slot="window-action-pane"] [id$=".action.exportDocument.execute"]';
      const unfolded = await unfoldActionsRail(page, 15_000);
      const row = await clickUncovered(page, rowSelector);
      await page.waitForTimeout(800);
      if ((await page.locator(executeSelector).count()) === 0) await clickUncovered(page, rowSelector);
      await page.waitForTimeout(800);
      const execute = await click(page, executeSelector);
      const deadline = Date.now() + 45_000;
      while (downloads.length === 0 && Date.now() < deadline) await page.waitForTimeout(250);
      const saved: Record<string, unknown>[] = [];
      for (const download of downloads) {
        const file = join(outDir, download.suggestedFilename());
        await download.saveAs(file);
        const bytes = new Uint8Array(await Bun.file(file).arrayBuffer());
        let gltf: unknown = null;
        if (file.endsWith(".gltf")) {
          const parsed = JSON.parse(new TextDecoder().decode(bytes)) as { asset?: { version?: string }; meshes?: unknown[]; accessors?: unknown[] };
          gltf = { assetVersion: parsed.asset?.version ?? null, meshes: parsed.meshes?.length ?? 0, accessors: parsed.accessors?.length ?? 0 };
        }
        saved.push({ file: download.suggestedFilename(), bytes: bytes.length, gltf });
      }
      report.export = { unfolded, row, execute, saved };
      await page.screenshot({ path: join(outDir, "export.png") }).catch(() => undefined);
    }
    report.windowTexts = await page.evaluate((ids) => ids.map((id) => [id, ((document.getElementById(id)?.querySelector('[data-slot="window-body"]') as HTMLElement | null)?.innerText ?? "").replace(/\s+/gu, " ").slice(0, 1_500)]), opened.opened);
  } catch (error) {
    report.fatal = String(error).slice(0, 800);
  } finally {
    report.finished = new Date().toISOString();
    writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 1));
    writeFileSync(join(outDir, "console.txt"), lines.join("\n") + "\n");
    await browser.close();
    if (!process.argv.includes("--keep-serve")) await serve.stop();
  }
}

await main();
