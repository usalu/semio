#!/usr/bin/env bun
import { chromium } from "playwright";
import { writeFile, mkdir } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = path.dirname(fileURLToPath(import.meta.url));
const outDir = path.join(ticketDir, "example-shots");
await mkdir(outDir, { recursive: true });

async function dismissOverlays(page) {
  for (let i = 0; i < 12; i++) {
    const closed = await page.evaluate(() => {
      const veils = Array.from(document.querySelectorAll(".ui-veil")) as HTMLElement[];
      let hit = false;
      for (const veil of veils) {
        if (getComputedStyle(veil).pointerEvents === "none") continue;
        veil.style.pointerEvents = "none";
        veil.click();
        hit = true;
      }
      const btn = Array.from(document.querySelectorAll("button")).find((el) =>
        /^(Überspringen|Skip|Weiter|Next|Fertig|Done|Schließen|Close)$/i.test((el.textContent || "").trim()),
      );
      if (btn) {
        (btn as HTMLElement).click();
        return (btn.textContent || "").trim();
      }
      return hit ? "veil-disabled" : null;
    });
    if (!closed) break;
    await page.waitForTimeout(250);
  }
}

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("pageerror", (err) => console.log("[DEBUG] pageerror", String(err).slice(0, 200)));
await page.addInitScript(() => {
  try {
    localStorage.clear();
    sessionStorage.clear();
  } catch {}
});
const url = "http://127.0.0.1:6029/?bust=" + Date.now() + "#generator";
console.log("[DEBUG] goto", url);
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 240000 });
await page.waitForTimeout(12000);
await dismissOverlays(page);
await page.waitForFunction(() => /semio · procedural · 3d/i.test(document.body?.innerText || ""), null, { timeout: 60000 });
const modeClicked = await page.evaluate(() => {
  const candidates = Array.from(document.querySelectorAll("button, [role=tab]")) as HTMLElement[];
  const gen = candidates.find((el) => /^(Generieren|Generate)$/i.test((el.textContent || "").trim()));
  if (gen) {
    gen.click();
    return (gen.textContent || "").trim();
  }
  return "no-mode";
});
console.log("[DEBUG] mode", modeClicked);
await page.waitForTimeout(6000);
await dismissOverlays(page);
const addGen = await page.evaluate(() => {
  const row = document.getElementById("procedural3d-play-generate.add-generation") as HTMLElement | null;
  if (!row) return "missing-row";
  row.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true, view: window }));
  return "clicked-id";
});
console.log("[DEBUG] addGeneration", addGen);
await page.waitForTimeout(8000);
const snap = await page.evaluate(() => {
  const world = document.querySelector(".semio-world-3d-host");
  const meshesRaw = world?.getAttribute("data-meshes-json") || "[]";
  let meshes: any[] = [];
  try {
    meshes = JSON.parse(meshesRaw);
  } catch {}
  const text = document.body?.innerText || "";
  return {
    hasWorld: !!world,
    meshCount: meshes.length,
    meshEdges: meshes.map((m: any) => m.data?.edgePositions?.length ?? m.data?.edge_positions?.length ?? 0),
    meshPositions: meshes.map((m: any) => m.data?.positions?.length ?? 0),
    layoutErr: (text.match(/Invalid \d+ panel layout:[^\n]*/)||[])[0] || null,
    hasGenerations: /GENERATIONS|Generationen/i.test(text),
    hasForm: /Formular|HEIGHT|Inputs/i.test(text),
    hasPreview: /Vorschau|Preview/i.test(text),
    hasGeneration1: /Generation 1/i.test(text),
    hasHeight: /HEIGHT|Höhe/i.test(text),
    generateRenderfehler: /Vorschau[\s\S]{0,200}Renderfehler/i.test(text),
    bodyText: text.slice(0, 500),
  };
});
console.log("[DEBUG] snap", JSON.stringify(snap));
await page.screenshot({ path: path.join(outDir, "demonstrator-generator.png"), fullPage: false }).catch(() => {});
await writeFile(path.join(ticketDir, "demonstrator-generator-report.json"), JSON.stringify({ modeClicked, addGen, snap }, null, 2));
await browser.close();
console.log("[DEBUG] report written");
