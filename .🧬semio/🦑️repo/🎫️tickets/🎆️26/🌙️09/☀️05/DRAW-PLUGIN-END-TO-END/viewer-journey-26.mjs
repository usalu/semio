/** 👁️ Drives the localized Viewer role and checks actual camera and component evidence. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { join, resolve } from "node:path";
import { setTimeout as delay } from "node:timers/promises";
import { hasZoomed, paintExtent } from "./viewer-paint-26.mjs";

const [origin, locale, output, pinned, coreIdentity] = process.argv.slice(2);
const labels = { en: { viewer: "Viewer", catalogue: "Catalogue" }, de: { viewer: "Betrachter", catalogue: "Katalog" } }[locale];
assert(labels && output && /^[0-9a-f]{64}$/.test(pinned ?? ""), "Explicit locale, fresh output and component pin required");
const expected = JSON.parse(coreIdentity ?? "[]");
assert(expected.length === 3 && new Set(expected.map(core => core.name)).size === 3, "Expected three pinned core identities");
assert(join(resolve(output), "report.json").length <= 256, "Viewer output path exceeds 256 characters");
await mkdir(output);
const fixtureDir = resolve("✏️s/🔌️plugins/🖍️draw/🧪️tests/🌐️browser");
const require = createRequire(join(fixtureDir, "🟦️.mjs"));
const { chromium } = require("playwright");
const fixture = JSON.parse(await readFile(join(fixtureDir, "🔣️.json"), "utf8"));
const url = new URL(origin);
url.searchParams.set("plugin", "draw");
url.searchParams.set("lang", locale);
const report = { componentSha256: pinned, locale, url: url.href, errors: [], cores: [], steps: [], result: "failed" };
const responses = new Map();
const check = (step, evidence) => { report.steps.push({ step, evidence }); console.log(`[DEBUG] Viewer ${locale} ${step}`); };
const cancellation = new AbortController();
let browser;
const stop = () => { cancellation.abort(); void browser?.close().catch(() => undefined); };
process.once("SIGTERM", stop);
process.once("SIGINT", stop);
try {
  browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
  const context = await browser.newContext({ locale, viewport: { width: 1600, height: 1000 }, serviceWorkers: "block" });
  const page = await context.newPage();
  await page.routeWebSocket(address => address.host === url.host && address.searchParams.has("token"), () => undefined);
  page.on("pageerror", error => report.errors.push(String(error)));
  page.on("console", message => {
    const text = message.text();
    if (message.type() === "error" && !text.startsWith("[DEBUG] ") || text.includes("refused: dispatch-failed")) report.errors.push(text);
  });
  await page.addInitScript(faults => {
    const NativeWorker = globalThis.Worker;
    globalThis.Worker = class extends NativeWorker {
      constructor(...args) {
        super(...args);
        this.addEventListener("message", event => {
          if (faults.includes(event.data?.kind)) console.error("Draw guest fault", JSON.stringify(event.data));
        });
        this.addEventListener("error", event => console.error("Draw worker error", event.message));
        this.addEventListener("messageerror", () => console.error("Draw worker message error"));
      }
    };
  }, fixture.guestFaults);
  context.on("response", response => {
    const address = new URL(response.url());
    const core = expected.find(candidate => new URL(candidate.url).pathname === address.pathname && address.origin === url.origin);
    if (core) responses.set(address.href, { core, status: response.status(), fromServiceWorker: response.fromServiceWorker() });
  });
  await page.goto(url.href, { waitUntil: "domcontentloaded", timeout: 60000 });
  await page.waitForFunction(() => document.documentElement.dataset.semioOsReady || document.documentElement.dataset.semioOsError, undefined, { timeout: 90000 });
  assert.equal(await page.locator("html").getAttribute("data-semio-os-ready"), fixture.ready);
  const control = (role, name) => page.getByRole("button", { name: new RegExp(`^${name}`) }).and(page.locator(`[id="playground.navbar.roles.${role}"]`));
  const editor = control("editor", "Editor"), viewer = control("viewer", labels.viewer);
  const surface = role => page.locator(`[data-surface-role="${role}"]:visible`).filter({ has: page.locator("canvas:visible") });
  const active = async (role, button) => {
    const area = surface(role);
    await area.waitFor({ state: "visible", timeout: 60000 });
    const focused = button.and(page.locator('[aria-pressed="true"]'));
    await focused.waitFor({ state: "visible", timeout: 30000 });
    assert.equal(await area.count(), 1, `Expected one active ${role} canvas surface`);
    return area;
  };
  const scope = area => area.evaluate(element => ({ space: element.getAttribute("data-surface-space-id"), document: element.getAttribute("data-surface-document-id") }));
  const throwFault = () => { cancellation.signal.throwIfAborted(); assert.deepEqual(report.errors, []); };
  const painted = async (area, matches, label, stableSamples = 1) => {
    const deadline = Date.now() + 30000;
    let last, previous, stable = 0;
    do {
      throwFault();
      const bitmap = await area.locator("canvas:visible").first().screenshot({ animations: "disabled", timeout: 10000 });
      last = await paintExtent(bitmap, fixture.bootPaint);
      const evidence = JSON.stringify(last);
      stable = matches(last) ? (previous === evidence ? stable + 1 : 1) : 0;
      if (stable >= stableSamples) return last;
      previous = evidence;
      await delay(250, undefined, { signal: cancellation.signal });
    } while (Date.now() < deadline);
    throw Error(`${label}: ${JSON.stringify(last)}`);
  };
  const initial = await active("editor", editor);
  const editorScope = await scope(initial);
  const fixtureControl = page.locator('[id="playground.navbar.fixture"]');
  await fixtureControl.getByText(fixture.example.label, { exact: true }).waitFor({ state: "visible", timeout: 60000 });
  const initialPaint = await painted(initial, frame => frame.pixels >= fixture.bootPaint.minimumPixels, "Initial Editor demo was not painted");
  check("initial-editor", { scope: editorScope, paint: initialPaint });
  await viewer.click({ timeout: 30000 });
  const area = await active("viewer", viewer);
  await surface("editor").waitFor({ state: "hidden", timeout: 30000 });
  const viewerScope = await scope(area);
  assert.deepEqual(viewerScope, editorScope, "Viewer must keep the same document and space");
  const before = await painted(area, frame => frame.pixels >= fixture.bootPaint.minimumPixels, "Viewer demo did not settle", 3);
  await page.screenshot({ path: join(output, "viewer.png"), animations: "disabled" });
  check("viewer-visible", { roleLabel: labels.viewer, scope: viewerScope, paint: before });
  const canvas = await area.locator("canvas:visible").first().boundingBox();
  assert(canvas && canvas.width > 100 && canvas.height > 100, "Interactive Viewer canvas required");
  await page.mouse.move(canvas.x + canvas.width / 2, canvas.y + canvas.height / 2);
  await page.mouse.wheel(0, -500);
  const after = await painted(area, frame => hasZoomed(before, frame), "Wheel did not enlarge the fixture in the same canvas", 2);
  await page.screenshot({ path: join(output, "zoom.png"), animations: "disabled" });
  check("viewer-wheel-zoom", { before, after });
  await editor.click({ timeout: 30000 });
  const restored = await active("editor", editor);
  await surface("viewer").waitFor({ state: "hidden", timeout: 30000 });
  await fixtureControl.getByText(fixture.example.label, { exact: true }).waitFor({ state: "visible", timeout: 30000 });
  await page.locator('[id="framework.panel.catalogue"]').filter({ hasText: labels.catalogue }).first().waitFor({ state: "visible", timeout: 30000 });
  assert.deepEqual(await scope(restored), editorScope, "Editor must restore the original document and space");
  const returnedPaint = await painted(restored, frame => frame.pixels >= fixture.bootPaint.minimumPixels, "Returned Editor demo was not painted");
  await page.screenshot({ path: join(output, "editor.png"), animations: "disabled" });
  check("editor-restored", { scope: editorScope, paint: returnedPaint });
  assert.equal(new Set([...responses.values()].map(response => response.core.name)).size, expected.length, "The Viewer journey must request every pinned Draw core");
  for (const [requestUrl, { core, status, fromServiceWorker }] of responses) {
    assert.equal(status, 200, requestUrl);
    assert.equal(fromServiceWorker, false, requestUrl);
    const exact = await fetch(requestUrl, { cache: "no-store", signal: AbortSignal.any([cancellation.signal, AbortSignal.timeout(45000)]) });
    assert(exact.ok && exact.body, `Failed to verify exact browser core URL ${requestUrl}`);
    const hash = createHash("sha256");
    let bytes = 0;
    for await (const chunk of exact.body) { hash.update(chunk); bytes += chunk.byteLength; }
    const sha256 = hash.digest("hex");
    assert.equal(sha256, core.sha256, requestUrl);
    assert.equal(bytes, core.bytes, requestUrl);
    report.cores.push({ name: core.name, requestUrl, status, bytes, sha256 });
  }
  throwFault();
  assert.equal(await page.locator("html").getAttribute("data-semio-os-error"), null);
  report.result = "passed";
} catch (error) {
  report.failure = String(error);
  throw error;
} finally {
  process.off("SIGTERM", stop);
  process.off("SIGINT", stop);
  await browser?.close().catch(() => undefined);
  await writeFile(join(output, "report.json"), JSON.stringify(report, null, 2) + "\n");
}
