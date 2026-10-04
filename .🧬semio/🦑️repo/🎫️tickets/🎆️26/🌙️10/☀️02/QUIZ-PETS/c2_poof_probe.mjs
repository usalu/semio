/** 🔬️ Ticket tool of work package C2: reads what the poof specimen of one species page holds while its dust is in the air — the effects root, its scale, the clouds and the first blobs with their computed paints and boxes — to see why dust would not show. `node c2_poof_probe.mjs [--url http://127.0.0.1:6253/] [--species housy]`. */
import { chromium } from "playwright";

const args = process.argv.slice(2);
const option = (name, fallback) => (args.includes(`--${name}`) ? args[args.indexOf(`--${name}`) + 1] : fallback);
const base = option("url", "http://127.0.0.1:6253/");
const species = option("species", "housy");
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.routeWebSocket(/./, (socket) => {
  const server = socket.connectToServer();
  server.onMessage((message) => {
    if (typeof message === "string" && /"type":"(full-reload|update)"/u.test(message)) return;
    socket.send(message);
  });
});
await page.goto(`${base}#${species}`, { waitUntil: "networkidle" });
const poof = page.locator(`.story[data-species="${species}"] .ground-light .specimen[data-scene="poof"]`);
await poof.scrollIntoViewIfNeeded();
await page.waitForFunction((id) => document.querySelector(`.story[data-species="${id}"] .ground-light .specimen[data-scene="poof"] [data-pet-puff][visibility="visible"]`) !== null, species, { timeout: 8000 });
const seen = await page.evaluate((id) => {
  const specimen = document.querySelector(`.story[data-species="${id}"] .ground-light .specimen[data-scene="poof"]`);
  const root = specimen.querySelector(".pet-effects");
  const cloud = root.querySelector('[data-pet-puff][visibility="visible"]');
  const circles = [...cloud.querySelectorAll("circle")].slice(0, 3).concat([...cloud.querySelectorAll("circle")].slice(8, 10));
  return {
    root: { transform: root.style.transform, box: root.getBoundingClientRect().toJSON(), position: getComputedStyle(root).position, display: getComputedStyle(root).display, overflow: getComputedStyle(root).overflow, parent: root.parentElement.className },
    cloud: { transform: cloud.getAttribute("transform"), opacity: cloud.getAttribute("opacity"), box: cloud.getBoundingClientRect().toJSON() },
    circles: circles.map((circle) => ({ cx: circle.getAttribute("cx"), cy: circle.getAttribute("cy"), r: circle.getAttribute("r"), fill: getComputedStyle(circle).fill, stroke: getComputedStyle(circle).stroke, box: circle.getBoundingClientRect().toJSON() })),
    stage: specimen.querySelector(".specimen-stage").getBoundingClientRect().toJSON(),
  };
}, species);
process.stdout.write(`${JSON.stringify(seen, null, 2)}\n`);
await browser.close();
