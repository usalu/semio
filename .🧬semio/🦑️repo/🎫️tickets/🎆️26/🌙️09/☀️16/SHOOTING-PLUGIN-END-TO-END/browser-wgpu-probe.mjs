/** 📸️ Verifies Shooting through the browser WGPU canvas and its real accessibility input bridge. */
import assert from "node:assert/strict";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";
import sharp from "sharp";
import { loadPlaywrightV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import { findWorkspaceRoot } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { browserLaunchOptionsV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🟦️.ts";

const output = join(import.meta.dir, "🗑️generated", "browser-wgpu");
await mkdir(output, { recursive: true });
const { chromium } = await loadPlaywrightV1(findWorkspaceRoot());
const launch = browserLaunchOptionsV1();
const angle = { darwin: "metal", win32: "d3d11" }[process.platform] ?? "vulkan";
const browser = await chromium.launch({
  ...launch,
  args: [...(launch.args ?? []), "--enable-unsafe-webgpu", "--ignore-gpu-blocklist", `--use-angle=${angle}`],
});
const page = await browser.newPage({ viewport: {width: 1600, height: 1000}, locale: "en-US", acceptDownloads: true });
const logs = [];
page.on("console", event => logs.push(`${event.type()} ${event.text()}`));
page.on("pageerror", error => logs.push(`pageerror ${error.message}`));
await page.addInitScript(() => localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"));
const mirror = page.locator("#semio-wgpu-accessibility");
const chrome = () => page.evaluate(async () => JSON.parse(await window.semioWgpuIntrospection.dumpChrome()));
const activate = async control => {
  await control.waitFor({state: "attached", timeout: 30_000});
  await control.dispatchEvent("click");
};
const named = (role, name) => mirror.getByRole(role, {name, exact: true, includeHidden: true}).first();
const select = async (label, option) => {
  const control = named("combobox", label);
  await activate(control);
  await control.and(mirror.locator('[aria-expanded="true"]')).waitFor({state: "attached", timeout: 30_000});
  const choice = named("option", option);
  await activate(choice);
  await choice.waitFor({state: "detached", timeout: 30_000});
};
const openInspector = async () => {
  if (await named("combobox", "Projection").count()) return;
  const panel = mirror.locator('[data-node-key="framework.panel.inspection"]').first();
  if (await panel.count()) {
    await activate(panel);
  } else {
    const hit = (await chrome()).hits.find(target => target.controlId === "framework.panel.inspection");
    assert.ok(hit, "inspector chrome control");
    const [x, y, width, height] = hit.rect;
    await page.mouse.click(x + width / 2, y + height / 2);
  }
};
const invoke = async label => {
  const command = named("button", label);
  if (await command.count()) {
    await activate(command);
    return;
  }
  await page.locator("canvas").first().focus();
  await page.keyboard.press(process.platform === "darwin" ? "Meta+k" : "Control+k");
  const input = mirror.locator('input[role="combobox"]').last();
  await input.waitFor({state: "attached", timeout: 30_000});
  await input.fill(label, {force: true});
  await activate(named("option", label));
};
const download = async (command, filename) => {
  const delivery = page.waitForEvent("download", {predicate: item => item.suggestedFilename() === filename, timeout: 90_000});
  await invoke(command);
  const result = await delivery;
  const path = join(output, filename);
  await result.saveAs(path);
  return readFile(path);
};
const fillColor = "#ce17c5";
const targetRgb = [206, 23, 197];
const countFill = async pixels => {
  const rgba = await sharp(pixels).ensureAlpha().raw().toBuffer();
  let count = 0;
  for (let index = 0; index < rgba.length; index += 4) {
    if (rgba[index + 3] >= 240 && targetRgb.every((channel, offset) => Math.abs(rgba[index + offset] - channel) <= 18)) count++;
  }
  return count;
};

try {
  const url = new URL(process.env.SEMIO_WGPU_PROBE_URL ?? "http://127.0.0.1:6119/");
  url.searchParams.set("plugin", "shooting");
  url.searchParams.set("example", "base-icon");
  await page.goto(url.href, {waitUntil: "domcontentloaded"});
  const boot = await page.waitForFunction(() => {
    const html = document.documentElement;
    const ready = html.getAttribute("data-semio-os-ready");
    const fault = html.getAttribute("data-semio-os-error");
    return ready !== null || fault !== null ? { ready, fault } : null;
  }, undefined, {timeout: 180_000});
  const {ready, fault} = await boot.jsonValue();
  assert.equal(fault, null, `Shooting boot reported ${fault}`);
  assert.equal(ready, "shooting", "Shooting reaches the ready beacon");
  const skip = page.getByRole("button", {name: "Skip", exact: true}).first();
  if (await skip.count() && await skip.isVisible()) await skip.click();
  await page.waitForFunction(async () => {
    const bridge = window.semioWgpuIntrospection;
    if (!bridge) return false;
    const state = JSON.parse(await bridge.dumpChrome());
    return state.surfaces.some(surface => surface.id.includes("shooting-icon") || surface.elementId.includes("shooting-icon"));
  }, null, {timeout: 90_000});
  const canvas = page.locator("canvas").first();
  const baseline = await countFill(await canvas.screenshot());
  const initial = JSON.parse(await download("Save Download", "shooting.shooting.ops"));
  await openInspector();
  await select("Projection", "Orthographic");
  await select("Framing", "Fit model");
  await select("Hidden lines", "On");
  await select("Merge connected lines", "On");
  const color = named("textbox", "Fill color (empty for material)");
  await color.fill(fillColor, {force: true});
  await color.press("Enter");
  const saved = JSON.parse(await download("Save Download", "shooting.shooting.ops"));
  const shot = saved.shots.find(item => item.id === saved.activeShotId);
  assert.equal(shot.id, initial.activeShotId, "editing keeps the selected shot");
  assert.equal(shot.drawing.fillColor, fillColor);
  assert.equal(shot.drawing.hiddenLines, true);
  assert.equal(shot.autoFit, true);
  assert.equal(shot.drawing.mergeLines, true);
  assert.equal(saved.savedCameras.find(camera => camera.id === shot.cameraId)?.camera.projection, "orthographic");
  let filled = 0;
  for (let attempt = 0; attempt < 60 && filled <= baseline + 100; attempt++) {
    filled = await countFill(await canvas.screenshot());
    if (filled <= baseline + 100) await page.waitForTimeout(500);
  }
  assert.ok(filled > baseline + 100, "styled SVG ink appears in a presented WGPU canvas frame");
  const svg = await download("Export Active Shot", `${shot.id}.svg`);
  assert.match(svg.toString("utf8"), /id="hidden-lines"/u);
  assert.equal((await sharp(svg).metadata()).width, shot.width);
  assert.ok(await countFill(await sharp(svg).png().toBuffer()) > 100, "download uses the same fill");
  assert.ok(!logs.some(line => line.startsWith("pageerror ")), "no uncaught page errors");
  await writeFile(join(output, "report.json"), JSON.stringify({ baseline, filled, shot: shot.id }, null, 2));
} finally {
  await writeFile(join(output, "console.txt"), logs.join("\n"));
  await page.screenshot({path: join(output, "final.png")}).catch(() => {});
  await browser.close();
}
