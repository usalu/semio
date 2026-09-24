#!/usr/bin/env bun
/** 🛸️ S3 (session 10) — G11 gaps G1 + G3: foreign kinds opened INSIDE one running `s` session, from the
 * space Home, each judged spawn → render → one Actions-rail mutation → undo → redo → 0 fault lines.
 *
 * One page, one session, no reload between kinds. Before the first open it reads the product's own plugin
 * census twice — the served `PLAYGROUND_SESSION` module (`virtual:semio-playground-session`) and the shell's
 * `window.__semioOsCatalogProbe` load status — and names every session plugin that is not `loaded`.
 *
 * Every open is attempted from the Home landing window FIRST (the palette's `spawn.*` rows are shell chrome
 * since S3/S4's `dropped action "spawnApp"` fix); only a Home open that yields neither a window NOR a visible
 * refusal notice falls back to the studio, and that fallback is recorded, never hidden — a localized refusal
 * is an answer, a silent drop is the defect. Per kind the probe keeps the console lines, the
 * shell's transient notices (`[data-semio-transient-notice]`, recorded by an init-script observer so a 4 s
 * notice cannot be missed between polls) and every plugin-module request, split by phase (boot vs open).
 *
 * The rail/ledger witness is the shared one from `🐍️s6-all-kinds-sweep.mjs` (history ledger + `#s-checkin`
 * count + structural render digest), so a row here means what a sweep row means.
 *
 * Usage: bun 🐍️s3-foreign-kind-in-s.mjs <baseUrl> [--tag <tag>] [--out <ticket-relative dir>] <pluginId[=appId]...>
 *   e.g. raster dag   ·   block=s.block.block2d@1/*#editor block=s.block.block3d@1/*#editor
 *   --out                                   capture folder inside this ticket (default `🗑️generated`)
 *   --locale <tag>                          seat the shell locale through Settings before the first open
 *   --example                               seat the program's first example through its own rail
 *                                           (`setActiveExample`) before judging the render
 *   S3_SIGN_IN_EMAIL / S3_SIGN_IN_PASSWORD  optional hub sign-in before the first open
 */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { FAULT, LIVE_ID, NOISE, awaitBeacon, clickUncovered, closeWindows, dismissIntroduction, enterStudio, fillStagedArgument, mutateUndoRedo, neutralDispatch, openPalette, readProbe, readShell, seatLocale, signIn, submitStagedVerb, unfoldActionsRail, windowIds } from "./🐍️s6-all-kinds-sweep.mjs";

const argv = process.argv.slice(2);
const baseUrl = argv[0] ?? "http://127.0.0.1:6070/";
const valueOf = (flag, fallback) => (argv.includes(flag) ? argv[argv.indexOf(flag) + 1] : fallback);
const tag = valueOf("--tag", "adhoc");
const outDir = valueOf("--out", "🗑️generated");
const locale = valueOf("--locale", "en");
const flagValues = new Set(["--tag", "--out", "--locale"].filter((flag) => argv.includes(flag)).map((flag) => argv.indexOf(flag) + 1));
const targets = argv.slice(1).filter((value, index) => !value.startsWith("--") && !flagValues.has(index + 1)).map((value) => {
  const [pluginId, appId] = value.split("=");
  return { pluginId, appId: appId ?? null };
});
const seatExample = argv.includes("--example");
const SPAWN_MS = Number(process.env.S3_SPAWN_MS ?? 90_000);
const out = fileURLToPath(new URL(`./${outDir}/s3-foreign-kind-${tag}.txt`, import.meta.url));
const shotPath = (name) => fileURLToPath(new URL(`./${outDir}/s3-${tag}-${name}.png`, import.meta.url));
const log = (...parts) => console.log("[s3]", ...parts);

/** 🔎️ The palette row for one target: the app-qualified id when the target names an app, else the bare
 * `spawn.<pluginId>` row (the plugin's FIRST program — `🏛️ShellHost` keeps that id stable). */
const spawnItemIds = (target) => (target.appId ? [`spawn.${target.pluginId}.${target.appId}`, `spawn.${target.pluginId}`] : [`spawn.${target.pluginId}`]);
const paletteQuery = (target) => (target.appId ? (/^s\.[^.]+\.([^@]+)@/u.exec(target.appId)?.[1] ?? target.pluginId) : target.pluginId);

/** 🚀️ Opens one target through the command palette from whichever window is active, and waits for a
 * window the page did not have before. Returns the row the palette actually offered. */
async function openTarget(page, target) {
  const before = await windowIds(page);
  await openPalette(page);
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  if ((await input.count()) === 0) return { windowIds: [], item: null, detail: "command palette never opened" };
  await input.fill(paletteQuery(target));
  await page.waitForTimeout(1_500);
  let chosen = null;
  for (const id of spawnItemIds(target)) {
    const item = page.locator(`[data-slot="command-item"][data-command-item-id="${id}"]`).first();
    await item.waitFor({ state: "visible", timeout: 10_000 }).catch(() => undefined);
    if ((await item.count()) > 0) {
      chosen = { id, label: ((await item.textContent()) ?? "").trim().slice(0, 80) };
      await item.click({ force: true }).catch(() => undefined);
      break;
    }
  }
  if (chosen === null) {
    await page.keyboard.press("Escape");
    return { windowIds: [], item: null, detail: `no ${spawnItemIds(target).join(" | ")} palette row` };
  }
  const deadline = Date.now() + SPAWN_MS;
  while (Date.now() < deadline) {
    const fresh = (await windowIds(page)).filter((id) => !before.includes(id));
    if (fresh.length > 0) {
      await page.waitForTimeout(4_000);
      return { windowIds: (await windowIds(page)).filter((id) => !before.includes(id)), item: chosen, detail: null };
    }
    await page.waitForTimeout(250);
  }
  return { windowIds: [], item: chosen, detail: "palette row clicked, no new window" };
}

/** 📚️ Seats the spawned program's FIRST example through the rail a human uses: the `setActiveExample`
 * row, its `exampleId` control resolved to the guest's own first live option, then submit. A spawned
 * program opens on its empty genesis document, which paints an empty world; the example is what the
 * single-plugin probes (b3a §16) judged the render on. */
async function seatFirstExample(page) {
  await unfoldActionsRail(page);
  const opened = await clickUncovered(page, '[data-slot="window-action-pane"] [id="action.setActiveExample"]');
  await page.waitForTimeout(1_200);
  const filled = await fillStagedArgument(page, "exampleId", LIVE_ID);
  const submitted = await submitStagedVerb(page, "setActiveExample");
  await page.waitForTimeout(3_000);
  await neutralDispatch(page);
  return { opened, filled, submitted };
}

/** 🖼️ What the opened windows actually paint — read inside each window's own `[data-slot="window"]`
 * element (its DOM id IS the window id; `[data-window-id]` is only the dock TAB), with the Actions rail overlay SUBTRACTED (its ~50 rows and
 * icons are framework chrome, not the document): element / svg / canvas counts, the document's own
 * `[data-ui-node-key]` roots, the body's `data-slot` set, its text and any window fault. The screenshots
 * beside the capture are the visual half of the same reading. */
const renderFacts = (page, ids) =>
  page.evaluate((wanted) => {
    const rows = [];
    for (const id of wanted) {
      const host = document.getElementById(id);
      const body = host?.querySelector('[data-slot="window-body"]') ?? null;
      if (host === null || body === null) { rows.push({ id, present: false }); continue; }
      const chrome = new Set([...body.querySelectorAll('[data-slot="window-action-pane"], [data-slot="window-action-pane-overlay"]')].flatMap((pane) => [pane, ...pane.querySelectorAll("*")]));
      const own = [...body.querySelectorAll("*")].filter((element) => !chrome.has(element));
      const canvases = own.filter((element) => element.tagName === "CANVAS").map((canvas) => `${canvas.width}x${canvas.height}`);
      const text = own.filter((element) => element.children.length === 0).map((element) => (element.textContent ?? "").trim()).filter((value) => value.length > 0).join(" ").replace(/\s+/gu, " ").slice(0, 200);
      rows.push({
        id,
        present: true,
        elements: own.length,
        svg: own.filter((element) => element instanceof SVGElement).length,
        canvases,
        uiNodeKeys: own.filter((element) => element.hasAttribute("data-ui-node-key")).map((element) => element.getAttribute("data-ui-node-key")).slice(0, 12),
        slots: [...new Set(own.map((element) => element.getAttribute("data-slot")).filter((slot) => slot !== null))].slice(0, 20),
        text,
        skeleton: body.querySelector('[data-slot="pane-host-root"] > [aria-busy="true"]') !== null,
        fault: host.querySelector("[data-semio-window-fault]")?.getAttribute("data-semio-window-fault") ?? null,
      });
    }
    return rows;
  }, ids);

const browser = await chromium.launch({ headless: process.env.S3_HEADED !== "1", args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.setDefaultNavigationTimeout(180_000);
await page.addInitScript(() => {
  const seen = [];
  Object.defineProperty(window, "__s3Notices", { value: seen });
  const record = () => {
    for (const element of document.querySelectorAll("[data-semio-transient-notice]")) {
      const text = (element.firstChild?.textContent ?? element.textContent ?? "").trim();
      const key = `${element.getAttribute("data-notice-code") ?? ""}|${text}`;
      if (seen.at(-1)?.key !== key) seen.push({ key, code: element.getAttribute("data-notice-code"), text, lang: document.documentElement.lang, at: Math.round(performance.now()) });
    }
  };
  new MutationObserver(record).observe(document, { subtree: true, childList: true, characterData: true });
  const unhandled = [];
  Object.defineProperty(window, "__s3Unhandled", { value: unhandled });
  window.addEventListener("unhandledrejection", (event) => unhandled.push(String(event.reason?.stack ?? event.reason).slice(0, 900)));
});

let phase = "boot";
const consoleLines = [];
const faults = [];
const refusals = [];
const moduleRequests = [];
const failedResponses = [];
page.on("pageerror", (error) => { faults.push({ phase, line: `pageerror: ${String(error)}`.slice(0, 300) }); });
page.on("console", (message) => {
  const text = message.text();
  consoleLines.push({ phase, line: `${message.type()}: ${text}`.slice(0, 300) });
  if (/refused:|dropped action|rejected/iu.test(text)) refusals.push(text.slice(0, 240));
  if (FAULT.test(text) && !NOISE.test(text)) faults.push({ phase, line: `${message.type()}: ${text}`.slice(0, 300) });
});
page.on("request", (request) => {
  const url = decodeURIComponent(request.url());
  if (/plugin-modules|extension-modules/u.test(url)) moduleRequests.push({ phase, url: url.replace(/^https?:\/\/[^/]+/u, "") });
});
page.on("response", (response) => { if (response.status() >= 400) failedResponses.push({ phase, status: response.status(), url: decodeURIComponent(response.url()).replace(/^https?:\/\/[^/]+/u, "") }); });

const result = { baseUrl, tag, beacon: null, census: null, signIn: null, rows: [] };
try {
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  result.beacon = await awaitBeacon(page, Date.now() + 300_000);
  log(`beacon ${result.beacon}`);
  if (result.beacon === null) throw new Error("shell never set a readiness beacon");
  await dismissIntroduction(page);
  await page.waitForTimeout(6_000);

  const session = await page.evaluate(async () => {
    try {
      const module = await import("/@id/virtual:semio-playground-session");
      const value = module.PLAYGROUND_SESSION;
      return { variant: value.variant, hostMode: value.hostMode, registryPluginId: value.registryPluginId, pluginIds: value.plugins.map((row) => row.pluginId) };
    } catch (error) {
      return { error: String(error) };
    }
  });
  const probe = await readProbe(page);
  const statusById = Object.fromEntries((probe?.plugins ?? []).map((row) => [row.pluginId, row.status]));
  const sessionIds = session.pluginIds ?? [];
  result.census = {
    playgroundSessionPluginsLength: sessionIds.length,
    sessionError: session.error ?? null,
    variant: session.variant ?? null,
    hostMode: session.hostMode ?? null,
    catalogProbeRows: probe?.plugins.length ?? null,
    loaded: (probe?.plugins ?? []).filter((row) => row.status === "loaded").length,
    programs: probe?.programs.length ?? null,
    missing: sessionIds.filter((id) => statusById[id] !== "loaded").map((id) => ({ pluginId: id, status: statusById[id] ?? "absent from catalog probe" })),
    bootModuleRequests: moduleRequests.filter((row) => row.phase === "boot").length,
  };
  log(`census ${JSON.stringify(result.census)}`);
  result.locale = await seatLocale(page, locale);
  log(`locale ${result.locale}`);
  result.homeWindows = await windowIds(page);
  await page.screenshot({ path: shotPath("home") }).catch(() => undefined);

  if (process.env.S3_SIGN_IN_EMAIL) {
    phase = "sign-in";
    result.signIn = (await signIn(page, process.env.S3_SIGN_IN_EMAIL, process.env.S3_SIGN_IN_PASSWORD ?? "")) ?? "ok";
    log(`sign-in ${result.signIn}`);
  }

  for (const target of targets) {
    const label = target.appId ?? target.pluginId;
    phase = `open:${label}`;
    const started = Date.now();
    const noticeCursor = (await page.evaluate(() => window.__s3Notices.length));
    const faultCursor = faults.length;
    const consoleCursor = consoleLines.length;
    const fromWindows = await windowIds(page);
    const row = { target: label, pluginId: target.pluginId, openedFrom: fromWindows.includes("s-home-main") && fromWindows.length === 1 ? "home" : `windows ${JSON.stringify(fromWindows)}` };
    let opened = await openTarget(page, target);
    row.homeOpen = { item: opened.item, windowIds: opened.windowIds, detail: opened.detail };
    row.homeNotices = opened.windowIds.length === 0 ? (await page.evaluate(() => window.__s3Notices)).slice(noticeCursor) : [];
    if (opened.windowIds.length === 0 && row.homeNotices.length === 0) {
      const studio = await enterStudio(page);
      row.studioFallback = studio;
      opened = await openTarget(page, target);
      row.studioOpen = { item: opened.item, windowIds: opened.windowIds, detail: opened.detail };
    }
    row.windowIds = opened.windowIds;
    row.openMs = Date.now() - started;
    if (opened.windowIds.length > 0) {
      row.render = await renderFacts(page, opened.windowIds);
      await page.screenshot({ path: shotPath(`${label.replace(/[^A-Za-z0-9]+/gu, "-")}-opened`) }).catch(() => undefined);
      if (seatExample) {
        row.example = await seatFirstExample(page);
        row.renderWithExample = await renderFacts(page, opened.windowIds);
        await page.screenshot({ path: shotPath(`${label.replace(/[^A-Za-z0-9]+/gu, "-")}-example`) }).catch(() => undefined);
      }
      phase = `mutate:${label}`;
      Object.assign(row, await mutateUndoRedo(page, refusals, target.pluginId));
      row.renderAfterRedo = await renderFacts(page, opened.windowIds);
      await page.screenshot({ path: shotPath(`${label.replace(/[^A-Za-z0-9]+/gu, "-")}-after-redo`) }).catch(() => undefined);
    }
    row.notices = (await page.evaluate(() => window.__s3Notices)).slice(noticeCursor);
    row.unhandledRejections = (await page.evaluate(() => window.__s3Unhandled.splice(0)));
    row.moduleRequestsDuringOpen = moduleRequests.filter((entry) => entry.phase === `open:${label}` || entry.phase === `mutate:${label}`).map((entry) => entry.url);
    row.consoleDuringOpen = consoleLines.slice(consoleCursor).map((entry) => entry.line).filter((line) => !NOISE.test(line)).slice(0, 25);
    row.faultLines = faults.slice(faultCursor).map((entry) => entry.line).slice(0, 5);
    row.faultCount = faults.length - faultCursor;
    row.bodiesRendered = (row.renderAfterRedo ?? row.render ?? []).length > 0 && (row.renderAfterRedo ?? row.render).every((facts) => facts.present && !facts.skeleton && !facts.fault);
    row.pass = opened.windowIds.length > 0 && row.bodiesRendered && (row.railRows ?? 0) > 0 && row.mutated === true && row.redoDiffersFromUndo === true && row.faultCount === 0;
    row.totalMs = Date.now() - started;
    result.rows.push(row);
    log(JSON.stringify({ target: row.target, openedFrom: row.openedFrom, homeOpen: row.homeOpen, studioOpen: row.studioOpen, render: row.render, example: row.example, renderWithExample: row.renderWithExample, mutation: row.mutation, edits: row.edits, undoLane: row.undoLane, redoLane: row.redoLane, notices: row.notices, faultCount: row.faultCount, renderAfterRedo: row.renderAfterRedo, bodiesRendered: row.bodiesRendered, pass: row.pass }));
    if (opened.windowIds.length > 0) await closeWindows(page, opened.windowIds);
    await page.waitForTimeout(1_500);
  }
  result.finalShell = await readShell(page).then((shell) => ({ ready: shell.ready, error: shell.error, windowFaults: shell.windowFaults })).catch(() => null);
} catch (error) {
  result.fatal = String(error);
  log(`FATAL ${result.fatal}`);
} finally {
  result.faults = faults.slice(0, 40);
  result.refusals = [...new Set(refusals)].slice(0, 30);
  result.failedResponses = failedResponses.slice(0, 40);
  result.bootModuleSample = moduleRequests.filter((row) => row.phase === "boot").map((row) => row.url).filter((url) => /raster|dag|block/u.test(url)).slice(0, 20);
  await browser.close();
}
writeFileSync(out, JSON.stringify(result, null, 2));
log(`=== S3 ${tag} → ${out} ===`);
log(`PASS ${result.rows.filter((row) => row.pass).length}/${result.rows.length}`);
process.exit(result.fatal ? 1 : 0);
