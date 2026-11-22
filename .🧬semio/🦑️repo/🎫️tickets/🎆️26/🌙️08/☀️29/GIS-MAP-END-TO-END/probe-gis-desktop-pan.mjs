import { chromium, devices } from "playwright";
import png from "pngjs";

const { defaultBrowserType, ...device } = devices["Desktop Chrome"];
const browser = await chromium.launch({ channel: "chrome", headless: true });
const delay = milliseconds => new Promise(resolve => setTimeout(resolve, milliseconds));
const capture = async canvas => png.PNG.sync.read(await canvas.screenshot({ scale: "css", timeout: 5000 }));
const score = (before, after) => {
  if (before.width !== after.width || before.height !== after.height) return { resized: true };
  let differing = 0;
  let grid = 0;
  for (let index = 0; index < before.data.length; index += 4) {
    if (Math.max(...[0, 1, 2].map(channel => Math.abs(before.data[index + channel] - after.data[index + channel]))) >= 16) differing++;
  }
  for (let y = 1; y < 12; y++) for (let x = 1; x < 12; x++) {
    const index = 4 * (Math.floor(y * before.height / 12) * before.width + Math.floor(x * before.width / 12));
    if ([0, 1, 2].some(channel => before.data[index + channel] >> 3 !== after.data[index + channel] >> 3)) grid++;
  }
  return { pixels: differing, grid, size: [before.width, before.height] };
};
try {
  const page = await browser.newPage({ ...device, locale: "en-US" });
  await page.goto(process.argv[2] ?? "http://127.0.0.1:6041/");
  await page.locator('[data-shell-ready="gis2d"]').first().waitFor({ timeout: 30000 });
  const canvas = page.locator(".semio-tiled-map-host canvas").first();
  await canvas.waitFor({ state: "visible" });
  await delay(2000);
  const { x, y, width, height } = await canvas.boundingBox();
  const cx = Math.round(x + width * 0.5);
  const cy = Math.round(y + height * 0.5);
  const dx = Math.round(Math.min(width * 0.3, 160));
  const terrain = await canvas.evaluate((element, points) => {
    const describe = node => ({ tag: node?.tagName, id: node?.id, className: String(node?.className ?? "").slice(0, 180) });
    const events = [];
    window.__gisPanProbeEvents = events;
    for (const type of ["pointerdown", "pointermove", "pointerup", "lostpointercapture", "auxclick", "wheel"]) {
      window.addEventListener(type, event => events.push({
        type, button: event.button, buttons: event.buttons, trusted: event.isTrusted,
        pointerType: event.pointerType, deltaY: event.deltaY, onCanvas: event.composedPath().includes(element),
        x: event.clientX, y: event.clientY,
      }), { capture: true, passive: true });
    }
    return { renderer: element.dataset.mapRenderer, targets: points.map(([x, y]) => describe(document.elementFromPoint(x, y))) };
  }, [[cx - 30, cy], [cx, cy], [cx - 30 + dx, cy]]);
  const getCrossings = () => page.evaluate(() => JSON.stringify(window.__semioCrossingCensus));
  const before = await capture(canvas);
  const crossings = await getCrossings();
  let during;
  if (process.argv.includes("--wheel-only")) {
    await page.mouse.move(cx, cy);
    await page.mouse.wheel(0, -360);
  } else {
    await page.mouse.move(cx - 30, cy);
    await page.mouse.down({ button: "middle" });
    for (let step = 1; step <= 12; step++) {
      await page.mouse.move(cx - 30 + dx * step / 12, cy);
      await delay(18);
    }
    during = score(before, await capture(canvas));
    await page.mouse.up({ button: "middle" });
  }
  await delay(600);
  const after = score(before, await capture(canvas));
  await delay(2400);
  const settled = score(before, await capture(canvas));
  const events = await page.evaluate(() => window.__gisPanProbeEvents);
  console.log(JSON.stringify({ ...terrain, during, after, settled, crossingsBefore: JSON.parse(crossings), crossingsAfter: JSON.parse(await getCrossings()), events }));
} finally {
  await browser.close();
}
