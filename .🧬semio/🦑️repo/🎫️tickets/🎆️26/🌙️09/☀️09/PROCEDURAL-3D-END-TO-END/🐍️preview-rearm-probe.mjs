/** 🔁️ Does an inspector edit re-arm the edit preview, and how many `previewEval` runs does it cost?
 *
 * The gap probe's `inspection-preview-rearm` step reads "the preview re-evaluated" as a change in the
 * edit preview's published payload DIGEST. This is the same reading taken on a fresh boot with nothing
 * else in the way, plus the two numbers that name the owner: how many `toolRunStart` invocations the
 * session logs (one landed mutation owes exactly ONE re-armed tick, so a storm is a broken latch), and
 * what the preview's own status phase says while the edit is outstanding.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=preview-rearm/run1 bun 🐍️preview-rearm-probe.mjs
 * @see 🐍️react-gap-probe.mjs step `inspection-preview-rearm`, 🐍️preview-rearm-recon.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { createHash } from "node:crypto";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "preview-rearm/run");
mkdirSync(outDir, { recursive: true });
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 60);

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 900)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 900)}`));

const report = { url, steps: [] };
const flush = () => { writeFileSync(join(outDir, "probe.json"), JSON.stringify(report, null, 2)); writeFileSync(join(outDir, "console.txt"), lines.join("\n")); };
const starts = () => lines.filter((line) => line.includes('"actionId":"toolRunStart"')).length;
const ticks = () => lines.filter((line) => line.includes('"actionId":"flowEvalTick"')).length;

const read = async () => {
  const raw = await page.evaluate(() => {
    const parse = (text) => { try { return JSON.parse(text ?? ""); } catch { return null; } };
    const previews = [...document.querySelectorAll("[data-meshes-json]")].map((el) => {
      const status = parse(el.getAttribute("data-status-json"));
      const payload = el.getAttribute("data-meshes-json") ?? "";
      const value = parse(payload);
      return { surfaceId: el.getAttribute("data-surface-id"), payload, bytes: payload.length, meshes: Array.isArray(value) ? value.length : 0, phase: status?.phase ?? null, cancellable: status?.cancellable ?? null, fault: status?.fault?.code ?? null };
    });
    const main = document.querySelector('[data-surface-id="window:procedural-main"]');
    const fixture = parse(main?.getAttribute("data-fixture-json"));
    return { previews, widgets: (fixture?.widgets ?? []).map((w) => ({ id: w.id, value: w.value })) };
  });
  raw.previews = raw.previews.map(({ payload, ...rest }) => ({ ...rest, digest: `${payload.length}:${createHash("sha256").update(payload).digest("hex").slice(0, 8)}` }));
  return raw;
};
const editPreviews = (state) => state.previews.filter((preview) => !/generate/.test(preview.surfaceId ?? ""));
const editMeshes = (state) => editPreviews(state).reduce((sum, preview) => sum + preview.meshes, 0);
const editDigest = (state) => editPreviews(state).map((preview) => preview.digest).join("|");
const until = async (pred, seconds) => { let v = await read(); for (let i = 0; i < seconds && !pred(v); i += 1) { await page.waitForTimeout(1000); v = await read(); } return v; };
const note = async (step, extra = {}) => { const state = await read(); report.steps.push({ step, ...state, ...extra, starts: starts(), ticks: ticks(), t: Date.now() - t0 }); console.log(`[DEBUG] ${step} digest=${editDigest(state)} meshes=${editMeshes(state)} starts=${starts()} ticks=${ticks()} ${JSON.stringify(extra)}`); flush(); };

await page.goto(url, { waitUntil: "domcontentloaded" });
await until((state) => editMeshes(state) > 0, 180);
await note("boot");
const armed = await read();
const startsAtBoot = starts();
const ticksAtBoot = ticks();

const SURFACE = "window:procedural-main";
const click = (id) => page.locator(`[id="${id}"]`).first().click({ timeout: 8000 }).catch((e) => lines.push(`${id} ${String(e).slice(0, 140)}`));
const readInspector = () => page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"] [id*="procedural-play-inspector"]')].map((el) => ({ id: el.id, value: el.value ?? null, text: (el.textContent ?? "").trim().slice(0, 60) })));
/** 🧹️ Drops every standing selection so the inspector really answers about ONE widget. */
const clearSelection = async () => {
  const host = await page.evaluate((id) => { const r = document.querySelector(`[data-surface-id="${id}"]`)?.getBoundingClientRect(); return r ? { x: r.x, y: r.y, width: r.width, height: r.height } : null; }, SURFACE);
  if (host) await page.mouse.click(host.x + host.width * 0.5, host.y + host.height - 40);
  await page.keyboard.press("Escape");
  await page.waitForTimeout(1200);
};
const widgetId = process.env.SEMIO_PROBE_WIDGET ?? "height";
await click("framework.panel.artifact");
await page.waitForTimeout(1800);
let inspectorRows = await readInspector();
for (let attempt = 0; attempt < 3 && inspectorRows.length === 0; attempt += 1) {
  await click("framework.panel.inspection");
  await page.waitForTimeout(2500);
  inspectorRows = await readInspector();
}
let selected = null;
for (let attempt = 0; attempt < 3 && selected === null; attempt += 1) {
  await clearSelection();
  await page.locator(`[data-slot="panel"] [id="panel:procedural-play-document/${widgetId}"]`).first().click({ timeout: 8000 }).catch((e) => lines.push(`${widgetId} row ${String(e).slice(0, 140)}`));
  await page.waitForTimeout(2500);
  inspectorRows = await readInspector();
  selected = inspectorRows.some((row) => row.id.endsWith("procedural-play-inspector.id") && row.text.includes(widgetId)) ? widgetId : null;
}
await note("inspector-open", { selected, inspectorRows: inspectorRows.map((row) => row.id) });

const field = page.locator('[id="procedural-play-inspector.value.input"], [data-slot="panel"] [id$="procedural-play-inspector.value.input"]').first();
const present = (await field.count()) > 0;
const before = await read();
let typed = null;
if (present) {
  const old = Number(await field.inputValue().catch(() => "0"));
  typed = Number.isFinite(old) ? Number((old + 1).toFixed(3)) : 1;
  await field.fill(String(typed), { timeout: 8000 }).catch(() => {});
  await page.keyboard.press("Enter");
  await field.blur().catch(() => {});
}
// 🖼️ "Re-evaluated" is a NEW payload that still carries geometry: the first thing a re-armed chain
// publishes is the emptied preview it is about to refill, and stopping there would call a preview that
// merely went blank a success.
const after = await until((state) => editDigest(state) !== editDigest(before) && editMeshes(state) > 0, settleSeconds);
const widgetAfter = after.widgets.find((widget) => widget.id === widgetId) ?? null;
await note("inspector-patched", {
  present,
  typed,
  widgetAfter,
  valueMoved: widgetAfter != null && typed != null && Number(widgetAfter.value) === typed,
  digestBefore: editDigest(before),
  digestAfter: editDigest(after),
  previewMoved: editDigest(after) !== editDigest(before) && editMeshes(after) > 0,
  previewArmed: editMeshes(armed) > 0,
  startsForThisEdit: starts() - startsAtBoot,
  ticksForThisEdit: ticks() - ticksAtBoot,
});

const verdict = report.steps.at(-1);
report.verdict = { rearmed: verdict.previewMoved && verdict.previewArmed && verdict.valueMoved, startsForThisEdit: verdict.startsForThisEdit, ticksForThisEdit: verdict.ticksForThisEdit, startsTotal: starts() };
flush();
console.log(`[DEBUG] PREVIEW REARM PROBE DONE ${JSON.stringify(report.verdict)}`);
await browser.close();
