/** 🧬 Generate-mode probe: boot → converge → ⌘⌥→ → open the Generations window's Actions menu → Add generation → wait for the generate preview meshes. */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "generate-mode");
mkdirSync(outDir, { recursive: true });
const lines = []; const t0 = Date.now();
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 600)}`));
const hosts = () => page.evaluate(() => [...document.querySelectorAll("[data-status-json]")].map((el) => { let m = 0; try { m = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]").length; } catch {} let st = null; try { st = JSON.parse(el.getAttribute("data-status-json")); } catch {} return { id: el.getAttribute("data-surface-id"), meshes: m, phase: st?.phase, ratio: st?.progress?.ratio, hint: st?.hint }; }));
const waitFor = async (label, pred, seconds) => { let h = null; for (let i = 0; i < seconds; i++) { await page.waitForTimeout(1000); h = await hosts(); if (pred(h)) break; } console.log(`[DEBUG] ${label} t=${Date.now() - t0} ${JSON.stringify(h)}`); await page.screenshot({ path: join(outDir, `${label}.png`) }); return h; };
await page.goto(url, { waitUntil: "domcontentloaded" });
await waitFor("boot", (h) => h.some((x) => x.id === "window:procedural-preview" && x.meshes > 0), 120);
await page.keyboard.press("Meta+Alt+ArrowRight");
await waitFor("generate-mode", (h) => h.some((x) => x.id === "window:generation3d-generate-preview"), 20);
const addRow = page.locator('[data-surface-id="window:generation3d-generations"] :text-is("Add Generation"), [id^="window:generation3d-generations"] :text-is("Add Generation"), :text-is("Add Generation")').first();
let clicked = false;
if (await addRow.count()) { await addRow.click({ timeout: 4000 }); clicked = true; console.log("[DEBUG] clicked Add Generation tree row"); }
else console.log("[DEBUG] Add Generation row not found");
await waitFor("generate-added", (h) => h.some((x) => x.id === "window:generation3d-generate-preview" && x.meshes > 0), 120);
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("DONE");
await browser.close();
