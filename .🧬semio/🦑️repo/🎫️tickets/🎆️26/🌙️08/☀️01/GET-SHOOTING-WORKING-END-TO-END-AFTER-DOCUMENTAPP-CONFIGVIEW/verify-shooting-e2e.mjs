#!/usr/bin/env bun
/** [DEBUG] Shooting e2e after DocumentApp ConfigView / RefCell migration. */
import { chromium } from "playwright";
import { join } from "node:path";

const baseUrl = process.env.SHOOTING_URL ?? "http://127.0.0.1:6019/";
const ticketDir = process.env.SHOOTING_TICKET_DIR;
const outPath = process.env.SHOOTING_VERIFY_OUT ?? (ticketDir ? join(ticketDir, "verify-result.json") : undefined);
const shotPath = process.env.SHOOTING_SCREENSHOT ?? (ticketDir ? join(ticketDir, "shooting-viewport.png") : "shooting-viewport.png");

async function probeMesh(path) {
  const res = await fetch(new URL(path, baseUrl));
  const ct = res.headers.get("content-type");
  const bytes = new Uint8Array(await res.arrayBuffer());
  const magic = bytes.length >= 4 ? String.fromCharCode(bytes[0], bytes[1], bytes[2], bytes[3]) : "";
  return { path, ok: res.ok, status: res.status, contentType: ct, bytes: bytes.byteLength, magic };
}

async function main() {
  const meshCandidates = ["/mesh/🧊️base.glb", "/mesh/base.glb", "/mesh/placeholder.glb", "/mesh/🧊️placeholder.glb"];
  const meshes = [];
  for (const path of meshCandidates) {
    try { meshes.push(await probeMesh(path)); } catch (e) { meshes.push({ path, error: String(e) }); }
  }
  console.log("[DEBUG] mesh probes", JSON.stringify(meshes, null, 2));

  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  const pageErrors = [];
  const consoleErrors = [];
  page.on("pageerror", (err) => pageErrors.push(String(err)));
  page.on("console", (msg) => {
    if (msg.type() === "error") consoleErrors.push(msg.text());
  });
  await page.goto(baseUrl, { waitUntil: "networkidle", timeout: 180_000 });
  await page.waitForTimeout(10_000);
  const bodyText = await page.locator("body").innerText();
  const canvas = await page.locator("canvas").count();
  const hasRenderError = /Could not load \/mesh\/|Unexpected token '<'|Render error/i.test(bodyText);
  const summary = {
    url: baseUrl,
    canvas,
    hasRenderError,
    pageErrors,
    consoleErrors: consoleErrors.slice(0, 40),
    meshOk: meshes.some((m) => m.ok && m.magic === "glTF"),
    meshes,
    bodySnippet: bodyText.slice(0, 600),
  };
  console.log("[DEBUG] shooting page", JSON.stringify(summary, null, 2));
  if (outPath) await Bun.write(outPath, `${JSON.stringify(summary, null, 2)}\n`);
  await page.screenshot({ path: shotPath, fullPage: true });
  console.log("[DEBUG] screenshot", shotPath);
  await browser.close();
  if (hasRenderError) throw new Error("render error still present");
  if (pageErrors.length) throw new Error(`page errors: ${pageErrors.join(" | ")}`);
  const actionFails = consoleErrors.filter((line) => /action failed|decodeAppFrame|channel-version|unsupported op format/i.test(line));
  if (actionFails.length) throw new Error(`action/protocol errors: ${actionFails.join(" | ")}`);
  if (canvas < 1) throw new Error("no canvas");
  if (!summary.meshOk) throw new Error("no mesh served as glTF");
  if (!/Scene/.test(bodyText) || !/Icon/.test(bodyText)) throw new Error("missing Scene/Icon chrome");
  console.log("[DEBUG] shooting e2e ok");
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
