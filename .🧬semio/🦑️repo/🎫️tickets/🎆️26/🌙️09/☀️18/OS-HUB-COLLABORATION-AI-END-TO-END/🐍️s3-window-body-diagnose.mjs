#!/usr/bin/env bun
/** 🩻️ S3 (session 10) — where a spawned program's window BODY lives in the `s` DOM, and what it holds:
 * spawns one program from Home and dumps every `window:*` id, every `[data-slot="window-body"]` with its
 * ancestry attributes, and the body's own `[data-ui-node-key]` / canvas / surface content.
 *
 * Usage: bun 🐍️s3-window-body-diagnose.mjs <baseUrl> <pluginId[=appId]> */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { awaitBeacon, dismissIntroduction, openPalette, windowIds } from "./🐍️s6-all-kinds-sweep.mjs";

const baseUrl = process.argv[2] ?? "http://127.0.0.1:6400/";
const [pluginId, appId] = (process.argv[3] ?? "dag").split("=");
const out = fileURLToPath(new URL("./wp-s3/generated/s3-window-body-diagnose.txt", import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const result = { console: [] };
page.on("console", (message) => { if (message.type() !== "debug" && !/DevTools|404/u.test(message.text())) result.console.push(`${message.type()}: ${message.text()}`.slice(0, 400)); });
page.on("pageerror", (error) => result.console.push(`pageerror: ${String(error)}`.slice(0, 400)));
try {
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  result.beacon = await awaitBeacon(page, Date.now() + 300_000);
  await dismissIntroduction(page);
  await page.waitForTimeout(5_000);
  const before = await windowIds(page);
  await openPalette(page);
  await page.locator("[role='dialog'] [data-slot='command-input']").first().fill(appId ? (/^s\.[^.]+\.([^@]+)@/u.exec(appId)?.[1] ?? pluginId) : pluginId);
  await page.waitForTimeout(1_500);
  await page.locator(`[data-slot="command-item"][data-command-item-id="${appId ? `spawn.${pluginId}.${appId}` : `spawn.${pluginId}`}"], [data-slot="command-item"][data-command-item-id="spawn.${pluginId}"]`).first().click({ force: true });
  const deadline = Date.now() + 90_000;
  while (Date.now() < deadline && (await windowIds(page)).every((id) => before.includes(id))) await page.waitForTimeout(250);
  await page.waitForTimeout(8_000);
  result.shellError = await page.evaluate(() => ({ dataset: { ...document.documentElement.dataset }, alerts: [...document.querySelectorAll('[role="alert"], [data-semio-window-fault], [data-semio-transient-notice]')].map((element) => (element.textContent ?? "").trim().slice(0, 300)) }));
  result.dump = await page.evaluate(() => {
    const attrs = (element) => Object.fromEntries([...element.attributes].filter((attr) => attr.name !== "class" && attr.name !== "style").map((attr) => [attr.name, attr.value.slice(0, 80)]));
    return {
      windowIdHosts: [...document.querySelectorAll("[data-window-id]")].map((element) => ({ tag: element.tagName, attrs: attrs(element), children: element.children.length, rect: element.getBoundingClientRect().toJSON() })),
      windowColonIds: [...document.querySelectorAll('[id^="window:"]')].map((element) => element.id).slice(0, 60),
      bodies: [...document.querySelectorAll('[data-slot="window-body"]')].map((body) => ({
        attrs: attrs(body),
        ancestors: (() => { const chain = []; let node = body.parentElement; while (node && chain.length < 6) { chain.push(attrs(node)); node = node.parentElement; } return chain; })(),
        rect: body.getBoundingClientRect().toJSON(),
        uiNodeKeys: [...body.querySelectorAll("[data-ui-node-key]")].map((element) => element.getAttribute("data-ui-node-key")).slice(0, 20),
        surfaces: [...body.querySelectorAll("[data-surface-id]")].map((element) => element.getAttribute("data-surface-id")),
        canvases: [...body.querySelectorAll("canvas")].map((canvas) => `${canvas.width}x${canvas.height}`),
        graphHosts: body.querySelectorAll(".semio-node-graph-host").length,
        directChildren: [...body.children].map((child) => `${child.tagName}${child.getAttribute("data-slot") ? `[${child.getAttribute("data-slot")}]` : ""}${child.id ? `#${child.id}` : ""}`),
        text: (body.innerText ?? "").replace(/\s+/gu, " ").slice(0, 200),
        paneHost: (() => { const host = body.querySelector('[data-slot="pane-host-root"]'); return host === null ? null : { attrs: attrs(host), html: host.outerHTML.slice(0, 600), rect: host.getBoundingClientRect().toJSON() }; })(),
      })),
      scrollSurfaces: [...document.querySelectorAll('[id^="framework.window."]')].filter((element) => !/\.(engagement|utilityBar|measures|search)/u.test(element.id)).map((element) => ({ id: element.id, inBody: element.closest('[data-slot="window-body"]') !== null, children: element.children.length, html: element.outerHTML.slice(0, 300) })).slice(0, 12),
    };
  });
  await page.screenshot({ path: fileURLToPath(new URL("./wp-s3/generated/s3-window-body-diagnose.png", import.meta.url)) });
} catch (error) {
  result.fatal = String(error);
} finally {
  await browser.close();
}
writeFileSync(out, JSON.stringify(result, null, 2));
console.log(JSON.stringify(result, null, 1).slice(0, 6000));
