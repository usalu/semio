/** 🔬️ S18 14c: opens one stdio editor (palette), runs its setActiveExample pre-verb, dumps the rendered inline-edit
 * inputs (table `cell-*`, tree `edit`) and optionally commits one to see whether the revision-bound verb applies.
 * usage: bun s18-probe-inline-edit.mjs <url> <kind e.g. csv|json|xml> [subsetNeedle] [commitValue] */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
const [url, kind, subset = "", commitValue = ""] = process.argv.slice(2);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
const out = [];
const log = (line) => { out.push(line); console.log(line); };
try {
  const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: process.env.S18_LOCALE ?? "en-US" })).newPage();
  page.on("pageerror", (error) => log(`pageerror ${String(error).slice(0, 300)}`));
  page.on("console", (message) => { const t = message.text(); if (/refused|dispatch-failed|revision|set-cell|set-node|fault/iu.test(t)) log(`console.${message.type()} ${t.slice(0, 300)}`); });
  await page.goto(url, { waitUntil: "domcontentloaded", timeout: 240_000 });
  await page.locator('[data-ui-node-key="s-home-create-space"]').first().waitFor({ state: "attached", timeout: 240_000 });
  await page.waitForTimeout(8_000);
  const programs = await page.evaluate(() => (window.__semioOsCatalogProbe?.programs ?? []).filter((p) => p.pluginId === "stdio").map((p) => p.appId));
  const appId = programs.find((id) => id.includes(`.${kind}@`) && id.endsWith("#editor") && (subset === "" ? /\/\*#|@[^/]+\/any#|\/✳️/u.test(id) || !id.includes("/") || true : id.includes(`/${subset}#`)));
  const candidates = programs.filter((id) => id.includes(`.${kind}@`) && id.endsWith("#editor"));
  log(`candidates ${JSON.stringify(candidates)}`);
  const chosen = subset === "" ? candidates.find((id) => /\/\*#/u.test(id)) ?? candidates[0] : candidates.find((id) => id.includes(`/${subset}#`));
  log(`chosen ${chosen} (first match ${appId})`);
  await page.evaluate(() => { if (document.activeElement instanceof HTMLElement) document.activeElement.blur(); document.body.focus(); });
  await page.keyboard.press("Meta+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 15_000 });
  await input.fill(kind);
  await page.waitForTimeout(1_200);
  await page.locator(`[data-slot="command-item"][data-command-item-id="spawn.stdio.${chosen}"]`).first().click({ timeout: 8_000 });
  await page.waitForTimeout(9_000);
  const toggles = await page.evaluate(() => [...document.querySelectorAll('[id$=".engagement.toggle"]')].map((t) => t.id));
  for (const id of toggles) await page.locator(`[id="${id}"]`).first().click({ force: true, timeout: 5_000 }).catch(() => undefined);
  await page.waitForTimeout(2_000);
  log(`rail ${JSON.stringify(await page.evaluate(() => [...new Set([...document.querySelectorAll('[data-slot="window-action-pane"] [id^="action."]')].map((e) => e.id))].slice(0, 60)))}`);
  await page.locator('[data-slot="window-action-pane"] [id="action.setActiveExample"]').first().click({ force: true, timeout: 8_000 }).catch((e) => log(`pre click ${String(e).slice(0, 100)}`));
  await page.waitForTimeout(800);
  const exec = page.locator('[data-slot="window-action-pane"] [id$=".action.setActiveExample.execute"], [id$="setActiveExample.execute"]').first();
  if ((await exec.count()) > 0) await exec.click({ force: true, timeout: 8_000 }).catch(() => undefined);
  await page.waitForTimeout(6_000);
  const dump = () => page.evaluate(() => [...document.querySelectorAll('[data-slot="window-body"] input, [data-slot="window-body"] textarea')].filter((e) => !e.closest('[data-slot="window-action-pane"]')).slice(0, 12).map((e) => ({ key: e.getAttribute("data-ui-node-key"), node: e.getAttribute("data-ui-node-id"), id: e.id.slice(0, 120), label: e.getAttribute("aria-label"), value: e.value.slice(0, 40), row: e.closest("[data-row-id],[data-table-row-key],tr")?.getAttribute("data-row-id") ?? e.closest("tr")?.getAttribute("data-ui-node-key") ?? null, item: e.closest('[role="treeitem"]')?.getAttribute("data-ui-node-key") ?? null })));
  const before = await dump();
  log(`inputs ${JSON.stringify(before, null, 0)}`);
  log(`checkin ${await page.evaluate(() => document.querySelector("#s-checkin")?.textContent ?? null)}`);
  log(`surfaces ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[data-slot="window-body"] [data-surface-id]')].map((e) => `${e.getAttribute("data-surface-id")}|${e.className.slice(0, 40)}|ta=${e.querySelectorAll("textarea").length}|btn=${e.querySelectorAll("button").length}`)))}`);
  log(`windows ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((w) => w.getAttribute("data-window-id"))))}`);
  log(`treeitems ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[data-slot="window-body"] [role="treeitem"]')].slice(0, 14).map((t) => `${t.getAttribute("aria-expanded")}|${t.getAttribute("data-ui-node-key")}|${(t.getAttribute("aria-label") ?? t.textContent ?? "").slice(0, 40)}|inputs=${t.querySelectorAll("input").length}`)))}`);
  const history = () => page.evaluate(() => [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((e) => !e.id.endsWith(".revert")).map((e) => `${e.id}|${e.textContent?.trim().slice(0, 50)}`));
  await page.locator('[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]').first().click({ force: true, timeout: 5_000 }).catch(() => undefined);
  await page.waitForTimeout(1_500);
  log(`history-before ${JSON.stringify(await history())}`);
  if (commitValue !== "") {
    const selector = process.env.S18_CONTROL ?? `[id="${before[0].id}"]`;
    const target = page.locator(`[data-slot="window-body"] ${selector}`).first();
    log(`control ${selector} count=${await target.count()} tag=${await target.evaluate((e) => e.tagName).catch(() => "?")}`);
    const current = await target.inputValue().catch(() => "");
    const next = commitValue.startsWith("+") ? `${current}${commitValue.slice(1)}` : commitValue.startsWith("s|") ? current.replace(commitValue.split("|")[1], commitValue.split("|")[2]) : commitValue;
    log(`typed ${JSON.stringify(next)}`);
    await target.click({ force: true });
    await target.fill(next);
    if (process.env.S18_APPLY) {
      await page.waitForTimeout(600);
      const apply = page.locator(`[data-slot="window-body"] button`, { hasText: new RegExp(`^${process.env.S18_APPLY}$`, "u") }).first();
      log(`apply count=${await apply.count()} disabled=${await apply.isDisabled().catch(() => "?")}`);
      await apply.click({ force: true, timeout: 5_000 }).catch((e) => log(`apply ${String(e).slice(0, 100)}`));
    } else await target.press("Enter");
    await page.waitForTimeout(6_000);
    log(`history-after ${JSON.stringify(await history())}`);
    log(`after-commit value ${JSON.stringify((await target.inputValue().catch(() => "?")).slice(-60))}`);
  }
} catch (error) {
  log(`FAIL ${String(error).slice(0, 400)}`);
} finally {
  await browser.close();
}
