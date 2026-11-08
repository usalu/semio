/** 🐤️ Energy canary probe: per language (en-US, de-DE) boot, fixture shown, one document action (create-surface on the demo = BESTEST 600: zone 1, construction 30) with undo and redo, no console errors. Exit 0 only when every check passes.
 * Selectors verified statically against the shell (ShellHost data-semio-os-*, window `[data-slot="window"]`, `.engagement.toggle`, `[data-slot="window-action-pane"]`, `action.<verb>`, `action.<verb>.arg.<arg>`, `*.action.<verb>.execute`, mod+z / mod+shift+z) and the energy editor (create-surface labels "Create surface"/"Fläche anlegen", structure leaves `surfaces: N`). */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.CANARY_URL;
const out = process.env.CANARY_OUT;
const bootSeconds = Number(process.env.CANARY_BOOT_SECONDS ?? 240);
const verb = process.env.CANARY_ACTION ?? "create-surface";
const args = JSON.parse(process.env.CANARY_ACTION_ARGS ?? '{"name":"Canary surface","zone":"1","construction":"30","class":"exteriorWall"}');
const languages = [
  { language: "en", locale: "en-US", rowLabel: /Create surface/i },
  { language: "de", locale: "de-DE", rowLabel: /Fläche anlegen/i },
];
mkdirSync(out, { recursive: true });
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const noisy = /favicon|Download the React DevTools|\[vite\]|Autofocus processing/;
const report = { url, languages: {} };
const diagnosticLine = /JSPI|WebAssembly|trapped|unreachable|panicked|ceiling|executing|turn (bound|limit)|budget|fuel|work ?limit|refus|retain|retire|wake|timer|executor|yield|frame-?worker|publication|patch|out of memory|memory|Fault|failed|crashed|cancel/i;
let failed = false;

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
for (const { language, locale, rowLabel } of languages) {
  const checks = [];
  const check = (name, ok, detail) => { checks.push({ name, ok: Boolean(ok), detail }); if (!ok) failed = true; console.log(`[${language}] ${ok ? "PASS" : "FAIL"} ${name} ${JSON.stringify(detail ?? "").slice(0, 500)}`); };
  const context = await browser.newContext({ locale, viewport: { width: 1600, height: 1000 } });
  const page = await context.newPage();
  const lines = [], errors = [], snapshots = [], failedRequests = [];
  await page.addInitScript(() => { window.__canaryLongTasks = []; try { new PerformanceObserver((l) => { for (const e of l.getEntries()) window.__canaryLongTasks.push({ ms: Math.round(e.duration), at: Math.round(e.startTime) }); }).observe({ entryTypes: ["longtask"] }); } catch {} });
  const t0 = Date.now();
  page.on("console", (m) => { const l = `${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1200)}`; lines.push(l); if (m.type() === "error" && !noisy.test(m.text())) errors.push(l); });
  page.on("requestfailed", (r) => failedRequests.push(`${r.method()} ${r.url().slice(0, 200)} ${r.failure()?.errorText}`));
  page.on("pageerror", (e) => { const l = `${Date.now() - t0} pageerror ${String(e).slice(0, 1200)}`; lines.push(l); errors.push(l); });
  const state = () => page.evaluate(() => {
    const body = document.body.innerText.replace(/\s+/g, " ");
    const surfaces = body.match(/surfaces: (\d+)/);
    const geometry = body.match(/Geometry: (\d+)/);
    const scope = Array.from(document.querySelectorAll(".semio-scope[data-shell-id]")).find((e) => e.getBoundingClientRect().width > 0);
    return {
      ready: document.querySelector("[data-semio-os-ready]") !== null,
      error: document.querySelector("[data-semio-os-error]")?.getAttribute("data-semio-os-error") ?? null,
      lang: scope?.lang || document.documentElement.lang,
      surfaces: surfaces ? Number(surfaces[1]) : null,
      geometry: geometry ? Number(geometry[1]) : null,
      model: /\(v\d+\)/.test(body),
      toggles: document.querySelectorAll('[id$=".engagement.toggle"]').length,
      body: body.slice(0, 1500),
    };
  });
  const metric = (s) => s.surfaces ?? s.geometry;
  const diagnostics = (phase) => page.evaluate(() => {
    const scope = Array.from(document.querySelectorAll(".semio-scope[data-shell-id]")).find((e) => e.getBoundingClientRect().width > 0);
    const w = window;
    const ledger = (w.__semioInputLedger?.recent ?? []).map((r) => ({ seq: r.inputSeq, action: r.action, outcome: r.outcome?.kind, detail: r.outcome?.reason ?? r.outcome?.message })).filter((r) => r.outcome && !/^(applied|dispatched|ok|accepted|committed)$/i.test(r.outcome)).slice(-20);
    const probe = w.__semioOsCatalogProbe;
    const mem = performance.memory ? { usedMb: Math.round(performance.memory.usedJSHeapSize / 1048576), limitMb: Math.round(performance.memory.jsHeapSizeLimit / 1048576) } : null;
    return {
      shellReady: scope?.getAttribute("data-shell-ready") ?? null,
      shellError: scope?.getAttribute("data-shell-error") ?? null,
      shellNotFound: scope?.getAttribute("data-shell-not-found") ?? null,
      osReady: document.querySelector("[data-semio-os-ready]") !== null,
      osError: document.querySelector("[data-semio-os-error]")?.getAttribute("data-semio-os-error") ?? null,
      catalogProbe: probe === undefined ? null : JSON.parse(JSON.stringify(probe, (k, v) => (typeof v === "function" ? undefined : v))),
      nonOkInputs: ledger,
      jsHeap: mem,
      jspi: typeof WebAssembly !== "undefined" && "Suspending" in WebAssembly,
      workers: performance.getEntriesByType("resource").filter((e) => /worker|\.wasm/i.test(e.name)).map((e) => ({ name: e.name.split("/").slice(-2).join("/"), ms: Math.round(e.duration), kb: Math.round((e.transferSize || 0) / 1024) })).slice(0, 40),
      longTasks: (w.__canaryLongTasks ?? []).slice(-20),
    };
  }).then((d) => { snapshots.push({ phase, t: Date.now() - t0, ...d }); return d; }).catch(() => null);
  const shot = (name) => page.screenshot({ path: join(out, `${language}-${name}.png`) }).catch(() => {});
  try {
    await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120000 });
    let s = null;
    for (let i = 0; i < bootSeconds; i++) { await sleep(1000); s = await state().catch(() => null); if (s && (s.error || (s.ready && s.toggles > 0 && s.model && i > 8))) break; }
    await shot("boot");
    const bootDiag = await diagnostics("boot");
    check("shell reports no fault", bootDiag && !bootDiag.shellError && !bootDiag.shellNotFound && !bootDiag.osError, { shellReady: bootDiag?.shellReady, shellError: bootDiag?.shellError, shellNotFound: bootDiag?.shellNotFound, osError: bootDiag?.osError });
    check("boot", s && s.ready && !s.error, { ready: s?.ready, error: s?.error, toggles: s?.toggles });
    check("fixture shown", s && s.model && metric(s) !== null, { model: s?.model, surfaces: s?.surfaces, geometry: s?.geometry, sample: s?.body.slice(0, 200) });
    check("language applied", s && s.lang?.toLowerCase().startsWith(language), { lang: s?.lang });
    if (s?.ready && !s.error && s.toggles > 0) {
      const windows = page.locator('[data-slot="window"]').filter({ has: page.locator('[id$=".engagement.toggle"]') });
      const count = await windows.count();
      let pane = windows.first();
      for (let i = 0; i < count; i++) if (/Geometry|Structure|Struktur/.test((await windows.nth(i).innerText().catch(() => "")))) { pane = windows.nth(i); break; }
      const rail = pane.locator('[data-slot="window-action-pane"]').first();
      if (!(await rail.isVisible().catch(() => false))) await pane.locator('[id$=".engagement.toggle"]').first().click({ timeout: 8000, force: true }).catch(() => {});
      await rail.waitFor({ state: "visible", timeout: 8000 }).catch(() => {});
      const row = pane.locator(`[id="action.${verb}"]`).first();
      await sleep(800);
      const rowText = (await row.innerText().catch(() => "")).replace(/\s+/g, " ").trim();
      const verbs = await pane.locator('[id^="action."]').evaluateAll((els) => els.map((e) => e.id).slice(0, 40)).catch(() => []);
      check("action row present, localized", rowLabel.test(rowText), { rowText, verbs });
      if (rowText) {
        const before = await state();
        const execute = pane.locator(`[id$=".action.${verb}.execute"]`).first();
        if (!(await execute.count())) await row.click({ timeout: 8000, force: true }).catch(() => {});
        await execute.waitFor({ state: "attached", timeout: 6000 }).catch(() => {});
        const filled = [];
        for (const [key, value] of Object.entries(args)) {
          const input = pane.locator(`[id="action.${verb}.arg.${key}"] input`).first();
          if (await input.count()) { await input.fill(String(value)).catch((e) => filled.push(`${key}:${String(e).slice(0, 60)}`)); filled.push(`${key}=${value}`); } else filled.push(`${key}:absent`);
        }
        const disabled = await execute.isDisabled().catch(() => null);
        await execute.click({ timeout: 8000 }).catch(() => {});
        let after = before;
        for (let i = 0; i < 40; i++) { await sleep(500); after = await state(); if (metric(after) !== metric(before)) break; }
        await shot("action");
        await diagnostics("after-action");
        check("action applied", metric(before) !== null && metric(after) === metric(before) + 1, { filled, executeDisabled: disabled, before: metric(before), after: metric(after) });
        const mod = "ControlOrMeta";
        await page.evaluate(() => { if (document.activeElement instanceof HTMLElement) document.activeElement.blur(); });
        await page.keyboard.press(`${mod}+z`);
        let undone = after;
        for (let i = 0; i < 20; i++) { await sleep(500); undone = await state(); if (metric(undone) === metric(before)) break; }
        check("undo", metric(undone) === metric(before), { value: metric(undone) });
        await page.keyboard.press(`${mod}+Shift+z`);
        let redone = undone;
        for (let i = 0; i < 20; i++) { await sleep(500); redone = await state(); if (metric(redone) === metric(after)) break; }
        await shot("redo");
        await diagnostics("after-redo");
        check("redo", metric(redone) === metric(after), { value: metric(redone) });
      }
    }
  } catch (e) { check("probe crashed", false, String(e).slice(0, 300)); }
  const endDiag = await diagnostics("end");
  check("no refused or failed inputs, no failed plugins", endDiag && endDiag.nonOkInputs.length === 0 && !JSON.stringify(endDiag.catalogProbe ?? {}).match(/failed|crashed/i), { nonOkInputs: endDiag?.nonOkInputs?.slice(0, 5), catalogProbe: JSON.stringify(endDiag?.catalogProbe ?? null).slice(0, 300) });
  check("no failed requests", failedRequests.length === 0, failedRequests.slice(0, 5));
  writeFileSync(join(out, `diagnostics-${language}.json`), JSON.stringify({ snapshots, failedRequests, diagnosticConsoleLines: lines.filter((l) => diagnosticLine.test(l)).slice(0, 300) }, null, 2));
  check("no console errors", errors.length === 0, errors.slice(0, 5));
  writeFileSync(join(out, `console-${language}.txt`), lines.join("\n"));
  report.languages[language] = checks;
  await context.close();
}
await browser.close();
writeFileSync(join(out, "report.json"), JSON.stringify(report, null, 2));
console.log(failed ? "CANARY RESULT: FAIL" : "CANARY RESULT: PASS");
process.exit(failed ? 1 : 0);
