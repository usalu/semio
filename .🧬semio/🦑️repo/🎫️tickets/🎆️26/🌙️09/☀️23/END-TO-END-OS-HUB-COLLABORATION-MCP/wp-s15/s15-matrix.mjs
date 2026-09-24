#!/usr/bin/env bun
/** 🧮️ S15 — the plugin × kind × step matrix inside ONE served `s` (React shell), as a user drives it.
 *
 * One row per spawnable program (`window.__semioOsCatalogProbe.programs`, app-qualified), opened from the Home
 * landing through the command palette, judged on:
 *
 *   open · body RENDERED (not the loading skeleton, no window fault, own content beside the chrome) ·
 *   the Actions chip is HIT-TESTABLE by a pointer (the element under its centre is the chip) ·
 *   one real verb from the Actions rail · undo · redo (ledger + `Check in (n)` + structural render digest,
 *   the shared S6 witness) · History rows (read back in the run's locale) · 0 fault lines · closed by its dock tab.
 *
 * Viewers are opened, render-checked, chip-checked and closed; a viewer offers no document verb, so the verb columns
 * read `viewer`. The witness helpers are imported from the shared S6 sweep (fresh module instance, so this run's
 * per-kind verb pins reach its `S6_VERBS`/`S6_ARGS` readers).
 *
 * Usage: bun s15-matrix.mjs <baseUrl> --tag <tag> [--locale de] [--roles editor,viewer] [--reload-every 10]
 *        [--only <plugin|plugin/kind>,...] [--skip <plugin|plugin/kind>,...]
 */
import { chromium } from "playwright";
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const argv = process.argv.slice(2);
const baseUrl = argv[0] ?? "http://127.0.0.1:6540/";
const valueOf = (flag, fallback) => (argv.includes(flag) ? argv[argv.indexOf(flag) + 1] : fallback);
const tag = valueOf("--tag", "adhoc");
const locale = valueOf("--locale", "en");
const roles = valueOf("--roles", "editor").split(",");
const reloadEvery = Number(valueOf("--reload-every", "10"));
const only = valueOf("--only", "").split(",").filter(Boolean);
const skip = valueOf("--skip", "").split(",").filter(Boolean);
const resume = argv.includes("--resume");
const generated = fileURLToPath(new URL("./generated/", import.meta.url));
const log = (...parts) => console.log("[s15]", ...parts);
const SWEEP = "/Users/ueli/Documents/semio/.tmp-ticket-0918/🐍️s6-all-kinds-sweep.mjs";

const census = JSON.parse(readFileSync(`${generated}s15-programs.json`, "utf8"));
const kindOf = (appId) => /^s\.[^.]+\.([^@]+)@/u.exec(appId)?.[1] ?? appId;
const roleOf = (appId) => appId.split("#").at(-1);
const subsetOf = (appId) => /@[^/]+\/([^#]+)#/u.exec(appId)?.[1] ?? "*";
const baseKeyOf = (program) => `${program.pluginId}/${kindOf(program.appId)}`;
const keyOf = (program) => `${baseKeyOf(program)}${subsetOf(program.appId) === "*" ? "" : `/${subsetOf(program.appId)}`}`;

/** 🗂️ Per-kind verb pins where a kind's document verb differs from its plugin's pin (measured on this serve). */
const KIND_VERBS = {
  "block/block2d": "addHandleKind",
  "block/block3d": "addRepresentation",
  "block/block5d": "addGripKind",
  "gis/gisterrain": "setExaggeration",
  "puzzle/puzzle3d": "duplicateSelection",
  "demonstrator/puzzle3d": "duplicateSelection",
  "stdio/txt": "replace-text",
  "stdio/md": "replace-text",
  "stdio/html": "replace-text",
  "stdio/json": "set-node",
  "stdio/xml": "set-node",
};
/** ✋️ A rail verb a kind's document verb needs first — puzzle3d's selection verbs act on the live selection. */
const KIND_PRE = { "puzzle/puzzle3d": "selectAll", "demonstrator/puzzle3d": "selectAll" };
/** 🧾️ Staged arguments for per-kind pins. */
const KIND_ARGS = {
  "gis/gisterrain.setExaggeration": { exaggeration: "2.5" },
  "stdio/txt.replace-text": { text: "S15 text" },
  "stdio/md.replace-text": { text: "# S15" },
  "stdio/html.replace-text": { text: "<p>S15</p>" },
  "stdio/json.set-node": { nodeId: "@liveId", value: "\"S15\"" },
  "stdio/xml.set-node": { nodeId: "@liveId", value: "S15" },
};
/** 🧭️ Kinds a plugin re-hosts from another plugin (📽️demonstrator composes other plugins' artifacts): their verb
 * pins and staged arguments are the ORIGIN plugin's. */
const KIND_ORIGIN = {
  "demonstrator/generation3d": "procedural",
  "demonstrator/cad": "cad",
  "demonstrator/puzzle3d": "puzzle",
  "demonstrator/curation": "sourcing",
  "demonstrator/process3d": "process",
  "demonstrator/gismap": "gis",
};
const defaults = await import(SWEEP);
const programs = census.programs.filter((program) => roles.includes(roleOf(program.appId)) && !(program.pluginId === "space" && /home|studio/u.test(program.appId)));
const verbs = { ...defaults.DEFAULT_VERBS };
const args = { ...defaults.DEFAULT_ARGS };
for (const program of programs) {
  const key = keyOf(program);
  const base = baseKeyOf(program);
  const origin = KIND_ORIGIN[base] ?? program.pluginId;
  const verb = KIND_VERBS[key] ?? KIND_VERBS[base] ?? defaults.DEFAULT_VERBS[origin];
  if (verb) verbs[key] = verb;
  for (const [argKey, value] of Object.entries(defaults.DEFAULT_ARGS)) if (argKey.startsWith(`${origin}.`)) args[`${key}.${argKey.slice(origin.length + 1)}`] = value;
  for (const [argKey, value] of Object.entries(KIND_ARGS)) if (argKey.startsWith(`${base}.`)) args[`${key}.${argKey.slice(base.length + 1)}`] = value;
}
/** 📕️ Each norm standard's `setSnapshot` stages ITS OWN codec-canonical document: the `➡️after` snapshot of the
 * standard's first committed mutation fixture (it differs from the seeded example by that mutation), folded to one
 * line by deleting line breaks only — never re-serialized, so f64 carriers keep their exact JSON form. */
const NORM_ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts";
const normSnapshot = (kind) => {
  const dir = readdirSync(NORM_ROOT).find((name) => name.endsWith(kind));
  if (!dir) return null;
  const mutations = `${NORM_ROOT}/${dir}/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations`;
  for (const mutation of readdirSync(mutations).sort()) for (const example of readdirSync(`${mutations}/${mutation}`).sort()) {
    try {
      return readFileSync(`${mutations}/${mutation}/${example}/📸️snapshot/➡️after/🔣️.json`, "utf8").replace(/\s*\n\s*/gu, "");
    } catch {
      continue;
    }
  }
  return null;
};
for (const program of programs.filter((entry) => entry.pluginId === "norm")) {
  const snapshot = normSnapshot(kindOf(program.appId));
  if (snapshot !== null) args[`${keyOf(program)}.setSnapshot`] = { snapshot };
}
process.env.S6_VERBS = JSON.stringify(verbs);
process.env.S6_ARGS = JSON.stringify(args);
process.env.S6_MAX_ROWS = process.env.S6_MAX_ROWS ?? "6";
const sweep = await import(`${SWEEP}?s15=${Date.now()}`);
const { FAULT, NOISE, awaitBeacon, clickUncovered, dismissIntroduction, mutateUndoRedo, readProbe, readShell, seatLocale, unfoldActionsRail, windowIds } = sweep;

const selected = programs.filter((program) => {
  const key = keyOf(program);
  if (only.length > 0 && !only.some((entry) => entry === program.pluginId || entry === key)) return false;
  return !skip.some((entry) => entry === program.pluginId || entry === key);
});

/** ⌨️ Opens the palette by the shell chord on a focused document, never by clicking chrome (a click at the
 * navbar's corner is the `Artifact` panel tab and leaves that panel open over every later window). */
async function openPalette(page) {
  await dismissIntroduction(page);
  await page.evaluate(() => {
    if (document.activeElement instanceof HTMLElement) document.activeElement.blur();
    document.body.focus();
  });
  await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 15_000 }).catch(() => undefined);
  return input;
}

async function openProgram(page, program) {
  const before = await windowIds(page);
  const input = await openPalette(page);
  if ((await input.count()) === 0) return { windowIds: [], item: null, detail: "command palette never opened" };
  await input.fill(kindOf(program.appId));
  await page.waitForTimeout(1_200);
  const ids = [`spawn.${program.pluginId}.${program.appId}`];
  const first = census.programs.find((entry) => entry.pluginId === program.pluginId);
  if (first?.appId === program.appId) ids.push(`spawn.${program.pluginId}`);
  let chosen = null;
  for (const id of ids) {
    const item = page.locator(`[data-slot="command-item"][data-command-item-id="${id}"]`).first();
    await item.waitFor({ state: "visible", timeout: 8_000 }).catch(() => undefined);
    if ((await item.count()) > 0) {
      chosen = { id, label: ((await item.textContent()) ?? "").trim().slice(0, 100) };
      await item.click({ timeout: 8_000 }).catch(() => item.click({ force: true }).catch(() => undefined));
      break;
    }
  }
  if (chosen === null) {
    await page.keyboard.press("Escape");
    return { windowIds: [], item: null, detail: `no ${ids.join(" | ")} palette row` };
  }
  const deadline = Date.now() + 90_000;
  while (Date.now() < deadline) {
    const fresh = (await windowIds(page)).filter((id) => !before.includes(id));
    if (fresh.length > 0) {
      await page.waitForTimeout(3_500);
      return { windowIds: (await windowIds(page)).filter((id) => !before.includes(id)), item: chosen, detail: null };
    }
    const notices = await page.evaluate(() => window.__s15Notices.length);
    if (notices > 0 && Date.now() > deadline - 80_000) {
      const last = await page.evaluate(() => window.__s15Notices.at(-1));
      if (/spawnProgram|open/iu.test(last?.code ?? "")) return { windowIds: [], item: chosen, detail: `refused: ${last.code} ${last.text}` };
    }
    await page.waitForTimeout(250);
  }
  return { windowIds: [], item: chosen, detail: "palette row pressed, no new window" };
}

/** 🖼️ What each opened window paints, the Actions rail overlay subtracted, plus whether its chips are
 * hit-testable by a pointer. */
const renderFacts = (page, ids) =>
  page.evaluate((wanted) => {
    const rows = [];
    for (const id of wanted) {
      const host = document.getElementById(id);
      const body = host?.querySelector('[data-slot="window-body"]') ?? null;
      if (host === null || body === null) {
        rows.push({ id, present: false });
        continue;
      }
      const chrome = new Set([...body.querySelectorAll('[data-slot="window-action-pane"], [data-slot="window-action-pane-overlay"], [data-slot="window-engagement-overlay"], [data-slot="utility-bar-overlay"], [data-slot="window-measures-overlay"], [data-slot="window-search-overlay"], [data-slot="pane-host"]')].flatMap((pane) => [pane, ...pane.querySelectorAll("*")]));
      const own = [...body.querySelectorAll("*")].filter((element) => !chrome.has(element));
      const canvases = own.filter((element) => element.tagName === "CANVAS").map((canvas) => `${canvas.width}x${canvas.height}`);
      const text = own.filter((element) => element.children.length === 0).map((element) => (element.textContent ?? "").trim()).filter((value) => value.length > 0).join(" ").replace(/\s+/gu, " ");
      const chips = [...host.querySelectorAll('[id$=".engagement.toggle"], [id$=".utilityBar.unfold"], [id$=".utilityBar.fold"]')].map((chip) => {
        const rect = chip.getBoundingClientRect();
        if (rect.width === 0 || rect.height === 0) return { id: chip.id, hit: "zero-size" };
        if (chip instanceof HTMLButtonElement && chip.disabled) return { id: chip.id.replace(/^framework\.window\./u, ""), hit: "self", disabled: true };
        const top = document.elementFromPoint(rect.left + rect.width / 2, rect.top + rect.height / 2);
        const hit = top !== null && (top === chip || chip.contains(top));
        return { id: chip.id.replace(/^framework\.window\./u, ""), hit: hit ? "self" : `${top?.tagName ?? "none"}[${top?.getAttribute("data-slot") ?? ""}]#${top?.closest("[id]")?.id ?? ""}` };
      });
      rows.push({
        id,
        present: true,
        elements: own.length,
        svg: own.filter((element) => element instanceof SVGElement).length,
        canvases,
        uiNodeKeys: own.filter((element) => element.hasAttribute("data-ui-node-key")).length,
        text: text.slice(0, 160),
        textChars: text.length,
        skeleton: body.querySelector('[data-slot="pane-host-root"] [aria-busy="true"]') !== null && own.length < 12,
        fault: host.querySelector("[data-semio-window-fault]")?.getAttribute("data-semio-window-fault") ?? null,
        chips,
      });
    }
    return rows;
  }, ids);

const rendered = (facts) => facts.length > 0 && facts.every((row) => row.present && !row.skeleton && !row.fault && (row.canvases.length > 0 || row.svg > 0 || row.textChars > 0 || row.uiNodeKeys > 0));
const chipsHit = (facts) => facts.every((row) => (row.chips ?? []).every((chip) => chip.hit === "self"));

async function closeProgram(page, ids) {
  const outcomes = [];
  for (const id of ids) {
    const outcome = await page.evaluate((windowId) => {
      const tab = [...document.querySelectorAll('[data-slot="mode-dock-tab"]')].find((element) => element.getAttribute("data-window-id") === windowId);
      if (!tab) return "no-tab";
      const button = tab.querySelector('[data-slot="mode-dock-tab-close"]') ?? tab.parentElement?.querySelector('[data-slot="mode-dock-tab-close"]');
      if (!(button instanceof HTMLElement)) return "no-close";
      button.click();
      return "clicked";
    }, id);
    outcomes.push(outcome);
    await page.waitForTimeout(400);
  }
  await page.waitForTimeout(1_200);
  const remaining = (await windowIds(page)).filter((id) => ids.includes(id));
  return { outcomes, remaining };
}

/** 🧾️ The History panel's rows in the run's locale, minus the probe's own neutral dispatches and the
 * undo/redo journal rows, so what is left is the verb's own row as the user reads it. */
const NEUTRAL_ROWS = /^(Clear Selection|Select All|Auswahl aufheben|Alles auswählen|Undo|Redo|Rückgängig|Wiederholen|Toggle Panel|Panel umschalten)\b/u;
const historyRows = (page) =>
  page.evaluate(() => [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((element) => !element.id.endsWith(".revert")).map((element) => (element.innerText ?? "").replace(/\s+/gu, " ").trim().slice(0, 70))).then((rows) => ({ all: rows.length, verbRows: [...new Set(rows.filter((row) => !NEUTRAL_ROWS.test(row)))].slice(-3) }));

const browser = await chromium.launch({ headless: process.env.S15_HEADED !== "1", args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.setDefaultNavigationTimeout(180_000);
await page.addInitScript(() => {
  const seen = [];
  Object.defineProperty(window, "__s15Notices", { value: seen });
  new MutationObserver(() => {
    for (const element of document.querySelectorAll("[data-semio-transient-notice]")) {
      const text = (element.firstChild?.textContent ?? element.textContent ?? "").trim();
      const key = `${element.getAttribute("data-notice-code") ?? ""}|${text}`;
      if (seen.at(-1)?.key !== key) seen.push({ key, code: element.getAttribute("data-notice-code"), text, lang: document.documentElement.lang });
    }
  }).observe(document, { subtree: true, childList: true, characterData: true });
});
const faults = [];
const refusals = [];
page.on("pageerror", (error) => faults.push(`pageerror: ${String(error)}`.slice(0, 300)));
/** 🪦️ Background work of a program the user just closed that reaches its retired instance (a brush-mesh page already
 * in flight) is dropped by design with ONE typed `instance-retired` line; counted per row as `closeDrops`, not as a fault. */
const CLOSE_DROP = /refused: instance-retired \(gesture window=/u;
const BRIDGE_NOISE = /WebSocket connection to 'ws:\/\/127\.0\.0\.1:\d+\/bridge' failed/u;
let bridgeNoise = 0;
page.on("console", (message) => {
  const text = message.text();
  if (BRIDGE_NOISE.test(text)) {
    bridgeNoise += 1;
    return;
  }
  if (/refused:|dropped action|rejected/iu.test(text)) refusals.push(text.slice(0, 240));
  if (FAULT.test(text) && !NOISE.test(text)) faults.push(`${message.type()}: ${text}`.slice(0, 300));
});

async function boot() {
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  const beacon = await awaitBeacon(page, Date.now() + 300_000);
  await dismissIntroduction(page);
  await page.waitForTimeout(3_000);
  const seated = await seatLocale(page, locale);
  await page.keyboard.press("Escape").catch(() => undefined);
  return { beacon, seated, lang: await page.evaluate(() => document.documentElement.lang) };
}

const out = `${generated}s15-matrix-${tag}.json`;
const previous = resume ? JSON.parse(readFileSync(out, "utf8")) : null;
const result = { baseUrl, tag, locale, roles, started: previous?.started ?? new Date().toISOString(), boots: previous?.boots ?? [], census: null, rows: (previous?.rows ?? []).filter((row) => row.pass) };
const done = new Set(result.rows.map((row) => `${row.key}#${row.role}`));
const flush = () => writeFileSync(out, JSON.stringify(result, null, 1));
try {
  result.boots.push(await boot());
  const probe = await readProbe(page);
  result.census = { registryRows: probe?.plugins.length ?? null, loaded: (probe?.plugins ?? []).filter((row) => row.status === "loaded").length, programs: probe?.programs.length ?? null, selected: selected.length };
  log(`boot ${JSON.stringify(result.boots[0])} census ${JSON.stringify(result.census)}`);
  let sinceBoot = 0;
  for (const program of selected) {
    if (done.has(`${keyOf(program)}#${roleOf(program.appId)}`)) continue;
    if (sinceBoot >= reloadEvery || !(await onHome())) {
      result.boots.push(await boot());
      sinceBoot = 0;
    }
    sinceBoot += 1;
    let row;
    try {
      row = await runRow(program);
    } catch (error) {
      log(`row ${keyOf(program)} interrupted (${String(error).split("\n")[0].slice(0, 120)}); re-booting and retrying once`);
      result.boots.push(await boot());
      sinceBoot = 1;
      row = await runRow(program).catch((retryError) => ({ key: keyOf(program), role: roleOf(program.appId), appId: program.appId, pass: false, openDetail: `probe error: ${String(retryError).split("\n")[0].slice(0, 160)}`, faultCount: 0 }));
      row.retried = true;
    }
    result.rows.push(row);
    flush();
    log(`${row.pass ? "PASS" : "FAIL"} ${row.key}#${row.role} open=${(row.windowIds ?? []).length > 0} rendered=${row.bodiesRendered} chips=${row.chipsHit} rail=${row.railRows ?? "-"} verb=${row.verb ?? "-"} edits=${JSON.stringify(row.edits ?? null)} lanes=${row.undoLane ?? "-"}/${row.redoLane ?? "-"} history=${JSON.stringify(row.historyRows?.verbRows ?? [])} faults=${row.faultCount} ${row.openDetail ?? ""} ${row.verbDetail ?? ""} ${Math.round((row.totalMs ?? 0) / 1000)}s`);
    if (!row.pass) {
      result.boots.push(await boot());
      sinceBoot = 0;
    }
  }
} catch (error) {
  result.fatal = String(error);
  log(`FATAL ${result.fatal}`);
} finally {
  result.finished = new Date().toISOString();
  result.refusals = [...new Set(refusals)].slice(0, 40);
  result.faultsTail = faults.slice(-20);
  result.agentBridgeConnectionErrors = bridgeNoise;
  flush();
  await browser.close();
}
log(`=== ${tag} → ${out}: PASS ${result.rows.filter((row) => row.pass).length}/${result.rows.length} ===`);
process.exit(result.fatal ? 1 : 0);

async function onHome() {
  const ids = await windowIds(page).catch(() => null);
  const beacon = await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready")).catch(() => null);
  return beacon !== null && ids !== null && ids.length === 1 && ids[0] === "s-home-main";
}

async function runRow(program) {
    const started = Date.now();
    const faultCursor = faults.length;
    const noticeCursor = await page.evaluate(() => window.__s15Notices.length);
    const role = roleOf(program.appId);
    const row = { key: keyOf(program), role, appId: program.appId, openedFrom: JSON.stringify(await windowIds(page)) };
    const opened = await openProgram(page, program);
    row.item = opened.item?.id ?? null;
    row.windowIds = opened.windowIds;
    row.openDetail = opened.detail;
    row.openMs = Date.now() - started;
    if (opened.windowIds.length > 0) {
      row.render = await renderFacts(page, opened.windowIds);
      for (const facts of row.render.filter((entry) => !entry.present)) {
        const rect = await page.evaluate((windowId) => {
          const tab = [...document.querySelectorAll('[data-slot="mode-dock-tab"]')].find((element) => element.getAttribute("data-window-id") === windowId);
          const box = tab?.getBoundingClientRect();
          return box ? [box.left, box.top, box.width, box.height] : null;
        }, facts.id);
        if (rect) await page.mouse.click(rect[0] + Math.min(20, rect[2] / 2), rect[1] + rect[3] / 2);
        await page.waitForTimeout(2_500);
        Object.assign(facts, (await renderFacts(page, [facts.id]))[0], { stacked: true });
      }
      row.bodiesRendered = rendered(row.render);
      row.chipsHit = chipsHit(row.render);
      if (role === "editor") {
        if (KIND_PRE[row.key]) {
          await unfoldActionsRail(page);
          row.pre = `${KIND_PRE[row.key]}:${await clickUncovered(page, `[data-slot="window-action-pane"] [id="action.${KIND_PRE[row.key]}"]`)}`;
          await page.waitForTimeout(1_500);
        }
        const verb = await mutateUndoRedo(page, refusals, row.key);
        Object.assign(row, { railRowIds: verb.railRowIds, railRows: verb.railRows, knownVerb: verb.knownVerb, knownVerbOffered: verb.knownVerbOffered, verb: verb.mutation, verbDetail: verb.mutationDetail, filled: verb.filled, edits: verb.edits, applied: verb.applied, undoLane: verb.undoLane, redoLane: verb.redoLane, mutated: verb.mutated ?? false, redoDiffersFromUndo: verb.redoDiffersFromUndo ?? false, attempts: (verb.attempts ?? []).map((attempt) => `${attempt.verbId}:${attempt.edits?.join(",")}:${attempt.undoLane ?? "-"}/${attempt.redoLane ?? "-"}${attempt.refusal ? `:${attempt.refusal.slice(0, 80)}` : ""}`) });
        row.historyRows = await historyRows(page);
        row.renderAfterRedo = (await renderFacts(page, opened.windowIds)).map((facts) => ({ id: facts.id, present: facts.present, skeleton: facts.skeleton, fault: facts.fault, canvases: facts.canvases, textChars: facts.textChars, text: facts.text?.slice(0, 100) }));
      }
      await page.screenshot({ path: `${generated}s15-${tag}-${row.key.replace(/[^A-Za-z0-9]+/gu, "-")}-${role}.png` }).catch(() => undefined);
      const closeCursor = faults.length;
      row.close = await closeProgram(page, opened.windowIds);
      row.closeDrops = faults.slice(closeCursor).filter((line) => CLOSE_DROP.test(line)).length;
      faults.splice(closeCursor, faults.length - closeCursor, ...faults.slice(closeCursor).filter((line) => !CLOSE_DROP.test(line)));
    }
    row.notices = (await page.evaluate(() => window.__s15Notices)).slice(noticeCursor).map((notice) => `${notice.code}|${notice.lang}|${notice.text}`.slice(0, 160));
    row.faultLines = faults.slice(faultCursor).slice(0, 4);
    row.faultCount = faults.length - faultCursor;
    const opens = opened.windowIds.length > 0;
    row.pass = role === "editor"
      ? opens && row.bodiesRendered && row.chipsHit && (row.railRows ?? 0) > 0 && row.mutated === true && row.redoDiffersFromUndo === true && row.faultCount === 0
      : opens && row.bodiesRendered && row.chipsHit && row.faultCount === 0;
    row.totalMs = Date.now() - started;
    return row;
}
