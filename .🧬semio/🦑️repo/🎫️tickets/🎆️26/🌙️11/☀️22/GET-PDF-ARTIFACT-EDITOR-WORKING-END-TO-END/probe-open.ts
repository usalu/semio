import { createRequire } from "node:module";
import { join } from "node:path";

/** 🔬️ Opens the two-page fixture in the 6260 pane and prints where the page size, the title and the page rows appear after the open, and the commit-checkpoint warnings the open causes.
 * Usage: `bun probe-open.ts` from the workspace root with the host on 6260.
 * @see ./🌐️pdf-browser-journey.md */

const root = process.cwd();
const osDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const fixture = join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
const { ensureRepositoryBrowserPathV1 } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts"));
const { awaitBeacon, clickUncovered, dismissIntroduction, unfoldActionsRail } = await import(join(root, osDev, "🧪️tests/🧮️program-matrix/🟦️.ts"));
const require = createRequire(join(root, osDev, "📦️packages/🟦️typescript/package.json"));
ensureRepositoryBrowserPathV1(root, process.env);
const { chromium } = require("playwright") as typeof import("playwright");
for (const signal of ["SIGTERM", "SIGHUP", "SIGINT", "SIGPIPE", "SIGUSR1", "SIGUSR2"] as const) process.on(signal, () => console.log(`probe received ${signal}`));
const browser = await chromium.launch({ handleSIGHUP: false, handleSIGINT: false, handleSIGTERM: false });
browser.on("disconnected", () => console.log("browser disconnected"));
const page = await browser.newPage({ locale: "en-US", viewport: { width: 1600, height: 1000 } });
const log: string[] = [];
page.on("console", message => { if (/commitCheckpoint|refused|owner-mismatch/u.test(message.text())) log.push(`${message.type()}: ${message.text().slice(0, 300)}`); });
page.on("crash", () => console.log("page crashed"));
page.on("close", () => console.log("page closed"));
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
console.log("sizes", JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("*")].filter(element => element.children.length === 0 && /595|842/u.test(element.textContent ?? "")).map(element => `${(element.textContent ?? "").trim().slice(0, 80)} @ ${element.closest("[data-window-id]")?.getAttribute("data-window-id") ?? "?"}`).slice(0, 20))));
console.log("inputs", JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("input")].map(input => `${input.getAttribute("aria-label") ?? input.id}=${input.value}`).filter(line => /595|842|Title|Titel/u.test(line)).slice(0, 20))));
console.log(log.slice(-40).join("\n"));
await browser.close();
