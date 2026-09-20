/** 🩺️ Shared batch-B interaction probe (ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END, slice B3b).
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
 *   4. `framework.history.redo` brings it back,
 *   5. no console error or refusal line in the whole run.
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const TICKET = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END";
const OUT = join(TICKET, "🗑️generated");
const FAULT = /unreachable|trapped|\btrap\b|panicked|fault|refused|dropped action|not-ui-safe|missing-owned|invalid-args|unsupported|pageerror|Uncaught|dispatch-failed/i;
const NOISE = /staged plugin module\(s\) are behind their source|\[stale\]|Failed to load resource: the server responded with a status of 404|Download the (React|Vue) DevTools|typed-operation slots/;

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
    windowIds: [...document.querySelectorAll('[data-slot="window"]')].map((el) => el.id),
    toggles: [...document.querySelectorAll('[id$=".engagement.toggle"]')].map((el) => el.id),
    actions: [...new Set([...document.querySelectorAll('[id^="action."]')].map((el) => el.id))].filter((id) => !id.startsWith("action.category.") && !/\.arg\./.test(id)),
    tabs: [...document.querySelectorAll('[role="tab"]')].map((el) => text(el)),
    documentRows: [...document.querySelectorAll('[role="treeitem"]')].filter((el) => !el.closest('[id$=".engagement"]') && !el.closest('[data-slot="panel"]')).map((el) => text(el).slice(0, 80)),
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
  render: JSON.stringify({ panes: shell.panes.map((pane) => `${pane.id}:${pane.chars}:${pane.svg}:${pane.canvases}`), rows: shell.documentRows }),
});

const sameWitness = (left, right) => JSON.stringify(left) === JSON.stringify(right);

/** ⏱️ How long one dispatch may take to reach the ledger. 25 s covers most artifacts; wfc's `wfc2d`
 * node-graph pane took ~26 s for `change-seed` to journal, so the budget is tunable per run
 * (`SEMIO_PROBE_SETTLE_MS`) rather than a constant that silently scores a slow app as dead. */
const SETTLE_MS = Number(process.env.SEMIO_PROBE_SETTLE_MS ?? 25_000);

/** 🆔️ `elementIdSegment` (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🆔️ElementId/🟦️.tsx:27`) in probe form:
 * the camelCase spelling every `childElementId` segment is normalized to. */
const segment = (raw) => {
  let out = "";
  let up = false;
  for (const ch of raw) {
    if (ch === "-" || ch === "_" || ch === " " || ch === ".") { up = true; continue; }
    if (!/[a-zA-Z0-9]/.test(ch)) continue;
    if (out.length === 0) out += ch.toLowerCase();
    else if (up) { out += ch.toUpperCase(); up = false; }
    else out += ch;
  }
  return out;
};

export async function runInteractionProbe(config) {
  const outDir = join(OUT, `b3b-${config.plugin}`);
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
  const until = async (predicate, budgetMs) => {
    const deadline = Date.now() + budgetMs;
    let shell = await readShell(page);
    while (Date.now() < deadline) {
      if (predicate(shell)) return { ok: true, shell };
      await page.waitForTimeout(500);
      shell = await readShell(page);
    }
    return { ok: predicate(shell), shell };
  };

  await page.goto(url, { waitUntil: "domcontentloaded", timeout: 180_000 });
  let shell = null;
  let reloads = 0;
  let scanned = 0;
  for (let i = 0; i < 240; i++) {
    await page.waitForTimeout(1000);
    // 🔁️ A cold variant cache re-optimizes its deps on the FIRST page load, and these dev servers run
    // with `SEMIO_VITE_HMR=0` (`🏗️builder/🌐️vite/🟦️.ts:151` sets `hmr:false`), so Vite has no channel to
    // send its `full-reload` after the re-optimization — every optimized-dep request then answers
    // `504 (Outdated Optimize Dep)` forever and the shell never boots. The reload the server cannot
    // ask for is issued here instead; the deps are on disk by then, so one reload is enough.
    const stale = lines.slice(scanned).some((l) => l.includes("Outdated Optimize Dep"));
    scanned = lines.length;
    if (reloads < 2 && stale) {
      reloads += 1;
      lines.push(`${Date.now() - t0} info probe reload #${reloads} after Outdated Optimize Dep`);
      await page.reload({ waitUntil: "domcontentloaded", timeout: 180_000 }).catch(() => {});
      await page.waitForTimeout(3000);
      continue;
    }
    shell = await readShell(page);
    if (shell.error) break;
    if (shell.ready && shell.panes.length && i > 8) break;
  }
  await page.waitForTimeout(4000);
  shell = await readShell(page);
  note("boot", { ready: shell.ready, error: shell.error, shellError: shell.shellError, windowFaults: shell.windowFaults, panes: shell.panes, windowIds: shell.windowIds, tabs: shell.tabs, combobox: shell.combobox }, 0);

  // 📄️ The default example has to have RENDERED: a pane with content or a document tree with rows.
  const rendered = shell.panes.some((pane) => pane.chars > 0 || pane.svg > 0 || pane.canvases > 0) || shell.documentRows.length > 0;
  note("example-rendered", { rendered, combobox: shell.combobox, paneChars: shell.panes.map((pane) => pane.chars), rows: shell.documentRows.length, sample: shell.documentRows.slice(0, 6) }, 0);

  {
    const from = lines.length;
    // 🕰️ One click and a flat 1.5 s wait was not enough on a heavy world artifact: `puzzle3d`'s two
    // World3d panes are still settling when the click lands, the footer tab had not mounted yet, and
    // the probe went on with `checkin: null` — i.e. `edits === -1`, which drops the settle back to the
    // unreliable any-change predicate for the whole run. The tab is clicked until the panel is
    // actually visible (an even number of clicks would leave it closed again, so it stops on the
    // first one that lands).
    let opened = "absent";
    for (let attempt = 0; attempt < 3; attempt += 1) {
      opened = await click('[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]');
      await page.waitForTimeout(2500);
      shell = await readShell(page);
      if (shell.checkin !== null) break;
    }
    note("open-history", { opened, openPanels: shell.openPanels, ledger: shell.ledger.length, checkin: shell.checkin }, from);
  }

  {
    const from = lines.length;
    for (const toggle of shell.toggles) {
      await click(`[id="${toggle}"]`);
      await page.waitForTimeout(900);
    }
    shell = await readShell(page);
    note("open-actions", { toggles: shell.toggles, actionCount: shell.actions.length, actions: shell.actions.slice(0, 60), present: shell.actions.includes(`action.${config.action}`) }, from);
  }

  let mutated = false;
  let before = witness(shell);
  if (config.action) {
    const from = lines.length;
    const row = `[id="action.${config.action}"]`;
    const clicked = await click(row);
    await page.waitForTimeout(1200);
    // 🧷️ An argument-less row IS its own trigger. A row with arguments folds open a staged form whose
    // `…​.action.<id>.execute` control is the real trigger, so the witness is re-taken after staging.
    // 🏷️ The staged form's rows carry the id, the CONTROL inside them does not: `buildActionSections`
    // (`🛠️ShellHelpers/🟦️.tsx:4234`) puts `action.<verb>.arg.<arg>` on the `TreeDataItem` and hands the
    // editor to `renderStagedArgControl`, which mints its own ids. Selecting the control by the row id
    // therefore never matched anything and every argument scored `absent` (jack `patchNodes` dispatched
    // with no args and was refused "missing value"). The row is the scope; the control is found inside.
    const filled = [];
    for (const [key, value] of Object.entries(config.args ?? {})) {
      const scopes = [`[id="action.${config.action}.arg.${key}"]`, `[id$=".arg.${key}"]`, `[id$="${key}"]`];
      let done = `${key}:absent`;
      // 🗨️ A verb reached through a DIALOG (`openAddObjectDialog` → `addObjectKind`, puzzle3d
      // `✏️editor/🦀️.rs:8619`) mounts its argument control as the dialog's own trigger button whose id
      // IS the arg key (`button#objectKind`), not a control nested in an Actions-pane row. Value
      // `"*"` takes whatever the first option is, which is what an agent with no catalogue knows.
      const trigger = page.locator(`button[id="${key}"], [id="${key}"][role="combobox"]`).first();
      if (await trigger.count()) {
        const option = value === "*"
          ? page.locator('[role="option"]').first()
          : page.locator('[role="option"]').filter({ hasText: new RegExp(`^\\s*${String(value).replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}\\s*$`, "i") }).first();
        done = await trigger.click({ force: true })
          .then(() => page.waitForTimeout(500))
          .then(() => option.click({ timeout: 5000, force: true }))
          .then(() => `${key}=${value}`)
          .catch((e) => `${key}:${String(e).split("\n")[0].slice(0, 80)}`);
        filled.push(done);
        continue;
      }
      for (const scope of scopes) {
        const select = page.locator(`${scope} select, select${scope}`).first();
        if (await select.count()) { done = await select.selectOption(String(value)).then(() => `${key}=${value}`).catch((e) => `${key}:${String(e).split("\n")[0].slice(0, 80)}`); break; }
        const combobox = page.locator(`${scope} [role="combobox"], [role="combobox"]${scope}`).first();
        if (await combobox.count()) {
          done = await combobox.click({ force: true })
            .then(() => page.waitForTimeout(400))
            .then(() => page.locator('[role="option"]').filter({ hasText: new RegExp(`^\\s*${String(value).replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}\\s*$`, "i") }).first().click({ timeout: 5000, force: true }))
            .then(() => `${key}=${value}`).catch((e) => `${key}:${String(e).split("\n")[0].slice(0, 80)}`);
          break;
        }
        const input = page.locator(`${scope} :is(input,textarea), :is(input,textarea)${scope}, [name="${key}"]`).first();
        if (await input.count()) { done = await input.fill(String(value)).then(() => `${key}=${value}`).catch((e) => `${key}:${String(e).split("\n")[0].slice(0, 80)}`); break; }
      }
      filled.push(done);
    }
    // 🩸️ B2b re-took the baseline AFTER staging the arguments, which scored every DEFAULTED verb as
    // inert: a row whose args all carry `default_value` dispatches on the row click itself (grid2d's
    // `change-seed` had already journaled "Change seed to 0" before the fill ran). The baseline
    // stays the pre-click witness; only the render string is re-read, for the staged-form diff.
    if (filled.length) await page.waitForTimeout(400);
    // 🩸️ The staged form's trigger is `childElementId("framework.window", windowId, "action", id,
    // "execute")`, and `childElementId` runs EVERY segment through `elementIdSegment`, which camelCases
    // it (`🆔️ElementId/🟦️.tsx:27`). The ROW keeps the raw id (`action.change-seed`) while its execute
    // control becomes `…​.action.changeSeed.execute`, so a dash-spelled verb needs both spellings —
    // b2b's single raw selector scored wfc's `change-seed` as having no trigger at all.
    // 🗨️ `#ui.dialog.submit` is the dialog's own trigger (`DialogDefinition::submit_label`); a pane row
    // with a staged form keeps the `…​.execute` control. Both are tried, dialog first when one is open.
    let submitted = await click(`[id="ui.dialog.submit"], [id$=".action.${config.action}.execute"], [id$=".action.${segment(config.action)}.execute"]`);
    const stagedIds = submitted === "ok" ? [] : await page.evaluate((ids) => [...document.querySelectorAll(ids.map((id) => `[id*="${id}"]`).join(", "))].map((el) => `${el.tagName.toLowerCase()}#${el.id}`).slice(0, 24), [config.action, segment(config.action)]);
    // 🩸️ Waiting for ANY witness change ended the settle on the shell's own chrome: clicking an Actions
    // row focuses its window, the host journals `shell.windowActivate` as an "Activate Window" ledger
    // row (`🏛️ShellHost/🟦️.tsx:10552`), the witness differed, and the wait returned BEFORE the verb's
    // own entry existed — wfc `bitmap`/`wfc2d` scored inert at both 25 s and 75 s while the very next
    // read (the undo step's) showed "Change seed to 1" sitting in the ledger. The uncommitted-edit
    // count is the only witness chrome rows never touch, so that is what the settle waits on; an app
    // with no check-in button (`edits === -1`) keeps the old any-change predicate.
    const settled = await until(
      before.edits < 0 ? (next) => !sameWitness(witness(next), before) : (next) => witness(next).edits > before.edits,
      SETTLE_MS,
    );
    shell = settled.shell;
    const after = witness(shell);
    const applied = after.ledger.filter((entry) => !entry.includes("~"));
    mutated = applied.length > before.ledger.filter((entry) => !entry.includes("~")).length && after.edits > before.edits;
    note("invoke-action", { action: config.action, clicked, filled, submitted, stagedIds, mutated, renderChanged: after.render !== before.render, before, after }, from);
  }

  let undone = false;
  if (config.action) {
    const from = lines.length;
    const afterInvoke = witness(shell);
    // ⏪️ The Actions pane's own `#action.undo` row, not the History panel's button: the pane overlays
    // the footer panel, so `framework.history.undo` is in the DOM but not hit-testable while the rail
    // this interaction was dispatched from is open. Both routes reach the same `undo` action.
    let clicked = await click('[id="action.undo"]');
    if (clicked !== "ok") clicked = await click('[id="framework.history.undo"] button, [id="framework.history.undo"]');
    const settled = await until((next) => witness(next).edits < afterInvoke.edits, SETTLE_MS);
    shell = settled.shell;
    const after = witness(shell);
    undone = after.edits < afterInvoke.edits && after.edits === before.edits;
    note("undo", { clicked, undone, before, afterInvoke, after }, from);
  }

  // ↷️ Redo is the bar's fourth clause: the retired edit must come back, and the chrome rows the
  // shell noted on the way must still not be undo targets (ticket 26/09/18 §3.2 laws A+B — a note
  // the shell declared no inverse for is logged with no `↶` and is never what undo/redo moves).
  let redone = false;
  if (config.action && undone) {
    const from = lines.length;
    const afterUndo = witness(shell);
    let clicked = await click('[id="action.redo"]');
    if (clicked !== "ok") clicked = await click('[id="framework.history.redo"] button, [id="framework.history.redo"]');
    const settled = await until((next) => witness(next).edits > afterUndo.edits, SETTLE_MS);
    shell = settled.shell;
    const after = witness(shell);
    redone = after.edits > afterUndo.edits;
    note("redo", { clicked, redone, afterUndo, after }, from);
  }

  // 🪞️ Law A's runtime witness: every ledger row the SHELL noted for its own chrome must be logged
  // without the revert affordance (`↶`), because no React call site declares an inverse for one.
  const CHROME = /^(Activate Window|Resize Window|Move Window|Toggle Panel|Switch Panel Tab|Set Theme|Reset Dock|Dock )/;
  const chromeRows = witness(shell).ledger.map((entry) => entry.slice(entry.indexOf(":") + 1)).filter((label) => CHROME.test(label));
  const chromeRevertible = chromeRows.filter((label) => label.includes("↶")).length;
  note("chrome-rows", { chromeRows: chromeRows.length, chromeRevertible, sample: chromeRows.slice(0, 8) }, lines.length);

  await page.screenshot({ path: join(OUT, `b3b-${config.plugin}.png`) });
  const faults = faultsSince(0);
  report.summary = {
    ready: shell.ready,
    error: shell.error,
    exampleRendered: report.steps.find((s) => s.step === "example-rendered")?.detail.rendered ?? false,
    actionCount: shell.actions.length,
    mutated,
    undone,
    redone,
    chromeRows: chromeRows.length,
    chromeRevertible,
    faultLines: faults.length,
    interactionBar: (report.steps.find((s) => s.step === "example-rendered")?.detail.rendered ?? false) && mutated && undone && redone && faults.length === 0 && !shell.error,
  };
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(OUT, `b3b-${config.plugin}-console.txt`), [
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

/** 🚦️ CLI entry: `bun 🐍️b3b-interaction-probe.mjs <plugin> <variant> <port> [action] [argsJson]`.
 * Omitting `action` runs a discovery pass (boot + rendered example + the Actions-pane inventory)
 * without dispatching anything, which is how each artifact's invokable verb is chosen. */
if (process.argv[1]?.includes("b3b-interaction-probe")) {
  const [plugin, variant, port, action, argsJson] = process.argv.slice(2);
  if (!plugin || !variant || !port) throw new Error("usage: <plugin> <variant> <port> [action] [argsJson]");
  const report = await runInteractionProbe({ plugin, variant, port: Number(port), action: action && action !== "-" ? action : undefined, args: argsJson ? JSON.parse(argsJson) : undefined });
  process.exit(report.summary.interactionBar ? 0 : 1);
}

