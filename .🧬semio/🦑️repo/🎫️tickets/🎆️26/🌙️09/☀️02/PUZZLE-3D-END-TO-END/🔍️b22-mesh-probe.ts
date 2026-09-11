/** 📐️ Wave B22 measurement probe — what one brush-mesh upload actually costs on the per-actor command
 * queue. Boots the puzzle 3d serve on 127.0.0.1:6013, timestamps every `[DEBUG] command ingress …`
 * console line the runtime already emits (no source tap needed), pairs enqueue with settle, and — once
 * the mesh storm is running — dispatches ONE user action and measures how long it waits behind it.
 *
 * Run: `bun 🔍️b22-mesh-probe.ts [--port=<n>] [--watch=<seconds>] [--action-at=<seconds>]`.
 * Output: `🗑️generated/b22-mesh-<stamp>.json` + `.ndjson` (one record per console line kept). */
import { chromium } from "playwright";
import { appendFileSync, mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const TICKET = import.meta.dir;
const OUT = join(TICKET, "🗑️generated");
mkdirSync(OUT, { recursive: true });
const stamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19);
const port = process.argv.find((a) => a.startsWith("--port="))?.slice(7) ?? "6013";
const watchSeconds = Number(process.argv.find((a) => a.startsWith("--watch="))?.slice(8) ?? 180);
const actionAtSeconds = Number(process.argv.find((a) => a.startsWith("--action-at="))?.slice(12) ?? 45);
const ndjson = join(OUT, `b22-mesh-${stamp}.ndjson`);
writeFileSync(ndjson, "");

const t0 = Date.now();
const at = () => Number(((Date.now() - t0) / 1000).toFixed(3));
const log = (m: string) => console.log(`[${at().toFixed(1)}s] ${m}`);

type Ingress = { readonly at: number; readonly actionId: string | null; readonly seq: number | null; readonly lane: string };
const enqueued: Ingress[] = [];
const settled: { readonly at: number; readonly status: string }[] = [];
const continuations: number[] = [];
let continuationsSinceSettle = 0;
const notices: string[] = [];
const samples: { readonly at: number; readonly posts: number; readonly messages: number; readonly enqueued: number; readonly settled: number }[] = [];

const record = (row: Record<string, unknown>) => appendFileSync(ndjson, `${JSON.stringify({ at: at(), ...row })}\n`);

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
/** 🔁️ Counts actor round trips without a source tap: every `submitTurn` reaches the wasm shard as one
 * `Worker.postMessage`, so the delta across a window of commands is that window's turn count. */
await page.addInitScript(() => {
  const counter = { posts: 0, messages: 0 };
  (window as unknown as { __b22: typeof counter }).__b22 = counter;
  const post = Worker.prototype.postMessage;
  Worker.prototype.postMessage = function patched(this: Worker, ...args: Parameters<Worker["postMessage"]>) {
    counter.posts += 1;
    return post.apply(this, args as never);
  } as Worker["postMessage"];
  const add = Worker.prototype.addEventListener;
  Worker.prototype.addEventListener = function patched(this: Worker, type: string, listener: EventListenerOrEventListenerObject | null, options?: boolean | AddEventListenerOptions) {
    if (type === "message" && typeof listener === "function") {
      return add.call(this, type, (event: Event) => {
        counter.messages += 1;
        return (listener as EventListener)(event);
      }, options);
    }
    return add.call(this, type, listener as EventListener, options);
  } as Worker["addEventListener"];
});
const workerTurns = async () => page.evaluate(() => (window as unknown as { __b22?: { posts: number; messages: number } }).__b22 ?? { posts: -1, messages: -1 }).catch(() => ({ posts: -1, messages: -1 }));
page.on("console", (msg) => {
  const text = msg.text();
  if (text.includes("command ingress lane")) {
    try {
      const parsed = JSON.parse(text.slice(text.indexOf("{"))) as { actionId: string | null; seq: number | null; lane: string };
      enqueued.push({ at: at(), actionId: parsed.actionId, seq: parsed.seq, lane: parsed.lane });
      record({ kind: "enqueue", ...parsed });
    } catch {
      record({ kind: "enqueue-unparsed", text: text.slice(0, 200) });
    }
    return;
  }
  if (text.includes("command ingress settled")) {
    settled.push({ at: at(), status: text.slice(text.indexOf("status=")) });
    continuations.push(continuationsSinceSettle);
    continuationsSinceSettle = 0;
    record({ kind: "settle", text: text.slice(0, 160) });
    return;
  }
  if (text.includes("command ingress continuation")) {
    continuationsSinceSettle = Number(text.match(/continuation (\d+)/)?.[1] ?? continuationsSinceSettle);
    record({ kind: "continuation", text: text.slice(0, 160) });
    return;
  }
  if (/puzzle3d-register-mesh|worker fault|panic|unreachable/i.test(text)) {
    notices.push(text.slice(0, 240));
    record({ kind: "notice", text: text.slice(0, 240) });
  }
});
page.on("pageerror", (error) => record({ kind: "pageerror", text: String(error).slice(0, 240) }));

log("navigating");
await page.goto(`http://127.0.0.1:${port}/?plugin=puzzle3d`, { waitUntil: "domcontentloaded", timeout: 60000 }).catch((error) => log(`goto: ${String(error).slice(0, 160)}`));
await page.waitForLoadState("domcontentloaded").catch(() => {});

const snapshot = async () =>
  page
    .evaluate(() => ({
      windows: document.querySelectorAll('[data-slot="window"]').length,
      canvases: document.querySelectorAll("canvas").length,
      dialogs: document.querySelectorAll('[role="dialog"]').length,
      meshUrls: Array.from(document.querySelectorAll("[data-instances-json]")).flatMap((el) => {
        try {
          return (JSON.parse(el.getAttribute("data-instances-json") || "[]") as { meshUrl?: string; url?: string }[]).map((i) => i.meshUrl ?? i.url ?? "");
        } catch {
          return [];
        }
      }),
    }))
    .catch(() => ({ windows: 0, canvases: 0, dialogs: 0, meshUrls: [] as string[] }));

/** ⏱️ Samples the worker-turn counters from the first paint, because the boot mesh storm is over before
 * any poll loop notices it. */
const sampler = setInterval(() => {
  void workerTurns().then((turns) => {
    samples.push({ at: at(), posts: turns.posts, messages: turns.messages, enqueued: enqueued.length, settled: settled.length });
  });
}, 500);

let booted = false;
for (let poll = 0; poll < 40; poll += 1) {
  await page.waitForTimeout(3000);
  const snap = await snapshot();
  if (snap.dialogs > 0 && poll % 3 === 0) {
    const skip = page.locator('[role="dialog"] button', { hasText: /skip/i }).first();
    if (await skip.count().catch(() => 0)) await skip.click({ timeout: 2000 }).catch(() => {});
  }
  if (snap.windows >= 2 && snap.canvases >= 2) {
    booted = true;
    log(`booted windows=${snap.windows} canvases=${snap.canvases} enqueued=${enqueued.length}`);
    break;
  }
  if (poll % 4 === 3) log(`waiting… windows=${snap.windows} canvases=${snap.canvases} enqueued=${enqueued.length}`);
}
const tour = page.locator("button", { hasText: /^\s*(x\s*)?skip\s*$/i }).first();
if (await tour.count().catch(() => 0)) await tour.click({ timeout: 3000 }).catch(() => {});

/** 🏙️ Switches to the document-scale example whose collision mesh is the 72-page Nakagin capsule — the
 * default Concrete Forest mesh only pages six ways and never reaches the ceiling under test. */
const example = process.argv.find((a) => a.startsWith("--example="))?.slice(10);
let switchedAt: number | null = null;
if (example) {
  const combo = page.locator('[role="combobox"]').first();
  await combo.click({ timeout: 5000 }).catch(() => {});
  const option = page.locator('[role="option"]').filter({ hasText: new RegExp(example, "i") }).first();
  switchedAt = at();
  if (await option.count().catch(() => 0)) await option.click({ timeout: 5000 }).catch(() => {});
  else log(`example "${example}" not offered`);
  log(`example switch requested at ${switchedAt.toFixed(1)}s`);
}

/** 🖱️ The interleaving measurement: one plain selection click on the world canvas, timed from the click
 * to the first `command ingress settled` that follows its own enqueue. */
let userAction: { readonly clickedAt: number; readonly enqueuedAt: number | null; readonly settledAt: number | null; readonly actionId: string | null; readonly queuedAhead: number } | null = null;
const fireUserAction = async () => {
  const before = enqueued.length;
  const settledBefore = settled.length;
  const queuedAhead = enqueued.length - settled.length;
  const clickedAt = at();
  const canvas = page.locator("canvas").last();
  const box = await canvas.boundingBox().catch(() => null);
  if (box) await page.mouse.click(box.x + box.width / 2, box.y + box.height / 2).catch(() => {});
  for (let poll = 0; poll < 600; poll += 1) {
    await page.waitForTimeout(500);
    const mine = enqueued.slice(before).find((row) => row.actionId !== "registerBrushMesh");
    if (mine) {
      const index = enqueued.indexOf(mine);
      if (settled.length > index) {
        userAction = { clickedAt, enqueuedAt: mine.at, settledAt: settled[index]!.at, actionId: mine.actionId, queuedAhead };
        return;
      }
    }
    if (settled.length - settledBefore > 400) break;
  }
  const mine = enqueued.slice(before).find((row) => row.actionId !== "registerBrushMesh") ?? null;
  userAction = { clickedAt, enqueuedAt: mine?.at ?? null, settledAt: null, actionId: mine?.actionId ?? null, queuedAhead };
};

const deadline = Date.now() + watchSeconds * 1000;
let fired = false;
while (Date.now() < deadline) {
  if (!fired && at() >= actionAtSeconds && enqueued.some((row) => row.actionId === "registerBrushMesh")) {
    fired = true;
    log(`firing user action at ${at().toFixed(1)}s (enqueued=${enqueued.length} settled=${settled.length})`);
    await fireUserAction();
    log(`user action: ${JSON.stringify(userAction)}`);
    continue;
  }
  await page.waitForTimeout(1000);
  const turns = samples.at(-1) ?? { posts: -1 };
  if (Math.round(at()) % 20 < 2) log(`enqueued=${enqueued.length} settled=${settled.length} mesh=${enqueued.filter((r) => r.actionId === "registerBrushMesh").length} posts=${turns.posts}`);
}

/** 🔁️ Worker round trips one mesh command costs, read off the sampler over the longest window in which
 * ONLY mesh commands settled — posts delta divided by settles delta. */
const meshTurnsPerCommand = () => {
  const meshIndices = new Set(enqueued.map((row, index) => (row.actionId === "registerBrushMesh" ? index : -1)).filter((index) => index >= 0));
  let best: { readonly commands: number; readonly posts: number; readonly perCommand: number } | null = null;
  for (let start = 0; start < samples.length; start += 1) {
    for (let end = samples.length - 1; end > start; end -= 1) {
      const commands = samples[end]!.settled - samples[start]!.settled;
      if (commands < 3) continue;
      const pure = Array.from({ length: commands }, (_, offset) => meshIndices.has(samples[start]!.settled + offset)).every(Boolean);
      if (!pure) continue;
      const posts = samples[end]!.posts - samples[start]!.posts;
      if (!best || commands > best.commands) best = { commands, posts, perCommand: Number((posts / commands).toFixed(1)) };
      break;
    }
  }
  return best;
};

clearInterval(sampler);
const mesh = enqueued.filter((row) => row.actionId === "registerBrushMesh");
const durations: number[] = [];
for (let index = 0; index < Math.min(enqueued.length, settled.length); index += 1) {
  if (enqueued[index]!.actionId !== "registerBrushMesh") continue;
  const started = index === 0 ? enqueued[index]!.at : Math.max(enqueued[index]!.at, settled[index - 1]!.at);
  durations.push(Number((settled[index]!.at - started).toFixed(3)));
}
const sorted = [...durations].sort((a, b) => a - b);
const quantile = (q: number) => (sorted.length === 0 ? null : sorted[Math.min(sorted.length - 1, Math.floor(q * sorted.length))]!);
const summary = {
  stamp,
  port,
  booted,
  example: example ?? null,
  switchedAt,
  watchSeconds,
  enqueued: enqueued.length,
  settled: settled.length,
  meshCommands: mesh.length,
  meshLanes: [...new Set(mesh.map((row) => row.lane))],
  firstMeshAt: mesh[0]?.at ?? null,
  lastMeshAt: mesh.at(-1)?.at ?? null,
  meshCommandSeconds: { min: quantile(0), p50: quantile(0.5), p90: quantile(0.9), max: sorted.at(-1) ?? null, total: Number(durations.reduce((a, b) => a + b, 0).toFixed(3)) },
  continuationsPerCommand: { max: Math.max(0, ...continuations), samples: continuations.slice(0, 40) },
  userAction,
  workerTurnsPerMeshCommand: meshTurnsPerCommand(),
  samples: samples.slice(0, 900),
  notices: notices.slice(0, 20),
};
writeFileSync(join(OUT, `b22-mesh-${stamp}.json`), `${JSON.stringify(summary, null, 2)}\n`);
log(JSON.stringify(summary, null, 2));
await browser.close();
