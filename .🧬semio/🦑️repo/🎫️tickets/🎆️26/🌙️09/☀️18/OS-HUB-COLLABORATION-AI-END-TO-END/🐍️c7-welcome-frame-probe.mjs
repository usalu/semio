/** 📬️ C7 — decode the `Welcome` frame the hub sends on a live document socket.
 *
 * C6 §3.4 ends with an inference: the bootstrap live region never renders, so the hub probably sent
 * `Bootstrap::None`. That is a guess read off a DOM element that is absent for two different
 * reasons. This probe reads the byte instead: a CDP `Network.webSocketFrameReceived` listener on the
 * document socket, and the wire decoder for exactly the prefix of `ServerFrame::Welcome`
 * (`📡️replication/📡️wire/🦀️.rs:934`) that precedes the bootstrap tag —
 * `lane u8 | 0u8 | str session_id | str resume_token | frontier | bootstrap tag`, where a frontier is
 * `str document_id | varint head_edit_ordinal | str head_edit_id | varint last_commit_seq | 32 bytes`
 * and the bootstrap tag is 0 None / 1 Snapshot / 2 Tail / 3 ArtifactBootstrap.
 *
 * Usage: bun 🐍️c7-welcome-frame-probe.mjs [user1|user2] [shellUrl] [hubHostPort] [spaceId] [documentId]
 */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { mkdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { join } from "node:path";

const WHO = process.argv[2] ?? "user1";
const SHELL = process.argv[3] ?? "http://127.0.0.1:6191";
const HUB = process.argv[4] ?? "127.0.0.1:7621";
const SPACE = process.argv[5] ?? "01a0c314-e41f-780d-a980-3adda40ca9f7";
const DOCUMENT = process.argv[6] ?? "artifact-0954e2d10d8fff9605f101b0dba34f3b";
const TAG = process.env.C7_TAG ?? "c7";
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
mkdirSync(OUT, { recursive: true });

const USERS = {
  user1: { email: "user1@semio.dev", password: "gm1-local-dev-pass-1" },
  user2: { email: "user2@semio.dev", password: "gm1-local-dev-pass-2" },
};
const user = USERS[WHO];
const t0 = Date.now();
const ms = () => Date.now() - t0;
const out = { who: WHO, email: user.email, sockets: [], frames: [], notes: [] };
const save = () => writeFileSync(join(OUT, `${TAG}-welcome-${WHO}.json`), JSON.stringify(out, null, 2));

const readVarint = (bytes, cursor) => {
  let value = 0n;
  let shift = 0n;
  for (;;) {
    const byte = bytes[cursor.at];
    if (byte === undefined) throw new Error("varint truncated");
    cursor.at += 1;
    value |= BigInt(byte & 0x7f) << shift;
    if ((byte & 0x80) === 0) return value;
    shift += 7n;
    if (shift > 63n) throw new Error("varint overflow");
  }
};
const readStr = (bytes, cursor) => {
  const length = Number(readVarint(bytes, cursor));
  const slice = bytes.subarray(cursor.at, cursor.at + length);
  if (slice.length !== length) throw new Error("str truncated");
  cursor.at += length;
  return new TextDecoder().decode(slice);
};
const readFrontier = (bytes, cursor) => ({
  documentId: readStr(bytes, cursor),
  headEditOrdinal: String(readVarint(bytes, cursor)),
  headEditId: readStr(bytes, cursor),
  lastCommitSeq: String(readVarint(bytes, cursor)),
  chainHash: (() => {
    const slice = bytes.subarray(cursor.at, cursor.at + 32);
    cursor.at += 32;
    return [...slice].map((byte) => byte.toString(16).padStart(2, "0")).join("");
  })(),
});
const BOOTSTRAP_TAGS = ["None", "Snapshot", "Tail", "ArtifactBootstrap"];
const decodeWelcome = (bytes) => {
  const cursor = { at: 0 };
  const lane = bytes[cursor.at];
  cursor.at += 1;
  const tag = bytes[cursor.at];
  cursor.at += 1;
  if (tag !== 0) return { lane, frameTag: tag, kind: "not-welcome" };
  const sessionId = readStr(bytes, cursor);
  const resumeToken = readStr(bytes, cursor);
  const serverFrontier = readFrontier(bytes, cursor);
  const bootstrapTag = bytes[cursor.at];
  cursor.at += 1;
  const decoded = { lane, kind: "Welcome", sessionId, resumeTokenBytes: resumeToken.length, serverFrontier, bootstrapTag, bootstrap: BOOTSTRAP_TAGS[bootstrapTag] ?? `unknown-${bootstrapTag}` };
  if (bootstrapTag === 3) {
    decoded.artifactBootstrap = {
      formatVersion: Number(readVarint(bytes, cursor)),
      descriptorHash: (() => {
        const slice = bytes.subarray(cursor.at, cursor.at + 32);
        cursor.at += 32;
        return [...slice].map((byte) => byte.toString(16).padStart(2, "0")).join("");
      })(),
      artifactSchema: readStr(bytes, cursor),
      artifactKind: readStr(bytes, cursor),
    };
  }
  return decoded;
};

const click = async (page, selector, timeout = 8_000) => {
  if (!(await page.locator(selector).count())) return "absent";
  return page.locator(selector).first().click({ timeout, force: true }).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 110));
};

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 } });
const page = await context.newPage();
const lines = [];
page.on("console", (message) => lines.push(`${ms()} ${message.type()} ${message.text().slice(0, 500)}`));
page.on("pageerror", (error) => lines.push(`${ms()} pageerror ${String(error).slice(0, 300)}`));

// The document socket is opened by the STORE WORKER, not the page, so the page's own CDP session
// never sees it. `Target.setAutoAttach` with `flatten` gives one session per worker, and
// `Network.enable` on each of those is what makes its websocket frames readable.
const cdp = await context.newCDPSession(page);
const socketsById = new Map();
const wire = (session, label) => {
  session.on("Network.webSocketCreated", ({ requestId, url }) => {
    socketsById.set(requestId, url);
    lines.push(`${ms()} ws-created [${label}] ${url.slice(0, 180)}`);
    out.sockets.push({ at: ms(), label, url });
    save();
  });
  session.on("Network.webSocketFrameReceived", ({ requestId, response }) => {
    const url = socketsById.get(requestId) ?? "<unknown>";
    if (!/\/documents\//u.test(url)) return;
    if (out.frames.filter((frame) => frame.url === url).length >= 4) return;
    let bytes;
    try {
      bytes = response.opcode === 2 ? Buffer.from(response.payloadData, "base64") : Buffer.from(response.payloadData, "utf8");
    } catch (error) {
      out.notes.push(`payload decode: ${String(error).slice(0, 120)}`);
      return;
    }
    let decoded;
    try {
      decoded = decodeWelcome(bytes);
    } catch (error) {
      decoded = { kind: "decode-failed", error: String(error).slice(0, 160) };
    }
    const row = { at: ms(), label, url, opcode: response.opcode, bytes: bytes.length, head: [...bytes.subarray(0, 24)].map((byte) => byte.toString(16).padStart(2, "0")).join(" "), decoded };
    out.frames.push(row);
    lines.push(`${ms()} ws-frame [${label}] ${bytes.length}B ${JSON.stringify(decoded)}`);
    save();
  });
};
wire(cdp, "page");
cdp.on("Target.attachedToTarget", async ({ sessionId, targetInfo }) => {
  const session = cdp.connection?.session(sessionId) ?? null;
  if (!session) {
    out.notes.push(`no session for ${targetInfo.type}`);
    return;
  }
  wire(session, targetInfo.type);
  await session.send("Network.enable").catch((error) => out.notes.push(`worker Network.enable: ${String(error).slice(0, 120)}`));
  await session.send("Runtime.runIfWaitingForDebugger").catch(() => {});
});
await cdp.send("Network.enable");
await cdp.send("Target.setAutoAttach", { autoAttach: true, waitForDebuggerOnStart: true, flatten: true });

await page.goto(`${SHELL}/?plugin=gis2d`, { waitUntil: "domcontentloaded", timeout: 180_000 });
for (let attempt = 0; attempt < 200; attempt += 1) {
  await page.waitForTimeout(1_000);
  const ready = await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"));
  if (ready && attempt > 6) break;
}

await page.locator('[data-semio-hub-sign-in=""]').first().click();
const form = page.locator("[data-semio-hub-workspace]");
await form.waitFor({ state: "visible", timeout: 30_000 });
await form.locator('input[type="email"]').fill(user.email);
await form.locator('input[type="password"]').fill(user.password);
await form.locator('button[type="submit"][aria-label="Sign in"]').click();
await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60_000 }).catch((error) => out.notes.push(`sign-in wait: ${String(error).split("\n")[0]}`));
await page.waitForTimeout(3_000);
await click(page, '[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]');
await page.waitForTimeout(2_000);

for (let attempt = 0; attempt < 6; attempt += 1) {
  if (await page.locator('[id="framework.sync.remote.path"]').count()) break;
  if (await page.locator('[id="framework.sync.remote"]').count()) await click(page, '[id="framework.sync.remote"]');
  else if (await page.locator('[id="ui.utilities.group.sync"]').count()) await click(page, '[id="ui.utilities.group.sync"]');
  else await click(page, '[data-slot="panel-tab-button"][id="s-sync-status"], [id="s-sync-status"]');
  await page.waitForTimeout(1_500);
}
const input = page.locator('[id="framework.sync.remote.path"]');
out.cardFound = (await input.count()) > 0;
if (out.cardFound) {
  await input.fill(`${HUB}/${SPACE}/${DOCUMENT}`);
  await page.waitForTimeout(400);
  const attach = input.locator("xpath=ancestor::*[@data-slot='popover-content'][1]").locator('button:has([data-icon="link"])');
  out.attachPressed = (await attach.count()) ? await attach.first().click({ force: true, timeout: 8_000 }).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 90)) : "absent";
}

const deadline = Date.now() + Number(process.env.C7_WAIT_MS ?? 120_000);
while (Date.now() < deadline) {
  await page.waitForTimeout(2_000);
  if (out.frames.some((frame) => frame.decoded?.kind === "Welcome")) break;
}
await page.waitForTimeout(4_000);
out.bootstrapRegion = await page.evaluate(() => [...document.querySelectorAll("[data-semio-bootstrap-status]")].map((el) => (el.textContent ?? "").replace(/\s+/g, " ").trim().slice(0, 200)));
save();
writeFileSync(join(OUT, `${TAG}-welcome-${WHO}-console.txt`), lines.join("\n"));
await page.screenshot({ path: join(OUT, `${TAG}-welcome-${WHO}.png`), fullPage: false }).catch(() => {});
console.log(`WHO ${WHO} card=${out.cardFound} attach=${out.attachPressed}`);
console.log(`SOCKETS ${out.sockets.length}`);
for (const socket of out.sockets) console.log(`  ${String(socket.at).padStart(7)}ms [${socket.label}] ${socket.url.slice(0, 190)}`);
console.log(`DOCUMENT FRAMES ${out.frames.length}`);
for (const frame of out.frames) console.log(`  ${String(frame.at).padStart(7)}ms ${frame.bytes}B head=${frame.head}\n    ${JSON.stringify(frame.decoded)}`);
const welcome = out.frames.find((frame) => frame.decoded?.kind === "Welcome");
console.log(`BOOTSTRAP ${welcome ? `${welcome.decoded.bootstrap} (tag ${welcome.decoded.bootstrapTag})` : "<no Welcome frame captured>"}`);
if (welcome) console.log(`FRONTIER ${JSON.stringify(welcome.decoded.serverFrontier)}`);
console.log(`BOOTSTRAP-REGION ${JSON.stringify(out.bootstrapRegion)}`);
for (const note of out.notes) console.log(`NOTE ${note}`);
await browser.close();
