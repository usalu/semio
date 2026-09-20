/** 🩺️ Slice B3a2 — the whole `🌍️gis` per-feature editing algebra in ONE live shell session.
 *
 * §9.2 landed `addFeature`/`moveFeature`/`renameFeature`/`deleteFeature` as fully defaulted rail
 * verbs, and designed them to chain with nothing typed: `addFeature` mints the lowest free
 * `position-N`, and an empty `featureId` addresses the collection's newest entry, so the other three
 * land on whatever `addFeature` just created. `🐍️b3a-gis2d-probe.mjs` proves ONE verb per browser
 * run; this walks all four in sequence against the same document and takes an undo/redo round trip on
 * each, which is the only way to see that the chain composes the way the native tests say it does.
 *
 * Witness per step: the History ledger's applied rows, the `#s-checkin` uncommitted-edit count, and
 * the app's own panel tree rows (`framework.panel.inspection`), which is where the document's
 * per-collection extents are published.
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const TICKET = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END";
const OUT = join(TICKET, "🗑️generated");
const PORT = Number(process.env.SEMIO_PROBE_PORT ?? 6040);
const FAULT = /unreachable|trapped|\btrap\b|panicked|fault|refused|dropped action|not-ui-safe|missing-owned|invalid-args|unsupported|pageerror|Uncaught|dispatch-failed/i;
const NOISE = /staged plugin module\(s\) are behind their source|\[stale\]|status of 404|Download the (React|Vue) DevTools|typed-operation slots|WebSocket connection to 'ws:\/\/[^']*\/bridge' failed: Error in connection establishment: net::ERR_CONNECTION_REFUSED/;

const read = (page) => page.evaluate(() => {
  const text = (el) => (el?.innerText ?? "").replace(/\s+/g, " ").trim();
  const visible = (el) => el instanceof HTMLElement && el.offsetParent !== null;
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    error: document.documentElement.getAttribute("data-semio-os-error"),
    ledger: [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((el) => !el.id.endsWith(".revert")).map((el) => text(el).slice(0, 72)),
    checkin: text(document.querySelector("#s-checkin")),
    panelRows: [...document.querySelectorAll('[data-slot="panel"]')]
      .filter((el) => visible(el) && !el.id.includes("framework.panel.history"))
      .flatMap((panel) => [...panel.querySelectorAll('[role="treeitem"]')].map((el) => text(el).slice(0, 72))),
    actions: [...new Set([...document.querySelectorAll('[id^="action."]')].map((el) => el.id))],
  };
});

const edits = (shell) => { const m = /\((\d+)\)\s*$/.exec(shell.checkin ?? ""); return m === null ? 0 : Number(m[1]); };

/** 🪞️ The app's own inspector rows, read by switching the single panel slot to it and back. */
const inspectorRows = async (page, click) => {
  await click('[data-slot="panel-tab-button"][id="framework.panel.inspection"], [id="framework.panel.inspection"]');
  await page.waitForTimeout(1500);
  const rows = (await read(page)).panelRows;
  await click('[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]');
  await page.waitForTimeout(1200);
  return rows;
};

const main = async () => {
  mkdirSync(OUT, { recursive: true });
  const lines = [];
  const t0 = Date.now();
  const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
  const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
  page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1500)}`));
  page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1500)}`));
  const report = { port: PORT, startedAt: new Date().toISOString(), steps: [] };
  const save = () => writeFileSync(join(OUT, "b3a2-gis-verb-sweep.json"), JSON.stringify(report, null, 2));
  const click = async (selector) => {
    if (!(await page.locator(selector).count())) return "absent";
    return page.locator(selector).first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).split("\n")[0].slice(0, 110));
  };

  await page.goto(`http://127.0.0.1:${PORT}/?plugin=gis2d`, { waitUntil: "domcontentloaded" });
  let shell = null;
  for (let i = 0; i < 240; i += 1) {
    await page.waitForTimeout(1000);
    shell = await read(page);
    if (shell.error || (shell.ready && i > 8)) break;
  }
  await page.waitForTimeout(4000);

  // 🪟️ History stays the OPEN tab for the whole run — it is the only source of the ledger rows — and
  // the app's own inspector is read by switching to it at each witness point and back (one panel slot).
  await click('[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]');
  await page.waitForTimeout(1500);
  for (const toggle of await page.locator('[id$=".engagement.toggle"]').all()) {
    await toggle.click({ timeout: 8000, force: true }).catch(() => {});
    await page.waitForTimeout(900);
  }
  shell = await read(page);
  report.steps.push({ step: "boot", ready: shell.ready, error: shell.error, panelRows: shell.panelRows, verbsPresent: ["addFeature", "moveFeature", "renameFeature", "deleteFeature"].filter((id) => shell.actions.includes(`action.${id}`)) });
  save();

  // 🔗️ add → move → rename → delete, each with its own undo/redo round trip on the SAME document.
  // 📍️ `moveFeature` is staged with a real destination: its `lon`/`lat` defaults are 0/0 and so are
  // `addFeature`'s, so an all-defaults move re-seats the minted position onto its own coordinate and
  // correctly diffs to nothing — a no-op, not a broken verb. Typing a destination is what a user does.
  const STAGED = { moveFeature: { lon: 11.25, lat: 48.5 } };
  for (const action of ["addFeature", "moveFeature", "renameFeature", "deleteFeature"]) {
    const from = lines.length;
    const before = await read(page);
    await click(`[id="action.${action}"]`);
    await page.waitForTimeout(1200);
    for (const [key, value] of Object.entries(STAGED[action] ?? {})) {
      const input = page.locator(`[id$=".arg.${key}"]:is(input,textarea), [id$="${key}"]:is(input,textarea), [name="${key}"]`).first();
      if (await input.count()) await input.fill(String(value)).catch(() => {});
      await page.waitForTimeout(300);
    }
    const submitted = await click(`[id$=".action.${action}.execute"]`);
    await page.waitForTimeout(6000);
    const after = await read(page);
    await click('[id="action.undo"]');
    await page.waitForTimeout(5000);
    const undone = await read(page);
    await click('[id="action.redo"]');
    await page.waitForTimeout(5000);
    const redone = await read(page);
    report.steps.push({
      step: action,
      submitted,
      editsBefore: edits(before),
      editsAfter: edits(after),
      editsUndone: edits(undone),
      editsRedone: edits(redone),
      mutated: edits(after) > edits(before) && after.ledger.length > before.ledger.length,
      undone: edits(undone) < edits(after),
      redone: edits(redone) === edits(after),
      newLedgerRow: after.ledger.slice(before.ledger.length)[0] ?? null,

      faults: lines.slice(from).filter((l) => FAULT.test(l) && !NOISE.test(l)).map((l) => l.slice(0, 500)),
    });
    save();
    console.log(action, JSON.stringify(report.steps.at(-1)).slice(0, 700));
  }

  report.inspectorAtEnd = await inspectorRows(page, click);
  const faults = lines.filter((l) => FAULT.test(l) && !NOISE.test(l));
  report.summary = {
    ready: shell.ready,
    verbs: report.steps.filter((s) => s.step !== "boot").map((s) => `${s.step}:${s.mutated ? "M" : "-"}${s.undone ? "U" : "-"}${s.redone ? "R" : "-"}`),
    faultLines: faults.length,
  };
  save();
  writeFileSync(join(OUT, "b3a2-gis-verb-sweep-console.txt"), [`# ${JSON.stringify(report.summary)}`, "", "## faults", ...faults, "", "## console", ...lines].join("\n"));
  console.log("SUMMARY", JSON.stringify(report.summary));
  await browser.close();
};

await main();
