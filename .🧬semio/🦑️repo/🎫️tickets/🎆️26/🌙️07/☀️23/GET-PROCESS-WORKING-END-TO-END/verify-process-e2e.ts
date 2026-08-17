#!/usr/bin/env bun
/** 🧪️ Process 3D react e2e: playground boots, Process chrome visible, canvas present, no render error. */

import { chromium } from "playwright";
import { writeFileSync } from "node:fs";

const baseUrl = process.env.PROCESS_URL ?? "http://127.0.0.1:6022/";
const outDir = "/Users/ueli/Documents/semio/.repo/🎫️/26/07/23/GET-PROCESS-WORKING-END-TO-END";

async function main(): Promise<void> {
  const browser = await chromium.launch({
    executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
    headless: true,
  });
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
  const pageErrors: string[] = [];
  const consoleMsgs: { type: string; text: string }[] = [];
  page.on("pageerror", (err) => pageErrors.push(String(err)));
  page.on("console", (msg) => {
    const text = msg.text();
    if (msg.type() === "error" || text.includes("[DEBUG]") || /process|wasm|plugin/i.test(text)) {
      consoleMsgs.push({ type: msg.type(), text: text.slice(0, 500) });
    }
  });

  console.log("[DEBUG] goto", baseUrl);
  await page.goto(baseUrl, { waitUntil: "networkidle", timeout: 120_000 });
  await page.waitForTimeout(12_000);

  const bodyText = await page.locator("body").innerText();
  const hasRenderError = /Render error|Renderfehler|Could not load|Unexpected token/i.test(bodyText);
  const canvas = await page.locator("canvas").count();
  const mentionsProcess = /Process|Workpiece|Stock|Rohteil|Cut|Drill|Attach/i.test(bodyText);
  const windowTitles = await page.locator('[data-slot="window"]').evaluateAll((nodes) =>
    nodes.map((n) => (n.textContent ?? "").slice(0, 120).replace(/\s+/g, " ")),
  );

  await page.screenshot({ path: `${outDir}/e2e-process.png`, fullPage: false });

  const summary = {
    canvas,
    hasRenderError,
    mentionsProcess,
    windowTitles,
    pageErrors,
    consoleSample: consoleMsgs.slice(0, 40),
    bodySnippet: bodyText.slice(0, 800),
  };
  writeFileSync(`${outDir}/e2e-report.json`, `${JSON.stringify(summary, null, 2)}\n`);
  console.log("[DEBUG] process page", JSON.stringify(summary, null, 2));
  await browser.close();

  if (hasRenderError) throw new Error("render error present");
  if (pageErrors.length > 0) throw new Error(`page errors: ${pageErrors.join(" | ")}`);
  if (canvas < 1) throw new Error("no canvas");
  if (!mentionsProcess) throw new Error("process chrome not visible in body text");
  console.log("[DEBUG] process e2e ok");
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
