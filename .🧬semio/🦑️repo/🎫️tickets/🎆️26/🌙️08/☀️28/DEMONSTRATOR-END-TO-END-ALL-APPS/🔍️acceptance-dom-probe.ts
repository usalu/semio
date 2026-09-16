/** 🔍️ Read-only DOM probe for the demonstrator acceptance failures: dumps each pane's window
 * containers, the surface host classes they actually carry, and the `data-*-json` attributes the
 * spec grades — so the spec can be matched against the REAL contract instead of a stale memory. */
import { chromium } from "playwright";

const BASE = process.env.PROBE_BASE_URL ?? "http://127.0.0.1:6029/";
const PANE = process.env.PROBE_PANE ?? "aussuchen";
const WAIT_MS = Number(process.env.PROBE_WAIT_MS ?? 60_000);

const browser = await chromium.launch({ args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const errors: string[] = [];
page.on("console", (m) => { if (m.type() === "error") errors.push(m.text()); });
page.on("pageerror", (e) => errors.push("PAGEERROR " + String(e)));
await page.goto(`${BASE}#${PANE}`, { waitUntil: "domcontentloaded", timeout: 120_000 });
await page.waitForFunction(
  (id) => {
    const el = document.querySelector(`[data-shell-id="${id}"]`) as HTMLElement | null;
    return !!el && (el.dataset.shellReady !== undefined || el.dataset.shellError !== undefined || el.dataset.shellNotFound !== undefined);
  },
  PANE,
  { timeout: WAIT_MS },
).catch(() => console.log("shell beacon never resolved"));
await page.waitForTimeout(Number(process.env.PROBE_SETTLE_MS ?? 20_000));

const report = await page.evaluate((id) => {
  const shell = document.querySelector(`[data-shell-id="${id}"]`) as HTMLElement | null;
  if (!shell) return { shell: null } as unknown as Record<string, unknown>;
  const windows = [...shell.querySelectorAll<HTMLElement>('[id^="framework.window."], [data-element-alias]')].map((el) => ({
    id: el.id || null,
    alias: el.getAttribute("data-element-alias"),
    visible: el.getBoundingClientRect().width > 0 && el.getBoundingClientRect().height > 0,
    hostClasses: [...el.querySelectorAll<HTMLElement>('[class*="semio-"]')].map((h) => [...h.classList].filter((c) => c.startsWith("semio-")).join(".")).filter(Boolean).slice(0, 8),
    dataAttrs: [...el.querySelectorAll<HTMLElement>("*")].flatMap((h) => [...h.attributes].filter((a) => a.name.startsWith("data-") && a.name.endsWith("-json")).map((a) => `${a.name}=${a.value.length}B`)).slice(0, 12),
    text: (el.innerText || "").replace(/\s+/g, " ").slice(0, 160),
  }));
  return { shellReady: shell.dataset.shellReady ?? null, shellError: shell.dataset.shellError ?? null, windows };
}, PANE);

console.log(JSON.stringify(report, null, 2));
console.log("CONSOLE ERRORS (" + errors.length + "):");
for (const e of [...new Set(errors)].slice(0, 30)) console.log("  -", e.slice(0, 300));
await browser.close();
