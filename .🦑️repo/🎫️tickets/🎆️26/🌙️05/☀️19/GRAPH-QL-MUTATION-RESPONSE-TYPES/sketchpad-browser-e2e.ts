/**
 * @emoji 🌐️ Browser E2E: sketchpad preview + WASM session + live subscription loop.
 * Prereq: `bun nx run @semio-tech/compose-sketchpad:build` (serves dist on 4181).
 * Run: `bun .repo/🎫️/26/05/19/GRAPH-QL-MUTATION-RESPONSE-TYPES/sketchpad-browser-e2e.ts`
 */
import { spawn } from "node:child_process";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const here = fileURLToPath(new URL(".", import.meta.url));
const repoRoot = resolve(here, "../../../../../..");
const sketchpadDir = resolve(repoRoot, "compose/client/lib/sketchpad/react");
const baseURL = process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:4181";

let preview: ReturnType<typeof spawn> | null = null;
if (!process.env.PLAYWRIGHT_BASE_URL) {
  const viteBin = resolve(repoRoot, "node_modules/vite/bin/vite.js");
  preview = spawn(process.execPath, [viteBin, "preview", "--port", "4181", "--host", "127.0.0.1"], {
    cwd: sketchpadDir,
    stdio: "pipe",
    env: { ...process.env, NODE_OPTIONS: "" },
  });
}

async function waitForServer(url: string, ms = 120_000): Promise<void> {
  const started = Date.now();
  while (Date.now() - started < ms) {
    try {
      const r = await fetch(url);
      if (r.ok) return;
    } catch {
      /* retry */
    }
    await new Promise((r) => setTimeout(r, 500));
  }
  throw new Error(`server not ready: ${url}`);
}

try {
  await waitForServer(baseURL);
  const browser = await chromium.launch({
    headless: true,
    channel: process.env.PW_CHANNEL ?? "msedge",
    timeout: 120_000,
  });
  const page = await browser.newPage();
  const errors: string[] = [];
  page.on("console", (m) => {
    if (m.type() === "error") errors.push(m.text());
  });

  console.log("[DEBUG] sketchpad-browser-e2e: goto", baseURL);
  await page.goto(baseURL, { waitUntil: "networkidle", timeout: 180_000 });
  await page.locator("#compose\\.sketchpad\\.navbar").first().waitFor({ state: "visible", timeout: 180_000 });

  await page.waitForFunction(
    () => {
      const reg = (window as { __COMPOSE_KIT_REGISTRY__?: () => { list: () => string[]; get: (id: string) => { session?: unknown; jsStoreId?: string } | null } }).__COMPOSE_KIT_REGISTRY__;
      return reg != null && reg().list().length > 0;
    },
    { timeout: 180_000 },
  );

  const wiring = await page.evaluate(async () => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const reg = (window as any).__COMPOSE_KIT_REGISTRY__?.();
    if (reg == null) return { ok: false, reason: "no-registry" };
    const kid = reg.list()[0];
    const row = kid == null ? null : reg.get(kid);
    const session = row?.session;
    const jsStoreId = row?.jsStoreId;
    if (session == null || jsStoreId == null || jsStoreId === "") {
      return { ok: false, reason: "no-session", kid };
    }
    const gqlLoopRunning = session.gqlLoopRunning === true;
    const kit = await session.store(jsStoreId).wip().theKit().kit();
    const before = await kit.name();
    const renamed = await kit.rename("browser-e2e-renamed");
    if (!renamed.ok) return { ok: false, reason: "rename-failed", gqlLoopRunning };
    await new Promise((r) => setTimeout(r, 500));
    const after = await kit.name();
    return { ok: true, gqlLoopRunning, kid, before, after };
  });
  if (!wiring.ok) throw new Error(`registry/session e2e failed: ${JSON.stringify(wiring)}`);
  if (!wiring.gqlLoopRunning) throw new Error("browser session did not start live subscription loop");
  if (wiring.after !== "browser-e2e-renamed") throw new Error(`rename did not persist in browser: ${JSON.stringify(wiring)}`);

  await page.locator("#compose\\.sketchpad\\.footer\\.alternative\\.select").first().waitFor({ state: "visible", timeout: 120_000 });

  console.log("[DEBUG] sketchpad-browser-e2e: PASS", wiring);
  await browser.close();
  if (errors.length) console.log("[DEBUG] sketchpad-browser-e2e: console errors", errors.slice(0, 5));
} finally {
  preview?.kill();
}
