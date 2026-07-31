#!/usr/bin/env bun
/** 🧪️ Shooting react e2e: mesh serves as glTF, scene boots without render error. */

import { chromium } from "playwright";

const baseUrl = process.env.SHOOTING_URL ?? "http://127.0.0.1:6019/";
const outLog = process.env.SHOOTING_VERIFY_LOG;

async function main(): Promise<void> {
  const mesh = await fetch(new URL("/mesh/base.glb", baseUrl));
  if (!mesh.ok) throw new Error(`mesh HTTP ${mesh.status}`);
  if (mesh.headers.get("content-type") !== "model/gltf-binary") {
    throw new Error(`unexpected content-type ${mesh.headers.get("content-type")}`);
  }
  const bytes = new Uint8Array(await mesh.arrayBuffer());
  const magic = String.fromCharCode(bytes[0]!, bytes[1]!, bytes[2]!, bytes[3]!);
  if (magic !== "glTF") throw new Error(`unexpected magic ${magic}`);
  console.log("[DEBUG] /mesh/base.glb ok", { bytes: bytes.byteLength, contentType: mesh.headers.get("content-type") });

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
  const bodyText = await page.locator("body").innerText();
  const hasRenderError = /Could not load \/mesh\/base\.glb|Unexpected token '<'|Render error/i.test(bodyText);
  const canvas = await page.locator("canvas").count();
  const summary = {
    canvas,
    hasRenderError,
    pageErrors,
    consoleErrors: consoleErrors.filter((line) => /mesh\/base\.glb|Unexpected token|GLTF|gltf/i.test(line)),
    bodySnippet: bodyText.slice(0, 400),
  };
  console.log("[DEBUG] shooting page", JSON.stringify(summary, null, 2));
  if (outLog) await Bun.write(outLog, `${JSON.stringify(summary, null, 2)}\n`);
  await browser.close();
  if (hasRenderError) throw new Error("render error still present");
  if (summary.consoleErrors.length > 0) throw new Error(`mesh-related console errors: ${summary.consoleErrors.join(" | ")}`);
  if (canvas < 1) throw new Error("no canvas");
  console.log("[DEBUG] shooting e2e ok");
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
