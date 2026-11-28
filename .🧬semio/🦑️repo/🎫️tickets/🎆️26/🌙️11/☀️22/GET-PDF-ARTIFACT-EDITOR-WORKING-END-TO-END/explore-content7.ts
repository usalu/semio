import { runSteps, session } from "./journey-kit.ts";
import type { Browser } from "playwright";
import { join } from "node:path";
const content = join(import.meta.dir, "🗑️generated/fixtures/content.pdf");
const sleep = (ms: number) => new Promise(r => setTimeout(r, ms));
async function explore(browser: Browser) {
  const pane = await session(browser, "en-US", "explore7");
  try {
    await pane.open(content);
    await sleep(3000);
    const sel = () => pane.page.evaluate(() => [...document.querySelectorAll('[id^="window:pdf.object/pdf-object-"]')].filter(e => e.tagName === "INPUT").map(e => e.id.slice(-8)).join(",") || (document.body.innerText.includes("Select an object") ? "none" : "?"));
    for (const [label, x, y] of [["text", 300, 197], ["rect", 330, 333], ["image", 598, 323], ["line", 400, 432]] as const) {
      if (process.env.MODE === "move") { await pane.page.mouse.move(x - 5, y - 5); await pane.page.mouse.move(x, y, { steps: 5 }); await pane.page.mouse.down(); await sleep(80); await pane.page.mouse.up(); }
      else if (process.env.MODE === "row") { await pane.page.getByRole("button", { name: /Image:/ }).first().click(); await sleep(1500); await pane.page.mouse.click(x, y); }
      else await pane.page.mouse.click(x, y);
      const seen: string[] = [];
      for (let i = 0; i < 6; i++) { await sleep(500); seen.push(await sel()); }
      console.log(label, x, y, seen.join(" | "));
    }
  } finally { await pane.context.close(); }
}
await runSteps(["explore"], { explore });
