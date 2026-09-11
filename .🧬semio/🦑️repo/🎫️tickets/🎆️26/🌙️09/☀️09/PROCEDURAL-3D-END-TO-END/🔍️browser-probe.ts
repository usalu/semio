/** 🔬️ Headless runtime probe for the procedural 3d playground.
 * Default target: http://127.0.0.1:6018/?plugin=generation3d
 * Ticket 26/09/09/PROCEDURAL-3D-END-TO-END. Uses the repo's Playwright (no new deps).
 *
 * bun 🔍️browser-probe.ts
 * bun 🔍️browser-probe.ts --url=http://127.0.0.1:6118/?plugin=generation3d
 * bun 🔍️browser-probe.ts --port=6018 --plugin=generation3d
 * bun 🔍️browser-probe.ts --mode=interact --steps=example,hover,select,orbit --example=box-shell-preview
 * bun 🔍️browser-probe.ts --label=boot-13-pre --settle=90
 */
import { chromium, type ConsoleMessage, type Page, type Request, type Response } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

type ConsoleRow = {
  at: string;
  elapsedMs: number;
  level: string;
  text: string;
  location?: { url: string; lineNumber: number; columnNumber: number };
  args: unknown[];
};

type NetworkFail = {
  at: string;
  elapsedMs: number;
  kind: "failed" | "http";
  url: string;
  method: string;
  status?: number;
  error?: string;
};

type PageFault = { at: string; elapsedMs: number; kind: "pageerror" | "unhandledrejection"; text: string; stack?: string };

type DomNode = { id: string; w: number; h: number; text: string };

type PreviewHost = {
  surfaceId: string | null;
  meshes: number;
  instances: number;
  status: unknown;
  interaction: unknown;
  w: number;
  h: number;
};

type Snapshot = {
  title: string;
  bodyText: string;
  windowIds: string[];
  windowTabs: { windowId: string; slot: string | null; text: string }[];
  windows: DomNode[];
  chrome: { navbar: boolean; examplePicker: boolean; title: string };
  panels: DomNode[];
  panelTabs: string[];
  canvases: { w: number; h: number; cssW: number; cssH: number }[];
  previewHosts: PreviewHost[];
  example: string;
  exampleOptions: string[];
  visibleFaults: string[];
  meshCount: number;
  instanceCount: number;
};

const TICKET = import.meta.dir;
const GENERATED = join(TICKET, "🗑️generated");
const DEFAULT_URL = "http://127.0.0.1:6018/?plugin=generation3d";
const FAULT_RE =
  /No plugins loaded|intake-budget-exhausted|native-owner-required|terminal-fault|worker fault|\[semio-plugin panic\]|SemioFaultError|plugin\.reactor-|trapped:|cabi_realloc|Unterminated string|Geometry extension unavailable|flow\.extension-not-contributed|shard 0 (lost|terminated)|did not publish|admission failed|Credits \{|NodeCapacity|section-root-mismatch|fixed-capacity/i;
const BOOT_FATAL_RE =
  /No plugins loaded|actor \S+ trapped|worker fault|\[semio-plugin panic\]|plugin\.reactor-turn-deadline|native-owner-required|cabi_realloc|Unterminated string in JSON/i;

const arg = (name: string): string | undefined => {
  const hit = process.argv.find((a) => a === `--${name}` || a.startsWith(`--${name}=`));
  if (!hit) return undefined;
  if (hit === `--${name}`) return "true";
  return hit.slice(name.length + 3);
};
const has = (name: string) => process.argv.includes(`--${name}`) || process.argv.some((a) => a.startsWith(`--${name}=`));

const positionalUrl = process.argv.slice(2).find((a) => !a.startsWith("--") && /^https?:\/\//.test(a));
const port = arg("port");
const plugin = arg("plugin") ?? "generation3d";
const url = positionalUrl ?? arg("url") ?? (port ? `http://127.0.0.1:${port}/?plugin=${plugin}` : DEFAULT_URL);
const mode = (arg("mode") ?? (has("interact") ? "interact" : "boot")).toLowerCase();
const steps = (arg("steps") ?? "example,hover,select,orbit")
  .split(",")
  .map((s) => s.trim())
  .filter(Boolean);
const example = arg("example") ?? "box-shell-preview";
const settleSec = Number(arg("settle") ?? "90");
const label = arg("label") ?? "boot-13-pre";
const headed = has("headed");
const navTimeoutMs = Number(arg("timeout") ?? "90") * 1000;

const t0 = Date.now();
const observedAt = new Date().toISOString();
const stamp = observedAt.replace(/[:.]/g, "-").slice(0, 19);
const runDir = join(GENERATED, `probe-${label}-${stamp}`);
mkdirSync(runDir, { recursive: true });
mkdirSync(GENERATED, { recursive: true });

const logLines: string[] = [];
const log = (m: string) => {
  const row = `[${((Date.now() - t0) / 1000).toFixed(1)}s ${new Date().toISOString()}] ${m}`;
  logLines.push(row);
  console.log(row);
};

const consoleRows: ConsoleRow[] = [];
const networkFails: NetworkFail[] = [];
const pageFaults: PageFault[] = [];

const serializeArgs = async (msg: ConsoleMessage): Promise<unknown[]> => {
  const out: unknown[] = [];
  for (const a of msg.args()) {
    try {
      out.push(await a.jsonValue());
    } catch {
      out.push(a.toString());
    }
  }
  return out;
};

const flushConsole = (() => {
  let pending: ConsoleRow | null = null;
  const flush = () => {
    if (pending) consoleRows.push(pending);
    pending = null;
  };
  return {
    push(row: ConsoleRow) {
      const last = pending;
      if (last && last.level === row.level && row.elapsedMs - last.elapsedMs < 12 && last.text.length < 4000) {
        const gap = last.text.endsWith(" ") || row.text.startsWith(" ") ? "" : " ";
        last.text = `${last.text}${gap}${row.text}`.slice(0, 4000);
        last.elapsedMs = row.elapsedMs;
        return;
      }
      flush();
      pending = row;
    },
    flush,
  };
})();

const attachCapture = (page: Page) => {
  page.on("console", (msg) => {
    const loc = msg.location();
    const row: ConsoleRow = {
      at: new Date().toISOString(),
      elapsedMs: Date.now() - t0,
      level: msg.type(),
      text: msg.text(),
      location: loc.url ? { url: loc.url, lineNumber: loc.lineNumber, columnNumber: loc.columnNumber } : undefined,
      args: [],
    };
    flushConsole.push(row);
    void serializeArgs(msg).then((args) => {
      row.args = args;
    });
  });
  page.on("pageerror", (err) => {
    pageFaults.push({
      at: new Date().toISOString(),
      elapsedMs: Date.now() - t0,
      kind: "pageerror",
      text: String(err),
      stack: err.stack,
    });
  });
  page.on("requestfailed", (req: Request) => {
    networkFails.push({
      at: new Date().toISOString(),
      elapsedMs: Date.now() - t0,
      kind: "failed",
      url: req.url(),
      method: req.method(),
      error: req.failure()?.errorText,
    });
  });
  page.on("response", (res: Response) => {
    if (res.status() < 400) return;
    networkFails.push({
      at: new Date().toISOString(),
      elapsedMs: Date.now() - t0,
      kind: "http",
      url: res.url(),
      method: res.request().method(),
      status: res.status(),
    });
  });
};

const snapshotScript = () => {
  const q = <T extends Element>(sel: string) => Array.from(document.querySelectorAll<T>(sel));
  const node = (el: Element): { id: string; w: number; h: number; text: string } => {
    const h = el as HTMLElement;
    return { id: h.id || h.getAttribute("data-key") || "?", w: h.offsetWidth, h: h.offsetHeight, text: (h.innerText || "").replace(/\s+/g, " ").slice(0, 160) };
  };
  const parseLen = (raw: string | null): number => {
    if (!raw) return 0;
    try {
      const v = JSON.parse(raw) as unknown;
      return Array.isArray(v) ? v.length : v && typeof v === "object" ? Object.keys(v as object).length : 0;
    } catch {
      return 0;
    }
  };
  const parseJson = (raw: string | null): unknown => {
    if (!raw) return null;
    try {
      return JSON.parse(raw);
    } catch {
      return raw.slice(0, 400);
    }
  };
  const hosts = q<HTMLElement>(".semio-world-3d-host, [data-meshes-json], [data-status-json]");
  const previewHosts = hosts.map((el) => {
    const meshes = parseLen(el.getAttribute("data-meshes-json"));
    const instances = parseLen(el.getAttribute("data-instances-json"));
    return {
      surfaceId: el.getAttribute("data-surface-id"),
      meshes,
      instances,
      status: parseJson(el.getAttribute("data-status-json")),
      interaction: parseJson(el.getAttribute("data-interaction-json")),
      w: el.offsetWidth,
      h: el.offsetHeight,
    };
  });
  const body = (document.body?.innerText || "").replace(/\s+/g, " ");
  const faultHits = body.match(
    /(?:No plugins loaded|intake-budget-exhausted[^\s]{0,80}|Geometry extension unavailable|Geometrie-Erweiterung nicht verfügbar|Unterminated string[^\s]{0,40}|plugin-ui\.[^\s]{0,80}|native-owner-required[^\s]{0,40}|phase:\s*faulted[^|]{0,80}|worker-boot-failed[^\s]{0,80})/gi,
  );
  const canvases = q("canvas").map((c) => {
    const el = c as HTMLCanvasElement;
    return { w: el.width, h: el.height, cssW: el.clientWidth, cssH: el.clientHeight };
  });
  return {
    title: document.title,
    bodyText: body.slice(0, 1200),
    windowIds: q("[id^='framework.window.']").map((e) => e.id),
    windowTabs: q("[data-window-id]").map((e) => ({
      windowId: e.getAttribute("data-window-id") || "?",
      slot: e.getAttribute("data-slot"),
      text: ((e as HTMLElement).innerText || "").replace(/\s+/g, " ").slice(0, 80),
    })),
    windows: q("[id^='framework.window.'], [data-slot='window']").map(node),
    chrome: {
      navbar: Boolean(document.querySelector('[data-slot="navbar"], #ui.navbar')),
      examplePicker: Boolean(document.querySelector('[role="combobox"], [data-slot="select-trigger"]')),
      title: document.title,
    },
    panels: q("[id^='framework.panel.']").filter((e) => !e.id.startsWith("framework.panelTab.")).map(node),
    panelTabs: q("[id^='framework.panelTab.']").map((e) => e.id).slice(0, 40),
    canvases,
    previewHosts,
    example: ((document.querySelector('[role="combobox"]') as HTMLElement | null)?.innerText || (document.querySelector("select") as HTMLSelectElement | null)?.selectedOptions[0]?.text || "").replace(/\s+/g, " ").slice(0, 80),
    exampleOptions: q('[role="option"]').map((e) => (e as HTMLElement).innerText.replace(/\s+/g, " ").slice(0, 80)),
    visibleFaults: [...new Set(faultHits ?? [])].slice(0, 20),
    meshCount: previewHosts.reduce((n, h) => n + h.meshes, 0),
    instanceCount: previewHosts.reduce((n, h) => n + h.instances, 0),
  };
};

const takeSnapshot = (page: Page) => page.evaluate(snapshotScript);

const dismissIntro = async (page: Page) => {
  const skip = page.locator("button", { hasText: /^\s*(x\s*)?skip\s*$/i }).first();
  if (await skip.count()) {
    await skip.click({ timeout: 2000 }).catch(() => {});
    log("dismissed skip/intro");
  }
};

const writeJson = (name: string, value: unknown) => {
  const path = join(runDir, name);
  writeFileSync(path, JSON.stringify(value, null, 2));
  return path;
};

const diagnosisOf = (snap: Snapshot | null, navError?: string): { ok: boolean; line: string; fatal: boolean } => {
  if (navError) return { ok: false, fatal: true, line: `serve/navigation failed: ${navError}` };
  if (!snap) return { ok: false, fatal: true, line: "no snapshot after settle" };
  const consoleFatal = [...consoleRows.map((r) => r.text), ...pageFaults.map((f) => f.text)].find((t) => BOOT_FATAL_RE.test(t));
  const bodyFatal = /No plugins loaded/i.test(snap.bodyText) || snap.visibleFaults.some((f) => /No plugins loaded|trapped|Unterminated string/i.test(f));
  const bodies = snap.windows.length > 0 || snap.windowIds.length > 0 || snap.canvases.length > 0;
  const tabs = [...new Set(snap.windowTabs.map((t) => t.windowId))];
  const chrome = snap.chrome.navbar || tabs.length > 0 || /procedural/i.test(snap.title);
  if (bodyFatal) return { ok: false, fatal: true, line: `boot fault: ${snap.visibleFaults[0] || "No plugins loaded"}` };
  if (consoleFatal && !bodies) return { ok: false, fatal: true, line: `boot fault: ${consoleFatal.slice(0, 220)}` };
  if (!bodies && chrome) return { ok: false, fatal: true, line: `boot fault: chrome rendered but window bodies empty (tabs=${tabs.join(",") || "none"}; title=${JSON.stringify(snap.title)})` };
  if (!bodies) return { ok: false, fatal: true, line: `boot fault: shell did not render (title=${JSON.stringify(snap.title)} windows=0 canvases=${snap.canvases.length})` };
  if (consoleFatal) return { ok: false, fatal: true, line: `boot fault after chrome: ${consoleFatal.slice(0, 220)}` };
  const previewFault = snap.visibleFaults[0] || (typeof snap.previewHosts[0]?.status === "object" && snap.previewHosts[0]?.status && /fault/i.test(JSON.stringify(snap.previewHosts[0].status)) ? JSON.stringify(snap.previewHosts[0].status).slice(0, 180) : "");
  const extra = previewFault ? `; preview: ${previewFault}` : snap.meshCount === 0 ? "; preview meshes=0" : "";
  return { ok: true, fatal: false, line: `shell rendered; windows=${snap.windowIds.join(",") || tabs.join(",")}; meshes=${snap.meshCount}${extra}` };
};

const captureSettleArtifacts = async (page: Page, name: string, snap: Snapshot) => {
  const png = join(runDir, `${name}.png`);
  await page.screenshot({ path: png, fullPage: false }).catch((e) => log(`screenshot ${name} failed: ${e}`));
  const html = await page.evaluate(() => document.documentElement.outerHTML).catch(() => "");
  const htmlPath = join(runDir, `${name}.dom.html`);
  writeFileSync(htmlPath, html);
  writeJson(`${name}.snapshot.json`, snap);
  return { png, html: htmlPath };
};

const waitSettle = async (page: Page): Promise<Snapshot> => {
  const deadline = Date.now() + settleSec * 1000;
  let last: Snapshot = await takeSnapshot(page);
  let i = 0;
  while (Date.now() < deadline) {
    await dismissIntro(page);
    last = await takeSnapshot(page);
    const bodies = last.windows.length > 0 || last.windowIds.length > 0 || last.canvases.length > 0;
    const tabs = [...new Set(last.windowTabs.map((t) => t.windowId))];
    const fatal = /No plugins loaded/i.test(last.bodyText);
    if (i === 0 || i % 4 === 0) log(`waiting… title=${JSON.stringify(last.title)} tabs=${tabs.join(",") || "0"} bodies=${last.windowIds.length} canvases=${last.canvases.length} meshes=${last.meshCount} faults=${last.visibleFaults.length}`);
    if (fatal && Date.now() - t0 > 8000) {
      log("visible 'No plugins loaded' after 8s — treating as settled fault");
      break;
    }
    if (bodies && Date.now() - t0 > 6000) {
      await page.waitForTimeout(2500);
      last = await takeSnapshot(page);
      log(`settled: windows=${JSON.stringify(last.windowIds)} canvases=${last.canvases.length} meshes=${last.meshCount}`);
      break;
    }
    await page.waitForTimeout(2000);
    i += 1;
  }
  return last;
};

const previewCanvas = (page: Page) => {
  const host = page.locator(".semio-world-3d-host canvas, [data-meshes-json] canvas, #framework\\.window\\.proceduralPreview canvas").last();
  return host;
};

const runStep = async (page: Page, name: string, fn: () => Promise<void>) => {
  const before = consoleRows.length;
  log(`step ${name}: start`);
  try {
    await fn();
    await page.waitForTimeout(2500);
    const snap = await takeSnapshot(page);
    const arts = await captureSettleArtifacts(page, `step-${name}`, snap);
    log(`step ${name}: ok windows=${snap.windowIds.length} meshes=${snap.meshCount} example=${JSON.stringify(snap.example)} newConsole=${consoleRows.length - before} png=${arts.png}`);
    return snap;
  } catch (e) {
    log(`step ${name}: FAILED ${String(e).slice(0, 300)}`);
    const snap = await takeSnapshot(page).catch(() => null);
    if (snap) await captureSettleArtifacts(page, `step-${name}`, snap);
    return snap;
  }
};

const interact = async (page: Page) => {
  if (steps.includes("example")) {
    await runStep(page, "example", async () => {
      const select = page.locator("select").first();
      if (await select.count()) {
        const opts = await select.locator("option").allTextContents();
        log(`example <select> options=${JSON.stringify(opts).slice(0, 300)}`);
        const byValue = await select.locator(`option[value="${example}"]`).count();
        if (byValue) await select.selectOption(example);
        else {
          const byText = await select.locator("option", { hasText: new RegExp(example.replace(/-/g, "[- ]"), "i") }).first().getAttribute("value");
          if (byText) await select.selectOption(byText);
          else await select.selectOption({ index: Math.min(1, Math.max(0, opts.length - 1)) });
        }
        return;
      }
      const combo = page.locator('[role="combobox"]').first();
      if (!(await combo.count())) throw new Error("no example picker (select or combobox)");
      await combo.click({ timeout: 4000 });
      await page.waitForTimeout(400);
      const options = page.locator('[role="option"]');
      const n = await options.count();
      log(`example combobox options=${n}`);
      const target = options.filter({ hasText: new RegExp(example.replace(/-/g, "[- ]"), "i") }).first();
      if (await target.count()) await target.click({ timeout: 4000 });
      else if (n > 1) await options.nth(1).click({ timeout: 4000 });
      else throw new Error("example picker opened but had no options");
      await page.waitForTimeout(Math.min(settleSec, 20) * 1000);
    });
  }
  if (steps.includes("hover")) {
    await runStep(page, "hover", async () => {
      const c = previewCanvas(page);
      if (!(await c.count())) throw new Error("no preview canvas for hover");
      const box = await c.boundingBox();
      if (!box) throw new Error("preview canvas has no box");
      await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
      await page.waitForTimeout(800);
    });
  }
  if (steps.includes("select")) {
    await runStep(page, "select", async () => {
      const c = previewCanvas(page);
      if (!(await c.count())) throw new Error("no preview canvas for select");
      await c.click({ position: { x: 180, y: 180 }, timeout: 5000 });
    });
  }
  if (steps.includes("orbit")) {
    await runStep(page, "orbit", async () => {
      const c = previewCanvas(page);
      if (!(await c.count())) throw new Error("no preview canvas for orbit");
      const box = await c.boundingBox();
      if (!box) throw new Error("preview canvas has no box");
      const x = box.x + box.width * 0.6;
      const y = box.y + box.height * 0.45;
      await page.mouse.move(x, y);
      await page.mouse.down();
      await page.mouse.move(x + 80, y + 30, { steps: 12 });
      await page.mouse.up();
    });
  }
};

let exitCode = 0;
let diagnosis = "probe did not finish";
let snap: Snapshot | null = null;
try {
  log(`start label=${label} mode=${mode} url=${url} settle=${settleSec}s headed=${headed}`);
  const browser = await chromium.launch({
    headless: !headed,
    args: [
      "--enable-unsafe-webgpu",
      "--enable-webgl",
      "--ignore-gpu-blocklist",
      "--disable-gpu-sandbox",
      "--use-angle=swiftshader",
      "--enable-unsafe-swiftshader",
      "--enable-features=Vulkan,UseSkiaRenderer",
    ],
  });
  const context = await browser.newContext({
    viewport: { width: 1440, height: 900 },
    ignoreHTTPSErrors: true,
  });
  const page = await context.newPage();
  attachCapture(page);
  await page.addInitScript(() => {
    window.addEventListener("unhandledrejection", (ev) => {
      const reason = ev.reason;
      const text = reason instanceof Error ? `${reason.name}: ${reason.message}` : String(reason);
      const stack = reason instanceof Error ? reason.stack : undefined;
      console.error(`[probe-unhandledrejection] ${text}${stack ? `\n${stack}` : ""}`);
    });
  });
  let navError: string | undefined;
  try {
    const res = await page.goto(url, { waitUntil: "domcontentloaded", timeout: navTimeoutMs });
    log(`navigated status=${res?.status() ?? "?"} title=${JSON.stringify(await page.title())}`);
    if (!res || res.status() >= 400) navError = `HTTP ${res?.status() ?? "no-response"}`;
  } catch (e) {
    navError = String(e).slice(0, 240);
    log(`navigation failed: ${navError}`);
  }
  if (!navError) snap = await waitSettle(page);
  if (snap) await captureSettleArtifacts(page, "settle", snap);
  if (snap && mode === "interact" && !navError) await interact(page);
  else if (snap && mode === "interact") log("skip interact: navigation failed");
  const finalSnap = snap ? await takeSnapshot(page).catch(() => snap) : snap;
  if (finalSnap) snap = finalSnap;
  const d = diagnosisOf(snap, navError);
  diagnosis = d.line;
  exitCode = d.fatal ? (navError && /net::|Timeout|ECONNREFUSED/i.test(navError) ? 2 : 1) : 0;
  await browser.close();
} catch (e) {
  diagnosis = `probe internal error: ${String(e).slice(0, 240)}`;
  exitCode = 3;
  log(diagnosis);
}

const faultTexts = [
  ...pageFaults.map((f) => f.text),
  ...consoleRows.filter((r) => FAULT_RE.test(r.text)).map((r) => r.text),
  ...(snap?.visibleFaults ?? []),
].filter((t, i, a) => t && a.indexOf(t) === i);

const summary = {
  schema: "semio.procedural3d.browser-probe/1",
  observedAt,
  wallClockLocal: new Date().toString(),
  label,
  restageNote: "baseline for boot #13; correlate wallClock against coordinator restage. Label boot-13-pre unless restage already swapped wasm.",
  url,
  mode,
  steps: mode === "interact" ? steps : [],
  example,
  settleSec,
  elapsedMs: Date.now() - t0,
  ok: exitCode === 0,
  fatal: exitCode !== 0,
  diagnosis,
  title: snap?.title ?? "",
  shellRendered: Boolean(snap && (snap.windows.length > 0 || snap.windowIds.length > 0 || (snap.canvases?.length ?? 0) > 0)),
  chromeRendered: Boolean(snap && (snap.chrome?.navbar || (snap.windowTabs?.length ?? 0) > 0)),
  windows: snap?.windows ?? [],
  windowIds: snap?.windowIds ?? [],
  windowTabs: snap?.windowTabs ?? [],
  chrome: snap?.chrome ?? null,
  panels: snap?.panels ?? [],
  panelTabs: snap?.panelTabs ?? [],
  canvases: snap?.canvases ?? [],
  meshCount: snap?.meshCount ?? 0,
  instanceCount: snap?.instanceCount ?? 0,
  previewHosts: snap?.previewHosts ?? [],
  exampleLabel: snap?.example ?? "",
  visibleFaults: snap?.visibleFaults ?? [],
  console: {
    total: consoleRows.length,
    errors: consoleRows.filter((r) => r.level === "error").length,
    warnings: consoleRows.filter((r) => r.level === "warning").length,
    faultLike: consoleRows.filter((r) => FAULT_RE.test(r.text)).length,
  },
  pageFaults: pageFaults.length,
  networkFailures: networkFails.length,
  artifacts: {
    dir: runDir,
    console: "console.jsonl",
    network: "network.jsonl",
    pageFaults: "page-faults.jsonl",
    summary: "summary.json",
    settlePng: "settle.png",
    settleDom: "settle.dom.html",
    log: "probe.log",
  },
  faultTexts: faultTexts.slice(0, 40),
  consoleExcerpt: consoleRows
    .filter((r) => r.level === "error" || FAULT_RE.test(r.text) || /\[DEBUG\]|PluginRuntime|hot-swap|trapped|thunk failed|action failed/i.test(r.text))
    .slice(0, 80)
    .map((r) => `${r.level}: ${r.text.slice(0, 400)}`),
};

flushConsole.flush();
writeFileSync(join(runDir, "console.jsonl"), consoleRows.map((r) => JSON.stringify(r)).join("\n") + (consoleRows.length ? "\n" : ""));
writeFileSync(join(runDir, "network.jsonl"), networkFails.map((r) => JSON.stringify(r)).join("\n") + (networkFails.length ? "\n" : ""));
writeFileSync(join(runDir, "page-faults.jsonl"), pageFaults.map((r) => JSON.stringify(r)).join("\n") + (pageFaults.length ? "\n" : ""));
writeFileSync(join(runDir, "probe.log"), logLines.join("\n") + "\n");
writeJson("summary.json", summary);

const baselinePrefix = join(GENERATED, label);
writeFileSync(`${baselinePrefix}-summary.json`, JSON.stringify(summary, null, 2));
writeFileSync(`${baselinePrefix}-console.jsonl`, consoleRows.map((r) => JSON.stringify(r)).join("\n") + (consoleRows.length ? "\n" : ""));
writeFileSync(`${baselinePrefix}-diagnosis.txt`, `${observedAt}\n${diagnosis}\n`);
writeFileSync(`${baselinePrefix}-probe.log`, logLines.join("\n") + "\n");
try {
  const { copyFileSync, existsSync } = await import("node:fs");
  const png = join(runDir, "settle.png");
  const html = join(runDir, "settle.dom.html");
  if (existsSync(png)) copyFileSync(png, `${baselinePrefix}-settle.png`);
  if (existsSync(html)) copyFileSync(html, `${baselinePrefix}-settle.dom.html`);
} catch {
  /* keep going */
}

log(`wrote ${runDir}`);
console.log(`DIAGNOSIS: ${diagnosis}`);
process.exit(exitCode);
