/** 🔬️ Ticket tool of work package C1b: checks the lifting of the product on the harness page (`c1b_harness`, port 6255) in headless Chromium — a lifted copy beside its transparent original on a light and a dark page (screenshots), the original still taking clicks at its place, and an instant give-back on hover, focus, a filled field, a picked option, a click and, on a quiet stage, any key; no transition on any row, no report of the Content-Security-Policy (`style-src-attr 'none'`), no console output, no sideways scroll. Prints one JSON object of measurements and the list of failed expectations (empty when all hold).
 *
 * Usage (from the repository root, harness running on 6255):
 *   node ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/c1b_browser_check.mjs" [--out <dir>] [--scenery]
 *
 * `--scenery` also draws the layer's scenery (a tilted pet on a rope with gun and hook, a standing ladder, particles)
 * and checks its order in the layer; it needs the harness served with the whole core (not `C1B_PETS=subset`).
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const here = dirname(fileURLToPath(import.meta.url));
const args = process.argv.slice(2);
const out = resolve(args.includes("--out") ? args[args.indexOf("--out") + 1] : join(here, "🗑️generated", "c1b", "browser"));
mkdirSync(out, { recursive: true });
const base = "http://127.0.0.1:6255/";
const failed = [];
const report = {};
const expect = (name, holds, detail) => {
  if (!holds) failed.push(`${name}: ${JSON.stringify(detail)}`);
};

const state = () => {
  const rows = [...document.querySelectorAll("[data-pet-prop]")];
  const layer = document.querySelector(".pet-layer");
  const copies = [...layer.children].filter((child) => child.localName !== "svg");
  return {
    copies: copies.length,
    opacity: rows.map((row) => getComputedStyle(row).opacity),
    inline: rows.map((row) => row.getAttribute("style")),
    animations: document.getAnimations().length,
    heard: [...window.c1b.heard],
    transitions: window.c1b.transitions.filter((entry) => entry.endsWith(":opacity") || entry.endsWith(":all")),
    violations: [...window.c1b.violations],
    clicks: [...window.c1b.clicks],
    sideways: document.documentElement.scrollWidth - window.innerWidth,
  };
};

const boxes = (index) => {
  const row = document.querySelectorAll("[data-pet-prop]")[index];
  const copy = [...document.querySelector(".pet-layer").children].find((child) => child.localName !== "svg");
  const original = row.getBoundingClientRect();
  const middle = { x: original.left + original.width / 2, y: original.top + original.height / 2 };
  const hit = document.elementFromPoint(middle.x, middle.y);
  const copied = copy?.getBoundingClientRect();
  return {
    original: { left: original.left, top: original.top, width: original.width, height: original.height },
    copy: copied === undefined ? null : { left: copied.left, top: copied.top, width: copied.width, height: copied.height },
    hitIsOriginal: hit !== null && hit.closest("[data-pet-prop]") === row,
    hitInLayer: hit !== null && hit.closest(".pet-layer") !== null,
    copyHit: copy === undefined ? null : document.elementsFromPoint(copied.left + copied.width - 4, copied.top + copied.height / 2).some((element) => copy.contains(element)),
    copyAttributes: copy === undefined ? [] : [...new Set([copy, ...copy.querySelectorAll("*")].flatMap((node) => node.getAttributeNames()))].sort(),
    middle,
  };
};

const browser = await chromium.launch();
const open = async (query) => {
  const context = await browser.newContext({ viewport: { width: 1280, height: 720 } });
  const page = await context.newPage();
  const noise = [];
  page.on("console", (message) => {
    if (!message.text().startsWith("[vite]")) noise.push(`${message.type()}: ${message.text()}`);
  });
  page.on("pageerror", (error) => noise.push(`pageerror: ${error}`));
  await page.goto(`${base}?${query}`, { waitUntil: "load" });
  await page.waitForFunction(() => window.c1b !== undefined);
  await page.waitForTimeout(400);
  return { context, page, noise };
};

try {
  for (const theme of ["light", "dark"]) {
    const { context, page, noise } = await open(`theme=${theme}`);
    const result = {};
    await page.mouse.move(2, 2);
    await page.evaluate(() => window.c1b.rest(1));
    await page.waitForTimeout(100);
    const lifted = await page.evaluate(state);
    const geometry = await page.evaluate(boxes, 1);
    const room = await page.evaluate(() => window.c1b.room);
    result.lifted = { ...lifted, geometry, room };
    await page.screenshot({ path: join(out, `lifted-${theme}.png`) });
    await page.screenshot({ path: join(out, `lifted-${theme}-card.png`), clip: { x: geometry.original.left - 80, y: geometry.original.top - 90, width: geometry.original.width + 200, height: 200 } });
    expect(`${theme}: one copy`, lifted.copies === 1, lifted.copies);
    expect(`${theme}: original transparent, the others not`, lifted.opacity.join(" ") === "1 0 1", lifted.opacity);
    expect(`${theme}: copy beside the original`, geometry.copy !== null && Math.abs(geometry.copy.left - geometry.original.left - room) < 0.6 && Math.abs(geometry.copy.top - geometry.original.top) < 0.6 && Math.abs(geometry.copy.width - geometry.original.width) < 0.6 && Math.abs(geometry.copy.height - geometry.original.height) < 0.6, geometry);
    expect(`${theme}: the original takes the point`, geometry.hitIsOriginal && !geometry.hitInLayer, geometry);
    expect(`${theme}: the copy takes no point`, geometry.copyHit === false, geometry.copyHit);
    expect(`${theme}: the copy carries no hooks`, geometry.copyAttributes.every((name) => ["class", "style", "inert", "aria-hidden", "viewBox", "d", "type", "value", "selected"].includes(name)), geometry.copyAttributes);
    expect(`${theme}: no sideways scroll`, lifted.sideways <= 0, lifted.sideways);

    await page.evaluate(() => window.c1b.at(1, 50));
    await page.screenshot({ path: join(out, `shove-${theme}-card.png`), clip: { x: geometry.original.left - 80, y: geometry.original.top - 90, width: geometry.original.width + 200, height: 200 } });

    await page.evaluate(() => window.c1b.rest(1));
    await page.mouse.click(geometry.middle.x, geometry.middle.y);
    result.click = await page.evaluate(state);
    expect(`${theme}: a click on the original's place reaches it and gives it back`, result.click.clicks[1] === 1 && result.click.copies === 0 && result.click.opacity[1] === "1" && result.click.heard.length === 1, result.click);

    await page.mouse.move(2, 2);
    await page.evaluate(() => window.c1b.clear());
    await page.evaluate(() => window.c1b.rest(1));
    await page.mouse.move(geometry.middle.x, geometry.middle.y);
    result.hover = await page.evaluate(state);
    expect(`${theme}: hover gives back at once, without a transition`, result.hover.copies === 0 && result.hover.opacity[1] === "1" && result.hover.heard.length === 2 && result.hover.animations === 0, result.hover);

    await page.mouse.move(2, 2);
    await page.evaluate(() => window.c1b.clear());
    await page.evaluate(() => window.c1b.rest(2));
    await page.focus("#guess");
    result.focus = await page.evaluate(state);
    expect(`${theme}: focus gives back at once`, result.focus.copies === 0 && result.focus.opacity[2] === "1" && result.focus.heard.length === 3, result.focus);
    await page.evaluate(() => document.activeElement.blur());

    await page.evaluate(() => window.c1b.clear());
    await page.evaluate(() => window.c1b.rest(2));
    await page.fill("#guess", "13");
    result.fill = await page.evaluate(state);
    expect(`${theme}: a filled field gives back at once`, result.fill.copies === 0 && result.fill.opacity[2] === "1" && result.fill.heard.length === 4, result.fill);
    await page.evaluate(() => document.activeElement.blur());

    await page.evaluate(() => window.c1b.clear());
    await page.evaluate(() => {
      const unit = document.querySelector("#unit");
      unit.selectedIndex = 2;
      window.c1b.rest(2);
      unit.dispatchEvent(new Event("input", { bubbles: true }));
    });
    result.scripted = await page.evaluate(state);
    expect(`${theme}: an input dispatched by a script gives back at once`, result.scripted.copies === 0 && result.scripted.heard.length === 5, result.scripted);

    await page.evaluate(() => window.c1b.clear());
    await page.evaluate(() => window.c1b.rest(2));
    await page.selectOption("#unit", { index: 0 });
    result.select = await page.evaluate(state);
    expect(`${theme}: a picked option gives back at once`, result.select.copies === 0 && result.select.heard.length === 6, result.select);
    await page.evaluate(() => document.activeElement.blur());

    await page.evaluate(() => window.c1b.clear());
    const path = await page.evaluate(() => {
      const seen = [];
      for (let age = 0; age <= 700; age += 2) {
        window.c1b.at(1, age);
        const layer = document.querySelector(".pet-layer");
        const copy = [...layer.children].find((child) => child.localName !== "svg");
        seen.push({ age, copy: copy === undefined ? null : Number(copy.style.opacity), original: getComputedStyle(document.querySelectorAll("[data-pet-prop]")[1]).opacity });
      }
      window.c1b.clear();
      return seen;
    });
    const doubled = path.filter((entry) => entry.copy !== null && entry.copy < 1 && entry.original === "0");
    const holes = path.filter((entry) => entry.copy === null && entry.original === "0");
    result.path = { samples: path.length, withCopy: path.filter((entry) => entry.copy !== null).length, transparent: path.filter((entry) => entry.original === "0").length, doubled: doubled.length, holes: holes.length };
    expect(`${theme}: never a hole or a half-transparent pair along the whole lift`, doubled.length === 0 && holes.length === 0, result.path);

    if (args.includes("--scenery")) {
    await page.evaluate(() => window.c1b.gear());
    result.scenery = await page.evaluate(() => {
      const layer = document.querySelector(".pet-layer");
      const pet = layer.querySelector("svg.pet");
      return {
        order: [...layer.children].map((child) => child.getAttribute("class")),
        transform: pet.style.transform,
        palette: ["--pet-body", "--pet-accent", "--pet-detail"].map((name) => pet.style.getPropertyValue(name)),
        tools: [...pet.querySelectorAll("[data-pet-tool]")].filter((tool) => tool.getAttribute("visibility") === "visible").map((tool) => tool.getAttribute("data-pet-tool")),
        ladders: layer.querySelectorAll('.pet-ladders [visibility="visible"]').length,
        particles: layer.querySelectorAll('.pet-effects [visibility="visible"]').length,
      };
    });
    const card = await page.evaluate(() => {
      const box = document.querySelector(".card").getBoundingClientRect();
      return { x: box.left, y: box.top, height: box.height };
    });
    await page.screenshot({ path: join(out, `scenery-${theme}.png`), clip: { x: card.x - 200, y: card.y - 60, width: 520, height: card.height + 260 } });
    expect(`${theme}: scenery in the layer's order — ladders behind the pet, particles in front`, result.scenery.order.join(" ") === "pet-ladders pet pet-effects", result.scenery.order);
    expect(`${theme}: the pet is tilted about its pivot`, /rotate\(/.test(result.scenery.transform), result.scenery.transform);
    expect(`${theme}: rope, hook and gun drawn`, ["rope", "hook", "gun"].every((tool) => result.scenery.tools.includes(tool)), result.scenery.tools);
    expect(`${theme}: a ladder and particles drawn`, result.scenery.ladders === 1 && result.scenery.particles > 0, result.scenery);
    await page.evaluate(() => window.c1b.ungear());
    }

    result.end = await page.evaluate(state);
    expect(`${theme}: no opacity transition ran on any row (the host gives rows one of 300 ms)`, result.end.transitions.length === 0, result.end.transitions);
    expect(`${theme}: no policy report`, result.end.violations.length === 0, result.end.violations);
    expect(`${theme}: the rows wear nothing of the lift afterwards`, result.end.inline.join("|") === "|letter-spacing: 0.2px;|", result.end.inline);
    expect(`${theme}: console silent`, noise.length === 0, noise);
    result.noise = noise;
    report[theme] = result;
    await context.close();
  }

  {
    const { context, page, noise } = await open("motion=reduced");
    await page.mouse.move(2, 2);
    await page.evaluate(() => window.c1b.rest(0));
    await page.evaluate(() => window.c1b.clear());
    await page.evaluate(() => window.c1b.rest(1));
    const { middle } = await page.evaluate(boxes, 1);
    await page.mouse.move(middle.x, middle.y);
    await page.mouse.move(2, 2);
    await page.waitForTimeout(100);
    const reduced = await page.evaluate(state);
    report.reduced = { ...reduced, all: await page.evaluate(() => [...window.c1b.transitions]), noise };
    expect("reduced motion: given back on hover", reduced.copies === 0 && reduced.heard.length === 1, reduced);
    expect("reduced motion: no opacity transition ran on any row", reduced.transitions.length === 0, reduced.transitions);
    expect("reduced motion: console silent", noise.length === 0, noise);
    await context.close();
  }

  {
    const { context, page, noise } = await open("quiet");
    await page.mouse.move(2, 2);
    await page.evaluate(() => window.c1b.rest(0));
    await page.keyboard.press("Shift");
    const quiet = await page.evaluate(state);
    report.quiet = { ...quiet, noise };
    expect("quiet: any key anywhere gives back", quiet.copies === 0 && quiet.heard.length === 1, quiet);
    await context.close();
  }
} finally {
  await browser.close();
}

writeFileSync(join(out, "report.json"), JSON.stringify({ report, failed }, null, 2));
process.stdout.write(`${JSON.stringify({ failed, screenshots: out }, null, 2)}\n`);
process.exitCode = failed.length === 0 ? 0 : 1;
