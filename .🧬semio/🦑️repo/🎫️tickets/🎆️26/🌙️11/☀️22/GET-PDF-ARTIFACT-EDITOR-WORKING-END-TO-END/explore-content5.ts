import { runSteps, session } from "./journey-kit.ts";
import type { Browser } from "playwright";
import { join } from "node:path";
const content = join(import.meta.dir, "🗑️generated/fixtures/content.pdf");
const sleep = (ms: number) => new Promise(r => setTimeout(r, ms));
async function explore(browser: Browser) {
  const pane = await session(browser, "en-US", "explore5");
  try {
    await pane.open(content);
    await sleep(3000);
    const fields = () => pane.page.evaluate(() => Object.fromEntries([...document.querySelectorAll('[id^="window:pdf.object/pdf-object"]')].filter(e => e.tagName === "INPUT").map(e => [e.id.replace("window:pdf.object/pdf-object-", ""), (e as HTMLInputElement).value])));
    const click = async (x: number, y: number) => {
      const box = await pane.page.evaluate(() => { const r = document.querySelector("canvas")!.getBoundingClientRect(); return { l: r.left, t: r.top, w: r.width }; });
      const s = box.w / 612;
      await pane.page.mouse.click(box.l + x * s, box.t + (792 - y) * s);
      await sleep(2000);
    };
    await click(172, 592); console.log("rect sel", JSON.stringify(await fields()), (await pane.canvasHash()).join());
    const w = pane.page.locator('[id="window:pdf.object/pdf-object-width"]');
    await w.fill("150"); await w.press("Enter"); await sleep(3000);
    console.log("after width", JSON.stringify(await fields()), (await pane.canvasHash()).join());
    await pane.shot("explore5-width");
    console.log("rows", (await pane.text()).split("\n").filter(l => /^(Vector|Image|Text:)/.test(l.trim())).join("|"));
    await click(440, 602); console.log("image click", JSON.stringify(await fields()), (await pane.canvasHash()).join());
    await pane.shot("explore5-image");
  } finally { await pane.context.close(); }
}
await runSteps(["explore"], { explore });
