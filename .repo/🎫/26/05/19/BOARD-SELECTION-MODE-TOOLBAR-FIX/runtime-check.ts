import { chromium } from "playwright";

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { height: 900, width: 1600 } });
const logs: string[] = [];

page.on("console", (message) => logs.push(message.text()));
console.log("[DEBUG] runtime-check goto");
await page.goto("http://127.0.0.1:6016/", { timeout: 60_000, waitUntil: "domcontentloaded" });
console.log("[DEBUG] runtime-check wait fixture shelf");
await page.getByTestId("board-play-fixture-shelf").waitFor({ state: "visible", timeout: 60_000 });

console.log("[DEBUG] runtime-check probe toolbar");
const defaultCount = await page.locator('button[title="Default"]').count();
console.log("[DEBUG] runtime-check probe canvas");
const result = await page
  .locator('[data-testid="board-canvas"]')
  .first()
  .evaluate((element) => {
    const canvas = element as HTMLCanvasElement & {
      __boardRenderer?: {
        scene: { nodes: Map<string, { id: string; x: number; y: number }> };
        selection: { getSnapshot: () => { ids: string[] } };
        setSelectionIds: (ids: string[]) => void;
        worldToScreen: (point: { x: number; y: number }) => { x: number; y: number };
      };
    };
    const renderer = canvas.__boardRenderer;
    if (!renderer) {
      throw new Error("missing renderer");
    }
    const nodes = [...renderer.scene.nodes.values()].slice(0, 2);
    if (nodes.length < 2) {
      throw new Error("need two nodes");
    }
    const fire = (node: { x: number; y: number }, init: { ctrlKey?: boolean; shiftKey?: boolean }) => {
      const point = renderer.worldToScreen({ x: node.x, y: node.y });
      const rect = canvas.getBoundingClientRect();
      const eventInit = { bubbles: true, button: 0, clientX: rect.left + point.x, clientY: rect.top + point.y, ...init };
      canvas.dispatchEvent(new MouseEvent("pointerdown", eventInit));
      canvas.dispatchEvent(new MouseEvent("pointerup", eventInit));
    };
    renderer.setSelectionIds([nodes[0].id]);
    fire(nodes[1], { shiftKey: true });
    const shift = renderer.selection.getSnapshot().ids;
    fire(nodes[0], { ctrlKey: true });
    const ctrl = renderer.selection.getSnapshot().ids;
    fire(nodes[1], { ctrlKey: true, shiftKey: true });
    const both = renderer.selection.getSnapshot().ids;
    fire(nodes[0], {});
    const plain = renderer.selection.getSnapshot().ids;
    return { both, ctrl, nodeIds: nodes.map((node) => node.id), plain, shift };
  });

console.log(`[DEBUG] board selection toolbar runtime ${JSON.stringify({ defaultCount, result, consoleErrors: logs.filter((text) => text.toLowerCase().includes("error")) })}`);
await browser.close();
