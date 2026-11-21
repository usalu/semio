/** 🖼️ Opens a large authored DCT PDF, verifies live canvas paint, and saves/reopens it in the mounted app. */
import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {mkdirSync, readFileSync} from "node:fs";
import {join, resolve} from "node:path";
import type {Locator, Page} from "playwright";
import {awaitBeacon, clickUncovered, dismissIntroduction, seatLocale, unfoldActionsRail, windowIds} from "@semio-tech/framework-os-dev/testing/program-matrix";

const [url, directory] = process.argv.slice(2);
assert(url && directory, "Expected the mounted PDF shell URL and authored fixture directory");
const assets = resolve(directory);
const source = join(assets, "large.pdf");
const saved = join(assets, "browser-saved.pdf");
const scratch = join(assets, "browser-temp");
const expected = JSON.parse(readFileSync(join(assets, "expect.json"), "utf8"));
assert.equal(readFileSync(source).length, expected.fileBytes);
assert(expected.fileBytes > 32_768 && expected.images.length === 2);
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

type DctPage = Window & { codedImageDraws: Set<string> };

async function boot(): Promise<Page> {
  const page = await browser.newPage({locale: "en-US", viewport: {width: 1600, height: 1000}, acceptDownloads: true});
  await page.addInitScript(() => {
    const current = window as DctPage;
    current.codedImageDraws = new Set();
    const draw = CanvasRenderingContext2D.prototype.drawImage;
    Object.defineProperty(CanvasRenderingContext2D.prototype, "drawImage", {
      configurable: true,
      writable: true,
      value(this: CanvasRenderingContext2D, ...args: unknown[]) {
        const [image] = args;
        const result = Reflect.apply(draw, this, args);
        if (image instanceof HTMLImageElement && image.naturalWidth === 176 && image.naturalHeight === 176
            && image.src.startsWith("data:image/png;base64,")) {
          current.codedImageDraws.add(image.src);
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
    return windows.some(window => window.querySelector('[data-ui-node-key="pdf-text-layer-page-2"]')) && windows.map(window => window.id);
  }, previous, {timeout});
  const ids = await inserted.jsonValue();
  assert(Array.isArray(ids));
  return page.locator(ids.map(id => '[data-slot="window"][id=' + JSON.stringify(id) + ']').join(","));
}

async function paints(page: Page, owned: Locator) {
  for (const [index, name] of ["Photo", "Gray"].entries()) {
    const button = owned.locator('[data-ui-node-key^="pdf-text-layer-p' + index + '-"]')
      .and(owned.getByRole("button", {name: "Image: " + name, exact: true}));
    await button.waitFor({timeout});
    assert.equal(await button.count(), 1);
  }
  await page.waitForFunction(() => (window as DctPage).codedImageDraws.size >= 2, undefined, {timeout});
  const rendered = await page.evaluate(async () => {
    const images = [...(window as DctPage).codedImageDraws];
    return Promise.all(images.map(async source => {
      const image = new Image();
      image.src = source;
      await image.decode();
      const canvas = document.createElement("canvas");
      canvas.width = canvas.height = 176;
      const context = canvas.getContext("2d", {willReadFrequently: true})!;
      context.drawImage(image, 0, 0);
      const pixels = context.getImageData(0, 0, 176, 176).data;
      let colored = 0, low = 255, high = 0, opaque = true;
      for (let index = 0; index < pixels.length; index += 4) {
        const [r, g, b, a] = pixels.subarray(index, index + 4);
        if (r !== g || g !== b) colored++;
        low = Math.min(low, r);
        high = Math.max(high, r);
        opaque &&= a === 255;
      }
      return {source, colored, range: high - low, opaque};
    }));
  });
  assert.equal(rendered.length, 2, "Expected two independent decoded image layers in the canvas");
  assert(rendered.every(image => image.opaque && image.range > 64), "Canvas drew transparent or placeholder image content");
  assert(rendered.some(image => image.colored === 0) && rendered.some(image => image.colored > 1_000));
  return rendered.map(image => createHash("sha256").update(image.source).digest("hex")).sort();
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
  assert.deepEqual(after, before, "Saved/reopened canvas must decode and paint identical image layers");
  console.log("[DEBUG] Mounted PDF opened both DCT pages, painted RGB and grayscale PNG layers, saved and reopened identical canvas pixels");
} finally {
  await browser.close();
}
