/** 🔬️ Ticket tool of work package G: targeted checks of the pet layer on the harness page in headless Chromium — what a still stage asks of the browser, whether pets ride a scrolling pane, what happens while the document is hidden, under an inert page, under forced colours and after the layer is gone. Prints one JSON object of measurements and a list of failed expectations (empty when all hold).
 *
 * Usage (from the repository root, harness running on 6199):
 *   node ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/wp_g_probe_layer.mjs" [--base http://127.0.0.1:6199/] [--out <dir for screenshots>]
 */
import { mkdirSync } from "node:fs";
import { resolve } from "node:path";
import { chromium } from "playwright";

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const index = args.indexOf(`--${name}`);
  return index < 0 ? fallback : args[index + 1];
};
const base = option("base", "http://127.0.0.1:6199/");
const out = option("out", "");
if (out) mkdirSync(resolve(out), { recursive: true });

const pets = () =>
  [...document.querySelectorAll(".pet-layer svg.pet")].map((pet) => {
    const match = /translate\((-?[\d.]+)px, (-?[\d.]+)px\)/.exec(pet.style.transform) ?? [0, "NaN", "NaN"];
    return { id: pet.getAttribute("data-pet"), x: Number(match[1]), y: Number(match[2]), opacity: Number(pet.style.opacity) };
  });
const probe = () => ({ ...window.petProbe, console: [...window.petProbe.console] });
const frames = async (page) => (await page.evaluate(probe)).frames;

const browser = await chromium.launch();
const report = {};
const failed = [];
const expect = (name, holds, detail) => {
  if (!holds) failed.push(`${name}: ${JSON.stringify(detail)}`);
};
const open = async (query, contextOptions = {}) => {
  const context = await browser.newContext({ viewport: { width: 1280, height: 800 }, ...contextOptions });
  const page = await context.newPage();
  const noise = [];
  page.on("console", (message) => {
    if (!/\[vite\]|React DevTools/.test(message.text())) noise.push(`${message.type()}: ${message.text()}`);
  });
  page.on("pageerror", (error) => noise.push(`pageerror: ${error}`));
  await page.goto(`${base}?${query}`, { waitUntil: "load" });
  await page.waitForTimeout(1200);
  return { context, page, noise };
};

try {
  {
    const { context, page, noise } = await open("mode=still&seed=4&capacity=6&menagerie=architecture");
    const first = await page.evaluate(probe);
    const standing = await page.evaluate(pets);
    await page.mouse.move(900, 300, { steps: 10 });
    await page.mouse.move(200, 600, { steps: 10 });
    await page.waitForTimeout(3000);
    const later = await page.evaluate(probe);
    report.still = { pets: standing.length, framesAfterMount: first.frames, framesAfterThreeSeconds: later.frames, timers: later.timers, console: later.console };
    expect("still: pets stand", standing.length === 6 && standing.every((pet) => pet.opacity === 1), standing);
    expect("still: at most one frame per mount (two under StrictMode), none later", first.frames <= 2 && later.frames === first.frames, report.still);
    expect("still: no timer", later.timers === 0, later.timers);
    expect("still: positions unchanged", JSON.stringify(await page.evaluate(pets)) === JSON.stringify(standing), null);

    const riders = await page.evaluate(() => {
      const inPane = [...document.querySelectorAll("#pane [data-pet-surface]")].map((card) => card.getBoundingClientRect().top);
      return inPane;
    });
    const before = await page.evaluate(pets);
    const paneBefore = await page.evaluate(() => document.querySelector("#pane .card").getBoundingClientRect().top);
    await page.evaluate(() => {
      document.querySelector("#pane").scrollTop = 20;
    });
    await page.waitForTimeout(300);
    const paneAfter = await page.evaluate(() => document.querySelector("#pane .card").getBoundingClientRect().top);
    const after = await page.evaluate(pets);
    const moved = before.map((pet, index) => ({ id: pet.id, from: pet.y, to: after.find((each) => each.id === pet.id)?.y ?? null, onPane: Math.abs(pet.y - paneBefore) < 31 && pet.x > 760 }));
    report.scroll = { cardTops: riders, cardMovedBy: paneAfter - paneBefore, moved, framesForScroll: (await frames(page)) - later.frames };
    for (const pet of moved) expect(`scroll: ${pet.id} ${pet.onPane ? "rides the card" : "stays"}`, pet.to !== null && Math.abs(pet.to - pet.from - (pet.onPane ? paneAfter - paneBefore : 0)) < 0.01, pet);
    expect("scroll: a still stage needs a frame per scroll, no more than a few", report.scroll.framesForScroll >= 1 && report.scroll.framesForScroll <= 3, report.scroll.framesForScroll);
    if (out) await page.screenshot({ path: resolve(out, "still-after-scroll.png") });

    await page.evaluate(() => document.querySelector("main").setAttribute("inert", ""));
    await page.waitForTimeout(600);
    const inert = await page.evaluate(pets);
    report.inert = { pets: inert.map((pet) => ({ id: pet.id, y: pet.y })), floor: 800 };
    expect("inert: nobody stands on a surface of an inert page (everyone on the floor or gone)", inert.every((pet) => Math.abs(pet.y - 800) <= 30), inert);
    if (out) await page.screenshot({ path: resolve(out, "still-inert-main.png") });
    await page.evaluate(() => document.querySelector("main").removeAttribute("inert"));
    await page.waitForTimeout(400);
    expect("still: console clean", noise.length === 0 && (await page.evaluate(probe)).console.length === 0, noise);
    await context.close();
  }

  {
    const { context, page, noise } = await open("mode=calm&seed=4&capacity=6&menagerie=architecture");
    await page.waitForTimeout(2000);
    const start = await frames(page);
    await page.waitForTimeout(3000);
    const running = (await frames(page)) - start;
    await page.evaluate(() => {
      Object.defineProperty(document, "visibilityState", { configurable: true, get: () => "hidden" });
      document.dispatchEvent(new Event("visibilitychange"));
    });
    const hiddenAt = await page.evaluate(probe);
    const frozen = JSON.stringify(await page.evaluate(pets));
    await page.mouse.move(640, 300, { steps: 5 });
    await page.waitForTimeout(3000);
    const hiddenLater = await page.evaluate(probe);
    const still = JSON.stringify(await page.evaluate(pets));
    await page.evaluate(() => {
      Object.defineProperty(document, "visibilityState", { configurable: true, get: () => "visible" });
      document.dispatchEvent(new Event("visibilitychange"));
    });
    await page.waitForTimeout(2000);
    const resumed = (await frames(page)) - hiddenLater.frames;
    report.hidden = { framesInThreeSecondsRunning: running, framesWhileHidden: hiddenLater.frames - hiddenAt.frames, timersWhileHidden: hiddenLater.timers - hiddenAt.timers, framesInTwoSecondsAfterReturn: resumed, pets: (await page.evaluate(pets)).length };
    expect("hidden: frames run while visible", running > 60, running);
    expect("hidden: no frame and no timer while hidden", report.hidden.framesWhileHidden === 0 && report.hidden.timersWhileHidden === 0, report.hidden);
    expect("hidden: nothing painted while hidden", frozen === still, null);
    expect("hidden: resumes", resumed > 40 && report.hidden.pets === 6, report.hidden);
    expect("calm: console clean", noise.length === 0 && (await page.evaluate(probe)).console.length === 0, noise);

    {
      const standing = JSON.stringify(await page.evaluate(pets));
      const start = await frames(page);
      await page.evaluate(() => {
        for (const child of document.querySelector(".app").children) child.setAttribute("inert", "");
      });
      await page.waitForTimeout(300);
      const settled = await frames(page);
      await page.waitForTimeout(3000);
      const during = await frames(page);
      const waiting = JSON.stringify(await page.evaluate(pets));
      if (out) await page.screenshot({ path: resolve(out, "calm-dialog-inert.png") });
      await page.evaluate(() => {
        for (const child of document.querySelector(".app").children) child.removeAttribute("inert");
      });
      await page.waitForTimeout(150);
      const after = await page.evaluate(pets);
      await page.waitForTimeout(1850);
      const heights = (list) => list.map((pet) => `${pet.id}@${pet.y}`).sort().join(" ");
      report.dialog = { framesUntilRest: settled - start, framesWhileInert: during - settled, framesInTwoSecondsAfter: (await frames(page)) - during, heightsBefore: heights(JSON.parse(standing)), heightsWhileInert: heights(JSON.parse(waiting)), heightsAfter: heights(after) };
      expect("dialog: the show rests while every child of the app is inert", report.dialog.framesWhileInert === 0, report.dialog);
      expect("dialog: nobody fell while it rested or when it resumed", report.dialog.heightsWhileInert === report.dialog.heightsBefore && report.dialog.heightsAfter === report.dialog.heightsBefore, report.dialog);
      expect("dialog: resumes", report.dialog.framesInTwoSecondsAfter > 40, report.dialog);
    }

    report.cost = await page.evaluate(() => {
      const bulk = document.createElement("div");
      bulk.style.cssText = "position:absolute;left:0;top:60px;width:1200px;height:600px;overflow:hidden";
      for (let section = 0; section < 10; section++) {
        const pane = document.createElement("section");
        if (section > 0) pane.setAttribute("inert", "");
        for (let index = 0; index < 12; index++) {
          const card = document.createElement("article");
          card.setAttribute("data-pet-surface", "");
          card.style.cssText = "display:inline-block;width:180px;margin:4px";
          card.innerHTML = "<h3>Card</h3>" + "<p>Some words to keep free.</p><button>Go</button><a href='#x'>link</a><ul><li>one</li><li>two</li></ul>".repeat(2);
          pane.append(card);
        }
        bulk.append(pane);
      }
      document.querySelector("main").append(bulk);
      const elements = bulk.querySelectorAll("*").length;
      const first = window.petSurvey(document);
      const rounds = 200;
      const start = performance.now();
      for (let round = 0; round < rounds; round++) window.petSurvey(document);
      const each = (performance.now() - start) / rounds;
      bulk.remove();
      return { elements, surfaces: first.surfaces.length, keepouts: first.keepouts.length, millisecondsPerSurvey: Number(each.toFixed(3)) };
    });
    expect("cost: a survey of a page of 1700 elements, nine tenths of them inert, stays under 1.5 ms", report.cost.millisecondsPerSurvey < 1.5, report.cost);

    await page.evaluate(() => document.querySelector("#root").remove());
    const cleared = await page.evaluate(() => document.querySelectorAll("svg.pet").length);
    report.removed = { petsLeftInDocument: cleared };
    await context.close();
  }

  {
    const { context, page, noise } = await open("mode=lively&seed=4&capacity=6&menagerie=architecture", { forcedColors: "active" });
    const state = await page.evaluate(() => ({ display: getComputedStyle(document.querySelector(".pet-layer")).display, children: document.querySelector(".pet-layer").childElementCount, probe: { ...window.petProbe } }));
    report.forcedColors = { display: state.display, children: state.children, frames: state.probe.frames, timers: state.probe.timers };
    expect("forced colours: hidden, empty, nothing scheduled", state.display === "none" && state.children === 0 && state.probe.frames === 0 && state.probe.timers === 0, report.forcedColors);
    expect("forced colours: console clean", noise.length === 0, noise);
    await context.close();
  }

  {
    const { context, page, noise } = await open("mode=calm&seed=4&capacity=6&menagerie=architecture");
    const clicks = await page.evaluate(() => {
      const hits = [];
      document.addEventListener("click", (event) => hits.push(event.target.tagName), true);
      window.petClicks = hits;
      return hits.length;
    });
    const standing = await page.evaluate(pets);
    const target = standing[0];
    const under = await page.evaluate(([x, y]) => document.elementFromPoint(x, y)?.closest(".pet-layer") === null, [target.x, target.y - 14]);
    await page.mouse.click(target.x, target.y - 14);
    await page.click("text=Open");
    await page.waitForTimeout(300);
    const hits = await page.evaluate(() => window.petClicks);
    report.pointer = { pet: target.id, elementUnderPetIsNotTheLayer: under, clickTargets: hits, before: clicks };
    expect("pointer: the layer is never hit", under === true && hits.length === 2 && hits[1] === "BUTTON" && !hits.includes("svg"), report.pointer);
    expect("pointer: console clean", noise.length === 0, noise);
    await context.close();
  }
} finally {
  await browser.close();
}
console.log(JSON.stringify({ report, failed }, null, 1));
process.exit(failed.length === 0 ? 0 : 1);
