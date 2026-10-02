/** 🐌️ Ticket tool of work package N: watches the pets of the real quiz site in slow motion. The page's clock is taken
 * over (Playwright's clock), time is advanced in steps of one simulation tick, and whenever something worth a look
 * begins — a pet turns round, leaves the ground (a hop, a glide, a fall), sets out on a walk or fades — the next frames
 * are captured as close-ups and laid out as one film strip per event.
 *
 * Usage (from the repository root, with the private stack up):
 *   node ".../wp_n_slowmo.mjs" --out <dir> --name <label> [--url http://127.0.0.1:6191] [--page <id>] [--mode calm|lively]
 *        [--seconds 60] [--want turn,air,walk,fade] [--only <species>] [--strips 6] [--width 1440] [--height 900] [--mobile 1] [--scheme light|dark]
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { chromium } from "playwright";

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const index = args.indexOf(`--${name}`);
  return index < 0 ? fallback : args[index + 1];
};
const url = option("url", "http://127.0.0.1:6191");
const out = resolve(option("out", "pet-slowmo"));
const name = option("name", "slow");
const target = option("page", "");
const mode = option("mode", "lively");
const seconds = Number(option("seconds", "60"));
const want = option("want", "turn,air,walk,fade").split(",");
const only = option("only", "");
const strips = Number(option("strips", "6"));
const width = Number(option("width", "1440"));
const height = Number(option("height", "900"));
const mobile = option("mobile", "") === "1";
const scheme = option("scheme", "light");
const CHOICES = ["off", "still", "calm", "lively"];
const TICK = 1000 / 64;
const PLANS = { turn: { frames: 12, every: 1, columns: 12 }, air: { frames: 16, every: 2, columns: 8 }, walk: { frames: 16, every: 4, columns: 8 }, fade: { frames: 10, every: 2, columns: 10 } };
mkdirSync(out, { recursive: true });

const look = () =>
  [...document.querySelectorAll(".pet-layer svg.pet")].map((pet) => {
    const place = /translate\(([-\d.]+)px, ([-\d.]+)px\) scale\(([-\d.]+), ([-\d.]+)\)/u.exec(pet.style.transform);
    const first = /matrix\(([-\d.e]+)/u.exec(pet.querySelector("g[transform]")?.getAttribute("transform") ?? "");
    return { id: pet.getAttribute("data-pet"), x: place === null ? 0 : Number(place[1]), y: place === null ? 0 : Number(place[2]), flip: place === null ? 1 : Math.sign(Number(place[3])), opacity: Number(pet.style.opacity || "1"), across: first === null ? 1 : Number(first[1]) };
  });

const browser = await chromium.launch();
const log = { name, events: [], errors: [] };
try {
  const context = await browser.newContext({ viewport: { width, height }, colorScheme: scheme, locale: "en-GB", isMobile: mobile, hasTouch: mobile, deviceScaleFactor: 2 });
  const page = await context.newPage();
  page.on("pageerror", (error) => log.errors.push(error.message));
  const sheet = await context.newPage();
  await sheet.setContent("<canvas></canvas>");
  const front = (card) => page.locator(`#quiz-main [data-card="${card}"]`).and(page.locator(":not([inert] *)"));
  const primary = (scope) => scope.locator('[data-overview-card-action="primary"]');
  await page.goto(url, { waitUntil: "domcontentloaded" });
  await front("introduction").waitFor({ timeout: 150_000 });
  await primary(front("introduction")).click();
  await front("identity").locator('input[type="radio"][value="anonymous"]').check();
  await primary(front("identity")).click();
  await page.locator("[data-layered-overview]").waitFor();
  if (mode !== "calm") {
    await page.evaluate(() => (window.location.hash = "prefs"));
    await page.locator('[data-layered-pane="prefs"][data-opened]').waitFor();
    await page.locator('[data-layered-pane="prefs"]').getByRole("group", { name: "Pets", exact: true }).getByRole("button").nth(CHOICES.indexOf(mode)).click();
    await page.locator(`.quiz-app[data-pets="${mode}"]`).waitFor();
    await page.keyboard.press("Escape");
  }
  await page.reload({ waitUntil: "domcontentloaded" });
  await page.locator("[data-layered-overview]").waitFor({ timeout: 60_000 });
  if (target !== "") {
    await page.evaluate((id) => (window.location.hash = id), target);
    await page.locator(`[data-layered-pane="${target}"][data-opened]`).waitFor();
  }
  await page.locator(".pet-layer svg.pet").first().waitFor({ state: "attached", timeout: 30_000 });
  await page.waitForTimeout(1500);
  await page.clock.install();
  await page.clock.pauseAt(Date.now() + 1000);

  const strip = async (frames, columns, path) => {
    const data = await sheet.evaluate(
      async ([images, perRow]) => {
        const loaded = await Promise.all(images.map((source) => new Promise((done) => {
          const image = new Image();
          image.onload = () => done(image);
          image.src = source;
        })));
        const canvas = document.querySelector("canvas");
        const [w, h] = [loaded[0].width, loaded[0].height];
        canvas.width = w * Math.min(perRow, loaded.length);
        canvas.height = h * Math.ceil(loaded.length / perRow);
        const pen = canvas.getContext("2d");
        loaded.forEach((image, index) => pen.drawImage(image, (index % perRow) * w, Math.floor(index / perRow) * h));
        return canvas.toDataURL("image/png");
      },
      [frames.map((frame) => `data:image/png;base64,${frame.toString("base64")}`), columns],
    );
    writeFileSync(path, Buffer.from(data.split(",")[1], "base64"));
  };

  const made = { turn: 0, air: 0, walk: 0, fade: 0 };
  let before = await page.evaluate(look);
  const steps = Math.round((seconds * 1000) / TICK);
  for (let step = 0; step < steps; step++) {
    await page.clock.runFor(TICK);
    const now = await page.evaluate(look);
    let found = null;
    for (const pet of now) {
      if (only !== "" && pet.id !== only) continue;
      const earlier = before.find((entry) => entry.id === pet.id);
      if (earlier === undefined) continue;
      const kind = Math.abs(pet.across) < 0.97 && Math.abs(earlier.across) >= 0.97 ? "turn" : pet.y !== earlier.y && Math.abs(pet.y - earlier.y) < 30 ? "air" : pet.x !== earlier.x && Math.abs(pet.x - earlier.x) < 3 ? "walk" : pet.opacity !== earlier.opacity && pet.opacity > 0.05 && pet.opacity < 0.95 ? "fade" : null;
      if (kind !== null && want.includes(kind) && made[kind] < strips) {
        found = { kind, pet };
        break;
      }
    }
    before = now;
    if (found === null) continue;
    const plan = PLANS[found.kind];
    const frames = [];
    const trace = [];
    const clip = { x: Math.max(0, Math.min(width - 150, found.pet.x - 75)), y: Math.max(0, Math.min(height - 110, found.pet.y - 85)), width: Math.min(150, width), height: 110 };
    for (let frame = 0; frame < plan.frames; frame++) {
      frames.push(await page.screenshot({ clip }));
      const seen = (await page.evaluate(look)).find((entry) => entry.id === found.pet.id);
      if (seen !== undefined) trace.push(`${seen.x.toFixed(1)},${seen.y.toFixed(1)} f${seen.flip} a${seen.across.toFixed(2)} o${seen.opacity.toFixed(2)}`);
      for (let skip = 0; skip < plan.every; skip++) await page.clock.runFor(TICK);
      step += plan.every;
    }
    made[found.kind] += 1;
    const file = `${name}-${found.kind}-${made[found.kind]}-${found.pet.id}.png`;
    await strip(frames, plan.columns, resolve(out, file));
    log.events.push({ kind: found.kind, pet: found.pet.id, atSeconds: Math.round((step * TICK) / 100) / 10, file, trace });
    before = await page.evaluate(look);
    if (want.every((kind) => made[kind] >= strips)) break;
  }
  await context.close();
} catch (error) {
  log.errors.push(`slowmo: ${error instanceof Error ? error.message : String(error)}`);
} finally {
  await browser.close();
}
writeFileSync(resolve(out, `${name}.json`), `${JSON.stringify(log, null, 1)}\n`);
process.stdout.write(`${JSON.stringify(log, null, 1)}\n`);
