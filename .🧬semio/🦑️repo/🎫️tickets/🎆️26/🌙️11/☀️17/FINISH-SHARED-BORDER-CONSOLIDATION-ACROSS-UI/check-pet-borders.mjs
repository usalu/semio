import { readFileSync } from "node:fs";
import assert from "node:assert/strict";
import { chromium } from "playwright-core";

const fixture = JSON.parse(readFileSync(new URL("./pet-border-cases.json", import.meta.url), "utf8"));
const stylesheet = new URL("../../../../../../../🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/📖️stories/🎨️.css", import.meta.url);
const pageContent = '<div class="roster"><a href="#pet">Pet</a></div><article class="card"><button>Inspect</button></article>';
const browser = await chromium.launch();
try {
  for (const deviceScaleFactor of [1, 2]) {
    const page = await browser.newPage({ deviceScaleFactor });
    try {
      await page.setContent(`<style>${readFileSync(stylesheet, "utf8")}</style>${pageContent}`);
      const snapshot = selector => page.locator(selector).evaluate(element => {
        const bounds = element.getBoundingClientRect().toJSON();
        const style = getComputedStyle(element);
        const edges = ["top", "right", "bottom", "left"];
        return { bounds, widths: edges.map(edge => style.getPropertyValue(`border-${edge}-width`)), colors: edges.map(edge => style.getPropertyValue(`border-${edge}-color`)), outline: style.outlineStyle };
      });
      for (const { owner, target } of fixture.ancestry) {
        await page.mouse.move(1000, 600);
        const idle = await snapshot(owner);
        await page.locator(target).hover();
        const hover = await snapshot(owner);
        assert.deepEqual(idle.widths, Array(4).fill(fixture.width));
        assert.deepEqual(hover.widths, idle.widths);
        assert.deepEqual(hover.bounds, idle.bounds);
        assert(hover.colors.every((color, index) => color !== idle.colors[index]));
      }
      await page.mouse.move(1000, 600);
      const idle = await snapshot(fixture.active);
      await page.locator(fixture.active).evaluate(element => element.setAttribute("aria-current", "page"));
      const active = await snapshot(fixture.active);
      assert.deepEqual(active.bounds, idle.bounds);
      assert.deepEqual(active.widths, idle.widths);
      assert(active.colors.every((color, index) => color !== idle.colors[index]));
      assert.equal(active.outline, "none");
      console.log(`Pet silhouettes passed at DPR ${deviceScaleFactor}`);
    } finally { await page.close(); }
  }
} finally { await browser.close(); }
