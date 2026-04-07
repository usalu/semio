import { chromium } from "playwright";

async function main() {
  const browser = await chromium.launch({ headless: true, args: ["--no-sandbox", "--disable-gpu", "--disable-dev-shm-usage"] });
  const page = await browser.newPage();
  const errors: string[] = [];
  const warnings: string[] = [];
  page.on("console", (msg) => {
    if (msg.type() === "error") errors.push(msg.text());
    if (msg.type() === "warning") warnings.push(msg.text());
  });
  page.on("pageerror", (err) => errors.push(`PageError: ${err.message}`));
  await page.goto("http://localhost:5173/");
  await page.waitForTimeout(3000);
  console.log("Page title:", await page.title());
  console.log("Errors:", errors.slice(0, 10));
  console.log("Warnings:", warnings.slice(0, 5));
  const hasReactFlow = await page.locator(".react-flow").count();
  console.log("ReactFlow count on home:", hasReactFlow);
  const allIds = await page.evaluate(() => Array.from(document.querySelectorAll("[id]")).map(el => el.id).filter(id => id.includes("diagram") || id.includes("workbench") || id.includes("scene")).slice(0, 20));
  console.log("Relevant IDs on home:", allIds);
  await page.goto("http://localhost:5173/kits/f042c2a4-3ba5-44b0-b22c-0ae8f568aacc/designs/37ba7ec4-9023-4be7-9ab6-e0ebc80007f8");
  await page.waitForTimeout(10000);
  console.log("URL:", page.url());
  const rfCount = await page.locator(".react-flow").count();
  const rfNodeCount = await page.locator(".react-flow__node").count();
  const glItems = await page.locator(".lm_item").count();
  const diagramEl = await page.locator("#diagram").count();
  console.log(`ReactFlow elements: ${rfCount}, Nodes: ${rfNodeCount}, GL items: ${glItems}, #diagram: ${diagramEl}`);
  const allDesignIds = await page.evaluate(() => Array.from(document.querySelectorAll("[id]")).map(el => el.id).filter(id => id.includes("diagram") || id.includes("workbench") || id.includes("scene") || id === "root").slice(0, 30));
  console.log("Design page IDs:", allDesignIds);
  const errors2 = errors.filter(e => !e.includes("favicon"));
  console.log("Runtime errors:", errors2.slice(0, 10));
  await browser.close();
}
main().catch(console.error);
