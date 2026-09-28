/** 🖱️ F3 — world canvas click-select + orbit-drag smoke after the WorldCanvas memo: opens puzzle3d, finds a hoverable point,
 * clicks it (the pane's `data-selection-json` must name it), then drags on empty space (the camera must move: the pane's
 * `data-selection-json` stays, the canvas repaints) and reports page errors. usage: bun f3-world-click.ts <baseUrl> */
import { chromium } from "playwright";
import { awaitBeacon, dismissIntroduction, windowIds } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";
const baseUrl = process.argv[2]!;
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
try {
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message.slice(0, 200)));
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  await awaitBeacon(page, Date.now() + 300_000);
  await dismissIntroduction(page);
  const before = await windowIds(page);
  await page.keyboard.press("Meta+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 15_000 });
  await input.fill("puzzle3d");
  await page.waitForTimeout(1_500);
  for (const id of ["spawn.puzzle.s.puzzle.puzzle3d@1/*#editor", "spawn.puzzle"]) {
    const item = page.locator(`[data-slot="command-item"][data-command-item-id="${id}"]`).first();
    if ((await item.count()) > 0) { await item.click({ force: true }); break; }
  }
  const deadline = Date.now() + 120_000;
  while (Date.now() < deadline && (await windowIds(page)).filter((id) => !before.includes(id)).length === 0) await page.waitForTimeout(500);
  await page.waitForTimeout(8_000);
  const selector = `[id$="MainPerspective"] .semio-world-3d-host`;
  const host = page.locator(selector).first();
  await host.waitFor({ state: "visible", timeout: 60_000 });
  const box = (await host.boundingBox())!;
  let target: { x: number; y: number; id: string } | null = null;
  for (let gy = 0.3; gy <= 0.8 && !target; gy += 0.05) for (let gx = 0.2; gx <= 0.8 && !target; gx += 0.05) {
    const x = box.x + box.width * gx, y = box.y + box.height * gy;
    await page.mouse.move(x, y);
    await page.waitForTimeout(120);
    const id = await page.evaluate((s) => document.querySelector(s)!.getAttribute("data-hover-paint-id"), selector);
    if (id) target = { x, y, id };
  }
  if (!target) throw new Error("no hoverable point");
  const selection = () => page.evaluate((s) => document.querySelector(s)?.closest("[data-selection-json]")?.getAttribute("data-selection-json") ?? document.querySelector(`${s} [data-selection-json], ${s}[data-selection-json]`)?.getAttribute("data-selection-json") ?? null, selector);
  const selectionBefore = await selection();
  await page.mouse.click(target.x, target.y);
  await page.waitForTimeout(1_500);
  const selectionAfterClick = await selection();
  const shot = async () => (await host.screenshot()).toString("base64");
  const beforeDrag = await shot();
  await page.mouse.move(box.x + box.width * 0.5, box.y + box.height * 0.85);
  await page.mouse.down({ button: "middle" });
  for (let step = 1; step <= 10; step += 1) await page.mouse.move(box.x + box.width * 0.5 + step * 12, box.y + box.height * 0.85 - step * 4);
  await page.mouse.up({ button: "middle" });
  await page.waitForTimeout(1_000);
  const afterDrag = await shot();
  console.log(JSON.stringify({ target: target.id, selectionBefore, selectionAfterClick, selectedAfterClick: (selectionAfterClick ?? "").includes(target.id), dragRepainted: beforeDrag !== afterDrag, errors }));
} finally {
  await browser.close();
}
