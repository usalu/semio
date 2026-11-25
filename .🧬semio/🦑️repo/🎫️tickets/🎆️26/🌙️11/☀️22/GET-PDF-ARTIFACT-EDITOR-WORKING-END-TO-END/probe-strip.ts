import { createRequire } from "node:module";
import { join } from "node:path";

/** 🔬️ Opens the two-page fixture in the 6260 pane and prints, per window, the strip rows found in the DOM before and after the open, and whether a refresh of the Pages window changes them.
 * Usage: `bun probe-strip.ts` from the workspace root with the host on 6260.
 * @see ./🌐️pdf-browser-journey.md */

const root = process.cwd();
const osDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const fixture = join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
const { ensureRepositoryBrowserPathV1 } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts"));
const { awaitBeacon, clickUncovered, dismissIntroduction, unfoldActionsRail } = await import(join(root, osDev, "🧪️tests/🧮️program-matrix/🟦️.ts"));
const require = createRequire(join(root, osDev, "📦️packages/🟦️typescript/package.json"));
ensureRepositoryBrowserPathV1(root, process.env);
const { chromium } = require("playwright") as typeof import("playwright");
const browser = await chromium.launch();
const page = await browser.newPage({ locale: "en-US", viewport: { width: 1600, height: 1000 } });
const log: string[] = [];
page.on("console", message => { if (/DEBUG|TRACE|refresh|thumbnail|strip/u.test(message.text())) log.push(`${message.type()}: ${message.text().slice(0, 300)}`); });
await page.goto("http://127.0.0.1:6260/");
console.log("beacon", await awaitBeacon(page, Date.now() + 240_000));
await dismissIntroduction(page);
const rows = () => page.evaluate(() => [...document.querySelectorAll("*")].filter(element => element.children.length === 0 && /^(?:Page|Seite) \d+ \/ \d+/u.test((element.textContent ?? "").trim())).map(element => { let pane = element.closest("[data-window-id],[data-window-kind-id],[data-slot]"); return `${(element.textContent ?? "").trim()} @ ${pane?.getAttribute("data-window-id") ?? pane?.getAttribute("data-window-kind-id") ?? pane?.getAttribute("data-slot") ?? "?"}`; }));
console.log("before", JSON.stringify(await rows()));
await unfoldActionsRail(page);
const [chooser] = await Promise.all([page.waitForEvent("filechooser", { timeout: 30_000 }), clickUncovered(page, '[data-slot="window-action-pane"] [id="action.openArtifactFile"]')]);
await chooser.setFiles(fixture);
await page.waitForTimeout(15_000);
console.log("after", JSON.stringify(await rows()));
console.log("pages-window", JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("[data-window-kind-id]")].map(element => `${element.getAttribute("data-window-kind-id")}: ${(element.textContent ?? "").replace(/\s+/gu, " ").slice(0, 160)}`))));
console.log(log.slice(-40).join("\n"));
await browser.close();
