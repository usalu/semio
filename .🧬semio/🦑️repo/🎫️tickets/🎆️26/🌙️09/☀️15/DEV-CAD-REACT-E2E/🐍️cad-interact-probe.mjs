import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

// 🧪️ Boot the cad react dev page, read every world-3d host's lanes, pick one object per pane through the
// framework `cad` interaction domain and record the settled selection + a screenshot.
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6020/?plugin=cad";
const bootSeconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 45);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "interact");
mkdirSync(outDir, { recursive: true });
const lines = [];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const t0 = Date.now();
page.on("console", (msg) => lines.push(`${Date.now() - t0} ${msg.type()} ${msg.text().slice(0, 1500)}`));
page.on("pageerror", (err) => lines.push(`${Date.now() - t0} pageerror ${String(err).slice(0, 1500)}`));
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(bootSeconds * 1000);
const readHosts = () => page.evaluate(() => [...document.querySelectorAll(".semio-world-3d-host")].map((el) => {
  const parse = (name) => { try { return JSON.parse(el.getAttribute(name) ?? "null"); } catch { return null; } };
  const meshes = parse("data-meshes-json") ?? [];
  const instances = parse("data-instances-json") ?? [];
  const rect = el.getBoundingClientRect();
  return {
    surfaceId: el.getAttribute("data-surface-id"),
    rect: { x: rect.x, y: rect.y, w: rect.width, h: rect.height },
    meshes: meshes.length,
    inlineMeshes: meshes.filter((m) => m && m.data).length,
    kindMeshes: meshes.filter((m) => m && m.kind).length,
    positions: meshes.reduce((n, m) => n + ((m && m.data && m.data.positions && m.data.positions.length) || 0), 0),
    instances: instances.length,
    instanceIds: instances.slice(0, 3).map((i) => i.id),
    selection: parse("data-selection-json"),
    camera: el.getAttribute("data-viewport-camera-json")?.slice(0, 300),
    referenceImages: [...el.querySelectorAll("img")].map((img) => ({ src: img.getAttribute("src"), complete: img.complete, w: img.naturalWidth })),
  };
}));
const before = await readHosts();
lines.push(`${Date.now() - t0} probe hosts-before ${JSON.stringify(before)}`);
await page.screenshot({ path: join(outDir, "boot.png"), type: "png" });
const picks = [];
const hoverAt = async (surfaceId, x, y) => {
  await page.mouse.move(x, y);
  await page.waitForTimeout(350);
  const host = (await readHosts()).find((h) => h.surfaceId === surfaceId);
  return host?.selection?.hoverTarget ?? null;
};
for (const host of before) {
  // 🎯️ The `cad` domain hover is shared by all four panes (a stale hover target from the previous pane
  // is not a hit here), so pick by CLICKING grid points until the domain selection names an object.
  let hit = null;
  outer: for (let row = 1; row < 5; row += 1) {
    for (let col = 1; col < 7; col += 1) {
      const x = host.rect.x + (host.rect.w * col) / 7;
      const y = host.rect.y + (host.rect.h * row) / 5;
      await page.mouse.move(x, y);
      await page.waitForTimeout(200);
      await page.mouse.click(x, y);
      await page.waitForTimeout(1200);
      const now = (await readHosts()).find((h) => h.surfaceId === host.surfaceId);
      const ids = now?.selection?.selectedIds ?? [];
      if (ids.length > 0) { hit = { x, y, ids, hoverTarget: now?.selection?.hoverTarget }; break outer; }
    }
  }
  const inspection = await page.evaluate(() => {
    const root = document.querySelector('[data-panel-tab-id="framework.panel.inspection"], [data-panel-id="framework.panel.inspection"]') ?? [...document.querySelectorAll("[data-tree-id], [role=tree]")].find((el) => el.textContent?.includes("cad-play-inspector") || el.getAttribute("data-tree-id")?.includes("inspector"));
    return root?.textContent?.slice(0, 400) ?? null;
  });
  const inspectorRows = await page.evaluate(() => [...document.querySelectorAll('[data-item-id^="cad-play-inspector."]')].map((el) => `${el.getAttribute("data-item-id")}=${el.textContent?.trim().slice(0, 80)}`).slice(0, 20));
  const treeMarks = await page.evaluate(() => [...document.querySelectorAll('[data-item-id][aria-selected="true"], [data-item-id][data-selected="true"], [data-item-id].selected')].map((el) => el.getAttribute("data-item-id")).slice(0, 10));
  picks.push({ surfaceId: host.surfaceId, hit, inspection, inspectorRows, treeMarks });
  lines.push(`${Date.now() - t0} probe pick ${JSON.stringify(picks.at(-1))}`);
  if (hit) await page.screenshot({ path: join(outDir, `pick-${host.surfaceId.replace(/[^a-z0-9-]/gi, "_")}.png`), type: "png" });
}
await page.screenshot({ path: join(outDir, "after-picks.png"), type: "png" });
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "hosts.json"), JSON.stringify({ before, picks }, null, 2));
const faults = lines.filter((l) => /settle|fault|pageerror|Unknown action|unhandled|ui\.fixed-capacity|error \[DEBUG\] .*fail/i.test(l) && !/contributions/.test(l));
console.log("DONE lines", lines.length, "faults", faults.length);
for (const f of faults.slice(0, 20)) console.log("FAULT", f.slice(0, 400));
console.log("HOSTS", JSON.stringify(before.map((h) => ({ s: h.surfaceId, meshes: h.meshes, inline: h.inlineMeshes, kind: h.kindMeshes, pos: h.positions, inst: h.instances }))));
console.log("PICKS", JSON.stringify(picks));
await browser.close();
