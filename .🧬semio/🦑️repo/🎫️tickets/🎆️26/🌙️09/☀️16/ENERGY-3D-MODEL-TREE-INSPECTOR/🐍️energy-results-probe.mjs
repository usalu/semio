/** 🌡️ Energy results-mode probe (ticket 26/09/16/ENERGY-3D-MODEL-TREE-INSPECTOR, DoD 4).
 *
 * Boots the energy react playground, samples the 3d scene's COLOURS, runs the annual simulation, and asserts the
 * scene recoloured by a result field with a legend:
 *   1. samples `data-meshes-json` / `data-instances-json` on the World3d host and reduces every `data.colors` array +
 *      every per-instance `color` into a digest and a coarse 8-bucket histogram (the authoritative before/after signal);
 *   2. arms the simulation tool by OPENING `#framework.category.tool` — opening the category auto-arms its only tool,
 *      and clicking `tool.energySimulation` would be a re-press that DISARMS it (predecessor ticket 26/09/06 pitfall);
 *   3. presses mod+enter and waits for the simulation window / tool-run panel to read `Finalized`;
 *   4. re-samples the colours and asserts they changed; reads the legend caption — only nodes INSIDE the World3d host or
 *      carrying a legend id/class, with the simulation window's own run text ("Final: … kWh after 8760 timesteps")
 *      excluded, so a document-wide "kWh" match cannot fake a legend;
 *   5. unfolds the SIMULATION window's Actions pane (`framework.window.energySimulation.engagement.toggle` — never the
 *      3d window's: `set-result-field` is declared on `energy.simulation` only), clicks `action.set-result-field`,
 *      picks the field through the form's shadcn Select (a BUTTON with role=combobox, so `fill` cannot work), and
 *      SEMIO_PROBE_RESULT_FIELD and asserts the colours changed AGAIN;
 *   5b. samples a TIMELINE frame of the 3d host every ~2 s during the run and again at Finalized / settled / after the
 *      field switch: `data-status-json` plus every counter-ish scalar it carries, the meshes/instances lane digests and
 *      byte sizes, the vertex-colour histogram, the distinct per-instance colours, the number of TEXT NODES rendered
 *      inside the host and whether one of them carries `kWh`/`W/m²` (a caption/legend), the Tool runs panel's step
 *      lines, and the count + a sample of console lines naming `energy.model.3d` / `window_bodies` / a UI refresh or
 *      dirty scope. Written to `timeline.txt` and `report.json.timeline`, and printed as a compact table at the end.
 *   6. screenshots every step. A canvas pixel histogram is attempted as a secondary signal and reported as
 *      "unavailable" when the WebGL drawing buffer cannot be read back — it never fails the probe on its own.
 *
 * Never throws: an absent window / absent run / absent legend is a FAIL assertion with a plain-English message, exit 1.
 *
 * Usage:
 *   cd .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/ENERGY-3D-MODEL-TREE-INSPECTOR
 *   SEMIO_PROBE_OUT=energy-results-1 bun 🐍️energy-results-probe.mjs
 * Env: SEMIO_PROBE_URL, SEMIO_PROBE_OUT, SEMIO_PROBE_SECONDS (120), SEMIO_PROBE_RUN_SECONDS (240),
 *      SEMIO_PROBE_WINDOW_3D (energy.model.3d), SEMIO_PROBE_RESULT_FIELD ("Solar transmitted"), SEMIO_PROBE_RESULT_ACTION (set-result-field),
 *      SEMIO_PROBE_REQUIRE_FIELD_SWITCH=1 (turn the optional result-field switch into a hard assertion).
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6106/?plugin=energy";
const bootSeconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 120);
const runSeconds = Number(process.env.SEMIO_PROBE_RUN_SECONDS ?? 240);
const windowKind = process.env.SEMIO_PROBE_WINDOW_3D ?? "energy.model.3d";
const surfaceId = `window:${windowKind}`;
// the live options are "Conduction loss", "Conduction gain", "Solar transmitted", "Solar absorbed" — there is no "solarGain"
const resultField = process.env.SEMIO_PROBE_RESULT_FIELD ?? "Solar transmitted";
const resultAction = process.env.SEMIO_PROBE_RESULT_ACTION ?? "set-result-field";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "energy-results");
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const report = { probe: "energy-results", url, surfaceId, resultField, resultAction, startedAt: new Date().toISOString(), assertions: [], steps: {} };
const flush = () => { writeFileSync(join(outDir, "console.txt"), lines.join("\n")); writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2)); };
const note = (key, value) => { report.steps[key] = value; console.log(`[DEBUG] ${key} ${JSON.stringify(value).slice(0, 900)}`); flush(); };
const assert = (id, ok, detail) => { report.assertions.push({ id, ok: Boolean(ok), detail }); console.log(`${ok ? "PASS" : "FAIL"} ${id} — ${detail}`); flush(); };
const skip = (id, detail) => { (report.skipped ??= []).push({ id, detail }); console.log(`SKIP ${id} — ${detail}`); flush(); };

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 6000 : 900)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const shot = (name) => page.screenshot({ path: join(outDir, `${name}.png`), type: "png" }).catch(() => {});

/** 🎨️ The colour fingerprint of one World3d surface: a digest plus an 8-bucket histogram over every colour channel triple. */
const colourSample = (id) => page.evaluate((id) => {
  const el = document.querySelector(`[data-surface-id="${id}"]`) ?? [...document.querySelectorAll("[data-surface-id]")].find((e) => e.hasAttribute("data-meshes-json"));
  if (!el) return null;
  const digest = (s) => { let h = 7; for (let i = 0; i < s.length; i++) h = (h * 31 + s.charCodeAt(i)) >>> 0; return h; };
  let meshes = [], instances = [];
  try { meshes = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]") ?? []; } catch {}
  try { instances = JSON.parse(el.getAttribute("data-instances-json") ?? "[]") ?? []; } catch {}
  const histogram = new Array(8).fill(0);
  let channels = 0;
  for (const m of meshes) {
    const colors = m?.data?.colors;
    if (!Array.isArray(colors)) continue;
    channels += colors.length;
    for (let i = 0; i + 2 < colors.length; i += 3) {
      const lum = (Number(colors[i]) * 0.3 + Number(colors[i + 1]) * 0.59 + Number(colors[i + 2]) * 0.11);
      histogram[Math.max(0, Math.min(7, Math.floor(lum * 8)))] += 1;
    }
  }
  const instanceColors = instances.map((i) => i?.color).filter(Boolean);
  const uniqueInstanceColors = [...new Set(instanceColors)];
  return {
    surfaceId: el.getAttribute("data-surface-id"),
    meshCount: meshes.length, instanceCount: instances.length,
    vertexColourChannels: channels, histogram,
    uniqueInstanceColors: uniqueInstanceColors.slice(0, 24), uniqueInstanceColourCount: uniqueInstanceColors.length,
    meshDigest: digest(el.getAttribute("data-meshes-json") ?? ""), instanceDigest: digest(el.getAttribute("data-instances-json") ?? ""),
    hostText: (el.innerText ?? "").replace(/\s+/g, " ").slice(0, 600),
  };
}, id);
/** 🖼️ Secondary signal only: read back the WebGL canvas through a 2d context. Often unavailable (no preserveDrawingBuffer). */
const canvasHistogram = (id) => page.evaluate(async (id) => {
  try {
    const el = document.querySelector(`[data-surface-id="${id}"]`) ?? [...document.querySelectorAll("[data-surface-id]")].find((e) => e.hasAttribute("data-meshes-json"));
    const canvas = el?.querySelector("canvas");
    if (!canvas) return { available: false, why: "no canvas element" };
    const bitmap = await createImageBitmap(canvas);
    const off = new OffscreenCanvas(64, 64);
    const ctx = off.getContext("2d");
    ctx.drawImage(bitmap, 0, 0, 64, 64);
    const data = ctx.getImageData(0, 0, 64, 64).data;
    const hist = new Array(8).fill(0);
    let nonBlank = 0;
    for (let i = 0; i < data.length; i += 4) {
      const lum = (data[i] * 0.3 + data[i + 1] * 0.59 + data[i + 2] * 0.11) / 255;
      hist[Math.max(0, Math.min(7, Math.floor(lum * 8)))] += 1;
      if (data[i + 3] > 8 && lum > 0.02) nonBlank += 1;
    }
    return { available: true, hist, nonBlank };
  } catch (error) { return { available: false, why: String(error).slice(0, 160) }; }
}, id);
/** 🩺️ The run state, read from EVERY place the shell can say it, not from one regex over `body.innerText`:
 * the visible body, the whole document's `textContent` (which still contains a panel that is currently tab-hidden —
 * `innerText` drops it), every `aria-live`/`role=status` node (the simulation window's own live region), and every
 * `panel:framework.panel.toolRun/…` row. The wave-1 simulation window gained a "Surfaces coloured by: …" line and a
 * `set-result-field` row, which made the old `busy=… · <state>` capture come back empty and the probe report
 * transitions `["false:"]` for a run that had in fact finalized. */
const runStateRead = () => page.evaluate(() => {
  const norm = (s) => (s ?? "").replace(/\s+/g, " ").trim();
  const bodyVisible = norm(document.body.innerText);
  const bodyAll = norm(document.body.textContent);
  const live = [...document.querySelectorAll('[aria-live], [role="status"]')].map((e) => norm(e.textContent)).filter(Boolean).slice(0, 8);
  const toolRunRows = [...document.querySelectorAll('[id^="panel:"]')].filter((e) => /toolrun/i.test(e.id)).map((e) => norm(e.textContent)).filter(Boolean).slice(0, 12);
  const haystack = [bodyVisible, bodyAll, ...live, ...toolRunRows].join(" | ");
  const busy = /busy=(true|false)\s*·\s*([^·|]+?)(?:\s*·\s*|\s*\|\s*|\s+mod\+|\s+Run:|$)/.exec(haystack);
  return {
    finalized: /Finalized|Abgeschlossen/i.test(haystack),
    busy: busy?.[1] ?? null,
    state: busy?.[2]?.trim() ?? null,
    runText: (haystack.match(/Final:[^|·]{0,120}|Finalized[^|·]{0,120}|\b\d+ of \d+ timesteps/) ?? [null])[0],
    colouredBy: (haystack.match(/Surfaces coloured by:?[^|·]{0,80}|Fl[äa]chen eingef[äa]rbt[^|·]{0,80}/i) ?? [null])[0],
    live, toolRunRows,
  };
});
const shell = async () => {
  const base = await page.evaluate(() => ({
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    error: document.documentElement.getAttribute("data-semio-os-error"),
    surfaces: [...document.querySelectorAll("[data-surface-id]")].map((e) => e.getAttribute("data-surface-id")),
    engagements: [...document.querySelectorAll('[id$=".engagement"]')].map((e) => e.id),
    actionRows: [...document.querySelectorAll('[id^="action."]')].map((e) => e.id),
    body: document.body.innerText.replace(/\s+/g, " ").slice(0, 1200),
  }));
  return { ...base, ...(await runStateRead()) };
};
/** 🏷️ The RESULTS legend — only nodes whose id says legend, or nodes INSIDE the World3d host. The simulation window's
 * own run text ("Final: 9509.739 kWh after 8760 of 8760 timesteps") is not a colour legend and is excluded explicitly:
 * a whole-document text scan for "kWh" passes on the 2026-09-16 baseline build that has no 3d window at all. */
const legend = (id) => page.evaluate((id) => {
  const host = document.querySelector(`[data-surface-id="${id}"]`) ?? [...document.querySelectorAll("[data-surface-id]")].find((e) => e.hasAttribute("data-meshes-json")) ?? null;
  const NOISE = /Final:|provisional|timesteps|Steady-state/i;
  const scoped = host ? [...host.querySelectorAll("*")] : [];
  const byId = [...document.querySelectorAll('[id*="legend" i], [data-legend], [class*="legend" i]')];
  const nodes = [...new Set([...scoped, ...byId])]
    .filter((e) => /legend/i.test(e.id ?? "") || /legend/i.test(e.className?.toString?.() ?? "") || e.hasAttribute("data-legend") || (e.children.length === 0 && /legend|legende|kWh|W\/m²|W\/m2|min\b.*\bmax\b/i.test((e.textContent ?? "").slice(0, 160))))
    .map((e) => ({ id: e.id || null, inWorldHost: Boolean(host && host.contains(e)), text: (e.textContent ?? "").trim().replace(/\s+/g, " ").slice(0, 160) }))
    .filter((e) => e.text && !NOISE.test(e.text));
  const seen = new Set();
  return { hostFound: Boolean(host), nodes: nodes.filter((n) => (seen.has(n.text) ? false : (seen.add(n.text), true))).slice(0, 20) };
}, id);
const changed = (a, b) => !a || !b ? false : a.meshDigest !== b.meshDigest || a.instanceDigest !== b.instanceDigest || JSON.stringify(a.histogram) !== JSON.stringify(b.histogram) || JSON.stringify(a.uniqueInstanceColors) !== JSON.stringify(b.uniqueInstanceColors);

//#region 🎞️ Recolour timeline
/** 🎞️ One timeline frame of the 3d host while the run is in flight — everything lane D needs to see WHERE the
 * result → colour chain stops: does the guest republish the surface at all (`data-status-json` counters, lane digests),
 * does the host render any text into the window (caption/legend), and does the shell log a refresh for this body. */
const hostFrame = (id) => page.evaluate((id) => {
  const el = document.querySelector(`[data-surface-id="${id}"]`) ?? [...document.querySelectorAll("[data-surface-id]")].find((e) => e.hasAttribute("data-meshes-json"));
  if (!el) return null;
  const digest = (s) => { let h = 7; for (let i = 0; i < s.length; i++) h = (h * 31 + s.charCodeAt(i)) >>> 0; return h; };
  const meshesRaw = el.getAttribute("data-meshes-json") ?? "";
  const instancesRaw = el.getAttribute("data-instances-json") ?? "";
  let status = null;
  try { status = JSON.parse(el.getAttribute("data-status-json") ?? "null"); } catch { status = el.getAttribute("data-status-json"); }
  // any counter-ish scalar the status doc carries, one level deep
  const counters = {};
  const walk = (obj, prefix, depth) => {
    if (!obj || typeof obj !== "object" || depth > 2) return;
    for (const [k, v] of Object.entries(obj)) {
      if (typeof v === "number" || typeof v === "string") { if (/rev|gen|version|rendered|frame|seq|epoch|updated|tick|step|count/i.test(k)) counters[`${prefix}${k}`] = v; }
      else walk(v, `${prefix}${k}.`, depth + 1);
    }
  };
  walk(status, "", 0);
  // text nodes actually rendered inside the world host (a caption/legend would be one of these)
  const walker = document.createTreeWalker(el, NodeFilter.SHOW_TEXT);
  const texts = [];
  while (walker.nextNode()) { const t = (walker.currentNode.nodeValue ?? "").trim(); if (t) texts.push(t.replace(/\s+/g, " ").slice(0, 80)); }
  const kWhText = texts.find((t) => /kWh|W\/m²|W\/m2/i.test(t)) ?? null;
  let histogram = new Array(8).fill(0), channels = 0, instanceColours = [];
  try {
    for (const m of JSON.parse(meshesRaw || "[]")) {
      const colors = m?.data?.colors;
      if (!Array.isArray(colors)) continue;
      channels += colors.length;
      for (let i = 0; i + 2 < colors.length; i += 3) {
        const lum = Number(colors[i]) * 0.3 + Number(colors[i + 1]) * 0.59 + Number(colors[i + 2]) * 0.11;
        histogram[Math.max(0, Math.min(7, Math.floor(lum * 8)))] += 1;
      }
    }
  } catch {}
  try { instanceColours = [...new Set(JSON.parse(instancesRaw || "[]").map((i) => i?.color).filter(Boolean))]; } catch {}
  const toolRunSteps = [...document.querySelectorAll('[id^="panel:"]')].filter((e) => /toolrun/i.test(e.id))
    .map((e) => (e.textContent ?? "").replace(/\s+/g, " ").trim()).filter(Boolean).slice(0, 6);
  return {
    status, counters,
    meshBytes: meshesRaw.length, instanceBytes: instancesRaw.length,
    meshDigest: digest(meshesRaw), instanceDigest: digest(instancesRaw),
    histogram, vertexColourChannels: channels, instanceColours,
    textNodes: texts.length, kWhInHost: Boolean(kWhText), kWhText,
    hostTexts: texts.slice(0, 6),
    toolRunSteps,
  };
}, id);
/** 📻️ The console lines lane D asked for: anything naming this window body or a UI refresh/dirty scope. */
const REFRESH_RE = /energy\.model\.3d|window_bodies|window bodies|refreshUi|refresh ui|dirty|spawn-job|job done|toolRun|surface-render/i;
const timeline = [];
const frame = async (label, consoleFrom) => {
  const host = await hostFrame(surfaceId);
  const run = await runStateRead();
  const consoleDelta = lines.slice(consoleFrom).filter((l) => REFRESH_RE.test(l));
  const entry = {
    t: Number(((Date.now() - t0) / 1000).toFixed(1)), label,
    finalized: run.finalized, busy: run.busy, state: run.state, runText: run.runText, colouredBy: run.colouredBy,
    statusPhase: host?.status?.phase ?? (host?.status == null ? null : typeof host.status === "string" ? host.status.slice(0, 40) : "object"),
    counters: host?.counters ?? {},
    meshDigest: host?.meshDigest ?? null, instanceDigest: host?.instanceDigest ?? null,
    meshBytes: host?.meshBytes ?? 0, histogram: host?.histogram ?? null,
    vertexColourChannels: host?.vertexColourChannels ?? null, instanceColours: host?.instanceColours ?? [],
    textNodes: host?.textNodes ?? null, kWhInHost: host?.kWhInHost ?? null, kWhText: host?.kWhText ?? null,
    hostTexts: host?.hostTexts ?? [],
    toolRunSteps: host?.toolRunSteps ?? [],
    refreshLines: consoleDelta.length,
    refreshSample: consoleDelta.slice(0, 3).map((l) => l.slice(0, 180)),
  };
  timeline.push(entry);
  report.timeline = timeline;
  writeFileSync(join(outDir, "timeline.txt"), timeline.map((e) =>
    `t=${String(e.t).padStart(6)}s ${e.label.padEnd(9)} run=${e.busy ?? "?"}/${e.finalized ? "final" : "…"} state="${(e.state ?? "").slice(0, 28)}" status=${e.statusPhase ?? "none"} counters=${JSON.stringify(e.counters)} meshDig=${e.meshDigest} instDig=${e.instanceDigest} meshBytes=${e.meshBytes} hist=${JSON.stringify(e.histogram)} instColours=${e.instanceColours.length} textNodes=${e.textNodes} kWhInHost=${e.kWhInHost} refresh+=${e.refreshLines} toolRun="${(e.toolRunSteps[0] ?? "").slice(0, 60)}"`).join("\n"));
  flush();
  return entry;
};
//#endregion 🎞️ Recolour timeline

try {
  await page.goto(url, { waitUntil: "domcontentloaded" });
  let s = null;
  for (let i = 0; i < bootSeconds; i++) { await page.waitForTimeout(1000); s = await shell(); if (s.error) break; if (s.ready && s.surfaces.length && i > 10) break; }
  note("boot", { ...s, body: undefined });
  await shot("1-boot");
  assert("boot.ready", Boolean(s?.ready) && !s?.error, `data-semio-os-ready=${s?.ready ?? "null"} data-semio-os-error=${s?.error ?? "null"}`);

  const before = await colourSample(surfaceId);
  const canvasBefore = await canvasHistogram(surfaceId);
  note("coloursBefore", { sample: before, canvas: canvasBefore });
  if (!before) assert("results.world3d", false, `${surfaceId} absent — the served build publishes no World3d surface (surfaces: ${s?.surfaces?.join(", ") || "none"}); DoD 4 cannot be measured until wave 1 lands the 3d window.`);
  else assert("results.world3d", true, `${before.surfaceId}: ${before.meshCount} meshes, ${before.instanceCount} instances, ${before.vertexColourChannels} vertex colour channels, ${before.uniqueInstanceColourCount} distinct instance colours`);

  // ── arm the tool and run ────────────────────────────────────────────────
  const category = page.locator('[id="framework.category.tool"]').first();
  let armed = "absent";
  if (await category.count()) {
    await category.click({ force: true }).catch(() => {});
    await page.waitForTimeout(900);
    // ⚠️ Do NOT click `tool.energySimulation`: opening the category already auto-armed it, a click is a re-press that disarms.
    armed = (await page.locator('[id="tool.energySimulation"]').count()) ? "auto-armed by the Tool category" : "tool row absent after opening the category";
  }
  await shot("2-tool-armed");
  const pre = await runStateRead();
  const consoleAtChord = lines.length;
  await frame("pre-chord", consoleAtChord);
  await page.keyboard.press(process.platform === "darwin" ? "Meta+Enter" : "Control+Enter");
  const seen = [];
  let runState = null;
  let lastFrameConsole = consoleAtChord;
  for (let i = 0; i < runSeconds * 2; i++) {
    await page.waitForTimeout(500);
    runState = await runStateRead();
    const key = `busy=${runState.busy ?? "?"} finalized=${runState.finalized} ${runState.state ?? ""} ${runState.runText ?? ""}`.trim();
    if (seen[seen.length - 1] !== key) seen.push(key);
    // 🎞️ one timeline frame every ~2 s while the run is in flight
    if (i % 4 === 3) { await frame("running", lastFrameConsole); lastFrameConsole = lines.length; }
    // a run counts as finished when it reads Finalized AND that is not just the pre-existing text from before the chord
    if (runState.finalized && (!pre.finalized || runState.runText !== pre.runText)) break;
  }
  await frame("finalized", lastFrameConsole);
  await page.waitForTimeout(3000);
  await frame("settled", lines.length - 1);
  note("run", { armed, pre, seen: seen.slice(-14), finalized: runState?.finalized, busy: runState?.busy, state: runState?.state, runText: runState?.runText, colouredBy: runState?.colouredBy, live: runState?.live, toolRunRows: runState?.toolRunRows });
  await shot("3-run");
  assert("results.run", Boolean(runState?.finalized) && (!pre.finalized || runState.runText !== pre.runText),
    `tool ${armed}; final state busy=${runState?.busy ?? "?"} "${runState?.state ?? ""}" runText="${runState?.runText ?? ""}"; transitions: ${JSON.stringify(seen.slice(-6))}${runState?.finalized ? "" : " — never reached Finalized"}`);

  // ── colours changed ─────────────────────────────────────────────────────
  await page.waitForTimeout(2500);
  const after = await colourSample(surfaceId);
  const canvasAfter = await canvasHistogram(surfaceId);
  note("coloursAfter", { sample: after, canvas: canvasAfter, changed: changed(before, after) });
  await shot("4-recoloured");
  if (!before || !after) assert("results.recoloured", false, "skipped — no World3d surface to compare");
  else assert("results.recoloured", changed(before, after),
    `meshDigest ${before.meshDigest}→${after.meshDigest}, instanceDigest ${before.instanceDigest}→${after.instanceDigest}, histogram ${JSON.stringify(before.histogram)}→${JSON.stringify(after.histogram)}, distinct instance colours ${before.uniqueInstanceColourCount}→${after.uniqueInstanceColourCount}; canvas readback ${canvasAfter?.available ? `available (nonBlank ${canvasBefore?.nonBlank}→${canvasAfter.nonBlank})` : `unavailable (${canvasAfter?.why})`}`);

  // ── legend ──────────────────────────────────────────────────────────────
  const caption = await legend(surfaceId);
  // 🏷️ The wave-1 result caption ("Surfaces coloured by: …") lives in the simulation window, not inside the world host —
  // accept it explicitly. It is specific enough not to be the false positive a bare "kWh" scan was.
  const colouredBy = (await runStateRead()).colouredBy;
  if (colouredBy) caption.nodes.unshift({ id: null, inWorldHost: false, text: colouredBy });
  note("legend", { ...caption, colouredBy, hostText: after?.hostText ?? null });
  assert("results.legend", caption.nodes.length > 0,
    caption.nodes.length ? `legend text: ${JSON.stringify(caption.nodes.slice(0, 4).map((c) => `${c.inWorldHost ? "[in world host] " : ""}${c.text}`))}` : `no legend caption inside the world host and no element with a legend id (world host found=${caption.hostFound}) — DoD 4's legend is missing`);

  // ── result-field switch through the SIMULATION window's Actions pane ────
  // 🎛️ The form has exactly ONE control — a shadcn Select trigger `#field` ("Coloured by", a BUTTON with
  // role=combobox) and NO execute button: picking an option IS the commit. The proof that it committed is the
  // simulation window's own caption flipping from "Surfaces coloured by: <old>" to the picked field.
  const st = await shell();
  const rowId = `action.${resultAction}`;
  const captionBefore = (await runStateRead()).colouredBy;
  let fieldSwitch = { engagement: null, toggled: "skipped", clicked: "skipped", picked: "skipped", options: [], captionBefore, captionAfter: null };
  // ⚠️ The SIMULATION window's engagement, never the 3d one: `set-result-field` is declared on `energy.simulation`
  // only, and a naive /simulation|result|3d|model/ match picked `framework.window.energyModel3d.engagement` first —
  // whose Actions pane lists setCamera/set-*-property and no result field, so the step reported "no action row".
  const engagement = st.engagements.find((id) => /energySimulation/i.test(id))
    ?? st.engagements.find((id) => /simulation/i.test(id) && !/3d|model3d/i.test(id))
    ?? st.engagements[0] ?? null;
  fieldSwitch.engagement = engagement;
  if (engagement) {
    const toggle = page.locator(`[id="${engagement}.toggle"]`).first();
    // 🫥️ `force`: a window header can overlay its own Actions toggle, so an actionability check never passes.
    if (await toggle.count()) fieldSwitch.toggled = await toggle.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
    await page.waitForTimeout(1600);
  }
  const opened = await shell();
  if (opened.actionRows.includes(rowId)) {
    fieldSwitch.clicked = await page.locator(`[id="${rowId}"]`).first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
    await page.waitForTimeout(2000);
    const select = page.locator('[id="field"], [name="field"], [id="resultField"], [id$=".field"]').first();
    fieldSwitch.control = (await select.count()) ? await select.evaluate((el) => ({ id: el.id || null, tag: el.tagName, role: el.getAttribute("role"), text: (el.textContent ?? "").trim().slice(0, 40) })).catch(() => null) : null;
    if (await select.count()) {
      await select.click({ force: true }).catch(() => {});
      await page.waitForTimeout(800);
      fieldSwitch.options = await page.locator('[role="option"]').allInnerTexts().catch(() => []);
      const norm = (t) => t.toLowerCase().replace(/[^a-z]/g, "");
      const want = norm(resultField);
      // match the env value against the option TEXT (case/space-insensitive, camelCase humanised); if it names no real
      // option, pick any option that differs from the field currently painted — the point is to CHANGE the field.
      const index = fieldSwitch.options.findIndex((t) => norm(t) === want || norm(t).includes(want) || want.includes(norm(t)));
      const currentIndex = fieldSwitch.options.findIndex((t) => (captionBefore ?? "").toLowerCase().includes(t.toLowerCase()));
      const chosen = index >= 0 ? index : fieldSwitch.options.findIndex((_, i) => i !== currentIndex);
      fieldSwitch.chosenOption = fieldSwitch.options[chosen] ?? null;
      fieldSwitch.chosenBecause = index >= 0 ? `SEMIO_PROBE_RESULT_FIELD=${resultField} matched it` : `'${resultField}' names no option; picked the first one that is not the current '${fieldSwitch.options[currentIndex] ?? "?"}'`;
      if (chosen >= 0) fieldSwitch.picked = await page.locator('[role="option"]').nth(chosen).click({ timeout: 6000 }).then(() => `ok:${fieldSwitch.chosenOption}`).catch((e) => String(e).slice(0, 140));
      else fieldSwitch.picked = `no option to pick among ${JSON.stringify(fieldSwitch.options)}`;
      await page.keyboard.press("Escape").catch(() => {});
      // there is no execute button in this form, but look for one anyway in case the shape changes
      const submit = page.locator(`[id$=".action.${resultAction}.execute"], [id$=".execute"], [id$=".apply"]`).first();
      fieldSwitch.executeButton = (await submit.count()) ? await submit.click({ timeout: 4000 }).then(() => "clicked") : "none (the select IS the commit)";
      await page.waitForTimeout(4000);
    } else fieldSwitch.picked = "the action form published no field control";
  } else fieldSwitch.clicked = `no ${rowId} row (action rows: ${opened.actionRows.slice(0, 12).join(", ") || "none"})`;
  fieldSwitch.captionAfter = (await runStateRead()).colouredBy;
  await frame("field-switch", lines.length - 1);
  const afterField = await colourSample(surfaceId);
  note("resultFieldSwitch", { ...fieldSwitch, sample: afterField, changed: changed(after, afterField) });
  await shot("5-result-field");
  const captionChanged = Boolean(fieldSwitch.captionAfter) && fieldSwitch.captionAfter !== fieldSwitch.captionBefore;
  const fieldDetail = `${rowId}: toggle=${fieldSwitch.toggled} row=${fieldSwitch.clicked} pick=${fieldSwitch.picked} (${fieldSwitch.chosenBecause ?? "-"}); options=${JSON.stringify(fieldSwitch.options)}; caption "${(fieldSwitch.captionBefore ?? "").slice(0, 40)}" → "${(fieldSwitch.captionAfter ?? "").slice(0, 40)}"; colours changed again=${changed(after, afterField)}`;
  if (String(fieldSwitch.clicked).startsWith("no ") && process.env.SEMIO_PROBE_REQUIRE_FIELD_SWITCH !== "1") skip("results.fieldSwitch", `${fieldDetail} — the window publishes no ${rowId} action; the result-field switcher is optional (set SEMIO_PROBE_REQUIRE_FIELD_SWITCH=1 to make it a hard assertion)`);
  // ✅️ The caption is the honest proof that the action COMMITTED. The scene recolouring again is reported but not
  // required here, because the colours revert at Finalized anyway — that is `results.recoloured`'s finding, not this one.
  else assert("results.fieldSwitch", captionChanged, fieldDetail);

  const faults = lines.filter((l) => /pageerror|trapped|panicked|unreachable|dropped action|Unknown action|fixed-capacity|surface-render|window-context-required/i.test(l)).map((l) => l.slice(0, 300));
  note("faults", faults.slice(0, 20));
  assert("no.faults", faults.length === 0, `${faults.length} fault line(s)${faults.length ? `: ${faults[0]}` : ""}`);
} catch (error) {
  report.crash = String(error?.stack ?? error).slice(0, 2000);
  assert("probe.completed", false, `probe threw: ${String(error).slice(0, 300)}`);
  await shot("error");
}

report.finishedAt = new Date().toISOString();
if (timeline.length) {
  console.log("── recolour timeline ──────────────────────────────────────────────");
  for (const e of timeline) console.log(`t=${String(e.t).padStart(6)}s ${e.label.padEnd(9)} run=${e.busy ?? "?"}/${e.finalized ? "final" : "…"} status=${e.statusPhase ?? "none"} counters=${JSON.stringify(e.counters)} meshDig=${e.meshDigest} instDig=${e.instanceDigest} bytes=${e.meshBytes} hist=${JSON.stringify(e.histogram)} instColours=${e.instanceColours.length} textNodes=${e.textNodes} kWhInHost=${e.kWhInHost} refresh+=${e.refreshLines}`);
  console.log("───────────────────────────────────────────────────────────────────");
}
report.passed = report.assertions.filter((a) => a.ok).length;
report.failed = report.assertions.filter((a) => !a.ok).length;
report.result = report.failed === 0 ? "PASS" : "FAIL";
flush();
console.log(`RESULT=${report.result} passed=${report.passed} failed=${report.failed} skipped=${(report.skipped ?? []).length} out=${outDir}`);
for (const a of report.skipped ?? []) console.log(`  SKIP ${a.id}: ${a.detail}`);
for (const a of report.assertions.filter((x) => !x.ok)) console.log(`  FAIL ${a.id}: ${a.detail}`);
await browser.close();
process.exitCode = report.failed === 0 ? 0 : 1;
