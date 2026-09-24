/** 🧪️ N2 — browser wgpu shell opens a SECOND, foreign-kind artifact in the same session.
 *
 * Boots an already-serving single-plugin wgpu playground (forms) in headless chromium with
 * `--use-angle=metal`, then stands in for the guest that emits `os.open-artifact` in production (the
 * `space` index editor's `openArtifact` command, which needs a hub-populated index this probe must not
 * start): the ONE stimulus is a `ReplayShellCommand { os.open-artifact }` appended to the next answer
 * the resident plugin's JS bridge hands the shell. Everything downstream is production code — the
 * Rust relay (`handle_open_artifact_relay`), the owner resolution from the generated activation table,
 * `install_plugin` → frame-Worker lazy-install door → module fetch, the `PluginInstall` progress/cancel
 * band, `switch_to_app`, and the document half. The stimulus hook is installed by rewriting ONE line of
 * the served frame-worker bundle in flight (`context.route`), so no repository file carries it.
 *
 * Evidence written to `wp-n2/generated/n2-probe-*`: console (page + worker), plugin-module network
 * requests with timestamps, `dumpStructure`/`dumpAccessibility` before and after, the DOM a11y mirror
 * text, screenshots, and a verdict. Paths via `fileURLToPath` (preamble rule 24).
 *
 * Usage: SEMIO_PROBE_URL=http://127.0.0.1:6391/?plugin=forms bun 🐍️n2-wgpu-foreign-open-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const here = dirname(fileURLToPath(import.meta.url));
const outDir = join(here, "wp-n2", "generated");
mkdirSync(outDir, { recursive: true });
const out = (name) => join(outDir, `n2-probe-${name}`);

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6391/?plugin=forms";
const bootSeconds = Number(process.env.SEMIO_PROBE_BOOT_SECONDS ?? 180);
const relayArgs = JSON.parse(process.env.SEMIO_PROBE_RELAY_ARGS ?? JSON.stringify({ artifactRef: "s.note.note@1/*", documentId: "n2-note-1", schema: "s.note" }));
const cancelKind = process.env.SEMIO_PROBE_CANCEL_KIND ?? "s.dag.dag@1/*";
const HOOK_ANCHOR = "const plugins = await mountPluginHandles(bootPlan.plugins);";

const started = Date.now();
const t = () => ((Date.now() - started) / 1000).toFixed(1);
const lines = [];
const moduleRequests = [];
const navigations = [];
const log = (line) => {
  lines.push(`${t()}s ${line}`);
};

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal", "--enable-gpu"] });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 }, deviceScaleFactor: 1 });
let hookInstalled = false;
await context.route(/frame-worker\.js(\?.*)?$/, async (route) => {
  const response = await route.fetch();
  const body = await response.text();
  if (!body.includes(HOOK_ANCHOR)) {
    log("hook: anchor missing in served frame worker");
    return route.fulfill({ response, body });
  }
  hookInstalled = true;
  return route.fulfill({ response, body: body.replace(HOOK_ANCHOR, `${HOOK_ANCHOR} globalThis.__n2ProbePlugins = plugins;`) });
});
const page = await context.newPage();
page.on("console", (message) => log(`console.${message.type()} ${message.text().slice(0, 400)}`));
page.on("pageerror", (error) => log(`pageerror ${String(error).slice(0, 400)}`));
page.on("framenavigated", (frame) => {
  if (frame === page.mainFrame()) navigations.push(`${t()}s ${frame.url()}`);
});
page.on("worker", (worker) => log(`worker ${worker.url().slice(-60)}`));
context.on("request", (request) => {
  const decoded = decodeURIComponent(request.url());
  if (decoded.includes("/🔌️plugin-modules/")) moduleRequests.push(`${t()}s ${request.method()} ${decoded.replace(/^https?:\/\/[^/]+/, "")}`);
});
context.on("requestfailed", (request) => log(`requestfailed ${request.failure()?.errorText ?? "?"} ${decodeURIComponent(request.url())}`));

await page.goto(url, { waitUntil: "domcontentloaded" });
const booted = await page
  .waitForFunction(() => typeof globalThis.semioWgpuIntrospection?.dumpStructure === "function", null, { timeout: bootSeconds * 1000 })
  .then(() => true)
  .catch(() => false);
log(`booted=${booted}`);
await page.waitForTimeout(8000);

const snapshot = async (name) => {
  const dumps = await page.evaluate(async () => {
    const introspection = globalThis.semioWgpuIntrospection;
    const read = async (kind) => {
      try {
        return (await introspection?.[kind]?.()) ?? "";
      } catch (error) {
        return `<failed: ${String(error)}>`;
      }
    };
    const mirror = document.getElementById("semio-wgpu-accessibility");
    const controls = mirror ? [...mirror.querySelectorAll("button, input, [role]")].map((node) => ({ tag: node.tagName.toLowerCase(), key: node.dataset.nodeKey ?? "", window: node.dataset.window ?? "", role: node.getAttribute("role") ?? "", label: (node.getAttribute("aria-label") ?? node.textContent ?? "").trim().slice(0, 120) })) : [];
    return { windows: mirror?.dataset.windows ?? "", structure: await read("dumpStructure"), accessibility: await read("dumpAccessibility"), chrome: await read("dumpChrome"), mirrorText: mirror?.innerText?.slice(0, 20000) ?? "", controls, timeOrigin: performance.timeOrigin };
  });
  writeFileSync(out(`${name}.json`), JSON.stringify(dumps, null, 2));
  await page.screenshot({ path: out(`${name}.png`), type: "png" });
  return dumps;
};

const before = await snapshot("before");
const worker = await (async () => {
  for (let attempt = 0; attempt < 40; attempt += 1) {
    for (const candidate of page.workers()) {
      const has = await candidate.evaluate(() => Array.isArray(globalThis.__n2ProbePlugins)).catch(() => false);
      if (has) return candidate;
    }
    await page.waitForTimeout(500);
  }
  return null;
})();
log(`hookInstalled=${hookInstalled} worker=${worker ? "found" : "missing"}`);

const arm = async (args) => worker?.evaluate((relay) => {
  const resident = globalThis.__n2ProbePlugins?.[0]?.handle;
  if (!resident) return "no resident handle";
  const stimulus = (globalThis.__n2ProbeStimulus = { fired: 0, via: [] });
  const effect = { replayShellCommand: { actionId: "os.open-artifact", args: relay } };
  for (const verb of ["handleAction", "handleCommand"]) {
    const original = resident[`__n2${verb}`] ?? resident[verb];
    resident[`__n2${verb}`] = original;
    resident[verb] = async (...call) => {
      const answer = await original(...call);
      if (stimulus.fired > 0) return answer;
      const parsed = JSON.parse(answer);
      parsed.requestedEffects = [...(parsed.requestedEffects ?? []), effect];
      stimulus.fired += 1;
      stimulus.via.push(verb);
      return JSON.stringify(parsed);
    };
  }
  return `armed ${JSON.stringify(relay)}`;
}, args);

const fire = async (label) => {
  const guestKeys = (await page.evaluate(() => {
    const mirror = document.getElementById("semio-wgpu-accessibility");
    return mirror ? [...mirror.querySelectorAll("button")].map((node, index) => ({ index, key: node.dataset.nodeKey ?? "", window: node.dataset.window ?? "", label: (node.getAttribute("aria-label") ?? node.textContent ?? "").trim() })) : [];
  })).filter((row) => row.key && !/^(framework|shell|s-|os\.|dock|navbar|footer)/.test(row.key));
  log(`${label}: ${guestKeys.length} guest-owned mirror buttons`);
  for (const row of guestKeys.slice(0, 12)) {
    await page.evaluate((index) => {
      const button = document.getElementById("semio-wgpu-accessibility")?.querySelectorAll("button")[index];
      button?.focus();
      button?.click();
    }, row.index);
    await page.waitForTimeout(1500);
    const fired = await worker.evaluate(() => globalThis.__n2ProbeStimulus?.fired ?? 0);
    log(`${label}: clicked ${row.window}/${row.key} (${row.label.slice(0, 40)}) fired=${fired}`);
    if (fired > 0) return row;
  }
  return null;
};

log(`arm open: ${await arm(relayArgs)}`);
const trigger = worker ? await fire("open") : null;
const installSeen = [];
for (let tick = 0; tick < 90; tick += 1) {
  await page.waitForTimeout(1000);
  const text = await page.evaluate(() => document.getElementById("semio-wgpu-accessibility")?.innerText ?? "");
  const structure = await page.evaluate(async () => (await globalThis.semioWgpuIntrospection?.dumpStructure?.()) ?? "");
  const phase = /Loading plugin|Plugin wird geladen|Resolving|shell\.plugin-install/.exec(text)?.[0];
  if (phase) installSeen.push(`${t()}s ${phase}`);
  if (structure.includes("s.note.note") || text.includes("note")) {
    log(`switch observed at tick ${tick}`);
    break;
  }
}
await page.waitForTimeout(4000);
const after = await snapshot("after");

let cancel = null;
if (worker && process.env.SEMIO_PROBE_SKIP_CANCEL !== "1") {
  log(`arm cancel: ${await arm({ artifactRef: cancelKind })}`);
  const cancelTrigger = await fire("cancel");
  const cancelTrace = [];
  for (let tick = 0; tick < 40 && cancelTrigger; tick += 1) {
    await page.waitForTimeout(250);
    const clicked = await page.evaluate(() => {
      const mirror = document.getElementById("semio-wgpu-accessibility");
      const button = mirror ? [...mirror.querySelectorAll("button")].find((node) => (node.dataset.nodeKey ?? "").includes("plugin-install.cancel")) : null;
      if (!button) return false;
      button.focus();
      button.click();
      return true;
    });
    const text = await page.evaluate(() => document.getElementById("semio-wgpu-accessibility")?.innerText ?? "");
    cancelTrace.push(`${t()}s clicked=${clicked} ${/Plugin load cancelled|Plugin-Laden abgebrochen|Loading plugin/.exec(text)?.[0] ?? "-"}`);
    if (clicked) break;
  }
  await page.waitForTimeout(3000);
  cancel = { trigger: cancelTrigger?.key ?? null, trace: cancelTrace, snapshot: (await snapshot("cancel")).mirrorText.match(/Plugin load cancelled|Plugin-Laden abgebrochen/)?.[0] ?? null };
}

const stimulus = worker ? await worker.evaluate(() => globalThis.__n2ProbeStimulus ?? null) : null;
writeFileSync(out("console.txt"), lines.join("\n"));
writeFileSync(out("module-requests.txt"), moduleRequests.join("\n"));
const verdict = {
  url,
  booted,
  hookInstalled,
  trigger: trigger?.key ?? null,
  stimulus,
  navigations,
  sameDocument: before.timeOrigin === after.timeOrigin,
  installSeen,
  noteModuleFetchedAfterTrigger: moduleRequests.filter((row) => row.includes("🗒️note")),
  structureBefore: before.structure.slice(0, 600),
  structureAfter: after.structure.slice(0, 600),
  noteInAccessibilityAfter: after.accessibility.includes("note") || after.mirrorText.includes("note"),
  documentNotice: /document sync is unavailable|could not be attached|Dokumentsynchronisierung|Dokument konnte/.exec(`${after.mirrorText}\n${after.chrome}`)?.[0] ?? null,
  cancel,
  errors: lines.filter((line) => /pageerror|console\.error/.test(line)).slice(0, 30),
};
writeFileSync(out("verdict.json"), JSON.stringify(verdict, null, 2));
console.log(JSON.stringify(verdict, null, 2));
await browser.close();
