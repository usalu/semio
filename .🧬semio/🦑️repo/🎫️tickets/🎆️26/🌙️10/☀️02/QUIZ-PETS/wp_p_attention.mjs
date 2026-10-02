/** 👀️ Ticket tool of work package P: looks at how the pets of the real quiz attend to the pointer. It enters the site as
 * an anonymous learner, waits for the pets on the overview, picks one that stands whole and idle, and takes close-ups of
 * it (three device pixels per pixel) while the pointer goes far to its left, far to its right, above it and comes to
 * rest beside it — with what the depiction says at every step: activity, facing, the pupils and the matrix of the bone
 * that carries the first eye.
 *
 * Usage (from the repository root, with a private stack up):
 *   node ".../wp_p_attention.mjs" --out <dir> [--url http://127.0.0.1:6193] [--width 1440] [--height 900] [--mobile 1] [--mode calm|lively] [--pet <id>]
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { chromium } from "playwright";

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const index = args.indexOf(`--${name}`);
  return index < 0 ? fallback : args[index + 1];
};
const url = option("url", "http://127.0.0.1:6193");
const out = resolve(option("out", "pet-attention"));
const width = Number(option("width", "1440"));
const height = Number(option("height", "900"));
const mobile = option("mobile", "") === "1";
const mode = option("mode", "calm");
const wanted = option("pet", "");
mkdirSync(out, { recursive: true });

const browser = await chromium.launch();
const log = [];
try {
  const context = await browser.newContext({ viewport: { width, height }, deviceScaleFactor: 3, isMobile: mobile, hasTouch: false, locale: "en-GB" });
  const page = await context.newPage();
  page.on("console", (message) => {
    if (message.type() === "error") log.push({ console: message.text() });
  });
  await page.goto(url, { waitUntil: "domcontentloaded" });
  const front = (card) => page.locator(`#quiz-main [data-card="${card}"]`).and(page.locator(":not([inert] *)"));
  await front("introduction").waitFor({ timeout: 150_000 });
  await front("introduction").locator('[data-overview-card-action="primary"]').click();
  await front("identity").locator('input[type="radio"][value="anonymous"]').check();
  await front("identity").locator('[data-overview-card-action="primary"]').click();
  await page.locator("[data-layered-overview]").waitFor();
  if (mode !== "calm") {
    await page.evaluate(() => (window.location.hash = "prefs"));
    await page.locator('[data-layered-pane="prefs"][data-opened]').waitFor();
    await page.locator('[data-layered-pane="prefs"]').getByRole("group", { name: "Pets", exact: true }).getByRole("button").nth(["off", "still", "calm", "lively"].indexOf(mode)).click();
    await page.keyboard.press("Escape");
  }
  await page.locator(".pet-layer svg.pet").first().waitFor({ state: "attached", timeout: 20_000 });
  await page.mouse.move(2, 2);
  await page.waitForTimeout(4500);

  const read = () =>
    page.evaluate(() =>
      [...document.querySelectorAll(".pet-layer svg.pet")].map((pet) => {
        const place = /translate\(([-\d.]+)px, ([-\d.]+)px\) scale\(([-\d.]+), ([-\d.]+)\)/u.exec(pet.style.transform);
        const pupils = [...pet.querySelectorAll(".pet-pupil")].map((pupil) => [Number(pupil.getAttribute("cx")), Number(pupil.getAttribute("cy"))]);
        const socket = pet.querySelector(".pet-pupil")?.parentElement?.parentElement;
        const box = pet.getBoundingClientRect();
        return { id: pet.getAttribute("data-pet"), activity: pet.getAttribute("data-pet-activity"), x: Number(place?.[1]), y: Number(place?.[2]), flip: Number(place?.[3]), opacity: Number(pet.style.opacity || "1"), pupils, head: socket?.getAttribute("transform") ?? null, box: { left: box.left, top: box.top, right: box.right, bottom: box.bottom } };
      }),
    );

  const pets = await read();
  const pet = pets.find((candidate) => (wanted === "" ? candidate.activity === "idle" && candidate.opacity === 1 && candidate.x > 160 && candidate.x < width - 160 && candidate.y > 140 : candidate.id === wanted)) ?? pets[0];
  if (pet === undefined) throw new Error("no pet on stage");
  const clip = { x: Math.max(0, pet.x - 110), y: Math.max(0, pet.y - 110), width: 220, height: 150 };
  const shot = async (label) => {
    const now = (await read()).find((candidate) => candidate.id === pet.id);
    log.push({ label, ...now });
    await page.screenshot({ path: resolve(out, `${label}.png`), clip });
  };
  const hold = async (label, x, y, steps) => {
    await page.mouse.move(x, y, { steps: 6 });
    for (const [index, wait] of steps.entries()) {
      await page.waitForTimeout(wait);
      await shot(`${label}-${index}`);
    }
  };
  const eye = pet.y - 30;
  await shot("00-before");
  await hold("01-far-left", Math.max(4, pet.x - 400), eye, [60, 60, 60, 60, 400, 900]);
  await hold("02-far-right", Math.min(width - 4, pet.x + 400), eye, [60, 60, 60, 60, 400, 900]);
  await hold("03-above", pet.x + 20, Math.max(40, pet.y - 320), [500, 900]);
  await hold("04-below-left", Math.max(4, pet.x - 260), Math.min(height - 40, pet.y + 200), [500, 900]);
  await hold("05-beside", pet.x + 46, eye, [200, 250, 250, 250, 400, 900, 1500]);
  await hold("06-on-it", pet.x, pet.y - 20, [500, 900]);
  await page.mouse.move(2, 2);
  await page.waitForTimeout(600);
  await shot("07-after");
  await page.screenshot({ path: resolve(out, "page.png") });
  writeFileSync(resolve(out, "attention.json"), `${JSON.stringify({ pet: pet.id, clip, log }, null, 1)}\n`);
  process.stdout.write(`${pet.id}: ${log.filter((entry) => entry.label !== undefined).map((entry) => `${entry.label} ${entry.activity} flip ${entry.flip} pupils ${JSON.stringify(entry.pupils?.[0])} head ${entry.head}`).join("\n")}\n`);
} finally {
  await browser.close();
}
