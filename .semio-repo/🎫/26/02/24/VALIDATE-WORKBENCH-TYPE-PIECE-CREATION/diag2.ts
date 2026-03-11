import { chromium } from "playwright";

async function main() {
  const browser = await chromium.launch({ headless: true, args: ["--no-sandbox", "--disable-gpu", "--disable-dev-shm-usage"] });
  const page = await browser.newPage();
  const errors: string[] = [];
  page.on("console", (msg) => {
    if (msg.type() === "error") errors.push(msg.text());
  });
  page.on("pageerror", (err) => errors.push(`PageError: ${err.message}`));
  await page.goto("http://localhost:5173/kits/f042c2a4-3ba5-44b0-b22c-0ae8f568aacc/designs/37ba7ec4-9023-4be7-9ab6-e0ebc80007f8");
  await page.waitForTimeout(10000);
  const diagramHtml = await page.evaluate(() => {
    const el = document.getElementById("diagram");
    return el?.innerHTML?.slice(0, 2000) ?? "NOT FOUND";
  });
  console.log("Diagram HTML:", diagramHtml);
  const storeState = await page.evaluate(() => {
    const store = (window as any).__SEMIO_STORE__;
    if (!store) return "NO STORE";
    const kitGuids = Array.from((store as any).kits?.keys() ?? []) as string[];
    if (kitGuids.length === 0) return "NO KITS";
    const kitStore = (store as any).kit(kitGuids[0]);
    if (!kitStore) return "NO KIT STORE";
    const kit = kitStore.snapshot();
    const designs = kit.designs ?? [];
    return { designCount: designs.length, firstDesignPieceCount: designs[0]?.pieces?.length ?? 0 };
  });
  console.log("Store state:", JSON.stringify(storeState));
  const scopeState = await page.evaluate(() => {
    const actor = (window as any).__SEMIO_ACTOR__;
    if (!actor) return "NO ACTOR";
    const snapshot = actor.getSnapshot();
    return { hasContext: !!snapshot?.context, designApps: Object.keys(snapshot?.context?.designApps || {}).length };
  });
  console.log("Actor state:", JSON.stringify(scopeState));
  await page.waitForTimeout(10000);
  const diagramHtml2 = await page.evaluate(() => {
    const el = document.getElementById("diagram");
    return el?.innerHTML?.slice(0, 2000) ?? "NOT FOUND";
  });
  console.log("Diagram HTML after 20s:", diagramHtml2);
  const rfCount2 = await page.locator(".react-flow").count();
  const rfNodeCount2 = await page.locator(".react-flow__node").count();
  console.log(`After 20s - ReactFlow: ${rfCount2}, Nodes: ${rfNodeCount2}`);
  console.log("Errors:", errors.slice(0, 10));
  await browser.close();
}
main().catch(console.error);
