/** 🔬️ Headless runtime evidence for the puzzle 3d fill ToolRun in the React shell (:6013), contract §6.7 (a)–(d):
 * start from the ToolRun panel, trace grows with danger + success verdicts while the committed document stays
 * untouched, pause + step add exactly one record, abort retires the provisional placements and leaves history
 * untouched, a small run completes, finalizes as one history row and undoes. An init script samples the
 * perspective window's `data-tool-run-*` counters every 200 ms and survives vite reloads.
 * Ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS.
 *
 * Run from the ticket folder: `bun 🔍️w5-fill-run-probe.ts [--port=6013] [--count=12] [--run-seconds=30]`.
 * Outputs `🗑️generated/W5-mac-react-e2e/fill-run-<stamp>.{md,ndjson}` and screenshots. */
import { chromium, type Page } from "@playwright/test";
import { mkdirSync, writeFileSync, appendFileSync } from "node:fs";

const arg = (name: string, fallback: string) => process.argv.find((a) => a.startsWith(`--${name}=`))?.slice(name.length + 3) ?? fallback;
const port = arg("port", "6013");
const smallCount = arg("count", "12");
const runSeconds = Number(arg("run-seconds", "30"));
const completeSeconds = Number(arg("complete-seconds", "240"));
const out = `${import.meta.dir}/🗑️generated/W5-mac-react-e2e`;
mkdirSync(out, { recursive: true });
const stamp = new Date().toISOString().replace(/[:.]/g, "-");
const md = `${out}/fill-run-${stamp}.md`;
const nd = `${out}/fill-run-${stamp}.ndjson`;
const t0 = Date.now();
const log = (line: string) => {
  const text = `[${((Date.now() - t0) / 1000).toFixed(1)}s] ${line}`;
  console.log(text);
  appendFileSync(md, `${text}\n`);
};
const verdicts: { name: string; ok: boolean }[] = [];
const verdict = (name: string, ok: boolean, detail: unknown) => {
  verdicts.push({ name, ok });
  appendFileSync(nd, `${JSON.stringify({ name, ok, detail })}\n`);
  log(`${ok ? "PASS" : "FAIL"} ${name} ${JSON.stringify(detail).slice(0, 600)}`);
};
writeFileSync(md, `# fill run probe ${stamp}\n\n`);

type Sample = { t: number; run: string; gen: string; rec: number; testing: number; success: number; danger: number; warning: number; committed: number; provisional: number; status: string; valueNow: string; valueText: string };

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const faults: string[] = [];
const historyPatches: string[] = [];
const consoleTail: string[] = [];
page.on("console", (msg) => {
  const text = msg.text();
  consoleTail.push(text.slice(0, 300));
  if (consoleTail.length > 3000) consoleTail.shift();
  if (/DuplicateSiblingKey|SemioFaultError|panicked|unreachable|toolRun\.[a-z-]+/.test(text) && msg.type() !== "debug") faults.push(text.slice(0, 400));
  if (text.includes("history patch applied")) historyPatches.push(text.slice(0, 400));
});
page.on("pageerror", (error) => faults.push(`pageerror ${String(error).slice(0, 400)}`));
await page.routeWebSocket(/\/\?token=|vite-hmr/, () => {});
await page.addInitScript(() => {
  const w = window as unknown as { __w5: { samples: unknown[] } };
  w.__w5 = { samples: [] };
  setInterval(() => {
    const host = document.querySelector('[data-surface-id="window:puzzle3d-main-perspective"]');
    if (!host) return;
    const read = (name: string) => host.getAttribute(`data-tool-run-${name}`) ?? "";
    let committed = -1;
    let provisional = -1;
    try {
      const instances = JSON.parse(host.getAttribute("data-instances-json") ?? "[]") as { provisional?: boolean }[];
      provisional = instances.filter((instance) => instance.provisional).length;
      committed = instances.length - provisional;
    } catch {}
    const status = [...document.querySelectorAll('[aria-live="polite"],[aria-live="assertive"]')].map((node) => node.textContent ?? "").find((text) => /·|Ready to start/.test(text)) ?? "";
    const bar = document.querySelector('[role="progressbar"]');
    w.__w5.samples.push({ t: Math.round(performance.now()), run: read("run"), gen: read("generation"), rec: Number(read("records")), testing: Number(read("testing")), success: Number(read("success")), danger: Number(read("danger")), warning: Number(read("warning")), committed, provisional, status, valueNow: bar?.getAttribute("aria-valuenow") ?? "", valueText: bar?.getAttribute("aria-valuetext") ?? "" });
    if (w.__w5.samples.length > 20000) w.__w5.samples.shift();
  }, 200);
});

const samples = async (): Promise<Sample[]> => {
  for (let attempt = 0; attempt < 5; attempt++) {
    try {
      return await page.evaluate(() => (window as unknown as { __w5?: { samples: Sample[] } }).__w5?.samples ?? []);
    } catch {
      await page.waitForTimeout(1000);
    }
  }
  return [];
};
const latest = async () => (await samples()).at(-1);
const waitFor = async (what: string, predicate: (sample: Sample) => boolean, budgetMs: number) => {
  const started = Date.now();
  while (Date.now() - started < budgetMs) {
    const sample = await latest();
    if (sample && predicate(sample)) return { sample, waitedMs: Date.now() - started };
    await page.waitForTimeout(250);
  }
  return { sample: await latest(), waitedMs: Date.now() - started, timedOut: what };
};
const button = (page: Page, name: string) => page.getByRole("button", { name, exact: true });
const clearVeil = async () => {
  const veiled = () => page.evaluate(() => [...document.querySelectorAll<HTMLElement>(".ui-veil")].some((el) => getComputedStyle(el).pointerEvents === "auto" && el.getBoundingClientRect().width > 600)).catch(() => false);
  for (let attempt = 0; attempt < 4 && (await veiled()); attempt++) {
    await page.getByText("Skip", { exact: true }).first().click({ force: true, timeout: 2000 }).catch(() => {});
    await page.waitForTimeout(700);
  }
};
const RUN_CONTROLS = new Set(["Start", "Pause", "Resume", "Step", "Abort", "Finalize", "Dismiss"]);
const click = async (name: string, budgetMs = 60000) => {
  await clearVeil();
  const target = button(page, name).first();
  if (RUN_CONTROLS.has(name) && !(await target.isVisible().catch(() => false))) {
    const tab = button(page, "Tool runs").first();
    if ((await tab.getAttribute("aria-pressed").catch(() => null)) === "false") await tab.click({ timeout: 10000 }).catch(() => {});
  }
  await target.waitFor({ state: "visible", timeout: budgetMs });
  const started = Date.now();
  while (Date.now() - started < budgetMs && (await target.isDisabled().catch(() => true))) await page.waitForTimeout(250);
  await target.click({ timeout: budgetMs });
  log(`clicked ${name}`);
};
const shot = async (name: string) => page.screenshot({ path: `${out}/fill-run-${stamp}-${name}.png`, timeout: 15000 }).catch((error) => log(`screenshot ${name} failed ${error}`));

await page.goto(`http://127.0.0.1:${port}/?plugin=puzzle3d`);
let boot = await waitFor("boot", (sample) => sample.committed > 0, 300000);
for (let stable = 0; stable < 10; ) {
  await page.waitForTimeout(1000);
  const next = await latest();
  stable = next?.committed === boot.sample?.committed ? stable + 1 : 0;
  boot = { ...boot, sample: next };
}
log(`booted committed=${boot.sample?.committed} waitedMs=${boot.waitedMs}`);
await page.getByText("Skip", { exact: true }).first().click({ timeout: 5000 }).catch(() => {});
const committedBase = boot.sample?.committed ?? -1;

const activeUtility = () => page.evaluate(() => (JSON.parse(document.querySelector('[data-surface-id="window:puzzle3d-main-perspective"]')?.getAttribute("data-interaction-json") ?? "{}") as { activeUtility?: string }).activeUtility ?? "").catch(() => "");
await click("Tool runs");
if (!(await button(page, "Fill").first().isVisible().catch(() => false))) await click("Tool");
for (let attempt = 0; attempt < 3 && (await activeUtility()) !== "fill"; attempt++) {
  await page.waitForTimeout(3000);
  if ((await activeUtility()) !== "fill") await click("Fill");
}
log(`active utility ${await activeUtility()}`);
log("fill tool selected, tool runs panel open");
const ready = await button(page, "Start").first().waitFor({ state: "visible", timeout: 60000 }).then(() => true).catch(() => false);
verdict("ready-group-offers-start", ready, { status: (await latest())?.status });
if (!ready) {
  const dom = await page.evaluate(() => {
    const tab = document.getElementById("framework.panel.toolRun");
    const panels = [...document.querySelectorAll<HTMLElement>("[data-panel-body-key], [data-body-key], [role='tabpanel']")].map((node) => `${node.getAttribute("data-panel-body-key") ?? node.getAttribute("data-body-key") ?? node.id}: ${node.innerText.slice(0, 200).replace(/\n/g, " | ")}`);
    return { tabPressed: tab?.getAttribute("aria-pressed"), panels, toolRunKeys: [...document.querySelectorAll("[id^='framework.toolRun']")].map((node) => node.id) };
  }).catch((error) => String(error));
  log(`ready diagnostics ${JSON.stringify(dom).slice(0, 3000)}`);
  log(`refresh console tail ${JSON.stringify(consoleTail.filter((line) => /refresh|panel|toolRun|activeTool|setActiveTool/i.test(line)).slice(-40)).slice(0, 6000)}`);
}
await shot("ready");

const setCount = async (value: string) => {
  const count = page.getByRole("spinbutton").first();
  await count.fill(value).catch((error) => log(`count fill failed ${error}`));
  await count.press("Enter").catch(() => {});
  await page.waitForTimeout(2000);
  log(`count ${value}`);
};
const patchesBeforeRun = historyPatches.length;
await click("Start");
const filmstripFrom = (await samples()).length;
for (const frame of [1, 2, 3]) {
  await page.waitForTimeout(2500);
  await shot(`running-${frame}`);
}
const paced = (await samples()).slice(filmstripFrom).filter((sample) => sample.run !== "");
const jumps = paced.slice(1).map((sample, index) => sample.rec - paced[index]!.rec);
verdict("e-paced-attempts-appear-one-by-one", paced.length > 10 && new Set(paced.map((sample) => sample.rec)).size >= 10 && Math.max(0, ...jumps) <= 3, { samples: paced.length, distinctRecordCounts: new Set(paced.map((sample) => sample.rec)).size, largestJump: Math.max(0, ...jumps), first: paced[0], last: paced.at(-1) });
verdict("e-the-candidate-under-test-is-visible", paced.some((sample) => sample.testing === 1), { testingSamples: paced.filter((sample) => sample.testing === 1).length, of: paced.length });
const running = await waitFor("trace shows both verdicts", (sample) => sample.danger > 0 && sample.success > 0 && sample.provisional > 0, runSeconds * 1000);
const completeA = await waitFor("default run completes", (sample) => /Complete/.test(sample.status), completeSeconds * 1000);
await shot("complete-default");
const trail = (await samples()).filter((sample) => sample.run !== "");
const monotonic = trail.every((sample, index) => index === 0 || sample.run !== trail[index - 1]!.run || sample.gen !== trail[index - 1]!.gen || sample.rec >= trail[index - 1]!.rec);
const doneA = completeA.sample;
verdict("a-trace-shows-danger-and-success-with-provisional-pieces", !("timedOut" in running), { waitedMs: running.waitedMs, sample: running.sample });
verdict("a-trace-records-grow-monotonically", monotonic && trail.length > 1, { samples: trail.length, distinctRecordCounts: [...new Set(trail.map((sample) => sample.rec))].length, lastRecords: trail.at(-1)?.rec });
verdict("a-every-placed-piece-is-provisional", !("timedOut" in completeA) && doneA?.provisional === doneA?.success && (doneA?.provisional ?? 0) > 0, { sample: doneA });
verdict("a-committed-document-untouched-while-running", doneA?.committed === committedBase && historyPatches.length === patchesBeforeRun, { committedBase, committed: doneA?.committed, historyPatches: historyPatches.slice(patchesBeforeRun) });
verdict("a-progress-exposes-aria", /of \d+/.test(doneA?.valueText ?? "") && doneA?.valueNow !== "", { status: doneA?.status, valueNow: doneA?.valueNow, valueText: doneA?.valueText });
const abortButtons = await page.getByRole("button", { name: "Abort", exact: true }).evaluateAll((nodes) => nodes.map((node) => ({ id: node.id, disabled: (node as HTMLButtonElement).disabled, visible: node.getBoundingClientRect().width > 0, path: [...Array(4)].reduce<{ node: Element | null; ids: string[] }>((acc) => ({ node: acc.node?.parentElement ?? null, ids: [...acc.ids, acc.node?.parentElement?.id ?? ""] }), { node, ids: [] }).ids.filter(Boolean) }))).catch((error) => String(error));
log(`abort buttons ${JSON.stringify(abortButtons)}`);
const consoleMark = consoleTail.length;
await click("Abort");
const abortedA = await waitFor("aborted", (sample) => /Aborted/.test(sample.status) && sample.provisional === 0, 60000);
log(`abort console ${JSON.stringify(consoleTail.slice(consoleMark).filter((line) => /toolRun|Abort|abort|rejected|stale|Fault/i.test(line)).slice(0, 40)).slice(0, 8000)}`);
await page.waitForTimeout(1500);
const afterAbort = await latest();
verdict("c-abort-retires-provisional", !("timedOut" in abortedA), { sample: abortedA.sample });
verdict("c-abort-leaves-document-and-history-untouched", afterAbort?.committed === committedBase && historyPatches.length === patchesBeforeRun, { committed: afterAbort?.committed, historyPatches: historyPatches.slice(patchesBeforeRun) });
verdict("c-trace-stays-after-abort", (afterAbort?.rec ?? 0) > 0, { records: afterAbort?.rec });
await shot("aborted");

await setCount("5000");
await click("Start");
const busy = await waitFor("large run running", (sample) => /Running/.test(sample.status) && sample.rec > 0, 120000);
await click("Pause");
const paused = await waitFor("paused", (sample) => /Paused/.test(sample.status), 30000);
await page.waitForTimeout(2000);
const beforeStep = await latest();
await page.waitForTimeout(2000);
const stillPaused = await latest();
await click("Step");
await page.waitForTimeout(3000);
const afterStep = await latest();
const decided = (sample?: Sample) => (sample?.success ?? 0) + (sample?.danger ?? 0) + (sample?.warning ?? 0);
verdict("b-pause-reaches-paused-and-holds", !("timedOut" in busy) && !("timedOut" in paused) && stillPaused?.rec === beforeStep?.rec, { busy: busy.sample?.status, before: beforeStep, held: stillPaused });
const shownUnit = (afterStep?.rec ?? 0) - (beforeStep?.rec ?? 0) === 1 && afterStep?.testing === 1 && decided(afterStep) === decided(beforeStep);
const decidedUnit = decided(afterStep) - decided(beforeStep) === 1 && afterStep?.rec === beforeStep?.rec;
verdict("b-step-shows-exactly-one-visible-unit", (shownUnit || decidedUnit) && /Paused/.test(afterStep?.status ?? ""), { shownUnit, decidedUnit, before: beforeStep, after: afterStep });
await shot("stepped");
await click("Abort");
await waitFor("large run aborted", (sample) => /Aborted/.test(sample.status) && sample.provisional === 0, 60000);

await page.locator("body").click({ position: { x: 5, y: 450 } }).catch(() => {});
await page.keyboard.press("Control+Enter");
const chordStart = await waitFor("chord start", (sample) => /Running|Complete|Starting/.test(sample.status), 30000);
await page.keyboard.press("Control+.");
const chordAbort = await waitFor("chord abort", (sample) => /Aborted/.test(sample.status), 30000);
verdict("g-mod-enter-starts-and-mod-period-aborts", !("timedOut" in chordStart) && !("timedOut" in chordAbort), { start: chordStart.sample?.status, abort: chordAbort.sample?.status });

await setCount(smallCount);
await click("Start");
const complete = await waitFor("small run completes", (sample) => /Complete/.test(sample.status), 240000);
await shot("complete-small");
verdict("d-small-run-completes", !("timedOut" in complete) && complete.sample?.provisional === Number(smallCount), { waitedMs: complete.waitedMs, sample: complete.sample });
const patchesBeforeFinalize = historyPatches.length;
await click("Finalize");
const finalized = await waitFor("finalized", (sample) => /Finalized/.test(sample.status) && sample.provisional === 0 && sample.committed === committedBase + Number(smallCount), 120000);
await page.waitForTimeout(2000);
verdict("d-finalize-commits-every-placement", !("timedOut" in finalized), { committedBase, sample: await latest() });
verdict("d-finalize-reaches-history", historyPatches.length > patchesBeforeFinalize || consoleTail.slice(-400).some((line) => /history/i.test(line) && /toolRunFinalize|Fill|historyUpserts":[1-9]/.test(line)), { historyPatches: historyPatches.slice(patchesBeforeFinalize), historyLines: consoleTail.slice(-400).filter((line) => /history/i.test(line)).slice(-6) });
await shot("finalized");
await page.locator("body").click({ position: { x: 5, y: 450 } }).catch(() => {});
await page.keyboard.press("Control+z");
const undone = await waitFor("undo", (sample) => sample.committed === committedBase, 60000);
verdict("d-one-undo-removes-every-piece", !("timedOut" in undone), { sample: undone.sample });

verdict("no-hard-faults", faults.length === 0, faults.slice(0, 10));
const pass = verdicts.filter((entry) => entry.ok).length;
log(`fill-run PASS=${pass} FAIL=${verdicts.length - pass}`);
await browser.close();
