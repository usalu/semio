import { runSteps, session, unfoldActionsRail } from "./journey-kit.ts";
import type { Browser } from "playwright";
import { join } from "node:path";
const content = join(import.meta.dir, "🗑️generated/fixtures/content.pdf");
async function explore(browser: Browser) {
  const pane = await session(browser, "en-US", "explore");
  try {
    await pane.open(content);
    await unfoldActionsRail(pane.page);
    await pane.page.waitForTimeout(3000);
    const infos = () => pane.page.evaluate(() => [...document.querySelectorAll("canvas")].map(c => { const r = c.getBoundingClientRect(); return `${c.width}x${c.height}@${Math.round(r.left)},${Math.round(r.top)} ${Math.round(r.width)}x${Math.round(r.height)}`; }));
    console.log(await infos(), await pane.canvasHash());
    await pane.insertText("Alpha Marker", "1");
    for (const ms of [1000, 3000, 6000]) { await pane.page.waitForTimeout(ms); console.log(ms, await pane.canvasHash()); }
    const c = pane.page.locator("canvas").first();
    await c.screenshot({ path: join(import.meta.dir, "🗑️generated/sweep-content/canvas-after-insert.png") });
  } finally { await pane.context.close(); }
}
await runSteps(["explore"], { explore });
