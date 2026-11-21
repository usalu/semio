import assert from "node:assert/strict";
import { chromium } from "playwright";

const base = "/@fs" + encodeURI(import.meta.dirname);
const browser = await chromium.launch({ channel: "chrome", headless: true });
try {
  for (const [width, height, part] of [[1440, 900, "layout"], [768, 1024, "window-rich-many"], [375, 812, "layout-mobile"]]) {
    const page = await browser.newPage({ viewport: { width, height } });
    page.on("pageerror", error => console.error("[DEBUG] OS page error", error.message));
    await page.goto("http://127.0.0.1:5481");
    console.log("[DEBUG]", await page.evaluate(async ([base, part]) => (await import(base + "/round3-os-run.js")).run(part, { wait: 300 }), [base, part]));
    if (part === "window-rich-many") {
      console.log("[DEBUG] cap rows", await page.locator('[data-slot="window-chrome-cap"]').evaluateAll(rows => rows.map(row => Array.from(row.children).map(chip => {
        const { left, right } = chip.getBoundingClientRect();
        return { slot: chip.getAttribute("data-slot"), text: chip.textContent.slice(0, 80), reach: chip.getAttribute("data-chip-reach"), abuts: chip.hasAttribute("data-chip-abut"), left, right };
      }))));
    }
    for (const [slot, edge] of [["navbar", "bottom"], ["footer", "top"]]) {
      const band = page.locator(`[data-slot="${slot}"]`).first();
      if (!await band.count()) continue;
      assert.equal(await band.getAttribute("data-shared-border-hover"), "declared");
      const getBand = () => band.evaluate((element, edge) => {
        const { x, y, width, height } = element.getBoundingClientRect();
        let path, centerline;
        for (const candidate of document.querySelectorAll("path[data-shared-border-segment], path[data-shared-border-main]")) {
          if (!candidate.getAttribute("d") || getComputedStyle(candidate).stroke === "none") continue;
          const transform = candidate.getScreenCTM();
          if (!transform) continue;
          const length = candidate.getTotalLength();
          for (let distance = 0; distance < length; distance += 4) {
            const p = candidate.getPointAtLength(distance).matrixTransform(transform);
            const q = candidate.getPointAtLength(distance + 1).matrixTransform(transform);
            if (Math.abs(p.x - (x + width / 2)) > 4 || p.y !== q.y || Math.abs(p.y - (edge === "top" ? y : y + height)) > 1) continue;
            path = candidate;
            centerline = p.y;
            break;
          }
          if (path) break;
        }
        return {
          rect: [x, y, width, height],
          content: getComputedStyle(element, edge === "top" ? "::before" : "::after").content,
          state: path?.getAttribute("data-shared-border-state"),
          stroke: path ? getComputedStyle(path).stroke : null,
          strokeWidth: path ? getComputedStyle(path).strokeWidth : null,
          centerline,
        };
      }, edge);
      await page.mouse.move(width / 2, height / 2);
      const baseline = await getBand();
      assert.equal(baseline.content, "none");
      assert.equal(baseline.strokeWidth, "1px");
      assert.equal(baseline.state, "normal");
      const [x, y, bandWidth, bandHeight] = baseline.rect;
      await page.mouse.move(x + bandWidth / 2, y + bandHeight / 2);
      await band.locator(':scope > [data-shared-border-overlay] [data-shared-border-state="hover"]').first().waitFor({ state: "attached", timeout: 2000 });
      const hovered = await getBand();
      assert.equal(hovered.state, "hover");
      assert.equal(hovered.strokeWidth, "1px");
      assert.notEqual(hovered.stroke, baseline.stroke);
      assert.deepEqual(hovered.rect, baseline.rect);
      assert.equal(hovered.centerline, baseline.centerline);
      console.log("[DEBUG]", width, slot, baseline, hovered);
    }
    const thick = await page.locator("#os-chrome-live").locator("*").evaluateAll(elements => elements.flatMap(element => {
      const style = getComputedStyle(element);
      const thickness = Math.max(...[style.borderTopWidth, style.borderRightWidth, style.borderBottomWidth, style.borderLeftWidth].map(value => parseFloat(value) || 0));
      const { width, height } = element.getBoundingClientRect();
      return thickness > 1 && width > 0 && height > 0 ? [{ slot: element.getAttribute("data-slot"), class: element.getAttribute("class"), thickness }] : [];
    }));
    console.log("[DEBUG]", width, "native borders over 1px:", thick);
    await page.close();
  }
} finally {
  await browser.close();
}
