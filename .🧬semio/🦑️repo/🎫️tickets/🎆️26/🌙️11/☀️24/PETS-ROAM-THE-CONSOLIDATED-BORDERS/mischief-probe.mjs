/** 🔬️ Reads on the dev page what the pet engine has to work with for mischief on a quiz's page: fixtures, perches and the station of each fixture. Usage: node mischief-probe.mjs [quiz] */
import { createRequire } from "node:module";
const require = createRequire("C:/semio/.cache/b/d/m/j/c642535c86e40717744c/node_modules/");
const { chromium } = require("playwright");
const quiz = process.argv[2] ?? "heating";
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto("http://127.0.0.1:6061/#" + quiz);
await page.waitForSelector(`[data-layered-pane="${quiz}"][data-opened] [data-pet-prop]`, { timeout: 180000 });
await page.waitForTimeout(3000);
console.log(JSON.stringify(await page.evaluate(async () => {
  const base = "/@fs/C:/semio/🧰️framework/🛍️products/🐾️pets/";
  const s = await import(base + "🎯️targets/⚛️react/🔨️modules/📡️survey/🟦️.ts");
  const t = await import(base + "🔨️modules/🏞️terrain/🟦️.ts");
  const m = await import(base + "🔨️modules/🪄️mischief/🟦️.ts");
  const c = await import("/@fs/C:/semio/🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/🐾️pets/🎪️stage/🟦️.tsx");
  const host = document.querySelector("svg.pet")?.parentElement ?? null;
  const ev = s.survey(document, { surfaces: c.COURSE_PET_SURFACES, open: c.COURSE_PET_OPEN, keepouts: s.PET_KEEPOUTS + ", " + c.COURSE_PET_KEEPOUTS, controls: c.COURSE_PET_CONTROLS, props: c.COURSE_PET_PROPS, frame: host });
  const perches = t.perchesOf(ev.surfaces, ev.keepouts, ev.width, ev.height, 48, 48);
  const round = (o) => Object.fromEntries(Object.entries(o).map(([k, v]) => [k, typeof v === "number" ? Math.round(v) : v]));
  return { pets: document.querySelectorAll("svg.pet").length, size: [ev.width, ev.height], surfaces: ev.surfaces.length, perches: perches.map((p) => [p.surface, Math.round(p.x0), Math.round(p.x1), Math.round(p.y)].join(":")), fixtures: ev.fixtures.map((f) => ({ ...round(f), station: m.stationFor(f, [], perches, ev.width) })).map((f) => ({ key: f.key, box: [f.x, f.y, f.width, f.height].join(","), station: f.station && round(f.station) })) };
}), null, 1));
await browser.close();
