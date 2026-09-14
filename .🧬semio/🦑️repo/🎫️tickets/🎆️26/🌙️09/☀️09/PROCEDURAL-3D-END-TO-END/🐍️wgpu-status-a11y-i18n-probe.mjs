/** ♿️⏳️🗣️ wgpu STATUS / ACCESSIBILITY / I18N probe — the runtime half of lane `wgpu-status-a11y-i18n`,
 * re-armed by lane `wgpu-a11y-status-i18n-runtime` (2026-09-14).
 *
 * Proves, from a user's seat, on `http://127.0.0.1:6118/?plugin=generation3d`:
 *   • ♿️ the ARIA mirror beside the canvas carries the app's LIVE accessibility tree — every live
 *     window, not just the largest — and keeps carrying it: the mirror is now driven by the
 *     transport's FRAME channel (`onDirectives`), where it used to hang off `onUiTurn`, which the
 *     transport raises only for a turn that BREACHED its budget. A healthy shell overruns once at
 *     boot, so the mirror was painted once, from a race, and never again (`nodeCount 0` on 6118).
 *   • ⏳️ the World3d compute-status pill, driven from the shell's own example picker so the surface
 *     is already live when the evaluation starts — the ordering the earlier lanes could not set up.
 *   • 🗣️ a locale switch to German through the command palette (`mod+p` → "Deutsch" → Enter), read
 *     back from the shell's own `[DEBUG] wgpu-shell locale resolved=…` trace, which names the
 *     resolved chrome strings, the app's own window labels and the os command registry. That trace
 *     is the ONLY runtime witness of a locale on this target: every translated string is painted
 *     into pixels and none of it is in the DOM, which is why two lanes could neither prove nor
 *     disprove a German session from outside the canvas.
 *
 * Nothing is clicked at a guessed pixel: every control is located from the shell's own
 * `os_host pointer hit` trace, and the palette is opened by its own chord. The wgpu browser tick is
 * INPUT-DRIVEN, so every wait pumps a 1 px nudge.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-status-a11y/run-1 bun 🐍️wgpu-status-a11y-i18n-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const baseUrl = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-status-a11y/run");
const bootSeconds = Number(process.env.SEMIO_PROBE_BOOT ?? 180);
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 45);
const statusExample = process.env.SEMIO_PROBE_EXAMPLE ?? "sphere-cut-with-torus";
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

/** ♿️ The ARIA subtree the host maintains beside the canvas — what an assistive technology reads.
 * `attr` is the RAW `data-node-count`: `null` means the mirror was never painted at all, `"0"` means
 * it was painted from an empty tree, and the two used to be indistinguishable to this probe. */
const mirror = () =>
  page
    .evaluate(() => {
      const root = document.getElementById("semio-wgpu-accessibility");
      if (!root) return null;
      return {
        role: root.getAttribute("role"),
        label: root.getAttribute("aria-label"),
        attr: root.getAttribute("data-node-count"),
        nodeCount: Number(root.dataset.nodeCount ?? 0),
        windows: (root.dataset.windows ?? "").split(" ").filter(Boolean),
        nodes: Array.from(root.children).map((node) => ({
          window: node.dataset.window ?? null,
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

const hitLines = (fromIndex = 0) => {
  const parsed = [];
  for (const line of lines.slice(fromIndex)) {
    const match = /os_host pointer hit x=([-\d.]+) y=([-\d.]+) targets=(\d+)(?:\s+(?!hit=)\w+=\S+)*\s+hit=(.*)$/.exec(line);
    if (!match) continue;
    const some = /Some\(\((\w+), Some\("([^"]+)"\)\)\)/.exec(match[4]);
    parsed.push({ x: Number(match[1]), y: Number(match[2]), targets: Number(match[3]), kind: some?.[1] ?? null, id: some?.[2] ?? null });
  }
  return parsed;
};

/** 🎯️ Sweeps a band and answers the centre of every control the shell's own hit trace named there. */
const sweep = async (label, y0, y1, x0, x1, step) => {
  const mark = lines.length;
  for (let y = y0; y < y1; y += step) {
    for (let x = x0; x < x1; x += 18) {
      await page.mouse.move(x, y);
      await page.waitForTimeout(28);
    }
  }
  const controls = {};
  for (const hit of hitLines(mark)) {
    if (!hit.id) continue;
    const entry = (controls[hit.id] ??= { id: hit.id, x0: hit.x, x1: hit.x, y0: hit.y, y1: hit.y });
    entry.x0 = Math.min(entry.x0, hit.x);
    entry.x1 = Math.max(entry.x1, hit.x);
    entry.y0 = Math.min(entry.y0, hit.y);
    entry.y1 = Math.max(entry.y1, hit.y);
  }
  for (const entry of Object.values(controls)) entry.point = [(entry.x0 + entry.x1) / 2, (entry.y0 + entry.y1) / 2];
  note(`sweep ${label}: ${JSON.stringify(Object.keys(controls))}`);
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

/** 🗣️ The shell's own locale-resolution trace, parsed — the runtime witness of a German session. */
const localeTrace = () => {
  const line = lastOf("wgpu-shell locale resolved=");
  if (!line) return null;
  const field = (name) => new RegExp(`${name}=\\[([^\\]]*)\\]`).exec(line)?.[1] ?? "";
  const pairs = (raw) => Object.fromEntries(raw.split(", ").filter(Boolean).map((entry) => [entry.slice(0, entry.indexOf("=")), entry.slice(entry.indexOf("=") + 1)]));
  return {
    locale: /resolved=(\S+)/.exec(line)?.[1] ?? null,
    terminology: /terminology=(\S+)/.exec(line)?.[1] ?? null,
    reason: /reason=(\S+)/.exec(line)?.[1] ?? null,
    chrome: pairs(field("chrome")),
    windows: pairs(field("windows")),
    commands: pairs(field("commands")),
  };
};

const report = { url: baseUrl, startedAt: new Date().toISOString(), steps: [] };
const record = (step, verdict, detail) => {
  report.steps.push({ step, verdict, t: at(), ...detail });
  note(`${step}: ${verdict} ${JSON.stringify(detail).slice(0, 700)}`);
};

//#region 🚀️Boot
await page.goto(baseUrl, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));
let windowIds = [];
for (let second = 0; second < bootSeconds; second += 1) {
  await pump(1000);
  windowIds = await liveWindowIds();
  if (windowIds.length > 0 && has("world3d surface=").length > 0) break;
}
record("boot", windowIds.length > 0 ? "pass" : "fail", { windowIds, seconds: Math.round(at() / 1000) });
// ⏳️ A live window is not a PAINTED shell: reading the chrome walk's products the instant the dock
// plans them samples a shell that has not walked its chrome or evaluated its example yet.
await pump(settleSeconds * 1000);
await shot("01-booted");
//#endregion 🚀️Boot

//#region ♿️Accessibility
let mirrored = null;
for (let attempt = 0; attempt < 12; attempt += 1) {
  await pump(700);
  mirrored = await mirror();
  if ((mirrored?.nodeCount ?? 0) > 0) break;
}
const hooks = await beaconKeys();
const perWindow = { "(all windows)": await introspect("dumpAccessibility", undefined) };
for (const id of windowIds) perWindow[id] = await introspect("dumpAccessibility", id);
const windowCount = (dump) => Object.fromEntries((dump?.windows ?? []).map((entry) => [entry.windowId, entry.nodes?.length ?? 0]));
const perWindowCounts = Object.fromEntries(Object.entries(perWindow).map(([id, dump]) => [id, { requested: dump?.windowId ?? null, windows: windowCount(dump), total: (dump?.windows ?? []).reduce((sum, entry) => sum + (entry.nodes?.length ?? 0), 0), error: dump?.error ?? null }]));
const labelled = (mirrored?.nodes ?? []).filter((node) => node.label);
record("accessibility:mirror", mirrored && mirrored.nodeCount > 0 ? "pass" : "fail", {
  mirrorRole: mirrored?.role ?? null,
  mirrorLabel: mirrored?.label ?? null,
  rawNodeCountAttribute: mirrored?.attr ?? null,
  nodeCount: mirrored?.nodeCount ?? 0,
  mirroredWindows: mirrored?.windows ?? [],
  labelled: labelled.length,
  roles: [...new Set((mirrored?.nodes ?? []).map((node) => node.role))],
  liveRegions: (mirrored?.nodes ?? []).filter((node) => node.live).length,
  shortcuts: (mirrored?.nodes ?? []).filter((node) => node.shortcut).length,
  hidden: (mirrored?.nodes ?? []).filter((node) => node.hidden === "true").length,
  firstLabels: labelled.slice(0, 12).map((node) => `${node.window}:${node.role}:${node.label}`),
});
record("accessibility:per-window", Object.values(perWindowCounts).some((entry) => entry.total > 0) ? "pass" : "fail", { hooks, perWindow: perWindowCounts });

/** ♿️ The mirror is only useful if it stays LIVE — it must still be the app's tree after the document
 * changes under it. The `status` section below switches the example, which rewrites every node name
 * in `procedural-main`, so that switch is this lane's liveness gesture; `accessibility:live` is
 * recorded there. A mirror wired to `onUiTurn` structurally cannot pass it: that channel is raised
 * only for a turn that BREACHED its budget, which a healthy shell does once, at boot. */
const mirrorBefore = mirrored;
//#endregion ♿️Accessibility

//#region ⏳️StatusPill
/** ⏳️ The pill needs an evaluation that starts AFTER the World3d surface is live. `?example=` in the
 * URL is the opposite ordering — it is evaluated inside `boot_shell`, before any window exists — so
 * the probe boots with no example and drives the shell's own example picker instead. */
const navbar = await sweep("navbar (example picker)", 8, 48, 6, viewport.width - 4, 8);
const trigger = navbar["playground.navbar.fixture"] ?? null;
let pickedRow = null;
const beforePills = pillTraces().length;
if (trigger) {
  await page.mouse.click(trigger.point[0], trigger.point[1]);
  await pump(2500);
  const rows = await sweep("example dropdown", 60, Math.round(viewport.height * 0.6), Math.round(viewport.width * 0.5 - 190), Math.round(viewport.width * 0.5 + 190), 10);
  const row = rows[`shell.example.${statusExample}`] ?? null;
  if (row) {
    pickedRow = row.id;
    await page.mouse.click(row.point[0], row.point[1]);
  } else {
    note(`no shell.example.${statusExample} row among ${JSON.stringify(Object.keys(rows))}`);
  }
}
record("status:trigger", pickedRow ? "pass" : "blocked", { control: trigger?.id ?? null, picked: pickedRow, offered: Object.keys(navbar) });
for (let tick = 0; tick < 200 && pillTraces().filter((row) => row.computing === "true").length === 0; tick += 1) {
  await pump(500);
  if (tick % 20 === 0 && pillTraces().length > beforePills) await shot(`02-status-pill-${tick}`);
}
await shot("02-status-pill-while-computing");
const pillAll = pillTraces();
const computing = pillAll.filter((row) => row.computing === "true");
const withRatio = computing.filter((row) => row.ratio && row.ratio !== "None");
record("status:pill-while-computing", computing.length > 0 ? "pass" : "fail", {
  announcements: pillAll.slice(0, 12),
  computing: computing.length,
  withRatio: withRatio.length,
  phases: [...new Set(computing.map((row) => row.phase))],
  total: pillAll.length,
  producerEverComputing: has('"computing":true').length + has('computing\\":true').length,
  cancelOffered: has("shell.world3d.cancel::").length,
});
await pump(20000);
const settled = producerStatus();
record("status:settled", Object.values(settled).some((status) => status && status.includes('phase\\":\\"idle')) ? "pass" : "unknown", {
  producerStatus: Object.fromEntries(Object.entries(settled).map(([id, status]) => [id, (status ?? "").slice(0, 200)])),
  lastPill: pillAll.at(-1) ?? null,
  mirrorNodes: (await mirror())?.nodeCount ?? 0,
});
await shot("03-status-settled");

const mirrorAfterExample = await mirror();
const labelsOf = (snapshot) => (snapshot?.nodes ?? []).filter((node) => node.label).map((node) => `${node.window}:${node.label}`);
const before = labelsOf(mirrorBefore);
const after = labelsOf(mirrorAfterExample);
record("accessibility:live", pickedRow && after.length > 0 && JSON.stringify(after) !== JSON.stringify(before) ? "pass" : pickedRow ? "fail" : "blocked", {
  gesture: pickedRow,
  before: { nodeCount: mirrorBefore?.nodeCount ?? 0, labels: before.length, sample: before.slice(0, 6) },
  after: { nodeCount: mirrorAfterExample?.nodeCount ?? 0, labels: after.length, sample: after.slice(0, 6) },
  arrived: after.filter((label) => !before.includes(label)).slice(0, 10),
  left: before.filter((label) => !after.includes(label)).slice(0, 10),
});
writeFileSync(join(outDir, "accessibility.json"), JSON.stringify({ mirror: mirrored, afterExample: mirrorAfterExample, perWindow }, null, 2));
//#endregion ⏳️StatusPill

//#region 🗣️Locale
/** 🗣️ EXACTLY ONE palette chord. `mod+p` focuses the palette's own query field, so a second chord is
 * not a toggle unless the shell looks through its own overlay fields — before that fix the second
 * chord fell through to the open palette's `Char` arm and typed a literal `p` into the query, which
 * is how an earlier run of this probe measured "the palette does not dispatch". */
const germanWords = ["Knoten", "Leitungen", "Eingang", "Ausgang", "Vorschau", "Generationen", "Formular", "Abbrechen", "Suchen", "Sprache", "Erscheinungsbild", "Bereit", "Zur Startseite"];
const germanEvidence = async (context) => {
  const trace = localeTrace();
  const texts = {};
  for (const id of await liveWindowIds()) {
    const dump = await introspect("dumpStructure", id);
    texts[id] = (dump?.nodes ?? []).map((node) => node.text).filter((text) => typeof text === "string" && text.length > 0);
  }
  const mirrorLabels = (await mirror())?.nodes?.map((node) => node.label).filter(Boolean) ?? [];
  const resolved = [...Object.values(trace?.chrome ?? {}), ...Object.values(trace?.windows ?? {}), ...Object.values(trace?.commands ?? {})];
  const found = [...new Set([...resolved, ...Object.values(texts).flat(), ...mirrorLabels])].filter((text) => germanWords.some((word) => text.includes(word)));
  return { context, localeTrace: trace, texts: Object.fromEntries(Object.entries(texts).map(([id, list]) => [id, list.slice(0, 24)])), labels: mirrorLabels.slice(0, 24), germanFound: found };
};

await page.evaluate(() => document.getElementById("semio-wgpu-canvas")?.focus({ preventScroll: true })).catch(() => {});
await pump(800);
await page.keyboard.press("Control+p");
await pump(2000);
await shot("04-palette-open");
const paletteOpened = has("wgpu-shell palette chord").at(-1) ?? null;
await page.keyboard.type("Deutsch", { delay: 80 });
await pump(1500);
await shot("05-palette-deutsch");
await page.keyboard.press("Enter");
await pump(8000);
await shot("06-german-chrome");
record("locale:palette-dispatch", has("wgpu-shell os command id=os.setLocale").length > 0 ? "pass" : "fail", {
  chord: paletteOpened,
  activate: has("wgpu-shell palette activate").at(-1) ?? null,
  osCommand: has("wgpu-shell os command").at(-1) ?? null,
});
const viaPalette = await germanEvidence("palette");
record("locale:german-via-palette", viaPalette.localeTrace?.locale === "de" && viaPalette.germanFound.length > 0 ? "pass" : "fail", {
  resolvedLocale: viaPalette.localeTrace?.locale ?? null,
  chrome: viaPalette.localeTrace?.chrome ?? null,
  windows: viaPalette.localeTrace?.windows ?? null,
  germanFound: viaPalette.germanFound.slice(0, 20),
});

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
  await pump(15000);
  await shot("07-german-boot");
  const viaBoot = await germanEvidence("de-DE browser locale");
  record("locale:german-via-boot-locale", viaBoot.germanFound.length > 0 ? "pass" : "fail", { windowIds: ids, resolvedLocale: viaBoot.localeTrace?.locale ?? null, germanFound: viaBoot.germanFound.slice(0, 20) });
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
