/** 🖱️ F3 session 14c — per-pane hover/selection smoke after the leftover structural-sharing fix: opens puzzle3d, finds a
 * hoverable point in MainPerspective, then reads every world pane's `data-selection-json` on hover, after the pointer
 * leaves, and after a click (the hovered pane must name the hover and its siblings must not; nobody keeps a stale hover;
 * the click's selection reaches every pane). usage: bun f3-world-hover-panes.ts <baseUrl> */
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
  const panes = () => page.evaluate(() => [...document.querySelectorAll(".semio-world-3d-host")].map((element) => {
    const json = element.closest("[data-selection-json]")?.getAttribute("data-selection-json") ?? element.querySelector("[data-selection-json]")?.getAttribute("data-selection-json") ?? "null";
    const parsed = JSON.parse(json) as { selectedIds?: string[]; hoverTarget?: { id: string } | null } | null;
    return { pane: element.closest("[id]")?.id.split("::").at(-1) ?? "?", hover: parsed?.hoverTarget?.id ?? null, selected: parsed?.selectedIds ?? null, paint: element.getAttribute("data-hover-paint-id") };
  }));
  await page.mouse.move(target.x, target.y);
  await page.waitForTimeout(1_000);
  const hovered = await panes();
  await page.mouse.move(box.x + 4, box.y + box.height - 4);
  await page.waitForTimeout(1_000);
  const left = await panes();
  await page.mouse.click(target.x, target.y);
  await page.waitForTimeout(1_500);
  const clicked = await panes();
  const main = (rows: typeof hovered) => rows.find((row) => row.pane.endsWith("MainPerspective"));
  const siblings = (rows: typeof hovered) => rows.filter((row) => !row.pane.endsWith("MainPerspective"));
  const verdict = {
    hoveredPaneNamesHover: main(hovered)?.hover === target.id || main(hovered)?.paint === target.id,
    siblingsFreeOfHover: siblings(hovered).every((row) => row.hover === null),
    noStaleHoverAfterLeave: left.every((row) => row.hover === null),
    clickReachesEveryPane: clicked.every((row) => (row.selected ?? []).includes(target!.id)),
  };
  console.log(JSON.stringify({ target: target.id, panes: hovered.length, verdict, hovered, left, clicked, errors }));
} finally {
  await browser.close();
}
