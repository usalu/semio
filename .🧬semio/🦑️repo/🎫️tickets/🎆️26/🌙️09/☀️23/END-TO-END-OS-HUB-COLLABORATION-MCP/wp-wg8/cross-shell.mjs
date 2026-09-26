/** 🤝️ WG8 — the React half of the cross-shell journey: user B in a real browser on a React `s` serve, one hub document
 * with the native wgpu user A (the Rust law `a_native_and_a_react_user_collaborate_on_one_hub_document`). Both sides step
 * through the file handshake in `DIR` (`native-<step>.json` ← A, `react-<step>.json` ← B).
 * Usage: bun cross-shell.mjs <reactUrl> <handshakeDir> <captureDir>   Env: CROSS_B_EMAIL, CROSS_B_PASSWORD */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { existsSync, mkdirSync, readFileSync, renameSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const URL = process.argv[2] ?? "http://127.0.0.1:6590/";
const DIR = process.argv[3];
const OUT = process.argv[4];
mkdirSync(DIR, { recursive: true });
mkdirSync(OUT, { recursive: true });
const USER = { email: process.env.CROSS_B_EMAIL ?? "user2@semio.dev", password: process.env.CROSS_B_PASSWORD ?? "gm1-local-dev-pass-2" };
const MODE = process.env.CROSS_MODE ?? "edits";
const t0 = Date.now();
const ms = () => Date.now() - t0;
const report = { url: URL, startedAt: new Date().toISOString(), steps: [] };
const lines = [];
const sockets = [];
const save = () => {
  writeFileSync(join(OUT, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(OUT, "console.txt"), lines.join("\n"));
};
const record = (step, detail) => {
  report.steps.push({ step, at: ms(), detail });
  console.log(`REACT ${step} ${JSON.stringify(detail).slice(0, 1500)}`);
  save();
};
const publish = (step, value) => {
  const staged = join(DIR, `.react-${step}.json`);
  writeFileSync(staged, JSON.stringify(value, null, 2));
  renameSync(staged, join(DIR, `react-${step}.json`));
  record(`publish ${step}`, value);
};
const awaitNative = async (step, budgetMs) => {
  const path = join(DIR, `native-${step}.json`);
  const started = Date.now();
  while (Date.now() - started < budgetMs) {
    if (existsSync(path)) {
      const value = JSON.parse(readFileSync(path, "utf8"));
      record(`native ${step}`, value);
      return value;
    }
    await new Promise((resolve) => setTimeout(resolve, 200));
  }
  record(`native ${step} missing`, { budgetMs });
  return null;
};

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--ignore-gpu-blocklist", "--enable-unsafe-webgpu"] });
const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
const page = await context.newPage();
page.on("console", (message) => {
  const text = message.text();
  if (/Download the React DevTools|\[vite\]|status of 404|\[stale\]/.test(text)) return;
  lines.push(`${ms()} ${message.type()} ${text.slice(0, 900)}`);
});
page.on("pageerror", (error) => lines.push(`${ms()} pageerror ${String(error).slice(0, 900)}`));
page.on("websocket", (ws) => {
  const row = { url: ws.url().replace(/^wss?:\/\/[^/]+/, "").slice(0, 160), openedAt: ms(), closedAt: null, received: 0 };
  sockets.push(row);
  ws.on("framereceived", () => (row.received += 1));
  ws.on("close", () => (row.closedAt = ms()));
});
const shot = (name) => page.screenshot({ path: join(OUT, `${name}.png`) }).catch(() => {});

/** 🪞️ What the React shell publishes about the document, its sync binding, the roster and the canvas presence. */
const read = () =>
  page.evaluate(() => {
    const text = (el) => (el?.innerText ?? "").replace(/\s+/g, " ").trim();
    const peers = [...document.querySelectorAll('[id="s-presence-peers"] [data-row-id^="peer:"]:not([data-row-id="peer:overflow"])')];
    return {
      ready: document.documentElement.getAttribute("data-semio-os-ready"),
      error: document.documentElement.getAttribute("data-semio-os-error"),
      ledger: [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((el) => !el.id.endsWith(".revert")).map((el) => ({ id: el.id, text: text(el).slice(0, 96), attrs: [...el.attributes].filter((a) => a.name.startsWith("data-")).map((a) => `${a.name}=${a.value}`).join(" ") })),
      syncPill: text(document.querySelector('[id="s-sync-status"]')),
      hub: document.querySelector("[data-semio-hub-connection]")?.getAttribute("data-semio-hub-connection") ?? null,
      peers: peers.map((el) => ({ id: el.getAttribute("data-row-id"), label: text(el).slice(0, 64) })),
      cursors: [...document.querySelectorAll("[data-peer-cursor]")].map((el) => ({ actor: el.getAttribute("data-peer-actor"), path: el.getAttribute("data-ui-path"), color: el.getAttribute("data-peer-color"), label: el.getAttribute("aria-label"), tool: el.getAttribute("data-peer-active-tool"), left: el.style.left, top: el.style.top })),
      viewports: [...document.querySelectorAll("[data-peer-viewport]")].map((el) => ({ actor: el.getAttribute("data-peer-actor"), path: el.getAttribute("data-ui-path") })),
      boards: [...document.querySelectorAll("canvas")].map((el) => { const r = el.getBoundingClientRect(); return { x: r.x, y: r.y, w: r.width, h: r.height, host: el.closest("[data-ui-path]")?.getAttribute("data-ui-path") ?? null }; }),
      actions: [...new Set([...document.querySelectorAll('[id^="action."]')].map((el) => el.id))].slice(0, 60),
      handleKinds: Number((/(\d+) Handle Kinds?/u.exec(document.body.innerText) ?? [])[1] ?? Number.NaN),
      opening: /Verifying document component|Restoring/u.test(document.body.innerText) ? "opening" : null,
      home: document.querySelectorAll('[data-ui-node-key="s-home-create-space"]').length > 0,
      spaceIndex: document.querySelectorAll('[data-ui-node-key="s-space-create-artifact"]').length > 0,
    };
  });

const click = async (selector) => {
  if (!(await page.locator(selector).count())) return "absent";
  return page.locator(selector).first().click({ timeout: 8_000, force: true }).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 120));
};

const until = async (budgetMs, test) => {
  const started = Date.now();
  let last = null;
  while (Date.now() - started < budgetMs) {
    last = await read();
    if (test(last)) return { ms: Date.now() - started, shell: last };
    await page.waitForTimeout(400);
  }
  return { ms: null, shell: last };
};

/** 🎛️ Runs one window action: opens the window's Actions pane when its row is not rendered yet, then activates it. */
async function runAction(actionId) {
  if (!(await page.locator(`[id="action.${actionId}"]`).count())) {
    await page.getByText("Actions", { exact: true }).first().click({ force: true, timeout: 8_000 }).catch(() => {});
    await page.waitForTimeout(800);
  }
  return click(`[id="action.${actionId}"]`);
}

async function rowAction(prefix, id, pattern) {
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
  return `no ${pattern} among ${count} buttons`;
}

async function openDocument(spaceId, documentId) {
  let spaceAction = "already in the space";
  const artifactRow = page.locator(`[data-ui-node-key="artifact:${documentId}"]`).first();
  const spaceRow = page.locator(`[data-ui-node-key="space:${spaceId}"]`).first();
  const landed = await Promise.race([artifactRow.waitFor({ state: "attached", timeout: 180_000 }).then(() => "artifact"), spaceRow.waitFor({ state: "attached", timeout: 180_000 }).then(() => "space")]);
  if (landed === "space") {
    spaceAction = await rowAction("space", spaceId, /open/i);
    await page.locator('[data-ui-node-key="s-space-create-artifact"]').first().waitFor({ state: "visible", timeout: 180_000 });
  }
  await page.locator(`[data-ui-node-key="artifact:${documentId}"]`).first().waitFor({ state: "attached", timeout: 180_000 });
  const artifactAction = await rowAction("artifact", documentId, /open/i);
  const live = await until(240_000, (shell) => shell.syncPill && !/detached|getrennt|connecting|verbindet|backoff/i.test(shell.syncPill) && sockets.some((row) => row.url.includes("/document/ws") && row.closedAt === null));
  const mounted = await until(240_000, (shell) => shell.opening === null && (MODE === "cursors" ? shell.boards.some((box) => box.w > 100 && box.h > 100) : Number.isFinite(shell.handleKinds)));
  const history = await click('[data-slot="panel-tab-button"][id="framework.panel.history"]');
  await page.waitForTimeout(1_000);
  return { spaceAction, artifactAction, history, liveAfterMs: live.ms === null || mounted.ms === null ? null : live.ms + mounted.ms, shell: mounted.shell, sockets: sockets.filter((row) => row.url.includes("/document/ws")) };
}

try {
  await page.goto(URL, { waitUntil: "domcontentloaded", timeout: 180_000 });
  await page.locator('[data-ui-node-key="s-home-create-space"]').first().waitFor({ state: "attached", timeout: 300_000 });
  await page.locator('[data-semio-hub-sign-in=""]').first().click();
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 30_000 });
  await form.locator('input[type="email"]').fill(USER.email);
  await form.locator('input[type="password"]').fill(USER.password);
  await form.locator('[id="os.hub.signIn.submit"]').click();
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60_000 });
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click();
  await page.locator("[data-semio-hub-workspace]").waitFor({ state: "hidden", timeout: 15_000 });
  record("signed-in", await read());

  const nativeOpen = await awaitNative("open", 900_000);
  if (!nativeOpen) throw new Error("the native user never opened its document");
  const opened = await openDocument(nativeOpen.spaceId, nativeOpen.documentId);
  await shot("1-open");
  publish("open", { live: opened.liveAfterMs !== null, ...opened, shell: undefined, syncPill: opened.shell?.syncPill, hub: opened.shell?.hub, boards: opened.shell?.boards, actions: opened.shell?.actions });

  const nativePresence = await awaitNative("presence", 300_000);
  const seen = await until(30_000, (shell) => shell.peers.some((row) => (row.id ?? "").includes(nativeOpen.actor) || (nativeOpen.userId && (row.id ?? "").includes(nativeOpen.userId))));
  publish("presence", { seesNative: seen.ms !== null, afterMs: seen.ms, peers: seen.shell?.peers, nativeSees: nativePresence?.seesReact });

  if (MODE === "cursors") {
    await awaitNative("cursor", 300_000);
    const nativeCursor = await until(20_000, (shell) => shell.cursors.some((cursor) => cursor.actor === nativeOpen.actor));
    await shot("2-native-cursor");
    const board = (nativeCursor.shell?.boards ?? []).filter((box) => box.w > 100 && box.h > 100).sort((a, b) => b.w * b.h - a.w * a.h)[0];
    if (board) {
      for (let step = 0; step < 6; step += 1) {
        await page.mouse.move(board.x + board.w * (0.3 + step * 0.05), board.y + board.h * 0.6);
        await page.waitForTimeout(250);
      }
    }
    publish("cursor", { seesNativeCursor: nativeCursor.ms !== null, afterMs: nativeCursor.ms, cursors: nativeCursor.shell?.cursors, viewports: nativeCursor.shell?.viewports, movedOver: board ?? null });
    for (let step = 0; step < 40; step += 1) {
      await page.mouse.move(board ? board.x + board.w * 0.4 + (step % 5) : 700, board ? board.y + board.h * 0.5 : 450);
      await page.waitForTimeout(250);
      if (existsSync(join(DIR, "native-cursor-seen.json"))) break;
    }
    await awaitNative("cursor-seen", 60_000);
  } else {
    const beforeNativeEdit = (await read()).handleKinds;
    await awaitNative("edit", 300_000);
    const ingested = await until(60_000, (shell) => shell.handleKinds > beforeNativeEdit);
    publish("ingested", { ingested: ingested.ms !== null, afterMs: ingested.ms, handleKinds: [beforeNativeEdit, ingested.shell?.handleKinds] });

    const beforeOwn = (await read()).handleKinds;
    const executed = await runAction("addHandleKind");
    const own = await until(60_000, (shell) => shell.handleKinds > beforeOwn);
    await shot("3-react-edit");
    publish("edit", { ok: own.ms !== null, executed, afterMs: own.ms, handleKinds: [beforeOwn, own.shell?.handleKinds], actions: own.shell?.actions });
    await awaitNative("ingested", 300_000);

    const nativeUndo = await awaitNative("undo", 300_000);
    const afterNativeUndo = await until(30_000, (shell) => shell.handleKinds < own.shell?.handleKinds);
    const beforeUndo = (await read()).handleKinds;
    const undo = await runAction("undo");
    const undone = await until(30_000, (shell) => shell.handleKinds < beforeUndo);
    publish("undo", { ok: undo === "ok", ownReverted: undone.ms !== null, afterMs: undone.ms, sawNativeUndo: afterNativeUndo.ms !== null, nativeUndo: nativeUndo?.ownReverted ?? null, handleKinds: [own.shell?.handleKinds, afterNativeUndo.shell?.handleKinds, beforeUndo, undone.shell?.handleKinds] });

    const socketsBeforeReload = sockets.length;
    await page.reload({ waitUntil: "domcontentloaded", timeout: 180_000 });
    const inPlace = (shell) => Number.isFinite(shell.handleKinds) && shell.opening === null && sockets.slice(socketsBeforeReload).some((row) => row.url.includes("/document/ws") && row.closedAt === null);
    const restored = await until(120_000, (shell) => inPlace(shell) || (shell.ready === "s" && (shell.home || shell.spaceIndex)));
    const restoredInPlace = restored.shell !== null && inPlace(restored.shell);
    const reopened = restoredInPlace ? { liveAfterMs: restored.ms, shell: restored.shell } : await openDocument(nativeOpen.spaceId, nativeOpen.documentId);
    record("reload", { restoredInPlace, afterMs: restored.ms, home: restored.shell?.home, spaceIndex: restored.shell?.spaceIndex });
    await shot("4-reloaded");
    const settledReload = await until(120_000, (shell) => shell.handleKinds === undone.shell?.handleKinds);
    publish("reloaded", { live: reopened.liveAfterMs !== null && settledReload.ms !== null, liveAfterMs: reopened.liveAfterMs, syncPill: reopened.shell?.syncPill, handleKinds: [undone.shell?.handleKinds, settledReload.shell?.handleKinds] });
    const beforeLast = (await read()).handleKinds;
    await awaitNative("after-reload-edit", 300_000);
    const converged = await until(60_000, (shell) => shell.handleKinds > beforeLast);
    publish("converged", { converged: converged.ms !== null, afterMs: converged.ms, handleKinds: [beforeLast, converged.shell?.handleKinds] });
  }
  await awaitNative("done", 300_000);
} catch (error) {
  record("error", String(error?.stack ?? error).slice(0, 2000));
  await shot("error");
} finally {
  report.sockets = sockets;
  save();
  await browser.close();
  console.log(`REACT done ${ms()} ms`);
}
