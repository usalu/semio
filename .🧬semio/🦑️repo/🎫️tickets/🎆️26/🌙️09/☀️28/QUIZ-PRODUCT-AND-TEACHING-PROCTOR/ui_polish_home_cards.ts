/** 🃏️ How the home cards fit their cells (work package "UI polish", item 4): for a list of viewports and both text sizes,
 * in German, the layout the overview chose, every card's height beside its cell's, how many cards are taller than their
 * cell, and which cards lie over each other. `bun ui_polish_home_cards.ts [site origin] [WxH …]` against a running stack. */
import { resolve } from "node:path";
import { chromium } from "playwright";
import { repoToolCacheEnv } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
process.env.PLAYWRIGHT_BROWSERS_PATH ??= repoToolCacheEnv(repoRoot).PLAYWRIGHT_BROWSERS_PATH;
const [origin = "http://127.0.0.1:6071", ...sizes] = process.argv.slice(2);
const viewports = (sizes.length > 0 ? sizes : ["1440x900", "1280x720", "1024x768", "1023x768", "900x700", "768x1024", "800x600", "640x360"]).map((size) => size.split("x").map(Number) as [number, number]);

const browser = await chromium.launch();
for (const textSize of ["normal", "largest"] as const) {
  for (const [width, height] of viewports) {
    const context = await browser.newContext({ viewport: { width, height }, locale: "de-DE", baseURL: origin });
    await context.addInitScript((preferences) => localStorage.getItem("semio.quiz.architecture.preferences") === null && localStorage.setItem("semio.quiz.architecture.preferences", preferences), JSON.stringify({ locale: "de", theme: "system", textSize, showCursors: true, showAnswers: true, animateIcons: false, pets: "off" }));
    const page = await context.newPage();
    await page.goto("/");
    await page.locator('#quiz-main [data-card="introduction"] [data-overview-card-action="primary"]').click();
    await page.locator('#quiz-main [data-card="identity"] [data-overview-card-action="primary"]').click();
    await page.locator("[data-layered-overview]").waitFor();
    await page.waitForTimeout(1_200);
    const seen = await page.evaluate(() => {
      const overview = document.querySelector<HTMLElement>("[data-layered-overview]")!;
      const cards = [...document.querySelectorAll<HTMLElement>("[data-layered-card]")].map((holder) => {
        const card = holder.querySelector<HTMLElement>("[data-card]")!;
        const cell = holder.querySelector<HTMLElement>(".quiz-home-cell");
        return { id: holder.dataset.layeredCard!, card: card.getBoundingClientRect(), cell: cell?.getBoundingClientRect() };
      });
      const over: string[] = [];
      for (const [index, a] of cards.entries()) for (const b of cards.slice(index + 1)) if (Math.min(a.card.right, b.card.right) - Math.max(a.card.left, b.card.left) > 1 && Math.min(a.card.bottom, b.card.bottom) - Math.max(a.card.top, b.card.top) > 1) over.push(`${a.id}/${b.id}`);
      return { mode: overview.dataset.mode, box: Math.round(overview.getBoundingClientRect().height), cards: cards.map(({ id, card, cell }) => `${id} ${Math.round(card.height)}${cell === undefined ? "" : `/${Math.round(cell.height)}`}`), tall: cards.filter(({ card, cell }) => cell !== undefined && card.height > cell.height + 1).length, over };
    });
    console.log(`[cards] ${width} × ${height}, ${textSize}: ${seen.mode} (overview ${seen.box} px high), taller than their cell ${seen.tall}, overlapping ${seen.over.length === 0 ? "none" : seen.over.join(" ")} — ${seen.cards.join(", ")}`);
    await context.close();
  }
}
await browser.close();
