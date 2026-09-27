/** 🧩️ S17 live extension rows: for every parent program an extension contributes to, open it in the served `s` shell (Home →
 * command palette → spawn row, exactly as the program matrix does) and read the contributions closure the host pushed into
 * the guest through `setContributions` (`window.__semioOsInstalledContributions()`), plus every extension plugin's install
 * status from the shell's own catalog probe. A topic-only extension "loaded with its parent" iff its plugin id carries a
 * topic contribution in that pushed closure.
 * usage: bun s17-extension-probe.ts <baseUrl> <out.json> [--locale en|de]
 */
import { writeFileSync } from "node:fs";
import type { Page } from "playwright";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import { ensureParityPlaywrightBrowsersPath } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts";
import { awaitBeacon, dismissIntroduction, seatLocale } from "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";

const PARENTS: readonly { pluginId: string; appId: string; extensions: readonly string[] }[] = [
  { pluginId: "flow", appId: "s.flow.flow@1/*#editor", extensions: ["bim", "brep", "dictionary", "draw", "list", "logic", "math", "primitive", "text"].map((slug) => `flow-extension-${slug}`) },
  { pluginId: "procedural", appId: "s.procedural.generation3d@1/*#editor", extensions: ["bim", "brep", "dictionary", "draw", "list", "logic", "math", "primitive", "text"].map((slug) => `flow-extension-${slug}`) },
  { pluginId: "process", appId: "s.process.process3d@1/*#editor", extensions: ["metal", "robotic", "concrete", "wood"].map((slug) => `process-extension-${slug}`) },
  { pluginId: "sourcing", appId: "s.sourcing.curation@1/*#editor", extensions: ["slabs", "windows", "beams"].map((slug) => `sourcing-module-${slug}`) },
  { pluginId: "cad", appId: "s.cad.cad@1/*#editor", extensions: ["aec-building", "aec-building-structure", "aec-building-energy", "spatial-shape"].map((slug) => `cad-extension-${slug}`) },
  { pluginId: "imperative", appId: "s.imperative.procedure@1/*#editor", extensions: ["control", "text", "effect", "logic", "math"].map((slug) => `imperative-extension-${slug}`) },
  { pluginId: "playbook", appId: "s.playbook.playbook@1/*#editor", extensions: ["playbook-module-procedural"] },
];

type Probe = { plugins: { pluginId: string; status: string; routerFault?: unknown }[]; programs: { pluginId: string; appId: string; label: string }[] };

const windowIds = (page: Page): Promise<string[]> => page.evaluate(() => [...document.querySelectorAll('[data-slot="mode-dock-tab"]')].map((element) => element.getAttribute("data-window-id") ?? "").filter(Boolean));

async function openProgram(page: Page, pluginId: string, appId: string): Promise<{ opened: string[]; detail: string | null }> {
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
  await input.fill(/^s\.[^.]+\.([^@]+)@/u.exec(appId)?.[1] ?? appId);
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
  await page.keyboard.press("Escape");
  return { opened: [], detail: `no spawn row for ${pluginId}/${appId}` };
}

async function closeWindows(page: Page, ids: readonly string[]): Promise<void> {
  for (const id of ids) {
    await page.evaluate((windowId) => {
      const tab = [...document.querySelectorAll('[data-slot="mode-dock-tab"]')].find((element) => element.getAttribute("data-window-id") === windowId);
      const button = tab?.querySelector('[data-slot="mode-dock-tab-close"]') ?? tab?.parentElement?.querySelector('[data-slot="mode-dock-tab-close"]');
      if (button instanceof HTMLElement) button.click();
    }, id);
    await page.waitForTimeout(500);
  }
}

function contributedBy(installed: string | null): Record<string, string[]> {
  if (installed === null) return {};
  const json = installed.slice(installed.indexOf("::") + 2);
  const entries = JSON.parse(json) as { pluginId?: string; topicContribution?: { topic?: string } }[];
  const out: Record<string, string[]> = {};
  for (const entry of entries) {
    const topic = entry.topicContribution?.topic;
    if (!entry.pluginId || !topic) continue;
    (out[entry.pluginId] ??= []).push(topic);
  }
  return out;
}

async function main(): Promise<void> {
  const [baseUrl, outPath] = process.argv.slice(2);
  const locale = process.argv.includes("--locale") ? process.argv[process.argv.indexOf("--locale") + 1]! : "en";
  if (!baseUrl || !outPath) throw new Error("usage: bun s17-extension-probe.ts <baseUrl> <out.json> [--locale en|de]");
  ensureParityPlaywrightBrowsersPath();
  const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
  const faults: string[] = [];
  page.on("pageerror", (error) => faults.push(`pageerror: ${String(error)}`.slice(0, 300)));
  const traces: string[] = [];
  page.on("console", (message) => {
    const text = message.text();
    if (/contribut|extension-not|flow\.extension|setContributions/iu.test(text)) traces.push(`${message.type()}: ${text}`.slice(0, 300));
    if (message.type() === "error" && !/WebSocket connection to 'ws:\/\/127\.0\.0\.1:\d+\/bridge' failed/u.test(text)) faults.push(text.slice(0, 300));
  });
  const report: Record<string, unknown> = { baseUrl, locale, started: new Date().toISOString() };
  try {
    await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
    report.beacon = await awaitBeacon(page, Date.now() + 300_000);
    await dismissIntroduction(page);
    await page.waitForTimeout(3_000);
    report.seated = await seatLocale(page, locale);
    await page.keyboard.press("Escape").catch(() => undefined);
    const rows: Record<string, unknown>[] = [];
    for (const parent of PARENTS) {
      const faultCursor = faults.length;
      const opened = await openProgram(page, parent.pluginId, parent.appId);
      await page.waitForTimeout(12_000);
      const installed = await page.evaluate(() => (window as unknown as { __semioOsInstalledContributions?: () => string | null }).__semioOsInstalledContributions?.() ?? null);
      const probe = await page.evaluate(() => {
        const value = (window as unknown as { __semioOsCatalogProbe?: Probe }).__semioOsCatalogProbe;
        return value ? { plugins: value.plugins, programs: value.programs } : null;
      });
      const byPlugin = contributedBy(installed);
      const windowTexts = await page.evaluate((ids) => ids.map((id) => [id, ((document.getElementById(id)?.querySelector('[data-slot="window-body"]') as HTMLElement | null)?.innerText ?? "").replace(/\s+/gu, " ").slice(0, 400)]), opened.opened);
      const statuses = Object.fromEntries(parent.extensions.map((id) => [id, probe?.plugins.find((plugin) => plugin.pluginId === id)?.status ?? "absent"]));
      const reached = parent.extensions.filter((id) => (byPlugin[id] ?? []).length > 0);
      rows.push({
        parent: parent.pluginId,
        appId: parent.appId,
        openDetail: opened.detail,
        windows: opened.opened,
        pushedChars: installed?.length ?? 0,
        pushedPlugins: Object.keys(byPlugin).length,
        statuses,
        reached,
        missing: parent.extensions.filter((id) => !reached.includes(id)),
        topicsByExtension: Object.fromEntries(parent.extensions.map((id) => [id, byPlugin[id] ?? []])),
        faults: faults.slice(faultCursor),
        windowTexts,
        traces: traces.splice(0),
        pass: opened.opened.length > 0 && reached.length === parent.extensions.length,
      });
      console.log(`[s17] ${parent.pluginId}: opened=${opened.opened.length} reached ${reached.length}/${parent.extensions.length} pushed=${installed?.length ?? 0} faults=${faults.length - faultCursor}`);
      writeFileSync(outPath, JSON.stringify({ ...report, rows }, null, 1));
      await page.screenshot({ path: outPath.replace(/\.json$/u, `-${parent.pluginId}.png`) }).catch(() => undefined);
      await closeWindows(page, opened.opened);
      await page.waitForTimeout(1_500);
    }
    report.rows = rows;
  } catch (error) {
    report.fatal = String(error).slice(0, 500);
  } finally {
    report.finished = new Date().toISOString();
    report.faultsTail = faults.slice(-20);
    writeFileSync(outPath, JSON.stringify(report, null, 1));
    await browser.close();
  }
}

await main();
