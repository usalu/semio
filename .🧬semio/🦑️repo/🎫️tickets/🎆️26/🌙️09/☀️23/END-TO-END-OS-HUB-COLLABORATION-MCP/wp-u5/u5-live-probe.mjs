#!/usr/bin/env bun
/** 🔭️ U5 live proofs inside one running `s` (React) serve.
 *
 *   devices            — 1440×900 / 375×812 / 768×1024: measured device, horizontal overflow, footer chrome.
 *   hub <hubPid>       — sign in (ada), then SIGSTOP → SIGCONT the U5 hub: the footer badge must read
 *                        online → reconnecting → online while a rAF monitor and a chrome interaction prove the
 *                        shell never froze; the same badge re-read in German.
 *   tasks              — open the Tasks window: running-task section + the live actor table, en + de.
 *
 * Usage: bun u5-live-probe.mjs <baseUrl> <mode> [args…] */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dismissIntroduction, openPalette, seatLocale, spawnProgram } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";

const [baseUrl = "http://127.0.0.1:6580/", mode = "devices", ...rest] = process.argv.slice(2);
const out = (name) => fileURLToPath(new URL(`./generated/${name}`, import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const report = { baseUrl, mode, startedAt: new Date().toISOString(), rows: [] };

async function openShell(viewport, options = {}) {
  const context = await browser.newContext({ viewport, hasTouch: viewport.width < 1024, isMobile: viewport.width < 768, ...options });
  const page = await context.newPage();
  const lines = [];
  page.on("console", (m) => lines.push(`${m.type()}: ${m.text()}`.slice(0, 300)));
  page.on("pageerror", (e) => lines.push(`pageerror: ${String(e)}`.slice(0, 300)));
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  await page.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined || document.documentElement.dataset.semioOsError !== undefined, undefined, { timeout: 300_000 });
  await dismissIntroduction(page);
  await page.waitForTimeout(3_000);
  return { context, page, lines };
}

const badge = (page) =>
  page.evaluate(() => {
    const el = document.querySelector('[role="status"][data-semio-hub-connection]');
    return el ? { state: el.getAttribute("data-semio-hub-connection"), label: el.getAttribute("aria-label"), text: (el.textContent ?? "").trim() } : null;
  });

async function signIn(page) {
  await page.locator('[data-semio-hub-sign-in=""]').first().click({ force: true });
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 60_000 });
  for (const skip of await page.getByRole("button", { name: /^(Skip|Überspringen)$/u }).all()) await skip.click({ force: true }).catch(() => undefined);
  await form.locator('input[type="email"]').fill("ada@example.org");
  await form.locator('input[type="password"]').fill("correct horse battery staple");
  await form.locator('[id="os.hub.signIn.submit"]').click({ force: true });
  await page.waitForFunction(() => document.querySelector('[role="status"][data-semio-hub-connection]')?.getAttribute("data-semio-hub-connection") === "online", undefined, { timeout: 120_000 }).catch(() => undefined);
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(1_000);
}

async function devices() {
  for (const viewport of [{ width: 1440, height: 900 }, { width: 375, height: 812 }, { width: 768, height: 1024 }]) {
    const { context, page, lines } = await openShell(viewport);
    const state = await page.evaluate(() => ({
      device: document.querySelector("[data-ui-device]")?.getAttribute("data-ui-device") ?? null,
      touch: document.querySelector("[data-ui-device]")?.classList.contains("touch") ?? null,
      scrollWidth: document.documentElement.scrollWidth,
      clientWidth: document.documentElement.clientWidth,
      hubBadge: document.querySelector('[role="status"][data-semio-hub-connection]')?.getAttribute("data-semio-hub-connection") ?? null,
      windows: [...document.querySelectorAll("[data-window-id]")].map((el) => el.getAttribute("data-window-id")),
      panelTabs: [...document.querySelectorAll('[role="tab"]')].map((el) => (el.textContent ?? "").trim()).filter(Boolean).slice(0, 20),
      footerControls: (() => {
        const footer = document.getElementById("ui.footer");
        if (!footer) return null;
        return [...footer.querySelectorAll("button, [role='status']")].map((el) => {
          const rect = el.getBoundingClientRect();
          const probe = rect.width > 0 ? document.elementFromPoint(Math.min(rect.left + rect.width / 2, innerWidth - 1), rect.top + rect.height / 2) : null;
          return { name: (el.getAttribute("aria-label") ?? el.textContent ?? "").trim().slice(0, 40), left: Math.round(rect.left), right: Math.round(rect.right), clipped: rect.right > innerWidth + 0.5 || rect.left < -0.5, reachable: probe !== null && (el === probe || el.contains(probe)) };
        });
      })(),
    }));
    await page.screenshot({ path: out(`u5-devices-${viewport.width}x${viewport.height}.png`) });
    report.rows.push({ viewport, ...state, overflowX: state.scrollWidth > state.clientWidth, footerClipped: (state.footerControls ?? []).filter((control) => control.clipped).map((control) => control.name), footerUnreachable: (state.footerControls ?? []).filter((control) => !control.reachable).map((control) => control.name), faults: lines.filter((l) => /pageerror|Uncaught/iu.test(l)) });
    await context.close();
  }
}

async function hub(hubPid) {
  const { page, lines } = await openShell({ width: 1440, height: 900 });
  await signIn(page);
  report.rows.push({ at: "signed in", badge: await badge(page) });
  await page.evaluate(() => {
    window.__u5Frames = { last: performance.now(), maxGap: 0, frames: 0 };
    const loop = (now) => {
      const state = window.__u5Frames;
      state.maxGap = Math.max(state.maxGap, now - state.last);
      state.last = now;
      state.frames += 1;
      requestAnimationFrame(loop);
    };
    requestAnimationFrame(loop);
  });
  await page.waitForTimeout(1_000);
  const stoppedAt = Date.now();
  process.kill(Number(hubPid), "SIGSTOP");
  let reconnectingAfterMs = null;
  const seen = [];
  while (Date.now() - stoppedAt < 25_000) {
    const current = await badge(page);
    if (seen.at(-1) !== current?.state) seen.push(current?.state);
    if (current?.state === "reconnecting") {
      reconnectingAfterMs = Date.now() - stoppedAt;
      break;
    }
    await page.waitForTimeout(250);
  }
  const during = await badge(page);
  const tasksTab = page.locator('[id="os.task-manager"]').first();
  const clickStarted = Date.now();
  await tasksTab.click({ timeout: 5_000 }).catch(() => undefined);
  const windowShown = await page.waitForSelector("[data-semio-task-manager-window]", { timeout: 5_000 }).then(() => Date.now() - clickStarted).catch(() => null);
  await page.screenshot({ path: out("u5-hub-reconnecting.png") });
  const frames = await page.evaluate(() => window.__u5Frames);
  report.rows.push({ at: "hub SIGSTOP", reconnectingAfterMs, transitions: seen, badge: during, tasksWindowShownAfterMs: windowShown, rafMaxGapMs: Math.round(frames.maxGap), rafFrames: frames.frames });
  const resumedAt = Date.now();
  process.kill(Number(hubPid), "SIGCONT");
  let onlineAfterMs = null;
  while (Date.now() - resumedAt < 30_000) {
    if ((await badge(page))?.state === "online") {
      onlineAfterMs = Date.now() - resumedAt;
      break;
    }
    await page.waitForTimeout(250);
  }
  report.rows.push({ at: "hub SIGCONT", onlineAfterMs, badge: await badge(page) });
  const seated = await seatLocale(page, "de");
  report.rows.push({ at: `locale ${seated}`, badge: await badge(page) });
  process.kill(Number(hubPid), "SIGSTOP");
  const stoppedDe = Date.now();
  let reconnectingDe = null;
  while (Date.now() - stoppedDe < 25_000) {
    const current = await badge(page);
    if (current?.state === "reconnecting") {
      reconnectingDe = current;
      break;
    }
    await page.waitForTimeout(250);
  }
  process.kill(Number(hubPid), "SIGCONT");
  report.rows.push({ at: "de hub SIGSTOP", badge: reconnectingDe, afterMs: Date.now() - stoppedDe });
  await page.screenshot({ path: out("u5-hub-reconnecting-de.png") });
  report.console = lines.filter((l) => /error|pageerror/iu.test(l)).slice(-30);
}

async function tasks() {
  const { page, lines } = await openShell({ width: 1440, height: 900 });
  for (const locale of ["en", "de"]) {
    if (locale === "de") report.rows.push({ at: `locale ${await seatLocale(page, "de")}` });
    await page.locator('[id="os.task-manager"]').first().click({ timeout: 10_000 }).catch(() => undefined);
    await page.waitForSelector("[data-semio-task-manager-window]", { timeout: 20_000 }).catch(() => undefined);
    await page.waitForTimeout(2_500);
    const state = await page.evaluate(() => {
      const root = document.querySelector("[data-semio-task-manager-window]");
      return root === null
        ? null
        : {
            sections: [...root.querySelectorAll("section")].map((section) => section.getAttribute("aria-label")),
            tasks: root.querySelector("[data-semio-task-manager-tasks]")?.getAttribute("data-semio-task-manager-tasks") ?? null,
            tasksEmpty: (root.querySelector("[data-semio-task-manager-tasks-empty]")?.textContent ?? "").trim(),
            actorEmpty: root.querySelector("[data-semio-task-manager-empty]")?.getAttribute("data-semio-task-manager-empty") ?? null,
            actorRows: root.querySelectorAll("tbody tr").length,
            headers: [...root.querySelectorAll("th")].map((th) => (th.textContent ?? "").trim()).slice(0, 12),
            firstRows: [...root.querySelectorAll("tbody tr")].slice(0, 5).map((tr) => [...tr.querySelectorAll("td")].slice(0, 4).map((td) => (td.textContent ?? "").trim()).join(" | ")),
            cancelButtons: [...root.querySelectorAll("button[aria-label]")].map((button) => button.getAttribute("aria-label")).slice(0, 6),
          };
    });
    await page.screenshot({ path: out(`u5-tasks-${locale}.png`) });
    report.rows.push({ at: `tasks ${locale}`, state });
  }
  report.console = lines.filter((l) => /error|pageerror/iu.test(l)).slice(-30);
}

/** 🚀️ Spawns one exact app (`spawn.<pluginId>.<appId>`) from the command palette. */
async function spawnApp(page, pluginId, appId) {
  const before = await page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((el) => el.getAttribute("data-window-id")));
  await openPalette(page);
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 20_000 }).catch(() => undefined);
  await input.fill(/^s\.[^.]+\.([^@]+)@/u.exec(appId)?.[1] ?? pluginId);
  const item = page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${pluginId}.${appId}"]`).first();
  await item.waitFor({ state: "visible", timeout: 20_000 }).catch(() => undefined);
  if ((await item.count()) === 0) return { windowIds: [], detail: `no spawn.${pluginId}.${appId} palette entry` };
  await item.click({ force: true });
  const deadline = Date.now() + 180_000;
  while (Date.now() < deadline) {
    const now = await page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((el) => el.getAttribute("data-window-id")));
    const opened = now.filter((id) => !before.includes(id));
    if (opened.length > 0) return { windowIds: opened, detail: null };
    await page.waitForTimeout(250);
  }
  return { windowIds: [], detail: "no new window" };
}

async function jobs(pluginId, appId) {
  const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
  const page = await context.newPage();
  const lines = [];
  const taskEvents = [];
  page.on("console", (m) => lines.push(`${m.type()}: ${m.text()}`.slice(0, 300)));
  await page.exposeFunction("__u5Task", (row) => taskEvents.push({ at: new Date().toISOString().slice(11, 23), ...row }));
  await page.addInitScript(() => {
    const seen = new Set();
    const record = () => {
      for (const row of document.querySelectorAll("[data-semio-task-manager-task]")) {
        const bar = row.querySelector("[role='progressbar']");
        const entry = { t: Math.round(performance.now()), id: row.getAttribute("data-semio-task-manager-task"), lane: row.getAttribute("data-semio-task-manager-lane"), state: row.getAttribute("data-semio-task-manager-state"), progress: bar?.getAttribute("aria-valuetext") ?? null, label: bar?.getAttribute("aria-label") ?? null };
        const key = JSON.stringify({ ...entry, t: 0 });
        if (seen.has(key)) continue;
        seen.add(key);
        window.__u5Task?.(entry);
      }
    };
    const start = () => new MutationObserver(record).observe(document.documentElement, { subtree: true, childList: true, attributes: true, attributeFilter: ["aria-valuetext", "data-semio-task-manager-state"] });
    if (document.documentElement) start();
    else document.addEventListener("DOMContentLoaded", start);
  });
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  await page.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined, undefined, { timeout: 300_000 });
  await dismissIntroduction(page);
  await page.waitForTimeout(3_000);
  await page.locator('[id="os.task-manager"]').first().click({ timeout: 10_000 }).catch(() => undefined);
  await page.waitForSelector("[data-semio-task-manager-window]", { timeout: 20_000 }).catch(() => undefined);
  const spawned = appId ? await spawnApp(page, pluginId, appId) : await spawnProgram(page, pluginId);
  report.rows.push({ at: `spawn ${pluginId}`, spawned });
  if ((await page.locator("[data-semio-task-manager-window]").count()) === 0) await page.locator('[id="os.task-manager"]').first().click({ timeout: 10_000 }).catch(() => undefined);
  const firstJob = await page.waitForSelector("[data-semio-task-manager-lane='job']", { timeout: 45_000 }).then(() => true).catch(() => false);
  await page.screenshot({ path: out(`u5-jobs-${pluginId}-running.png`) });
  let cancelled = null;
  if (firstJob) {
    const id = await page.locator("[data-semio-task-manager-lane='job']").first().getAttribute("data-semio-task-manager-task");
    await page.locator(`[id="os.task-manager.cancel.${id}"]`).click({ timeout: 5_000 }).catch(() => undefined);
    const gone = await page.waitForFunction((jobId) => !document.querySelector(`[data-semio-task-manager-task="${jobId}"]`), id, { timeout: 30_000 }).then(() => true).catch(() => false);
    cancelled = { id, gone };
  }
  const actors = await page.evaluate(() => [...document.querySelectorAll("[data-semio-task-manager-window] tbody tr")].map((tr) => [...tr.querySelectorAll("td")].slice(0, 4).map((td) => (td.textContent ?? "").trim()).join(" | ")));
  report.rows.push({ at: "job rows", firstJob, cancelled, actors, taskEvents });
  report.console = lines.filter((l) => /pageerror|fault|trap|job/iu.test(l)).slice(-40);
}

if (mode === "jobs") await jobs(rest[0] ?? "fem", rest[1]);
if (mode === "devices") await devices();
if (mode === "hub") await hub(rest[0]);
if (mode === "tasks") await tasks();
writeFileSync(out(`u5-live-${mode}${rest[0] ? `-${rest[0]}` : ""}.json`), JSON.stringify(report, null, 1));
await browser.close();
for (const row of report.rows) console.log(JSON.stringify(row).slice(0, 700));
