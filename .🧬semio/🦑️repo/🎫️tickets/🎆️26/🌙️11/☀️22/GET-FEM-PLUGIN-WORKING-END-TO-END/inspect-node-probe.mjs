/** 🧷️ Selects FEM 2D node n1 and opens Inspection like the browser smoke, logging rejected UI patches and the inspection panel's controls. Usage: node inspect-node-probe.mjs <url> */
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const url = process.argv[2] ?? "http://127.0.0.1:6086/?plugin=fem2d";
const browser = await chromium.launch();
try {
  const page = await browser.newPage({ locale: "en-US", viewport: { width: 1600, height: 1000 } });
  let clicked = false;
  page.on("console", message => { const text = message.text(); if (clicked && /fem2d (maintenance|retire|close)|live cleanup|refused|zero-progress/.test(text) || /unnamed|rejected|invariant|pageerror|panicked/i.test(text)) console.log(`console ${text.slice(0, 1500)}`); });
  page.on("pageerror", error => console.log(`pageerror ${error.message.slice(0, 600)}`));
  await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120000 });
  await page.waitForFunction(() => Boolean(document.querySelector("[data-semio-os-ready], [data-semio-os-error]")), null, { timeout: 120000 });
  await page.getByRole("combobox", { name: "Example", exact: true }).getByText("Demo", { exact: true }).waitFor({ state: "visible", timeout: 180000 });
  await page.waitForTimeout(20000);
  const artifact = page.getByRole("button", { name: "Artifact", exact: true }).first();
  await artifact.click();
  const n1 = page.getByText(/^n1 · \(0[.,]00,/).first();
  console.log(`n1 ${await n1.evaluate(node => `${node.tagName} ${node.getAttribute("role")} ${JSON.stringify(node.textContent)} ${node.closest("[role]")?.outerHTML.slice(0, 400)}`)}`);
  clicked = true;
  await n1.click();
  await page.waitForTimeout(8000);
  console.log(`selected ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("[aria-selected=true],[aria-current],[data-selected=true]")].map(node => node.textContent?.slice(0, 60))))}`);
  await page.getByRole("button", { name: "Inspection", exact: true }).first().click();
  await page.getByRole("spinbutton", { name: "X", exact: true }).first().waitFor({ state: "visible", timeout: 60000 }).catch(() => {});
  console.log(`x ${await page.getByRole("spinbutton", { name: "X", exact: true }).or(page.getByLabel("X", { exact: true })).count()}`);
  const x = page.getByRole("spinbutton", { name: "X", exact: true }).first();
  if (await x.count()) { await x.fill("1.5"); await x.press("Enter"); if (process.argv.includes("--click")) await artifact.click(); for (let t = 5; t <= 120; t += 5) { await page.waitForTimeout(5000); const n = await page.getByText(/^n1 · \(1[.,]50,/).count(); if (n || t % 30 === 0) console.log(`edited ${n} at ${t}s`); if (n) break; } }
  console.log(`inputs ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("input,select,button,textarea,[role=spinbutton],[role=slider],[role=switch],[role=checkbox]")].filter(node => !(node.getAttribute("aria-label") || node.getAttribute("aria-labelledby") || node.textContent?.trim() || node.id && document.querySelector(`label[for="${node.id}"]`))).map(node => node.outerHTML.slice(0, 300))))}`);
  await page.screenshot({ path: fileURLToPath(new URL("./🗑️generated/inspect-node.png", import.meta.url)) });
} finally {
  await browser.close();
}
