/** ⏱️ Ticket tool of work package C2: measures how long the animation frames of the stories gallery take for one species page at a few viewport heights (headless Chromium), and how many specimens are painted — to keep the species pages smooth. `node c2_frame_probe.mjs [--url http://127.0.0.1:6253/] [--species sunny]`. */
import { chromium } from "playwright";

const args = process.argv.slice(2);
const option = (name, fallback) => (args.includes(`--${name}`) ? args[args.indexOf(`--${name}`) + 1] : fallback);
const base = option("url", "http://127.0.0.1:6253/");
const species = option("species", "sunny");
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
await page.waitForSelector(`.story[data-species="${species}"]`);
for (const height of [900, 2400, 5000]) {
  await page.setViewportSize({ width: 1440, height });
  await new Promise((done) => setTimeout(done, 1500));
  const measured = await page.evaluate(
    () =>
      new Promise((done) => {
        const times = [];
        let last = performance.now();
        const step = (now) => {
          times.push(now - last);
          last = now;
          if (times.length < 90) requestAnimationFrame(step);
          else {
            const sorted = [...times].sort((one, other) => one - other);
            done({ frames: times.length, median: sorted[45], p90: sorted[81], max: sorted.at(-1), specimens: document.querySelectorAll(".specimen").length, painted: [...document.querySelectorAll(".specimen-stage svg.pet")].length, nodes: document.querySelectorAll("*").length });
          }
        };
        requestAnimationFrame(step);
      }),
  );
  process.stdout.write(`${height}: ${JSON.stringify(measured)}\n`);
}
await browser.close();
