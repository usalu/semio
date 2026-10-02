/** 🏟️ Ticket tool (work package O2): watches the sandbox of a running stories gallery — the real `PetLayer` on mock cards — and reports, per species, how far its painted drawing ever reached below the edge it stands on and beyond the box of its species, with close-ups of every pet a few times during the run.
 *
 * Usage (from the repository root, with the gallery running on a private port):
 *   node ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/wp_o2_gallery.mjs" --url http://127.0.0.1:6347/ --out <dir> --name <label> [--scene envelope] [--mode lively] [--capacity 5] [--seconds 60] [--dark 1]
 */
import { mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { chromium } from "playwright";

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const index = args.indexOf(`--${name}`);
  return index < 0 ? fallback : args[index + 1];
};
const url = option("url", "http://127.0.0.1:6347/");
const out = resolve(option("out", "."));
const name = option("name", "gallery");
const scene = option("scene", "envelope");
const mode = option("mode", "lively");
const capacity = option("capacity", "5");
const seconds = Number(option("seconds", "60"));
const menagerie = resolve(import.meta.dirname, "../../../../../../../🎓️teaching/🏛️architecture/🐾️pets");
const sizes = {};
for (const entry of readdirSync(menagerie, { withFileTypes: true })) {
  if (!entry.isDirectory()) continue;
  const species = JSON.parse(readFileSync(resolve(menagerie, entry.name, "🔣️.json"), "utf8"));
  sizes[species.id] = [species.size.width, species.size.height, species.locomotion.hover ?? 0];
}
mkdirSync(out, { recursive: true });

const look = (boxes) =>
  [...document.querySelectorAll(".pet-layer svg.pet")].map((pet) => {
    const place = /translate\(([-\d.]+)px, ([-\d.]+)px\) scale\(([-\d.]+), ([-\d.]+)\)/u.exec(pet.style.transform);
    const id = pet.getAttribute("data-pet");
    const [width, height, hover] = boxes[id] ?? [0, 0, 0];
    const x = place === null ? 0 : Number(place[1]);
    const y = place === null ? 0 : Number(place[2]);
    const size = place === null ? 1 : Number(place[4]);
    const origin = pet.parentElement.getBoundingClientRect();
    let below = -Infinity;
    let beside = -Infinity;
    for (const shape of pet.querySelectorAll("path, ellipse, rect, line, circle")) {
      const box = shape.getBoundingClientRect();
      const style = getComputedStyle(shape);
      if ((box.width === 0 && box.height === 0) || (box.width * box.height === 0 && style.stroke === "none")) continue;
      const stroke = style.stroke === "none" ? 0 : ((parseFloat(style.strokeWidth) || 0) / 2) * size;
      below = Math.max(below, (box.bottom + stroke - origin.top - y) / size - hover);
      beside = Math.max(beside, ((box.right + stroke - origin.left - x) / size) - width / 2, -width / 2 - (box.left - stroke - origin.left - x) / size);
    }
    return { id, x: origin.left + x, y: origin.top + y, size, opacity: Number(pet.style.opacity || "1"), below, beside, width, height, hover };
  });

const worst = {};
const errors = [];
const browser = await chromium.launch();
try {
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: 2, locale: "en-GB" });
  page.on("pageerror", (error) => errors.push(String(error)));
  page.on("console", (message) => {
    if (message.type() === "error") errors.push(message.text());
  });
  await page.goto(url, { waitUntil: "networkidle" });
  await page.getByRole("button", { name: "Sandbox", exact: true }).click();
  await page.waitForSelector(".room");
  if (option("dark", "") === "1") await page.getByLabel("Dark page").check();
  await page.getByLabel("Scene").selectOption(scene);
  await page.getByLabel("Capacity").fill(capacity);
  await page.getByLabel("Mode").selectOption(mode);
  const steps = Math.round(seconds * 5);
  for (let step = 0; step < steps; step++) {
    await page.waitForTimeout(200);
    const pets = await page.evaluate(look, sizes);
    for (const pet of pets) {
      if (pet.opacity < 1) continue;
      const entry = (worst[pet.id] ??= { below: -Infinity, beside: -Infinity, samples: 0 });
      entry.below = Math.max(entry.below, pet.below);
      entry.beside = Math.max(entry.beside, pet.beside);
      entry.samples++;
    }
    if (step % Math.round(steps / 6) === Math.round(steps / 12)) {
      const shot = String(Math.floor(step / Math.round(steps / 6)));
      await page.screenshot({ path: resolve(out, `${name}-${shot}.png`), clip: { x: 0, y: 0, width: 1440, height: 900 }, scale: "css" });
      let index = 0;
      for (const pet of pets) {
        const reach = 34 * pet.size;
        await page.screenshot({ path: resolve(out, `${name}-${shot}-${pet.id}.png`), clip: { x: Math.max(0, pet.x - pet.width / 2 - reach), y: Math.max(0, pet.y - pet.height * pet.size - reach), width: pet.width * pet.size + 2 * reach, height: pet.height * pet.size + reach + 24 } });
        index++;
      }
    }
  }
} catch (error) {
  errors.push(String(error));
} finally {
  await browser.close();
}
const lines = Object.entries(worst).map(([id, entry]) => `${id.padEnd(9)} ${String(entry.samples).padStart(4)} samples · lowest point ${entry.below.toFixed(2)} px below the edge it stands on (floaters: below their hover line) · farthest ${entry.beside.toFixed(2)} px beyond its box sideways`);
writeFileSync(resolve(out, `${name}.txt`), `${lines.join("\n")}\nerrors: ${JSON.stringify(errors)}\n`);
console.log(`${lines.join("\n")}\nerrors: ${JSON.stringify(errors)}`);
