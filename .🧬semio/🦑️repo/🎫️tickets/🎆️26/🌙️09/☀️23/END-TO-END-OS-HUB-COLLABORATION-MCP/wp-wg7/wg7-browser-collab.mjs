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

const SHELL_FROM_ENV = process.env.SEMIO_PROBE_URL;
const SHELL = SHELL_FROM_ENV;
const HUB = process.env.WG7_HUB;
if (!HUB || !SHELL_FROM_ENV) throw new Error("WG7_HUB and SEMIO_PROBE_URL are required (rule 23: no default hub or serve port)");
const SPACE = process.env.WG7_SPACE;
const DOCUMENT = process.env.WG7_DOCUMENT;
const EDIT_ACTION = process.env.WG7_EDIT_ACTION ?? "";
if (!SPACE || !DOCUMENT) throw new Error("WG7_SPACE and WG7_DOCUMENT are required");
const USERS = [
  { label: "A", email: "user1@semio.dev", password: "gm1-local-dev-pass-1", locale: "en-US" },
  { label: "B", email: "user2@semio.dev", password: "gm1-local-dev-pass-2", locale: "de-DE" },
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
  const context = await browser.newContext({ viewport: { width: 1600, height: 1000 }, deviceScaleFactor: 1, locale: user.locale });
  const page = await context.newPage();
  const lines = [];
  const hub = [];
  const documentFrames = [];
  page.on("console", (message) => lines.push(`${ms()} ${message.type()} ${message.text()}`));
  page.on("pageerror", (error) => lines.push(`${ms()} pageerror ${String(error)}`));
  page.on("response", (response) => {
    if (response.url().startsWith(HUB)) hub.push(`${ms()} ${response.status()} ${response.request().method()} ${response.url().replace(HUB, "")}`);
  });
  page.on("websocket", (socket) => {
    hub.push(`${ms()} ws-open ${socket.url().replace(HUB.replace("http", "ws"), "")}`);
    if (!socket.url().includes("/document/ws")) return;
    const text = (payload) => (typeof payload === "string" ? payload : Buffer.from(payload).toString("latin1"));
    const keep = (direction) => (frame) => {
      const body = typeof frame.payload === "string" ? Buffer.from(frame.payload) : Buffer.from(frame.payload);
      documentFrames.push({ at: ms(), direction, bytes: body.length, namesDocument: text(frame.payload).includes(DOCUMENT), ...(documentFrames.length < 400 && body.length <= 4096 ? { base64: body.toString("base64") } : {}) });
    };
    socket.on("framesent", keep("sent"));
    socket.on("framereceived", keep("received"));
  });
  await page.goto(SHELL, { waitUntil: "domcontentloaded", timeout: 180_000 });
  await page.waitForFunction(() => typeof globalThis.semioWgpuIntrospection?.dumpStructure === "function", null, { timeout: 180_000 });
  await page.waitForTimeout(8000);
  return { user, context, page, lines, hub, documentFrames };
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

/** 🔄️ Opens the Sync card only when its dock switch is off — activating an open card's switch closes it. */
async function openSyncCard(page) {
  const pill = (await projection(page)).find((node) => node.key === "s-sync-status" && node.windowId === "shell.chrome");
  return pill?.checked === true ? "open" : await activate(page, "s-sync-status", 2000);
}

/** 📏️ The hub's head for the document (its committed edit count), read as user A through the hub's own REST surface — what a
 * joiner must show right after it attaches, however many edits came before it. */
async function documentHead() {
  const [user] = USERS;
  const signIn = await fetch(`${HUB}/auth/sessions`, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: user.email, password: user.password, deviceInstanceId: `wg7head${Date.now().toString(16)}`, clientClass: "browser" }) }).then((response) => response.json());
  const document = await fetch(`${HUB}/spaces/${encodeURIComponent(SPACE)}/documents/${encodeURIComponent(DOCUMENT)}`, { headers: { authorization: `Bearer ${signIn.token}` } }).then((response) => response.json());
  return Number(document.head_seq ?? 0);
}

async function attach(session) {
  const { page } = session;
  const panel = await openSyncCard(page);
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
    peers: nodes.filter((node) => node.key === "s-presence-peers" && !/^(No one else is here|Niemand sonst ist hier)$/u.test(String(node.label ?? ""))).map((node) => node.label).slice(0, 8),
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
const hubHead = await documentHead();
for (const session of sessions) {
  const shown = await waitFor(session.page, (state) => blockRows(state).length >= hubHead, 30_000);
  record(`4b ${session.user.label} shows the document's existing content after attach (hub head ${hubHead})`, shown.ok, { blocks: blockRows(shown.last).length, hubHead });
}
const before = await view(b.page);
const addText = EDIT_ACTION || keyEndingWith(await projection(a.page), "note-play-blocks.add.text") || "";
const edited = addText ? await activate(a.page, addText, 6000) : "no-edit-control";
const aAfter = await view(a.page);
record("5 A authors an edit (Add Text)", edited === "activated" && blockRows(aAfter).length > 0, { addText, edited, blocksA: blockRows(aAfter).slice(0, 4), historyA: aAfter.history.slice(0, 3) });
const seen = await waitFor(b.page, (state) => blockRows(state).length > blockRows(before).length, 60_000);
await paintedFrame(b.page, "05-b-after-edit");
record("6 B sees A's edit without a reload", seen.ok, { blocksBefore: blockRows(before).length, blocksB: blockRows(seen.last).slice(0, 4), syncB: seen.last.sync });
const aBefore = await view(a.page);
const addTextB = EDIT_ACTION || keyEndingWith(await projection(b.page), "note-play-blocks.add.text") || "";
const editedB = addTextB ? await activate(b.page, addTextB, 6000) : "no-edit-control";
const bAfter = await view(b.page);
record("7 B authors an edit (Add Text)", editedB === "activated" && blockRows(bAfter).length > blockRows(seen.last).length, { addTextB, editedB, blocksB: blockRows(bAfter).slice(0, 4) });
const seenByA = await waitFor(a.page, (state) => blockRows(state).length > blockRows(aBefore).length, 60_000);
await paintedFrame(a.page, "07-a-after-b-edit");
await paintedFrame(b.page, "07-b-after-own-edit");
record("8 A sees B's edit without a reload", seenByA.ok, { blocksBefore: blockRows(aBefore).length, blocksA: blockRows(seenByA.last).slice(0, 4), syncA: seenByA.last.sync });
for (const session of sessions) {
  const sent = session.documentFrames.filter((frame) => frame.direction === "sent");
  record(`9 document frames name ${DOCUMENT} (${session.user.label})`, sent.some((frame) => frame.namesDocument), { sent: sent.length, sentNamingDocument: sent.filter((frame) => frame.namesDocument).length, received: session.documentFrames.length - sent.length, locale: session.user.locale, syncLabel: (await view(session.page)).sync });
}

/** 🫀️ One liveness sample: the frame worker's own stats answer (a frozen worker never answers) and how long it took. */
async function liveness(page) {
  const started = Date.now();
  const raw = await Promise.race([page.evaluate(async () => (await globalThis.semioWgpuIntrospection?.dumpFrameStats?.()) ?? ""), new Promise((resolve) => setTimeout(() => resolve(null), 5000))]);
  return { answered: raw !== null, latencyMs: Date.now() - started, stats: typeof raw === "string" ? raw.slice(0, 160) : null };
}
const SYNC_CARD = "s-sync-status";
const MEDIUM_CUT_MS = Number(process.env.WG7_MEDIUM_CUT_MS ?? 20_000);
const LINK_KEY = /(?:^|\/)framework\.sync\.link\.([a-z-]+)$/u;
const syncCard = (state) => state.nodes.filter((node) => node.windowId === SYNC_CARD);
const linkLine = (state) => syncCard(state).flatMap((node) => {
  const match = LINK_KEY.exec(String(node.key));
  return match ? [{ code: match[1], key: node.key }] : [];
});
const linkText = (state) => syncCard(state).filter((node) => node.role === "paragraph" && /verbindung|connection|zugriff|access/iu.test(String(node.label ?? ""))).map((node) => node.label).slice(0, 4);
const GERMAN_LINK_LINE = /Verbindung|Zugriff/u;
const cardDigest = (state) => syncCard(state).map((node) => `${String(node.key).split("/").pop()}=${String(node.label ?? "").slice(0, 60)}`).slice(0, 16);
if (process.env.WG7_OUTAGE === "1") {
  const ensureSyncCard = openSyncCard;
  for (const session of sessions) await ensureSyncCard(session.page);
  const aBlocksOnline = blockRows(await view(a.page)).length;
  await a.context.setOffline(true);
  const offlineAt = Date.now();
  await a.page.waitForTimeout(3000);
  const beats = [await liveness(a.page), await liveness(a.page)];
  record("10 a 15 s cut never freezes A (the frame worker keeps answering)", beats.every((beat) => beat.answered && beat.latencyMs < 2000), { beats });
  const offlineEdit = await activate(a.page, keyEndingWith(await projection(a.page), "note-play-blocks.add.text") ?? "absent", 4000);
  const aOffline = await view(a.page);
  record("11 A edits while offline: admitted locally and shown as queued (en)", offlineEdit === "activated" && blockRows(aOffline).length > aBlocksOnline && /pending|ausstehend/iu.test(aOffline.sync ?? ""), { offlineEdit, blocksBefore: aBlocksOnline, blocksAfter: blockRows(aOffline).length, sync: aOffline.sync });
  const bBefore = blockRows(await view(b.page)).length;
  await a.page.waitForTimeout(Math.max(0, 15_000 - (Date.now() - offlineAt)));
  await a.context.setOffline(false);
  const onlineAt = Date.now();
  const relinked = await waitFor(a.page, (state) => /live|connected|persisted|verbunden|gespeichert/iu.test(state.sync ?? "") && linkLine(state).length === 0, 60_000);
  const delivered = await waitFor(b.page, (state) => blockRows(state).length > bBefore, 60_000);
  record("12 after the 15 s cut A relinks in place and B receives the offline edit", relinked.ok && delivered.ok, { cutMs: onlineAt - offlineAt, relinkedAfterMs: Date.now() - onlineAt, syncA: relinked.last.sync, blocksB: blockRows(delivered.last).length, blocksBBefore: bBefore });
  await paintedFrame(a.page, "12-a-relinked");
  await ensureSyncCard(a.page);
  await a.context.setOffline(true);
  const mediumAt = Date.now();
  const short = await waitFor(a.page, (state) => linkLine(state).some((row) => row.code === "reconnecting"), MEDIUM_CUT_MS);
  await paintedFrame(a.page, "12b-a-reconnecting-en");
  record(`12b a ${MEDIUM_CUT_MS / 1000} s cut is spoken as a short shortage once the link drops (en)`, short.ok, { afterMs: Date.now() - mediumAt, link: linkLine(short.last), texts: linkText(short.last), card: cardDigest(short.last), sync: short.last.sync, beat: await liveness(a.page) });
  await a.page.waitForTimeout(Math.max(0, MEDIUM_CUT_MS - (Date.now() - mediumAt)));
  await a.context.setOffline(false);
  const back = await waitFor(a.page, (state) => /live|connected|persisted|verbunden|gespeichert/iu.test(state.sync ?? "") && linkLine(state).length === 0, 60_000);
  record(`12c after ${MEDIUM_CUT_MS / 1000} s A relinks and the line clears`, back.ok, { syncA: back.last.sync, link: linkLine(back.last), card: cardDigest(back.last) });
  await ensureSyncCard(b.page);
  await b.context.setOffline(true);
  const longAt = Date.now();
  const expired = await waitFor(b.page, (state) => linkLine(state).some((row) => row.code === "link-expired"), 120_000);
  await paintedFrame(b.page, "13-b-expired-de");
  record("13 a long cut expires B's link and says so (de)", expired.ok, { afterMs: Date.now() - longAt, link: linkLine(expired.last), texts: linkText(expired.last), card: cardDigest(expired.last), sync: expired.last.sync, beat: await liveness(b.page) });
  await b.context.setOffline(false);
  await b.page.waitForTimeout(15_000);
  await ensureSyncCard(b.page);
  const after = await view(b.page);
  record("13d B's expiry line speaks B's browser tongue (de-DE → German)", linkText(expired.last).some((line) => GERMAN_LINK_LINE.test(line)), { texts: linkText(expired.last) });
  record("14 an expired link never relinks by itself", linkLine(after).some((row) => row.code === "link-expired") && !/live|verbunden|connected|gespeichert|persisted/iu.test(after.sync ?? ""), { link: linkLine(after), texts: linkText(after), card: cardDigest(after), sync: after.sync });
}

for (const session of sessions) {
  writeFileSync(out(`console-${session.user.label}.txt`), session.lines.join("\n"));
  writeFileSync(out(`hub-${session.user.label}.txt`), session.hub.join("\n"));
  writeFileSync(out(`frames-${session.user.label}.json`), JSON.stringify(session.documentFrames, null, 1));
}
writeFileSync(out("steps.json"), JSON.stringify({ shell: SHELL, hub: HUB, space: SPACE, document: DOCUMENT, steps }, null, 2));
await browser.close();
console.log(`RESULT ${steps.filter((step) => step.pass).length}/${steps.length}`);
