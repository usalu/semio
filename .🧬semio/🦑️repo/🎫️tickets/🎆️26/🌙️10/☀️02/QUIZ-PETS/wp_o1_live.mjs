/** 🎥️ Ticket tool (work package O1): films the sandbox of a running stories gallery — the real `PetLayer` — and writes, per species, a contact sheet of what it did: one crop around the pet per sample, in order, with the place it stood.
 *
 * Usage (from the repository root, with the gallery running on a private port):
 *   node "<ticket>/wp_o1_live.mjs" --url http://127.0.0.1:6237/ --out <directory> --name <label> [--scene physics] [--mode lively] [--seconds 40] [--every 250] [--scale 2] [--capacity 7] [--seed 3] [--dark 1] [--ids sunny,kettly] [--poke 1]
 */
import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { chromium } from "playwright";

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const index = args.indexOf(`--${name}`);
  return index < 0 ? fallback : args[index + 1];
};
const url = option("url", "http://127.0.0.1:6237/");
const out = resolve(option("out", "."));
const name = option("name", "live");
const scene = option("scene", "home");
const mode = option("mode", "lively");
const seconds = Number(option("seconds", "40"));
const every = Number(option("every", "250"));
const scale = option("scale", "2");
const capacity = option("capacity", "7");
const seed = option("seed", "3");
const dark = option("dark", "") === "1";
const poke = option("poke", "") === "1";
const only = option("ids", "").split(",").filter((id) => id.length > 0);
const frames = join(out, `${name}-frames`);
mkdirSync(frames, { recursive: true });

const slide = async (page, label, value) => {
  const input = page.locator("label.control", { hasText: label }).locator("input").first();
  await input.evaluate((element, wanted) => {
    const setter = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, "value").set;
    setter.call(element, wanted);
    element.dispatchEvent(new Event("input", { bubbles: true }));
    element.dispatchEvent(new Event("change", { bubbles: true }));
  }, value);
};

const errors = [];
const samples = [];
const browser = await chromium.launch();
try {
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, locale: "en-GB", deviceScaleFactor: 1 });
  page.on("pageerror", (error) => errors.push(String(error)));
  page.on("console", (message) => {
    if (message.type() === "error") errors.push(message.text());
  });
  await page.goto(url, { waitUntil: "networkidle" });
  await page.getByRole("button", { name: "Sandbox", exact: true }).click();
  await page.waitForSelector(".room");
  if (dark) await page.getByLabel("Dark page").check();
  await page.getByLabel("Scene").selectOption(scene);
  await slide(page, "Capacity", capacity);
  await slide(page, "Scale", scale);
  await slide(page, "Seed", seed);
  await page.getByLabel("Mode").selectOption(mode);
  const count = Math.max(1, Math.round((seconds * 1000) / every));
  const started = Date.now();
  for (let index = 0; index < count; index++) {
    const wait = started + (index + 1) * every - Date.now();
    if (wait > 0) await page.waitForTimeout(wait);
    if (poke && index > 0 && index % 40 === 0) await page.getByRole("button", { name: "Poke a pet" }).click();
    if (index % 24 === 12) await page.mouse.move(200 + ((index * 37) % 1000), 300 + ((index * 53) % 400));
    const pets = await page.evaluate(() =>
      [...document.querySelectorAll(".pet-layer svg.pet")].map((pet) => {
        const place = /translate\(([-\d.]+)px, ([-\d.]+)px\) scale\(([-\d.]+), ([-\d.]+)\)/u.exec(pet.style.transform);
        return { id: pet.getAttribute("data-pet"), x: place ? Number(place[1]) : 0, y: place ? Number(place[2]) : 0, flip: place ? Number(place[3]) : 1, opacity: Number(pet.style.opacity || "1") };
      }),
    );
    const file = `f${String(index).padStart(4, "0")}.jpg`;
    await page.screenshot({ path: join(frames, file), type: "jpeg", quality: 88 });
    samples.push({ at: Date.now() - started, file, pets });
  }
  const ids = [...new Set(samples.flatMap((sample) => sample.pets.map((pet) => pet.id)))].filter((id) => only.length === 0 || only.includes(id));
  const size = 86 * Number(scale);
  const across = 9;
  const rows = 6;
  for (const id of ids) {
    const cells = [];
    for (const sample of samples) {
      const pet = sample.pets.find((candidate) => candidate.id === id);
      if (!pet) continue;
      const left = Math.round(pet.x - size / 2);
      const top = Math.round(pet.y - size * 0.82);
      cells.push(`<figure><div style="width:${size}px;height:${size}px;background:url('${pathToFileURL(join(frames, sample.file)).href}') ${-left}px ${-top}px no-repeat"></div><figcaption>${(sample.at / 1000).toFixed(2)}s x${Math.round(pet.x)} y${Math.round(pet.y)}${pet.flip < 0 ? " ←" : " →"}${pet.opacity < 1 ? ` ${pet.opacity.toFixed(2)}` : ""}</figcaption></figure>`);
    }
    for (let part = 0; part * across * rows < cells.length; part++) {
      const html = `<!doctype html><meta charset="utf-8"><style>body{margin:6px;background:#777;font:9px monospace;color:#fff;display:flex;flex-wrap:wrap;gap:2px;width:${(size + 2) * across}px}figure{margin:0}h1{font-size:12px;width:100%;margin:0 0 4px}</style><h1>${id} · ${scene} · ${mode} · scale ${scale} · seed ${seed}${dark ? " · dark" : ""} · every ${every} ms · part ${part + 1}</h1>${cells.slice(part * across * rows, (part + 1) * across * rows).join("")}`;
      const sheet = join(out, `${name}-${id}-${part + 1}.html`);
      writeFileSync(sheet, html);
      const tab = await browser.newPage({ viewport: { width: (size + 2) * across + 14, height: 800 }, deviceScaleFactor: 1 });
      await tab.goto(pathToFileURL(sheet).href, { waitUntil: "load" });
      await tab.screenshot({ path: join(out, `${name}-${id}-${part + 1}.png`), fullPage: true });
      await tab.close();
      rmSync(sheet);
    }
  }
  writeFileSync(join(out, `${name}.json`), `${JSON.stringify({ scene, mode, scale, seed, dark, errors, samples: samples.map((sample) => ({ at: sample.at, pets: sample.pets })) })}\n`);
  console.log(JSON.stringify({ name, ids, samples: samples.length, errors }));
} finally {
  await browser.close();
  rmSync(frames, { recursive: true, force: true });
}
