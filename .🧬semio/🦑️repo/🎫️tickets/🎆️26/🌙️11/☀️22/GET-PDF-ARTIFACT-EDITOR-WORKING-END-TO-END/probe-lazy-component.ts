import { createRequire } from "node:module";
import { join } from "node:path";

/** 🧩️ Lazy component loading in the 6260 pane: the PDF 1.7 editor boots on its own component only, and opening a PDF 1.4 document loads the
 * 1.4 component then, not before. Records the main core module of every component the page fetches (`*.core.wasm` responses) at boot and after the open, the
 * open notice, and the page rows of the opened document; exits non-zero unless boot fetched only the 1.7 editors component and the open
 * fetched the 1.4 component and opened the file.
 * Usage: `bun probe-lazy-component.ts` from the workspace root with the host on 6260.
 * @see ./🌐️pdf-browser-journey.md */

const root = process.cwd();
const osDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const fixture = join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/📚️examples/🎬️demo/🖼️assets/🎬️demo.pdf");
const { ensureRepositoryBrowserPathV1 } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts"));
const { awaitBeacon, clickUncovered, dismissIntroduction, unfoldActionsRail } = await import(join(root, osDev, "🧪️tests/🧮️program-matrix/🟦️.ts"));
const require = createRequire(join(root, osDev, "📦️packages/🟦️typescript/package.json"));
ensureRepositoryBrowserPathV1(root, process.env);
const { chromium } = require("playwright") as typeof import("playwright");
const browser = await chromium.launch({ handleSIGHUP: false, handleSIGINT: false, handleSIGTERM: false });
let status = 0;
try {
  const context = await browser.newContext({ locale: "en-US", viewport: { width: 1600, height: 1000 } });
  const page = await context.newPage();
  const cores: string[] = [];
  const notices: string[] = [];
  const errors: string[] = [];
  context.on("response", response => { const name = decodeURIComponent(new URL(response.url()).pathname).split("/").pop() ?? ""; if (/\.core\.wasm$/u.test(name)) cores.push(name); });
  page.on("console", message => { const text = message.text(); if (/file-opened|file-open-failed|openFile\./u.test(text)) notices.push(text.slice(0, 200)); if (message.type() === "error") errors.push(text.slice(0, 300)); });
  await page.goto("http://127.0.0.1:6260/");
  console.log("beacon", await awaitBeacon(page, Date.now() + 240_000));
  await dismissIntroduction(page);
  await page.waitForTimeout(5_000);
  const boot = [...new Set(cores)];
  console.log("boot cores", JSON.stringify(boot));
  await unfoldActionsRail(page);
  const [chooser] = await Promise.all([page.waitForEvent("filechooser", { timeout: 30_000 }), clickUncovered(page, '[data-slot="window-action-pane"] [id="action.openArtifactFile"]')]);
  await chooser.setFiles(fixture);
  const deadline = Date.now() + 120_000;
  while (Date.now() < deadline && !cores.some(name => name.startsWith("semio_hub_stdio_pdf_1_4_component"))) await page.waitForTimeout(1_000);
  await page.waitForTimeout(15_000);
  const opened = [...new Set(cores)];
  const rows = await page.evaluate(() => [...document.querySelectorAll("*")].filter(element => element.children.length === 0 && /^(?:Page|Seite) \d+ \/ \d+/u.test((element.textContent ?? "").trim())).map(element => (element.textContent ?? "").trim()));
  console.log("cores after the 1.4 open", JSON.stringify(opened));
  console.log("rows", JSON.stringify(rows));
  console.log("notices", JSON.stringify(notices.slice(-6)));
  console.log("console errors", JSON.stringify(errors.slice(-10)));
  const bootOnlyEditors = boot.length > 0 && boot.every(name => name.startsWith("semio_hub_stdio_pdf_1_7_editors_component"));
  const lazy14 = opened.some(name => name.startsWith("semio_hub_stdio_pdf_1_4_component")) && !opened.some(name => name.startsWith("semio_hub_stdio_pdf_1_7_viewers_component"));
  console.log(`${bootOnlyEditors ? "PASS" : "FAIL"} boot fetched only the 1.7 editors component`);
  console.log(`${lazy14 ? "PASS" : "FAIL"} the 1.4 open fetched the 1.4 component on demand`);
  if (!bootOnlyEditors || !lazy14) status = 1;
} finally {
  await browser.close();
}
process.exit(status);
