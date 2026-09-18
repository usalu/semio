/** 🩺️ Shared batch-B dormant-plugin boot probe (ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END).
 * Boots a react playground headless, settles the shell, records console errors/warnings and an
 * accessibility summary (window titles, panes, action count), then exercises one real interaction:
 * pick the default example from the Example combobox, open the primary window's Actions pane and
 * execute one mutation action, asserting the artifact state signature changed.
 * Every caller passes its own config through `runBootProbe`; the per-plugin wrappers are
 * `🐍️b1b-<plugin>-boot-probe.mjs`.
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

const TICKET = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END";
const OUT = join(TICKET, "🗑️generated");
const FAULT = /unreachable|trapped|\btrap\b|panicked|fault|refused|dropped action|not-ui-safe|missing-owned|invalid-args|unsupported|pageerror|Uncaught/i;
const NOISE = /staged plugin module\(s\) are behind their source|\[stale\]|Failed to load resource: the server responded with a status of 404|Download the (React|Vue) DevTools/;

/** 🧾️ Reads the whole shell surface the probe reasons about in one page evaluation. */
const readShell = (page) => page.evaluate(() => {
  const text = (el) => (el?.innerText ?? "").replace(/\s+/g, " ").trim();
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    error: document.documentElement.getAttribute("data-semio-os-error"),
    title: document.title,
    panes: [...document.querySelectorAll("[data-surface-id]")].map((el) => ({ id: el.getAttribute("data-surface-id"), canvases: el.querySelectorAll("canvas").length, svg: el.querySelectorAll("svg *").length, chars: text(el).length })),
    windowTitles: [...document.querySelectorAll("[data-window-kind-id]")].map((el) => el.getAttribute("data-window-kind-id")),
    toggles: [...document.querySelectorAll('[id$=".engagement.toggle"]')].map((el) => el.id),
    actions: [...new Set([...document.querySelectorAll('[id^="action."]')].map((el) => el.id))].filter((id) => !id.startsWith("action.category.")),
    tabs: [...document.querySelectorAll('[role="tab"]')].map((el) => text(el)),
    treeitems: [...document.querySelectorAll('[role="treeitem"]')].map((el) => text(el).slice(0, 80)),
    documentRows: [...document.querySelectorAll('[role="treeitem"]')].filter((el) => !el.closest('[id$=".engagement"]')).map((el) => text(el).slice(0, 80)),
    combobox: [...document.querySelectorAll('[role="combobox"]')].map((el) => text(el)),
    body: text(document.body),
  };
});

/** 🔬️ The artifact-state witness an interaction has to move: the text of every non-panel pane plus
 * the Inspection tree, which is what every one of these plugins projects its document into. */
/** 🔬️ The artifact-state witness an interaction has to move. It deliberately EXCLUDES every row
 * inside a window's `…​.engagement` (Actions) pane: expanding an action row to reach its arguments
 * is itself a tree change, and scoring that as the document change would pass a dead verb. */
const signature = (shell) => JSON.stringify({
  panes: shell.panes.map((pane) => `${pane.id}:${pane.chars}:${pane.svg}:${pane.canvases}`),
  rows: shell.documentRows,
});

async function runBootProbe(config) {
  const outDir = join(OUT, process.env.SEMIO_PROBE_OUT ?? `b1b-${config.plugin}`);
  mkdirSync(outDir, { recursive: true });
  const url = process.env.SEMIO_PROBE_URL ?? `http://127.0.0.1:${config.port}/?plugin=${config.variant}`;
  const lines = [];
  const t0 = Date.now();
  const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
  const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
  page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 4000 : 800)}`));
  page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
  const report = { plugin: config.plugin, variant: config.variant, port: config.port, url, startedAt: new Date().toISOString(), steps: [] };
  const faultsSince = (from) => lines.slice(from).filter((l) => FAULT.test(l) && !NOISE.test(l)).map((l) => l.slice(0, 600));
  const step = async (name, detail, from) => {
    report.steps.push({ step: name, ms: Date.now() - t0, detail, faults: faultsSince(from) });
    writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
    console.log(`[DEBUG] ${name} ${JSON.stringify(detail).slice(0, 900)}`);
  };
  const click = async (locator) => ((await locator.count()) ? locator.first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 140)) : "absent");

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
  await step("boot", { ready: shell.ready, error: shell.error, panes: shell.panes, tabs: shell.tabs, combobox: shell.combobox, windowTitles: shell.windowTitles }, 0);

  {
    const from = lines.length;
    const before = signature(shell);
    const box = page.locator('[role="combobox"]').first();
    const opened = await click(box);
    await page.waitForTimeout(1200);
    const option = page.locator('[role="option"]', { hasText: config.exampleLabel }).first();
    const picked = await click(option);
    await page.waitForTimeout(1000);
    let after = shell;
    for (let i = 0; i < 40; i++) { await page.waitForTimeout(500); after = await readShell(page); if (signature(after) !== before) break; }
    await step("load-example", { opened, picked, changed: signature(after) !== before, combobox: after.combobox }, from);
    shell = after;
  }

  {
    const from = lines.length;
    for (const toggle of shell.toggles) { await click(page.locator(`[id="${toggle}"]`)); await page.waitForTimeout(900); }
    shell = await readShell(page);
    await step("open-actions-pane", { toggles: shell.toggles, actionCount: shell.actions.length, actions: shell.actions.slice(0, 60) }, from);
  }

  {
    const from = lines.length;
    // 🧷️ An argument-less action row IS its own trigger: the click dispatches straight away and no
    // `.execute` control is ever rendered, so the witness must be taken BEFORE the row is touched.
    const before = signature(shell);
    const row = page.locator(`[id="action.${config.action}"]`);
    const expanded = await click(row);
    await page.waitForTimeout(1200);
    const filled = [];
    for (const [key, value] of Object.entries(config.args ?? {})) {
      const select = page.locator(`select[id$="${key}"], select[name="${key}"]`).first();
      if (await select.count()) { filled.push(await select.selectOption(String(value)).then(() => `${key}=${value}`).catch((e) => `${key}:${String(e).slice(0, 80)}`)); continue; }
      const input = page.locator(`[id$="${key}"]:is(input,textarea), [name="${key}"]`).first();
      filled.push((await input.count()) ? await input.fill(String(value)).then(() => `${key}=${value}`).catch((e) => `${key}:${String(e).slice(0, 80)}`) : `${key}:absent`);
    }
    const submitted = await click(page.locator(`[id$=".action.${config.action}.execute"]`));
    let after = shell;
    for (let i = 0; i < 40; i++) { await page.waitForTimeout(500); after = await readShell(page); if (signature(after) !== before) break; }
    const changed = signature(after) !== before;
    await step("invoke-action", { action: config.action, expanded, filled, submitted, changed, before: before.slice(0, 300), after: signature(after).slice(0, 300) }, from);
    shell = after;
  }

  await page.screenshot({ path: join(OUT, `b1b-${config.plugin}.png`) });
  const faults = faultsSince(0);
  report.summary = {
    ready: shell.ready,
    error: shell.error,
    panes: shell.panes.length,
    windowTitles: shell.windowTitles,
    actionCount: shell.actions.length,
    exampleLoaded: report.steps.find((s) => s.step === "load-example")?.detail.changed ?? false,
    interactionChangedState: report.steps.find((s) => s.step === "invoke-action")?.detail.changed ?? false,
    faultLines: faults.length,
    booted: !shell.error && shell.ready === config.readyId && faults.length === 0 && (report.steps.find((s) => s.step === "invoke-action")?.detail.changed ?? false),
  };
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(OUT, `b1b-${config.plugin}-console.txt`), [
    `# ${config.plugin} (${config.variant}) ${url} ${report.startedAt}`,
    `# summary ${JSON.stringify(report.summary)}`,
    "",
    "## faults",
    ...faults,
    "",
    "## accessibility summary",
    JSON.stringify({ windowTitles: shell.windowTitles, panes: shell.panes, tabs: shell.tabs, actions: shell.actions, combobox: shell.combobox }, null, 1),
    "",
    "## console",
    ...lines,
  ].join("\n"));
  console.log("[DEBUG] SUMMARY", JSON.stringify(report.summary));
  await browser.close();
  return report;
}

export { runBootProbe, OUT, TICKET };
