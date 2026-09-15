/** 🔬️ What ONE slider step actually costs, stage by stage — the recon behind lane
 * `slider-latency-incremental-eval`.
 *
 * 🧾 Unlike `🐍️slider-live-preview-probe.mjs`, which drags at 60 Hz and grades the GESTURE, this
 * one moves the knob by a SINGLE step from rest and then records everything until the chain settles,
 * so the numbers are the chain's own and not a burst's supersession behaviour:
 *   census    — `progress.nodesDone` / `progress.nodesTotal` off the preview's own status object.
 *               `nodesTotal` is how many nodes the chain declared it would walk, which is the direct
 *               reading of dirty-set precision: a `height` edit on the hex column must declare the
 *               extrude branch, never all seven nodes.
 *   meshes    — per-entry `id` + a content digest of the entry's own bytes, sampled every 50 ms, so a
 *               re-delivered but UNCHANGED mesh is visible as a digest that never moved while the
 *               payload as a whole was republished.
 *   ledger    — every `performInvocation` / `command ingress crossed` console line on the page's own
 *               clock (the hook is installed before the first app module).
 *   spans     — the renderer's `semio.hop.*` User Timing entries.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6025/?plugin=generation3d \
 *          SEMIO_PROBE_OUT=slider-latency/recon1 bun 🐍️incremental-eval-recon.mjs
 * @see 🐍️slider-live-preview-probe.mjs, 🐍️react-hop-cost-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6025/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "slider-latency/recon");
const bootSeconds = Number(process.env.SEMIO_PROBE_BOOT ?? 180);
const watchSeconds = Number(process.env.SEMIO_PROBE_WATCH ?? 25);
const steps = Number(process.env.SEMIO_PROBE_STEPS ?? 1);
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 900)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 900)}`));

await page.addInitScript(() => {
  const ledger = [];
  window.__reconLedger = ledger;
  for (const channel of ["log", "warn", "info", "debug", "error"]) {
    const original = console[channel].bind(console);
    console[channel] = (...args) => {
      try {
        const raw = args.map((value) => (typeof value === "string" ? value : "")).join(" ");
        if (raw.includes("performInvocation") || raw.includes("command ingress") || raw.includes("[DEBUG] chain")) {
          ledger.push({ t: performance.now(), raw: raw.slice(0, 400) });
        }
      } catch {}
      original(...args);
    };
  }
  const fnv = (text) => {
    let hash = 0x811c9dc5;
    for (let index = 0; index < text.length; index += 1) {
      hash ^= text.charCodeAt(index);
      hash = Math.imul(hash, 0x01000193) >>> 0;
    }
    return hash.toString(16);
  };
  window.__reconSample = (surfaceId, valueSelector) => {
    const surface = document.querySelector(`[data-surface-id="${surfaceId}"]`);
    const payload = surface?.getAttribute("data-meshes-json") ?? "";
    let status = null;
    let meshes = null;
    try {
      status = JSON.parse(surface?.getAttribute("data-status-json") ?? "null");
    } catch {}
    try {
      meshes = JSON.parse(payload);
    } catch {}
    const entries = Array.isArray(meshes)
      ? meshes.map((mesh) => {
          const text = JSON.stringify(mesh);
          const data = mesh?.data ?? {};
          return { id: mesh?.id ?? mesh?.instanceId ?? null, role: mesh?.role ?? null, bytes: text.length, digest: fnv(text), tris: Array.isArray(data.indices) ? data.indices.length / 3 : 0, pts: Array.isArray(data.positions) ? data.positions.length / 3 : 0 };
        })
      : [];
    const control = valueSelector ? document.querySelector(valueSelector) : null;
    return {
      t: performance.now(),
      value: control ? (control.getAttribute("aria-valuenow") ?? control.value ?? null) : null,
      bytes: payload.length,
      digest: `${payload.length}:${fnv(payload)}`,
      entries,
      phase: status?.phase ?? null,
      progress: status?.progress ?? null,
    };
  };
  window.__reconStart = (surfaceId, valueSelector) => {
    window.__reconSamples = [];
    window.__reconTimer = setInterval(() => {
      try {
        window.__reconSamples.push(window.__reconSample(surfaceId, valueSelector));
      } catch {}
    }, 50);
  };
  window.__reconStop = () => {
    clearInterval(window.__reconTimer);
    return window.__reconSamples ?? [];
  };
});

const surfaces = () =>
  page.evaluate(() =>
    [...document.querySelectorAll("[data-status-json]")].map((el) => {
      let status = null;
      try {
        status = JSON.parse(el.getAttribute("data-status-json"));
      } catch {}
      return { id: el.getAttribute("data-surface-id"), bytes: (el.getAttribute("data-meshes-json") ?? "").length, phase: status?.phase };
    }),
  );

await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120_000 });
let seen = [];
for (let index = 0; index < bootSeconds; index += 1) {
  seen = await surfaces();
  if (seen.some((entry) => entry.bytes > 200)) break;
  await page.waitForTimeout(1000);
}
const surfaceId = seen.find((entry) => entry.bytes > 200)?.id ?? seen[0]?.id ?? null;
lines.push(`${Date.now() - t0} recon surface ${surfaceId} ${JSON.stringify(seen)}`);

const selector = '[role="slider"][aria-label="Column Height"]';
await page.waitForSelector(selector, { timeout: 60_000 });
await page.waitForTimeout(3000);
await page.evaluate(() => performance.clearMeasures());
await page.evaluate(() => (window.__reconLedger.length = 0));
await page.evaluate(([id, sel]) => window.__reconStart(id, sel), [surfaceId, selector]);
await page.waitForTimeout(500);

const handle = await page.$(selector);
const box = await handle.boundingBox();
const track = await (await handle.evaluateHandle((el) => el.closest('[data-slot="slider"]') ?? el.parentElement ?? el)).asElement().boundingBox();
const rail = track && track.width > box.width ? track : box;
const y = box.y + box.height / 2;
const from = box.x + box.width / 2;
const stepPx = Math.max(8, rail.width / 20);
await page.mouse.move(from, y);
await page.mouse.down();
const pressedAtMs = await page.evaluate(() => performance.now());
for (let index = 1; index <= steps; index += 1) await page.mouse.move(from + stepPx * index, y);
await page.mouse.up();
const releasedAtMs = await page.evaluate(() => performance.now());
lines.push(`${Date.now() - t0} recon gesture pressed=${pressedAtMs} released=${releasedAtMs} stepPx=${stepPx}`);

await page.waitForTimeout(watchSeconds * 1000);
const samples = await page.evaluate(() => window.__reconStop());
const ledger = await page.evaluate(() => window.__reconLedger);
const spans = await page.evaluate(() =>
  performance
    .getEntriesByType("measure")
    .filter((entry) => entry.name.startsWith("semio.hop."))
    .map((entry) => ({ stage: entry.name.slice("semio.hop.".length), startMs: entry.startTime, durationMs: entry.duration, detail: entry.detail ?? null })),
);

writeFileSync(join(outDir, "samples.json"), JSON.stringify({ surfaceId, pressedAtMs, releasedAtMs, samples }, null, 1));
writeFileSync(join(outDir, "ledger.json"), JSON.stringify(ledger, null, 1));
writeFileSync(join(outDir, "spans.json"), JSON.stringify(spans, null, 1));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));

const changes = [];
let previous = null;
for (const sample of samples) {
  const key = `${sample.value}|${sample.digest}|${sample.phase}|${sample.progress?.nodesDone}/${sample.progress?.nodesTotal}|${sample.progress?.inFlight}`;
  if (key !== previous) {
    changes.push({ t: Math.round(sample.t - pressedAtMs), value: sample.value, digest: sample.digest, meshes: sample.entries.length, phase: sample.phase, nodes: `${sample.progress?.nodesDone ?? "-"}/${sample.progress?.nodesTotal ?? "-"}`, inFlight: sample.progress?.inFlight ?? null, ids: sample.entries.map((entry) => `${entry.id ?? "?"}:${entry.digest}`) });
    previous = key;
  }
}
writeFileSync(join(outDir, "changes.json"), JSON.stringify(changes, null, 1));
for (const change of changes) console.log("[DEBUG] recon", JSON.stringify(change));
console.log(`RECON DONE surface=${surfaceId} samples=${samples.length} changes=${changes.length} out=${outDir}`);
await browser.close();
