/** 🛑 Exercises cancellation of a live retained PDF image decode through the Page surface. */
import assert from "node:assert/strict";
import { copyFileSync, existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { deflateSync } from "node:zlib";
import type { ElementHandle, Locator } from "playwright";
import { python } from "./📜️e2e-harness.ts";

const [url, outputDirectory] = process.argv.slice(2);
assert(url && outputDirectory, "Expected a shell URL (or --fixture-only) and the ticket output directory");
const output = resolve(outputDirectory);
mkdirSync(output, { recursive: true });
const corpus = JSON.parse(readFileSync(new URL("./image-authoring.json", import.meta.url), "utf8"));
const pngBytes = maximumPng(corpus.bounds.maxFileBytes, corpus.bounds.maxPixels);
writeFileSync(join(output, "decode-cancellation.png"), pngBytes);
if (url !== "--fixture-only") {
  obtainSource();
  await exercise();
}

function obtainSource() {
  const target = join(output, "image-source.pdf");
  if (existsSync(target)) return;
  const work = join(output, "oracle-source");
  const made = Bun.spawnSync(python("📜️pdf-raw-png-history-oracle.py", work, "--self-test"), { stdout: "inherit", stderr: "inherit" });
  assert.equal(made.exitCode, 0, "the PNG history oracle could not derive image-source.pdf");
  copyFileSync(join(work, "oracle-proof", "image-source.pdf"), target);
}

function maximumPng(maxBytes: number, maxPixels: number): Buffer {
  const edge = Math.sqrt(maxPixels);
  assert(Number.isInteger(edge) && edge > 0);
  const chunk = (kind: string, data: Buffer) => {
    const encoded = Buffer.alloc(12 + data.length);
    encoded.writeUInt32BE(data.length, 0);
    encoded.write(kind, 4, 4, "ascii");
    data.copy(encoded, 8);
    encoded.writeUInt32BE(Bun.hash.crc32(encoded.subarray(4, 8 + data.length)), 8 + data.length);
    return encoded;
  };
  const signature = Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]);
  const header = Buffer.alloc(13);
  header.writeUInt32BE(edge, 0);
  header.writeUInt32BE(edge, 4);
  header.set([8, 6], 8);
  const rows = Buffer.alloc(edge * (edge * 4 + 1));
  let state = 0x533b99b1;
  for (let row = 0; row < edge; row++) {
    const start = row * (edge * 4 + 1);
    for (let column = 0; column < edge * 4; column++) {
      state ^= state << 13;
      state ^= state >>> 17;
      state ^= state << 5;
      rows[start + 1 + column] = state & 255;
    }
  }
  const ihdr = chunk("IHDR", header);
  const idat = chunk("IDAT", deflateSync(rows, { level: 0 }));
  const iend = chunk("IEND", Buffer.alloc(0));
  const padding = maxBytes - signature.length - ihdr.length - idat.length - iend.length - 12;
  assert(padding > 0);
  const bytes = Buffer.concat([signature, ihdr, chunk("raNd", Buffer.alloc(padding)), idat, iend]);
  assert.equal(bytes.length, maxBytes);
  return bytes;
}

async function exercise() {
  const testing = await import("@semio-tech/framework-os-dev/testing/program-matrix");
  const owner = resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser");
  const { ensureRepositoryBrowserPathV1 } = await import(join(owner, "🧰️preparation/🟦️.ts"));
  const { PLAYWRIGHT_MODULE_SPECIFIER } = await import(join(owner, "🟦️.ts"));
  ensureRepositoryBrowserPathV1(process.cwd(), process.env);
  const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  const work = join(output, "cancel-browser-temp");
  mkdirSync(work, { recursive: true });
  const browser = await chromium.launch({
    args: process.platform === "darwin" ? ["--in-process-gpu", "--disable-accelerated-2d-canvas", "--use-angle=swiftshader"] : [],
    headless: true,
    downloadsPath: output,
    env: { ...process.env, TMPDIR: work, TMP: work, TEMP: work },
  });
  try {
    const page = await browser.newPage({ locale: "en-US", viewport: { width: 1600, height: 1000 }, acceptDownloads: true });
    const rail = '[data-slot="window-action-pane"]';
    const row = (id: string) => rail + ' [id="action.' + id + '"]';
    const click = async (id: string) => {
      await testing.unfoldActionsRail(page);
      await testing.clickUncovered(page, row(id));
    };
    const choose = async (activate: () => Promise<unknown>, file: string | { name: string; mimeType: string; buffer: Buffer }) => {
      const [picker] = await Promise.all([page.waitForEvent("filechooser", { timeout: 60_000 }), activate()]);
      await picker.setFiles(file);
    };
    const save = async (name: string) => {
      const [download] = await Promise.all([page.waitForEvent("download", { timeout: 60_000 }), click("saveArtifactFile")]);
      const destination = join(output, name);
      await download.saveAs(destination);
      assert.equal(await download.failure(), null);
      return readFileSync(destination);
    };

    await page.goto(url, { waitUntil: "commit", timeout: 300_000 });
    assert((await testing.awaitBeacon(page, Date.now() + 300_000))?.startsWith("ready"));
    await testing.dismissIntroduction(page);
    assert.equal(await testing.seatLocale(page, "en"), "en=en");
    await testing.unfoldActionsRail(page);
    if (!await page.locator(row("openArtifactFile")).first().isVisible()) {
      await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
      await page.locator("[role=dialog] [data-slot=command-input]").first().fill("pdf");
      await page.locator('[data-command-item-id="spawn.stdio-pdf.s.stdio.pdf@1.7/*#editor"]').click();
    }
    const previous = await testing.windowIds(page);
    await choose(() => click("openArtifactFile"), join(output, "image-source.pdf"));
    const windows = await page.waitForFunction(known => {
      const added = [...document.querySelectorAll('[data-slot="window"]')].filter(window => window.id && !known.includes(window.id));
      return added.some(window => window.querySelector('[data-ui-node-key="pdf-text-layer-page-1"]')) && added.map(window => window.id);
    }, previous, { timeout: 120_000 });
    const ids = await windows.jsonValue();
    assert(Array.isArray(ids));
    const owned = page.locator(ids.map(id => '[data-slot="window"][id=' + JSON.stringify(id) + ']').join(","));
    const placements = owned.locator('[data-ui-node-key^="pdf-text-layer-p0-"]').and(owned.getByRole("button", { name: /^Image(?:$|: )/u }));
    await placements.nth(1).waitFor({ timeout: 60_000 });
    const canvas = await owned.locator('[data-ui-node-key="pdf.page"]').elementHandle({ timeout: 60_000 });
    const baseline = await save("decode-cancellation-before.pdf");
    const image = { name: "decode-cancellation.png", mimeType: "image/png", buffer: pngBytes };
    const prepareInsert = async () => {
      await click("insert-image");
      const matrix: number[] = corpus.existing.insertPlacement;
      for (const [field, value] of Object.entries({ page: 0, x: matrix[4], y: matrix[5], width: matrix[0], height: matrix[3] })) {
        const input = page.locator(row("insert-image.arg." + field)).locator("input").first();
        await input.fill(String(value));
        await input.blur();
      }
      return page.locator(rail).getByRole("button", { name: "Execute", exact: true });
    };
    const execute = await prepareInsert();
    const control = owned.locator('[data-ui-node-key="pdf-object"] [data-ui-node-key^="framework.operation."][data-ui-node-key$=".cancel"]')
      .and(owned.getByRole("button", { name: "Cancel", exact: true }));
    const press = cancelVisibleDecode(owned, control, canvas);
    const [key] = await Promise.all([press, choose(() => execute.click(), image)]);
    await owned.locator('[data-ui-node-key=' + JSON.stringify(key) + ']').waitFor({ state: "detached", timeout: 60_000 });
    assert.equal(await placements.count(), 2);
    assert(await canvas.evaluate(node => node.isConnected), "the original Page canvas stays mounted after cancellation");
    assert.deepEqual(await save("decode-cancellation-after.pdf"), baseline);
    const retry = await prepareInsert();
    await choose(() => retry.click(), image);
    await placements.nth(2).waitFor({ timeout: 60_000 });
    await click("undo");
    await placements.nth(2).waitFor({ state: "detached", timeout: 60_000 });
    assert.deepEqual(await save("decode-cancellation-undone.pdf"), baseline);
    console.log("[DEBUG] Mounted PDF cancelled one active bounded PNG decode, preserved downloaded bytes, and undid an uncancelled retry");
  } finally {
    await browser.close();
  }
}

async function cancelVisibleDecode(owned: Locator, control: Locator, canvas: ElementHandle): Promise<string> {
  await control.waitFor({ state: "visible", timeout: 60_000 });
  const id = await control.getAttribute("data-ui-node-key");
  assert.match(id ?? "", /^framework\.operation\.[0-9a-f]{16}\.cancel$/u);
  const key = id!.slice(0, -".cancel".length);
  assert.equal(await owned.locator('[data-ui-node-key=' + JSON.stringify(key + ".progress") + ']')
    .and(owned.getByRole("progressbar")).count(), 1);
  assert(await canvas.evaluate(node => node.isConnected), "the original Page canvas stays mounted while progress is visible");
  await control.press("Enter");
  return key;
}
