/** 🏟️ Ticket tool of work package N: watches the sandbox of a running stories gallery for a while — the scene, the
 * liveliness and the capacity it is told — and writes a screenshot every few seconds plus where every pet stood, whether
 * two of them ever stood in each other and whether a pet ever covered a control or a text block.
 *
 * Usage (from the repository root, with the gallery running):
 *   node ".../wp_n_gallery.mjs" --out <dir> --name <label> [--url http://127.0.0.1:6074/] [--scene home] [--mode Calm|Lively|Still] [--seconds 60] [--every 3000] [--capacity 6] [--dark 1] [--width 1440] [--height 900]
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { chromium } from "playwright";

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const index = args.indexOf(`--${name}`);
  return index < 0 ? fallback : args[index + 1];
};
const url = option("url", "http://127.0.0.1:6074/");
const out = resolve(option("out", "."));
const name = option("name", "gallery");
const scene = option("scene", "home");
const mode = option("mode", "Calm");
const seconds = Number(option("seconds", "60"));
const every = Number(option("every", "3000"));
const capacity = option("capacity", "6");
const width = Number(option("width", "1440"));
const height = Number(option("height", "900"));
mkdirSync(out, { recursive: true });

const look = () => {
  const boxOf = (rect) => ({ left: rect.left, top: rect.top, right: rect.right, bottom: rect.bottom });
  const overlap = (a, b) => Math.max(0, Math.min(a.right, b.right) - Math.max(a.left, b.left)) * Math.max(0, Math.min(a.bottom, b.bottom) - Math.max(a.top, b.top));
  const pets = [...document.querySelectorAll(".pet-layer svg.pet")].map((pet) => {
    let drawn = null;
    for (const child of pet.children) {
      const rect = child.getBoundingClientRect();
      if (rect.width === 0 && rect.height === 0) continue;
      drawn = drawn === null ? boxOf(rect) : { left: Math.min(drawn.left, rect.left), top: Math.min(drawn.top, rect.top), right: Math.max(drawn.right, rect.right), bottom: Math.max(drawn.bottom, rect.bottom) };
    }
    const place = /translate\(([-\d.]+)px, ([-\d.]+)px\)/u.exec(pet.style.transform);
    return { id: pet.getAttribute("data-pet"), x: place === null ? 0 : Math.round(Number(place[1])), y: place === null ? 0 : Math.round(Number(place[2])), opacity: Number(pet.style.opacity || "1"), drawn };
  });
  const stacked = [];
  for (let first = 0; first < pets.length; first++) {
    for (let second = first + 1; second < pets.length; second++) {
      const [one, two] = [pets[first], pets[second]];
      if (one.drawn === null || two.drawn === null || one.opacity < 0.2 || two.opacity < 0.2) continue;
      const least = Math.min((one.drawn.right - one.drawn.left) * (one.drawn.bottom - one.drawn.top), (two.drawn.right - two.drawn.left) * (two.drawn.bottom - two.drawn.top));
      if (least > 0 && overlap(one.drawn, two.drawn) / least > 0.15) stacked.push(`${one.id}+${two.id}`);
    }
  }
  const covered = [];
  for (const element of document.querySelectorAll(".room button, .room a[href], .room p, .room h2, .room h3, .room li, .room input, .room [data-pet-keepout]")) {
    const box = boxOf(element.getBoundingClientRect());
    for (const pet of pets) if (pet.drawn !== null && pet.opacity >= 0.2 && overlap(pet.drawn, box) > 6) covered.push(`${pet.id}>${element.tagName.toLowerCase()}`);
  }
  return { pets: pets.map((pet) => `${pet.id}@${pet.x},${pet.y}${pet.opacity < 1 ? `~${pet.opacity.toFixed(2)}` : ""}`), stacked, covered };
};

const facts = { name, scene, mode, errors: [], samples: [] };
const browser = await chromium.launch();
try {
  const page = await browser.newPage({ viewport: { width, height }, locale: "en-GB" });
  page.on("pageerror", (error) => facts.errors.push(String(error)));
  page.on("console", (message) => {
    if (message.type() === "error") facts.errors.push(message.text());
  });
  await page.goto(url, { waitUntil: "networkidle" });
  await page.getByRole("button", { name: "Sandbox", exact: true }).click();
  await page.waitForSelector(".room");
  if (option("dark", "") === "1") await page.getByLabel("Dark page").check();
  await page.getByLabel("Scene").selectOption(scene);
  await page.getByLabel("Capacity").fill(capacity);
  await page.getByLabel("Mode").selectOption(mode.toLowerCase());
  facts.controls = await page.evaluate(() => [...document.querySelectorAll("label")].map((label) => (label.textContent ?? "").trim()).slice(0, 40));
  const count = Math.max(1, Math.round((seconds * 1000) / every));
  for (let index = 0; index < count; index++) {
    await page.waitForTimeout(every);
    const sample = await page.evaluate(look);
    facts.samples.push({ at: ((index + 1) * every) / 1000, ...sample });
    if (index % 4 === 3 || index === count - 1) await page.screenshot({ path: resolve(out, `${name}-${String(index + 1).padStart(2, "0")}.png`) });
  }
} catch (error) {
  facts.errors.push(String(error));
} finally {
  await browser.close();
}
writeFileSync(resolve(out, `${name}.json`), `${JSON.stringify(facts, null, 1)}\n`);
process.stdout.write(`${JSON.stringify({ errors: facts.errors, controls: facts.controls, stacked: [...new Set(facts.samples.flatMap((sample) => sample.stacked))], covered: [...new Set(facts.samples.flatMap((sample) => sample.covered))] })}\n${facts.samples.map((sample) => `${String(sample.at).padStart(4)} ${sample.pets.join("  ")} ${sample.stacked.join(" ")}`).join("\n")}\n`);
