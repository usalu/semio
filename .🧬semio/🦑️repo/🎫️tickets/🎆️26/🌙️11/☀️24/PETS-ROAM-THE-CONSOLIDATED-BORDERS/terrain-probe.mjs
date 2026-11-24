/** 🗺️ Lists on the dev page the walls and surfaces the survey of a quiz's page yields and what lies in a given band of the page. Usage: node terrain-probe.mjs [quiz] [y] */
import { createRequire } from "node:module";
const require = createRequire("C:/semio/.cache/b/d/m/j/c642535c86e40717744c/node_modules/");
const { chromium } = require("playwright");
const quiz = process.argv[2] ?? "heating";
const band = Number(process.argv[3] ?? 370);
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1280, height: 720 } });
await page.goto("http://127.0.0.1:6061/#" + quiz);
await page.waitForSelector(`[data-layered-pane="${quiz}"][data-opened] [data-pet-prop]`, { timeout: 180000 });
await page.waitForTimeout(3000);
console.log(JSON.stringify(await page.evaluate(async (band) => {
  const base = "/@fs/C:/semio/🧰️framework/🛍️products/🐾️pets/";
  const s = await import(base + "🎯️targets/⚛️react/🔨️modules/📡️survey/🟦️.ts");
  const c = await import("/@fs/C:/semio/🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/🐾️pets/🎪️stage/🟦️.tsx");
  const host = document.querySelector("svg.pet")?.parentElement ?? null;
  const keep = s.PET_KEEPOUTS + ", " + c.COURSE_PET_KEEPOUTS;
  const ev = s.survey(document, { surfaces: c.COURSE_PET_SURFACES, open: c.COURSE_PET_OPEN, keepouts: keep, controls: c.COURSE_PET_CONTROLS, props: c.COURSE_PET_PROPS, frame: host });
  const tag = (e) => e.tagName.toLowerCase() + [...e.attributes].filter((a) => a.name.startsWith("data-") || a.name === "id").map((a) => `[${a.name}=${a.value.slice(0, 24)}]`).join("") + "." + String(e.className).slice(0, 60);
  const box = (e) => { const r = e.getBoundingClientRect(); return [r.x, r.y, r.width, r.height].map(Math.round).join(","); };
  const inBand = (e) => { const r = e.getBoundingClientRect(); return r.width > 0 && r.top <= band && band <= r.bottom; };
  return {
    walls: ev.walls.map((w) => JSON.stringify(w, (k, v) => (typeof v === "number" ? Math.round(v) : v))),
    surfaces: ev.surfaces.map((w) => JSON.stringify(w, (k, v) => (typeof v === "number" ? Math.round(v) : v))),
    keepoutsInBand: [...document.querySelectorAll(keep)].filter(inBand).map((e) => box(e) + " " + tag(e)),
    surfacesInDom: [...document.querySelectorAll(c.COURSE_PET_SURFACES)].filter((e) => e.getBoundingClientRect().width > 0).map((e) => box(e) + " " + tag(e)),
  };
}, band), null, 1));
await browser.close();
