import { createRequire } from "node:module";
import { join } from "node:path";

/** 🔬️ Presses Open File in the 6260 pane and prints whether the action exists, what the press hit, every console line and notice after it, and whether a file chooser opened.
 * Usage: `bun probe-open-action.ts` from the workspace root with the host on 6260.
 * @see ./🌐️pdf-browser-journey.md */

const root = process.cwd();
const osDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const { ensureRepositoryBrowserPathV1 } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts"));
const { awaitBeacon, clickUncovered, dismissIntroduction, unfoldActionsRail } = await import(join(root, osDev, "🧪️tests/🧮️program-matrix/🟦️.ts"));
const require = createRequire(join(root, osDev, "📦️packages/🟦️typescript/package.json"));
ensureRepositoryBrowserPathV1(root, process.env);
const { chromium } = require("playwright") as typeof import("playwright");
const browser = await chromium.launch();
const page = await browser.newPage({ locale: "en-US", viewport: { width: 1600, height: 1000 } });
const log: string[] = [];
page.on("console", message => log.push(`${message.type()}: ${message.text().slice(0, 400)}`));
page.on("pageerror", error => log.push(`pageerror: ${error.message}`));
await page.addInitScript(() => { const seen: string[] = []; Object.defineProperty(window, "__notices", { value: seen }); new MutationObserver(() => { for (const element of document.querySelectorAll("[data-semio-transient-notice]")) { const line = `${element.getAttribute("data-notice-code") ?? ""}|${(element.textContent ?? "").trim().slice(0, 200)}`; if (!seen.includes(line)) seen.push(line); } }).observe(document, { subtree: true, childList: true, characterData: true }); });
await page.addInitScript(() => { const click = HTMLInputElement.prototype.click; HTMLInputElement.prototype.click = function () { console.log(`[DEBUG] input.click type=${this.type} active=${navigator.userActivation.isActive} t=${Math.round(performance.now())}`); return click.call(this); }; document.addEventListener("pointerup", () => console.log(`[DEBUG] pointerup t=${Math.round(performance.now())}`), true); });
await page.goto("http://127.0.0.1:6260/");
console.log("beacon", await awaitBeacon(page, Date.now() + 240_000));
await dismissIntroduction(page);
await unfoldActionsRail(page);
const selector = '[data-slot="window-action-pane"] [id="action.openArtifactFile"]';
console.log("open buttons", await page.locator(selector).count(), "save buttons", await page.locator('[data-slot="window-action-pane"] [id="action.saveArtifactFile"]').count());
console.log("file actions", JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[id^="action."]')].map(element => element.id).filter(id => /File|Artifact/u.test(id)))));
console.log("open attrs", JSON.stringify(await page.evaluate((sel) => [...document.querySelectorAll(sel)].map(element => ({ tag: element.tagName, disabled: (element as HTMLButtonElement).disabled, aria: element.getAttribute("aria-disabled"), title: element.getAttribute("title"), text: (element.textContent ?? "").trim().slice(0, 60), visible: (element as HTMLElement).offsetParent !== null, attrs: [...element.attributes].map(attribute => `${attribute.name}=${attribute.value.slice(0, 60)}`).join(" ") })), selector)));
console.log("save attrs", JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[data-slot="window-action-pane"] [id="action.saveArtifactFile"]')].slice(0, 1).map(element => [...element.attributes].map(attribute => `${attribute.name}=${attribute.value.slice(0, 60)}`).join(" ")))));
const saveStart = log.length;
const download = page.waitForEvent("download", { timeout: 30_000 }).then(() => "downloaded", error => `none: ${String(error).slice(0, 60)}`);
console.log("save press", await clickUncovered(page, '[data-slot="window-action-pane"] [id="action.saveArtifactFile"]'), await download);
console.log("console after save:\n" + log.slice(saveStart).filter(line => line.includes("[DEBUG]")).join("\n"));
const hit = await page.evaluate((sel) => { const row = document.querySelector(sel) as HTMLElement; row.scrollIntoView({ block: "nearest" }); const box = row.getBoundingClientRect(); const top = document.elementFromPoint(box.x + box.width / 2, box.y + box.height / 2) as HTMLElement | null; const pane = row.closest('[data-slot="window-action-pane"]') as HTMLElement; const paneBox = pane.getBoundingClientRect(); return { row: [box.x, box.y, box.width, box.height].map(Math.round), pane: [paneBox.x, paneBox.y, paneBox.width, paneBox.height].map(Math.round), top: top === null ? null : `${top.tagName}.${(top.className || "").toString().slice(0, 80)} slot=${top.getAttribute("data-slot")} text=${(top.textContent ?? "").trim().slice(0, 40)}`, self: top !== null && row.contains(top) }; }, selector);
console.log("open hit", JSON.stringify(hit));
const before = log.length;
const chooser = page.waitForEvent("filechooser", { timeout: 15_000 }).then(() => "opened", error => `none: ${String(error).slice(0, 80)}`);
console.log("press", await clickUncovered(page, selector));
console.log("chooser", await chooser);
console.log("notices", JSON.stringify(await page.evaluate(() => (window as unknown as { __notices: string[] }).__notices)));
console.log("console after press:\n" + log.slice(before).join("\n"));
await page.screenshot({ path: join(import.meta.dir, "🗑️generated", "probe-open-action.png") });
await browser.close();
