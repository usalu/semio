import {createRequire} from "node:module";
import {resolve} from "node:path";
import {fileURLToPath} from "node:url";

const ticket = fileURLToPath(new URL(".", import.meta.url));
const workspace = resolve(ticket, "../../../../../../..");
const require = createRequire(resolve(workspace, "🏢️semio-tech/🎡️play/package.json"));
const {chromium} = require("playwright");
const browser = await chromium.launch({args: ["--in-process-gpu", "--disable-dev-shm-usage"]});
const page = await browser.newPage({locale: "en"});
const started = Date.now();
const report = (event, details) => console.log("[DEBUG]", ((Date.now() - started) / 1000).toFixed(1), event, details);
page.on("pageerror", error => report("page error", error.message));
page.on("console", message => {
  if (["warning", "error"].includes(message.type()) && /command|dispatch|refus|documentTransfer/i.test(message.text())) {
    report(message.type(), message.text().slice(0, 2000));
  }
});

try {
  const base = process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:6033";
  await page.goto(new URL("/#din4108", base).href);
  const shell = page.locator('[data-shell-id="din4108"]');
  const cancel = page.getByRole("button", {name: "Cancel command", exact: true}).first();
  await page.getByText("Outliner", {exact: true}).first().waitFor({timeout: 120_000});
  await cancel.waitFor({timeout: 45_000});
  report("before cancelling the queued command", (await page.locator("body").innerText()).slice(0, 345));
  await cancel.click();
  const picker = page.locator('[id="playground.navbar.fixture"]');
  await picker.waitFor({state: "visible", timeout: 30_000});
  const prior = (await picker.innerText()).trim();
  await picker.click();
  const selected = page.getByRole("option").filter({hasNotText: prior}).first();
  const next = (await selected.innerText()).trim();
  await selected.click();
  try {
    await page.waitForFunction(label => document.getElementById("playground.navbar.fixture")?.innerText.trim() === label, next, {timeout: 35_000});
  } finally {
    report("after choosing next example", {prior, next, settled: (await picker.innerText()).trim(), shell: await shell.getAttribute("data-shell-ready"), text: (await page.locator("body").innerText()).slice(0, 650)});
  }
} finally {
  await browser.close();
}
