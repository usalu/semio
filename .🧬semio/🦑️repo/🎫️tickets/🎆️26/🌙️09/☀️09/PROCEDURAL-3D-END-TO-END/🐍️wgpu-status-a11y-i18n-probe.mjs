/** ♿️⏳️🗣️ wgpu STATUS / ACCESSIBILITY / I18N probe — the runtime half of lane `wgpu-status-a11y-i18n`.
 *
 * Proves, from a user's seat, on `http://127.0.0.1:6118/?plugin=generation3d`:
 *   • ⏳️ the World3d compute-status PILL paints while an example evaluates (phase + ratio) and is
 *     gone once the producer settles — the shell's own
 *     `[DEBUG] wgpu world3d status pill surface=… phase=… label=… ratio=…` trace, which prints once
 *     per CHANGE, cross-checked against the producer's own `world3d surface=… status=Some(…)` line;
 *   • ♿️ the accessibility projection is readable for the generation3d windows — both as the ARIA
 *     mirror the host maintains beside the canvas (`#semio-wgpu-accessibility`, real `role`/
 *     `aria-label`/`aria-live`/`aria-keyshortcuts`/`aria-hidden` attributes) and as the renderer's
 *     own `dumpAccessibility(windowId)` per window;
 *   • 🗣️ a locale switch to German through the command palette (`mod+p` → "Deutsch" → Enter) turns
 *     the chrome AND the generation3d window labels German.
 *
 * Nothing is clicked at a guessed pixel: the palette is opened by its own chord, and the example
 * picker is derived from the shell's own `os_host pointer hit` trace the way `🐍️wgpu-journey-probe.mjs`
 * derives it. The wgpu browser tick is INPUT-DRIVEN, so every wait pumps a 1 px nudge.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-status-a11y/run-1 bun 🐍️wgpu-status-a11y-i18n-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const baseUrl = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-status-a11y/run");
const bootSeconds = Number(process.env.SEMIO_PROBE_BOOT ?? 180);
const viewport = { width: 1440, height: 900 };
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const note = (text) => {
  lines.push(`${at()} PROBE ${text}`);
  console.log(`[DEBUG] ${text}`);
};

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
let page = await browser.newPage({ viewport });
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 6000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));

const has = (needle) => lines.filter((line) => line.includes(needle));
const lastOf = (needle) => has(needle).at(-1) ?? null;
const shot = (name) => page.screenshot({ path: join(outDir, `${name}.png`) }).catch(() => {});

const dropOverlay = () =>
  page
    .evaluate(() => {
      const overlay = Array.from(document.body.children).find((node) => node.tagName === "DIV" && node.id !== "root" && node.id !== "semio-wgpu-accessibility");
      if (!overlay) return null;
      const text = (overlay.textContent ?? "").slice(0, 200);
      overlay.remove();
      return text;
    })
    .catch(() => null);

const introspect = (kind, windowId) =>
  page
    .evaluate(
      async ([kind, id]) => {
        const beacon = globalThis.semioWgpuIntrospection;
        if (typeof beacon?.[kind] !== "function") return null;
        try {
          const raw = await beacon[kind](id);
          return raw ? JSON.parse(raw) : null;
        } catch (error) {
          return { error: String(error) };
        }
      },
      [kind, windowId],
    )
    .catch(() => null);

const liveWindowIds = async () => (await introspect("dumpStructure", undefined))?.windowIds ?? [];

/** 🔬️ Which introspection hooks the host actually attached — `dumpAccessibility` is the new one, so
 * "0 nodes" must never be confused with "the hook was never published". */
const beaconKeys = () => page.evaluate(() => Object.keys(globalThis.semioWgpuIntrospection ?? {})).catch(() => []);

/** ♿️ The ARIA subtree the host maintains beside the canvas — what an assistive technology reads. */
const mirror = () =>
  page
    .evaluate(() => {
      const root = document.getElementById("semio-wgpu-accessibility");
      if (!root) return null;
      return {
        role: root.getAttribute("role"),
        label: root.getAttribute("aria-label"),
        nodeCount: Number(root.dataset.nodeCount ?? 0),
        nodes: Array.from(root.children).map((node) => ({
          role: node.getAttribute("role"),
          label: node.getAttribute("aria-label"),
          live: node.getAttribute("aria-live"),
          shortcut: node.getAttribute("aria-keyshortcuts"),
          hidden: node.getAttribute("aria-hidden"),
          disabled: node.getAttribute("aria-disabled"),
          describedBy: node.getAttribute("aria-describedby"),
          description: node.querySelector("span")?.textContent ?? null,
          key: node.dataset.nodeKey ?? null,
          depth: Number(node.dataset.depth ?? 0),
          focusable: node.dataset.focusable === "true",
          actionable: node.dataset.actionable === "true",
          focused: node.dataset.focused === "true",
        })),
      };
    })
    .catch(() => null);

const pump = async (ms, park) => {
  const deadline = Date.now() + ms;
  let flip = 0;
  const point = park ?? [3, 3];
  while (Date.now() < deadline) {
    await page.waitForTimeout(180);
    flip = 1 - flip;
    await page.mouse.move(point[0] + flip, point[1]).catch(() => {});
  }
};

const dockPlan = () => {
  const line = lastOf("wgpu-shell dock plan");
  if (!line) return {};
  const plan = {};
  for (const token of line.split(" ").slice(1)) {
    const match = /^(.+)@(\d+(?:\.\d+)?)x(\d+(?:\.\d+)?)\+(-?\d+(?:\.\d+)?),(-?\d+(?:\.\d+)?)$/.exec(token);
    if (match) plan[match[1]] = { w: Number(match[2]), h: Number(match[3]), x: Number(match[4]), y: Number(match[5]) };
  }
  return plan;
};

const hitLines = (fromIndex = 0) => {
  const parsed = [];
  for (const line of lines.slice(fromIndex)) {
    const match = /os_host pointer hit x=([-\d.]+) y=([-\d.]+) targets=(\d+) hit=(.*)$/.exec(line);
    if (!match) continue;
    const some = /Some\(\((\w+), Some\("([^"]+)"\)\)\)/.exec(match[4]);
    parsed.push({ x: Number(match[1]), y: Number(match[2]), targets: Number(match[3]), kind: some?.[1] ?? null, id: some?.[2] ?? null });
  }
  return parsed;
};

const sweepNavbar = async (label) => {
  const plan = dockPlan();
  const tops = Object.values(plan).map((body) => body.y);
  const bottom = tops.length ? Math.min(...tops) : 40;
  const rows = [];
  for (let y = 8; y < Math.max(bottom, 44); y += 8) rows.push(y);
  const mark = lines.length;
  for (const y of rows) {
    for (let x = 6; x < viewport.width - 4; x += 18) {
      await page.mouse.move(x, y);
      await page.waitForTimeout(35);
    }
  }
  const controls = {};
  for (const hit of hitLines(mark)) {
    if (!hit.id) continue;
    const entry = (controls[hit.id] ??= { id: hit.id, x0: hit.x, x1: hit.x, y: hit.y });
    entry.x0 = Math.min(entry.x0, hit.x);
    entry.x1 = Math.max(entry.x1, hit.x);
  }
  for (const entry of Object.values(controls)) entry.point = [(entry.x0 + entry.x1) / 2, entry.y];
  note(`sweep ${label}: controls=${JSON.stringify(Object.keys(controls))}`);
  return controls;
};

/** ⏳️ Every status-pill announcement the shell printed, newest last. */
const pillTraces = () =>
  has("wgpu world3d status pill").map((line) => ({
    surface: /surface=(\S+)/.exec(line)?.[1] ?? null,
    phase: /phase=(\S+)/.exec(line)?.[1] ?? null,
    label: /label="((?:[^"\\]|\\.)*)"/.exec(line)?.[1] ?? null,
    ratio: /ratio=(Some\([\d.]+\)|None)/.exec(line)?.[1] ?? null,
    computing: /computing=(true|false)/.exec(line)?.[1] ?? null,
    rect: /rect=(\S+)/.exec(line)?.[1] ?? null,
  }));

/** 🌍️ The producer's own published status, per surface. */
const producerStatus = () => {
  const byId = {};
  for (const line of has("world3d surface=")) {
    const surface = /world3d surface=(\S+)/.exec(line)?.[1];
    if (!surface) continue;
    byId[surface] = /status=Some\("((?:[^"\\]|\\.)*)"\)/.exec(line)?.[1] ?? null;
  }
  return byId;
};

const report = { url: baseUrl, startedAt: new Date().toISOString(), steps: [] };
const record = (step, verdict, detail) => {
  report.steps.push({ step, verdict, t: at(), ...detail });
  note(`${step}: ${verdict} ${JSON.stringify(detail).slice(0, 600)}`);
};

//#region 🚀️Boot
await page.goto(baseUrl, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));
let windowIds = [];
for (let second = 0; second < bootSeconds; second += 1) {
  await pump(1000);
  windowIds = await liveWindowIds();
  if (windowIds.length > 0 && Object.keys(dockPlan()).length > 0) break;
}
record("boot", windowIds.length > 0 ? "pass" : "fail", { windowIds, seconds: Math.round(at() / 1000) });
await dropOverlay();
await shot("01-booted");
//#endregion 🚀️Boot

//#region ♿️Accessibility
/** ♿️ The mirror refreshes on a UI turn with a 400 ms floor, so a single sample can legitimately
 * catch it between the first publish and the first refresh — poll until it carries the tree. */
let mirrored = null;
for (let attempt = 0; attempt < 12; attempt += 1) {
  await pump(700);
  mirrored = await mirror();
  if ((mirrored?.nodeCount ?? 0) > 0) break;
}
const hooks = await beaconKeys();
const perWindow = { "(default)": await introspect("dumpAccessibility", undefined) };
for (const id of windowIds) perWindow[id] = await introspect("dumpAccessibility", id);
const perWindowCounts = Object.fromEntries(Object.entries(perWindow).map(([id, dump]) => [id, { windowId: dump?.windowId ?? null, windowIds: dump?.windowIds ?? null, count: dump?.nodes?.length ?? 0, error: dump?.error ?? null }]));
const labelled = (mirrored?.nodes ?? []).filter((node) => node.label);
record("accessibility:mirror", mirrored && mirrored.nodeCount > 0 ? "pass" : "fail", {
  mirrorRole: mirrored?.role ?? null,
  mirrorLabel: mirrored?.label ?? null,
  nodeCount: mirrored?.nodeCount ?? 0,
  labelled: labelled.length,
  roles: [...new Set((mirrored?.nodes ?? []).map((node) => node.role))],
  liveRegions: (mirrored?.nodes ?? []).filter((node) => node.live).length,
  shortcuts: (mirrored?.nodes ?? []).filter((node) => node.shortcut).length,
  hidden: (mirrored?.nodes ?? []).filter((node) => node.hidden === "true").length,
  firstLabels: labelled.slice(0, 12).map((node) => `${node.role}:${node.label}`),
});
record("accessibility:per-window", Object.values(perWindowCounts).some((entry) => entry.count > 0) ? "pass" : "fail", { hooks, perWindow: perWindowCounts });
writeFileSync(join(outDir, "accessibility.json"), JSON.stringify({ mirror: mirrored, perWindow }, null, 2));
//#endregion ♿️Accessibility

//#region ⏳️StatusPill
/** ⏳️ The bundled example evaluates at ~7 s, LONG before the World3d surface first attaches at ~27 s,
 * so the boot evaluation is already settled by the time the shell can observe any status at all. A
 * live re-evaluation is therefore needed, and the one chrome control this session reliably offers is
 * the surface-role switch: it recreates the session, which re-evaluates while the shell is up. */
const navbar = await sweepNavbar("before the live re-evaluation");
const trigger = navbar["playground.navbar.fixture"] ?? navbar["playground.navbar.roles.viewer"] ?? null;
const beforePills = pillTraces().length;
if (trigger) {
  await page.mouse.click(trigger.point[0], trigger.point[1]);
  for (let tick = 0; tick < 60 && pillTraces().length === beforePills; tick += 1) await pump(250);
  await shot("02-status-pill-while-computing");
}
const pillDuring = pillTraces().slice(beforePills);
record("status:trigger", trigger ? "pass" : "blocked", { control: trigger?.id ?? null, offered: Object.keys(navbar) });
await pump(10000);
const pillAll = pillTraces();
const settled = producerStatus();
const computing = pillAll.filter((row) => row.computing === "true");
const withRatio = computing.filter((row) => row.ratio && row.ratio !== "None");
record("status:pill-while-computing", computing.length > 0 ? "pass" : "fail", {
  announcements: pillAll.slice(0, 12),
  computing: computing.length,
  withRatio: withRatio.length,
  phases: [...new Set(computing.map((row) => row.phase))],
  fromTrigger: pillDuring.length,
  total: pillAll.length,
  producerEverComputing: has('computing\\":true').length,
});
const mirrorAfter = await mirror();
record("status:settled", Object.values(settled).some((status) => status && status.includes('phase\\":\\"idle')) ? "pass" : "unknown", {
  producerStatus: Object.fromEntries(Object.entries(settled).map(([id, status]) => [id, (status ?? "").slice(0, 160)])),
  lastPill: pillAll.at(-1) ?? null,
  mirrorNodes: mirrorAfter?.nodeCount ?? 0,
});
await shot("03-status-settled");
//#endregion ⏳️StatusPill

//#region 🗣️Locale
/** 🗣️ Two routes, both real: the shell's own quick-search palette (`mod+p` → "Deutsch" → Enter), and
 * a fresh German-locale browser context, which is what a German user's browser actually does — the
 * host reads `navigator.language` and hands the worker its boot locale. The second is the fallback
 * when the palette cannot be reached, and is reported as such rather than as the same proof. */
const germanWords = ["Knoten", "Leitungen", "Eingang", "Ausgang", "Workflow", "Vorschau", "Generationen", "Formular", "Beispiel", "Einstellungen", "Anzeige", "Arbeitsbereich", "Abbrechen", "Zurücksetzen"];
const germanEvidence = async (context) => {
  const texts = {};
  for (const id of await liveWindowIds()) {
    const dump = await introspect("dumpStructure", id);
    texts[id] = (dump?.nodes ?? []).map((node) => node.text).filter((text) => typeof text === "string" && text.length > 0);
  }
  const labels = (await mirror())?.nodes?.map((node) => node.label).filter(Boolean) ?? [];
  const found = [...new Set([...Object.values(texts).flat(), ...labels])].filter((text) => germanWords.some((word) => text.includes(word)));
  return { context, texts: Object.fromEntries(Object.entries(texts).map(([id, list]) => [id, list.slice(0, 24)])), labels: labels.slice(0, 24), germanFound: found };
};

await page.click("#semio-wgpu-canvas", { position: { x: 40, y: 400 } }).catch(() => {});
await pump(600);
await page.keyboard.press("Control+p");
await pump(1500);
await shot("04-palette-open");
await page.keyboard.type("Deutsch", { delay: 60 });
await pump(1500);
await shot("05-palette-deutsch");
await page.keyboard.press("Enter");
await pump(6000);
await dropOverlay();
await shot("06-german-chrome");
const viaPalette = await germanEvidence("palette");
record("locale:german-via-palette", viaPalette.germanFound.length > 0 ? "pass" : "fail", viaPalette);

if (viaPalette.germanFound.length === 0) {
  const german = await browser.newContext({ viewport, locale: "de-DE" });
  const originalPage = page;
  page = await german.newPage();
  page.on("console", (m) => lines.push(`${at()} de ${m.type()} ${m.text().slice(0, 4000)}`));
  await page.goto(baseUrl, { waitUntil: "domcontentloaded" }).catch(() => {});
  let ids = [];
  for (let second = 0; second < bootSeconds; second += 1) {
    await pump(1000);
    ids = await liveWindowIds();
    if (ids.length > 0) break;
  }
  for (let attempt = 0; attempt < 12 && (await mirror())?.nodeCount === 0; attempt += 1) await pump(700);
  await shot("07-german-boot");
  const viaBoot = await germanEvidence("de-DE browser locale");
  record("locale:german-via-boot-locale", viaBoot.germanFound.length > 0 ? "pass" : "fail", { windowIds: ids, ...viaBoot });
  writeFileSync(join(outDir, "german.json"), JSON.stringify({ viaPalette, viaBoot }, null, 2));
  await page.close().catch(() => {});
  page = originalPage;
} else {
  writeFileSync(join(outDir, "german.json"), JSON.stringify({ viaPalette }, null, 2));
}
//#endregion 🗣️Locale

writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "pill-traces.json"), JSON.stringify(pillTraces(), null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
note(`done — ${report.steps.map((step) => `${step.step}=${step.verdict}`).join(" ")}`);
await browser.close();
