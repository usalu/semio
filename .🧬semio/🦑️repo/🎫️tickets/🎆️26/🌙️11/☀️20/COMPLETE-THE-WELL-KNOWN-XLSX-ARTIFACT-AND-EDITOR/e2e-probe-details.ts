import {writeFileSync} from "node:fs";
import {join} from "node:path";
import {session} from "./e2e-harness.ts";
import {budgetWorkbook} from "./e2e-fixture.ts";

const out = join(import.meta.dir, "🗑️generated/e2e/probe-details");
const s = await session("en", process.env.SERVE ?? "http://127.0.0.1:6072/", out);
try {
  await s.boot();
  const opened = await s.openViaPicker(budgetWorkbook(), "Budget.xlsx");
  console.log("opened", opened.opened);
  await s.page.waitForTimeout(5000);
  const items = () => s.page.evaluate(() => [...document.querySelectorAll('[role="treeitem"]')].filter(e => (e as HTMLElement).offsetParent !== null).map(e => `${e.id}|${e.getAttribute("aria-expanded")}|${((e as HTMLElement).innerText ?? "").trim().split("\n")[0]}`));
  console.log("BEFORE\n" + (await items()).join("\n"));
  const opc = s.page.locator('[role="treeitem"]', {hasText: "Opc [opc]"}).first();
  await opc.dispatchEvent("click"); await s.page.waitForTimeout(2000);
  console.log("AFTER\n" + (await items()).join("\n"));
  await s.page.waitForTimeout(5000);
  console.log("AFTER-WAIT\n" + (await items()).join("\n"));
  const xp = s.page.locator('[role="treeitem"]', {hasText: "Xml parts [xmlParts]"}).first();
  await xp.dispatchEvent("click"); await s.page.waitForTimeout(2000);
  console.log("XP-TOGGLE1\n" + (await items()).join("\n"));
  await xp.dispatchEvent("click"); await s.page.waitForTimeout(4000);
  console.log("XP-TOGGLE2\n" + (await items()).join("\n"));
  await s.shot("details-xmlparts");
  const keys = await s.page.evaluate(() => [...document.querySelectorAll("[data-ui-node-key]")].filter(e => (e as HTMLElement).offsetParent !== null && /sd-|snapshot-details/.test(e.getAttribute("data-ui-node-key") ?? "")).map(e => `${e.getAttribute("data-ui-node-key")}=${(e as HTMLInputElement).value ?? ""}`).slice(0, 60));
  console.log("KEYS\n" + keys.join("\n"));
  await s.shot("details");
} finally { await s.browser.close(); }
