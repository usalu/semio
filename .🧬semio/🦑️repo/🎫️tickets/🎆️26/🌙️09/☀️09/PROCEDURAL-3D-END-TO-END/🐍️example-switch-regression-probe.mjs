/** 🔁 Minimal example-switch endurance probe: boot once, then walk the bundled examples ROUND after
 * ROUND in the picker's own order, and record for every switch whether the flow window republished the
 * graph the picker names and whether every preview settled.
 *
 * 🪪️ The journey probe walks each example ONCE, so a session-scoped exhaustion that only bites from the
 * Nth switch on reads there as "those five examples are broken" instead of "the session dies at switch
 * N". This probe varies only the COUNT, so the first divergence is an index, not a geometry.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_ROUNDS=3 bun 🐍️example-switch-regression-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "example-switch");
const rounds = Number(process.env.SEMIO_PROBE_ROUNDS ?? 3);
const budget = Number(process.env.SEMIO_PROBE_BUDGET ?? 25);
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
/** 🩺️ Arms the guest's runtime-diagnostics traces BEFORE the first script runs, so every more-work turn
 * prints `PatchTracker::debug_state()` — and with it `registry=resident=<slots>s/<items>i/<bytes>B of
 * <aggregate>B`, the only live read of the process-wide resident ledger a browser session can get. */
if (process.env.SEMIO_PROBE_DIAGNOSTICS !== "0") await page.addInitScript(() => { try { globalThis.localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 2000)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));

/** 📷️ The published surface contract plus the flow window's published GRAPH, which is what a switch moves. */
const snap = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const hosts = [...document.querySelectorAll("[data-status-json], [data-meshes-json]")].map((el) => {
    const st = parse(el.getAttribute("data-status-json")) ?? {};
    const fx = parse(el.getAttribute("data-fixture-json"));
    let meshes = 0; try { const v = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]"); meshes = Array.isArray(v) ? v.length : 0; } catch {}
    const widgetIds = fx && Array.isArray(fx.widgets)
      ? fx.widgets.map((w) => { const inner = w && typeof w === "object" ? Object.values(w)[0] : null; return (w && w.id) ?? (inner && inner.id) ?? null; }).filter(Boolean)
      : null;
    return { surfaceId: el.getAttribute("data-surface-id"), meshes, phase: st.phase ?? null, ratio: st.progress?.ratio ?? null, computing: st.computing ?? null, fault: st.fault?.code ?? null, widgetIds };
  });
  const combo = document.querySelector('[role="combobox"]');
  return { hosts, example: combo?.innerText?.replace(/\s+/g, " ").trim() ?? null, faults: [...document.querySelectorAll("[data-fault-code]")].map((e) => e.getAttribute("data-fault-code")).slice(0, 10) };
});

/** 📈️ Settled means what the published `World3dComputeStatusV1` says and nothing else. */
const settled = (h) => Boolean(h) && h.phase === "idle" && h.ratio === 1 && h.computing !== true && !h.fault;
/** ⚖️ A switch converged when every preview settled AND the flow window publishes the picked example's graph. */
const converged = (s, widgets) => {
  const main = s.hosts.find((h) => h.surfaceId === "window:procedural-main");
  const previews = s.hosts.filter((h) => h.surfaceId && h.surfaceId.endsWith("-preview"));
  if (previews.length === 0 || !previews.every(settled)) return false;
  if (!main) return true;
  return JSON.stringify([...(main.widgetIds ?? [])].sort()) === JSON.stringify([...widgets].sort());
};

const results = [];
let consoleCursor = 0;
const since = () => { const slice = lines.slice(consoleCursor); consoleCursor = lines.length; return slice; };
/** 🚨️ Only the lines that name a refusal, a fault or a trap — the rest is per-turn noise. */
const notable = (slice) => slice.filter((l) => /ResidentCredit|registry-|unadmitted|SemioFaultError|pageerror|exhausted|refused|Capacity|InvalidLimits|unreachable|panicked/i.test(l)).map((l) => l.slice(0, 400));
/** 📊️ The last resident-ledger census this step saw, as `[slots, items, bytes, aggregate]`. */
const census = (slice) => {
  let found = null;
  for (const line of slice) { const m = /resident=(\d+)s\/(\d+)i\/(\d+)B of (\d+)B/.exec(line); if (m) found = [Number(m[1]), Number(m[2]), Number(m[3]), Number(m[4])]; }
  return found;
};

const step = async (label, widgets, seconds) => {
  let last = null, stable = 0; const start = Date.now();
  for (let i = 0; i < seconds; i++) {
    await page.waitForTimeout(1000); last = await snap();
    if (converged(last, widgets)) { stable += 1; if (stable >= 2) break; } else stable = 0;
  }
  const main = last.hosts.find((h) => h.surfaceId === "window:procedural-main");
  const slice = since();
  const row = {
    label, index: results.length, t: Date.now() - t0, seconds: (Date.now() - start) / 1000,
    converged: converged(last, widgets), example: last.example, expect: widgets,
    mainWidgets: main?.widgetIds ?? null,
    previews: last.hosts.filter((h) => h.surfaceId?.endsWith("-preview")).map((h) => [h.surfaceId, h.meshes, h.phase, h.ratio, h.fault]),
    census: census(slice), notable: notable(slice), faults: last.faults,
  };
  results.push(row);
  console.log(`[DEBUG] ${String(row.index).padStart(2)} ${label}: converged=${row.converged} in ${row.seconds.toFixed(0)}s census=${JSON.stringify(row.census)} main=${JSON.stringify(row.mainWidgets)} previews=${JSON.stringify(row.previews)}${row.notable.length ? ` notable=${JSON.stringify(row.notable)}` : ""}`);
  return last;
};

const EXPECTED = {
  "Hexagonal Mushroom Column": ["height", "radius", "sides", "profile", "extrusion-axis", "extrude", "column-preview"],
  "Rectangle Extrude Volume": ["width", "height", "distance", "rect", "vector", "extrude", "volume"],
  "Sphere Cut With Torus": ["slider_2", "brep_prim3d_sphere_3", "brep_prim3d_torus_4", "brep_bool_cut_5", "brep_measure_volume_2", "preview_3"],
  "Box Fillet Preview": ["size", "radius", "box", "fillet", "preview"],
  "Sphere Box Fuse": ["radius", "size", "sphere", "box", "fuse", "preview"],
  "Face Sweep Extrude": ["width", "height", "distance", "rect", "face", "vector", "extrude"],
  "Rectangle Wire Preview": ["width", "height", "rect"],
  "Box Shell Preview": ["size", "thickness", "box", "shell"],
  "No example": [],
};

await page.goto(url, { waitUntil: "domcontentloaded" });
await step("boot", EXPECTED["Hexagonal Mushroom Column"], budget + 25);

const combo = page.locator('[role="combobox"]').first();
await combo.click({ timeout: 5000 }); await page.waitForTimeout(400);
const options = (await page.locator('[role="option"]').allInnerTexts()).map((t) => t.replace(/\s+/g, " ").trim()).filter(Boolean);
await page.keyboard.press("Escape"); await page.waitForTimeout(200);
console.log("[DEBUG] options", JSON.stringify(options));

let firstDivergence = null;
for (let round = 1; round <= rounds; round += 1) {
  for (const text of options) {
    await combo.click({ timeout: 5000 }); await page.waitForTimeout(400);
    await page.locator('[role="option"]').filter({ hasText: text }).first().click({ timeout: 5000 });
    const row = await step(`r${round}:${text}`, EXPECTED[text] ?? [], budget);
    void row;
    const last = results[results.length - 1];
    if (!last.converged && firstDivergence === null) {
      firstDivergence = last.index;
      await page.screenshot({ path: join(outDir, `divergence-${last.index}.png`) });
      console.log(`[DEBUG] FIRST DIVERGENCE at switch #${last.index} (${last.label})`);
    }
  }
}

const switches = results.slice(1);
console.log(`[DEBUG] switches=${switches.length} converged=${switches.filter((r) => r.converged).length} firstDivergence=${firstDivergence}`);
writeFileSync(join(outDir, "results.json"), JSON.stringify({ url, rounds, budget, firstDivergence, results }, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
await browser.close();
