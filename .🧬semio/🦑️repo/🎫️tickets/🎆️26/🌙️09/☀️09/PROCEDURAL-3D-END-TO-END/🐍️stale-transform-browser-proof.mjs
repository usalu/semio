/** 🌐️ End-to-end proof in the real browser: boot the procedural 3d playground, save a deep host module
 * the way an editor does (write a temporary file, rename it onto the target), reload, and require the
 * edited module's own line in the page console. A stale serve reloads yesterday's module and prints
 * nothing.
 * @see 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts */
import { chromium } from "playwright";
import { readFileSync, writeFileSync, renameSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const port = Number(process.env.SEMIO_PROBE_PORT ?? 6029);
const url = `http://127.0.0.1:${port}/?plugin=generation3d`;
const repoRoot = "/Users/ueli/Documents/semio";
const target = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx");
const MARKER = "[DEBUG] stale-guard";
const outDir = join(import.meta.dir, "🗑️generated", "stale-transform-browser-proof");
mkdirSync(outDir, { recursive: true });

const stripMarkers = () => {
  const current = readFileSync(target, "utf8");
  const cleaned = current.split("\n").filter((line) => !line.includes(MARKER)).join("\n");
  if (cleaned !== current) writeFileSync(target, cleaned);
};
const atomicAppend = (line) => {
  const temporary = `${target}.stale-guard-tmp`;
  writeFileSync(temporary, `${readFileSync(target, "utf8")}${line}\n`);
  renameSync(temporary, target);
};

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
let lines = [];
page.on("console", (message) => lines.push(message.text().slice(0, 600)));
page.on("pageerror", (error) => lines.push(`pageerror ${String(error).slice(0, 600)}`));

await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(25_000);
const freshnessBanner = lines.find((line) => line.includes("transform freshness"));
const bootErrors = lines.filter((line) => line.startsWith("pageerror")).length;
console.log(`boot: ${lines.length} console lines, ${bootErrors} page errors`);
console.log(`banner: ${freshnessBanner ?? "MISSING"}`);

const stamp = `${MARKER} browser ${Date.now()}`;
atomicAppend(`console.info(${JSON.stringify(stamp)});`);
lines = [];
await page.reload({ waitUntil: "domcontentloaded" });
await page.waitForTimeout(25_000);
const sawEdit = lines.some((line) => line.includes(stamp));
const reloadBanner = lines.find((line) => line.includes("transform freshness"));
console.log(`after atomic save + reload: edited module executed = ${sawEdit ? "YES (fresh)" : "NO (STALE)"}`);
console.log(`banner: ${reloadBanner ?? "MISSING"}`);
await page.screenshot({ path: join(outDir, "after-edit.png"), type: "png" });

stripMarkers();
lines = [];
await page.reload({ waitUntil: "domcontentloaded" });
await page.waitForTimeout(20_000);
console.log(`after removing the debug line: still executed = ${lines.some((line) => line.includes(stamp)) ? "YES (STALE)" : "NO (fresh)"}`);
console.log(`final page errors: ${lines.filter((line) => line.startsWith("pageerror")).length}`);
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log(`marker residue: ${readFileSync(target, "utf8").includes(MARKER)}`);
await browser.close();
