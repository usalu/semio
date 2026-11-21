/** 🌐️ Drives the mounted PDF editor using the shell file chooser and page inspector. */
import assert from "node:assert/strict";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import type { Locator, Page } from "playwright";
import { awaitBeacon, clickUncovered, dismissIntroduction, seatLocale, unfoldActionsRail, windowIds } from "@semio-tech/framework-os-dev/testing/program-matrix";

const [base, source, generated, mode] = process.argv.slice(2);
assert(base && source && generated, "Expected shell URL, PDF fixture and generated output directory");
assert(mode === undefined || mode === "large-vector" || mode === "large-text" || mode === "large-images" || mode === "edit-content" || mode === "cancel-large" || mode === "page-history" || mode === "style-properties" || mode === "reject-invalid",
  "Expected optional large-vector, large-text, large-images, edit-content, cancel-large page-history, style-properties or reject-invalid case");
type CancelLarge = { readonly fileName: string; readonly baselinePath: string; readonly baselinePages: number; readonly taskLane: string; readonly expectedNotice: string; readonly slowCpuRate: number };
const cancelLarge: CancelLarge | undefined = mode === "cancel-large"
  ? JSON.parse(readFileSync(new URL("./🧪️mounted-cancel.json", import.meta.url), "utf8"))
  : undefined;
type ContentEdit = {
  page: {crop: {before: number[]; after: number[]}; trim: {before: number[]; after: number[]}};
  text: {before: string; after: string};
  vector: {at: [number, number]; newX: number};
  image: {width: number; newWidth: number};
  info: {before: string; after: string; author: string};
};
const contentEdit: ContentEdit | undefined = mode === "edit-content"
  ? JSON.parse(readFileSync(new URL("./🔣️mounted-content-edit.json", import.meta.url), "utf8"))
  : undefined;
type ControlledImages = {
  pages: {image: number}[];
  images: {name: string}[];
  imageSize: [number, number];
  imageOrigin: [number, number];
};
const controlledImages: ControlledImages | undefined = mode === "large-images"
  ? JSON.parse(readFileSync(new URL("./🎞️fixture.json", import.meta.url), "utf8"))
  : undefined;
const large: {readonly byteLength: number; readonly pageCount: number} | undefined = mode === "large-vector" || cancelLarge
  ? JSON.parse(readFileSync("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/📄️natural-file-edit/🔣️.json", "utf8")).largeVector
  : mode === "large-text" ? {byteLength: 33_038, pageCount: 48}
  : controlledImages ? {byteLength: 38_096, pageCount: controlledImages.pages.length} : undefined;
const pageHistory: PageHistorySpec | undefined = mode === "page-history"
  ? JSON.parse(readFileSync(new URL("./🔣️mounted-page-history.json", import.meta.url), "utf8")) : undefined;
const styleProperties: StyleProperties | undefined = mode === "style-properties"
  ? JSON.parse(readFileSync(new URL("./🔣️mounted-style-properties.json", import.meta.url), "utf8")) : undefined;
const invalidPdf: { expectedNotice: string; pageCount: number } | undefined = mode === "reject-invalid"
  ? JSON.parse(readFileSync(new URL("./🔣️mounted-invalid-pdf.json", import.meta.url), "utf8")) : undefined;
const reportName = invalidPdf ? "pdf-mounted-reject-invalid" : styleProperties ? "pdf-mounted-style-properties" : pageHistory ? "pdf-mounted-page-history" : cancelLarge ? "pdf-mounted-cancel" : contentEdit ? "pdf-mounted-content" :
  mode === "large-text" ? "pdf-mounted-large-text" : controlledImages ? "pdf-mounted-large-images" :
  large ? "pdf-mounted-large" : "pdf-mounted";
if (large) assert.equal(readFileSync(source).byteLength, large.byteLength, "Expected the independently verified large PDF fixture");
const output = resolve(generated);
mkdirSync(output, { recursive: true });
const scratch = join(output, `${reportName}-browser-tmp`);
mkdirSync(scratch, { recursive: true });
const browserOwner = resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser");
const { ensureRepositoryBrowserPathV1 } = await import(join(browserOwner, "🧰️preparation/🟦️.ts"));
const { PLAYWRIGHT_MODULE_SPECIFIER } = await import(join(browserOwner, "🟦️.ts"));
ensureRepositoryBrowserPathV1(process.cwd(), process.env);
const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
const args = process.platform === "darwin" ? ["--in-process-gpu", "--disable-accelerated-2d-canvas", "--use-angle=swiftshader"] : [];
const browser = await chromium.launch({args, headless: true, downloadsPath: output, env: {...process.env, TMPDIR: scratch, TMP: scratch, TEMP: scratch}});
const context = await browser.newContext({locale: "en-US", viewport: {width: 1600, height: 1000}, acceptDownloads: true});
const page = await context.newPage();
const diagnostics: string[] = [];
await page.addInitScript(() => {
  const events: string[] = [];
  Object.defineProperty(window, "__pdfBrowserNotices", {value: events});
  const seen = new WeakMap<Element, string>();
  new MutationObserver(() => {
    for (const node of document.querySelectorAll("[data-semio-transient-notice]")) {
      const event = `${node.getAttribute("data-notice-code") ?? ""}|${node.textContent?.trim() ?? ""}`;
      if (seen.get(node) !== event) {
        seen.set(node, event);
        events.push(event);
      }
    }
  }).observe(document, {childList: true, subtree: true, attributes: true, characterData: true});
});

const traceOpen = process.env.SEMIO_PDF_TRACE_OPEN === "1";
if (traceOpen) await page.addInitScript(() => {
  const trace: unknown[] = [];
  const workers = new WeakSet<Worker>();
  const recent = new Map<string, unknown>();
  const replies: unknown[] = [];
  const controls: {actor: unknown}[] = [];
  const receipts: unknown[] = [];
  const post = Worker.prototype.postMessage;
  Object.defineProperty(window, "__pdfActorTraffic", {value: {trace, replies, controls, receipts}});
  const record = (row: unknown) => {
    trace.push(row);
    if (trace.length > 256) trace.shift();
  };
  const shape = (value: unknown, depth = 0): unknown => {
    if (value == null) return null;
    if (depth > 2 || typeof value !== "object") return typeof value;
    if (ArrayBuffer.isView(value)) return [value.constructor.name, value.byteLength];
    const source = value as Record<string, unknown>;
    if (Array.isArray(value)) return value.slice(0, 4).map(item => shape(item, depth + 1));
    const entries = Object.entries(source).slice(0, 16).map(([key, val]) => [
      key, ["kind", "tag", "type", "action", "command", "phase"].includes(key) && typeof val === "string" ? val.slice(0, 100)
        : ["instance", "pageIndex", "pageCount", "commandIndex", "commandCount", "kind", "itemCount"].includes(key) && typeof val === "number" ? val
        : ["seq", "epoch", "operation", "generation"].includes(key) && typeof val === "bigint" ? val.toString()
        : shape(val, depth + 1),
    ]);
    return Object.fromEntries(entries);
  };
  Worker.prototype.postMessage = function(this: Worker, message: unknown, ...rest: unknown[]) {
    const data = message as Record<string, unknown> | null;
    if (data && typeof data.requestId === "string") {
      if (!workers.has(this)) {
        workers.add(this);
        this.addEventListener("message", event => {
          const answer = event.data;
          if (answer?.kind === "original-turn-receipt" || answer?.kind === "original-producer-held") {
            receipts.push({kind: answer.kind, actor: answer.actorId, input: shape(answer.receipt?.input), spent: Object.fromEntries(Object.entries(answer.receipt?.spent ?? {}).map(([key, value]) => [key, typeof value === "bigint" ? value.toString() : value])), work: String(answer.receipt?.workUnitsSpent ?? ""), refusal: shape(answer.refusal)});
            if (receipts.length > 64) receipts.shift();
          }
          if (answer?.kind === "result" && answer.ok === false) {
            replies.push({requestId: answer.requestId, request: recent.get(answer.requestId), error: String(answer.error).slice(0, 350), recent: trace.slice(-32)});
            if (replies.length > 8) replies.shift();
          }
        });
      }
      const row = {requestId: data.requestId, type: data.kind, actor: data.actorId,
        events: shape(data.events), page: shape((data.commandPage as {cursor?: unknown} | undefined)?.cursor),
        request: shape(data.request), frame: shape(data.frame)};
      recent.set(data.requestId, row);
      if (recent.size > 256) recent.delete(recent.keys().next().value!);
      record(row);
      if (data.kind !== "turn" || data.commandPage || (Array.isArray(data.events) && data.events.length)) {
        controls.push(row);
        if (controls.filter(control => control.actor === row.actor).length > 80) controls.splice(controls.findIndex(control => control.actor === row.actor), 1);
        if (controls.length > 512) controls.shift();
      }
    }
    Reflect.apply(post, this, [message, ...rest]);
  };
});

if (traceOpen) page.on("worker", worker => {
  void worker.evaluate(() => {
    const original = globalThis.Error;
    globalThis.Error = new Proxy(original, {
      construct(target, args, receiver) {
        if (typeof args[0] === "string" && args[0].startsWith("original-turn.native-return-refused")) {
          const cause = args[1]?.cause;
          const receipt = cause?.payload ?? cause;
          const spend = receipt?.spent;
          const grant = receipt?.input?.grant;
          const values = (record: Record<string, unknown> | undefined) =>
            Object.fromEntries(Object.entries(record ?? {}).filter(([, value]) => typeof value === "number" || typeof value === "bigint")
              .map(([key, value]) => [key, String(value)]));
          console.warn("[DEBUG] PDF component refusal accounting " + JSON.stringify({
            kind: receipt?.failure?.tag, input: values(receipt?.input), grant: values(grant),
            spent: values(spend), work: String(receipt?.workUnitsSpent ?? ""),
          }));
        }
        return Reflect.construct(target, args, receiver);
      },
    });
  }).catch(() => undefined);
});
page.on("pageerror", error => diagnostics.push(error.message));
page.on("console", message => {
  if (message.type() === "error" || message.type() === "warning") diagnostics.push(message.text());
});

async function windowAction(page: Page, id: string) {
  await unfoldActionsRail(page);
  return `[data-slot="window-action-pane"] [id="action.${id}"]`;
}

async function openFile(page: Page, action: string, path: string, pageCount: number) {
  const before = await windowIds(page);
  const noticeCursor = await page.evaluate(() => ((window as Window & {__pdfBrowserNotices?: string[]}).__pdfBrowserNotices ?? []).length);
  const [picker] = await Promise.all([page.waitForEvent("filechooser", {timeout: 30_000}), clickUncovered(page, action)]);
  await picker.setFiles(resolve(path));
  const windows = await page.waitForFunction(({known, pageCount, noticeCursor}) => {
    const notices = (window as Window & {__pdfBrowserNotices?: string[]}).__pdfBrowserNotices ?? [];
    const failure = notices.slice(noticeCursor).find(notice =>
      /^shell\.(?:openFile\.|documentTransfer\.(?:file-open-failed|open-cancelled|transfer-busy)\|)/u.test(notice));
    if (failure) return {failure};
    const added = [...document.querySelectorAll('[data-slot="window"]')]
      .filter(window => window.id && !known.includes(window.id));
    const inspector = added.some(window => window.querySelector('[data-ui-node-key="pdf-page-rotation"]'));
    const finalPage = added.some(window => {
      const label = window.querySelector(`[data-ui-node-key="pdf-text-layer-page-${pageCount - 1}"]`);
      return label?.textContent?.trim() === `Page ${pageCount} / ${pageCount}`;
    });
    return inspector && finalPage ? {ids: added.map(window => window.id)} : false;
  }, {known: before, pageCount, noticeCursor}, {timeout: 120_000});
  const outcome = await windows.jsonValue();
  assert(typeof outcome === "object" && outcome !== null, "Expected a mounted PDF open outcome");
  if ("failure" in outcome) throw Error(outcome.failure);
  assert("ids" in outcome && outcome.ids);
  return page.locator(outcome.ids.map(id => `[data-slot="window"][id=${JSON.stringify(id)}]`).join(","));
}

async function verifyRotation(owned: Locator, name: string, degrees: string) {
  await owned.locator('[data-ui-node-key="pdf-object-kind"]').filter({hasText: new RegExp(`^${name}$`)}).waitFor({timeout: 60_000});
  const rotation = owned.locator('[data-ui-node-key="pdf-page-rotation"]');
  await rotation.waitFor({state: "visible"});
  assert.equal(await rotation.inputValue(), degrees, `${name} should be at ${degrees} degrees`);
  return rotation;
}

async function inspectorField(owned: Locator, id: string, value: string) {
  const field = owned.locator(`[data-ui-node-key="${id}"]`);
  await field.waitFor({state: "visible", timeout: 60_000});
  assert.equal(await field.inputValue(), value, id);
  return field;
}

async function commitInspectorField(owned: Locator, id: string, before: string, after: string) {
  const field = await inspectorField(owned, id, before);
  await field.fill(after);
  await field.blur();
  await field.and(owned.locator(':not([data-draft-dirty="true"])')).waitFor({timeout: 60_000});
  assert.equal(await field.inputValue(), after, id);
}

async function selectInspector(owned: Locator, name: string | RegExp, kind: string) {
  await owned.getByRole("button", {name, exact: true}).press("Enter", {timeout: 60_000});
  await owned.locator('[data-ui-node-key="pdf-object-kind"]')
    .filter({hasText: new RegExp(`^${kind}$`)}).waitFor({timeout: 60_000});
}

async function verifyOrEditContent(owned: Locator, spec: ContentEdit, editing: boolean) {
  const phase = editing ? "before" : "after";
  const field = (id: string, before: string, after: string) => editing
    ? commitInspectorField(owned, id, before, after)
    : inspectorField(owned, id, after);
  await selectInspector(owned, `Text: ${spec.text[phase]}`, "Text");
  await field("pdf-object-text", spec.text.before, spec.text.after);
  await owned.getByRole("button", {name: `Text: ${spec.text.after}`, exact: true}).waitFor({timeout: 60_000});
  await selectInspector(owned, "Vector graphic", "Vector");
  await field("pdf-object-x", String(spec.vector.at[0]), String(spec.vector.newX));
  await selectInspector(owned, /^Image(?:$|: )/u, "Image");
  await field("pdf-object-width", String(spec.image.width), String(spec.image.newWidth));

  await selectInspector(owned, "Page 1 / 2", "Page 1");
  await field("pdf-page-title", spec.info.before, spec.info.after);
  await owned.getByText(`Title: ${spec.info.after}`, {exact: true}).waitFor({timeout: 60_000});
  await field("pdf-page-crop-box", spec.page.crop.before.join(" "), spec.page.crop.after.join(" "));
  await field("pdf-page-trim-box", spec.page.trim.before.join(" "), spec.page.trim.after.join(" "));
  await inspectorField(owned, "pdf-page-author", spec.info.author);
}

async function verifyTrimHistory(page: Page, owned: Locator, spec: ContentEdit) {
  const ids = await owned.evaluateAll(windows => windows.map(window => window.id));
  for (const [action, trim] of [["undo", spec.page.trim.before], ["redo", spec.page.trim.after]] as const) {
    await clickUncovered(page, await windowAction(page, action));
    await selectInspector(owned, "Page 1 / 2", "Page 1");
    await page.waitForFunction(({ids, expected}) => ids.some(id => {
      const field = document.getElementById(id)?.querySelector('[data-ui-node-key="pdf-page-trim-box"]');
      return field instanceof HTMLInputElement && field.value === expected;
    }), {ids, expected: trim.join(" ")}, {timeout: 60_000});
    await inspectorField(owned, "pdf-page-crop-box", spec.page.crop.after.join(" "));
    await inspectorField(owned, "pdf-page-title", spec.info.after);
  }
}

type StyleProperties = {
  fontSize: {before: string; after: string};
  fillRed: {before: string; after: string};
  language: {before: string; after: string};
  subject: {before: string; after: string};
  text: string;
  author: string;
};

async function verifyOrEditStyles(owned: Locator, spec: StyleProperties, editing: boolean) {
  const field = (id: string, value: {before: string; after: string}) => editing
    ? commitInspectorField(owned, id, value.before, value.after)
    : inspectorField(owned, id, value.after);
  await selectInspector(owned, `Text: ${spec.text}`, "Text");
  await field("pdf-object-size", spec.fontSize);
  await selectInspector(owned, "Vector graphic", "Vector");
  await field("pdf-object-fill-red", spec.fillRed);
  await selectInspector(owned, "Page 1 / 2", "Page 1");
  await field("pdf-page-language", spec.language);
  await field("pdf-page-subject", spec.subject);
  await inspectorField(owned, "pdf-page-author", spec.author);
}

type PageHistorySpec = {
  pages: {id: string; text: string | null; width: number; height: number}[];
  states: string[][];
  edits: {action: string; args: Record<string, number>}[];
};

async function verifyPageState(target: Page, owned: Locator, spec: PageHistorySpec, order: string[]) {
  const ids = await owned.evaluateAll(windows => windows.map(window => window.id));
  await target.waitForFunction(({ids, order}) => {
    const owned = ids.map(id => document.getElementById(id)).filter(node => node !== null);
    const markers = owned.flatMap(root => [...root.querySelectorAll('[data-ui-node-key^="pdf-text-layer-page-"]')])
      .filter(node => /^pdf-text-layer-page-\d+$/u.test(node.getAttribute("data-ui-node-key") ?? ""));
    return markers.length === order.length && markers.every((node, index) => node.textContent?.trim() === `Page ${index + 1} / ${order.length}`);
  }, {ids, order}, {timeout: 60_000});
  for (const [position, id] of order.entries()) {
    const leaf = spec.pages.find(page => page.id === id);
    assert(leaf, "Each history entry must name a fixture page");
    await selectInspector(owned, `Page ${position + 1} / ${order.length}`, `Page ${position + 1}`);
    const width = owned.locator('[data-ui-node-key="pdf-page-width"]');
    const height = owned.locator('[data-ui-node-key="pdf-page-height"]');
    await target.waitForFunction(({ids, expected}) => ids.some(id => {
      const scope = document.getElementById(id);
      const width = scope?.querySelector('[data-ui-node-key="pdf-page-width"]');
      const height = scope?.querySelector('[data-ui-node-key="pdf-page-height"]');
      return width instanceof HTMLInputElement && height instanceof HTMLInputElement && width.value === String(expected.width) && height.value === String(expected.height);
    }), {ids, expected: leaf}, {timeout: 30_000});
    assert.equal(await width.inputValue(), String(leaf.width));
    assert.equal(await height.inputValue(), String(leaf.height));
    const pageItems = owned.locator(`[data-ui-node-key^="pdf-text-layer-p${position}-"]`);
    if (leaf.text) {
      await pageItems.and(owned.getByRole("button", {name: `Text: ${leaf.text}`, exact: true})).waitFor({timeout: 30_000});
    } else {
      assert.equal(await pageItems.and(owned.getByRole("button", {name: /^Text: /u})).count(), 0);
    }
  }
}

async function testPageHistory(target: Page, owned: Locator, spec: PageHistorySpec) {
  await verifyPageState(target, owned, spec, spec.states[0]!);
  for (const [index, edit] of spec.edits.entries()) {
    await clickUncovered(target, await windowAction(target, edit.action));
    for (const [name, value] of Object.entries(edit.args)) {
      const input = target.locator(await windowAction(target, edit.action + ".arg." + name)).locator("input").first();
      await input.fill(String(value));
      await input.blur();
    }
    await target.locator('[data-slot="window-action-pane"]').getByRole("button", {name: "Execute", exact: true}).click();
    await verifyPageState(target, owned, spec, spec.states[index + 1]!);
  }
  for (let index = spec.states.length - 2; index >= 0; index--) {
    await clickUncovered(target, await windowAction(target, "undo"));
    await verifyPageState(target, owned, spec, spec.states[index]!);
  }
  for (const order of spec.states.slice(1)) {
    await clickUncovered(target, await windowAction(target, "redo"));
    await verifyPageState(target, owned, spec, order);
  }
  const final = spec.states.at(-1)!;
  const saved = await saveAndReopen(target, final.length);
  await verifyPageState(target, saved, spec, final);
}

async function verifyControlledImages(owned: Locator, spec: ControlledImages) {
  for (const [pageIndex, page] of spec.pages.entries()) {
    await selectInspector(owned, `Page ${pageIndex + 1} / ${spec.pages.length}`, `Page ${pageIndex + 1}`);
    const name = spec.images[page.image].name;
    const row = owned.locator(`[data-ui-node-key^="pdf-text-layer-p${pageIndex}-"]`)
      .and(owned.getByRole("button", {name: `Image: ${name}`, exact: true}));
    await row.press("Enter", {timeout: 60_000});
    await owned.locator('[data-ui-node-key="pdf-object-kind"]')
      .filter({hasText: /^Image$/u}).waitFor({timeout: 60_000});
    await inspectorField(owned, "pdf-object-x", String(spec.imageOrigin[0]));
    await inspectorField(owned, "pdf-object-y", String(spec.imageOrigin[1]));
    await inspectorField(owned, "pdf-object-width", String(spec.imageSize[0]));
    await inspectorField(owned, "pdf-object-height", String(spec.imageSize[1]));
  }
}

async function saveAndReopen(page: Page, pageCount = 2) {
  const save = await windowAction(page, "saveArtifactFile");
  const cursor = await page.evaluate(() => ((window as Window & {__pdfBrowserNotices?: string[]}).__pdfBrowserNotices ?? []).length);
  const refusal = page.waitForFunction(from => {
    const notices = (window as Window & {__pdfBrowserNotices?: string[]}).__pdfBrowserNotices ?? [];
    return notices.slice(from).find(notice => /^shell\.documentTransfer\.(?:file-save-failed|file-save-cancelled|transfer-busy)\|/u.test(notice)) ?? false;
  }, cursor, {timeout: 60_000}).then(async notice => { throw Error(await notice.jsonValue()); });
  const [download] = await Promise.all([
    Promise.race([page.waitForEvent("download", {timeout: 60_000}), refusal]),
    clickUncovered(page, save),
  ]);
  const saved = join(output, `${reportName}.pdf`);
  await download.saveAs(saved);
  assert.equal(await download.failure(), null);
  assert.equal(readFileSync(saved).subarray(0, 5).toString("ascii"), "%PDF-");
  return openFile(page, await windowAction(page, "openArtifactFile"), saved, pageCount);
}

async function cancelLargeOpen(page: Page, action: string, source: string, spec: CancelLarge) {
  await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
  await page.locator("[role=dialog] [data-slot=command-input]").first().fill("task manager");
  await page.locator('[data-command-item-id$="os.openTaskManager"]').click();
  const manager = page.locator("[data-semio-task-manager-tasks]");
  await manager.waitFor({state: "visible"});
  const before = (await windowIds(page)).sort();
  const noticeCursor = await page.evaluate(() => ((window as Window & {__pdfBrowserNotices?: string[]}).__pdfBrowserNotices ?? []).length);
  await unfoldActionsRail(page);
  const [picker] = await Promise.all([page.waitForEvent("filechooser", {timeout: 30_000}), clickUncovered(page, action)]);
  const throttling = await page.context().newCDPSession(page);
  await throttling.send("Emulation.setCPUThrottlingRate", {rate: spec.slowCpuRate});
  try {
    await picker.setFiles(resolve(source));
    const task = manager.locator(`[data-semio-task-manager-lane="${spec.taskLane}"]`)
      .filter({has: page.getByText(spec.fileName, {exact: true})});
    await task.waitFor({state: "visible", timeout: 45_000});
    const progress = task.getByRole("progressbar").and(task.locator('[data-semio-task-manager-progress]:not([data-semio-task-manager-progress=""]):not([data-semio-task-manager-progress="0"])'));
    await progress.waitFor({state: "visible", timeout: 45_000});
    const completed = Number(await progress.getAttribute("data-semio-task-manager-progress"));
    assert(completed > 0, "Expected recorded progress for the original PDF transfer");
    await task.locator("[data-semio-task-manager-cancel]").click({timeout: 10_000});
    await page.waitForFunction(({expected, fileName, cursor}) =>
      ((window as Window & {__pdfBrowserNotices?: string[]}).__pdfBrowserNotices ?? []).slice(cursor)
        .some(notice => notice.startsWith(expected + "|") && notice.includes(fileName)),
      {expected: spec.expectedNotice, fileName: spec.fileName, cursor: noticeCursor}, {timeout: 90_000});
    await task.waitFor({state: "detached", timeout: 45_000});
    assert.deepEqual((await windowIds(page)).sort(), before, "A cancelled PDF transfer must preserve the original editor windows");
    console.log(`[DEBUG] Application cancelled ${spec.fileName} after ${completed} bytes of transfer progress`);
  } finally {
    await throttling.send("Emulation.setCPUThrottlingRate", {rate: 1}).catch(() => undefined);
    await throttling.detach();
  }
}

async function selectSecondPage(page: Page, owned: Locator) {
  const canvas = owned.locator(".semio-canvas-2d-host canvas");
  await canvas.waitFor({state: "visible"});
  const bounds = await canvas.boundingBox();
  assert(bounds, "The page canvas has no bounds");
  const middle = {x: bounds.width / 2, y: bounds.height / 2};
  await page.mouse.move(bounds.x + middle.x, bounds.y + middle.y);
  const steps = 22;
  for (let i = 0; i < steps; i++) {
    await page.mouse.wheel(0, 100);
    await page.evaluate(() => new Promise<void>(resolveFrame => requestAnimationFrame(() => resolveFrame())));
  }
  const zoom = Math.pow(0.9, steps);
  await canvas.click({position: {x: middle.x + 150 * zoom, y: middle.y + 1010 * zoom}});
}

try {
  await page.goto(base, {waitUntil: "commit", timeout: 300_000});
  const beacon = await awaitBeacon(page, Date.now() + 300_000);
  assert(beacon?.startsWith("ready"), `The shell did not boot: ${beacon}`);
  await dismissIntroduction(page);
  assert.equal(await seatLocale(page, "en"), "en=en", "Could not select English in the shell");
  const open = await windowAction(page, "openArtifactFile");
  if (!await page.locator(open).first().isVisible()) {
    await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
    const input = page.locator("[role=dialog] [data-slot=command-input]").first();
    await input.fill("pdf");
    await page.locator("[data-command-item-id=\"spawn.stdio-pdf.s.stdio.pdf@1.7/*#editor\"]").click();
    await windowAction(page, "openArtifactFile");
  }
  if (invalidPdf) {
    const baseline = await openFile(page, open,
      "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf",
      invalidPdf.pageCount);
    await verifyRotation(baseline, "Page 1", "0");
    const stable = (await windowIds(page)).sort();
    const cursor = diagnostics.length;
    await assert.rejects(
      openFile(page, await windowAction(page, "openArtifactFile"), source, invalidPdf.pageCount),
      error => error instanceof Error && error.message.startsWith(invalidPdf.expectedNotice + "|"),
    );
    const refusal = diagnostics.slice(cursor);
    assert(refusal.some(message => message.includes("file-open-failed")));
    assert(!refusal.some(message => message.includes("natural-file.import-and-retirement-failed")), refusal.join("\n"));
    assert.deepEqual((await windowIds(page)).sort(), stable);
    await verifyRotation(baseline, "Page 1", "0");
    console.log("[DEBUG] Malformed PDF retired its fresh owner and left the active PDF editor intact");
  } else if (cancelLarge && large) {
    const baseline = await openFile(page, open, cancelLarge.baselinePath, cancelLarge.baselinePages);
    await verifyRotation(baseline, "Page 1", "0");
    await cancelLargeOpen(page, open, source, cancelLarge);
    await verifyRotation(baseline, "Page 1", "0");
    const retry = await openFile(page, await windowAction(page, "openArtifactFile"), source, large.pageCount);
    await retry.getByRole("button", {name: `Page ${large.pageCount} / ${large.pageCount}`, exact: true}).press("Enter");
    await verifyRotation(retry, `Page ${large.pageCount}`, "0");
    await page.screenshot({path: join(output, `${reportName}.png`)});
    console.log("[DEBUG] Original editor stayed intact and the cancelled large PDF opened successfully on retry");
  } else {
    const owned = await openFile(page, open, source, large?.pageCount ?? 2);
    await verifyRotation(owned, "Page 1", "0");
    if (styleProperties) {
      await verifyOrEditStyles(owned, styleProperties, true);
      const reopened = await saveAndReopen(page);
      await verifyOrEditStyles(reopened, styleProperties, false);
      console.log("[DEBUG] Mounted font size, vector fill, language and subject edits survived save/reopen");
    } else if (pageHistory) {
      await testPageHistory(page, owned, pageHistory);
      await page.screenshot({path: join(output, `${reportName}.png`)});
      console.log("[DEBUG] Mounted page insertion, move, removal, undo, redo, save and reopen");
    } else if (contentEdit) {
      await verifyOrEditContent(owned, contentEdit, true);
      await verifyTrimHistory(page, owned, contentEdit);
      const reopened = await saveAndReopen(page);
      await verifyOrEditContent(reopened, contentEdit, false);
      await page.screenshot({path: join(output, `${reportName}.png`)});
      console.log("[DEBUG] Mounted PDF edited text, vector position, image width, title and page boxes, verified trim undo/redo, saved and reopened");
    } else if (large) {
      const lastPage = owned.getByRole("button", {name: `Page ${large.pageCount} / ${large.pageCount}`, exact: true});
      await lastPage.press("Enter", {timeout: 60_000});
      await verifyRotation(owned, `Page ${large.pageCount}`, "0");
      if (mode === "large-text") {
        const text = `Text: Page ${large.pageCount} of ${large.pageCount}`;
        await owned.getByRole("button", {name: text, exact: true}).waitFor({timeout: 60_000});
        console.log(`[DEBUG] Verified the accessible final page text: ${text}`);
      } else if (controlledImages) {
        await verifyControlledImages(owned, controlledImages);
        const reopened = await saveAndReopen(page, large.pageCount);
        await verifyControlledImages(reopened, controlledImages);
        console.log("[DEBUG] Mounted, saved and reopened all four controlled image pages with their expected image names and geometry");
      } else {
        const finalPageGraphics = owned.locator(`[data-ui-node-key^="pdf-text-layer-p${large.pageCount - 1}-"]`)
          .and(owned.getByRole("button", {name: "Vector graphic", exact: true}));
        await finalPageGraphics.first().waitFor({timeout: 60_000});
      }
      await page.screenshot({path: join(output, `${reportName}.png`)});
      console.log(`[DEBUG] Mounted ${large.byteLength}-byte PDF and selected its final page ${large.pageCount} through the Text Layer`);
    } else {
      await selectSecondPage(page, owned);
      const rotation = await verifyRotation(owned, "Page 2", "0");
      await rotation.fill("90");
      await rotation.blur();
      await page.screenshot({path: join(output, `${reportName}.png`)});

      const reopened = await saveAndReopen(page);
      await verifyRotation(reopened, "Page 1", "0");
      const pageTwo = reopened.getByRole("button", {name: "Page 2 / 2", exact: true});
      await pageTwo.press("Enter");
      await verifyRotation(reopened, "Page 2", "90");
      console.log("[DEBUG] Mounted PDF opened, rotated page 2, saved and reopened with page 1 unchanged");
    }
  }
} catch (error) {
  if (traceOpen) {
    const traffic = await page.evaluate(() => Reflect.get(window, "__pdfActorTraffic")).catch(() => undefined);
    if (traffic) writeFileSync(join(output, `${reportName}-actor-traffic.json`), JSON.stringify(traffic, null, 2));
  }
  await page.screenshot({path: join(output, `${reportName}-failed.png`)}).catch(() => undefined);
  diagnostics.push(await page.locator("body").innerText({timeout: 1_500}).catch(() => ""));
  throw error;
} finally {
  const notices = await page.evaluate(() => (window as Window & {__pdfBrowserNotices?: string[]}).__pdfBrowserNotices ?? []).catch(() => []);
  writeFileSync(join(output, `${reportName}-console.json`), JSON.stringify([...diagnostics, ...notices], null, 2));
  await browser.close();
}
