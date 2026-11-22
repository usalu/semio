import assert from "node:assert/strict";
import { chromium } from "playwright";
import { mkdir, mkdtemp, readFile, rm } from "node:fs/promises";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { maximumTerrainBrowserRaster } from "./terrain-browser-fixtures.mjs";

const cliArgs = process.argv.slice(2);
const requested = new URL(cliArgs.find((argument) => !argument.startsWith("--")) ?? "http://127.0.0.1:6070/");
assert.equal(requested.protocol, "http:");
assert(["127.0.0.1", "localhost", "[::1]"].includes(requested.hostname));
assert(!requested.username && !requested.password, "Supply a loopback URL without credentials");
assert(!requested.search && !requested.hash, "Supply the full s landing or Studio URL");
assert(requested.pathname === "/" || /^\/spaces\/[^/]+(?:\/studio)?$/.test(requested.pathname));
const landing = new URL("/", requested);
const existingSpace = requested.pathname === "/" ? null : requested;
const brokerPath = "/_semio/dev/local-session";
const homeRoleQuery = '[data-slot="surface-role-chip"][data-app-id="s.space.home@1/*#editor"]';

const health = await fetch(landing, { signal: AbortSignal.timeout(5_000) }).catch((error) => {
  throw new Error(`Full s shell unavailable at ${landing.href} before browser launch`, { cause: error });
});
assert(health.ok, `Full s shell returned HTTP ${health.status}`);
await health.body?.cancel();

const generated = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
await mkdir(generated, { recursive: true });
const profile = await mkdtemp(join(generated, "s-shell-terrain-"));
const editorId = "s.gis.gisterrain@1/*#editor";
const viewerId = "s.gis.gisterrain@1/*#viewer";
const shellQuery = "[data-semio-os-ready][data-history-json]";
const graphQuery = '[data-surface-id="s.play.workflow"][data-host-snapshot-json]';
const chipQuery = '[data-slot="surface-role-chip"]';
const surfaceQuery = (id, role) => `[data-surface-app-id="${id}"][data-surface-role="${role}"]`;
const modifier = process.platform === "darwin" ? "Meta" : "Control";
const observed = { completed: [] };
const errors = [];
let stage = "launch the browser";
let browser;
let page;
let createdSpace;

function complete(nextStage) {
  observed.completed.push(stage);
  stage = nextStage;
}

async function drag(source, x, y) {
  await source.scrollIntoViewIfNeeded();
  const bounds = await source.boundingBox();
  assert(bounds, "The Catalogue Editor row must be visible");
  await page.mouse.move(bounds.x + bounds.width / 2, bounds.y + bounds.height / 2);
  await page.mouse.down();
  try {
    await page.mouse.move(x, y, { steps: 20 });
    await page.mouse.move(x + 1, y + 1);
  } finally {
    await page.mouse.up();
  }
}

try {
  browser = await chromium.launchPersistentContext(profile, {
    channel: "chrome",
    headless: true,
    locale: "en-US",
    viewport: { width: 1440, height: 900 },
    downloadsPath: generated,
    args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader"],
  });
  page = browser.pages()[0] ?? await browser.newPage();
  page.setDefaultTimeout(20_000);
  page.on("pageerror", (error) => errors.push(error.message));
  complete("sign in through the local Dev broker");
  const offered = page.waitForResponse((response) => {
    const url = new URL(response.url());
    return url.origin === landing.origin && url.pathname === brokerPath && response.request().method() === "GET";
  }, { timeout: 90_000 });
  const verified = browser.waitForEvent("response", {
    predicate: (response) => {
      const url = new URL(response.url());
      return url.origin === landing.origin && url.pathname === "/_semio/hub/auth/sessions/me"
        && response.request().method() === "GET" && response.ok();
    },
    timeout: 90_000,
  });
  void verified.catch(() => {});
  const [response, broker] = await Promise.all([page.goto(landing.href, { waitUntil: "domcontentloaded", timeout: 50_000 }), offered]);
  assert(response?.ok(), `s landing returned HTTP ${response?.status() ?? "unknown"}`);
  assert(broker.ok(), `The local Dev session broker returned HTTP ${broker.status()}`);
  assert.equal((await broker.json()).offered, true, "The running s Dev row has no first-party local broker session");
  await verified;
  const shell = page.locator(shellQuery);
  await shell.waitFor({ timeout: 90_000 });
  await shell.locator(homeRoleQuery).waitFor();
  let space = existingSpace;
  if (space === null) {
    complete("create a disposable private Atelier on Home");
    const name = "Terrain E2E " + crypto.randomUUID();
    await shell.locator("#s-home-create-space").click();
    const dialog = page.getByRole("dialog", { name: "Create Space", exact: true });
    await dialog.getByRole("textbox", { name: "Name", exact: true }).fill(name);
    await dialog.getByRole("button", { name: "Create", exact: true }).click();
    const row = shell.locator('tr[data-row-id^="space:"]').filter({ has: page.getByText(name, { exact: true }) });
    await row.waitFor({ timeout: 90_000 });
    const rowId = await row.getAttribute("data-row-id");
    assert(/^space:[A-Za-z0-9][A-Za-z0-9._-]{0,255}$/u.test(rowId ?? ""), "Home publishes the newly created Space ID");
    const spaceId = rowId.slice("space:".length);
    createdSpace = { rowId, name };
    space = new URL(`/spaces/${encodeURIComponent(spaceId)}/studio`, landing);
  }
  const isStudio = space.pathname.endsWith("/studio");
  const spaceId = decodeURIComponent(space.pathname.split("/")[2]);
  assert(/^[A-Za-z0-9][A-Za-z0-9._-]{0,255}$/u.test(spaceId), "Supply a canonical Space identifier");
  observed.spaceId = spaceId;
  complete(isStudio ? "mount the persisted Studio Workflow" : "open the Space Index");
  await page.goto(space.href, { waitUntil: "domcontentloaded", timeout: 50_000 });
  let reopenDocument;
  if (isStudio) {
    const studio = shell.locator(`${surfaceQuery("s.space.studio@1/*#editor", "editor")}[data-surface-space-id="${spaceId}"][data-surface-document-id]`).filter({ has: page.locator(graphQuery) });
    await studio.waitFor({ timeout: 180_000 });
    observed.workflowDocument = await studio.getAttribute("data-surface-document-id");
    assert(observed.workflowDocument, "Studio is attached to a persisted Workflow");
    const graph = studio.locator(graphQuery);
    const workflow = graph.locator('[data-gesture-surface="flow"]');
    await workflow.waitFor({ timeout: 90_000 });
    const snapshot = async () => JSON.parse(await graph.getAttribute("data-host-snapshot-json"));
    const initial = await snapshot();
    assert(Array.isArray(initial.widgets));
    complete("expand Catalogue → semio → gis → 3d");
    const row = (key) => shell.locator(`[data-slot="tree-item-row"][id="s-play-catalogue.document.${key}"]`);
    const catalogue = shell.getByRole("tab", { name: "Catalogue", exact: true });
    await catalogue.waitFor();
    if (await catalogue.getAttribute("aria-selected") !== "true") await catalogue.click();
    for (const group of ["semio", "semio.gis", "semio.gis.3d"]) {
      const branch = row(group);
      await branch.waitFor();
      if (await branch.getAttribute("aria-expanded") !== "true") await branch.click();
    }
    const editorRow = row(`semio.gis.3d.gis/${editorId}`);
    const viewerRow = row(`semio.gis.3d.gis/${viewerId}`);
    await editorRow.getByText("Editor", { exact: true }).waitFor();
    await viewerRow.getByText("Viewer", { exact: true }).waitFor();
    for (const application of [editorRow, viewerRow]) assert.equal(await application.getAttribute("data-draggable"), "true");
    complete("drag the registered Editor into Workflow");
    const canvas = await workflow.boundingBox();
    assert(canvas, "Workflow canvas must be visible");
    await drag(editorRow, canvas.x + canvas.width * 0.65, canvas.y + canvas.height * 0.42);
    const createdHandle = await page.waitForFunction(({ selector, existingIds, appId }) => {
      const raw = document.querySelector(selector)?.getAttribute("data-host-snapshot-json");
      if (!raw) return false;
      const state = JSON.parse(raw);
      return state.widgets.find((widget) => widget.params?.pluginId === "gis" && widget.params.appId === appId && !existingIds.includes(widget.id)) ?? false;
    }, { selector: graphQuery, existingIds: initial.widgets.map(({ id }) => id), appId: editorId }, { timeout: 180_000 });
    const created = await createdHandle.jsonValue();
    await createdHandle.dispose();
    assert.equal(created.params.instanceId, created.id, "Workflow publishes the exact instance ID for node activation");
    const openWorkflowNode = async (instanceId) => {
      const state = await snapshot();
      const node = state.layout[instanceId];
      assert(node && state.camera, "Workflow publishes the position of the Terrain node");
      const view = await workflow.boundingBox();
      assert(view);
      const { x, y, zoom } = state.camera;
      const center = { x: (node.x - x) * zoom + view.width / 2, y: (node.y - y) * zoom + view.height / 2 };
      await workflow.hover({ position: center });
      await workflow.dblclick({ position: center });
    };
    observed.workflowNode = created.id;
    complete("open the dropped Terrain Editor");
    await openWorkflowNode(created.id);
    reopenDocument = async () => {
      await workflow.waitFor({ state: "visible", timeout: 90_000 });
      assert.equal(await studio.getAttribute("data-surface-document-id"), observed.workflowDocument, "Studio remounts the same Workflow");
      await page.waitForFunction(({ query, instanceId }) => {
        const raw = document.querySelector(query)?.getAttribute("data-host-snapshot-json");
        return raw && JSON.parse(raw).widgets.some((widget) => widget.id === instanceId);
      }, { query: graphQuery, instanceId: created.id }, { timeout: 90_000 });
      await openWorkflowNode(created.id);
    };
  } else {
    complete("create GIS Terrain from the Space Index");
    const name = "Terrain E2E " + crypto.randomUUID();
    const create = shell.locator("#s-space-create-artifact");
    await create.waitFor({ state: "visible", timeout: 90_000 });
    await create.click();
    const dialog = page.getByRole("dialog", { name: "Create Artifact", exact: true });
    await dialog.getByRole("textbox", { name: "Name", exact: true }).fill(name);
    const kind = dialog.getByLabel("Kind", { exact: true });
    await kind.click();
    await page.getByRole("option", { name: /^(GIS\s+)?Terrain$/i }).click();
    await dialog.getByRole("button", { name: "Create", exact: true }).click();
    observed.artifactName = name;
    reopenDocument = async () => {
      await page.goto(space.href, { waitUntil: "domcontentloaded", timeout: 50_000 });
      await create.waitFor({ state: "visible", timeout: 90_000 });
      const row = shell.getByRole("row").filter({ hasText: name });
      await row.getByRole("button", { name: "Open", exact: true }).click();
    };
  }
  const editor = shell.locator(surfaceQuery(editorId, "editor")).filter({ has: page.locator(".semio-world-3d-host") });
  await editor.waitFor({ timeout: 90_000 });
  const terrain = editor.locator("xpath=ancestor::*[@data-slot='window-body']");
  observed.editor = {
    app: await editor.getAttribute("data-surface-app-id"),
    document: await editor.getAttribute("data-surface-document-id"),
    primaryApp: await shell.locator(chipQuery).getAttribute("data-app-id"),
  };
  complete("load the Terrain Demo example");
  await page.locator('[id="playground.navbar.fixture"]').click();
  await page.getByRole("option", { name: "Demo", exact: true }).click();
  const features = terrain.locator('[data-slot="terrain-feature-list"] li');
  const demoPin = features.filter({ hasText: "Institut de Botanique" });
  await demoPin.waitFor({ state: "attached", timeout: 90_000 });
  const originalFeatures = (await features.allTextContents()).toSorted();
  assert(originalFeatures.length > 0);
  observed.demoFeatures = originalFeatures;
  complete("clear overlays and reverse the Terrain edit");
  const overlay = terrain.locator('[data-slot="window-engagement-overlay"]');
  const pane = overlay.locator('[data-slot="window-action-pane"]');
  const toggle = overlay.locator('button[id$=".engagement.toggle"]');
  await toggle.waitFor();
  if (await overlay.getAttribute("data-folded") === "true") await toggle.click();
  const history = async () => JSON.parse(await shell.getAttribute("data-history-json"));
  const waitForAction = async (before, action) => {
    await page.waitForFunction(({ selector, before, action }) => {
      const raw = document.querySelector(selector)?.getAttribute("data-history-json");
      if (!raw) return false;
      const state = JSON.parse(raw);
      return state.cursor > before.cursor && state.actionIds.at(-1) === action;
    }, { selector: shellQuery, before, action }, { timeout: 120_000 });
  };
  const clear = pane.locator('[data-slot="tree-item-row"][id="action.clearOverlays"]');
  const beforeEdit = await history();
  await clear.click();
  await demoPin.waitFor({ state: "detached" });
  assert.equal(await features.count(), 0);
  await waitForAction(beforeEdit, "clearOverlays");
  const replay = async (action, keys) => {
    const before = await history();
    await page.keyboard.press(keys);
    await waitForAction(before, action);
  };
  await replay("undo", `${modifier}+z`);
  await demoPin.waitFor({ state: "attached" });
  assert.deepEqual((await features.allTextContents()).toSorted(), originalFeatures);
  await replay("redo", `${modifier}+Shift+z`);
  await demoPin.waitFor({ state: "detached" });
  assert.equal(await features.count(), 0);
  await replay("undo", `${modifier}+z`);
  await demoPin.waitFor({ state: "attached" });
  assert.deepEqual((await features.allTextContents()).toSorted(), originalFeatures);
  if (cliArgs.includes("--exchange")) {
    complete("exchange an exact 64 KiB elevation raster through the signed Terrain Editor");
    const { selected, expected, samples, filename } = await maximumTerrainBrowserRaster();
    assert.equal(selected.byteLength, 65_536);
    const action = (id) => pane.locator(`[data-slot="tree-item-row"][id="action.${id}"]`);
    const download = async () => {
      const [file] = await Promise.all([page.waitForEvent("download", { timeout: 120_000 }), action("exportElevationGrid").click()]);
      try {
        assert.equal(file.suggestedFilename(), "terrain-elevation.asc");
        assert.equal(await file.failure(), null);
        return await readFile(await file.path(), "utf8");
      } finally {
        await file.delete();
      }
    };
    const oldRaster = await download();
    const beforeImport = await history();
    const [chooser] = await Promise.all([
      page.waitForEvent("filechooser", { timeout: 60_000 }),
      action("importElevationGrid").click(),
    ]);
    await chooser.setFiles({ name: filename, mimeType: "text/plain", buffer: selected });
    await waitForAction(beforeImport, "importElevationGridFile");
    const afterImport = await history();
    assert.equal(afterImport.entries, beforeImport.entries + 1);
    assert.equal(await download(), expected);
    await replay("undo", `${modifier}+z`);
    assert.equal(await download(), oldRaster);
    await replay("redo", `${modifier}+Shift+z`);
    assert.equal(await download(), expected);
    await replay("undo", `${modifier}+z`);
    observed.signedRaster = { bytes: selected.byteLength, samples, undo: true, redo: true };
  }
  complete("reload and reopen the persisted Terrain document");
  assert(observed.editor.document, "The Terrain Editor window exposes its artifact document ID");
  await page.reload({ waitUntil: "domcontentloaded", timeout: 60_000 });
  await shell.waitFor({ state: "visible", timeout: 90_000 });
  if (!(await editor.isVisible())) await reopenDocument();
  await editor.waitFor({ timeout: 90_000 });
  assert.equal(await editor.getAttribute("data-surface-document-id"), observed.editor.document);
  await demoPin.waitFor({ state: "attached", timeout: 90_000 });
  assert.deepEqual((await features.allTextContents()).toSorted(), originalFeatures);
  const reopenedHistory = await history();
  assert(reopenedHistory.actionIds.includes("clearOverlays"), "Terrain History restores the saved overlay edit");
  observed.reopened = { node: observed.workflowNode, document: observed.editor.document, history: reopenedHistory.actionIds };
  complete("switch the same Terrain document to Viewer and back");
  const roles = shell.locator('[id="playground.navbar.roles"]');
  observed.roles = {
    available: await roles.isVisible(),
    primaryApp: await shell.locator(chipQuery).getAttribute("data-app-id"),
  };
  assert(observed.roles.primaryApp === editorId && observed.roles.available, "The navbar exposes no role switch scoped to the open Terrain Editor");
  assert(observed.editor.document, "The Terrain Editor window exposes no mounted document ID");
  const switchTo = (role) => roles.locator(`[data-role="${role}"]`).click();
  await switchTo("viewer");
  const viewer = shell.locator(surfaceQuery(viewerId, "viewer")).filter({ has: page.locator(".semio-world-3d-host") });
  await viewer.waitFor({ timeout: 90_000 });
  assert.equal(await viewer.getAttribute("data-surface-document-id"), observed.editor.document);
  assert.deepEqual((await viewer.locator('[data-slot="terrain-feature-list"] li').allTextContents()).toSorted(), originalFeatures);
  assert.equal(await viewer.getByRole("button", { name: "Raise Terrain", exact: true }).count(), 0);
  await switchTo("editor");
  await editor.waitFor({ timeout: 90_000 });
  assert.equal(await editor.getAttribute("data-surface-document-id"), observed.editor.document);
  observed.completed.push(stage);
  assert.deepEqual(errors, []);
} catch (error) {
  observed.failed = stage;
  observed.path = page === undefined ? undefined : new URL(page.url()).pathname;
  observed.pageErrors = errors;
  observed.status = await page?.locator('[role="status"], [role="alert"]').allTextContents().catch(() => []);
  console.error("[DEBUG]", JSON.stringify(observed));
  throw error;
} finally {
  try {
    if (createdSpace) {
      await page.goto(landing.href, { waitUntil: "domcontentloaded", timeout: 50_000 });
      const home = page.locator(shellQuery);
      await home.locator(homeRoleQuery).waitFor({ timeout: 90_000 });
      const row = home.locator(`tr[data-row-id="${createdSpace.rowId}"]`).filter({ has: page.getByText(createdSpace.name, { exact: true }) });
      await row.getByRole("button", { name: "Delete", exact: true }).click();
      const dialog = page.getByRole("dialog", { name: "Delete Space?", exact: true });
      await dialog.getByRole("button", { name: "Delete", exact: true }).click();
      await row.waitFor({ state: "detached", timeout: 90_000 });
      observed.removedCreatedSpace = true;
    }
  } catch (error) {
    if (!observed.failed) throw error;
    console.error("[DEBUG] The disposable Space could not be removed through Home.");
  } finally {
    try {
      await browser?.close();
    } finally {
      await rm(profile, { recursive: true, force: true });
    }
  }
}
console.log(JSON.stringify(observed));
