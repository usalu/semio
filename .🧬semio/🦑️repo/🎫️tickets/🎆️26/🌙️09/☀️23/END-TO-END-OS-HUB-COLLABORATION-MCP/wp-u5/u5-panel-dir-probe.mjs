#!/usr/bin/env bun
/** ↔️ U5 S12 — reading direction of panel CONTENT: opens each bottom-right window (Settings, Marketplace, History,
 * Tasks) and records the computed `direction` of its content root and of a text line, with a screenshot each.
 * Usage: bun u5-panel-dir-probe.mjs <baseUrl> <tag> */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dismissIntroduction } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";

const [baseUrl, tag] = process.argv.slice(2);
const out = (name) => fileURLToPath(new URL(`./generated/${name}`, import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined, undefined, { timeout: 300_000 });
await dismissIntroduction(page);
await page.waitForTimeout(2_000);
const rows = [];
for (const id of ["os.task-manager", "framework.settings", "os.marketplace", "framework.panel.history", "framework.chat"]) {
  const tab = page.locator(`[id="${id}"], [data-panel-tab-id="${id}"]`).first();
  const present = await tab.count();
  if (present) await tab.click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(1_500);
  const anchor = id === "framework.chat" ? "top-right" : "bottom-right";
  const state = await page.evaluate((anchor) => {
    const panel = document.querySelector(`[data-slot="panel"][data-anchor="${anchor}"][data-panel-visible="true"]`);
    const body = panel?.querySelector('[data-slot="panel-content"]') ?? panel;
    const firstText = body ? [...body.querySelectorAll("p, td, th, span, h3")].find((element) => (element.textContent ?? "").trim().length > 3) : null;
    return panel ? { panelDir: panel.getAttribute("dir"), contentDirection: body ? getComputedStyle(body).direction : null, text: (firstText?.textContent ?? "").trim().slice(0, 60), textDirection: firstText ? getComputedStyle(firstText).direction : null, scrollOverflow: body ? body.scrollWidth - body.clientWidth : null } : null;
  }, anchor);
  rows.push({ id, present, state });
  await page.screenshot({ path: out(`u5-panel-dir-${tag}-${id.replaceAll(".", "-")}.png`) });
}
writeFileSync(out(`u5-panel-dir-${tag}.json`), JSON.stringify(rows, null, 1));
for (const row of rows) console.log(JSON.stringify(row));
await browser.close();
