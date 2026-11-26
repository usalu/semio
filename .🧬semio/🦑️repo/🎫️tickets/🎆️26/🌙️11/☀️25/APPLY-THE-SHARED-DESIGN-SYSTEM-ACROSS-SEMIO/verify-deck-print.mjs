import assert from "node:assert/strict";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import tailwind from "@tailwindcss/vite";
import { chromium } from "playwright";
import { createServer } from "vite";

const ticket = dirname(fileURLToPath(import.meta.url));
const output = resolve(ticket, "🗑️generated/print2");
const server = await createServer({
  configFile: false,
  root: ticket,
  cacheDir: resolve(output, "vite"),
  plugins: [tailwind()],
  server: { host: "127.0.0.1", port: 6182, strictPort: true, fs: { allow: [resolve(ticket, "../../../../../../..")] } },
});
await server.listen();
try {
  const browser = await chromium.launch({ channel: "chrome", headless: true });
  try {
    for (const colorScheme of ["light", "dark"]) {
      const page = await browser.newPage({ viewport: { width: 1024, height: 600 }, colorScheme });
      page.on("pageerror", error => console.error("[DEBUG]", error));
      await page.goto("http://127.0.0.1:6182/print-deck.html?print-pdf&theme=" + colorScheme);
      await page.waitForSelector(".pdf-page > .slide-number-pdf[data-level=pane]");
      await page.emulateMedia({ media: "print", colorScheme });
      const heading = page.getByRole("heading", { name: "Printable slide" });
      await heading.waitFor();
      const marks = await page.locator(".pdf-page").evaluate(sheet => {
        const mark = node => {
          const { x, y, width, height } = node.getBoundingClientRect();
          const { backgroundColor, backgroundImage, outlineColor } = getComputedStyle(node);
          return { x, y, width, height, backgroundColor, backgroundImage, outlineColor, level: node.getAttribute("data-level") };
        };
        return [mark(sheet), mark(sheet.querySelector(".slide-background")), mark(sheet.querySelector(".slide-number-pdf"))];
      });
      const [sheet, content, chip] = marks;
      assert.deepEqual(marks.map(item => item.level), ["base", "window", "pane"]);
      assert.notEqual(sheet.backgroundColor, content.backgroundColor);
      assert.notEqual(content.backgroundColor, chip.backgroundColor);
      assert.notEqual(chip.backgroundImage, "none");
      assert.ok(content.x > sheet.x && content.y > sheet.y);
      assert.ok(content.x + content.width < sheet.x + sheet.width);
      assert.ok(content.y + content.height < sheet.y + sheet.height);
      assert.ok(chip.x + chip.width <= content.x + content.width);
      assert.ok(chip.y + chip.height <= content.y + content.height);
      const text = await heading.boundingBox();
      assert.ok(text.x >= content.x && text.y >= content.y);
      assert.ok(text.x + text.width <= content.x + content.width);
      assert.ok(text.y + text.height <= content.y + content.height);
      await page.pdf({ path: resolve(output, `deck-${colorScheme}.pdf`), printBackground: true, preferCSSPageSize: true });
      console.log("[DEBUG] Reveal", colorScheme, marks);
      await page.close();
    }
  } finally {
    await browser.close();
  }
} finally {
  await server.close();
}
