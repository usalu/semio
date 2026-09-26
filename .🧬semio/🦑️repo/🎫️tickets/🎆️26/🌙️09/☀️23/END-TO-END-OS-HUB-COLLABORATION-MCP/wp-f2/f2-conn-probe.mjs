#!/usr/bin/env bun
/** 🔌️ F2 — per-origin connection inventory of the `s` React shell.
 *
 * Boots `s` in Chromium with a NetLog, then opens N programs one after the other (palette chord, like F1's census). After
 * every step: every request still open per origin and resource type (Playwright request events, page + dedicated workers +
 * service worker), the browser network process's ESTABLISHED TCP sockets to the serve port (`lsof`), and a probe fetch of a
 * tiny same-origin file (a queued request shows as a multi-second probe). The NetLog is reduced afterwards by
 * `f2-netlog-inventory.py` (per-origin concurrency over time, every request's queue time, socket-pool stall events).
 *
 * usage: bun f2-conn-probe.mjs <baseUrl> <tag> [--programs n] [--only plugin,...] [--sign-in] [--headed] */
import { chromium } from "playwright";
import { execFileSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { openProgramByPalette, settleMain, sleep } from "../wp-f1/f1-lib.mjs";

const argv = process.argv.slice(2);
const baseUrl = argv[0] ?? "http://127.0.0.1:6580/";
const tag = argv[1] ?? "adhoc";
const valueOf = (flag, fallback) => (argv.includes(flag) ? argv[argv.indexOf(flag) + 1] : fallback);
const programCount = Number(valueOf("--programs", "8"));
const only = valueOf("--only", "").split(",").filter(Boolean);
const generated = fileURLToPath(new URL("./generated/", import.meta.url));
const netlog = `${generated}f2-netlog-${tag}.json`;
const port = new URL(baseUrl).port;
const log = (...parts) => console.log(`[f2 ${new Date().toISOString().slice(11, 19)}]`, ...parts);
const sweep = await import("/Users/ueli/Documents/semio/.tmp-ticket-0918/🐍️s6-all-kinds-sweep.mjs");

const browser = await chromium.launch({ headless: !argv.includes("--headed"), args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist", `--log-net-log=${netlog}`, "--net-log-capture-mode=Default"] });
const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
const page = await context.newPage();
page.setDefaultNavigationTimeout(300_000);
const cdp = await context.newCDPSession(page);
await cdp.send("Performance.enable", { timeDomain: "timeTicks" });

const open = new Map();
const t0 = Date.now();
const track = (request) => {
  const url = new URL(request.url());
  if (!url.protocol.startsWith("http") && !url.protocol.startsWith("ws")) return;
  open.set(request, { origin: url.origin, path: decodeURIComponent(url.pathname).slice(0, 90), type: request.resourceType(), at: Date.now() - t0 });
};
const settle = (request) => open.delete(request);
context.on("request", track);
context.on("requestfinished", settle);
context.on("requestfailed", settle);
const mux = { sockets: 0, open: new Map(), opened: 0, ended: 0, cancelled: 0 };
page.on("websocket", (socket) => {
  const url = new URL(socket.url());
  if (url.pathname === "/semio-stream-mux") {
    mux.sockets += 1;
    const routes = new Map();
    socket.on("framesent", ({ payload }) => {
      const frame = JSON.parse(String(payload));
      if (frame.kind === "open") {
        routes.set(frame.stream, frame.route);
        mux.open.set(`${mux.sockets}:${frame.stream}`, frame.route);
        mux.opened += 1;
      } else if (frame.kind === "cancel") {
        mux.open.delete(`${mux.sockets}:${frame.stream}`);
        mux.cancelled += 1;
      }
    });
    socket.on("framereceived", ({ payload }) => {
      const frame = JSON.parse(String(payload));
      if (frame.kind === "end") {
        mux.open.delete(`${mux.sockets}:${frame.stream}`);
        mux.ended += 1;
      }
    });
    socket.on("close", () => {
      for (const key of [...mux.open.keys()]) if (key.startsWith(`${mux.sockets}:`)) mux.open.delete(key);
    });
  }
  const key = { url: socket.url() };
  open.set(key, { origin: url.origin, path: decodeURIComponent(url.pathname).slice(0, 90), type: "websocket", at: Date.now() - t0 });
  socket.on("close", () => open.delete(key));
});

/** 🧭️ Every client-side TCP socket to the serve port (`…->127.0.0.1:<port>`): only this probe's browser talks to the slice's serve. */
const socketsTo = () => {
  try {
    const text = execFileSync("lsof", ["-nP", `-iTCP@127.0.0.1:${port}`, "-sTCP:ESTABLISHED"], { encoding: "utf8", maxBuffer: 64 << 20 });
    return text.split("\n").filter((line) => line.includes(`->127.0.0.1:${port} `)).length;
  } catch {
    return 0;
  }
};

const probeFetch = () =>
  page.evaluate(async () => {
    const started = performance.now();
    const controller = new AbortController();
    const timer = setTimeout(() => controller.abort(), 20_000);
    try {
      const response = await fetch(`/favicon.ico?f2=${Math.random()}`, { cache: "no-store", signal: controller.signal });
      await response.arrayBuffer();
      return { ms: +(performance.now() - started).toFixed(1), status: response.status };
    } catch (error) {
      return { ms: +(performance.now() - started).toFixed(1), status: controller.signal.aborted ? "queued>20s" : String(error).slice(0, 80) };
    } finally {
      clearTimeout(timer);
    }
  });

const inventory = () => {
  const rows = [...open.values()].filter((row) => Date.now() - t0 - row.at > 3_000);
  const byOrigin = {};
  for (const row of rows) {
    const origin = (byOrigin[row.origin] ??= { held: 0, rows: [] });
    origin.held += row.type === "websocket" ? 0 : 1;
    origin.rows.push(`${row.type} ${row.path}`);
  }
  return byOrigin;
};

const steps = [];
const snapshot = async (label) => {
  const byRoute = {};
  for (const route of mux.open.values()) byRoute[route] = (byRoute[route] ?? 0) + 1;
  const row = { label, t: Date.now() - t0, inventory: inventory(), sockets: socketsTo(), probe: await probeFetch(), mux: { sockets: mux.sockets, openStreams: byRoute, opened: mux.opened, cancelled: mux.cancelled, ended: mux.ended } };
  steps.push(row);
  const serve = row.inventory[new URL(baseUrl).origin];
  log(label, `held=${serve?.held ?? 0}`, `sockets=${row.sockets}`, `probe=${row.probe.ms}ms/${row.probe.status}`, `mux=${JSON.stringify(row.mux)}`, JSON.stringify(serve?.rows ?? []).slice(0, 300));
};

await page.goto(baseUrl, { waitUntil: "commit" });
const beacon = await sweep.awaitBeacon(page, Date.now() + 300_000);
await sweep.dismissIntroduction(page);
await page.keyboard.press("Escape").catch(() => undefined);
await settleMain(cdp, 20_000);
log("beacon", beacon);
await snapshot("home");
let spaces = [];
if (argv.includes("--sign-in")) {
  const env = Object.fromEntries(readFileSync(valueOf("--env", "/Users/ueli/Documents/semio/.tmp-ticket/wp-u5/u5-7800.env"), "utf8").split("\n").filter((line) => line.includes("=")).map((line) => [line.slice(0, line.indexOf("=")), line.slice(line.indexOf("=") + 1)]));
  await page.locator("[data-semio-hub-sign-in]").first().click();
  const workspace = page.locator("[data-semio-hub-workspace]");
  await workspace.waitFor({ state: "visible", timeout: 60_000 });
  await workspace.locator('input[type="email"]').fill(env.U5_EMAIL);
  await workspace.locator('input[type="password"]').fill(env.U5_PASSWORD);
  await workspace.locator('form:has(input[type="password"]) button[type="submit"]').first().click();
  await page.waitForFunction(() => /Signed in as|Angemeldet als/u.test(document.querySelector("[data-semio-hub-workspace]")?.textContent ?? ""), undefined, { timeout: 180_000 });
  const deadline = Date.now() + 120_000;
  while (Date.now() < deadline && spaces.length === 0) {
    spaces = await page.evaluate(() => [...document.querySelectorAll("[data-semio-hub-workspace] li[data-space-id]")].map((row) => ({ id: row.getAttribute("data-space-id"), text: (row.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 60) })));
    if (spaces.length === 0) await sleep(1_000);
  }
  log("spaces", spaces.length, JSON.stringify(spaces.slice(0, 6)));
  await page.locator('[id="os.hub.signIn.cancel"]').click({ force: true }).catch(() => undefined);
  await sleep(4_000);
  await snapshot("signed-in");
}
const hubDocs = Number(valueOf("--hub", "0"));
if (hubDocs > 0 && spaces.length > 0) {
  const wanted = new RegExp(valueOf("--space", "S15 Space"), "u");
  const space = spaces.find((row) => wanted.test(row.text)) ?? spaces[0];
  await page.goto(new URL(`spaces/${space.id}`, baseUrl).toString(), { waitUntil: "commit" });
  await sweep.awaitBeacon(page, Date.now() + 300_000);
  await page.locator('[data-ui-node-key="s-space-create-artifact"]').first().waitFor({ state: "attached", timeout: 240_000 }).catch(() => undefined);
  const artifactRows = () => page.locator('[data-ui-node-key^="artifact:"]').evaluateAll((elements) => elements.map((element) => element.getAttribute("data-ui-node-key").slice("artifact:".length)));
  let rows = [];
  const deadline = Date.now() + 120_000;
  while (Date.now() < deadline && rows.length === 0) {
    rows = await artifactRows();
    if (rows.length === 0) await sleep(1_000);
  }
  const windowsNow = () => page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")));
  const indexWindows = await windowsNow();
  await snapshot(`space ${space.text.slice(0, 40)} → ${rows.length} artifact rows, windows ${indexWindows.join(",")}`);
  for (const id of rows.slice(0, hubDocs)) {
    const before = await windowsNow();
    const buttons = page.locator(`[data-ui-node-key="artifact:${id}"] button`);
    let pressed = false;
    for (let index = 0, count = await buttons.count(); index < count && !pressed; index += 1) {
      const button = buttons.nth(index);
      const name = `${(await button.getAttribute("aria-label")) ?? ""} ${(await button.getAttribute("title")) ?? ""} ${(await button.textContent()) ?? ""}`;
      if (/^\s*(open|öffnen)\b/iu.test(name.trim()) || /\b(open|öffnen)\b/iu.test(name)) {
        await button.focus();
        await button.press("Enter");
        pressed = true;
      }
    }
    const openDeadline = Date.now() + 180_000;
    let opened = [];
    while (pressed && Date.now() < openDeadline && opened.length === 0) {
      opened = (await windowsNow()).filter((windowId) => !before.includes(windowId));
      if (opened.length === 0) await sleep(1_000);
    }
    await sleep(5_000);
    await snapshot(`hub doc ${id.slice(0, 8)} → ${pressed ? `${opened.length} window(s)` : "no open action"}`);
    for (const windowId of indexWindows) await page.locator(`[data-slot="mode-dock-tab"][data-window-id="${windowId}"]`).first().click({ force: true }).catch(() => undefined);
    await sleep(1_500);
  }
}
const probe = await page.evaluate(() => window.__semioOsCatalogProbe ?? null);
const programs = (probe?.programs ?? []).filter((program) => program.appId.endsWith("#editor") && (only.length === 0 || only.includes(program.pluginId)));
const seen = new Set();
const chosen = programs.filter((program) => (seen.has(program.pluginId) ? false : seen.add(program.pluginId))).slice(0, programCount);
for (const program of chosen) {
  const opened = await openProgramByPalette(page, program);
  await sleep(2_000);
  await snapshot(`open ${program.pluginId} ${program.appId.split("@")[0]} → ${opened.windowIds.length} window(s)${opened.detail ? ` (${opened.detail})` : ""}`);
}
await sleep(5_000);
await snapshot("settled");
await browser.close();
const out = `${generated}f2-conn-${tag}.json`;
writeFileSync(out, JSON.stringify({ baseUrl, tag, beacon, netlog, steps }, null, 1));
log("wrote", out);
