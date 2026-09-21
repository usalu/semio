#!/usr/bin/env bun
/** 🔬️ S6 — EVERY plugin kind the `s` registry offers, driven inside the real `s` host (ticket 26/09/18).
 *
 * S5 drove nine foreign programs plus the studio inside `s` and proved the full round trip for three
 * (`📓️s5-spawned-app-windows-in-s.md` §6). This is the SWEEP the same acceptance asks for: one row per
 * spawnable program in the registry, each judged on the same clauses —
 *
 *   spawn · windows present · Actions rail present · one real mutation · undo · redo · 0 fault lines
 *
 * 🧾️ **The oracle is the LEDGER, not the canvas text.** S5's probe (and this one's first draft) judged
 * mutate/undo/redo by diffing `textContent` of the window bodies and the engagement pane. That oracle is
 * blind twice over: unfolding the Actions rail injects ~50 static row labels into the same subtree, and a
 * graph/3d window's body is an SVG or a canvas whose text never changes at all. Measured here: `dag
 * addNode` scored "nothing moved" for the verb AND its undo under that oracle, then scored a clean
 * mutation under this one. The witness is the one `🐍️b3d-interaction-probe.mjs` already proves against
 * every single-plugin serve on this ticket — the framework History panel's applied ledger rows plus the
 * uncommitted-edit count the check-in button prints — so the `s` host is judged by the same yardstick as
 * the playground shells it has to match.
 *
 * ⏪️ **Undo is taken from the rail's own `action.undo` row, not the keyboard chord.** The action pane
 * overlays the footer, and a chord pressed while a spawned editor owns the canvas is not the same lane as
 * the row a human clicks. S5's five `redo ≠ undo: no` rows were all chord rows.
 *
 * 🧹️ Each spawn is CLOSED before the next: sixty live wasm instances in one page measures memory
 * pressure, not "the host hosts every artifact", and the close ladder is itself part of the round trip.
 *
 * Usage:
 *   bun 🐍️s6-all-kinds-sweep.mjs <baseUrl> --census
 *   bun 🐍️s6-all-kinds-sweep.mjs <baseUrl> --tag <tag> <pluginId...>
 */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const argv = process.argv.slice(2);
const baseUrl = argv[0] ?? "http://127.0.0.1:6071/";
const census = argv.includes("--census");
const tagIndex = argv.indexOf("--tag");
const tag = tagIndex >= 0 ? argv[tagIndex + 1] : "adhoc";
const wanted = argv.slice(1).filter((value, index) => !value.startsWith("--") && !(tagIndex >= 0 && index === tagIndex));
// 🪧️ preamble rule 24: `new URL(x, import.meta.url).pathname` percent-encodes emoji segments and the
// capture lands outside the ticket folder. `fileURLToPath` is the only spelling that writes here.
const generated = fileURLToPath(new URL("./🗑️generated/", import.meta.url));

const SPAWN_MS = Number(process.env.S6_SPAWN_MS ?? 60_000);
const SETTLE_MS = Number(process.env.S6_SETTLE_MS ?? 25_000);
const MAX_ROWS = Number(process.env.S6_MAX_ROWS ?? 4);
const log = (...parts) => console.log("[s6]", ...parts);

/** 🗂️ The verb each artifact's own batch report named as its document mutation (b2b, b3a–b3d, f2, pb2,
 * and S5 §6): a rail row id, not a guess. The probe clicks THIS row first when the rail offers it and
 * only falls back to the scan when it does not.
 *
 * 🧷️ **Baked in, not handed in by the caller's environment.** S6 and S7 passed this map through
 * `S6_VERBS`, so the map that produced `3/35` and `22/35` lived in a shell command and died with it;
 * S8 had to reconstruct it from the captures and re-ran six kinds against a map nobody could diff.
 * A permanent probe's own map belongs in the probe. `S6_VERBS` still overrides it wholesale.
 *
 * ✏️ Two ids are corrected against the artifacts' own manifests: `wfc`'s verb is `change-seed`
 * (S7 drove `changeSeed`, which the rail never offers), and `procedural`'s is `generate`
 * (S7 drove `nodeGraphEdit`, which is 🧮️mathematical's). */
const DEFAULT_VERBS = {
  animate: "addTile", architect: "setAdjacencyKind", block: "addHandleKind", cad: "addNode", dag: "addNode",
  demonstrator: "changeSchema", draw: "addLayer", energy: "rename-zone", fem: "addNode", flow: "addWidget",
  forms: "addStep", gis: "addFeature", imperative: "addStep", layout: "addPage", lowpoly: "addPrimitive",
  mathematical: "nodeGraphEdit", norm: "setSnapshot", note: "addBlock", playbook: "addStep",
  "playbook-module-procedural": "importSolidGeometry", procedural: "generate", process: "addStep",
  puzzle: "addNode", raster: "addLayer", reasoning: "addNode", remodel: "addStream", sequence: "addStep",
  shooting: "addShot", sourcing: "curationSetCount", stdio: "paste", trinity: "setParameter",
  vcs: "incrementCounter", wfc: "change-seed", writer: "paste",
};
const KNOWN_VERBS = process.env.S6_VERBS ? JSON.parse(process.env.S6_VERBS) : DEFAULT_VERBS;
/** 🩻️ **The third shape: a staged value is DOCUMENT-SPECIFIC and must be read off the LIVE document.**
 *
 * S8 measured `filled: []` and concluded the sweep cannot fill a staged form; S9 landed the filler and
 * measured the layer under it — 🔋️energy's `rename-zone` now stages, submits and REACHES the guest, and
 * the guest refuses it for a domain reason: `mutation.target-missing the energy model has no zone with
 * id 1`. `zone: "1"` is b3d's value for b3d's own seeded document; the studio-spawned instance holds a
 * different one, and no static map can ever be right for every kind's every instance.
 *
 * So an argument whose value is `LIVE_ID` is not a value at all — it is an instruction to resolve one
 * from the focused program's OWN surfaces at fill time, in this order:
 *   1. the argument control's own options, when it is a `<select>` or a shadcn combobox: those options
 *      are populated by the guest from the live document, so they ARE the live ids;
 *   2. the measure tree / inspector rows and any `data-*-id` carrier inside the spawned window bodies;
 *   3. the id the guest last printed into a History `op_lines` row (`… id=path-6d…`).
 * Whatever it resolves is recorded in `filled` verbatim, so a row that still fails names the value it
 * was refused with (ticket 26/09/18 S10, S9 §3.3's named third shape). */
const LIVE_ID = "@liveId";

/** 🔎️ Every identifier the focused program currently PRINTS, newest surfaces first. Pure read, no
 * dispatch — it is called between staging an argument and submitting the verb. */
const liveDocumentIds = (page) =>
  page.evaluate(() => {
    const ids = [];
    const push = (value) => {
      const trimmed = typeof value === "string" ? value.trim() : "";
      if (trimmed.length > 0 && trimmed.length <= 64 && !ids.includes(trimmed)) ids.push(trimmed);
    };
    for (const element of document.querySelectorAll('[data-slot="window-body"] [data-row-id], [data-slot="window-body"] [data-node-id], [data-slot="window-body"] [data-feature-id], [data-slot="window-body"] [data-item-id], [data-slot="window-body"] [data-zone-id]')) {
      for (const name of ["data-row-id", "data-node-id", "data-feature-id", "data-item-id", "data-zone-id"]) push(element.getAttribute(name) ?? "");
    }
    for (const row of document.querySelectorAll('[data-slot="window-measure-tree-row"], [data-slot="window-measures-body"] [id]')) {
      push(row.getAttribute("data-measure-id") ?? "");
      const label = (row.textContent ?? "").trim();
      const head = /^([A-Za-z0-9][A-Za-z0-9._-]{0,63})\b/u.exec(label);
      if (head) push(head[1]);
    }
    for (const entry of document.querySelectorAll('[id^="framework.history.entry."]')) {
      for (const match of (entry.textContent ?? "").matchAll(/\bid=([A-Za-z0-9._-]{1,64})/gu)) push(match[1]);
    }
    return ids;
  });

/** 🧾️ Staged arguments a verb refuses to run without, taken verbatim from the refusals S5 captured
 * (`missing field question_ids`, `missing field example_id`, …) and from the batch probes' own configs.
 * Keys are `<pluginId>.<verbId>` first, bare `<verbId>` second. */
const DEFAULT_ARGS = {
  "energy.rename-zone": { zone: LIVE_ID, newName: "ProbeZone" },
  "fem.addNode": { x: "3.5", y: "4.5" },
  "wfc.change-seed": { seed: "7" },
  "trinity.setParameter": { parameterId: LIVE_ID, value: "3" },
  "sourcing.curationSetCount": { delta: "1" },
};
const KNOWN_ARGS = process.env.S6_ARGS ? JSON.parse(process.env.S6_ARGS) : DEFAULT_ARGS;

/** 🎯️ The exact app a kind must be spawned AS, when the bare `spawn.<pluginId>` entry would resolve
 * to a different one. 📕️norm contributes thirty programs (fifteen standards × editor/viewer) and the
 * acceptance names `din16798`; without this the probe spawned whatever came first, or nothing. */
const DEFAULT_APPS = { norm: "s.norm.din16798@1/*#editor" };
const KNOWN_APPS = process.env.S6_APPS ? JSON.parse(process.env.S6_APPS) : DEFAULT_APPS;

const FAULT = /unreachable|trapped|\btrap\b|panicked|fault|refused|dropped action|not-ui-safe|missing-owned|invalid-args|unsupported|pageerror|Uncaught|dispatch-failed/i;
const NOISE = /staged plugin module\(s\) are behind their source|\[stale\]|Failed to load resource: the server responded with a status of 404|Download the (React|Vue) DevTools|typed-operation slots/;

/** 🧾️ The whole shell surface in one evaluation — the same shape `🐍️b3d-interaction-probe.mjs` reads. */
const readShell = (page) =>
  page.evaluate(() => {
    const text = (element) => (element?.innerText ?? "").replace(/\s+/gu, " ").trim();
    const entries = [...document.querySelectorAll('[id^="framework.history.entry."]')]
      .filter((element) => !element.id.endsWith(".revert"))
      .map((element) => ({ id: element.id, label: text(element).slice(0, 60), dimmed: element.className.includes("opacity") || element.getAttribute("data-dimmed") === "true" }));
    const checkin = document.querySelector("#s-checkin");
    return {
      ready: document.documentElement.getAttribute("data-semio-os-ready"),
      error: document.documentElement.getAttribute("data-semio-os-error"),
      windowFaults: [...document.querySelectorAll("[data-semio-window-fault]")].map((element) => `${element.getAttribute("data-semio-window-fault-code")}: ${element.getAttribute("data-semio-window-fault")}`),
      panes: [...document.querySelectorAll("[data-surface-id]")].map((element) => ({ id: element.getAttribute("data-surface-id"), canvases: element.querySelectorAll("canvas").length, svg: element.querySelectorAll("svg *").length, chars: text(element).length })),
      windowIds: [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")).filter((id) => typeof id === "string"),
      toggles: [...document.querySelectorAll('[id$=".engagement.toggle"]')].map((element) => element.id),
      actions: [...new Set([...document.querySelectorAll('[data-slot="window-action-pane"] [id^="action."]')].map((element) => element.id))].filter((id) => !id.startsWith("action.category.") && !/\.arg\./u.test(id)),
      stagedRows: [...new Set([...document.querySelectorAll('[data-slot="window-action-pane"] [id^="action."]')].map((element) => `${element.id}|${text(element).slice(0, 40)}`))],
      ledger: entries,
      checkin: checkin === null ? null : text(checkin),
      bodies: [...document.querySelectorAll('[data-slot="window-body"]')].map((body) => {
        const pane = body.querySelector('[data-slot="window-action-pane"], [data-slot="window-action-pane-overlay"]');
        const paneElements = pane === null ? 0 : pane.querySelectorAll("*").length + 1;
        const paneChars = pane === null ? 0 : text(pane).length;
        return `${body.querySelectorAll("*").length - paneElements}/${text(body).length - paneChars}/${body.querySelectorAll("svg *").length}/${body.querySelectorAll("canvas").length}`;
      }),
      measures: [...document.querySelectorAll('[data-slot="window-measures-body"], [data-slot="window-measure-tree-row"]')].map((row) => text(row).slice(0, 60)),
    };
  });

/** 🔢️ The uncommitted-applied-edit count the check-in button prints as `Check in (N)`. */
const editCount = (shell) => {
  const match = /\((\d+)\)\s*$/u.exec(shell.checkin ?? "");
  return match === null ? (shell.checkin === null ? -1 : 0) : Number(match[1]);
};

/** 🔬️ What an interaction has to move.
 *
 * 🧾️ `edits`/`applied` are the witness every single-plugin probe on this ticket uses (the framework
 * History panel's applied rows and the `Check in (n)` count). **Inside `s`, driving a SPAWNED program,
 * they do not work** — measured here, not assumed: spawned `draw`, `addLayer` dispatched with no
 * refusal, `edits [0,0] applied [3,3]` after 25 s of nudged settling, while the same verb's ledger row
 * (`create-layer index=1 path base { id=path-6d…`) and `Check In (1)` did appear later in the
 * `🐍️s6-ledger-diagnose.mjs` run once two further dispatches had happened. They are kept because when
 * they DO move they are the strongest possible statement, and their absence is recorded per row.
 *
 * 📐️ `render` is therefore the verdict this sweep scores on — S5 §6's `redo ≠ undo` clause, on a
 * STRUCTURAL digest rather than S5's `textContent` diff. Two blindnesses of the text oracle are fixed:
 * unfolding the Actions rail injects ~50 static row labels into the same subtree (so the pane is
 * excluded by name), and a graph or 3d window's body is an SVG or a canvas whose text never changes
 * (so element counts, svg nodes and canvases are counted). Measured: `dag addNode` read "nothing
 * moved" for verb AND undo under the text oracle. */
const witness = (shell) => ({
  applied: shell.ledger.filter((entry) => !entry.dimmed).length,
  ledger: shell.ledger.map((entry) => `${entry.id}${entry.dimmed ? "~" : ""}:${entry.label}`),
  edits: editCount(shell),
  render: JSON.stringify([...shell.panes.map((pane) => `${pane.id}:${pane.chars}:${pane.svg}:${pane.canvases}`), ...shell.bodies, ...shell.measures]),
});

async function awaitBeacon(page, deadline) {
  while (Date.now() < deadline) {
    const beacon = await page.evaluate(() => {
      const data = document.documentElement.dataset;
      if (data.semioOsReady !== undefined) return `ready:${data.semioOsReady}`;
      if (data.semioOsError !== undefined) return `error:${data.semioOsError}`;
      if (data.semioOsNotFound !== undefined) return `not-found:${data.semioOsNotFound}`;
      return null;
    });
    if (beacon !== null) return beacon;
    await page.waitForTimeout(1000);
  }
  return null;
}

const windowIds = (page) => page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")).filter((id) => typeof id === "string"));
const readProbe = (page) => page.evaluate(() => window.__semioOsCatalogProbe ?? null);

async function dismissIntroduction(page) {
  const deadline = Date.now() + 60_000;
  while (Date.now() < deadline) {
    if ((await page.locator('[data-slot="introduction-veil"]').count()) === 0) return;
    const skip = page.locator('[data-slot="introduction-veil"] button', { hasText: /skip|überspringen/iu }).first();
    if ((await skip.count()) > 0) await skip.click({ force: true }).catch(() => undefined);
    else await page.keyboard.press("Escape").catch(() => undefined);
    await page.waitForTimeout(500);
  }
}

async function openPalette(page) {
  await dismissIntroduction(page);
  await page.locator('[data-slot="navbar"]').first().click({ force: true, position: { x: 4, y: 4 } }).catch(() => undefined);
  await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
  await page.locator("[role='dialog'] [data-slot='command-input']").first().waitFor({ state: "visible", timeout: 15_000 }).catch(() => undefined);
}

async function signIn(page, email, password) {
  const badge = page.locator('[data-semio-hub-sign-in=""]').first();
  if ((await badge.count()) === 0) return "no sign-in badge";
  await badge.click({ force: true }).catch(() => undefined);
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 60_000 }).catch(() => undefined);
  if ((await form.count()) === 0) return "hub workspace never opened";
  await form.locator('input[type="email"]').fill(email);
  await form.locator('input[type="password"]').fill(password);
  await form.locator('button[type="submit"][aria-label="Sign in"]').click({ force: true }).catch(() => undefined);
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => undefined);
  // 🚪️ Leave the workspace overlay by its OWN named control and do not navigate. The two lines this
  // replaces clicked `[data-semio-hub-workspace] button[aria-label]`'s FIRST match — whatever the
  // overlay happens to render first — and then `page.goBack()`, a history navigation that is no part
  // of signing in. Measured on 2026-09-21 (`🗑️generated/s10-sweep-before.txt`): that pair left the
  // signed-in shell throwing `TypeError: Cannot read properties of undefined (reading 'en')`, after
  // which the command palette never opened and `window.__semioOsCatalogProbe` was never published, so
  // every row of the sweep died at `studio: command palette never opened`. The same sign-in WITHOUT
  // the navigation reaches Home with its studios table rendered and survives a full page reload
  // (`🐍️s10-boot-diagnose.mjs`, S10 §1.4) — this is the shape `🐍️s9-home-actions-probe.mjs` proved.
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click({ force: true }).catch(() => undefined);
  await page.waitForFunction(() => document.querySelector('[id="s-home-main"]') !== null || document.querySelectorAll(".semio-table-host").length > 0, undefined, { timeout: 180_000 }).catch(() => undefined);
  await page.waitForTimeout(4_000);
  return null;
}

/** 🏗️ Home cannot spawn (`spawnApp` is the STUDIO app's verb, measured by S3) — Home → studio is the
 * middle leg every per-kind spawn below stands on. */
async function enterStudio(page) {
  if ((await windowIds(page)).some((id) => /studio|s-workflow/iu.test(id))) return { detail: "already in a studio" };
  await openPalette(page);
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  if ((await input.count()) === 0) return { detail: "command palette never opened" };
  // 🔎️ Type the query first. The palette renders a WINDOW of its rows, and the studio's own entry
  // (`spawn.space.s.space.studio@1/*#editor`) sits outside the first twenty on this registry — an
  // unfiltered look found `0` matches and reported "no studio palette entry" while the entry existed
  // (measured 2026-09-21, `🗑️generated/s10-studio-entry-palette.txt`: 20 rows unfiltered, the studio
  // row present only after typing). `spawnProgram` below already types its plugin id for exactly this
  // reason; `enterStudio` did not.
  await input.fill("studio");
  await page.waitForTimeout(2_000);
  const item = page.locator('[data-slot="command-item"][data-command-item-id="spawn.space.s.space.studio@1/*#editor"]').first();
  if ((await item.count()) === 0) {
    await page.keyboard.press("Escape");
    return { detail: "no studio palette entry" };
  }
  const before = await windowIds(page);
  await item.click({ force: true }).catch(() => undefined);
  const deadline = Date.now() + 180_000;
  while (Date.now() < deadline) {
    const opened = (await windowIds(page)).find((id) => !before.includes(id));
    if (opened) {
      await page.waitForTimeout(6_000);
      return { windowId: opened, detail: null };
    }
    await page.waitForTimeout(500);
  }
  return { detail: "studio spawn dispatched but no new window opened" };
}

async function spawnProgram(page, pluginId) {
  const before = await windowIds(page);
  await openPalette(page);
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  if ((await input.count()) === 0) return { windowIds: [], detail: "command palette never opened" };
  const wanted = KNOWN_APPS[pluginId];
  // 🔎️ Type the NARROWEST query that renders the wanted row. The palette renders a window of its
  // matches, so typing `norm` (thirty programs) leaves `din16798`'s own row unrendered and the
  // exact-id locator finds nothing; typing `din16798` renders it. Derived from the app id's artifact
  // kind segment so the map stays a single declaration.
  await input.fill(wanted ? (/^s\.[^.]+\.([^@]+)@/u.exec(wanted)?.[1] ?? pluginId) : pluginId);
  await page.waitForTimeout(1_500);
  // 🎯️ Address the kind's OWN app when this slice names one (📕️norm contributes 30 programs across
  // fifteen standards, and the acceptance is about `din16798` in particular, not "whichever one the
  // bare entry resolves to"). Then the bare `spawn.<pluginId>` entry, then ANY `spawn.<pluginId>.…`
  // entry — 📕️norm reported `no spawn.norm palette entry` while offering thirty of them, because the
  // probe only ever looked for the bare id (measured 2026-09-21, `🗑️generated/s6-sweep-s10e.txt`).
  let item = wanted ? page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${pluginId}.${wanted}"]`).first() : page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${pluginId}"]`).first();
  await item.waitFor({ state: "visible", timeout: 20_000 }).catch(() => undefined);
  if ((await item.count()) === 0) item = page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${pluginId}"]`).first();
  if ((await item.count()) === 0) item = page.locator(`[data-slot="command-item"][data-command-item-id^="spawn.${pluginId}."]`).first();
  if ((await item.count()) === 0) {
    await page.keyboard.press("Escape");
    return { windowIds: [], detail: `no spawn.${pluginId} palette entry` };
  }
  await item.click({ force: true }).catch(() => undefined);
  const deadline = Date.now() + SPAWN_MS;
  while (Date.now() < deadline) {
    if ((await windowIds(page)).some((id) => !before.includes(id))) {
      await page.waitForTimeout(3_000);
      return { windowIds: (await windowIds(page)).filter((id) => !before.includes(id)), detail: null };
    }
    await page.waitForTimeout(250);
  }
  return { windowIds: [], detail: "no new window after spawn" };
}

/** 🎛️ The Actions rail is FOLDED inside each window's engagement pane (S5 §4.2a). */
async function unfoldActionsRail(page) {
  const toggles = page.locator('[id$=".engagement.toggle"]');
  const count = await toggles.count();
  for (let index = 0; index < count; index += 1) await toggles.nth(index).click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(1_200);
  return count;
}

const click = async (page, selector) => {
  const locator = page.locator(selector).first();
  if ((await locator.count()) === 0) return "absent";
  return locator
    .click({ force: true, timeout: 8_000 })
    .then(() => "ok")
    .catch((error) => String(error).split("\n")[0].slice(0, 80));
};

/** 🫥️ `force: true` presses the row's centre POINT whatever is painted on top of it, and still
 * reports `ok`. FL1/PB3 measured the consequence on the single-plugin serves: with two windows and a
 * docked panel over the rail, `action.undo`'s centre answers the inspection tree, the press journals a
 * chrome `Select` instead of undoing, and the row reads exactly like a guest that refuses to undo.
 * Retire only the panels that geometrically cover the point, press again, reopen nothing — the History
 * panel docks elsewhere, so the ledger and `#s-checkin` witness survive. Ported verbatim in behaviour
 * from `🐍️b3a-interaction-probe.mjs`, which is why the single-plugin serves score these kinds PASS
 * and this sweep did not. */
const clickUncovered = async (page, selector) => {
  if ((await page.locator(selector).count()) === 0) return "absent";
  const covering = await page
    .locator(selector)
    .first()
    .evaluate((row) => {
      const rect = row.getBoundingClientRect();
      if (rect.width === 0 || rect.height === 0) return [];
      const x = rect.left + rect.width / 2;
      const y = rect.top + rect.height / 2;
      const top = document.elementFromPoint(x, y);
      if (top === null || row === top || row.contains(top) || top.contains(row)) return [];
      const own = row.closest('[data-slot="panel"]');
      const hits = new Set();
      for (const panel of document.querySelectorAll('[data-slot="panel"]')) {
        if (!(panel instanceof HTMLElement) || panel.offsetParent === null || panel === own) continue;
        const box = panel.getBoundingClientRect();
        if (box.left <= x && box.right >= x && box.top <= y && box.bottom >= y) hits.add(panel.id.replace(/^framework\.panelTab\./u, ""));
      }
      return [...hits];
    })
    .catch(() => []);
  if (covering.length > 0) {
    for (const panel of covering) await click(page, `[data-slot="panel-tab-button"][id="${panel}"], [id="${panel}"]`);
    await page.waitForTimeout(1_200);
  }
  return click(page, selector);
};

/** 🕰️ Raising the History tab is a TOGGLE, not "bring to front": pressing it while history is already
 * the raised tab CLOSES it, and every later witness then reads `edits: -1` with an empty ledger — the
 * exact blindness the sweep raises history to avoid. Press only when it is not already open. */
const raiseHistory = async (page) => {
  const open = await page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"]')].some((element) => element instanceof HTMLElement && element.offsetParent !== null && /framework\.panel\.history/u.test(element.id)));
  if (open) return "already-open";
  const clicked = await click(page, '[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]');
  await page.waitForTimeout(1_500);
  return clicked;
};

/** 🧾️ Fills ONE staged argument of an unfolded action form.
 *
 * 🧷️ The sweep used to look for `[data-slot="window-action-pane"] [id$=".arg.<key>"]:is(input,textarea)`
 * and nothing else, so it recorded `filled: []` / `submitted: "absent"` for every verb whose form is
 * not a bare text input — which is why eight kinds that PASS on their own single-plugin serve scored
 * "the verb moved nothing" here (S8 §4.2). Every shape the single-plugin probe already drives is
 * driven here: a native `<select>`, a shadcn combobox BUTTON carrying the arg id, an
 * `input[type=range]` (which Playwright's `fill` refuses outright, so React's own value setter is
 * called), a Radix `[role="slider"]` thumb that carries NO id at all, and a plain input. Selectors
 * fall back from the pane-scoped exact id to the bare id and the `name` attribute, because a spawned
 * program's form is not always mounted inside the host's action pane. */
const fillStagedArgument = async (page, key, value) => {
  const live = value === LIVE_ID;
  const scopes = [`[data-slot="window-action-pane"] `, ""];
  for (const scope of scopes) {
    const select = page.locator(`${scope}select[id$=".arg.${key}"], ${scope}select[id$="${key}"], ${scope}select[name="${key}"]`).first();
    if ((await select.count()) > 0) {
      // 🩻️ A `<select>`'s options ARE the live document: the guest fills them from the snapshot it is
      // rendering, so choosing one of them is the third shape's first and cheapest resolution. A
      // requested value that is not among them is a stale static guess, so the first real option wins.
      const options = await select.evaluate((element) => [...element.options].map((option) => option.value).filter((option) => option.length > 0));
      const chosen = live ? options[0] : (options.includes(String(value)) ? String(value) : (options[0] ?? String(value)));
      if (chosen === undefined) return `${key}:no-live-option`;
      return select.selectOption(chosen).then(() => `${key}=${chosen}${String(chosen) === String(value) ? "" : "(live)"}`).catch((error) => `${key}:${String(error).split("\n")[0].slice(0, 60)}`);
    }
    const combobox = page.locator(`${scope}[role="combobox"][id$=".arg.${key}"], ${scope}[role="combobox"][id="${key}"], ${scope}[role="combobox"][id$=".${key}"]`).first();
    if ((await combobox.count()) > 0) {
      const opened = await combobox.click({ timeout: 8_000, force: true }).then(() => true).catch(() => false);
      if (!opened) return `${key}:combobox-unclickable`;
      await page.waitForTimeout(500);
      // 🩻️ Same law for the shadcn combobox — its `[role="option"]` list is the guest's live enumeration.
      const exact = live ? null : page.locator(`[role="option"][data-value="${value}"], [role="option"]:has-text("${value}")`).first();
      if (exact !== null && (await exact.count()) > 0) {
        return exact.click({ timeout: 8_000, force: true }).then(() => `${key}=${value}`).catch((error) => `${key}:${String(error).split("\n")[0].slice(0, 60)}`);
      }
      const option = page.locator('[role="option"]').first();
      if ((await option.count()) === 0) return `${key}:no-live-option`;
      const label = ((await option.textContent()) ?? "").trim().slice(0, 40);
      return option.click({ timeout: 8_000, force: true }).then(() => `${key}=${label}(live)`).catch((error) => `${key}:${String(error).split("\n")[0].slice(0, 60)}`);
    }
    const input = page.locator(`${scope}[id$=".arg.${key}"]:is(input,textarea), ${scope}[id$="${key}"]:is(input,textarea), ${scope}[name="${key}"]`).first();
    if ((await input.count()) === 0) continue;
    if (live) {
      // 🩻️ A free-text id field carries no enumeration, so the id is harvested from the surfaces the
      // program itself prints. An empty harvest is reported as such rather than papered over with a
      // guess that the guest would refuse with `mutation.target-missing` (S9 §3.3).
      const candidates = await liveDocumentIds(page);
      if (candidates.length === 0) return `${key}:no-live-id`;
      return input.fill(candidates[0]).then(() => `${key}=${candidates[0]}(live)`).catch((error) => `${key}:${String(error).split("\n")[0].slice(0, 60)}`);
    }
    if ((await input.getAttribute("type")) === "range") {
      return input
        .evaluate((element, next) => {
          const setter = Object.getOwnPropertyDescriptor(window.HTMLInputElement.prototype, "value")?.set;
          setter?.call(element, String(next));
          element.dispatchEvent(new Event("input", { bubbles: true }));
          element.dispatchEvent(new Event("change", { bubbles: true }));
          return element.value;
        }, value)
        .then((seated) => `${key}=${seated}`)
        .catch((error) => `${key}:${String(error).split("\n")[0].slice(0, 60)}`);
    }
    return input.fill(String(value)).then(() => `${key}=${value}`).catch((error) => `${key}:${String(error).split("\n")[0].slice(0, 60)}`);
  }
  const thumb = live ? null : page.locator('[data-slot="window-action-pane"] [role="slider"], [role="slider"]').first();
  if (thumb !== null && (await thumb.count()) > 0) {
    const now = () => thumb.getAttribute("aria-valuenow").then(Number);
    await thumb.focus().catch(() => undefined);
    let current = await now();
    for (let step = 0; step < 80 && Math.abs(current - Number(value)) > 1e-9; step += 1) {
      await page.keyboard.press(current < Number(value) ? "ArrowRight" : "ArrowLeft");
      await page.waitForTimeout(60);
      const next = await now();
      if (next === current) break;
      current = next;
    }
    return `${key}=${current}${current === Number(value) ? "" : `(asked ${value})`}`;
  }
  return `${key}:absent`;
};

/** 🚀️ The staged form's trigger is `….action.<id>.execute` in most rails, but not in all of them:
 * 🔋️energy's zones rail stages `rename-zone`'s two args fine and still answers `absent` to the
 * exact-suffix selector, so the verb was never dispatched and the app read as inert (measured by B3d).
 * Fall back to any execute control that names the verb before giving up. */
const submitStagedVerb = async (page, verbId) => {
  for (const selector of [
    `[data-slot="window-action-pane"] [id$=".action.${verbId}.execute"]`,
    `[id$=".action.${verbId}.execute"]`,
    `[id$="${verbId}.execute"]`,
    `[id*="${verbId}"][id$=".execute"]`,
    `[data-slot="window-action-pane"] [id$=".execute"]:visible`,
  ]) {
    const outcome = await click(page, selector);
    if (outcome !== "absent") return outcome;
  }
  return "absent";
};

const until = async (page, predicate, budgetMs, nudge) => {
  const deadline = Date.now() + budgetMs;
  let shell = await readShell(page);
  while (Date.now() < deadline) {
    if (predicate(shell)) return { ok: true, shell };
    if (nudge) await nudge();
    else await page.waitForTimeout(500);
    shell = await readShell(page);
  }
  return { ok: predicate(shell), shell };
};

/** 🫧️ The shell's History panel is the HOST program's panel surface, and nothing refreshes the host
 * while a spawned program owns the canvas (`refreshSpawnedUi` fetches "no panels, no labels" by its
 * own docstring, and its branch in `applyHostEffects` has no host half). Measured on the live `s`
 * host, spawned `draw` + `addLayer`, with 8 s of quiet in between (`🗑️generated/s6-ledger-draw.txt`):
 * the ledger row and the `Check In (n)` count do not appear until the NEXT dispatch of any kind. So a
 * probe that reads the ledger straight after a verb reads the state from before it — which is what
 * scored every kind `mutated: false` on this sweep's first run.
 *
 * This is that next dispatch, made harmless: toggling the framework History panel is a shell config
 * action against the HOST session (it journals `Toggle Panel`, never an uncommitted EDIT, so
 * `#s-checkin`'s count and the applied-row count are untouched by the nudge itself). It is the probe
 * paying for a host-side staleness this slice measured but did not fix (§2 of the report). */
async function neutralDispatch(page) {
  for (const verb of ["clearSelection", "selectAll"]) {
    if ((await click(page, `[data-slot="window-action-pane"] [id="action.${verb}"]`)) === "ok") {
      await page.waitForTimeout(2_000);
      return verb;
    }
  }
  await page.waitForTimeout(2_000);
  return null;
}

/** ✏️ One verb → undo → redo, judged by the ledger. */
async function runVerb(page, verbId, args, refusals) {
  const cursor = refusals.length;
  await raiseHistory(page);
  let before = witness(await readShell(page));
  const clicked = await clickUncovered(page, `[data-slot="window-action-pane"] [id="action.${verbId}"]`);
  await page.waitForTimeout(1_200);
  const filled = [];
  for (const [key, value] of Object.entries(args ?? {})) filled.push(await fillStagedArgument(page, key, value));
  if (filled.length > 0) {
    await page.waitForTimeout(400);
    before = witness(await readShell(page));
  }
  const submitted = await submitStagedVerb(page, verbId);
  await page.waitForTimeout(2_000);

  // ⏳️ **The host's reading of a spawned program is TWO dispatches behind.** Measured, not assumed
  // (§2 of the report, `🗑️generated/s6-ledger-draw.txt` + this sweep's own `edits` arrays): after
  // `addLayer` on a spawned `draw` the shell's `Check in (n)` count and the ledger row for it do not
  // appear on the verb's own turn, do not appear after 8 s of quiet, and do not appear after the next
  // dispatch either — they appear after the SECOND one. A probe that reads once per dispatch therefore
  // compares post-verb against pre-verb for every kind, which is exactly the five `redo ≠ undo: no`
  // rows S5 §6 reported and could not explain.
  //
  // So the round trip is driven with two further NEUTRAL dispatches after the redo — a pure view verb
  // that repaints and journals no uncommitted edit — and the three readings that matter are taken two
  // dispatches after the verb, the undo and the redo respectively.
  // 🛤️ TWO undo lanes, because they are not the same lane and this sweep measured them disagreeing.
  // The rail's own `action.undo` row is what a human clicks and is what `🐍️b3d-interaction-probe.mjs`
  // drives against every single-plugin serve; the reserved shell CHORD is what S5 §6 drove inside `s`.
  // Measured here on a spawned `draw`: the verb moved the edit count 0 → 1 and the RAIL row left it at
  // 1 across two further dispatches, so whichever lane answers is recorded per row rather than assumed.
  await neutralDispatch(page);
  const readVerb = witness(await readShell(page));
  const undoClick = await clickUncovered(page, '[data-slot="window-action-pane"] [id="action.undo"]');
  await neutralDispatch(page);
  let readUndo = witness(await readShell(page));
  let undoLane = readUndo.edits < readVerb.edits || readUndo.render !== readVerb.render ? "rail" : null;
  if (undoLane === null) {
    await page.keyboard.press(process.platform === "darwin" ? "Meta+KeyZ" : "Control+KeyZ");
    await neutralDispatch(page);
    readUndo = witness(await readShell(page));
    if (readUndo.edits < readVerb.edits || readUndo.render !== readVerb.render) undoLane = "chord";
  }
  const redoClick = await clickUncovered(page, '[data-slot="window-action-pane"] [id="action.redo"]');
  await neutralDispatch(page);
  let readRedo = witness(await readShell(page));
  let redoLane = readRedo.edits > readUndo.edits || readRedo.render !== readUndo.render ? "rail" : null;
  if (redoLane === null) {
    await page.keyboard.press(process.platform === "darwin" ? "Meta+Shift+KeyZ" : "Control+Shift+KeyZ");
    await neutralDispatch(page);
    readRedo = witness(await readShell(page));
    if (readRedo.edits > readUndo.edits || readRedo.render !== readUndo.render) redoLane = "chord";
  }

  const mutated = readVerb.edits > before.edits || readVerb.applied > before.applied || readVerb.render !== before.render;
  const undone = readUndo.edits < readVerb.edits || readUndo.render !== readVerb.render;
  const redone = readRedo.edits > readUndo.edits || readRedo.render !== readUndo.render;
  // 🧾️ S5 §6's verdict, the ONE clause that covers the whole round trip: the verb reached the spawned
  // instance, its undo reached it and its redo reached it, because the instance reads a different
  // document after the redo than after the undo.
  const redoDiffersFromUndo = readRedo.edits !== readUndo.edits || readRedo.render !== readUndo.render;

  return {
    verbId,
    clicked,
    filled,
    submitted,
    mutated,
    undone,
    redone,
    redoDiffersFromUndo,
    undoClick,
    redoClick,
    undoLane,
    redoLane,
    edits: [before.edits, readVerb.edits, readUndo.edits, readRedo.edits],
    applied: [before.applied, readVerb.applied, readUndo.applied, readRedo.applied],
    lastLedgerRow: readVerb.ledger.at(-1) ?? null,
    refusals: refusals.slice(cursor, cursor + 3),
  };
}

async function mutateUndoRedo(page, refusals, pluginId) {
  const railToggles = await unfoldActionsRail(page);
  const shell = await readShell(page);
  const ids = shell.actions.map((id) => id.replace(/^action\./u, ""));
  if (ids.length === 0) return { railToggles, railRows: 0, mutation: null, mutationDetail: "no Actions rail row after unfolding" };
  const known = KNOWN_VERBS[pluginId];
  const scanned = ids.filter((id) => {
    if (/close|quit|delete|remove|reset|export|checkpoint|alternative|^undo$|^redo$|^commit$|^checkout/iu.test(id)) return false;
    if (/^set[A-Z]|selection|selectall|reorganize|viewport|zoom|^pan|^fit|^focus|^hover|granularity|^open|^toggle|^copy|^cut/iu.test(id)) return false;
    return true;
  });
  const order = known && ids.includes(known) ? [known, ...scanned.filter((id) => id !== known)] : scanned;
  let best = { railToggles, railRows: ids.length, mutation: null, mutationDetail: `none of ${scanned.length} rail rows moved the ledger`, knownVerbOffered: known !== undefined && ids.includes(known), knownVerb: known ?? null, attempts: [] };
  const attempts = [];
  for (const verbId of order.slice(0, MAX_ROWS)) {
    const attempt = await runVerb(page, verbId, KNOWN_ARGS[`${pluginId}.${verbId}`] ?? KNOWN_ARGS[verbId], refusals);
    attempts.push({ verbId, mutated: attempt.mutated, undone: attempt.undone, redone: attempt.redone, redoDiffersFromUndo: attempt.redoDiffersFromUndo, undoLane: attempt.undoLane, redoLane: attempt.redoLane, edits: attempt.edits, applied: attempt.applied, refusal: attempt.refusals[0] ?? null });
    if (best.mutation === null && attempt.mutated) best = { railToggles, railRows: ids.length, mutation: verbId, mutationDetail: "verb dispatched and moved the document; redo/undo pair did not read two different documents", knownVerbOffered: known !== undefined && ids.includes(known), knownVerb: known ?? null, ...attempt };
    if (attempt.mutated && attempt.redoDiffersFromUndo) {
      return { railToggles, railRows: ids.length, mutation: verbId, mutationDetail: null, knownVerbOffered: known !== undefined && ids.includes(known), knownVerb: known ?? null, attempts, ...attempt };
    }
  }
  return { ...best, attempts };
}

/** 🚪️ Close every window a spawn opened, so the next kind starts from the studio again. */
async function closeWindows(page, ids) {
  for (const id of ids) {
    await click(page, `[id="${id}.windowControls.close"]`);
    await page.waitForTimeout(250);
  }
  await page.waitForTimeout(1_000);
}

const browser = await chromium.launch({ headless: process.env.S6_HEADED !== "1", args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.setDefaultNavigationTimeout(180_000);
const faults = [];
const refusals = [];
page.on("pageerror", (error) => faults.push(`pageerror: ${String(error)}`.slice(0, 260)));
page.on("console", (message) => {
  const text = message.text();
  if (/refused:|dropped action|rejected/iu.test(text)) refusals.push(text.slice(0, 240));
  if (FAULT.test(text) && !NOISE.test(text)) faults.push(`${message.type()}: ${text}`.slice(0, 260));
});

const result = { baseUrl, tag, beacon: null, signIn: null, studio: null, programs: [], rows: [] };
try {
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  result.beacon = await awaitBeacon(page, Date.now() + 300_000);
  log(`beacon ${result.beacon ?? "none"}`);
  if (result.beacon === null) throw new Error("shell never set a readiness beacon");
  if (process.env.S6_SIGN_IN_EMAIL) {
    result.signIn = await signIn(page, process.env.S6_SIGN_IN_EMAIL, process.env.S6_SIGN_IN_PASSWORD ?? "");
    log(`sign-in ${result.signIn ?? "ok"}`);
  }
  result.studio = await enterStudio(page);
  log(`studio ${JSON.stringify(result.studio)}`);
  // 🧾️ The ledger lives in the framework History panel; its rows are absent from the DOM while the
  // panel is closed, and the edit count comes from the space shell's own check-in button.
  result.historyPanel = await click(page, '[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]');
  await page.waitForTimeout(1_500);
  const boot = await readShell(page);
  result.checkin = boot.checkin;
  result.ledgerAtBoot = boot.ledger.length;
  log(`history panel ${result.historyPanel}; checkin ${JSON.stringify(boot.checkin)}; ledger ${boot.ledger.length}`);
  const probe = await readProbe(page);
  if (probe === null) throw new Error("shell exposed no window.__semioOsCatalogProbe");
  result.shellPluginId = probe.shellPluginId;
  result.programs = probe.programs.map((entry) => ({ pluginId: entry.pluginId, appId: entry.appId }));
  result.registryRows = probe.plugins.length;
  result.loaded = probe.plugins.filter((row) => row.status === "loaded").map((row) => row.pluginId);
  log(`registry ${probe.plugins.length} rows, ${result.loaded.length} loaded, ${probe.programs.length} spawnable programs`);
  if (!census) {
    for (const pluginId of wanted) {
      const started = Date.now();
      const faultCursor = faults.length;
      const loadedBefore = result.loaded.includes(pluginId);
      const spawned = await spawnProgram(page, pluginId);
      const after = await readProbe(page);
      const status = after?.plugins.find((row) => row.pluginId === pluginId)?.status ?? "absent";
      const row = {
        pluginId,
        loadedBeforeOpen: loadedBefore,
        lazyInstalled: !loadedBefore && status === "loaded",
        statusAfterOpen: status,
        windowIds: spawned.windowIds,
        openDetail: spawned.detail,
        openMs: Date.now() - started,
      };
      if (spawned.windowIds.length > 0) Object.assign(row, await mutateUndoRedo(page, refusals, pluginId));
      row.faultLines = faults.slice(faultCursor).slice(0, 3);
      row.faultCount = faults.length - faultCursor;
      row.pass = spawned.windowIds.length > 0 && (row.railRows ?? 0) > 0 && row.mutated === true && row.redoDiffersFromUndo === true && row.faultCount === 0;
      row.totalMs = Date.now() - started;
      result.rows.push(row);
      log(JSON.stringify({ ...row, ledger: undefined, before: undefined, afterInvoke: undefined }));
      if (spawned.windowIds.length > 0) await closeWindows(page, spawned.windowIds);
    }
  }
} catch (error) {
  result.fatal = String(error);
  log(`FATAL ${result.fatal}`);
} finally {
  result.faults = faults.slice(0, 40);
  result.refusals = [...new Set(refusals)].slice(0, 30);
  await browser.close();
}

const out = `${generated}s6-sweep-${tag}.txt`;
writeFileSync(out, JSON.stringify(result, null, 2));
log(`=== S6 SWEEP ${tag} → ${out} ===`);
log(`PASS ${result.rows.filter((row) => row.pass).length}/${result.rows.length}`);
process.exit(result.fatal ? 1 : 0);
