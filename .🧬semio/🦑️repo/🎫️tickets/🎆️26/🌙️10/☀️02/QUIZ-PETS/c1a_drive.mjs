/** 🖐️ Ticket tool of work package C1a (second round): drives the learner's hand on the harness page in headless Chromium — hovers, clicks, holds and drags a pet, clicks a button a pet covers, selects words beside the pets, calls a drag off with Escape and plays a deed through the layer's handle — and prints what reached the stage and the page, with screenshots.
 *
 * Usage (from the repository root, harness running on 6253):
 *   node ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/c1a_drive.mjs" [--base http://127.0.0.1:6253/] [--out <dir>] [--query mode=calm&seed=5]
 */
import { mkdirSync } from "node:fs";
import { resolve } from "node:path";
import { chromium } from "playwright";

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const index = args.indexOf(`--${name}`);
  return index < 0 ? fallback : args[index + 1];
};
const address = option("base", "http://127.0.0.1:6253/");
const out = resolve(option("out", ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/🗑️generated/c1a/browser"));
const query = option("query", "mode=calm&seed=5");
mkdirSync(out, { recursive: true });

const failed = [];
const report = {};
const expect = (name, holds, detail) => {
  if (!holds) failed.push(`${name}: ${JSON.stringify(detail)}`);
};

const browser = await chromium.launch();
const context = await browser.newContext({ viewport: { width: 1280, height: 800 } });
const page = await context.newPage();
const noise = [];
page.on("console", (message) => {
  if (!/\[vite\]|React DevTools/.test(message.text())) noise.push(`${message.type()}: ${message.text()}`);
});
page.on("pageerror", (error) => noise.push(`pageerror: ${error}`));
await page.goto(`${address}?${query}`, { waitUntil: "load" });
await page.waitForTimeout(3000);

const mark = () => page.evaluate(() => ({ events: window.petRecord.length, page: window.pageRecord.length }));
const since = (from) => page.evaluate((from) => ({ events: window.petRecord.slice(from.events).filter((event) => event.kind !== "surveyed" && event.kind !== "stirred"), page: window.pageRecord.slice(from.page) }), from);
const cursor = () => page.evaluate(() => document.documentElement.getAttribute("data-pet-cursor"));
const actors = () => page.evaluate(() => (window.petFrame?.actors ?? []).map((actor) => ({ species: actor.species, x: actor.x, y: actor.y, footing: actor.footing, opacity: actor.opacity, body: actor.body })));
const middle = (actor) => ({ x: actor.body.x + actor.body.width / 2, y: actor.body.y + actor.body.height / 2 });
const grounded = async () => (await actors()).filter((actor) => actor.footing === "perch" && actor.opacity > 0.5 && actor.body !== undefined);
const shoot = (name) => page.screenshot({ path: resolve(out, `${name}.png`) });

try {
  const cast = await grounded();
  report.cast = cast;
  expect("pets stand", cast.length >= 2, cast);
  const [first, second] = cast;
  const point = middle(first);

  {
    await page.mouse.move(point.x, point.y, { steps: 4 });
    await page.waitForTimeout(150);
    report.hover = await cursor();
    expect("hover shows grab", report.hover === "grab", report.hover);
    await shoot("1-hover");
    const from = await mark();
    await page.mouse.down();
    await page.waitForTimeout(60);
    await page.mouse.up();
    await page.waitForTimeout(300);
    const after = await since(from);
    report.click = after;
    expect("click: pressed then released at the same point", JSON.stringify(after.events.map((event) => event.kind)) === JSON.stringify(["pressed", "released"]), after.events);
    expect("click: the page heard nothing", after.page.length === 0, after.page);
    await shoot("2-clicked");
  }

  {
    await page.waitForTimeout(800);
    const [now] = (await grounded()).filter((actor) => actor.species === first.species);
    const start = now === undefined ? point : middle(now);
    await page.mouse.move(start.x, start.y);
    const from = await mark();
    await page.mouse.down();
    await page.mouse.move(start.x + 60, start.y - 40, { steps: 8 });
    await page.waitForTimeout(200);
    await page.mouse.move(start.x + 160, start.y - 120, { steps: 12 });
    await page.waitForTimeout(200);
    report.dragCursor = await cursor();
    report.dragHeld = await page.evaluate(() => window.petFrame?.held ?? null);
    await shoot("3-dragging");
    await page.mouse.up();
    await page.waitForTimeout(600);
    const after = await since(from);
    const kinds = after.events.map((event) => event.kind);
    report.drag = { kinds: kinds.join(" "), dragged: kinds.filter((kind) => kind === "dragged").length, last: after.events.at(-1), page: after.page };
    expect("drag: pressed, dragged, released", kinds[0] === "pressed" && kinds.at(-1) === "released" && kinds.filter((kind) => kind === "dragged").length >= 3, kinds);
    expect("drag: released where the pointer let go", Math.abs(after.events.at(-1)?.x - (start.x + 160)) < 0.01 && Math.abs(after.events.at(-1)?.y - (start.y - 120)) < 0.01, after.events.at(-1));
    expect("drag: the page heard nothing", after.page.length === 0, after.page);
    expect("drag: cursor grabbing while the stage holds the pet", report.dragHeld === null || report.dragCursor === "grabbing", { held: report.dragHeld, cursor: report.dragCursor });
    await shoot("4-released");
  }

  {
    await page.waitForTimeout(1500);
    const pets = await grounded();
    const target = pets.find((actor) => actor.species === second.species) ?? pets[0];
    const spot = middle(target);
    await page.evaluate((spot) => {
      const button = document.getElementById("covered");
      button.hidden = false;
      button.style.left = `${spot.x - 30}px`;
      button.style.top = `${spot.y - 15 - button.parentElement.getBoundingClientRect().top}px`;
    }, spot);
    await page.waitForTimeout(100);
    const from = await mark();
    report.coveredHit = await page.evaluate((spot) => document.elementFromPoint(spot.x, spot.y)?.id ?? null, spot);
    await page.mouse.click(spot.x, spot.y);
    await page.waitForTimeout(200);
    const after = await since(from);
    report.covered = after;
    expect("covered button: the control has its press and its click", after.page.includes("click:covered") && after.page.includes("pointerdown:covered"), after.page);
    expect("covered button: no press for the pets", !after.events.some((event) => event.kind === "pressed"), after.events);
    await shoot("5-covered-button");
    await page.evaluate(() => {
      document.getElementById("covered").hidden = true;
    });
  }

  {
    const box = await page.locator("#words").boundingBox();
    const from = await mark();
    await page.mouse.move(box.x + 2, box.y + box.height / 2);
    await page.mouse.down();
    await page.mouse.move(box.x + box.width * 0.6, box.y + box.height / 2, { steps: 10 });
    await page.mouse.up();
    await page.waitForTimeout(200);
    report.selected = await page.evaluate(() => document.getSelection()?.toString() ?? "");
    const after = await since(from);
    expect("selection: words beside the pets select", report.selected.length > 20, report.selected);
    expect("selection: no press for the pets", !after.events.some((event) => event.kind === "pressed"), after.events);
    await shoot("6-selected");
    await page.mouse.click(5, 790);
    await page.waitForTimeout(200);
  }

  {
    await page.waitForTimeout(1500);
    const pets = await grounded();
    const spot = middle(pets[0]);
    await page.mouse.move(spot.x, spot.y);
    const from = await mark();
    await page.mouse.down();
    await page.mouse.move(spot.x - 120, spot.y - 80, { steps: 12 });
    await page.waitForTimeout(150);
    await page.keyboard.press("Escape");
    await page.waitForTimeout(100);
    report.escapeCursor = await cursor();
    await page.mouse.move(spot.x - 140, spot.y - 90, { steps: 3 });
    await page.mouse.up();
    await page.waitForTimeout(400);
    const after = await since(from);
    const kinds = after.events.map((event) => event.kind);
    report.escape = { kinds: kinds.join(" "), page: after.page };
    expect("escape: pressed, dragged, cancelled and nothing after", kinds[0] === "pressed" && kinds.at(-1) === "cancelled" && !kinds.includes("released"), kinds);
    expect("escape: the page heard neither the Escape nor a click", after.page.length === 0, after.page);
    await shoot("7-escaped");
  }

  {
    const from = await mark();
    const species = (await actors())[0]?.species;
    await page.evaluate((species) => window.petHand.current.play(species, "hello"), species);
    await page.waitForTimeout(200);
    const after = await since(from);
    report.played = after.events;
    expect("played: the deed reaches the stage", after.events.some((event) => event.kind === "played" && event.species === species && event.deed === "hello"), after.events);
  }

  {
    const quiet = await context.newPage();
    quiet.on("console", (message) => {
      if (!/\[vite\]|React DevTools/.test(message.text())) noise.push(`${message.type()}: ${message.text()}`);
    });
    quiet.on("pageerror", (error) => noise.push(`pageerror: ${error}`));
    await quiet.goto(`${address}?mode=calm&quiet=1&seed=5&keepouts=loose`, { waitUntil: "load" });
    await quiet.waitForTimeout(3000);
    const resting = await quiet.evaluate(() => window.petFrame.actors.map((actor) => ({ species: actor.species, body: actor.body })));
    const spot = { x: resting[0].body.x + resting[0].body.width / 2, y: resting[0].body.y + resting[0].body.height / 2 };
    await quiet.evaluate((spot) => {
      const button = document.getElementById("covered");
      button.hidden = false;
      button.style.left = `${spot.x - 30}px`;
      button.style.top = `${spot.y - 15 - button.parentElement.getBoundingClientRect().top}px`;
    }, spot);
    await quiet.waitForTimeout(300);
    const before = await quiet.evaluate(() => ({ events: window.petRecord.length, page: window.pageRecord.length }));
    const inside = await quiet.evaluate((spot) => window.petFrame.actors.some((actor) => spot.x >= actor.body.x && spot.x <= actor.body.x + actor.body.width && spot.y >= actor.body.y && spot.y <= actor.body.y + actor.body.height), spot);
    const under = await quiet.evaluate((spot) => document.elementFromPoint(spot.x, spot.y)?.id ?? null, spot);
    await quiet.mouse.click(spot.x, spot.y);
    await quiet.waitForTimeout(200);
    await quiet.screenshot({ path: resolve(out, "5b-button-under-a-resting-pet.png") });
    const onButton = await quiet.evaluate((from) => ({ events: window.petRecord.slice(from.events).filter((event) => !["surveyed", "stirred"].includes(event.kind)), page: window.pageRecord.slice(from.page) }), before);
    await quiet.evaluate(() => {
      document.getElementById("covered").hidden = true;
    });
    await quiet.waitForTimeout(300);
    const mid = await quiet.evaluate(() => ({ events: window.petRecord.length, page: window.pageRecord.length }));
    const later = await quiet.evaluate(() => ({ actors: window.petFrame.actors.map((actor) => ({ species: actor.species, body: actor.body })), focused: document.activeElement?.id ?? document.activeElement?.nodeName, selection: document.getSelection()?.toString() ?? "" }));
    report.coveredQuietAfter = later;
    const moved = later.actors.find((actor) => actor.species === resting[0].species).body;
    await quiet.mouse.click(moved.x + moved.width / 2, moved.y + moved.height / 2);
    await quiet.waitForTimeout(200);
    const onPet = await quiet.evaluate((from) => ({ events: window.petRecord.slice(from.events).filter((event) => !["surveyed", "stirred"].includes(event.kind)), page: window.pageRecord.slice(from.page) }), mid);
    report.coveredQuiet = { spot, inside, under, onButton, onPet };
    expect("button under a resting pet: the point lies in the pet's body and the button is what the browser hits", inside && under === "covered", { inside, under });
    expect("button under a resting pet: the button has its press and click, the pets nothing", onButton.page.includes("click:covered") && onButton.events.length === 0, onButton);
    expect("the same pet without the button (it stepped aside from the focused button): the pet's press",onPet.events.map((event) => event.kind).join(" ") === "pressed released" && onPet.page.length === 0, onPet);
    await quiet.close();
  }

  {
    const phone = await browser.newContext({ viewport: { width: 1280, height: 800 }, hasTouch: true, isMobile: true });
    const touch = await phone.newPage();
    touch.on("console", (message) => {
      if (!/\[vite\]|React DevTools/.test(message.text())) noise.push(`${message.type()}: ${message.text()}`);
    });
    touch.on("pageerror", (error) => noise.push(`pageerror: ${error}`));
    await touch.goto(`${address}?mode=calm&quiet=1&seed=5`, { waitUntil: "load" });
    await touch.waitForTimeout(3000);
    const pads = await touch.evaluate(() => [...document.querySelectorAll(".pet-pad")].map((pad) => { const box = pad.getBoundingClientRect(); return { x: box.x + box.width / 2, y: box.y + box.height / 2, width: box.width, height: box.height, events: getComputedStyle(pad).pointerEvents, action: getComputedStyle(pad).touchAction }; }));
    const coarse = await touch.evaluate(() => matchMedia("(pointer: coarse)").matches);
    const grounded = await touch.evaluate(() => window.petFrame.actors.filter((actor) => actor.footing === "perch").length);
    const hits = await touch.evaluate((pads) => pads.map((pad) => document.elementFromPoint(pad.x, pad.y)?.className ?? null), pads);
    const from = await touch.evaluate(() => ({ events: window.petRecord.length, page: window.pageRecord.length }));
    await touch.touchscreen.tap(pads[0].x, pads[0].y);
    await touch.waitForTimeout(300);
    const cdp = await phone.newCDPSession(touch);
    const finger = (type, x, y) => cdp.send("Input.dispatchTouchEvent", { type, touchPoints: type === "touchEnd" ? [] : [{ x, y, id: 1 }] });
    await finger("touchStart", pads[0].x, pads[0].y);
    for (let step = 1; step <= 10; step++) {
      await finger("touchMove", pads[0].x + step * 12, pads[0].y - step * 8);
      await touch.waitForTimeout(20);
    }
    await finger("touchEnd", 0, 0);
    await touch.waitForTimeout(300);
    const after = await touch.evaluate((from) => ({ events: window.petRecord.slice(from.events).filter((event) => !["surveyed", "stirred"].includes(event.kind)), page: window.pageRecord.slice(from.page), scrolled: document.scrollingElement.scrollTop }), from);
    await touch.screenshot({ path: resolve(out, "8-touch-pads.png") });
    const kinds = after.events.map((event) => event.kind).filter((kind) => kind !== "unpointed");
    report.touch ={ coarse, grounded, pads, hits, kinds: kinds.join(" "), pointers: [...new Set(after.events.filter((event) => event.kind === "pressed").map((event) => event.pointer))], page: after.page };
    expect("touch: one pad per grounded pet, taking touches without panning", coarse && pads.length === grounded && pads.every((pad) => pad.events === "auto" && pad.action === "none") && hits.every((hit) => hit === "pet-pad"), report.touch);
    expect("touch: a tap and a drag on a pad are presses of a finger, the drag is followed to its end", kinds.join(" ").startsWith("pressed released pressed dragged") && kinds.at(-1) === "released" && report.touch.pointers.join() === "touch", report.touch);
    expect("touch: the page heard nothing", after.page.length === 0, after.page);
    await phone.close();
  }

  report.walls = await page.evaluate(() => window.petRecord.filter((event) => event.kind === "surveyed").at(-1)?.walls ?? null);
  expect("survey: walls of every card", Array.isArray(report.walls) && report.walls.length >= 6, report.walls);
  report.console = [...(await page.evaluate(() => window.petConsole)), ...noise];
  expect("console stays empty", report.console.length === 0, report.console);
} finally {
  await browser.close();
}
console.log(JSON.stringify({ report, failed }, null, 1));
process.exitCode = failed.length === 0 ? 0 : 1;
