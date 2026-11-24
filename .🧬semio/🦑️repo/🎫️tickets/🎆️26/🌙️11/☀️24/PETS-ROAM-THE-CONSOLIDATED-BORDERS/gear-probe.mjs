/** 🧗️ Reads on the dev page what the pet engine has to work with for gear on a quiz's page: perches, pitches and the routes a ladder, a rope or a climb opens between perches. Usage: node gear-probe.mjs [quiz] */
import { createRequire } from "node:module";
const require = createRequire("C:/semio/.cache/b/d/m/j/c642535c86e40717744c/node_modules/");
const { chromium } = require("playwright");
const quiz = process.argv[2] ?? "heating";
const size = { width: Number(process.argv[3] ?? 40), height: Number(process.argv[4] ?? 40) };
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1280, height: 720 } });
await page.goto("http://127.0.0.1:6061/" + (quiz === "home" ? "" : "#" + quiz));
await page.waitForSelector(quiz === "home" ? "#course-main [data-card] [data-window-frame]" : `[data-layered-pane="${quiz}"][data-opened] [data-pet-prop]`, { timeout: 180000 });
await page.waitForTimeout(3000);
console.log(JSON.stringify(await page.evaluate(async (size) => {
  const base = "/@fs/C:/semio/🧰️framework/🛍️products/🐾️pets/";
  const s = await import(base + "🎯️targets/⚛️react/🔨️modules/📡️survey/🟦️.ts");
  const t = await import(base + "🔨️modules/🏞️terrain/🟦️.ts");
  const g = await import(base + "🔨️modules/🧗️climbing/🟦️.ts");
  const c = await import("/@fs/C:/semio/🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/🐾️pets/🎪️stage/🟦️.tsx");
  const host = document.querySelector("svg.pet")?.parentElement ?? null;
  const ev = s.survey(document, { surfaces: c.COURSE_PET_SURFACES, open: c.COURSE_PET_OPEN, keepouts: s.PET_KEEPOUTS + ", " + c.COURSE_PET_KEEPOUTS, controls: c.COURSE_PET_CONTROLS, props: c.COURSE_PET_PROPS, frame: host });
  const perches = t.perchesOf(ev.surfaces, ev.keepouts, ev.width, ev.height, size.height + 8, size.width);
  const pitches = t.wallsOf(ev.walls, ev.keepouts, ev.width, ev.height, size.width + 8, 0.5 * size.height);
  const name = (p) => [p.surface, Math.round(p.x0), Math.round(p.x1), Math.round(p.y)].join(":");
  const routes = { raise: [], grapple: [], wall: [] };
  for (const from of perches) for (const to of perches) {
    if (!(to.y < from.y)) continue;
    for (const gear of ["ladder", "grapple", "climb"]) {
      const legs = g.routeOf((from.x0 + from.x1) / 2, from, to, [gear], size, 1e9, pitches, [], ev.keepouts);
      if (legs !== null) routes[legs[0].means]?.push(name(from) + " → " + name(to));
    }
  }
  return { keys: Object.keys(ev), size: [ev.width, ev.height], walls: ev.walls.length, perches: perches.map(name), pitches: pitches.map((p) => [p.wall, p.surface, p.side, Math.round(p.x), Math.round(p.y0), Math.round(p.y1)].join(":")), keepouts: ev.keepouts.map((k) => [k.x, k.y, k.width, k.height].map(Math.round).join(",")), routes };
}, size), null, 1));
await browser.close();
