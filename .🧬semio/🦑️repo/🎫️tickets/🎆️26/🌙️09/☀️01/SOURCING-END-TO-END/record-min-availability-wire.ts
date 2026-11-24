import { mkdir, writeFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { join } from "node:path";

/** 🎙️ Records the DOM events and outgoing wire payloads of the Minimum availability fill + Enter (sourcing, 6081).
 * Usage: `repo record-min-availability-wire.ts [single|journey]`; writes `🗑️generated/min-availability-wire-<scenario>.json`.
 * @see ./🔎️browser-min-availability-wire.md */
const root = process.cwd();
const osDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
const url = process.env.SOURCING_URL ?? "http://127.0.0.1:6081/";
const out = join(import.meta.dir, "🗑️generated");
const { ensureRepositoryBrowserPathV1 } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧰️preparation/🟦️.ts"));
const { awaitBeacon, dismissIntroduction } = await import(join(root, osDev, "🧪️tests/🧮️program-matrix/🟦️.ts"));
const require = createRequire(join(root, osDev, "📦️packages/🟦️typescript/package.json"));
ensureRepositoryBrowserPathV1(root, process.env);
const { chromium } = require("playwright") as typeof import("playwright");
await mkdir(out, { recursive: true });
const browser = await chromium.launch();
const context = await browser.newContext({ locale: "en-US", viewport: { width: 1800, height: 1100 } });
const page = await context.newPage();
const consoleLines: string[] = [];
const clock = Date.now();
page.on("console", m => consoleLines.push(`+${Date.now() - clock}ms ${m.type()}: ${m.text()}`.slice(0, 500)));
page.on("pageerror", e => consoleLines.push(`pageerror: ${e.message}`.slice(0, 500)));
await page.addInitScript(() => {
  const w = window as any;
  w.__wire = [];
  const t0 = performance.now();
  const note = (kind: string, data: unknown) => {
    let text: string;
    try { text = typeof data === "string" ? data : data instanceof ArrayBuffer || ArrayBuffer.isView(data) ? `[binary ${(data as ArrayBuffer).byteLength}] ${new TextDecoder().decode(data as ArrayBuffer).slice(0, 600)}` : JSON.stringify(data); } catch { text = String(data); }
    if (/gesture|abort/u.test(text) || kind === "dom") w.__wire.push({ t: Math.round(performance.now() - t0), kind, text: text.slice(0, 600) });
  };
  w.__note = note;
  for (const proto of [Worker.prototype, MessagePort.prototype, window as any]) {
    const original = proto.postMessage; if (!original) continue;
    proto.postMessage = function (this: unknown, ...args: unknown[]) { note("postMessage", args[0]); return original.apply(this, args as never); };
  }
  const stringify = JSON.stringify;
  JSON.stringify = function (this: unknown, ...args: [unknown]) { const text = stringify.apply(this, args as never); if (typeof text === "string" && /"gesture"|"abort"/u.test(text)) note("JSON.stringify", text); return text; } as typeof JSON.stringify;
  for (const type of ["focusin", "focusout", "input", "change", "keydown", "keyup", "beforeinput", "pointerdown", "pointerup", "click"]) {
    document.addEventListener(type, event => {
      const target = event.target as HTMLElement | null;
      if (!target || target.getAttribute?.("role") !== "spinbutton" && !(target as HTMLInputElement).type?.includes("number")) return;
      note("dom", `${type} key=${(event as KeyboardEvent).key ?? ""} value=${(target as HTMLInputElement).value} label=${target.getAttribute("aria-label")}`);
    }, true);
  }
});
await page.goto(url);
await awaitBeacon(page, Date.now() + 300_000);
await dismissIntroduction(page);
await page.waitForTimeout(3000);
const pool = page.getByRole("tabpanel", { name: "Pool", exact: true });
const names = () => pool.locator("tbody tr").evaluateAll(rows => rows.map(row => row.querySelector("td")?.textContent?.replace(/^\s*(Drag to window|Verschieben)\s*/u, "").trim() ?? ""));
const mark = (label: string) => page.evaluate(label => (window as any).__wire.push({ t: -1, kind: "mark", text: label }), label);
const input = pool.getByRole("spinbutton", { name: "Minimum availability", exact: true });
const before = await names();
const scenario = process.argv[2] ?? "single";
await page.evaluate(() => (window as any).__wire.push({ t: -1, kind: "mark", text: "scenario start" }));
consoleLines.push(`+${Date.now() - clock}ms MARK scenario start`);
if (scenario === "tick") {
  await input.fill("25"); consoleLines.push(`+${Date.now() - clock}ms MARK filled`);
  await page.waitForTimeout(9000); consoleLines.push(`+${Date.now() - clock}ms MARK held 9s`);
  await writeFile(join(out, "min-availability-wire-tick.json"), JSON.stringify({ wire: await page.evaluate(() => (window as any).__wire), console: consoleLines }, null, 2));
  console.log(consoleLines.filter(l => !/GL Driver/.test(l)).join("\n"));
  await browser.close(); process.exit(0);
}
if (scenario === "baseline") consoleLines.push(`+${Date.now() - clock}ms MARK baseline sort`);
const firstColumn = () => pool.locator("tbody tr td:nth-child(4)").allInnerTexts();
await mark("fill 25");
if (scenario === "quick") await input.fill("25");
else if (scenario !== "baseline") { await input.fill("25");
await page.waitForTimeout(500);
await mark("press Enter");
await input.press("Enter");
await page.waitForTimeout(4000); }
const afterEnter = await names();
if (scenario === "journey") {
  await mark("fill 0");
  await input.fill("0");
  await input.press("Enter");
  await page.waitForTimeout(1500);
}
await mark("click Availability header sort button (blur)");
const sortBefore = await firstColumn();
await pool.getByRole("columnheader", { name: "Availability", exact: true }).getByRole("button").click().catch(() => undefined);
await page.waitForTimeout(4000);
const afterBlur = await names();
const sortAfter = await firstColumn();
await mark("second sort click");
await pool.getByRole("columnheader", { name: "Availability", exact: true }).getByRole("button").click().catch(() => undefined);
await page.waitForTimeout(3000);
const sortAfter2 = await firstColumn();
const wire = await page.evaluate(() => (window as any).__wire);
await writeFile(join(out, `min-availability-wire-${scenario}.json`), JSON.stringify({ scenario, before, afterEnter, afterBlur, sortBefore, sortAfter, sortAfter2, wire, console: consoleLines }, null, 2));
console.log(consoleLines.filter(l => !/GL Driver|ladder zero|live cleanup zero/.test(l)).join("\n").slice(0, 3000));
console.log(JSON.stringify({ before: before.length, afterEnter: afterEnter.length, afterBlur: afterBlur.length, sortBefore, sortAfter, sortAfter2 }), wire.map((w: any) => `${w.t} ${w.kind} ${w.text}`).join("\n"), consoleLines.slice(-10).join("\n"));
await browser.close();
