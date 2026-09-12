/** 🧊 Mesh-delivery probe: why an example whose evaluation reports `ok` publishes ZERO preview meshes.
 *
 * Picks each named example and records, once a second, the preview window's `data-meshes-json`
 * length, the status object's `phase`/`ratio`/`meshesLen`/`instancesLen`/`diagnostics`, plus every
 * `[DEBUG] tessellate wire` line (the request head and the answer envelope head of every
 * `tessellate` round trip) so the transfer can be reconstructed chunk by chunk.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=mesh-delivery/run-1 \
 *   SEMIO_PROBE_PICK="Sphere Cut With Torus" bun 🐍️mesh-delivery-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "mesh-delivery");
const bootWait = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 240);
const pickWait = Number(process.env.SEMIO_PROBE_PICK_WAIT ?? 180);
const picks = (process.env.SEMIO_PROBE_PICK ?? "Sphere Cut With Torus,Box Fillet Preview,Sphere Box Fuse,Rectangle Wire Preview,Box Shell Preview").split(",").map((s) => s.trim()).filter(Boolean);
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 4000)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));

/** 📸 One sample of every mounted host surface's published preview payload and status object. */
const snap = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const hosts = [...document.querySelectorAll("[data-status-json], [data-meshes-json]")].map((el) => {
    const meshesJson = el.getAttribute("data-meshes-json") ?? "";
    const instancesJson = el.getAttribute("data-instances-json") ?? "";
    const meshes = parse(meshesJson);
    const st = parse(el.getAttribute("data-status-json"));
    const nodeStatuses = st && typeof st === "object" && !st.phase ? Object.values(st).map((v) => v?.status ?? "?") : undefined;
    return {
      surfaceId: el.getAttribute("data-surface-id"),
      meshesJsonLen: meshesJson.length,
      instancesJsonLen: instancesJson.length,
      meshCount: Array.isArray(meshes) ? meshes.length : 0,
      meshShapes: Array.isArray(meshes) ? meshes.map((m) => ({ id: m?.id, pos: m?.data?.positions?.length ?? 0, idx: m?.data?.indices?.length ?? 0, edge: m?.data?.edgePositions?.length ?? 0 })) : [],
      phase: st?.phase, ratio: st?.progress?.ratio, unitsDone: st?.progress?.unitsDone, unitsTotal: st?.progress?.unitsTotal,
      facesDone: st?.progress?.facesDone, facesTotal: st?.progress?.facesTotal, inFlight: st?.progress?.inFlight,
      diagnostics: st?.diagnostics ?? null, debug: st?.debug ?? null, fault: st?.fault?.code ?? null,
      nodeStatuses,
    };
  });
  const combo = document.querySelector('[role="combobox"]');
  return { hosts, example: combo?.innerText?.replace(/\s+/g, " ").trim() ?? null };
});

/** ⏱️ Samples once a second for `seconds`, keeping every distinct sample so the mesh/status history
 * of the whole transfer is visible instead of only its last frame. */
const watch = async (label, seconds) => {
  const samples = [];
  let previous = "";
  let quiet = 0;
  const settleFor = Number(process.env.SEMIO_PROBE_QUIET ?? 30);
  for (let i = 0; i < seconds; i++) {
    await page.waitForTimeout(1000);
    const s = await snap();
    const key = JSON.stringify(s.hosts.map((h) => [h.surfaceId, h.meshesJsonLen, h.meshCount, h.phase, h.ratio, h.unitsDone, h.inFlight, h.diagnostics]));
    if (key !== previous) { samples.push({ t: Date.now() - t0, ...s }); previous = key; quiet = 0; } else quiet += 1;
    const preview = s.hosts.find((h) => h.surfaceId && h.surfaceId.endsWith("-preview"));
    // ⚖️ A payload that merely stopped moving is NOT delivery: a vector/point marker is a mesh too,
    // so the settle is "nothing changed for `settleFor` seconds with no round trip in flight".
    if (preview && quiet >= settleFor && (preview.inFlight ?? 0) === 0) { samples.push({ t: Date.now() - t0, settled: true, ...s }); break; }
  }
  const last = samples[samples.length - 1] ?? null;
  console.log(`[DEBUG] ${label}: ${JSON.stringify(last?.hosts?.map((h) => [h.surfaceId, h.meshCount, h.meshesJsonLen, h.phase, h.ratio, h.diagnostics]))}`);
  results.push({ label, samples });
  await page.screenshot({ path: join(outDir, `${results.length}-${label.replace(/[^a-z0-9]+/gi, "-")}.png`) });
  return last;
};

/** 🧵 Every tessellate round trip the run made, as (request, answer-head) pairs. */
const tessellateWire = () => lines.filter((l) => l.includes("[DEBUG] tessellate wire") || l.includes("capability: tessellate"));

const results = [];
await page.goto(url, { waitUntil: "domcontentloaded" });
await watch("boot", bootWait);
for (const text of picks) {
  const mark = lines.length;
  const combo = page.locator('[role="combobox"]').first();
  await combo.click({ timeout: 8000 }); await page.waitForTimeout(500);
  await page.locator('[role="option"]').filter({ hasText: text }).first().click({ timeout: 8000 });
  await watch(`pick:${text}`, pickWait);
  const slug = text.replace(/[^a-z0-9]+/gi, "-");
  writeFileSync(join(outDir, `console-${slug}.txt`), lines.slice(mark).join("\n"));
  writeFileSync(join(outDir, `wire-${slug}.txt`), lines.slice(mark).filter((l) => l.includes("tessellate")).join("\n"));
}
writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "tessellate-wire.txt"), tessellateWire().join("\n"));
console.log("[DEBUG] DONE steps", results.length);
await browser.close();
