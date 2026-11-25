import assert from "node:assert/strict";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import tailwind from "@tailwindcss/vite";
import { chromium } from "playwright";
import { createServer } from "vite";

const ticket = dirname(fileURLToPath(import.meta.url));
const root = resolve(ticket, "../../../../../../..");
const server = await createServer({
  configFile: false,
  root: ticket,
  cacheDir: resolve(ticket, "🗑️generated/react-vite"),
  plugins: [tailwind()],
  server: { host: "127.0.0.1", port: 6074, strictPort: true, fs: { allow: [root] } },
});
await server.listen();
try {
  const browser = await chromium.launch({ channel: "chrome", headless: true });
  try {
    const page = await browser.newPage();
    page.on("pageerror", error => console.error(error));
    await page.goto("http://127.0.0.1:6074/react-probe.html");
    await page.waitForSelector("[data-icon-shot-frame]");
    for (const colorScheme of ["light", "dark"]) {
      await page.emulateMedia({ colorScheme, reducedMotion: "reduce" });
      await page.evaluate(scheme => document.documentElement.classList.toggle("dark", scheme === "dark"), colorScheme);
      const surfaces = await page.evaluate(() => {
        const windowFrame = document.querySelector("[data-window-frame]");
        const body = windowFrame.querySelector("[data-window-frame-content]");
        const fill = windowFrame.querySelector("[data-slot=window-chrome-body-surface]");
        const chip = windowFrame.querySelector("[data-window-frame-chip]");
        return {
          window: {
            body: body.dataset.level,
            fill: fill.dataset.level,
            chip: chip.dataset.level,
            touches: chip.getBoundingClientRect().top === windowFrame.getBoundingClientRect().top,
            blur: getComputedStyle(chip).backdropFilter,
          },
          shots: [...document.querySelectorAll("[data-icon-shot-frame]")].map(frame => {
            const dimensions = frame.querySelector("[data-glass-chip]");
            const style = getComputedStyle(frame);
            const border = Number.parseFloat(style.borderBottomWidth);
            return {
              shape: frame.dataset.iconShotShape,
              level: dimensions.dataset.level,
              blur: getComputedStyle(dimensions).backdropFilter,
              border,
              radius: style.borderBottomLeftRadius,
              touches: Math.abs(frame.getBoundingClientRect().bottom - dimensions.getBoundingClientRect().bottom - border) < 0.01,
            };
          }),
        };
      });
      assert.deepEqual([surfaces.window.body, surfaces.window.fill, surfaces.window.chip], ["window", "window", "pane"]);
      assert(surfaces.window.touches);
      assert.match(surfaces.window.blur, /blur\(/);
      assert.deepEqual(surfaces.shots.map(shot => shot.shape), ["rectangle", "ellipse"]);
      for (const shot of surfaces.shots) {
        assert.equal(shot.level, "panel");
        assert.equal(shot.border, 1);
        assert(shot.touches);
        assert.match(shot.blur, /blur\(/);
        assert.equal(shot.radius, shot.shape === "ellipse" ? "50%" : "0px");
      }
      console.log(`${colorScheme} chrome and crop guide geometry verified`);
    }
  } finally {
    await browser.close();
  }
} finally {
  await server.close();
}
