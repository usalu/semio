/** 🩺️ Shared batch-B interaction probe (ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END, slice B3d).
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

export async function runInteractionProbe(config) {
  const outDir = join(OUT, `b3d-${config.plugin}`);
  mkdirSync(outDir, { recursive: true });
  const url = process.env.SEMIO_PROBE_URL ?? `http://127.0.0.1:${config.port}/?plugin=${config.variant}`;
  const lines = [];
  const t0 = Date.now();
  const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
  const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
  page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 4000 : 1200)}`));
  page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
  // 🧩️ Which guest components the host actually fetched — the witness that an extension/module was
  // activated on demand (`on-extension-request`) rather than merely declared in the receipt.
  const moduleRequests = [];
  page.on("requestfinished", (request) => {
    const url = request.url();
    if (!/plugin-modules|extensions?\//.test(url)) return;
    if (!/\.(wasm|js|mjs|json)(\?|$)/.test(url)) return;
    moduleRequests.push(`${Date.now() - t0} ${request.method()} ${url.replace(/^https?:\/\/[^/]+/, "")}`);
  });
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

  // ⏱️ The repo host runs at load average ~100 with peer cargo fleets; the 30 s default navigation
  // budget times out on a Vite dev server that is otherwise healthy (it answers `curl` in <1 s).
  page.setDefaultNavigationTimeout(180_000);
  await page.goto(url, { waitUntil: "domcontentloaded", timeout: 180_000 });
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
      const input = page.locator(`[id$=".arg.${key}"]:is(input,textarea), [id$="${key}"]:is(input,textarea), [name="${key}"]`).first();
      filled.push((await input.count()) ? await input.fill(String(value)).then(() => `${key}=${value}`).catch((e) => `${key}:${String(e).split("\n")[0].slice(0, 80)}`) : `${key}:absent`);
    }
    if (filled.length) {
      await page.waitForTimeout(400);
      before = witness(await readShell(page));
    }
    const submitted = await click(`[id$=".action.${config.action}.execute"]`);
    // ⏳️ Only an APPLIED edit counts as settled — an incidental `Activate Window` config entry also
    // moves the witness and used to end the wait before the plugin's own mutation had landed.
    const settled = await until((next) => witness(next).edits > before.edits, 30_000);
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

  await page.screenshot({ path: join(OUT, `b3d-${config.plugin}.png`) });
  const faults = faultsSince(0);
  report.moduleRequests = moduleRequests;
  report.summary = {
    ready: shell.ready,
    error: shell.error,
    exampleRendered: report.steps.find((s) => s.step === "example-rendered")?.detail.rendered ?? false,
    actionCount: shell.actions.length,
    mutated,
    undone,
    faultLines: faults.length,
    interactionBar: (report.steps.find((s) => s.step === "example-rendered")?.detail.rendered ?? false) && mutated && undone && faults.length === 0 && !shell.error,
  };
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(OUT, `b3d-${config.plugin}-console.txt`), [
    `# ${config.plugin} (${config.variant}) ${url} ${report.startedAt}`,
    `# summary ${JSON.stringify(report.summary)}`,
    "",
    "## faults",
    ...faults,
    "",
    "## steps",
    ...report.steps.map((s) => `${s.step} ${s.ms}ms ${JSON.stringify(s.detail)}`),
    "",
    "## module requests",
    ...moduleRequests,
    "",
    "## console",
    ...lines,
  ].join("\n"));
  console.log("SUMMARY", JSON.stringify(report.summary));
  await browser.close();
  return report;
}

export { OUT, TICKET };
