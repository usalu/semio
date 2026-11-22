/** 🔍️ Captures the actual Draw canvas hierarchy from the local React playground. */
import { chromium } from "playwright";
const target = Bun.argv[2];
if (!target) throw Error("Expected a local Draw playground URL");
const browser = await chromium.launch({ headless: true });
try {
  const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
  page.on("pageerror", error => console.log("[DEBUG] Draw page error", error.message));
  page.on("console", message => {
    if (message.type() === "error" || message.type() === "warning") console.log("[DEBUG] Draw browser", message.type(), message.text().slice(0, 1200));
  });
  page.on("response", response => {
    if (response.status() >= 400) console.log("[DEBUG] Draw HTTP failure", response.status(), response.url());
  });
  await page.goto(target, { waitUntil: "domcontentloaded" });
  await page.getByRole("tab", { name: "Canvas" }).first().waitFor({ timeout: 45000 }).catch(() => undefined);
  const shape = await page.evaluate(() => {
    const attributes = node => Object.fromEntries(Array.from(node.attributes).map(attribute => [attribute.name, attribute.value.slice(0, 500)]));
    const bounds = node => { const bounds = node.getBoundingClientRect(); return [bounds.x, bounds.y, bounds.width, bounds.height]; };
    const canvases = Array.from(document.querySelectorAll("canvas"), node => {
      const parents = [];
      for (let parent = node, depth = 0; parent && depth < 6; parent = parent.parentElement, depth++) parents.push({ name: parent.localName, attributes: attributes(parent), size: bounds(parent) });
      return parents;
    });
    const surfaces = Array.from(document.querySelectorAll("[data-surface-id],[data-surface],[data-body-key]"), node => ({ name: node.localName, attributes: attributes(node), size: bounds(node) }));
    return { title: document.title, canvases, surfaces, body: document.body.innerText.slice(0, 2200) };
  });
  console.log(JSON.stringify(shape));
} finally { await browser.close(); }
