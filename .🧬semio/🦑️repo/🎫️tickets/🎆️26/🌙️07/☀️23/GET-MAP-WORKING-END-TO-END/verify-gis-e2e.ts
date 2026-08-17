#!/usr/bin/env bun
/** 🧪️ GIS map react e2e: playground boots with canvas/chrome and no fatal errors. */

import { chromium } from "playwright";

const baseUrl = process.env.GIS_URL ?? "http://127.0.0.1:6040/";
const expectTitle = process.env.GIS_EXPECT_TITLE ?? "gis";
const outLog = process.env.GIS_VERIFY_LOG;

async function main(): Promise<void> {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  const pageErrors: string[] = [];
  const consoleErrors: string[] = [];
  page.on("pageerror", (err) => pageErrors.push(String(err)));
  page.on("console", (msg) => {
    if (msg.type() === "error") consoleErrors.push(msg.text());
  });
  await page.goto(baseUrl, { waitUntil: "networkidle", timeout: 120_000 });
  await page.waitForTimeout(10_000);
  const title = await page.title();
  const bodyText = await page.locator("body").innerText();
  const canvas = await page.locator("canvas").count();
  const chromeOk =
    /Document|Map|Layer|LOD|Selection|Catalogue|Inspection|View/i.test(bodyText);
  const fatal = /Could not load|Unexpected token '<'|Render error|failed to instantiate|missing field/i.test(bodyText);
  const actionablePageErrors = pageErrors.filter(
    (line) => !/NoCompatibleDevice|Failed to fetch dynamically imported module/i.test(line),
  );
  const summary = {
    title,
    canvas,
    chromeOk,
    fatal,
    pageErrors,
    actionablePageErrors,
    consoleErrors: consoleErrors.slice(0, 30),
    bodySnippet: bodyText.slice(0, 800),
  };
  console.log("[DEBUG] gis page", JSON.stringify(summary, null, 2));
  if (outLog) await Bun.write(outLog, `${JSON.stringify(summary, null, 2)}\n`);
  await browser.close();
  if (!title.toLowerCase().includes(expectTitle.toLowerCase()) && !title.toLowerCase().includes("semio")) {
    throw new Error(`unexpected title: ${title}`);
  }
  if (fatal) throw new Error("fatal render text still present");
  if (!chromeOk) throw new Error("expected gis chrome missing from body");
  if (actionablePageErrors.length > 0) throw new Error(`page errors: ${actionablePageErrors.join(" | ")}`);
  if (canvas < 1) throw new Error("no canvas");
  console.log("[DEBUG] gis e2e ok");
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
