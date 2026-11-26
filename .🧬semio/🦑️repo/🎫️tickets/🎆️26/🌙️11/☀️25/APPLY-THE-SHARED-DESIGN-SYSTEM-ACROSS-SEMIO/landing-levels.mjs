import assert from "node:assert/strict";
import { chromium } from "playwright";

const browser = await chromium.launch({ channel: "chrome", headless: true });
try {
  for (const colorScheme of ["light", "dark"]) {
    await Promise.all([6192, 6193].map(async port => {
      const page = await browser.newPage({ colorScheme, viewport: {width:1280,height:900} });
      page.on("pageerror", error => console.error("[DEBUG]", port, error.message));
      await page.addInitScript(appearance => localStorage.setItem("ui.chrome.appearance", appearance),colorScheme);
      await page.goto("http://127.0.0.1:"+port, { waitUntil:"domcontentloaded",timeout:120000 });
      const cards = page.locator("[data-layered-overlay] [data-window-frame]");
      await cards.first().waitFor({timeout:120000});
      const states = await cards.evaluateAll(frames=>frames.map(frame=>{
        const body=frame.querySelector("[data-slot=window-chrome-body-surface]");
        const chip=frame.querySelector("[data-window-frame-chip]");
        const style=chip && getComputedStyle(chip);
        return {body:body?.getAttribute("data-level"),chip:chip?.getAttribute("data-level"),glass:chip?.classList.contains("ui-glass"),blur:style?.backdropFilter,bodyPaint:body && getComputedStyle(body).backgroundColor};
      }));
      for (const state of states) {
        assert.equal(state.body,"dialog"); assert.equal(state.chip,"menu");
        assert(state.glass && state.blur.includes("blur("));
      }
      console.log(JSON.stringify({port,colorScheme,count:states.length,...states[0]}));
      await page.close();
    }));
  }
} finally { await browser.close(); }
