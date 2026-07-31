#!/usr/bin/env bun
/** 🧪️ Trinity react e2e: jack/rewrite playground boots with canvas and no fatal errors. */

import { chromium } from "playwright";

const baseUrl = process.env.TRINITY_URL ?? "http://127.0.0.1:6054/";
const expectTitle = process.env.TRINITY_EXPECT_TITLE ?? "trinity";
const outLog = process.env.TRINITY_VERIFY_LOG;

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
  await page.waitForTimeout(8_000);
  const title = await page.title();
  const bodyText = await page.locator("body").innerText();
  const canvas = await page.locator("canvas").count();
  const chromeOk =
    /Document/i.test(bodyText) &&
    /Catalogue|Inspection|Jack Query|Rewrite|Results|LHS|RHS/i.test(bodyText);
  const fatal = /Could not load|Unexpected token '<'|Render error|failed to instantiate/i.test(bodyText);
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
    consoleErrors: consoleErrors.slice(0, 20),
    bodySnippet: bodyText.slice(0, 600),
  };
  console.log("[DEBUG] trinity page", JSON.stringify(summary, null, 2));
  if (outLog) await Bun.write(outLog, `${JSON.stringify(summary, null, 2)}\n`);
  await browser.close();
  if (!title.toLowerCase().includes(expectTitle.toLowerCase()) && !title.toLowerCase().includes("semio")) {
    throw new Error(`unexpected title: ${title}`);
  }
  if (fatal) throw new Error("fatal render text still present");
  if (!chromeOk) throw new Error("expected trinity chrome missing from body");
  if (actionablePageErrors.length > 0) throw new Error(`page errors: ${actionablePageErrors.join(" | ")}`);
  if (canvas < 1) throw new Error("no canvas");
  console.log("[DEBUG] trinity e2e ok");
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
