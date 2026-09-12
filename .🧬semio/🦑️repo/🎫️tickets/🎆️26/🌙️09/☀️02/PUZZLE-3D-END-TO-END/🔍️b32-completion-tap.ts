/** 🪝️ Wave B32b — one page, diagnostics ARMED, three Document-scope mutations dispatched through the
 * world host's own `onAction` (the temporary `__b32Dispatch` tap), so the completion→refresh→guest-answer
 * hop table is read for a BROKEN verb and a WORKING verb under identical conditions.
 *
 * Run: `bun 🔍️b32-completion-tap.ts [--port=6013]`. Writes the whole console stream to
 * `🗑️generated/b32-tap-<stamp>.txt` and prints the hop table on stdout.
 */
import { chromium } from "playwright";
import { appendFileSync, mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const TICKET = import.meta.dir;
const OUT = join(TICKET, "🗑️generated");
mkdirSync(OUT, { recursive: true });
const stamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19);
const LOG = join(OUT, `b32-tap-${stamp}.txt`);
writeFileSync(LOG, "");
const port = process.argv.find((a) => a.startsWith("--port="))?.slice(7) ?? "6013";
const log = (line: string) => {
  console.log(line);
  appendFileSync(LOG, `${line}\n`);
};

const browser = await chromium.launch({ headless: true });
const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
await context.addInitScript(() => {
  try {
    localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1");
  } catch {
    /* private mode */
  }
});
const page = await context.newPage();
const consoleBuf: string[] = [];
page.on("console", (msg) => {
  const line = `${msg.type()}: ${msg.text().slice(0, 900)}`;
  consoleBuf.push(line);
  appendFileSync(LOG, `CONSOLE ${line}\n`);
});
page.on("pageerror", (err) => {
  const line = `pageerror: ${String(err).slice(0, 500)}`;
  consoleBuf.push(line);
  appendFileSync(LOG, `CONSOLE ${line}\n`);
});

const snapshot = async () =>
  page
    .evaluate(() => ({
      windows: document.querySelectorAll('[data-slot="window"]').length,
      canvases: document.querySelectorAll("canvas").length,
      dialogs: document.querySelectorAll('[role="dialog"]').length,
      tap: typeof (globalThis as { __b32Dispatch?: unknown }).__b32Dispatch,
    }))
    .catch(() => ({ windows: 0, canvases: 0, dialogs: 0, tap: "undefined" }));

log(`navigating http://127.0.0.1:${port}/?plugin=puzzle3d`);
await page.goto(`http://127.0.0.1:${port}/?plugin=puzzle3d`, { waitUntil: "domcontentloaded", timeout: 60000 }).catch((error) => log(`goto: ${String(error).slice(0, 200)}`));

let booted = false;
for (let i = 0; i < 60; i++) {
  await page.waitForTimeout(3000);
  const s = await snapshot();
  if (s.dialogs > 0 && i % 3 === 0) {
    const skip = page.locator('[role="dialog"] button', { hasText: /skip/i }).first();
    if (await skip.count()) await skip.click({ timeout: 2000 }).catch(() => {});
  }
  if (s.windows >= 2 && s.canvases >= 2 && s.tap === "function") {
    log(`booted after ${(i + 1) * 3}s ${JSON.stringify(s)}`);
    booted = true;
    break;
  }
  if (i % 5 === 4) log(`waiting… ${JSON.stringify(s)}`);
}
if (!booted) log(`NOT BOOTED ${JSON.stringify(await snapshot())}`);
const skipTour = page.locator("button", { hasText: /^\s*(x\s*)?skip\s*$/i }).first();
if (await skipTour.count()) {
  await skipTour.click({ timeout: 3000 }).catch(() => {});
  await page.waitForTimeout(1500);
}
await page.keyboard.press("Escape").catch(() => {});

const readInstances = async () =>
  page.evaluate(() => document.querySelector("#puzzle3d-main-perspective [data-instances-json]")?.getAttribute("data-instances-json") ?? "").catch(() => "");
const readRows = async () =>
  page
    .evaluate(() =>
      Array.from(document.querySelectorAll('[data-slot="tree-item"], [role="treeitem"]'))
        .map((r) => `${r.id || "?"}|${(r as HTMLElement).innerText.replace(/\n/g, " ").trim().slice(0, 60)}`)
        .slice(0, 24),
    )
    .catch(() => [] as string[]);

const openPanel = async (match: RegExp) => {
  const tabs = await page.evaluate(() => Array.from(document.querySelectorAll('[data-slot="panel-tab-button"]')).map((b) => ({ id: b.id, text: (b as HTMLElement).innerText.replace(/\n/g, " ").trim().slice(0, 40) })));
  const hit = tabs.find((t) => match.test(t.id) || match.test(t.text));
  log(`openPanel ${match} hit=${JSON.stringify(hit)} tabs=${JSON.stringify(tabs).slice(0, 600)}`);
  if (!hit) return false;
  await page.locator(`[data-slot="panel-tab-button"][id="${hit.id}"]`).first().click({ force: true, timeout: 4000 }).catch(() => {});
  await page.waitForTimeout(1500);
  return true;
};

await openPanel(/document|artifact|outliner|puzzle3d-play-document/i);
log(`rows=${JSON.stringify(await readRows()).slice(0, 1200)}`);

const selectRow = async (objectId: string) => {
  const row = page.locator(`[role="treeitem"][id*="${objectId}"], [data-slot="tree-item"][id*="${objectId}"]`).first();
  const count = await row.count();
  if (count) await row.click({ force: true, timeout: 4000, position: { x: 6, y: 6 } }).catch(() => {});
  else {
    const byText = page.locator('[role="treeitem"], [data-slot="tree-item"]').filter({ hasText: /Concrete Forest Left/i }).first();
    if (await byText.count()) await byText.click({ force: true, timeout: 4000 }).catch(() => {});
  }
  await page.waitForTimeout(2500);
  const selected = await page
    .evaluate(() => {
      const el = document.querySelector("#puzzle3d-main-perspective [data-selection-json]") as HTMLElement | null;
      return el?.getAttribute("data-selection-json")?.slice(0, 200) ?? null;
    })
    .catch(() => null);
  log(`selectRow ${objectId} rowLocator=${count} selection=${selected}`);
  return selected;
};

const settle = async (read: () => Promise<string>, changed: (value: string) => boolean, budgetMs = 30000) => {
  const start = Date.now();
  let value = await read();
  while (!changed(value) && Date.now() - start < budgetMs) {
    await page.waitForTimeout(500);
    value = await read();
  }
  return { value, waitedMs: Date.now() - start, ok: changed(value) };
};

const HOP_RE = /b32 dispatch|b32 refreshUi hop|b32 brushMesh|completion apply|applyHostEffects|refreshUi coalesced|performInvocation|command ingress lane|refreshUi dropped|history patch applied|disconnect|detach/i;
const hops = (mark: number) => consoleBuf.slice(mark).filter((row) => HOP_RE.test(row));

const runVerb = async (label: string, action: string, args: unknown, read: () => Promise<string>) => {
  const before = await read();
  const mark = consoleBuf.length;
  await page.evaluate(([a, v]) => (globalThis as { __b32Dispatch?: (action: string, args?: unknown) => unknown }).__b32Dispatch?.(a as string, v), [action, args] as [string, unknown]).catch((error) => log(`${label} dispatch threw ${String(error).slice(0, 200)}`));
  const result = await settle(read, (value) => value.length > 0 && value !== before);
  log(`\n===== ${label} (${action}) =====`);
  log(`beforeLen=${before.length} afterLen=${result.value.length} changed=${result.ok} waitedMs=${result.waitedMs}`);
  log(`before=${before.slice(0, 260)}`);
  log(`after =${result.value.slice(0, 260)}`);
  for (const row of hops(mark).slice(0, 80)) log(`  HOP ${row.slice(0, 700)}`);
  return result;
};

const meshCensus = async () =>
  page
    .evaluate(() =>
      Array.from(document.querySelectorAll<HTMLElement>("[data-interaction-json]")).map((element) => {
        try {
          const parsed = JSON.parse(element.getAttribute("data-interaction-json") || "{}") as { meshResidency?: number; meshReuploadUrls?: string[] };
          return `${element.getAttribute("data-surface-id") ?? "?"} residency=${parsed.meshResidency} reupload=${JSON.stringify(parsed.meshReuploadUrls)}`;
        } catch {
          return "parse-error";
        }
      }),
    )
    .catch(() => [] as string[]);
log(`meshCensus(boot)=${JSON.stringify(await meshCensus())}`);
await selectRow("seed-left-001");
await runVerb("BROKEN? translateSelection", "translateSelection", { ids: ["seed-left-001"], mode: "mesh", dx: 7, dy: 0, dz: 0 }, readInstances);
log(`meshCensus(after translate)=${JSON.stringify(await meshCensus())}`);
await runVerb("BROKEN? worldRelocate", "worldRelocate", { objectId: "seed-left-001", position: [11, 4, 0] }, readInstances);
await runVerb("CONTROL addTargetVolume", "addTargetVolume", { origin: [3, 3, 0] }, async () =>
  page.evaluate(() => document.querySelector("#puzzle3d-main-perspective [data-target-volumes-json]")?.getAttribute("data-target-volumes-json") ?? "").catch(() => ""),
);

log(`\n===== presence / notice tail =====`);
for (const row of consoleBuf.filter((r) => /presence|disconnect|detach|reconnect/i.test(r)).slice(-20)) log(`  ${row.slice(0, 500)}`);
log(`\nconsole lines=${consoleBuf.length} log=${LOG}`);
await browser.close();
