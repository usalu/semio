/** 🔎️ What a fresh browser sees of the dev site: `bun navbar_probe.ts [site origin]` prints the console problems, the requests that failed, the cards in front and the start of the page's text. */
import { resolve } from "node:path";
import { chromium } from "playwright";
import { repoToolCacheEnv } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
process.env.PLAYWRIGHT_BROWSERS_PATH ??= repoToolCacheEnv(repoRoot).PLAYWRIGHT_BROWSERS_PATH;
const origin = process.argv[2] ?? "http://localhost:6061";
const browser = await chromium.launch();
const page = await (await browser.newContext({ viewport: { width: 1280, height: 720 }, locale: "en-GB", baseURL: origin })).newPage();
const problems: string[] = [];
const failed: string[] = [];
page.on("console", (message) => (message.type() === "error" || message.type() === "warning") && problems.push(`${message.type()}: ${message.text()} @ ${message.location().url}`));
page.on("pageerror", (error) => problems.push(JSON.stringify({ name: error.name, message: error.message, stack: error.stack, own: Object.getOwnPropertyNames(error) })));
page.on("requestfailed", (request) => failed.push(`${request.failure()?.errorText} ${request.url()}`));
page.on("response", (response) => response.status() >= 400 && failed.push(`${response.status()} ${response.url()}`));
await page.goto("/");
await page.waitForTimeout(Number(process.argv[3] ?? 8000));
console.log(JSON.stringify({ problems, failed, cards: await page.locator("[data-card]").evaluateAll((cards) => cards.map((card) => card.getAttribute("data-card"))), text: (await page.locator("body").innerText()).slice(0, 300) }, null, 2));
await browser.close();
