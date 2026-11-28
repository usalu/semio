import { runSteps, session } from "./journey-kit.ts";
import type { Browser } from "playwright";
import { join } from "node:path";
const content = join(import.meta.dir, "🗑️generated/fixtures/content.pdf");
async function explore(browser: Browser) {
  const pane = await session(browser, "en-US", "explore6");
  try {
    await pane.open(content);
    await pane.page.waitForTimeout(4000);
    console.log(JSON.stringify(await pane.page.evaluate(() => [...document.querySelectorAll("canvas")].map(element => {
      const r = element.getBoundingClientRect(); const ctx = element.getContext("2d")!; const col = Math.floor(element.width * 0.97);
      const d = ctx.getImageData(col, 0, 1, element.height).data; const samples: string[] = [];
      for (let y = 0; y < element.height; y += 40) samples.push(`${y}:${d[y*4]},${d[y*4+1]},${d[y*4+2]}`);
      return { w: element.width, h: element.height, rect: [r.left, r.top, r.width, r.height], samples };
    }))));
  } finally { await pane.context.close(); }
}
await runSteps(["explore"], { explore });
