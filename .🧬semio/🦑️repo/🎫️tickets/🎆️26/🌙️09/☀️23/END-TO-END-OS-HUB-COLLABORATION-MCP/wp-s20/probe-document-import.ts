/** 🔬️ S20 ticket probe: opens one editor from the palette, Export Document → Import Document through the file chooser, then
 * samples the Tasks window's document-transfer row and the console for `seconds` — shows whether an archive load stays
 * pending/running and whether its progress moves.
 * Usage: bun probe-document-import.ts <serve> <pluginId> <appId> <outDir> [seconds] [archive to import instead of seating + exporting] */
import { mkdirSync } from "node:fs";
import { join } from "node:path";
import { chromium } from "playwright";
import { awaitBeacon, dismissIntroduction, windowIds } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";

const [base, pluginId, appId, outDir, secondsText, givenArchive] = process.argv.slice(2);
const seconds = Number(secondsText ?? 90);
mkdirSync(outDir!, { recursive: true });
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 }, acceptDownloads: true })).newPage();
const lines: string[] = [];
const stamp = (): string => new Date().toISOString().slice(11, 23);
let pending: string | null = null;
page.on("console", (message) => lines.push(`${stamp()} ${message.type()}: ${message.text()}`.slice(0, 500)));
page.on("pageerror", (error) => lines.push(`${stamp()} pageerror: ${String(error)}`.slice(0, 500)));
page.on("filechooser", (chooser) => {
  lines.push(`${stamp()} FILECHOOSER ${pending ?? "none"}`);
  void chooser.setFiles(pending === null ? [] : [pending]);
  pending = null;
});
const palette = async (query: string, item: string): Promise<boolean> => {
  await dismissIntroduction(page);
  await page.evaluate(() => {
    if (document.activeElement instanceof HTMLElement) document.activeElement.blur();
    document.body.focus();
  });
  await page.keyboard.press("Meta+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 15_000 });
  await input.fill(query);
  const row = page.locator(item.split("|").map((id) => `[data-slot="command-item"][data-command-item-id="${id}"]`).join(", ")).first();
  await row.waitFor({ state: "visible", timeout: 8_000 }).catch(() => undefined);
  if ((await row.count()) === 0) {
    console.log("palette rows", JSON.stringify(await page.locator('[data-slot="command-item"]').evaluateAll((rows) => rows.map((element) => element.getAttribute("data-command-item-id")))));
    await page.keyboard.press("Escape");
    return false;
  }
  await row.click({ force: true });
  return true;
};
await page.goto(base!, { waitUntil: "commit" });
console.log("beacon", await awaitBeacon(page, Date.now() + 300_000));
await dismissIntroduction(page);
for (let second = 0; second < 600; second += 3) {
  const probe = (await page.evaluate(() => (window as unknown as { __semioOsCatalogProbe?: { plugins: { pluginId: string; status: string }[]; programs: { pluginId: string; appId: string }[] } }).__semioOsCatalogProbe ?? null)) as { plugins: { status: string }[]; programs: { pluginId: string; appId: string }[] } | null;
  if (probe !== null && probe.plugins.length > 0 && probe.plugins.every((entry) => ["loaded", "failed", "crashed", "available"].includes(entry.status))) {
    console.log("catalog", probe.plugins.length, JSON.stringify(probe.programs.filter((program) => program.appId === appId)));
    break;
  }
  await page.waitForTimeout(3_000);
}
await page.waitForTimeout(3_000);
console.log("open", await palette(/^s\.[^.]+\.([^@]+)@/u.exec(appId!)?.[1] ?? pluginId!, `spawn.${pluginId}.${appId}|spawn.${pluginId}`));
for (let index = 0; index < 90 && (await windowIds(page)).length < 2; index++) await page.waitForTimeout(1_000);
await page.waitForTimeout(6_000);
const picker = page.locator('[id="playground.navbar.fixture"]').first();
if (givenArchive === undefined && (await picker.count()) > 0) {
  await picker.click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(800);
  const options = await page.locator('[role="option"]').evaluateAll((rows) => rows.map((row) => ({ value: row.getAttribute("data-value") ?? "", text: (row.textContent ?? "").trim() })));
  const wanted = options.findIndex((option) => option.value !== "" && !/^_+none_+$/u.test(option.value) && !/^(empty|leer|none|keine)$/iu.test(option.text));
  console.log("example", wanted >= 0 ? JSON.stringify(options[wanted]) : "none");
  if (wanted >= 0) await page.locator('[role="option"]').nth(wanted).click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(8_000);
}
let archive = givenArchive ?? "";
if (givenArchive === undefined) {
  const downloadWait = page.waitForEvent("download", { timeout: 60_000 });
  console.log("export", await palette("Export Document", "command.os.os.exportDocument"));
  const download = await downloadWait;
  archive = join(outDir!, download.suggestedFilename());
  await download.saveAs(archive);
}
console.log("archive", archive);
pending = archive;
const cursor = lines.length;
console.log("import", await palette("Import Document", "command.os.os.importDocument"));
await page.waitForTimeout(3_000);
console.log("tasks", await palette("Open Tasks", "command.os.os.openTaskManager"));
for (let second = 0; second < seconds; second += 5) {
  const rows = await page.locator('[data-slot="task-manager-task-progress-text"]').allTextContents().catch(() => []);
  const notice = await page.locator("[data-semio-transient-notice]").first().getAttribute("data-notice-code").catch(() => null);
  console.log(`t+${second}s tasks=${JSON.stringify(rows)} notice=${notice}`);
  if (notice?.startsWith("shell.documentTransfer.import")) break;
  if (notice === "shell.documentTransfer.imported") break;
  await page.waitForTimeout(5_000);
}
await page.screenshot({ path: join(outDir!, "probe.png") });
console.log(lines.slice(cursor).join("\n"));
await browser.close();
