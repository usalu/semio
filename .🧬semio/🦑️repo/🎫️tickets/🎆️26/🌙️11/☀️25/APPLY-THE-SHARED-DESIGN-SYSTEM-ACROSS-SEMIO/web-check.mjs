import assert from "node:assert/strict";
import { chromium } from "playwright";

const browser = await chromium.launch({ channel: "chrome", headless: true });
try {
  for (const width of [1440, 390]) {
    const page = await browser.newPage({ viewport: { width, height: 900 }, locale: "en-US" });
    await page.goto(process.argv[2] ?? "http://localhost:6072/");
    await page.locator(".roster a").nth(1).waitFor();
    const navigation = await page.locator(".action-group").evaluate((group) => {
      const [left, right] = group.querySelectorAll("button");
      return [right.getBoundingClientRect().left - left.getBoundingClientRect().right, getComputedStyle(left).borderInlineEndWidth, getComputedStyle(right).borderInlineStartWidth];
    });
    assert.deepEqual(navigation, [0, "1px", "0px"]);
    const gallery = await page.evaluate(() => {
      const [first, second] = document.querySelectorAll(".roster a");
      const style = getComputedStyle(document.querySelector(".story-section .story-heading"));
      return {
        gap: second.getBoundingClientRect().left - first.getBoundingClientRect().right,
        seam: [getComputedStyle(first).borderInlineEndWidth, getComputedStyle(second).borderInlineStartWidth],
        edges: [style.borderTopWidth, style.borderRightWidth, style.borderBottomWidth, style.borderLeftWidth],
        alpha: style.backgroundColor,
        tint: style.backgroundImage,
        blur: style.backdropFilter,
        width: document.documentElement.scrollWidth,
      };
    });
    assert.equal(gallery.gap, 0);
    assert.deepEqual(gallery.seam, ["1px", "0px"]);
    assert.deepEqual(gallery.edges, ["0px", "1px", "1px", "0px"]);
    assert.equal(gallery.width, width);
    assert.match(gallery.blur, /^blur\(16px\)/u);
    assert.match(gallery.alpha, /\/ 0\.76\)/u);
    assert.match(gallery.tint, /\/ 0\.1\)/u);

    await page.getByRole("button", { name: "Sandbox", exact: true }).click();
    const sandbox = await page.locator(".card .tab").evaluate((chip) => {
      const style = getComputedStyle(chip);
      const card = chip.closest(".card").getBoundingClientRect();
      const bounds = chip.getBoundingClientRect();
      return {
        edges: [style.borderTopWidth, style.borderRightWidth, style.borderBottomWidth, style.borderLeftWidth],
        top: bounds.top - card.top,
        right: card.right - bounds.right,
        alpha: style.backgroundColor,
        tint: style.backgroundImage,
        blur: style.backdropFilter,
        width: document.documentElement.scrollWidth,
      };
    });
    assert.deepEqual(sandbox.edges, ["0px", "0px", "1px", "1px"]);
    assert.equal(sandbox.top, 1);
    assert.equal(sandbox.right, 1);
    assert.match(sandbox.blur, /^blur\(32px\)/u);
    assert.match(sandbox.alpha, /\/ 0\.52\)/u);
    assert.match(sandbox.tint, /\/ 0\.1\)/u);
    assert.equal(sandbox.width, width);
    console.log(width, { gallery, sandbox });
    await page.close();
  }
} finally {
  await browser.close();
}
