/** 🔬️ Opens the overview of a fresh learner at one viewport and text size and says what the page did meanwhile:
 * `bun overview_probe.ts [origin] [width] [height] [text size] [locale]` prints every console error, page error, crash
 * and navigation, the layout the overview chose and the box of every card. */
import { resolve } from "node:path";
import { chromium } from "playwright";
import { repoToolCacheEnv } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";

process.env.PLAYWRIGHT_BROWSERS_PATH ??= repoToolCacheEnv(resolve(import.meta.dir, "../../../../../../..")).PLAYWRIGHT_BROWSERS_PATH;
const [origin = "http://localhost:6061", width = "1280", height = "800", textSize = "largest", locale = "de"] = process.argv.slice(2);
const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width: Number(width), height: Number(height) }, locale: locale === "de" ? "de-DE" : "en-GB", baseURL: origin });
await context.addInitScript((preferences) => localStorage.getItem("semio.quiz.architecture.preferences") === null && localStorage.setItem("semio.quiz.architecture.preferences", preferences), JSON.stringify({ locale, theme: "dark", textSize, pets: "off", petsChosen: true, animateIcons: false, iconsChosen: true }));
const page = await context.newPage();
page.on("console", (message) => message.type() === "error" && console.log("console error:", message.text().slice(0, 400)));
page.on("pageerror", (error) => console.log("page error:", String(error).slice(0, 400)));
page.on("crash", () => console.log("crash"));
page.on("framenavigated", (frame) => frame === page.mainFrame() && console.log("navigated:", frame.url()));
await page.goto("/");
await page.locator('#quiz-main [data-card="introduction"] [data-overview-card-action="primary"]').click();
await page.locator('#quiz-main [data-card="identity"] [data-overview-card-action="primary"]').click();
console.log("identified");
try {
  await page.locator("[data-layered-overview]").waitFor({ timeout: 15000 });
  await page.waitForTimeout(3000);
  console.log(
    JSON.stringify(
      await page.evaluate(() => ({
        mode: document.querySelector("[data-layered-overview]")?.getAttribute("data-mode"),
        cards: [...document.querySelectorAll<HTMLElement>("[data-layered-card]")].map((card) => {
          const box = (card.firstElementChild ?? card).getBoundingClientRect();
          return `${card.dataset.layeredCard} ${Math.round(box.left)},${Math.round(box.top)} ${Math.round(box.width)}×${Math.round(box.height)}`;
        }),
      })),
    ),
  );
  await page.screenshot({ path: resolve(import.meta.dir, "🗑️generated", `overview-${width}-${textSize}.png`) });
} catch (error) {
  console.log("failed:", String(error).split("\n")[0]);
}
await browser.close();
