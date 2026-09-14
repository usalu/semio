/** ⏱️ wgpu EDIT-MODE CONVERGENCE PROFILE — where the 3–5× against React goes, hop by hop.
 *
 * One page load of one example in ONE lane, instrumented three ways at once:
 *   • the full `[DEBUG]` console with millisecond stamps, bucketed per second into the hops the
 *     round-trip is made of (`os_host drain events` → `frame build admitted` → `render begin/leave`
 *     → `refreshUi` → `reconcile` → `world3d surface` → `invokeExtension`);
 *   • the GAP ledger — for each named trace, the longest silence between two consecutive occurrences,
 *     which is what "the host tick waits for input" looks like from outside;
 *   • a CPU profile of the frame Worker over a window in the middle of the convergence, ranked by
 *     self time, taken over the worker's OWN debugger socket (Playwright only attaches to pages), the
 *     way `🐍️wgpu-spin-stack-probe.mjs` does.
 *
 * The convergence predicate is `🐍️wgpu-example-matrix-probe.mjs`'s, verbatim, so the seconds this
 * probe reports are comparable with `📓️wgpu-end-to-end-verification-2026-09-14.md` §2.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_EXAMPLE=hexagonal-mushroom-column SEMIO_PROBE_LANE=edit \
 *        SEMIO_PROBE_OUT=wgpu-perf/edit-base bun 🐍️wgpu-edit-perf-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const origin = process.env.SEMIO_PROBE_ORIGIN ?? "http://127.0.0.1:6118";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-perf/edit-base");
const example = process.env.SEMIO_PROBE_EXAMPLE ?? "hexagonal-mushroom-column";
const lane = process.env.SEMIO_PROBE_LANE ?? "edit";
const budgetSeconds = Number(process.env.SEMIO_PROBE_BUDGET ?? 90);
const profileFromSeconds = Number(process.env.SEMIO_PROBE_PROFILE_FROM ?? 8);
const profileSeconds = Number(process.env.SEMIO_PROBE_PROFILE ?? 12);
const cdpPort = Number(process.env.SEMIO_PROBE_CDP_PORT ?? 9341);
const nudge = (process.env.SEMIO_PROBE_NUDGE ?? "1") !== "0";
const viewport = { width: 1440, height: 900 };
mkdirSync(outDir, { recursive: true });

const axis = example === "(none)" ? "" : `&example=${encodeURIComponent(example)}`;
const LANES = {
  edit: `${origin}/?plugin=generation3d&mode=edit${axis}`,
  viewer: `${origin}/?plugin=generation3d&role=viewer${axis}`,
  generate: `${origin}/?plugin=generation3d&mode=generate${axis}`,
};
const url = LANES[lane];

/** 🔌️ One raw CDP session over a target's own debugger socket. */
class RawCdp {
  #socket;
  #next = 1;
  #pending = new Map();
  events = [];
  static async open(target) {
    const session = new RawCdp();
    session.#socket = new WebSocket(target);
    await new Promise((resolve, reject) => {
      session.#socket.onopen = () => resolve();
      session.#socket.onerror = (error) => reject(new Error(`cdp-open-failed: ${String(error?.message ?? error)}`));
    });
    session.#socket.onmessage = (message) => {
      const payload = JSON.parse(message.data);
      if (payload.id !== undefined) {
        const waiter = session.#pending.get(payload.id);
        session.#pending.delete(payload.id);
        waiter?.(payload);
      } else session.events.push(payload);
    };
    return session;
  }
  send(method, params = {}, timeoutMs = 30000) {
    const id = this.#next++;
    this.#socket.send(JSON.stringify({ id, method, params }));
    return new Promise((resolve) => {
      const timer = setTimeout(() => {
        this.#pending.delete(id);
        resolve({ timedOut: true, method });
      }, timeoutMs);
      this.#pending.set(id, (payload) => {
        clearTimeout(timer);
        resolve(payload);
      });
    });
  }
  close() {
    try {
      this.#socket.close();
    } catch {}
  }
}

/** 📊️ Self-time ranking of a `Profiler.stop` profile — the hot frame, not the whole tree. */
const rankSelfTime = (profile) => {
  if (!profile?.nodes) return [];
  const byId = new Map(profile.nodes.map((node) => [node.id, node]));
  const total = profile.samples?.length || 1;
  const hits = new Map();
  for (const id of profile.samples ?? []) hits.set(id, (hits.get(id) ?? 0) + 1);
  return [...hits.entries()]
    .sort((a, b) => b[1] - a[1])
    .slice(0, 25)
    .map(([id, count]) => {
      const frame = byId.get(id)?.callFrame ?? {};
      return { pct: Math.round((count / total) * 1000) / 10, fn: frame.functionName || "(anonymous)", url: (frame.url || "").slice(-56), line: frame.lineNumber };
    });
};

/** 🧭️ The hops the convergence round-trip is made of, each a console needle. */
const HOPS = {
  drainEvents: "os_host drain events",
  frameAdmitted: "frame build admitted",
  frameSuperseded: "frame build superseded",
  renderBegin: "wgpu-shell render begin",
  renderLeave: "wgpu-shell render leave",
  refreshUi: "refreshUi",
  reconcile: "reconcile",
  documentPage: "document page",
  world3d: "world3d surface=",
  invoke: "invokeExtension",
  resolve: "extension resolved",
  typedOperation: "typedOperation",
  capacity: "Capacity",
  dockPlan: "dock plan",
  present: "present_step",
};

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal", `--remote-debugging-port=${cdpPort}`] });
const page = await browser.newPage({ viewport });
const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 3000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 1200)}`));

const has = (needle) => lines.filter((line) => line.includes(needle));

const dump = (windowId) =>
  page
    .evaluate(async (id) => {
      const beacon = globalThis.semioWgpuIntrospection;
      if (typeof beacon?.dumpStructure !== "function") return null;
      const parse = async (call) => {
        try {
          const raw = await call();
          return raw ? JSON.parse(raw) : null;
        } catch {
          return null;
        }
      };
      return { structure: await parse(() => beacon.dumpStructure(id)), stats: await parse(() => beacon.dumpFrameStats(id)) };
    }, windowId)
    .catch(() => null);

const world3dTraces = () => {
  const byId = {};
  for (const line of has("world3d surface=")) {
    const surface = /world3d surface=(\S+)/.exec(line)?.[1];
    if (!surface) continue;
    byId[surface] = { instances: Number(/ instances=(\d+)/.exec(line)?.[1] ?? 0), lines: Number(/ lines=(\d+)/.exec(line)?.[1] ?? 0), stateMeshes: Number(/ state-meshes=(\d+)/.exec(line)?.[1] ?? 0) };
  }
  return byId;
};
const geometryEvidence = () => {
  for (const line of has("world3d surface=")) {
    if (/\\"positions\\":\[-?\d/.test(line)) return "solid";
    if (/\\"edgePositions\\":\[-?\d/.test(line)) return "wire";
  }
  return null;
};

const measure = async () => {
  const ids = (await dump(undefined))?.structure?.windowIds ?? [];
  let scenePasses = 0;
  for (const id of ids) {
    const stats = (await dump(id))?.stats ?? null;
    scenePasses = Math.max(scenePasses, stats?.scenePasses ?? 0);
  }
  const alert = await page.evaluate(() => document.querySelector('[role="alert"]')?.textContent?.trim()?.slice(0, 200) ?? null).catch(() => null);
  const traces = world3dTraces();
  const meshy = Object.entries(traces).filter(([, t]) => t.stateMeshes > 0 && (t.instances > 0 || t.lines > 0));
  return { windowIds: ids, scenePasses, meshy: meshy.map(([id, t]) => ({ surface: id, ...t })), geometry: geometryEvidence(), alert, converged: scenePasses > 0 && meshy.length > 0 && geometryEvidence() !== null && !alert };
};

await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 300)}`));

let profileSession = null;
let profileStartedAt = null;
let hot = [];
let firstConvergedMs = null;
let stable = 0;
let flip = 0;
const samples = [];

for (let second = 0; second < budgetSeconds; second += 1) {
  for (let step = 0; step < 5; step += 1) {
    await page.waitForTimeout(200);
    if (nudge) {
      flip = 1 - flip;
      await page.mouse.move(3 + flip, 3).catch(() => {});
    }
  }
  if (second === profileFromSeconds && profileSeconds > 0) {
    const targets = await fetch(`http://127.0.0.1:${cdpPort}/json/list`)
      .then((response) => response.json())
      .catch(() => []);
    const worker = targets.filter((target) => target.type === "worker").find((target) => decodeURIComponent(target.url).includes("frame")) ?? targets.find((target) => target.type === "worker");
    if (worker) {
      profileSession = await RawCdp.open(worker.webSocketDebuggerUrl).catch(() => null);
      if (profileSession) {
        await profileSession.send("Profiler.enable", {}, 5000);
        await profileSession.send("Profiler.setSamplingInterval", { interval: 300 }, 5000);
        await profileSession.send("Profiler.start", {}, 5000);
        profileStartedAt = at();
        lines.push(`${at()} PROBE profiler started on ${decodeURIComponent(worker.url).slice(-60)}`);
      }
    } else lines.push(`${at()} PROBE no worker target to profile (targets=${targets.length})`);
  }
  if (profileSession && profileStartedAt !== null && at() - profileStartedAt >= profileSeconds * 1000 && hot.length === 0) {
    const stopped = await profileSession.send("Profiler.stop", {}, 60000);
    hot = rankSelfTime(stopped?.result?.profile);
    lines.push(`${at()} PROBE profiler stopped hot=${JSON.stringify(hot.slice(0, 10))}`);
    profileSession.close();
    profileSession = null;
  }
  const sample = await measure();
  samples.push({ second: second + 1, scenePasses: sample.scenePasses, meshSurfaces: sample.meshy.length, converged: sample.converged });
  if (sample.converged) {
    firstConvergedMs ??= at();
    stable += 1;
    if (stable >= 2 && hot.length > 0) break;
  } else stable = 0;
}
if (profileSession) {
  const stopped = await profileSession.send("Profiler.stop", {}, 60000);
  hot = rankSelfTime(stopped?.result?.profile);
  profileSession.close();
}

/** 📏️ Longest silence between two consecutive occurrences of a needle, and the per-second histogram. */
const gapLedger = () => {
  const ledger = {};
  for (const [name, needle] of Object.entries(HOPS)) {
    const stamps = lines.filter((line) => line.includes(needle)).map((line) => Number(line.slice(0, line.indexOf(" "))));
    if (stamps.length === 0) {
      ledger[name] = { count: 0 };
      continue;
    }
    let maxGap = stamps[0];
    let maxGapAt = 0;
    for (let index = 1; index < stamps.length; index += 1) {
      const gap = stamps[index] - stamps[index - 1];
      if (gap > maxGap) {
        maxGap = gap;
        maxGapAt = stamps[index - 1];
      }
    }
    const histogram = {};
    for (const stamp of stamps) {
      const bucket = Math.floor(stamp / 1000);
      histogram[bucket] = (histogram[bucket] ?? 0) + 1;
    }
    ledger[name] = { count: stamps.length, firstMs: stamps[0], lastMs: stamps.at(-1), maxGapMs: maxGap, maxGapAtMs: maxGapAt, perSecond: histogram };
  }
  return ledger;
};

const report = {
  url,
  example,
  lane,
  timeToMeshSeconds: firstConvergedMs === null ? null : Math.round(firstConvergedMs / 10) / 100,
  elapsedSeconds: Math.round(at() / 1000),
  consoleLines: lines.length,
  hot,
  gaps: gapLedger(),
  samples,
};
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
await browser.close();
console.log("DONE", JSON.stringify({ example, lane, timeToMeshSeconds: report.timeToMeshSeconds, out: outDir, hot: hot.slice(0, 8) }, null, 2));
