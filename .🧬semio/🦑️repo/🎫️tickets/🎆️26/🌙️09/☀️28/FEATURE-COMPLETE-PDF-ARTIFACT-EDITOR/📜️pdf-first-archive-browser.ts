/** 🗃️ Opens one natural PDF in the mounted shell and lists the first-genesis folder archives it durably wrote (G17). */
import assert from "node:assert/strict";
import { existsSync, mkdirSync, readdirSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

const [url, outputDirectory, dataDirectory, pdf] = process.argv.slice(2);
assert(url && outputDirectory && dataDirectory && pdf, "Expected shell URL, output directory, S_DATA_DIR and a PDF to open");
const output = resolve(outputDirectory);
const documents = join(resolve(dataDirectory), "os", "local-documents");
mkdirSync(output, { recursive: true });
const archives = () => existsSync(documents)
  ? readdirSync(documents).map(name => join(documents, name, ".semio", "events.semio")).filter(existsSync).sort() : [];
const before = archives();
const testing = await import("@semio-tech/framework-os-dev/testing/program-matrix");
const owner = resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser");
const { ensureRepositoryBrowserPathV1 } = await import(join(owner, "🧰️preparation/🟦️.ts"));
const { PLAYWRIGHT_MODULE_SPECIFIER } = await import(join(owner, "🟦️.ts"));
ensureRepositoryBrowserPathV1(process.cwd(), process.env);
const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
const work = join(output, "first-archive-browser-temp");
mkdirSync(work, { recursive: true });
const browser = await chromium.launch({
  args: process.platform === "darwin" ? ["--in-process-gpu", "--disable-accelerated-2d-canvas", "--use-angle=swiftshader"] : [],
  headless: true,
  env: { ...process.env, TMPDIR: work, TMP: work, TEMP: work },
});
try {
  const page = await browser.newPage({ locale: "en-US", viewport: { width: 1600, height: 1000 } });
  const rail = '[data-slot="window-action-pane"]';
  const row = (id: string) => rail + ' [id="action.' + id + '"]';
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
  const [picker] = await Promise.all([page.waitForEvent("filechooser", { timeout: 60_000 }),
    testing.unfoldActionsRail(page).then(() => testing.clickUncovered(page, row("openArtifactFile")))]);
  await picker.setFiles(resolve(pdf));
  await page.waitForFunction(known => [...document.querySelectorAll('[data-slot="window"]')]
    .some(window => window.id && !known.includes(window.id) && window.querySelector('[data-ui-node-key="pdf-text-layer-page-1"]')), previous, { timeout: 120_000 });
  const deadline = Date.now() + 60_000;
  while (archives().length === before.length && Date.now() < deadline) await Bun.sleep(500);
  const after = archives();
  const added = after.filter(path => !before.includes(path));
  writeFileSync(join(output, "first-archive-listing.json"), JSON.stringify({ before, after, added }, null, 2));
  assert.equal(added.length, 1, "opening one natural PDF must durably write exactly one new first-genesis folder archive");
  console.log("[DEBUG] Mounted open wrote one first-genesis folder archive", added[0]);
} finally {
  await browser.close();
}
