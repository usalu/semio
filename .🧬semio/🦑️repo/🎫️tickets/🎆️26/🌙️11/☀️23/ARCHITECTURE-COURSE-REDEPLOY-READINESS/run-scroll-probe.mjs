/** 📜️ Starts a run of a quiz on the dev page as an anonymous learner and says, for the task on screen, what scrolls and how far: every scroll container from the task card up, the boxes of its children, and a picture of the page scrolled to its end. Usage: node run-scroll-probe.mjs <output directory> [quiz] [width] [height] */
import { createRequire } from "node:module";
import { mkdirSync } from "node:fs";
import { join } from "node:path";
const require = createRequire("C:/semio/.cache/b/d/m/j/c642535c86e40717744c/node_modules/");
const { chromium } = require("playwright");
const out = process.argv[2];
mkdirSync(out, { recursive: true });
const quiz = process.argv[3] ?? "physics";
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: Number(process.argv[4] ?? 1920), height: Number(process.argv[5] ?? 915) } });
await page.goto("http://127.0.0.1:6061/#" + quiz);
const intro = page.locator('[data-slot="introduction-info-box"]');
const start = page.locator(`[data-layered-pane="${quiz}"] [data-card="quiz-challenge"] [data-overview-card-action="primary"]`);
await start.or(intro).first().waitFor({ timeout: 180000 });
await page.waitForTimeout(1500);
if ((await intro.count()) > 0) await intro.locator('[data-slot="introduction-close"]').click();
await page.locator(`[data-layered-pane="${quiz}"] [data-card="quiz-challenge"] [data-overview-card-action="primary"]`).click();
const identity = page.getByRole("dialog").filter({ has: page.locator('input[type="radio"][value="anonymous"]') });
await identity.waitFor({ timeout: 30000 });
await identity.locator('input[type="radio"][value="anonymous"]').locator("xpath=ancestor::label[1]").click();
await identity.locator('[data-overview-card-action="primary"]').click();
const run = page.locator('#course-main [data-card="run"]').and(page.locator(":not([inert] *)"));
await run.waitFor({ timeout: 60000 });
await page.waitForTimeout(2500);
await page.screenshot({ path: join(out, "run-top.png") });
console.log(await page.evaluate(() => {
  const lines = [];
  const task = document.querySelector('#course-main [data-card="task"]:not([inert] *)') ?? document.querySelector('#course-main [data-card="run"]');
  const box = (e) => { const r = e.getBoundingClientRect(); return [r.x, r.y, r.width, r.height].map(Math.round).join(","); };
  const name = (e) => e.tagName.toLowerCase() + [...e.attributes].filter((a) => a.name.startsWith("data-") && a.value.length < 26).map((a) => `[${a.name}${a.value ? "=" + a.value : ""}]`).join("").slice(0, 110) + "." + String(e.className).slice(0, 70);
  for (let e = task; e && e !== document.documentElement; e = e.parentElement) { const s = getComputedStyle(e); lines.push("up " + box(e) + " sh=" + e.scrollHeight + " ch=" + e.clientHeight + " ov=" + s.overflowY + " minH=" + s.minHeight + " pb=" + s.paddingBottom + " inline=" + (e.getAttribute("style") ?? "").slice(0, 80) + " " + name(e)); }
  const scroller = [...document.querySelectorAll("#course-main *, .course-app")].filter((e) => /auto|scroll/.test(getComputedStyle(e).overflowY) && e.scrollHeight - e.clientHeight > 20 && !e.closest("[inert]")).sort((a, b) => (b.scrollHeight - b.clientHeight) - (a.scrollHeight - a.clientHeight))[0];
  if (scroller) { lines.push("SCROLLER " + box(scroller) + " sh=" + scroller.scrollHeight + " ch=" + scroller.clientHeight + " " + name(scroller)); const walk = (e, level) => { if (level > 4) return; for (const child of e.children) { const r = child.getBoundingClientRect(); if (r.height < 1) continue; lines.push("  ".repeat(level + 1) + box(child) + " " + name(child) + " mb=" + getComputedStyle(child).marginBottom + " pb=" + getComputedStyle(child).paddingBottom + " minH=" + getComputedStyle(child).minHeight); walk(child, level + 1); } }; walk(scroller, 0); scroller.scrollTop = scroller.scrollHeight; }
  return lines.join("\n");
}));
const measure = (label) => page.evaluate((label) => { const scroll = document.querySelector("#course-main .course-page-scroll:not([inert] *)"); const main = document.querySelector("main"); const task = document.querySelector('#course-main [data-card="task"]:not([inert] *)'); const tall = [...document.querySelectorAll("#course-main *")].filter((e) => !e.closest("[inert]") && e.getBoundingClientRect().height > 1200).map((e) => e.tagName.toLowerCase() + "." + String(e.className).slice(0, 40) + "=" + Math.round(e.getBoundingClientRect().height) + (e.style.minBlockSize ? " min=" + e.style.minBlockSize : "")).slice(0, 8); return label + ": scroll sh=" + scroll?.scrollHeight + " ch=" + scroll?.clientHeight + " main.min=" + main.style.minBlockSize + " task=" + Math.round(task?.getBoundingClientRect().height ?? -1) + " app sh=" + document.querySelector(".course-app").scrollHeight + " tall=" + JSON.stringify(tall); }, label);
console.log(await measure("start"));
const task = page.locator('#course-main [data-card="task"]').and(page.locator(":not([inert] *)"));
const selects = task.locator("select");
await selects.first().selectOption({ index: 1 });
await page.waitForTimeout(800);
console.log(await measure("one assigned"));
for (let index = 1; index < (await selects.count()); index += 1) await selects.nth(index).selectOption({ index: 1 + (index % 2) });
await page.waitForTimeout(1200);
console.log(await measure("all assigned"));
await selects.first().focus();
await page.keyboard.press("Tab");
await page.waitForTimeout(800);
console.log(await measure("after tab"));
const reveal = task.locator("[data-crowd-gate] button").first();
if ((await reveal.count()) > 0) { await reveal.click(); await page.waitForTimeout(1500); console.log(await measure("crowd shown")); }
console.log(await page.evaluate(() => [...document.querySelectorAll("#course-main [tabindex], #course-main [role=button]")].filter((e) => !e.closest("[inert]") && e.getAttribute("tabindex") !== "-1" && e.getBoundingClientRect().height > 120).map((e) => e.tagName.toLowerCase() + "." + String(e.className).slice(0, 50) + " h=" + Math.round(e.getBoundingClientRect().height) + " sh=" + e.scrollHeight).join(" | ")));
await page.evaluate(() => { const scroll = document.querySelector("#course-main .course-page-scroll:not([inert] *)"); scroll.scrollTop = scroll.scrollHeight; });
await page.waitForTimeout(600);
await page.screenshot({ path: join(out, "run-end.png") });
if (process.argv[6] === "submit") {
  await page.locator('#course-main [data-card="run"] [data-overview-card-action="primary"]').first().click();
  const confirm = page.getByRole("alertdialog");
  await confirm.waitFor({ timeout: 20000 });
  await confirm.locator('[data-overview-card-action="primary"]').click();
  await page.locator('#course-main [data-card="results"]').and(page.locator(":not([inert] *)")).waitFor({ timeout: 60000 });
  await page.waitForTimeout(2500);
  console.log(await page.evaluate(() => { const scroll = document.querySelector("#course-main .course-page-scroll:not([inert] *)"); return "results: scroll sh=" + scroll?.scrollHeight + " ch=" + scroll?.clientHeight + " figures=" + [...document.querySelectorAll("#course-main [data-crowd-figure]")].filter((e) => !e.closest("[inert]")).map((e) => e.getAttribute("data-crowd-figure") + ":" + [e.getBoundingClientRect().width, e.getBoundingClientRect().height].map(Math.round).join("x")).join(" "); }));
  await page.screenshot({ path: join(out, "results.png"), fullPage: false });
  await page.evaluate(() => { const scroll = document.querySelector("#course-main .course-page-scroll:not([inert] *)"); scroll.scrollTop = 500; });
  await page.waitForTimeout(500);
  await page.screenshot({ path: join(out, "results-2.png") });
}
await browser.close();
