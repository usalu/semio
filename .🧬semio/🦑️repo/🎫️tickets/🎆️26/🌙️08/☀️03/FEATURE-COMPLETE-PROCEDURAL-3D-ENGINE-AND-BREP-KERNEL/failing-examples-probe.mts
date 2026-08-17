import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { join } from "node:path";

const BASE = process.env.PROCEDURAL3D_URL ?? "http://127.0.0.1:6018";
const OUT = process.env.PROBE_OUT ?? join(import.meta.dir, "failing-examples-report.json");
const EXAMPLES = [
  "Rectangle Wire Preview",
  "Sphere Cut With Torus",
  "Box Shell Preview",
];

async function selectExample(page, label: string) {
  const trigger = page.locator('[data-slot="select-trigger"]').first();
  await trigger.click({ timeout: 5000 });
  const item = page.locator('[data-slot="select-item"]').filter({ hasText: label }).first();
  await item.click({ timeout: 5000 });
}

async function snap(page) {
  return page.evaluate(() => {
    const host = document.querySelector(".semio-world-3d-host");
    const meshes = JSON.parse(host?.getAttribute("data-meshes-json") || "[]");
    const status = JSON.parse(host?.getAttribute("data-status-json") || "{}");
    const graph = document.querySelector(".semio-node-graph-host, [class*='NodeGraph']");
    const graphH = graph ? Math.round((graph as HTMLElement).getBoundingClientRect().height) : -1;
    // scrape widget status chips / data attributes
    const statuses: Record<string, unknown> = {};
    document.querySelectorAll("[data-widget-id],[data-node-id]").forEach((el) => {
      const id = el.getAttribute("data-widget-id") || el.getAttribute("data-node-id");
      if (!id) return;
      statuses[id] = {
        status: el.getAttribute("data-status"),
        error: el.getAttribute("data-error") || el.getAttribute("title"),
        text: (el as HTMLElement).innerText?.slice(0, 200),
      };
    });
    // eval json if exposed
    const evalEl = document.querySelector("[data-eval-json], [data-flow-eval-json]");
    const evalJson = evalEl?.getAttribute("data-eval-json") || evalEl?.getAttribute("data-flow-eval-json") || null;
    // also window debug hooks
    const w = window as any;
    return {
      meshCount: meshes.length,
      meshes: meshes.map((m: any) => ({
        id: m.id,
        positions: m.data?.positions?.length ?? 0,
        indices: m.data?.indices?.length ?? 0,
        edgePositions: m.data?.edgePositions?.length ?? m.data?.edge_positions?.length ?? 0,
      })),
      status,
      graphH,
      statuses,
      evalJson: evalJson ? evalJson.slice(0, 4000) : null,
      hooks: {
        hasEval: typeof w.__semioEvalJson === "string",
        evalLen: typeof w.__semioEvalJson === "string" ? w.__semioEvalJson.length : 0,
      },
      bodyText: document.body.innerText.slice(0, 500),
    };
  });
}

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1400, height: 900 } });
await page.goto(BASE, { waitUntil: "networkidle", timeout: 120000 });
await page.waitForTimeout(3000);

const results = [];
for (const label of EXAMPLES) {
  console.log("[DEBUG] start", label);
  try {
    await selectExample(page, label);
  } catch (e) {
    console.log("[DEBUG] select failed", label, String(e));
  }
  await page.waitForTimeout(8000);
  const s = await snap(page);
  console.log("[DEBUG] snap", label, JSON.stringify({ meshCount: s.meshCount, graphH: s.graphH, meshes: s.meshes }));
  results.push({ label, snap: s });
  await page.screenshot({ path: join(import.meta.dir, `failing-${label.replaceAll(" ", "-").toLowerCase()}.png`), fullPage: false });
}
writeFileSync(OUT, JSON.stringify(results, null, 2));
console.log("[DEBUG] wrote", OUT);
await browser.close();
