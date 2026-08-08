#!/usr/bin/env bun
import { chromium } from "playwright";
import { writeFile, mkdir } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = path.dirname(fileURLToPath(import.meta.url));
const outDir = path.join(ticketDir, "example-shots");
await mkdir(outDir, { recursive: true });

const examples = (process.env.EXAMPLES?.split(",") ?? [
  "hexagonal-mushroom-column",
  "rectangle-extrude-volume",
  "sphere-cut-with-torus",
  "box-fillet-preview",
  "sphere-box-fuse",
  "face-sweep-extrude",
  "rectangle-wire-preview",
  "box-shell-preview",
]).filter(Boolean);

const labels: Record<string, string> = {
  "hexagonal-mushroom-column": "Hexagonal Mushroom Column",
  "rectangle-extrude-volume": "Rectangle Extrude Volume",
  "sphere-cut-with-torus": "Sphere Cut With Torus",
  "box-fillet-preview": "Box Fillet Preview",
  "sphere-box-fuse": "Sphere Box Fuse",
  "face-sweep-extrude": "Face Sweep Extrude",
  "rectangle-wire-preview": "Rectangle Wire Preview",
  "box-shell-preview": "Box Shell Preview",
};

const expectedKey: Record<string, string> = {
  "hexagonal-mushroom-column": "column-preview",
  "rectangle-extrude-volume": "rect",
  "sphere-cut-with-torus": "brep_bool_cut_5",
  "box-fillet-preview": "fillet",
  "sphere-box-fuse": "fuse",
  "face-sweep-extrude": "face",
  "rectangle-wire-preview": "rect",
  "box-shell-preview": "shell",
};

async function openPage(browser: Awaited<ReturnType<typeof chromium.launch>>, fixtureId?: string) {
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
  for (let attempt = 0; attempt < 3; attempt++) {
    try {
      const fixtureQ = fixtureId ? ("&fixture=" + encodeURIComponent(fixtureId)) : "";
      await page.goto("http://127.0.0.1:6018/?plugin=procedural3d" + fixtureQ + "&bust=" + Date.now(), {
        waitUntil: "domcontentloaded",
        timeout: 240000,
      });
      await page.waitForSelector(".semio-node-graph-host[data-status-json]", { timeout: 240000 });
      await page.waitForTimeout(4000);
      return page;
    } catch (err) {
      console.log("[DEBUG] openPage retry", attempt, String(err));
      await page.waitForTimeout(2000);
    }
  }
  throw new Error("openPage failed");
}

async function snap(page: Awaited<ReturnType<Awaited<ReturnType<typeof chromium.launch>>["newPage"]>>) {
  try {
    return await page.evaluate(() => {
      const status = JSON.parse(document.querySelector(".semio-node-graph-host")?.getAttribute("data-status-json") || "{}");
      const meshes = JSON.parse(document.querySelector(".semio-world-3d-host")?.getAttribute("data-meshes-json") || "[]");
      const entries = Object.values(status) as any[];
      return {
        allOk: entries.length > 0 && entries.every((e) => e.status === "ok"),
        meshCount: meshes.length,
        keys: Object.keys(status).sort(),
        blocked: Object.entries(status).filter(([, v]: any) => v.status === "blocked").map(([k]) => k),
        errors: Object.entries(status).filter(([, v]: any) => v.status === "error").map(([k]) => k),
        errorDetails: Object.entries(status)
          .filter(([, v]: any) => v.status === "error")
          .map(([k, v]: any) => ({ k, message: v.message ?? v.error ?? v })),
        trigger: (document.getElementById("playground.navbar.fixture.trigger")?.textContent || "").trim(),
        graphH: Math.round((document.querySelector(".semio-node-graph-host") as HTMLElement | null)?.getBoundingClientRect().height || 0),
        meshEdges: meshes.map((m: any) => m.data?.edgePositions?.length ?? m.data?.edge_positions?.length ?? 0),
        meshPositions: meshes.map((m: any) => m.data?.positions?.length ?? 0),
      };
    });
  } catch (err) {
    return {
      allOk: false,
      meshCount: 0,
      keys: [] as string[],
      blocked: [] as string[],
      errors: ["evaluate-failed"],
      errorDetails: [{ k: "snap", message: String(err) }],
      trigger: "",
      graphH: 0,
      meshEdges: [] as number[],
      meshPositions: [] as number[],
      destroyed: true,
    };
  }
}

async function selectExample(page: any, exampleId: string) {
  const label = labels[exampleId] ?? exampleId;
  for (let attempt = 0; attempt < 6; attempt++) {
    const how = await page.evaluate(async ({ label }: { label: string }) => {
      const trigger = document.getElementById("playground.navbar.fixture.trigger") as HTMLButtonElement | null;
      if (!trigger) return "no-trigger";
      const current = (trigger.textContent || "").trim();
      if (current === label) return "already:" + label;
      trigger.click();
      await new Promise((r) => setTimeout(r, 800));
      const items = Array.from(document.querySelectorAll("[data-slot=select-item], [role=option]")) as HTMLElement[];
      const item = items.find((el) => ((el.textContent || "").trim() === label));
      if (!item) {
        trigger.click();
        return "missing:" + items.map((el) => (el.textContent || "").trim()).slice(0, 12).join("|");
      }
      item.dispatchEvent(new MouseEvent("pointerdown", { bubbles: true }));
      item.click();
      await new Promise((r) => setTimeout(r, 500));
      return "clicked:" + label + " -> " + ((document.getElementById("playground.navbar.fixture.trigger")?.textContent || "").trim());
    }, { label }).catch((err: unknown) => "evaluate-failed:" + String(err));
    await page.waitForTimeout(1500);
    const s = await snap(page);
    if ((s as any).destroyed) return { how: "destroyed", needReload: true };
    if (s.trigger === label && s.keys.includes(expectedKey[exampleId]!)) {
      return { how, needReload: false };
    }
    console.log("[DEBUG] select retry", exampleId, how, s.trigger, s.keys);
  }
  return { how: "failed-select", needReload: false };
}

async function settle(page: any, exampleId: string, timeoutMs = 120000) {
  const want = expectedKey[exampleId]!;
  const label = labels[exampleId]!;
  const deadline = Date.now() + timeoutMs;
  let last: any = null;
  let kickedTessellate = false;
  while (Date.now() < deadline) {
    last = await snap(page);
    if (last.destroyed) return last;
    const fixtureOk = last.trigger === label && last.keys.includes(want);
    const hasMesh = last.meshCount > 0 || (last.meshEdges ?? []).some((n: number) => n > 0) || (last.meshPositions ?? []).some((n: number) => n > 0);
    if (fixtureOk && last.allOk && hasMesh) return last;
    if (fixtureOk && last.allOk && !hasMesh && !kickedTessellate && Date.now() > deadline - timeoutMs + 15000) {
      kickedTessellate = true;
      await page.evaluate(() => {
        const host = document.querySelector(".semio-node-graph-host") as HTMLElement | null;
        host?.dispatchEvent(new CustomEvent("semio:request-preview-tessellate", { bubbles: true }));
      }).catch(() => {});
    }
    await page.waitForTimeout(1000);
  }
  return last;
}

const browser = await chromium.launch({ headless: true });
let page = await openPage(browser);
const results: any[] = [];

for (const exampleId of examples) {
  if (exampleId === "rectangle-wire-preview" || exampleId === "box-shell-preview") {
    console.log("[DEBUG] fresh page before fragile example", exampleId);
    await page.close().catch(() => {});
    page = await openPage(browser);
  }
  console.log("[DEBUG] example start", exampleId);
  let howInfo = await selectExample(page, exampleId);
  if (howInfo.needReload || howInfo.how === "failed-select") {
    console.log("[DEBUG] reloading with fixture", exampleId, howInfo.how);
    await page.close().catch(() => {});
    page = await openPage(browser, exampleId);
    howInfo = await selectExample(page, exampleId);
  }
  await page.waitForTimeout(2000);
  let settled = await settle(page, exampleId);
  const settleBad = settled?.destroyed || !(settled?.trigger === labels[exampleId] && settled?.keys?.includes(expectedKey[exampleId]!));
  if (settleBad) {
    console.log("[DEBUG] settle mismatch/destroyed; reload with fixture", exampleId, settled?.trigger, settled?.keys);
    await page.close().catch(() => {});
    page = await openPage(browser, exampleId);
    howInfo = await selectExample(page, exampleId);
    await page.waitForTimeout(2000);
    settled = await settle(page, exampleId);
  }
  const shot = path.join(outDir, exampleId + ".png");
  try { await page.screenshot({ path: shot, fullPage: false, timeout: 8000 }); } catch {}
  results.push({ exampleId, how: howInfo.how, snap: settled, shot });
  console.log("[DEBUG] example done", JSON.stringify({
    exampleId, how: howInfo.how, allOk: settled?.allOk, meshCount: settled?.meshCount, keys: settled?.keys,
    blocked: settled?.blocked, errors: settled?.errors, errorDetails: settled?.errorDetails,
    graphH: settled?.graphH, trigger: settled?.trigger, meshEdges: settled?.meshEdges, meshPositions: settled?.meshPositions,
  }));
}

await writeFile(path.join(ticketDir, "example-shots-report.json"), JSON.stringify({ results }, null, 2));
await browser.close();
console.log("[DEBUG] report written");
