/** 🧰️ C10 shared Playwright helpers for two humans collaborating in the React `s` shell over one hub. */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { createHash } from "node:crypto";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

export const OUT = "/Users/ueli/Documents/semio/.tmp-ticket/wp-c11/generated";
mkdirSync(OUT, { recursive: true });

export const USERS = [
  { label: "user1", email: "user1@semio.dev", password: "gm1-local-dev-pass-1" },
  { label: "user2", email: "user2@semio.dev", password: "gm1-local-dev-pass-2" },
];

const t0 = Date.now();
export const ms = () => Date.now() - t0;

/** 🧾️ Step recorder writing `<tag>-run.txt` and `<tag>-report.json` after every step. */
export function recorder(tag, sessions) {
  const report = { tag, startedAt: new Date().toISOString(), steps: [] };
  const save = () => {
    writeFileSync(join(OUT, `${tag}-report.json`), JSON.stringify(report, null, 2));
    writeFileSync(join(OUT, `${tag}-console.txt`), sessions.flatMap((s) => s.lines.map((line) => `${s.user.label} ${line}`)).join("\n"));
  };
  const record = (step, pass, detail) => {
    const verdict = pass === null ? "SKIP" : pass ? "PASS" : "FAIL";
    const text = typeof detail === "string" ? detail : JSON.stringify(detail);
    report.steps.push({ step, verdict, at: ms(), detail });
    const line = `STEP ${step}: ${verdict} — ${text}`;
    console.log(line.slice(0, 4000));
    save();
  };
  return { report, record, save };
}

/** 🌐️ Two independent browser contexts, one per human, each on its own serve; `locale` seats the browser language the
 * shell boots in, `users` overrides the humans (their hub's own credentials). */
export async function openSessions(urls, { locale = "en-US", users = USERS } = {}) {
  const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
  const sessions = [];
  for (const [index, user] of users.entries()) {
    const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, locale });
    const page = await context.newPage();
    const lines = [];
    const sockets = [];
    page.on("console", (message) => {
      const text = message.text();
      if (/Download the React DevTools|\[vite\]|agent-bridge|status of 404/.test(text)) return;
      lines.push(`${ms()} ${message.type()} ${text.slice(0, 900)}`);
    });
    page.on("pageerror", (error) => lines.push(`${ms()} pageerror ${String(error).slice(0, 900)}`));
    page.on("response", (response) => {
      const url = response.url();
      if (/_semio\/hub\/(?!auth\/sessions\/me)|:7800\/(?!auth\/sessions\/me)/.test(url)) lines.push(`${ms()} http ${response.status()} ${response.request().method()} ${url.replace(/^https?:\/\/[^/]+/, "").slice(0, 160)}`);
    });
    page.on("websocket", (ws) => {
      const row = { url: ws.url(), openedAt: ms(), closedAt: null, received: 0 };
      sockets.push(row);
      ws.on("framereceived", () => (row.received += 1));
      ws.on("close", () => {
        row.closedAt = ms();
        lines.push(`${ms()} ws-closed ${ws.url().replace(/^wss?:\/\/[^/]+/, "").slice(0, 160)}`);
      });
    });
    sessions.push({ user, url: urls[index], context, page, lines, sockets });
  }
  return { browser, sessions };
}

export async function boot(session) {
  await session.page.goto(session.url, { waitUntil: "domcontentloaded", timeout: 180_000 });
  await session.page.locator('[data-ui-node-key="s-home-create-space"]').first().waitFor({ state: "attached", timeout: 180_000 });
}

export async function signIn(session) {
  const page = session.page;
  await page.locator('[data-semio-hub-sign-in=""]').first().click();
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 30_000 });
  await form.locator('input[type="email"]').fill(session.user.email);
  await form.locator('input[type="password"]').fill(session.user.password);
  await form.locator('[id="os.hub.signIn.submit"]').click();
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60_000 });
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click();
  await page.locator("[data-semio-hub-workspace]").waitFor({ state: "hidden", timeout: 15_000 });
}

/** ⌨️ Keyboard activation of a UI node by its stable `data-ui-node-key` (window-scoped DOM ids). */
export async function activate(page, key) {
  const node = page.locator(`[data-ui-node-key="${key}"]`).first();
  await node.waitFor({ state: "attached", timeout: 30_000 });
  await node.focus();
  await node.press("Enter");
}

export const dialog = (page) => page.locator('[role="dialog"][data-slot="dialog-content"]');

export async function selectOption(page, triggerId, option) {
  await page.locator(`[id="${triggerId}"]`).click();
  const matcher = typeof option === "string" ? { name: option, exact: true } : { name: option };
  await page.getByRole("option", matcher).first().click();
  await page.waitForTimeout(200);
}

/** 🗂️ Picks an artifact-kind option by its encoded choice (`data-value` carries the kind id), since the hub's creation
 * catalog labels every kind with its surface role ("Editor") rather than the kind's name. */
export async function selectKind(page, kindId) {
  await page.locator('[id="kindChoice"]').click();
  await page.locator(`[role="option"][data-value*='"kindId":"${kindId}"']`).first().click();
  await page.waitForTimeout(200);
}

export async function listOptions(page, triggerId) {
  await page.locator(`[id="${triggerId}"]`).click();
  await page.waitForTimeout(600);
  const options = await page.getByRole("option").allInnerTexts();
  await page.keyboard.press("Escape");
  await page.waitForTimeout(300);
  return options;
}

export async function submitDialog(page) {
  await page.locator('[id="ui.dialog.submit"]').click();
  await dialog(page).waitFor({ state: "hidden", timeout: 20_000 });
}

export async function rowIds(page, prefix) {
  return new Set(await page.locator(`[data-ui-node-key^="${prefix}:"]`).evaluateAll((els) => els.map((el) => el.getAttribute("data-ui-node-key"))));
}

export async function waitNewRow(page, prefix, before, deadlineMs) {
  const deadline = Date.now() + deadlineMs;
  while (Date.now() < deadline) {
    for (const id of await rowIds(page, prefix)) if (!before.has(id)) return id.slice(prefix.length + 1);
    await page.waitForTimeout(400);
  }
  throw new Error(`no new ${prefix} row within ${deadlineMs} ms`);
}

/** 🧭️ Waits for one row of a WINDOWED table (only rows inside its scroll window exist in the DOM): checks the rendered rows,
 * then pages every `table-window-scroll` container top to bottom, as a human scrolls; leaves the table on the row. */
export async function waitRow(page, prefix, id, deadlineMs) {
  const row = page.locator(`[data-ui-node-key="${prefix}:${id}"]`).first();
  const deadline = Date.now() + deadlineMs;
  while (Date.now() < deadline) {
    if ((await row.count()) > 0) return;
    const scrollers = page.locator('[data-slot="table-window-scroll"]');
    for (let index = 0, count = await scrollers.count(); index < count && (await row.count()) === 0; index += 1) {
      for (let top = 0; ; ) {
        const metrics = await scrollers.nth(index).evaluate((element, y) => { element.scrollTop = y; return { top: element.scrollTop, scrollHeight: element.scrollHeight, clientHeight: element.clientHeight }; }, top).catch(() => null);
        await page.waitForTimeout(400);
        if (metrics === null || (await row.count()) > 0 || metrics.top + metrics.clientHeight >= metrics.scrollHeight) break;
        top = metrics.top + Math.max(1, Math.floor(metrics.clientHeight * 0.8));
      }
    }
    if ((await row.count()) > 0) return;
    await page.waitForTimeout(500);
  }
  throw new Error(`timeout waiting for [data-ui-node-key="${prefix}:${id}"] (${deadlineMs} ms, windowed tables paged)`);
}

/** 🔎️ Finds the row whose text contains `name` in a WINDOWED table (pages every `table-window-scroll` top to bottom, as a
 * human scrolls); answers its id (the key after `<prefix>:`) or throws after `deadlineMs`. */
export async function waitNamedRow(page, prefix, name, deadlineMs) {
  const find = () => page.locator(`[data-ui-node-key^="${prefix}:"]`).evaluateAll((elements, wanted) => elements.find((element) => (element.textContent ?? "").includes(wanted))?.getAttribute("data-ui-node-key") ?? null, name);
  const deadline = Date.now() + deadlineMs;
  while (Date.now() < deadline) {
    let key = await find();
    const scrollers = page.locator('[data-slot="table-window-scroll"]');
    for (let index = 0, count = await scrollers.count(); index < count && key === null; index += 1) {
      for (let top = 0; ; ) {
        const metrics = await scrollers.nth(index).evaluate((element, y) => { element.scrollTop = y; return { top: element.scrollTop, scrollHeight: element.scrollHeight, clientHeight: element.clientHeight }; }, top).catch(() => null);
        await page.waitForTimeout(400);
        key = await find();
        if (metrics === null || key !== null || metrics.top + metrics.clientHeight >= metrics.scrollHeight) break;
        top = metrics.top + Math.max(1, Math.floor(metrics.clientHeight * 0.8));
      }
    }
    if (key !== null) return key.slice(prefix.length + 1);
    await page.waitForTimeout(500);
  }
  throw new Error(`no ${prefix} row named ${JSON.stringify(name)} within ${deadlineMs} ms (windowed tables paged)`);
}

/** 🎛️ The labelled buttons of one table row (row actions render as labelled buttons). */
export async function rowActions(page, prefix, id) {
  return page.locator(`[data-ui-node-key="${prefix}:${id}"] button`).evaluateAll((els) => els.map((el) => ({ label: el.getAttribute("aria-label") ?? "", title: el.getAttribute("title") ?? "", text: (el.textContent ?? "").trim().slice(0, 40), key: el.getAttribute("data-ui-node-key") ?? "" })));
}

export async function clickRowAction(page, prefix, id, pattern) {
  const buttons = page.locator(`[data-ui-node-key="${prefix}:${id}"] button`);
  const count = await buttons.count();
  for (let index = 0; index < count; index += 1) {
    const button = buttons.nth(index);
    const name = `${(await button.getAttribute("aria-label")) ?? ""} ${(await button.getAttribute("title")) ?? ""} ${(await button.textContent()) ?? ""}`;
    if (pattern.test(name)) {
      await button.focus();
      await button.press("Enter");
      return name.trim();
    }
  }
  throw new Error(`row ${prefix}:${id} has no action matching ${pattern} (${JSON.stringify(await rowActions(page, prefix, id))})`);
}

/** 🪞️ Everything the shell publishes about sync, presence, history and the open document. */
export async function read(page) {
  return page.evaluate(() => {
    const text = (el) => (el?.innerText ?? "").replace(/\s+/g, " ").trim();
    const peerNodes = [...document.querySelectorAll('[id="s-presence-peers"] [data-row-id^="peer:"]:not([data-row-id="peer:overflow"])')];
    return {
      ready: document.documentElement.getAttribute("data-semio-os-ready"),
      error: document.documentElement.getAttribute("data-semio-os-error"),
      url: location.pathname,
      ledger: [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((el) => !el.id.endsWith(".revert")).map((el) => text(el).slice(0, 96)),
      checkin: text(document.querySelector('[id="s-checkin"]')),
      syncPill: text(document.querySelector('[id="s-sync-status"]')),
      executionTarget: [...document.querySelectorAll("[data-semio-execution-target-status]")].map((el) => `${el.getAttribute("data-semio-execution-target-status")}:${text(el).slice(0, 80)}`),
      diagnostics: [...document.querySelectorAll("[data-semio-execution-target-diagnostic]")].map((el) => el.getAttribute("data-semio-execution-target-diagnostic")?.slice(0, 200)),
      peers: peerNodes.map((el) => el.getAttribute("data-row-id")),
      peerLabels: peerNodes.map((el) => text(el).slice(0, 64)),
      peerColors: peerNodes.map((el) => getComputedStyle(el.firstElementChild ?? el).borderTopColor),
      peerCursors: document.querySelectorAll('[data-peer-cursor], [data-testid="peer-cursor"]').length,
      windows: [...document.querySelectorAll('[data-slot="window"]')].map((el) => el.id).slice(0, 12),
      notices: [...document.querySelectorAll('[data-slot="toast"], [role="status"], [role="alert"]')].map((el) => text(el).slice(0, 160)).filter(Boolean).slice(0, 8),
    };
  });
}

export async function canvasHash(page) {
  const canvas = page.locator("canvas").first();
  if (!(await canvas.count())) return "no-canvas";
  return canvas
    .screenshot({ timeout: 15_000 })
    .then((buffer) => `${createHash("sha256").update(buffer).digest("hex").slice(0, 16)}:${buffer.length}`)
    .catch((error) => `canvas-error ${String(error).split("\n")[0].slice(0, 60)}`);
}

export async function shot(session, tag, name) {
  await session.page.screenshot({ path: join(OUT, `${tag}-${name}-${session.user.label}.png`) }).catch(() => {});
}
