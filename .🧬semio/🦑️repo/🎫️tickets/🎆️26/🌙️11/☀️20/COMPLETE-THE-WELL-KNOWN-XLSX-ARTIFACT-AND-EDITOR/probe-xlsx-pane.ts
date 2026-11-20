import { strict as assert } from "node:assert";
import { mkdir } from "node:fs/promises";
import { createRequire } from "node:module";
import { join } from "node:path";

const root = process.cwd();
const osDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const repoBrowser = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts";
const url = "http://127.0.0.1:6272/";
const response = await fetch(url, { signal: AbortSignal.timeout(5_000) });
assert.equal(response.ok, true, "the managed XLSX pane must answer");
await response.body?.cancel();
const { ensureRepositoryBrowserPathV1 } = await import(join(root, repoBrowser));
const { awaitBeacon, clickUncovered, dismissIntroduction, unfoldActionsRail } = await import(join(root, osDev, "🧪️tests/🧮️program-matrix/🟦️.ts"));
const { readXlsx } = await import(join(root, "🌎️hub/🧪️tests/🚪️io-matrix/🟦️.ts"));
const require = createRequire(join(root, osDev, "📦️packages/🟦️typescript/package.json"));
ensureRepositoryBrowserPathV1(root, process.env);
const { chromium } = require("playwright") as typeof import("playwright");
const browser = await chromium.launch();
try {
  const page = await browser.newPage({ locale: "en-US", acceptDownloads: true });
  const faults: string[] = [];
  page.on("pageerror", error => faults.push(error.message));
  page.on("console", message => { if (/^input #\d+ .* refused: /u.test(message.text())) faults.push(message.text()); });
  await page.goto(url);
  assert.match(await awaitBeacon(page, Date.now() + 120_000) ?? "", /^ready/u);
  await dismissIntroduction(page);
  const a1 = page.getByRole("textbox", { name: "A1", exact: true }).first();
  await a1.waitFor({ state: "visible", timeout: 30_000 });
  assert.equal(await a1.inputValue(), "");
  const editedText = "Native XLSX pane edit";
  await a1.fill(editedText);
  await a1.press("Enter");
  const waitForCell = (value: string) => page.waitForFunction((expected) => {
    const cell = document.querySelector<HTMLInputElement>('input[aria-label="A1"]');
    return cell?.value === expected && !cell.hasAttribute("data-draft-dirty");
  }, value);
  const clickAction = async (id: string) => {
    const selector = `[data-slot="window-action-pane"] [id="action.${id}"]`;
    if (await page.locator(selector).count() === 0) await unfoldActionsRail(page);
    assert.equal(await clickUncovered(page, selector), "ok");
  };
  const output = join(import.meta.dir, "🗑️generated");
  await mkdir(output, { recursive: true });
  const verifySavedCell = async (stage: string, value: string | null) => {
    const [download] = await Promise.all([page.waitForEvent("download", { timeout: 60_000 }), clickAction("saveArtifactFile")]);
    assert.match(download.suggestedFilename(), /\.xlsx$/iu);
    const path = join(output, `standalone-${stage}.xlsx`);
    await download.saveAs(path);
    const workbook = await readXlsx(page, await Bun.file(path).bytes(), "A1");
    assert.equal(workbook.valid, true);
    assert.deepEqual(workbook.sheetNames, ["Sheet1"]);
    assert.equal(workbook.text, value);
  };
  await waitForCell(editedText);
  await verifySavedCell("edit", editedText);
  await clickAction("undo");
  await waitForCell("");
  await verifySavedCell("undo", null);
  await clickAction("redo");
  await waitForCell(editedText);
  await verifySavedCell("redo", editedText);
  assert.deepEqual(faults, []);
  console.log("Managed XLSX editor created Sheet1 and saved its A1 edit as a readable workbook");
} finally {
  await browser.close();
}
