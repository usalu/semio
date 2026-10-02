/** 🔭️ Ticket tool of work package G: watches a page with a pet layer in headless Chromium for a while — screenshots every few seconds, twice a second a measurement of where every pet stands relative to the surfaces, the floor and what must stay free — while it moves the pointer, pokes a pet, scrolls a pane and moves the keyboard focus. Prints a JSON summary; everything the page logs counts as a finding.
 *
 * Usage (from the repository root):
 *   node ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/wp_g_watch_pets.mjs" --url http://127.0.0.1:6199/?seed=3 --out <dir> [--seconds 60] [--width 1280] [--height 800] [--pane "#pane"] [--before "<selector to click first>"]
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { chromium } from "playwright";

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const index = args.indexOf(`--${name}`);
  return index < 0 ? fallback : args[index + 1];
};
const url = option("url", "http://127.0.0.1:6199/");
const out = resolve(option("out", "pet-watch"));
const seconds = Number(option("seconds", "60"));
const width = Number(option("width", "1280"));
const height = Number(option("height", "800"));
const pane = option("pane", "#pane");
const before = option("before", "");
const reduced = option("reduced", "") === "1";
mkdirSync(out, { recursive: true });

const KEEP = "a[href], button, input, select, textarea, summary, p, li, h1, h2, h3, h4, h5, h6, label, [data-pet-keepout]";

const measure = (keep) => {
  const layer = document.querySelector(".pet-layer");
  const origin = layer ? layer.getBoundingClientRect() : { left: 0, top: 0, width: innerWidth, height: innerHeight };
  const boxOf = (rect) => ({ left: rect.left, top: rect.top, right: rect.right, bottom: rect.bottom });
  const pets = [...document.querySelectorAll(".pet-layer svg.pet")].map((pet) => {
    const match = /translate\((-?[\d.]+)px, (-?[\d.]+)px\) scale\((-?[\d.]+), (-?[\d.]+)\)/.exec(pet.style.transform) ?? [0, "NaN", "NaN", "1", "1"];
    let drawn = null;
    for (const child of pet.children) {
      const rect = child.getBoundingClientRect();
      if (rect.width === 0 && rect.height === 0) continue;
      drawn = drawn === null ? boxOf(rect) : { left: Math.min(drawn.left, rect.left), top: Math.min(drawn.top, rect.top), right: Math.max(drawn.right, rect.right), bottom: Math.max(drawn.bottom, rect.bottom) };
    }
    const pupil = pet.querySelector(".pet-pupil");
    return { id: pet.getAttribute("data-pet"), x: Number(match[1]) + origin.left, y: Number(match[2]) + origin.top, flip: Number(match[3]), scale: Number(match[4]), opacity: Number(pet.style.opacity), drawn, look: pupil ? Number(pupil.getAttribute("cx")) * Math.sign(Number(match[3])) : 0 };
  });
  const clipped = (element) => {
    let box = boxOf(element.getBoundingClientRect());
    for (let node = element.parentElement; node; node = node.parentElement) {
      const style = getComputedStyle(node);
      if (style.overflowX === "visible" && style.overflowY === "visible") continue;
      const rect = node.getBoundingClientRect();
      if (box.top < rect.top || box.top >= rect.bottom) return null;
      box = { left: Math.max(box.left, rect.left), top: box.top, right: Math.min(box.right, rect.right), bottom: Math.min(box.bottom, rect.bottom) };
    }
    return box;
  };
  const surfaces = [...document.querySelectorAll("[data-pet-surface]")].filter((element) => !element.closest("[inert], [hidden]")).map(clipped).filter((box) => box !== null);
  const kept = [...document.querySelectorAll(keep)].filter((element) => !element.closest("[inert], [hidden], .pet-layer") && !element.matches("[data-pet-surface]") && element.querySelector("[data-pet-surface]") === null).map((element) => ({ ...boxOf(element.getBoundingClientRect()), tag: element.tagName.toLowerCase(), text: (element.textContent ?? "").trim().slice(0, 24) }));
  return { pets, surfaces, kept, floor: origin.top + origin.height, hidden: document.visibilityState, probe: window.petProbe ? { ...window.petProbe, console: [...window.petProbe.console] } : null };
};

const browser = await chromium.launch();
const findings = { url, console: [], errors: [], failed: [], samples: 0, standing: 0, hovering: 0, airborne: 0, fading: 0, overKept: [], pets: new Set(), gaze: { asked: 0, followed: 0 }, looks: [], notes: [] };
try {
  const context = await browser.newContext({ viewport: { width, height }, deviceScaleFactor: Number(option("density", "2")), reducedMotion: reduced ? "reduce" : "no-preference" });
  const page = await context.newPage();
  page.on("console", (message) => findings.console.push(`${message.type()}: ${message.text()}`));
  page.on("pageerror", (error) => findings.errors.push(String(error)));
  page.on("requestfailed", (request) => findings.failed.push(`${request.url()} ${request.failure()?.errorText ?? ""}`));
  await page.goto(url, { waitUntil: "load" });
  if (before) await page.click(before);
  await page.waitForTimeout(1500);

  let pointer = null;
  let pointedAt = 0;
  const started = Date.now();
  const elapsed = () => (Date.now() - started) / 1000;
  const sample = async () => {
    const seen = await page.evaluate(measure, KEEP);
    findings.samples += 1;
    for (const pet of seen.pets) {
      findings.pets.add(pet.id);
      if (!(pet.opacity >= 0.99)) {
        findings.fading += 1;
        continue;
      }
      const ground = seen.surfaces.find((surface) => pet.x >= surface.left - 0.6 && pet.x <= surface.right + 0.6 && surface.top - pet.y >= -0.6 && surface.top - pet.y <= 40);
      const floor = seen.floor - pet.y >= -0.6 && seen.floor - pet.y <= 40;
      const gap = ground ? ground.top - pet.y : floor ? seen.floor - pet.y : null;
      if (gap === null) {
        findings.airborne += 1;
        continue;
      }
      if (gap <= 0.6) findings.standing += 1;
      else findings.hovering += 1;
      if (pet.drawn) for (const kept of seen.kept) if (pet.drawn.left < kept.right && pet.drawn.right > kept.left && pet.drawn.top < kept.bottom && pet.drawn.bottom > kept.top) findings.overKept.push({ at: Number(elapsed().toFixed(1)), pet: pet.id, x: pet.x, y: pet.y, over: `${kept.tag} "${kept.text}"`, overlap: [Math.min(pet.drawn.right, kept.right) - Math.max(pet.drawn.left, kept.left), Math.min(pet.drawn.bottom, kept.bottom) - Math.max(pet.drawn.top, kept.top)].map((value) => Number(value.toFixed(1))) });
      if (pointer !== null && Date.now() - pointedAt > 1200 && Date.now() - pointedAt < 3500 && Math.abs(pointer.x - pet.x) > 150) {
        findings.gaze.asked += 1;
        if (Math.sign(pet.look) === Math.sign(pointer.x - pet.x) && Math.abs(pet.look) > 0.2) findings.gaze.followed += 1;
      }
    }
    return seen;
  };
  const shot = async (name) => {
    await page.screenshot({ path: resolve(out, `${name}.png`) });
  };
  const portraits = async (name) => {
    const seen = await page.evaluate(measure, KEEP);
    for (const pet of seen.pets) {
      const view = page.viewportSize();
      const left = Math.max(0, Math.min(view.width - 80, pet.x - 40));
      const top = Math.max(0, Math.min(view.height - 70, pet.y - 62));
      await page.screenshot({ path: resolve(out, `${name}-${pet.id}.png`), clip: { x: left, y: top, width: 80, height: 70 } });
      findings.looks.push({ at: name, pet: pet.id, x: Math.round(pet.x), y: Math.round(pet.y), flip: pet.flip, look: Number(pet.look.toFixed(2)), pointer });
    }
  };
  const point = async (x, y) => {
    await page.mouse.move(x, y, { steps: 8 });
    pointer = { x, y };
    pointedAt = Date.now();
  };

  const plan = [
    { at: 4, name: "pointer-right", run: () => point(width - 30, 120) },
    { at: 10, name: "pointer-left", run: () => point(30, height - 200) },
    { at: 16, name: "pointer-middle", run: () => point(width / 2, 60) },
    {
      at: 22,
      name: "poke",
      run: async () => {
        const seen = await page.evaluate(measure, KEEP);
        const pet = seen.pets.find((each) => each.opacity >= 0.99);
        if (!pet) return findings.notes.push("poke: no pet to poke");
        await page.mouse.click(pet.x, pet.y - 14);
        pointer = null;
        findings.notes.push(`poke: clicked ${pet.id} at ${pet.x.toFixed(0)}, ${(pet.y - 14).toFixed(0)}`);
      },
    },
    {
      at: 30,
      name: "scroll-pane",
      run: async () => {
        const moved = await page.evaluate((selector) => {
          const element = document.querySelector(selector);
          if (!element) return null;
          element.scrollTop += 90;
          return element.scrollTop;
        }, pane);
        findings.notes.push(`scroll: ${pane} scrollTop = ${moved}`);
      },
    },
    {
      at: 38,
      name: "keyboard-focus",
      run: async () => {
        pointer = null;
        await page.keyboard.press("Tab");
        await page.keyboard.press("Tab");
        await page.keyboard.press("Tab");
        findings.notes.push(`focus: ${await page.evaluate(() => `${document.activeElement?.tagName} "${(document.activeElement?.textContent ?? "").trim().slice(0, 20)}"`)}`);
      },
    },
    { at: 46, name: "pointer-far-right", run: () => point(width - 10, height - 60) },
    {
      at: 52,
      name: "resize",
      run: async () => {
        pointer = null;
        await page.setViewportSize({ width: width - 180, height: height - 60 });
        findings.notes.push(`resize: ${width - 180}x${height - 60}`);
      },
    },
  ];

  await shot("00-start");
  let done = 0;
  let nextShot = 5;
  while (elapsed() < seconds) {
    const step = plan[done];
    if (step && elapsed() >= step.at) {
      await step.run();
      done += 1;
      await page.waitForTimeout(1300);
      await sample();
      await shot(`${String(Math.round(elapsed())).padStart(2, "0")}-${step.name}`);
      if (step.name.startsWith("pointer") || step.name === "poke") await portraits(`${String(Math.round(elapsed())).padStart(2, "0")}-${step.name}`);
      continue;
    }
    await sample();
    if (elapsed() >= nextShot) {
      await shot(`${String(Math.round(elapsed())).padStart(2, "0")}-life`);
      nextShot += 5;
    }
    await page.waitForTimeout(500);
  }
  const last = await sample();
  await shot(`${String(Math.round(elapsed())).padStart(2, "0")}-end`);
  findings.probe = last.probe;
  findings.last = last.pets.map((pet) => ({ id: pet.id, x: Number(pet.x.toFixed(1)), y: Number(pet.y.toFixed(1)), scale: pet.scale, opacity: pet.opacity }));
  findings.surfaces = last.surfaces.map((surface) => ({ left: Math.round(surface.left), right: Math.round(surface.right), top: Math.round(surface.top) }));
  findings.floor = last.floor;
} finally {
  await browser.close();
}
const summary = { ...findings, pets: [...findings.pets].sort(), overKept: findings.overKept.slice(0, 40), overKeptCount: findings.overKept.length };
writeFileSync(resolve(out, "summary.json"), JSON.stringify(summary, null, 1));
console.log(JSON.stringify(summary, null, 1));
