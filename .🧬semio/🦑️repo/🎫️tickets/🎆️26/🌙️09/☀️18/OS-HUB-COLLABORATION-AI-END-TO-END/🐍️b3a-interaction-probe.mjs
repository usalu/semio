/** 🩺️ Shared slice-B3a interaction probe (block · gis) (ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END, slice B3a).
 *
 * B1b's `🐍️b1b-boot-probe.mjs` scored "the document changed" from the rendered pane text, which is
 * blind on a canvas-only surface (`reasoning`, `dag`) — it reported `addNode` as inert while the
 * verb had in fact dispatched. This probe measures the ONE witness every artifact app publishes
 * regardless of how it paints: the framework-injected History panel's ledger
 * (`framework.history.entry.<seq>` rows, `🏛️ShellHost/🟦️.tsx:9112`) plus the uncommitted-edit count
 * the check-in button carries (`#s-checkin`, same file `:8878`), and it then walks the ledger back
 * with `#framework.history.undo`.
 *
 * The interaction bar a plugin has to clear:
 *   1. the default example renders (a document witness is non-empty at boot),
 *   2. one Actions-panel row dispatches and appends an APPLIED mutation entry to the ledger,
 *   3. `framework.history.undo` retires that entry,
 *   4. no console error or refusal line in the whole run.
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const TICKET = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END";
const OUT = join(TICKET, "🗑️generated");
const FAULT = /unreachable|trapped|\btrap\b|panicked|fault|refused|dropped action|not-ui-safe|missing-owned|invalid-args|unsupported|pageerror|Uncaught|dispatch-failed/i;
// 🔇️ `ws://…/bridge … ERR_CONNECTION_REFUSED` is the shell dialling the MCP agent bridge (slice M7)
// on a run where no bridge process is listening — environmental, not a plugin fault. Narrow on
// purpose: any other refusal, including a bridge error that is not a connection refusal, still counts.
const NOISE = /staged plugin module\(s\) are behind their source|\[stale\]|Failed to load resource: the server responded with a status of 404|Download the (React|Vue) DevTools|typed-operation slots|WebSocket connection to 'ws:\/\/[^']*\/bridge' failed: Error in connection establishment: net::ERR_CONNECTION_REFUSED/;

/** 🧾️ The whole shell surface one page evaluation, including the ledger witness. */
const readShell = (page) => page.evaluate(() => {
  const text = (el) => (el?.innerText ?? "").replace(/\s+/g, " ").trim();
  const visible = (el) => el instanceof HTMLElement && el.offsetParent !== null;
  const entries = [...document.querySelectorAll('[id^="framework.history.entry."]')]
    .filter((el) => !el.id.endsWith(".revert"))
    .map((el) => ({ id: el.id, label: text(el).slice(0, 60), dimmed: el.className.includes("opacity") || el.getAttribute("data-dimmed") === "true" }));
  const checkin = document.querySelector("#s-checkin");
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    error: document.documentElement.getAttribute("data-semio-os-error"),
    shellError: document.documentElement.getAttribute("data-semio-os-shell-error"),
    windowFaults: [...document.querySelectorAll("[data-semio-window-fault]")].map((el) => `${el.getAttribute("data-semio-window-fault-code")}: ${el.getAttribute("data-semio-window-fault")}`),
    panes: [...document.querySelectorAll("[data-surface-id]")].map((el) => ({ id: el.getAttribute("data-surface-id"), canvases: el.querySelectorAll("canvas").length, svg: el.querySelectorAll("svg *").length, chars: text(el).length })),
    // 🪧️ The plugin's OWN published UI tree, which is the document projection for every app that
    // renders UiNodes straight into the window body instead of into a `[data-surface-id]` pane
    // (block2d prints `Node kind: …` / `6 Handle Kinds, 11 Handles` there and publishes no pane at
    // all, so a `[data-surface-id]`-only witness scores it as "nothing rendered"). The engagement
    // overlay is excluded — that is framework chrome carrying the Actions rail, not the document.
    uiNodes: [...document.querySelectorAll("[data-ui-node-key]")]
      .filter((el) => el.closest('[data-slot="window-body"]') !== null && el.closest('[data-slot="window-engagement-overlay"]') === null)
      .filter((el) => el.parentElement?.closest("[data-ui-node-key]") === null)
      .map((el) => `${el.getAttribute("data-ui-node-key")}|${text(el).slice(0, 300)}`),
    windowIds: [...document.querySelectorAll('[data-slot="window"]')].map((el) => el.id),
    toggles: [...document.querySelectorAll('[id$=".engagement.toggle"]')].map((el) => el.id),
    actions: [...new Set([...document.querySelectorAll('[id^="action."]')].map((el) => el.id))].filter((id) => !id.startsWith("action.category.") && !/\.arg\./.test(id)),
    tabs: [...document.querySelectorAll('[role="tab"]')].map((el) => text(el)),
    documentRows: [...document.querySelectorAll('[role="treeitem"]')].filter((el) => !el.closest('[id$=".engagement"]') && !el.closest('[data-slot="panel"]')).map((el) => text(el).slice(0, 80)),
    // 🪞️ The app's OWN panel tabs (artifact / inspector / catalogue …), which is where a
    // canvas-painting plugin publishes the only textual projection of its document. The framework's
    // history panel is excluded so the ledger cannot masquerade as the document reflecting state.
    // Only the framework's OWN panels are excluded by id (history, chat, settings, marketplace,
    // task manager). The artifact and inspection trees are contributed by the app under the same
    // `framework.panelTab.framework.panel.*` id space, so the old blanket prefix exclusion dropped
    // exactly the projection this witness exists to read (measured on block2d: the artifact panel
    // carries `panel:block2d-play-document/handleKind:*` rows and `panelRows` still read 0).
    panelRows: [...document.querySelectorAll('[data-slot="panel"]')].filter((el) => visible(el) && !/framework\.panel\.history|framework\.chat|framework\.settings|framework\.marketplace|os\.task-manager/.test(el.id)).flatMap((panel) => [...panel.querySelectorAll('[role="treeitem"]')].map((el) => `${panel.id}|${text(el).slice(0, 80)}`)),
    combobox: [...document.querySelectorAll('[role="combobox"]')].map((el) => text(el)),
    openPanels: [...document.querySelectorAll('[data-slot="panel"]')].filter(visible).map((el) => el.id.replace(/^framework\.panelTab\./, "")),
    ledger: entries,
    checkin: checkin === null ? null : text(checkin),
  };
});

/** 🔢️ The uncommitted-applied-edit count the check-in button prints as `Check in (N)`. */
const editCount = (shell) => {
  const match = /\((\d+)\)\s*$/.exec(shell.checkin ?? "");
  return match === null ? (shell.checkin === null ? -1 : 0) : Number(match[1]);
};

/** 🔬️ What an interaction has to move: the ledger's applied entries, the uncommitted-edit count and
 * the rendered document (pane text/geometry + non-panel tree rows), all three together so a ledger
 * row over an unchanged document (a journaled fatal diff) cannot be scored as a mutation. */
const witness = (shell) => ({
  ledger: shell.ledger.map((entry) => `${entry.id}${entry.dimmed ? "~" : ""}:${entry.label}`),
  edits: editCount(shell),
  render: JSON.stringify({ panes: shell.panes.map((pane) => `${pane.id}:${pane.chars}:${pane.svg}:${pane.canvases}`), rows: shell.documentRows, panelRows: shell.panelRows, uiNodes: shell.uiNodes }),
});

const sameWitness = (left, right) => JSON.stringify(left) === JSON.stringify(right);

export async function runInteractionProbe(config) {
  const outDir = join(OUT, `b3a-${config.plugin}`);
  mkdirSync(outDir, { recursive: true });
  const url = process.env.SEMIO_PROBE_URL ?? `http://127.0.0.1:${config.port}/?plugin=${config.variant}`;
  const lines = [];
  const t0 = Date.now();
  const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
  const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
  page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 4000 : 1200)}`));
  page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
  const report = { plugin: config.plugin, variant: config.variant, port: config.port, url, action: config.action, startedAt: new Date().toISOString(), steps: [] };
  const faultsSince = (from) => lines.slice(from).filter((l) => FAULT.test(l) && !NOISE.test(l)).map((l) => l.slice(0, 700));
  const note = (name, detail, from) => {
    report.steps.push({ step: name, ms: Date.now() - t0, detail, faults: faultsSince(from) });
    writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
    console.log(`${name} ${JSON.stringify(detail).slice(0, 1000)}`);
  };
  const click = async (selector) => {
    const locator = page.locator(selector).first();
    if (!(await page.locator(selector).count())) return "absent";
    return locator.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).split("\n")[0].slice(0, 120));
  };
  // 🫥️ An Actions-rail ROW can be covered exactly the way the engagement TOGGLE can. `force: true`
  // clicks the row's centre point regardless, so the press lands on whatever is painted on top and
  // still reports `ok`. Measured on fem2d (B3d): with two windows and both the artifact and the
  // inspection panel docked, `action.undo`'s centre answers the inspection tree, the click journals
  // a `Select` chrome note instead of undoing, and the edit count sits at 1 for the whole budget —
  // which reads exactly like a guest that refuses to undo. Retire only the panels that geometrically
  // cover the row, press again, and reopen nothing (the history panel docks elsewhere, so the ledger
  // and `#s-checkin` witness survive).
  // 🕰️ Raising the History tab is a TOGGLE, not an idempotent "bring to front": pressing it while
  // history is already the raised tab of its dock CLOSES it, and every later witness then reads
  // `edits: -1` with an empty ledger — the exact blindness this probe raises history to avoid
  // (measured on 🪵️sourcing by B3f: the mutation, the undo and the redo were all visible in the
  // render witness while `mutated`/`undone`/`redone` all scored false). Every raise goes through
  // here, and it presses only when history is not already open.
  const raiseHistory = async () => {
    const open = await page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"]')].some((el) => el instanceof HTMLElement && el.offsetParent !== null && /framework\.panel\.history/.test(el.id)));
    if (open) return "already-open";
    const clicked = await click('[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]');
    await page.waitForTimeout(1500);
    return clicked;
  };

  const clickUncovered = async (selector) => {
    if (!(await page.locator(selector).count())) return "absent";
    // 📐️ The row is resolved through the LOCATOR, not `document.querySelector`: candidate selectors
    // include Playwright-only forms such as `… >> nth=1` for the per-rail copies, which are not
    // valid CSS. The row's OWN panel is excluded from the covering set — a panel-hosted row
    // (🖨️raster's `…raster-play-layers.add.pixel` lives in the artifact tree) is always "under" its
    // own panel, and retiring that panel deletes the row this step exists to press.
    const covering = await page.locator(selector).first().evaluate((row) => {
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
        if (box.left <= x && box.right >= x && box.top <= y && box.bottom >= y) hits.add(panel.id.replace(/^framework\.panelTab\./, ""));
      }
      return [...hits];
    }).catch(() => []);
    if (covering.length) {
      for (const panel of covering) await click(`[data-slot="panel-tab-button"][id="${panel}"], [id="${panel}"]`);
      await page.waitForTimeout(1200);
    }
    const outcome = await click(selector);
    return covering.length ? `${outcome} (uncovered ${covering.join(",")})` : outcome;
  };

  // 🪟️ A multi-window app mounts the SAME action rows, under the SAME element ids, once per window
  // rail. Measured on fem2d (B3d, `🗑️generated/b3d-undo-row-census-fem2d.txt`): all 34 rows exist
  // twice, `fem2d-model` at x≈13 and `fem2d-results` at x≈811, and the ACTIVE window is
  // `fem2d-results`. A document mutation reaches the document from either rail, but `undo` is
  // dispatched in the active window (see `📓️project-panel-actions-dispatch-in-active-window`), so
  // pressing the first-in-DOM copy undoes nothing and looks exactly like a guest that refuses to
  // undo — fem2d and fem3d both scored `undone: false` that way. Prefer the copy inside the active
  // window and fall back to the first when no window claims focus.
  const activeScoped = async (selector) => {
    const scoped = `[data-slot="window"][data-active="true"] ${selector}`;
    return (await page.locator(scoped).count()) ? scoped : selector;
  };

  // ⏱️ The ledger row and the uncommitted-edit count move on the dispatch ack, but the app's own UI
  // is re-published one guest turn later — measured on block2d, the board text trails the edit count
  // by ~500 ms on mutate, undo AND redo (`🗑️generated/b3a2-block2d-undo-refresh.txt`). Returning the
  // instant the predicate fires therefore reads a document projection that is one step stale and
  // scores `panelRoundTrip` false on an app whose panel does round-trip. Hold for a settle grace and
  // re-read before handing the witness back.
  const SETTLE_GRACE_MS = 2500;
  // ⏳️ Dispatch/undo/redo budget. 25 s is right on a calm machine; under fleet load a guest can take
  // far longer and a slow guest is indistinguishable from a stolen undo through a fixed budget
  // (B3b measured trinity jack landing its undo at 112 s and its redo at 136 s at load ≈ 110 and
  // nearly wrote the timeout up as the §3.2 theft). `SEMIO_PROBE_SETTLE_MS` raises it for every
  // slice that shares this probe; the default is unchanged.
  const STEP_BUDGET_MS = Number(process.env.SEMIO_PROBE_SETTLE_MS ?? 25_000);
  const until = async (predicate, budgetMs) => {
    const deadline = Date.now() + budgetMs;
    let shell = await readShell(page);
    while (Date.now() < deadline) {
      if (predicate(shell)) {
        await page.waitForTimeout(SETTLE_GRACE_MS);
        return { ok: true, shell: await readShell(page) };
      }
      await page.waitForTimeout(500);
      shell = await readShell(page);
    }
    await page.waitForTimeout(SETTLE_GRACE_MS);
    shell = await readShell(page);
    return { ok: predicate(shell), shell };
  };

  await page.goto(url, { waitUntil: "domcontentloaded" });
  let shell = null;
  for (let i = 0; i < 240; i++) {
    await page.waitForTimeout(1000);
    shell = await readShell(page);
    if (shell.error) break;
    if (shell.ready && shell.panes.length && i > 8) break;
  }
  await page.waitForTimeout(4000);
  shell = await readShell(page);
  note("boot", { ready: shell.ready, error: shell.error, shellError: shell.shellError, windowFaults: shell.windowFaults, panes: shell.panes, windowIds: shell.windowIds, tabs: shell.tabs, combobox: shell.combobox }, 0);

  // 📄️ The default example has to have RENDERED: a pane with content or a document tree with rows.
  const rendered = shell.panes.some((pane) => pane.chars > 0 || pane.svg > 0 || pane.canvases > 0) || shell.documentRows.length > 0 || shell.uiNodes.length > 0;
  note("example-rendered", { rendered, combobox: shell.combobox, paneChars: shell.panes.map((pane) => pane.chars), rows: shell.documentRows.length, uiNodes: shell.uiNodes, sample: shell.documentRows.slice(0, 6) }, 0);

  // 🪞️ The app's own panel tabs, opened FIRST so "the panel reflects state" is measured on a
  // projection that is actually mounted. `framework.panel.inspection`/`.artifact` are the two every
  // artifact editor declares; a plugin that names others passes them as `config.panels`.
  // Order matters: these share a dock with the History panel and displace it (measured on block2d —
  // `#s-checkin` and the whole ledger disappeared between `open-history` and the first witness, so
  // every edit count read `-1` and the bar could never be scored), so history is opened LAST.
  {
    const from = lines.length;
    const opened = [];
    for (const tab of config.panels ?? ["framework.panel.inspection", "framework.panel.artifact"]) {
      opened.push(`${tab}=${await click(`[data-slot="panel-tab-button"][id="${tab}"], [id="${tab}"]`)}`);
      await page.waitForTimeout(1200);
    }
    shell = await readShell(page);
    note("open-app-panels", { opened, openPanels: shell.openPanels, panelRows: shell.panelRows.length, sample: shell.panelRows.slice(0, 8) }, from);
  }

  {
    const from = lines.length;
    const opened = await raiseHistory();
    shell = await readShell(page);
    note("open-history", { opened, openPanels: shell.openPanels, ledger: shell.ledger.length, checkin: shell.checkin }, from);
  }

  {
    const from = lines.length;
    const pressed = [];
    for (const toggle of shell.toggles) {
      pressed.push(`${toggle}=${await click(`[id="${toggle}"]`)}`);
      // ⏳️ The rail unfolds and mounts its rows asynchronously; 900 ms was short enough that block2d
      // read `actionCount: 0` on a rail that carries 23 rows a second later.
      await page.waitForTimeout(2500);
    }
    shell = await readShell(page);
    // 🪟️ A docked panel can sit OVER the window's engagement rail, so the toggle press lands on the
    // panel's own tree and the rail never unfolds — the press still reports `ok`, and the only
    // symptom is `actionCount: 0` on an app that has 23 rows. Measured on block2d: the artifact
    // panel occupies (3,3)–(303,483) and the toggle sits at (10,64,75,22), so `elementFromPoint` at
    // the toggle centre answers `panel:block2d-play-document/handleKind:b-l-m`. Retire ONLY the
    // panels that geometrically cover the toggle (the history panel docks bottom-right and never
    // does, so the ledger and `#s-checkin` witness survive) and press again.
    // 🪟️ Recovery must also run when SOME rail mounted but the wanted verb did not. With two windows
    // a docked panel can cover exactly one toggle: 📏️layout mounted the `layout-preview` rail (17
    // generic rows) while `layout-blueprint`'s stayed folded, and `action.addPage`/`action.addFrame`
    // — which only exist there — were simply absent, so layout read as an app with no mutation at all.
    if (!shell.actions.length || !shell.actions.includes(`action.${config.action}`)) {
      const covering = await page.evaluate((ids) => {
        const hits = new Set();
        for (const id of ids) {
          const rect = document.querySelector(`[id="${id}"]`)?.getBoundingClientRect();
          if (rect === undefined) continue;
          for (const panel of document.querySelectorAll('[data-slot="panel"]')) {
            if (!(panel instanceof HTMLElement) || panel.offsetParent === null) continue;
            const box = panel.getBoundingClientRect();
            if (box.left < rect.right && box.right > rect.left && box.top < rect.bottom && box.bottom > rect.top) hits.add(panel.id.replace(/^framework\.panelTab\./, ""));
          }
        }
        return [...hits];
      }, shell.toggles);
      for (const panel of covering) pressed.push(`retire:${panel}=${await click(`[data-slot="panel-tab-button"][id="${panel}"], [id="${panel}"]`)}`);
      await page.waitForTimeout(1200);
      for (const toggle of shell.toggles) {
        pressed.push(`retry:${toggle}=${await click(`[id="${toggle}"]`)}`);
        await page.waitForTimeout(2500);
      }
      shell = await readShell(page);
    }
    note("open-actions", { toggles: shell.toggles, pressed, openPanels: shell.openPanels, actionCount: shell.actions.length, actions: shell.actions.slice(0, 60), present: shell.actions.includes(`action.${config.action}`) }, from);
  }

  // 🎚️ Some verbs only mutate once the window carries state the palette cannot stage as an argument
  // (gis `cut` needs a selection). `setup` rows are dispatched and settled BEFORE the witness is taken,
  // so whatever they change is baseline, not the measured mutation.
  for (const step of config.setup ?? []) {
    const from = lines.length;
    const clicked = await click(`[id="action.${step}"]`);
    await page.waitForTimeout(1200);
    // 🧷️ Same two-stage trigger as the measured verb: a row that carries staged arguments only folds
    // its form open on the first click, so the `…​.action.<id>.execute` control is what dispatches it.
    const submitted = await click(`[id$=".action.${step}.execute"]`);
    await page.waitForTimeout(2500);
    shell = await readShell(page);
    note(`setup:${step}`, { clicked, submitted, ledger: shell.ledger.length, checkin: shell.checkin }, from);
  }

  let mutated = false;
  let before = witness(shell);
  {
    const from = lines.length;
    // 🌲️ Not every app publishes its verbs on an engagement rail. 🖨️raster contributes its mutations
    // as ARTIFACT-PANEL tree rows (`…raster-play-layers.add.pixel` → `addLayer {kind:"pixel"}`), so
    // `actionCount` is 0 for it however hard the rail is pressed and the bar could never be scored.
    // `rowSelector` names that row directly; the panels it lives in are re-opened first, because the
    // rail-recovery step above retires whatever covers the toggle — including that very panel.
    const row = config.rowSelector ?? `[id="action.${config.action}"]`;
    // 🔁️ The app panels and the History panel SHARE a dock, so whichever is raised hides the other —
    // and `#s-checkin`, the edit-count witness, goes with the History panel (measured on raster:
    // every witness taken with the artifact tree raised reads `edits: -1`). In `rowSelector` mode the
    // row lives in an app panel, so the panel is raised only for the press and the History panel is
    // put back before anything is measured; `before` was already taken with History raised, so the
    // two witnesses are comparable.
    let clicked;
    if (config.rowSelector !== undefined) {
      for (const tab of config.panels ?? ["framework.panel.artifact"]) {
        await click(`[data-slot="panel-tab-button"][id="${tab}"], [id="${tab}"]`);
        await page.waitForTimeout(1200);
      }
      clicked = await clickUncovered(row);
      await raiseHistory();
    } else {
      clicked = await clickUncovered(row);
    }
    await page.waitForTimeout(1200);
    // 🧷️ An argument-less row IS its own trigger. A row with arguments folds open a staged form whose
    // `…​.action.<id>.execute` control is the real trigger, so the witness is re-taken after staging.
    const filled = [];
    for (const [key, value] of Object.entries(config.args ?? {})) {
      const select = page.locator(`select[id$=".arg.${key}"], select[id$="${key}"], select[name="${key}"]`).first();
      if (await select.count()) {
        filled.push(await select.selectOption(String(value)).then(() => `${key}=${value}`).catch((e) => `${key}:${String(e).split("\n")[0].slice(0, 80)}`));
        continue;
      }
      // 🔽️ A staged select arg renders as a shadcn combobox BUTTON carrying the arg id, not a native
      // `<select>`: open it, then pick the listbox option whose value/text names the wanted choice.
      const combobox = page.locator(`[role="combobox"][id$=".arg.${key}"], [role="combobox"][id="${key}"], [role="combobox"][id$=".${key}"]`).first();
      if (await combobox.count()) {
        const picked = await combobox
          .click({ timeout: 8000, force: true })
          .then(() => page.waitForTimeout(500))
          .then(() => page.locator(`[role="option"][data-value="${value}"], [role="option"]:has-text("${value}")`).first().click({ timeout: 8000, force: true }))
          .then(() => `${key}=${value}`)
          .catch((e) => `${key}:${String(e).split("\n")[0].slice(0, 80)}`);
        filled.push(picked);
        continue;
      }
      const input = page.locator(`[id$=".arg.${key}"]:is(input,textarea), [id$="${key}"]:is(input,textarea), [name="${key}"]`).first();
      if (!(await input.count())) {
        // 🎚️ An `ActionArgDef::slider` does NOT render as `input[type=range]`: it is a Radix thumb,
        // `<span role="slider" aria-valuenow=… >` carrying no id at all, so every id-keyed selector
        // above misses it and the arg silently stages nothing (measured on gisterrain's
        // `setExaggeration`: `value:absent`, and the verb dispatched the staged default 2.5).
        // Drive it from the keyboard, which is also what a user without a mouse does.
        const thumb = page.locator('[role="slider"]').first();
        if (await thumb.count()) {
          const now = () => thumb.getAttribute("aria-valuenow").then(Number);
          await thumb.focus().catch(() => {});
          let current = await now();
          for (let step = 0; step < 80 && Math.abs(current - Number(value)) > 1e-9; step++) {
            await page.keyboard.press(current < Number(value) ? "ArrowRight" : "ArrowLeft");
            await page.waitForTimeout(60);
            const next = await now();
            if (next === current) break;
            if ((current < Number(value)) !== (next <= Number(value))) { current = next; break; }
            current = next;
          }
          filled.push(`${key}=${current}${current === Number(value) ? "" : `(asked ${value})`}`);
          continue;
        }
        filled.push(`${key}:absent`);
        continue;
      }
      // 🎚️ A slider arg (`ActionArgDef::slider`, e.g. gisterrain's `setExaggeration`) renders as
      // `input[type=range]`, which Playwright's `fill` refuses outright. Drive it through React's own
      // value setter so the controlled component sees the change, then fire `input` + `change`.
      if ((await input.getAttribute("type")) === "range") {
        filled.push(await input.evaluate((el, next) => {
          const setter = Object.getOwnPropertyDescriptor(window.HTMLInputElement.prototype, "value")?.set;
          setter?.call(el, String(next));
          el.dispatchEvent(new Event("input", { bubbles: true }));
          el.dispatchEvent(new Event("change", { bubbles: true }));
          return el.value;
        }, value).then((seated) => `${key}=${seated}`).catch((e) => `${key}:${String(e).split("\n")[0].slice(0, 80)}`));
        continue;
      }
      filled.push(await input.fill(String(value)).then(() => `${key}=${value}`).catch((e) => `${key}:${String(e).split("\n")[0].slice(0, 80)}`));
    }
    if (filled.length) {
      await page.waitForTimeout(400);
      before = witness(await readShell(page));
    }
    // 🚀️ The staged form's trigger is `….action.<id>.execute` in most rails, but not in all of them
    // (measured on 🔋️energy's zones rail: `rename-zone`'s two args stage fine — `filled:
    // ["zone=1","newName=ProbeZone"]` — and the exact-suffix selector still answered `absent`, so
    // the verb was never dispatched and the app read as inert). Fall back to any execute control
    // that names the verb before giving up.
    let submitted = await click(`[id$=".action.${config.action}.execute"]`);
    if (submitted === "absent") submitted = await click(`[id$="${config.action}.execute"]`);
    if (submitted === "absent") submitted = await click(`[id*="${config.action}"][id$=".execute"]`);
    if (submitted === "absent") submitted = await click(`[id$=".execute"]:visible`);
    const settled = await until((next) => !sameWitness(witness(next), before), STEP_BUDGET_MS);
    shell = settled.shell;
    const after = witness(shell);
    const applied = after.ledger.filter((entry) => !entry.includes("~"));
    mutated = applied.length > before.ledger.filter((entry) => !entry.includes("~")).length && after.edits > before.edits;
    note("invoke-action", { action: config.action, clicked, filled, submitted, mutated, renderChanged: after.render !== before.render, before, after }, from);
  }

  // 🔁️ `undo`/`redo` may need MORE than one candidate control. A multi-window app mounts the row
  // once per rail under the same element id (fem2d: 34 rows × 2, `🗑️generated/b3d-undo-row-census-fem2d.txt`)
  // and the press only reaches the document from the rail of the window that owns it, which is not
  // reliably the active one nor reliably the first in the DOM — measured on fem2d, where the SAME
  // build retires the edit from one rail and ignores it from the other. Press candidates in order,
  // re-checking the predicate after each, and stop the moment it fires so a working route is never
  // pressed twice.
  const pressUntil = async (selectors, predicate) => {
    const tried = [];
    const share = Math.max(8_000, Math.floor(STEP_BUDGET_MS / Math.max(1, selectors.length)));
    for (const selector of selectors) {
      // ⌨️ `key:<chord>` is the keyboard route, which is the ONLY undo a rail-less app offers: 🖨️raster
      // publishes no `action.undo` row, and its History button shares a dock with the artifact panel
      // it needs open, so every pointer route answers `absent` / `Element is not visible`. The
      // plugin's own end-to-end ticket drives it exactly this way (`mod+z`).
      const outcome = selector.startsWith("key:")
        ? await page.keyboard.press(selector.slice(4)).then(() => "ok").catch((e) => String(e).split("\n")[0].slice(0, 110))
        : await clickUncovered(selector);
      tried.push(`${selector}=${outcome}`);
      if (!outcome.startsWith("ok")) continue;
      const settled = await until(predicate, share);
      if (settled.ok) return { tried, shell: settled.shell };
      shell = settled.shell;
    }
    return { tried, shell: await readShell(page) };
  };

  let undone = false;
  {
    const from = lines.length;
    const afterInvoke = witness(shell);
    // ⏪️ The Actions pane's own `#action.undo` row, not the History panel's button: the pane overlays
    // the footer panel, so `framework.history.undo` is in the DOM but not hit-testable while the rail
    // this interaction was dispatched from is open. Both routes reach the same `undo` action.
    // 🧾️ An app with no engagement rail (🖨️raster publishes its verbs as artifact-panel tree rows)
    // has no `action.undo` row at all, and the History panel's own button shares a dock with the app
    // panels — after the mutation it is in the DOM but NOT visible, so the only remaining route
    // answers `Element is not visible`. Bring the History tab back to the front first.
    if (!(await page.locator('[id="action.undo"]').count())) await raiseHistory();
    const copies = await page.locator('[id="action.undo"]').count();
    const pressed = await pressUntil([
      await activeScoped('[id="action.undo"]'),
      ...[...Array(copies).keys()].map((index) => `[id="action.undo"] >> nth=${index}`),
      '[id="framework.history.undo"] button, [id="framework.history.undo"]',
      "key:Meta+z",
      "key:Control+z",
    ], (next) => witness(next).edits < afterInvoke.edits);
    const clicked = pressed.tried.join(" ; ");
    shell = pressed.shell;
    const after = witness(shell);
    undone = after.edits < afterInvoke.edits && after.edits === before.edits;
    note("undo", { clicked, undone, before, afterInvoke, after }, from);
  }

  let redone = false;
  let panelRoundTrip = false;
  {
    const from = lines.length;
    const afterInvoke = report.steps.find((s) => s.step === "invoke-action")?.detail.after ?? before;
    const afterUndo = witness(shell);
    // 🧾️ An app with no engagement rail (🖨️raster publishes its verbs as artifact-panel tree rows)
    // has no `action.redo` row at all, and the History panel's own button shares a dock with the app
    // panels — after the mutation it is in the DOM but NOT visible, so the only remaining route
    // answers `Element is not visible`. Bring the History tab back to the front first.
    if (!(await page.locator('[id="action.redo"]').count())) await raiseHistory();
    const copies = await page.locator('[id="action.redo"]').count();
    const pressed = await pressUntil([
      await activeScoped('[id="action.redo"]'),
      ...[...Array(copies).keys()].map((index) => `[id="action.redo"] >> nth=${index}`),
      '[id="framework.history.redo"] button, [id="framework.history.redo"]',
      "key:Meta+Shift+z",
      "key:Control+Shift+z",
    ], (next) => witness(next).edits > afterUndo.edits);
    const clicked = pressed.tried.join(" ; ");
    shell = pressed.shell;
    const after = witness(shell);
    redone = after.edits > afterUndo.edits && after.edits === afterInvoke.edits;
    // 🪞️ "the panel reflects state": the rendered document/panel projection has to come BACK to the
    // post-mutation projection and have differed from it while undone, so a ledger-only replay over a
    // frozen panel cannot be scored as a round trip. Reported separately from `redone` because a
    // canvas-only surface publishes no text witness to move.
    panelRoundTrip = after.render === afterInvoke.render && afterUndo.render !== afterInvoke.render;
    note("redo", { clicked, redone, panelRoundTrip, afterInvoke, afterUndo, after }, from);
  }

  await page.screenshot({ path: join(OUT, `b3a-${config.plugin}.png`) });
  const faults = faultsSince(0);
  report.summary = {
    ready: shell.ready,
    error: shell.error,
    exampleRendered: report.steps.find((s) => s.step === "example-rendered")?.detail.rendered ?? false,
    actionCount: shell.actions.length,
    mutated,
    undone,
    redone,
    panelRoundTrip,
    faultLines: faults.length,
    interactionBar: (report.steps.find((s) => s.step === "example-rendered")?.detail.rendered ?? false) && mutated && undone && redone && faults.length === 0 && !shell.error,
  };
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(OUT, `b3a-${config.plugin}-console.txt`), [
    `# ${config.plugin} (${config.variant}) ${url} ${report.startedAt}`,
    `# summary ${JSON.stringify(report.summary)}`,
    "",
    "## faults",
    ...faults,
    "",
    "## steps",
    ...report.steps.map((s) => `${s.step} ${s.ms}ms ${JSON.stringify(s.detail)}`),
    "",
    "## console",
    ...lines,
  ].join("\n"));
  console.log("SUMMARY", JSON.stringify(report.summary));
  await browser.close();
  return report;
}

export { OUT, TICKET };
