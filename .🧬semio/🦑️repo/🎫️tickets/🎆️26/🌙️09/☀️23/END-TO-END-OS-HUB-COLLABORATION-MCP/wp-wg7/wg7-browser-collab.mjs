/** 🤝️ WG7 — two humans collaborate on ONE hub document through the wasm32 BROWSER wgpu shell.
 *
 * Two isolated Chromium contexts (WebGPU on Metal), each a real wgpu shell served by the renderer's own
 * browser build, each signing in as a different hub user through the shell's hub workspace, each
 * attaching the SAME hub document through the sync card's `remote://host/space/document`, then user A
 * authors an edit and user B's shell must show it without a reload. Every control is reached through the
 * accessibility mirror (`#semio-wgpu-accessibility`), so the run also proves keyboard reachability.
 *
 * Usage (from wp-wg7): SEMIO_PROBE_URL=http://127.0.0.1:6550/?plugin=note \
 *   WG7_SPACE=<space id> WG7_DOCUMENT=<document id> bun wg7-browser-collab.mjs [tag]
 * Output: wp-wg7/generated/collab-<tag>-*.{png,json,txt}
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const here = dirname(fileURLToPath(import.meta.url));
const tag = process.argv[2] ?? "run";
const outDir = join(here, "generated");
mkdirSync(outDir, { recursive: true });
const out = (name) => join(outDir, `collab-${tag}-${name}`);

const SHELL = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6550/?plugin=note";
const HUB = process.env.WG7_HUB ?? "http://127.0.0.1:7800";
const SPACE = process.env.WG7_SPACE;
const DOCUMENT = process.env.WG7_DOCUMENT;
const EDIT_ACTION = process.env.WG7_EDIT_ACTION ?? "";
if (!SPACE || !DOCUMENT) throw new Error("WG7_SPACE and WG7_DOCUMENT are required");
const USERS = [
  { label: "A", email: "user1@semio.dev", password: "gm1-local-dev-pass-1" },
  { label: "B", email: "user2@semio.dev", password: "gm1-local-dev-pass-2" },
];
const MIRROR = "#semio-wgpu-accessibility";
const steps = [];
const startedAt = Date.now();
const ms = () => `+${((Date.now() - startedAt) / 1000).toFixed(1)}s`;
const record = (name, pass, detail) => {
  steps.push({ name, pass, at: ms(), ...detail });
  console.log(`[step] ${pass ? "PASS" : "FAIL"} ${name} ${ms()} :: ${JSON.stringify(detail).slice(0, 600)}`);
};

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });

/** 🔍️ The accessibility projection as one flat array of {key, role, label, …}. */
async function projection(page) {
  const raw = await page.evaluate(async () => (await globalThis.semioWgpuIntrospection?.dumpAccessibility?.()) ?? "");
  try {
    const parsed = JSON.parse(raw);
    return (parsed.windows ?? []).flatMap((surface) => (surface.nodes ?? []).map((node) => ({ ...node, windowId: surface.windowId })));
  } catch {
    return [];
  }
}

async function activate(page, key, settleMs = 2500) {
  const outcome = await page.evaluate(({ selector, nodeKey }) => {
    const element = document.querySelector(`${selector} [data-node-key="${nodeKey}"]`);
    if (!element) return "absent";
    if (element.disabled === true) return "disabled";
    element.focus();
    element.dispatchEvent(new MouseEvent("click", { bubbles: true }));
    return "activated";
  }, { selector: MIRROR, nodeKey: key });
  await page.waitForTimeout(settleMs);
  return outcome;
}

async function typeInto(page, key, value, settleMs = 1500, verify = false, budgetMs = 15_000) {
  const started = Date.now();
  let applied = false;
  while (!applied && Date.now() - started < budgetMs) {
    applied = await page.evaluate(({ selector, nodeKey, text }) => {
      const element = document.querySelector(`${selector} [data-node-key="${nodeKey}"]`);
      if (!(element instanceof HTMLInputElement) && !(element instanceof HTMLTextAreaElement)) return false;
      element.focus();
      element.value = text;
      element.dispatchEvent(new Event("input", { bubbles: true }));
      element.dispatchEvent(new Event("change", { bubbles: true }));
      return true;
    }, { selector: MIRROR, nodeKey: key, text: value });
    await page.waitForTimeout(applied ? settleMs : 500);
    if (applied && verify) applied = (await projection(page)).some((node) => node.key === key && node.valueText === value);
  }
  return applied;
}

async function awaitEnabled(page, suffix, budgetMs = 15_000) {
  const started = Date.now();
  let node = (await projection(page)).find((row) => String(row.key).endsWith(suffix));
  while ((!node || node.disabled === true) && Date.now() - started < budgetMs) {
    await page.waitForTimeout(500);
    node = (await projection(page)).find((row) => String(row.key).endsWith(suffix));
  }
  return node ? node.key : null;
}

async function submitInput(page, key, settleMs = 4000) {
  const submitted = await page.evaluate(({ selector, nodeKey }) => {
    const element = document.querySelector(`${selector} [data-node-key="${nodeKey}"]`);
    if (!element) return false;
    element.focus();
    element.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", code: "Enter", bubbles: true }));
    return true;
  }, { selector: MIRROR, nodeKey: key });
  await page.waitForTimeout(settleMs);
  return submitted;
}

const keyEndingWith = (nodes, suffix) => nodes.find((node) => String(node.key).endsWith(suffix))?.key;
const labelOf = (nodes, key) => nodes.find((node) => node.key === key)?.label ?? null;

/** 🖼️ A painted frame is one whose canvas region is not a single colour. */
async function paintedFrame(page, name) {
  const png = await page.screenshot({ path: out(`${name}.png`), type: "png" });
  const distinct = await page.evaluate(async (bytes) => {
    const blob = new Blob([new Uint8Array(bytes)], { type: "image/png" });
    const bitmap = await createImageBitmap(blob);
    const canvas = new OffscreenCanvas(bitmap.width, bitmap.height);
    const context = canvas.getContext("2d");
    context.drawImage(bitmap, 0, 0);
    const data = context.getImageData(0, 0, bitmap.width, bitmap.height).data;
    const colours = new Set();
    for (let index = 0; index < data.length && colours.size < 64; index += 4 * 97) colours.add(`${data[index]},${data[index + 1]},${data[index + 2]}`);
    return colours.size;
  }, [...png]);
  return distinct;
}

async function boot(user) {
  const context = await browser.newContext({ viewport: { width: 1600, height: 1000 }, deviceScaleFactor: 1 });
  const page = await context.newPage();
  const lines = [];
  const hub = [];
  page.on("console", (message) => lines.push(`${ms()} ${message.type()} ${message.text()}`));
  page.on("pageerror", (error) => lines.push(`${ms()} pageerror ${String(error)}`));
  page.on("response", (response) => {
    if (response.url().startsWith(HUB)) hub.push(`${ms()} ${response.status()} ${response.request().method()} ${response.url().replace(HUB, "")}`);
  });
  page.on("websocket", (socket) => hub.push(`${ms()} ws-open ${socket.url().replace(HUB.replace("http", "ws"), "")}`));
  await page.goto(SHELL, { waitUntil: "domcontentloaded", timeout: 180_000 });
  await page.waitForFunction(() => typeof globalThis.semioWgpuIntrospection?.dumpStructure === "function", null, { timeout: 180_000 });
  await page.waitForTimeout(8000);
  return { user, context, page, lines, hub };
}

/** 🔁️ Re-selects a hub connection: WGr's measured workaround for the mirror's stale enabled state after a
 * `Value` action (the draft lands, the control stays disabled until another hub verb republishes). */
async function republish(page, preferRemote) {
  const connections = (await projection(page)).filter((node) => String(node.key).includes("sign-in.connection"));
  const connection = (preferRemote ? connections.find((node) => String(node.key).includes("remote:")) : undefined) ?? connections[0];
  return connection ? activate(page, connection.key, 2000) : "absent";
}

async function awaitKey(page, suffix, budgetMs = 40_000) {
  const started = Date.now();
  let key = keyEndingWith(await projection(page), suffix);
  while (!key && Date.now() - started < budgetMs) {
    await page.waitForTimeout(1000);
    key = keyEndingWith(await projection(page), suffix);
  }
  return key;
}

async function signIn(session) {
  const { page, user } = session;
  const opened = await activate(page, "framework.hub.signIn", 2000);
  const address = await awaitKey(page, "framework.hub.address");
  const typedAddress = address ? await typeInto(page, address, HUB) : false;
  let added = "absent";
  for (let attempt = 0; attempt < 4 && added !== "activated"; attempt += 1) {
    await republish(page, false);
    const add = await awaitKey(page, "framework.hub.sign-in.add", 5_000);
    added = add ? await activate(page, add, 4000) : "absent";
  }
  const minted = () => session.hub.some((line) => / 20[01] POST \/auth\/sessions$/u.test(line));
  let selected = "absent", typedEmail = false, typedPassword = false, submitted = "absent", attempts = 0;
  for (; attempts < 4 && !minted(); attempts += 1) {
    selected = await republish(page, true);
    const email = await awaitKey(page, "framework.hub.email", 5_000);
    const password = await awaitKey(page, "framework.hub.password", 5_000);
    typedEmail = email ? await typeInto(page, email, user.email) : false;
    typedPassword = password ? await typeInto(page, password, user.password) : false;
    await republish(page, true);
    const submit = await awaitKey(page, "framework.hub.sign-in.submit", 5_000);
    submitted = submit ? await activate(page, submit, 12_000) : "absent";
  }
  await republish(page, true);
  await page.waitForTimeout(3000);
  const nodes = await projection(page);
  const pill = nodes.find((node) => node.key === "s-hub-connection" || node.key === "framework.hub.signIn");
  const spaceRows = nodes.filter((node) => String(node.key).includes(SPACE)).map((node) => node.label).slice(0, 4);
  const close = keyEndingWith(nodes, "framework.hub.close");
  if (close) await activate(page, close, 1500);
  return { opened, typedAddress, added, selected, typedEmail, typedPassword, submitted, attempts, minted: minted(), spaceRows, pill: pill ? { key: pill.key, label: pill.label } : null, auth: session.hub.filter((line) => line.includes("/auth/")) };
}

async function attach(session) {
  const { page } = session;
  const panel = await activate(page, "s-sync-status", 2000);
  const remote = await awaitKey(page, "framework.sync.remote", 10_000);
  const chose = remote ? await activate(page, remote, 2000) : "absent";
  const path = await awaitKey(page, "framework.sync.remote.path", 10_000);
  const uri = `${HUB.replace(/^https?:\/\//u, "")}/${SPACE}/${DOCUMENT}`;
  const typed = path ? await typeInto(page, path, uri, 1500, true) : false;
  const submitted = path ? await submitInput(page, path, 3000) : false;
  const attachKey = await awaitEnabled(page, "framework.sync.attach");
  const pressed = attachKey ? await activate(page, attachKey, 4000) : "absent";
  return { panel, chose, typed, submitted, pressed, uri };
}

async function view(page) {
  const nodes = await projection(page);
  return {
    sync: labelOf(nodes, "s-sync-status"),
    peers: nodes.filter((node) => String(node.key).startsWith("peer:") || String(node.key).includes("presence")).map((node) => node.label).slice(0, 8),
    history: nodes.filter((node) => String(node.key).startsWith("framework.history.entry.") && !String(node.key).endsWith(".revert")).map((node) => node.label).slice(0, 40),
    nodes,
  };
}

async function waitFor(page, predicate, budgetMs) {
  const started = Date.now();
  let last = await view(page);
  while (Date.now() - started < budgetMs) {
    if (predicate(last)) return { ok: true, last };
    await page.waitForTimeout(1500);
    last = await view(page);
  }
  return { ok: predicate(last), last };
}

const sessions = [];
for (const user of USERS) sessions.push(await boot(user));
for (const session of sessions) {
  const distinct = await paintedFrame(session.page, `01-boot-${session.user.label}`);
  record(`1 painted frame (${session.user.label})`, distinct > 4, { distinctColours: distinct });
}
for (const session of sessions) {
  const outcome = await signIn(session);
  record(`2 hub sign-in (${session.user.label} ${session.user.email})`, outcome.minted, outcome);
}
for (const session of sessions) {
  const outcome = await attach(session);
  const live = await waitFor(session.page, (state) => /live|connected|persisted|verbunden/iu.test(state.sync ?? ""), 120_000);
  await paintedFrame(session.page, `03-attached-${session.user.label}`);
  writeFileSync(out(`03-projection-${session.user.label}.json`), JSON.stringify(live.last.nodes, null, 1));
  record(`3 document attached and live (${session.user.label})`, live.ok, { ...outcome, sync: live.last.sync, sockets: session.hub.filter((line) => line.includes("ws-open") || line.includes("open-plan") || line.includes("socket-grants") || line.includes("execution-target")) });
}
const [a, b] = sessions;
const peers = await waitFor(b.page, (state) => state.peers.length >= 1, 30_000);
record("4 presence shows the other user", peers.ok, { peersSeenByB: peers.last.peers });
const blockRows = (state) => state.nodes.filter((node) => String(node.key).startsWith("note-play-block:")).map((node) => node.label);
for (const session of sessions) await activate(session.page, "framework.panel.artifact", 3000);
const before = await view(b.page);
const addText = EDIT_ACTION || keyEndingWith(await projection(a.page), "note-play-blocks.add.text") || "";
const edited = addText ? await activate(a.page, addText, 6000) : "no-edit-control";
const aAfter = await view(a.page);
record("5 A authors an edit (Add Text)", edited === "activated" && blockRows(aAfter).length > 0, { addText, edited, blocksA: blockRows(aAfter).slice(0, 4), historyA: aAfter.history.slice(0, 3) });
const seen = await waitFor(b.page, (state) => blockRows(state).length > blockRows(before).length, 60_000);
await paintedFrame(b.page, "05-b-after-edit");
record("6 B sees A's edit without a reload", seen.ok, { blocksBefore: blockRows(before).length, blocksB: blockRows(seen.last).slice(0, 4), syncB: seen.last.sync });

for (const session of sessions) {
  writeFileSync(out(`console-${session.user.label}.txt`), session.lines.join("\n"));
  writeFileSync(out(`hub-${session.user.label}.txt`), session.hub.join("\n"));
}
writeFileSync(out("steps.json"), JSON.stringify({ shell: SHELL, hub: HUB, space: SPACE, document: DOCUMENT, steps }, null, 2));
await browser.close();
console.log(`RESULT ${steps.filter((step) => step.pass).length}/${steps.length}`);
