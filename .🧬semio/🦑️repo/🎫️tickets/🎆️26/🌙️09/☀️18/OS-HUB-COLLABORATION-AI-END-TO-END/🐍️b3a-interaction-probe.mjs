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
    windowIds: [...document.querySelectorAll('[data-slot="window"]')].map((el) => el.id),
    toggles: [...document.querySelectorAll('[id$=".engagement.toggle"]')].map((el) => el.id),
    actions: [...new Set([...document.querySelectorAll('[id^="action."]')].map((el) => el.id))].filter((id) => !id.startsWith("action.category.") && !/\.arg\./.test(id)),
    tabs: [...document.querySelectorAll('[role="tab"]')].map((el) => text(el)),
    documentRows: [...document.querySelectorAll('[role="treeitem"]')].filter((el) => !el.closest('[id$=".engagement"]') && !el.closest('[data-slot="panel"]')).map((el) => text(el).slice(0, 80)),
    // 🪞️ The app's OWN panel tabs (artifact / inspector / catalogue …), which is where a
    // canvas-painting plugin publishes the only textual projection of its document. The framework's
    // history panel is excluded so the ledger cannot masquerade as the document reflecting state.
    panelRows: [...document.querySelectorAll('[data-slot="panel"]')].filter((el) => visible(el) && !el.id.startsWith("framework.panelTab.framework.panel.")).flatMap((panel) => [...panel.querySelectorAll('[role="treeitem"]')].map((el) => `${panel.id}|${text(el).slice(0, 80)}`)),
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
  render: JSON.stringify({ panes: shell.panes.map((pane) => `${pane.id}:${pane.chars}:${pane.svg}:${pane.canvases}`), rows: shell.documentRows, panelRows: shell.panelRows }),
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
  const rendered = shell.panes.some((pane) => pane.chars > 0 || pane.svg > 0 || pane.canvases > 0) || shell.documentRows.length > 0;
  note("example-rendered", { rendered, combobox: shell.combobox, paneChars: shell.panes.map((pane) => pane.chars), rows: shell.documentRows.length, sample: shell.documentRows.slice(0, 6) }, 0);

  {
    const from = lines.length;
    const opened = await click('[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]');
    await page.waitForTimeout(1500);
    shell = await readShell(page);
    note("open-history", { opened, openPanels: shell.openPanels, ledger: shell.ledger.length, checkin: shell.checkin }, from);
  }

  // 🪞️ The app's own panel tabs, opened BEFORE the witness so "the panel reflects state" is measured
  // on a projection that is actually mounted. `framework.panel.inspection`/`.artifact` are the two
  // every artifact editor declares; a plugin that names others passes them as `config.panels`.
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
    for (const toggle of shell.toggles) {
      await click(`[id="${toggle}"]`);
      await page.waitForTimeout(900);
    }
    shell = await readShell(page);
    note("open-actions", { toggles: shell.toggles, actionCount: shell.actions.length, actions: shell.actions.slice(0, 60), present: shell.actions.includes(`action.${config.action}`) }, from);
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
    const row = `[id="action.${config.action}"]`;
    const clicked = await click(row);
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
      filled.push((await input.count()) ? await input.fill(String(value)).then(() => `${key}=${value}`).catch((e) => `${key}:${String(e).split("\n")[0].slice(0, 80)}`) : `${key}:absent`);
    }
    if (filled.length) {
      await page.waitForTimeout(400);
      before = witness(await readShell(page));
    }
    const submitted = await click(`[id$=".action.${config.action}.execute"]`);
    const settled = await until((next) => !sameWitness(witness(next), before), 25_000);
    shell = settled.shell;
    const after = witness(shell);
    const applied = after.ledger.filter((entry) => !entry.includes("~"));
    mutated = applied.length > before.ledger.filter((entry) => !entry.includes("~")).length && after.edits > before.edits;
    note("invoke-action", { action: config.action, clicked, filled, submitted, mutated, renderChanged: after.render !== before.render, before, after }, from);
  }

  let undone = false;
  {
    const from = lines.length;
    const afterInvoke = witness(shell);
    // ⏪️ The Actions pane's own `#action.undo` row, not the History panel's button: the pane overlays
    // the footer panel, so `framework.history.undo` is in the DOM but not hit-testable while the rail
    // this interaction was dispatched from is open. Both routes reach the same `undo` action.
    let clicked = await click('[id="action.undo"]');
    if (clicked !== "ok") clicked = await click('[id="framework.history.undo"] button, [id="framework.history.undo"]');
    const settled = await until((next) => witness(next).edits < afterInvoke.edits, 25_000);
    shell = settled.shell;
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
    let clicked = await click('[id="action.redo"]');
    if (clicked !== "ok") clicked = await click('[id="framework.history.redo"] button, [id="framework.history.redo"]');
    const settled = await until((next) => witness(next).edits > afterUndo.edits, 25_000);
    shell = settled.shell;
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
