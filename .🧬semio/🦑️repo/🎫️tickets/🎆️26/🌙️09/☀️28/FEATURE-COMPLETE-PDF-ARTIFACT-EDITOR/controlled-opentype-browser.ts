/** 🖋️ Verifies the controlled 48-page embedded font specimen in the mounted PDF editor. */
import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {mkdirSync, readFileSync} from "node:fs";
import {join, resolve} from "node:path";
import type {Locator, Page} from "playwright";
import {awaitBeacon, clickUncovered, dismissIntroduction, seatLocale, unfoldActionsRail, windowIds} from "@semio-tech/framework-os-dev/testing/program-matrix";

const [url, directory] = process.argv.slice(2);
assert(url && directory, "Expected the mounted PDF shell URL and authored fixture directory");
const assets = resolve(directory);
const source = join(assets, "raw.pdf");
const saved = join(assets, "browser-saved.pdf");
const scratch = join(assets, "browser-temp");
const expected = JSON.parse(readFileSync(join(assets, "expect.json"), "utf8")).fixtures[0];
assert.equal(readFileSync(source).length, expected.byteLength);
assert(expected.byteLength > 32_768 && expected.pages.length === 48);
mkdirSync(scratch, {recursive: true});

const browserOwner = resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser");
const {ensureRepositoryBrowserPathV1} = await import(join(browserOwner, "🧰️preparation/🟦️.ts"));
const {PLAYWRIGHT_MODULE_SPECIFIER} = await import(join(browserOwner, "🟦️.ts"));
ensureRepositoryBrowserPathV1(process.cwd(), process.env);
const {chromium}: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
const flags = process.platform === "darwin" ? ["--in-process-gpu", "--disable-accelerated-2d-canvas", "--use-angle=swiftshader"] : [];
const browser = await chromium.launch({
  args: flags, headless: true, downloadsPath: assets,
  env: {...process.env, TMPDIR: scratch, TMP: scratch, TEMP: scratch},
});
const actionRow = (id: string) => '[data-slot="window-action-pane"] [id="action.' + id + '"]';
const timeout = 120_000;

type FontPage = Window & { embeddedGlyphFills: Map<HTMLCanvasElement, Set<string>> };

async function boot(): Promise<Page> {
  const page = await browser.newPage({locale: "en-US", viewport: {width: 1600, height: 1000}, acceptDownloads: true});
  await page.addInitScript(() => {
    const current = window as FontPage;
    current.embeddedGlyphFills = new Map();
    const paths = new WeakMap<Path2D, string>();
    const OriginalPath = window.Path2D;
    window.Path2D = class extends OriginalPath {
      constructor(value?: string | Path2D) {
        super(value);
        if (typeof value === "string" && (value.match(/[CQL]/g)?.length ?? 0) >= 4) paths.set(this, value);
      }
    };
    const originalFill = CanvasRenderingContext2D.prototype.fill;
    Object.defineProperty(CanvasRenderingContext2D.prototype, "fill", {
      configurable: true,
      writable: true,
      value(this: CanvasRenderingContext2D, ...args: unknown[]) {
        const result = Reflect.apply(originalFill, this, args);
        if (args[0] instanceof OriginalPath) {
          const path = paths.get(args[0]);
          if (path) {
            const fills = current.embeddedGlyphFills.get(this.canvas) ?? new Set();
            fills.add(path);
            current.embeddedGlyphFills.set(this.canvas, fills);
          }
        }
        return result;
      },
    });
  });
  await page.goto(url, {waitUntil: "commit", timeout: 300_000});
  assert((await awaitBeacon(page, Date.now() + 300_000))?.startsWith("ready"), "Mounted shell did not boot");
  await dismissIntroduction(page);
  assert.equal(await seatLocale(page, "en"), "en=en");
  await unfoldActionsRail(page);
  if (!await page.locator(actionRow("openArtifactFile")).first().isVisible()) {
    await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
    await page.locator("[role=dialog] [data-slot=command-input]").first().fill("pdf");
    await page.locator('[data-command-item-id="spawn.stdio-pdf.s.stdio.pdf@1.7/*#editor"]').click();
  }
  return page;
}

async function action(page: Page, id: string) {
  await unfoldActionsRail(page);
  await clickUncovered(page, actionRow(id));
}

async function openFile(page: Page, native: string): Promise<Locator> {
  const previous = await windowIds(page);
  const [picker] = await Promise.all([page.waitForEvent("filechooser", {timeout}), action(page, "openArtifactFile")]);
  await picker.setFiles(native);
  const inserted = await page.waitForFunction(known => {
    const windows = [...document.querySelectorAll('[data-slot="window"]')].filter(window => window.id && !known.includes(window.id));
    return windows.some(window => window.querySelector('[data-ui-node-key="pdf-text-layer-page-47"]')) && windows.map(window => window.id);
  }, previous, {timeout});
  const ids = await inserted.jsonValue();
  assert(Array.isArray(ids));
  return page.locator(ids.map(id => '[data-slot="window"][id=' + JSON.stringify(id) + ']').join(","));
}

async function paints(page: Page, owned: Locator) {
  for (const number of [1, 48]) {
    const selection = owned.getByRole("button", {name: "Page " + number + " / 48", exact: true});
    await selection.press("Enter", {timeout});
    const text = owned.locator('[data-ui-node-key^="pdf-text-layer-p' + (number - 1) + '-"]')
      .and(owned.getByRole("button", {name: "Text: AgRé", exact: true}));
    await text.first().waitFor({timeout});
    assert.equal(await text.count(), 2);
  }
  const ids = await owned.evaluateAll(windows => windows.map(window => window.id));
  const painted = await page.waitForFunction(owners => {
    const canvases = [...(window as FontPage).embeddedGlyphFills];
    const paths = canvases.filter(([canvas]) => {
      const owner = canvas.closest('[data-slot="window"]');
      return owner && owners.includes(owner.id);
    }).flatMap(([, fills]) => [...fills]);
    return paths.length >= 2 && [...new Set(paths)].sort();
  }, ids, {timeout});
  const outlinePaths = await painted.jsonValue();
  assert(outlinePaths && outlinePaths.length >= 2);
  return outlinePaths.map(path => createHash("sha256").update(path).digest("hex")).sort();
}

try {
  const original = await boot();
  const before = await paints(original, await openFile(original, source));
  const [download] = await Promise.all([original.waitForEvent("download", {timeout}), action(original, "saveArtifactFile")]);
  await download.saveAs(saved);
  assert.equal(await download.failure(), null);
  await original.close();

  const reopened = await boot();
  const after = await paints(reopened, await openFile(reopened, saved));
  assert.deepEqual(after, before, "Saved/reopened canvas must paint identical embedded glyph paths");
  console.log("[DEBUG] Mounted PDF exposed AgRé on pages 1 and 48, painted embedded glyph outlines, saved and reopened");
} finally {
  await browser.close();
}
