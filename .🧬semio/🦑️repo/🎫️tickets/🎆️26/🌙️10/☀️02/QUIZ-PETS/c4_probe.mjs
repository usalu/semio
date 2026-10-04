/** 🔬️ Ticket tool of work package C4: looks at the pets of the running site in headless Chromium the way the browser spec `🐕️pet-walk` will — enters as an anonymous learner, then per `--mode` dumps what every pet's depiction says (activity, footing, state, mood, place, the body box computed from the transform against the box the browser lays out), clicks a pet four times, drags one up and lets it go, or circles one — and writes a JSON log and screenshots.
 *
 * Usage (from the repository root):
 *   node ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/c4_probe.mjs" --url http://127.0.0.1:6243 --out <dir> --mode dump|click|drag|circle [--tempo 1] [--lively 1]
 */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const index = args.indexOf(`--${name}`);
  return index < 0 ? fallback : args[index + 1];
};
const url = option("url", "http://127.0.0.1:6243");
const out = resolve(option("out", "c4-probe"));
const mode = option("mode", "dump");
const tempo = option("tempo", "");
const lively = option("lively", "") === "1";
mkdirSync(out, { recursive: true });
const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../..");
const petsRoot = resolve(repoRoot, "🎓️teaching/🏛️architecture/🐾️pets");
const ensemble = JSON.parse(readFileSync(resolve(petsRoot, "🔣️.json"), "utf8"));
const species = Object.fromEntries(ensemble.species.map((path) => JSON.parse(readFileSync(resolve(petsRoot, path), "utf8"))).map((kind) => [kind.id, { width: kind.size.width, height: kind.size.height, hover: kind.locomotion.gait === "float" ? (kind.locomotion.hover ?? 0) : 0, gear: kind.gear, grip: kind.grip }]));

const log = [];
const note = (entry) => {
  log.push({ at: Date.now(), ...entry });
  process.stdout.write(`${JSON.stringify(entry)}\n`);
};

const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: "en-GB" });
if (tempo !== "") await context.addInitScript((value) => {
  const mark = () => {
    document.documentElement?.setAttribute("data-pets-tempo", value);
    return document.documentElement !== null;
  };
  if (mark()) return;
  const watch = new MutationObserver(() => mark() && watch.disconnect());
  watch.observe(document, { childList: true });
}, tempo);
const page = await context.newPage();
page.on("console", (message) => message.type() === "error" && note({ console: message.text() }));
page.on("pageerror", (error) => note({ pageerror: error.message }));
await page.goto(url);
await page.locator('#quiz-main [data-card="introduction"] [data-overview-card-action="primary"]').click();
await page.locator('#quiz-main [data-card="identity"] input[type="radio"][value="anonymous"]').check();
await page.locator('#quiz-main [data-card="identity"] [data-overview-card-action="primary"]').click();
await page.locator("[data-layered-overview]").waitFor();
await page.locator(".pet-layer svg.pet").first().waitFor({ timeout: 60_000 });
if (lively) {
  await page.evaluate(() => (window.location.hash = "prefs"));
  await page.locator('[data-layered-pane="prefs"][data-opened]').waitFor();
  await page.locator('[data-layered-pane="prefs"]').getByRole("group", { name: "Pets", exact: true }).getByRole("button").nth(3).click();
  await page.keyboard.press("Escape");
}
await page.mouse.move(2, 2);
await page.waitForTimeout(3000);

const pets = () =>
  page.evaluate((kinds) => {
    return [...document.querySelectorAll(".pet-layer svg.pet")].map((pet) => {
      const id = pet.getAttribute("data-pet");
      const kind = kinds[id];
      const ctm = pet.getScreenCTM();
      const corners = [[-kind.width / 2, -kind.height], [kind.width / 2, -kind.height], [kind.width / 2, 0], [-kind.width / 2, 0]].map(([x, y]) => ({ x: ctm.a * x + ctm.c * y + ctm.e, y: ctm.b * x + ctm.d * y + ctm.f }));
      const parts = [...pet.children].filter((child) => child.getAttribute("class") !== "pet-gear").map((child) => child.getBoundingClientRect()).filter((box) => box.width > 0 || box.height > 0);
      return {
        id,
        activity: pet.getAttribute("data-pet-activity"),
        footing: pet.getAttribute("data-pet-footing"),
        state: pet.getAttribute("data-pet-state"),
        mood: pet.getAttribute("data-pet-mood"),
        transform: pet.style.transform,
        opacity: pet.style.opacity,
        body: { left: Math.min(...corners.map((c) => c.x)), right: Math.max(...corners.map((c) => c.x)), top: Math.min(...corners.map((c) => c.y)), bottom: Math.max(...corners.map((c) => c.y)) },
        drawn: { left: Math.min(...parts.map((b) => b.left)), right: Math.max(...parts.map((b) => b.right)), top: Math.min(...parts.map((b) => b.top)), bottom: Math.max(...parts.map((b) => b.bottom)) },
      };
    });
  }, species);

const before = await pets();
note({ pets: before });
await page.screenshot({ path: resolve(out, `${mode}-0.png`) });

const pick = (list, wanted = () => true) => list.find((pet) => pet.activity === "idle" && pet.footing === "perch" && Number(pet.opacity || "1") > 0.99 && wanted(pet));
if (mode === "click") {
  const pet = pick(before);
  if (pet === undefined) note({ error: "no idle perched pet" });
  else {
    const x = (pet.body.left + pet.body.right) / 2;
    const y = (pet.body.top + pet.body.bottom) / 2;
    note({ clicking: pet.id, x, y, under: await page.evaluate(([px, py]) => document.elementFromPoint(px, py)?.outerHTML.slice(0, 120), [x, y]) });
    for (let click = 1; click <= 4; click++) {
      await page.mouse.click(x, y);
      const seen = new Set();
      for (let look = 0; look < 6; look++) {
        const now = (await pets()).find((candidate) => candidate.id === pet.id);
        seen.add(`${now.activity}/${now.footing}/${now.mood}`);
        await page.waitForTimeout(60);
      }
      note({ click, seen: [...seen], cursor: await page.evaluate(() => document.documentElement.getAttribute("data-pet-cursor")) });
      await page.screenshot({ path: resolve(out, `click-${click}.png`) });
    }
  }
}
if (mode === "drag") {
  const pet = pick(before, (candidate) => species[candidate.id].gear.includes("parachute")) ?? pick(before);
  if (pet === undefined) note({ error: "no idle perched pet" });
  else {
    const x = (pet.body.left + pet.body.right) / 2;
    const y = (pet.body.top + pet.body.bottom) / 2;
    note({ dragging: pet.id, gear: species[pet.id].gear, x, y });
    const toX = Number(option("to-x", String(x + 60)));
    const toY = Number(option("to-y", String(y - 240)));
    await page.mouse.move(x, y);
    await page.mouse.down();
    for (let step = 1; step <= 30; step++) {
      await page.mouse.move(x + ((toX - x) * step) / 30, y + ((toY - y) * step) / 30);
      await page.waitForTimeout(16);
    }
    const held = (await pets()).find((candidate) => candidate.id === pet.id);
    note({ held, cursor: await page.evaluate(() => document.documentElement.getAttribute("data-pet-cursor")) });
    await page.screenshot({ path: resolve(out, "drag-held.png") });
    for (let step = 0; step < 20; step++) {
      await page.mouse.move(toX, toY - (step % 2) * 0.5);
      await page.waitForTimeout(30);
    }
    await page.mouse.up();
    const seen = [];
    for (let look = 0; look < 120; look++) {
      const now = (await pets()).find((candidate) => candidate.id === pet.id);
      const line = `${now.activity}/${now.footing}`;
      if (seen.at(-1) !== line) seen.push(line);
      if (look === 15) await page.screenshot({ path: resolve(out, "drag-falling.png") });
      await page.waitForTimeout(50);
    }
    note({ after: seen });
    await page.screenshot({ path: resolve(out, "drag-landed.png") });
  }
}
if (mode === "circle") {
  const clear = await page.evaluate((list) => {
    const controls = 'a[href], button, input, select, textarea, summary, label, [role="button"], [role="link"], [role="checkbox"], [role="radio"], [role="tab"], [tabindex]:not([tabindex="-1"]), [data-quiz-grip], [data-quiz-drop], [data-layered-card]';
    return list.filter((pet) => {
      const cx = (pet.body.left + pet.body.right) / 2;
      const cy = (pet.body.top + pet.body.bottom) / 2;
      const radius = Math.max(pet.body.right - pet.body.left, pet.body.bottom - pet.body.top) / 2 + 14;
      for (let step = 0; step < 36; step++) {
        const x = cx + radius * Math.cos((step / 36) * 2 * Math.PI);
        const y = cy + radius * Math.sin((step / 36) * 2 * Math.PI);
        if (x < 1 || y < 1 || x > innerWidth - 1 || y > innerHeight - 1) return false;
        if (document.elementFromPoint(x, y)?.closest(controls) != null) return false;
      }
      return true;
    }).map((pet) => pet.id);
  }, before);
  note({ clear });
  const pet = pick(before, (candidate) => clear.includes(candidate.id));
  if (pet === undefined) note({ error: "no idle perched pet" });
  else {
    const cx = (pet.body.left + pet.body.right) / 2;
    const cy = (pet.body.top + pet.body.bottom) / 2;
    const radius = Math.max(pet.body.right - pet.body.left, pet.body.bottom - pet.body.top) / 2 + 14;
    note({ circling: pet.id, state: pet.state, cx, cy, radius });
    await page.mouse.move(cx + radius, cy);
    const points = Number(option("points", "24"));
    const laps = Number(option("laps", "2.5"));
    const pause = option("pause", "25");
    const stamps = [];
    for (let step = 0; step <= points * laps; step++) {
      const turn = (step / points) * 2 * Math.PI;
      await page.mouse.move(cx + radius * Math.cos(turn), cy + radius * Math.sin(turn));
      if (pause === "frame") await page.evaluate(() => new Promise((done) => requestAnimationFrame(() => done())));
      else await page.waitForTimeout(Number(pause));
      stamps.push(Date.now());
    }
    note({ lapMs: ((stamps.at(-1) - stamps[0]) / (stamps.length - 1)) * points });
    const seen = [];
    for (let look = 0; look < 80; look++) {
      const now = (await pets()).find((candidate) => candidate.id === pet.id);
      const line = `${now.activity}/${now.state}`;
      if (seen.at(-1) !== line) seen.push(line);
      await page.waitForTimeout(50);
    }
    note({ after: seen });
    await page.screenshot({ path: resolve(out, "circle-after.png") });
  }
}
if (mode === "control") {
  const pet = pick(before, (candidate) => species[candidate.id].gear.includes("parachute"));
  note({ cards: await page.evaluate(() => [...document.querySelectorAll("[data-layered-card]")].map((card) => { const box = card.getBoundingClientRect(); return [card.getAttribute("data-layered-card"), card.tagName, Math.round(box.x), Math.round(box.y), Math.round(box.width), Math.round(box.height), getComputedStyle(card).display]; })) });
  const target = await page.evaluate(() => {
    const box = document.querySelector('[data-layered-card="heating"] [data-slot="window-chrome-body-surface"]')?.getBoundingClientRect();
    return box === undefined ? null : { x: box.x, y: box.y, width: box.width, height: box.height };
  });
  if (pet === undefined || target === null) note({ error: "no parachute pet or no heating card" });
  else {
    const x = (pet.body.left + pet.body.right) / 2;
    const y = (pet.body.top + pet.body.bottom) / 2;
    const toX = target.x + target.width / 2;
    const toY = target.y + 30;
    note({ carrying: pet.id, to: [toX, toY], card: target });
    await page.mouse.move(x, y);
    await page.mouse.down();
    for (let step = 1; step <= 30; step++) {
      await page.mouse.move(x + ((toX - x) * step) / 30, y + ((toY - y) * step) / 30);
      await page.waitForTimeout(16);
    }
    for (let step = 0; step < 20; step++) {
      await page.mouse.move(toX, toY);
      await page.waitForTimeout(30);
    }
    await page.mouse.up();
    let clicked = null;
    for (let look = 0; look < 200 && clicked === null; look++) {
      const now = (await pets()).find((candidate) => candidate.id === pet.id);
      const middle = { x: (now.body.left + now.body.right) / 2, y: (now.body.top + now.body.bottom) / 2 };
      const inside = middle.x > target.x + 4 && middle.x < target.x + target.width - 4 && middle.y > target.y + 4 && middle.y < target.y + target.height - 4;
      const under = await page.evaluate(([px, py]) => {
        const element = document.elementFromPoint(px, py);
        return { tag: element?.tagName, card: element?.closest("[data-layered-card]")?.getAttribute("data-layered-card") ?? null, control: element?.closest("a[href], button, input, select, textarea, summary, label") != null };
      }, [middle.x, middle.y]);
      if (look % 10 === 0) note({ look, activity: now.activity, footing: now.footing, opacity: now.opacity, middle, inside, under });
      if (inside && (now.footing === "air" || now.footing === "chute") && under.card === "heating" && !under.control) {
        await page.mouse.click(middle.x, middle.y);
        clicked = { middle, activity: now.activity, footing: now.footing, opacity: now.opacity };
      } else await page.waitForTimeout(40);
    }
    note({ clicked });
    await page.waitForTimeout(600);
    note({ opened: await page.locator('[data-layered-pane="heating"]').getAttribute("data-opened"), hash: await page.evaluate(() => window.location.hash), after: (await pets()).find((candidate) => candidate.id === pet.id) });
    await page.screenshot({ path: resolve(out, "control-after.png") });
  }
}
writeFileSync(resolve(out, `${mode}.json`), JSON.stringify(log, null, 1));
await browser.close();
