/** 🚑️ Host-side ingress + refresh latency probe for the procedural 3d REACT door.
 *
 * Reproduces and counts the three host faults this lane owns, on ONE boot:
 *  - `command ingress did not complete within N continuations`
 *  - the hot-swap `no channel` fault raised on an example switch
 *  - the owed `refreshUi {kind:"full"}` pass the host fires after every pick
 *
 * and measures what the gate asks for: worker crossings per `flowEvalTick` hop, and
 * slider-drag → first preview update, host-side.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6021/?plugin=generation3d SEMIO_PROBE_OUT=host-refresh/before bun 🐍️host-ingress-latency-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6021/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "host-refresh/run");
const example = process.env.SEMIO_PROBE_EXAMPLE ?? "hexagonal-mushroom-column";
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1200)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1200)}`));
await page.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });

const hosts = () => page.evaluate(() => [...document.querySelectorAll("[data-status-json]")].map((el) => {
  let meshes = 0;
  try { meshes = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]").length; } catch {}
  let status = null;
  try { status = JSON.parse(el.getAttribute("data-status-json")); } catch {}
  return { id: el.getAttribute("data-surface-id"), meshes, meshesHash: (el.getAttribute("data-meshes-json") ?? "").length, phase: status?.phase, ratio: status?.progress?.ratio, computing: status?.computing ?? null };
}));
const preview = async () => (await hosts()).find((entry) => entry.id === "window:procedural-preview") ?? null;
const converged = async () => {
  const current = await preview();
  return Boolean(current) && current.phase === "idle" && current.ratio === 1 && current.meshes > 0;
};
const waitConverged = async (seconds) => {
  for (let step = 0; step < seconds; step += 1) {
    if (await converged()) return true;
    await page.waitForTimeout(1000);
  }
  return converged();
};

/** 📏️ `worker.turn` spans are the real worker crossings; `invoke`+`flowEvalTick` are the hops. */
const spans = () => page.evaluate(() => performance.getEntriesByType("measure").filter((entry) => entry.name.startsWith("semio.hop.")).map((entry) => ({ stage: entry.name.slice("semio.hop.".length), startMs: entry.startTime, durationMs: entry.duration, detail: entry.detail ?? null })));
const clearSpans = () => page.evaluate(() => { for (const entry of performance.getEntriesByType("measure")) if (entry.name.startsWith("semio.hop.")) performance.clearMeasures(entry.name); });
const crossingsPerHop = (collected) => {
  const hops = collected.filter((span) => span.stage === "invoke" && span.detail?.actionId === "flowEvalTick").length;
  const crossings = collected.filter((span) => span.stage === "worker.turn").length;
  const refreshScopes = {};
  for (const span of collected.filter((entry) => entry.stage === "refresh")) refreshScopes[String(span.detail?.scope ?? "?")] = (refreshScopes[String(span.detail?.scope ?? "?")] ?? 0) + 1;
  /** 🧾️ What each settle that REPORTS did: stop reason, crossings spent, whether it still owed a surface. */
  const settleStops = {};
  let settleContinuations = 0;
  let settleSamples = 0;
  for (const span of collected.filter((entry) => entry.stage === "refresh.turn")) {
    const stop = String(span.detail?.stop ?? "(none)");
    settleStops[stop] = (settleStops[stop] ?? 0) + 1;
    settleContinuations += Number(span.detail?.continuations ?? 0);
    settleSamples += 1;
  }
  /** 🧵️ The crossings that carried nothing — the cost this lane and the drive lane both aim at. */
  const crossingKinds = {};
  for (const span of collected.filter((entry) => entry.stage === "worker.turn")) {
    const key = String(span.detail?.eventKinds ?? "").length ? String(span.detail.eventKinds) : "(none)";
    crossingKinds[key] = (crossingKinds[key] ?? 0) + 1;
  }
  return { hops, crossings, perHop: hops ? crossings / hops : 0, refreshScopes, settleStops, settleContinuations, settleSamples, crossingKinds };
};

await page.goto(`${url}&example=${example}`, { waitUntil: "domcontentloaded" });
const bootOk = await waitConverged(180);
const bootState = await hosts();
lines.push(`${Date.now() - t0} probe boot converged=${bootOk} ${JSON.stringify(bootState)}`);

//#region 🖱️Pick
const previewBox = await page.evaluate(() => {
  const element = document.querySelector('[data-surface-id="window:procedural-preview"]');
  if (!element) return null;
  const box = element.getBoundingClientRect();
  return { x: Math.round(box.x), y: Math.round(box.y), w: Math.round(box.width), h: Math.round(box.height) };
});
lines.push(`${Date.now() - t0} probe previewBox ${JSON.stringify(previewBox)}`);
await clearSpans();
const pickStart = Date.now();
if (previewBox) {
  await page.mouse.move(previewBox.x + previewBox.w / 2, previewBox.y + previewBox.h / 2);
  await page.waitForTimeout(400);
  await page.mouse.click(previewBox.x + previewBox.w / 2, previewBox.y + previewBox.h / 2);
}
await page.waitForTimeout(4000);
const pickSpans = await spans();
const pickCost = crossingsPerHop(pickSpans);
lines.push(`${Date.now() - t0} probe pick ms=${Date.now() - pickStart} ${JSON.stringify(pickCost)}`);
//#endregion 🖱️Pick

//#region 🎚️SliderDrag
const sliderList = () => page.evaluate(() => [...document.querySelectorAll('[role="slider"]')].map((el, index) => {
  const box = el.getBoundingClientRect();
  return { index, label: el.getAttribute("aria-label"), now: el.getAttribute("aria-valuenow"), min: el.getAttribute("aria-valuemin"), max: el.getAttribute("aria-valuemax"), rect: { x: Math.round(box.x), y: Math.round(box.y), w: Math.round(box.width), h: Math.round(box.height) } };
}).filter((entry) => entry.rect.w > 8 && entry.rect.h > 2));
const sliders = await sliderList();
lines.push(`${Date.now() - t0} probe sliders ${JSON.stringify(sliders)}`);
await clearSpans();
const dragReport = { found: sliders.length, valueBefore: sliders[0]?.now ?? null, valueAfter: null, firstUpdateMs: null, releaseToIdleMs: null };
const target = sliders[0] ?? null;
if (target) {
  const before = await preview();
  // 🎚️ The node-graph inline sliders are 29 px knobs; a pointer drag across one is a few pixels and
  // lands as a hover. The `role="slider"` keyboard contract is the reliable value change, and it is
  // the same command the pointer drag dispatches — which is what this lane measures the host cost of.
  await page.evaluate((index) => { const el = [...document.querySelectorAll('[role="slider"]')][index]; el?.focus?.(); }, target.index);
  const dragStart = Date.now();
  for (let step = 0; step < 6; step += 1) {
    await page.keyboard.press("ArrowRight");
    await page.waitForTimeout(30);
  }
  dragReport.valueAfter = await page.evaluate((index) => [...document.querySelectorAll('[role="slider"]')][index]?.getAttribute("aria-valuenow") ?? null, target.index);
  for (let poll = 0; poll < 200; poll += 1) {
    const now = await preview();
    if (now && before && (now.meshesHash !== before.meshesHash || now.computing === true || now.phase !== before.phase)) { dragReport.firstUpdateMs = Date.now() - dragStart; break; }
    await page.waitForTimeout(25);
  }
  const releaseAt = Date.now();
  for (let poll = 0; poll < 400; poll += 1) {
    if (await converged()) { dragReport.releaseToIdleMs = Date.now() - releaseAt; break; }
    await page.waitForTimeout(100);
  }
}
const dragSpans = await spans();
const dragCost = crossingsPerHop(dragSpans);
lines.push(`${Date.now() - t0} probe drag ${JSON.stringify({ ...dragReport, ...dragCost })}`);
//#endregion 🎚️SliderDrag

//#region 🔀️ExampleSwitch
await clearSpans();
const switched = await page.evaluate(async () => {
  const combo = document.querySelector('[role="combobox"]');
  if (!combo) return { ok: false, reason: "no-combobox" };
  combo.dispatchEvent(new PointerEvent("pointerdown", { bubbles: true }));
  combo.click();
  await new Promise((resolve) => setTimeout(resolve, 600));
  const options = [...document.querySelectorAll('[role="option"]')].map((el) => el.textContent?.trim() ?? "");
  return { ok: true, options };
});
lines.push(`${Date.now() - t0} probe switch-open ${JSON.stringify(switched).slice(0, 700)}`);
const picked = await page.evaluate(async () => {
  const options = [...document.querySelectorAll('[role="option"]')];
  const next = options.find((el) => {
    const label = el.textContent?.trim() ?? "";
    return label.length > 0 && !/^no example$/i.test(label) && el.getAttribute("aria-selected") !== "true";
  });
  if (!next) return null;
  const label = next.textContent?.trim() ?? "";
  next.click();
  return label;
});
lines.push(`${Date.now() - t0} probe switch-picked ${JSON.stringify(picked)}`);
const switchedConverged = await waitConverged(180);
const switchSpans = await spans();
lines.push(`${Date.now() - t0} probe switch ${JSON.stringify({ converged: switchedConverged, ...crossingsPerHop(switchSpans) })}`);
//#endregion 🔀️ExampleSwitch

//#region 🖱️HoverStorm
await clearSpans();
if (previewBox) {
  for (let step = 0; step < 12; step += 1) {
    await page.mouse.move(previewBox.x + 60 + step * 17, previewBox.y + 80 + (step % 5) * 23);
    await page.waitForTimeout(120);
  }
}
await page.waitForTimeout(3000);
lines.push(`${Date.now() - t0} probe hover ${JSON.stringify(crossingsPerHop(await spans()))}`);
//#endregion 🖱️HoverStorm

await page.screenshot({ path: join(outDir, "final.png"), type: "png" });
const text = lines.join("\n");
writeFileSync(join(outDir, "console.txt"), text);
const count = (needle) => text.split("\n").filter((line) => line.includes(needle)).length;
const summary = {
  bootConverged: bootOk,
  continuationCeiling: count("continuations (observed statuses"),
  noChannel: count("no channel"),
  pageErrors: count(" pageerror "),
  owedFullRefresh: text.split("\n").filter((line) => line.includes("refreshUi lane") && line.includes('"full"')).length,
  refreshLaneOwed: text.split("\n").filter((line) => line.includes('"decision":"owed"')).length,
  drag: dragReport,
  pick: pickCost,
};
writeFileSync(join(outDir, "summary.json"), JSON.stringify(summary, null, 2));
console.log("DONE", JSON.stringify(summary));
await browser.close();
