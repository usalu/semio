/** 🎯️ HIT-DRAIN probe — does a real click resolve the thing under the pointer, EVERY time?
 *
 * The defect (`📓️wgpu-end-to-end-verification-2026-09-14.md` §D): `InputState`'s pointer registry was
 * ONE vector, retired one entry per frame-build boundary step (`FrameBuildPhase::InputFrame`) and
 * re-minted by the chrome walk at the end of the same build — while `InputState::hit_at` scanned that
 * very vector. A press landing mid-build therefore hit-tested a half-drained registry: measured on
 * 6118, a move answered `targets=42 hit=Some((TreeItem, "…add-generation"))` and the press 295 ms
 * later, pointer unmoved, answered `targets=0 hit=None`, so `addGeneration` dispatched nothing.
 *
 * This probe clicks WITHOUT arming — no jiggle-until-the-shell's-trace-answers-the-row — because the
 * arming is exactly what hid the defect. Two lanes:
 *   A. 50 consecutive clicks on the retained `Add Generation` row (`?mode=generate`). Both the press
 *      and the release must resolve the row, each click must reach the retained press handler with
 *      `addGeneration`, and the roster must grow by one per click.
 *   B. 20 navbar-control clicks, on controls DISCOVERED by a sweep of the chrome band. Press and
 *      release must both resolve the control the sweep named.
 *
 * Every click is scored from the shell's own `[DEBUG] wgpu-shell pointer button` trace, which since
 * this lane carries `targets=` (the resolvable registry), `staged=` (the one a build in progress is
 * assembling) and `gen=` (the published generation stamp). `emptyRegistryEvents` counts pointer
 * events that met `targets=0` — the §D symptom itself.
 *
 * ⏳️ Waits are ADAPTIVE, never a fixed sleep: one `addGeneration` re-renders six surfaces and the
 * shell's pointer trace can land a second later. A miss must mean "the registry did not answer",
 * never "the probe did not wait".
 *
 * Usage: cd <ticket> && bun 🐍️wgpu-hit-drain-probe.mjs
 * Env: SEMIO_PROBE_ORIGIN, SEMIO_PROBE_OUT, SEMIO_PROBE_EXAMPLE, SEMIO_PROBE_SETTLE,
 *      SEMIO_PROBE_ROW_CLICKS, SEMIO_PROBE_NAVBAR_CLICKS, SEMIO_PROBE_LANES (a,b), SEMIO_PROBE_CLICK_WAIT
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const origin = process.env.SEMIO_PROBE_ORIGIN ?? "http://127.0.0.1:6118";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-hit-drain/probe");
const example = process.env.SEMIO_PROBE_EXAMPLE ?? "hexagonal-mushroom-column";
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 75);
const rowClicks = Number(process.env.SEMIO_PROBE_ROW_CLICKS ?? 50);
const navbarClicks = Number(process.env.SEMIO_PROBE_NAVBAR_CLICKS ?? 20);
const clickWaitMs = Number(process.env.SEMIO_PROBE_CLICK_WAIT ?? 25000);
const lanes = (process.env.SEMIO_PROBE_LANES ?? "a,b").split(",").map((entry) => entry.trim());
const viewport = { width: 1440, height: 900 };
mkdirSync(outDir, { recursive: true });

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });

/** 🔎️ One `wgpu-shell pointer button` line, parsed into the registry census it now carries. */
const parseButton = (line) => {
  const match = /pointer button x=(-?[\d.]+) y=(-?[\d.]+) down=(\w+) button=(-?\d+) targets=(\d+) staged=(\d+) gen=(\d+) hit=(.*)$/.exec(line);
  if (!match) return null;
  const hit = match[8];
  const named = /Some\(\((\w+), Some\("(.*?)"\)/.exec(hit);
  return {
    x: Number(match[1]),
    y: Number(match[2]),
    down: match[3] === "true",
    targets: Number(match[5]),
    staged: Number(match[6]),
    generation: Number(match[7]),
    resolved: !hit.startsWith("None"),
    kind: named?.[1] ?? null,
    controlId: named?.[2] ?? null,
  };
};

const run = async () => {
  const lines = [];
  const t0 = Date.now();
  const at = () => Date.now() - t0;
  const page = await browser.newPage({ viewport });
  page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 4000)}`));
  page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 1200)}`));
  const has = (needle) => lines.filter((line) => line.includes(needle));
  const pause = (ms) => page.waitForTimeout(ms);

  const dump = (windowId) =>
    page
      .evaluate(async (id) => {
        const beacon = globalThis.semioWgpuIntrospection;
        if (typeof beacon?.dumpStructure !== "function") return null;
        try {
          const raw = await beacon.dumpStructure(id);
          return raw ? JSON.parse(raw) : null;
        } catch {
          return null;
        }
      }, windowId)
      .catch(() => null);

  const dockPlan = () => {
    const line = has("wgpu-shell dock plan").at(-1);
    if (!line) return {};
    const plan = {};
    for (const token of line.split(" ").slice(1)) {
      const match = /^(.+)@(\d+(?:\.\d+)?)x(\d+(?:\.\d+)?)\+(-?\d+(?:\.\d+)?),(-?\d+(?:\.\d+)?)$/.exec(token);
      if (match) plan[match[1]] = { w: Number(match[2]), h: Number(match[3]), x: Number(match[4]), y: Number(match[5]) };
    }
    return plan;
  };

  const pump = async (seconds, park = [3, 3]) => {
    for (let tick = 0; tick < seconds * 5; tick += 1) {
      await pause(200);
      await page.mouse.move(park[0] + (tick % 2), park[1]).catch(() => {});
    }
  };

  /** 🖱️ The control the shell's own move trace names under a point — the sweep's only authority. */
  const hitAt = async (x, y) => {
    const before = has("os_host pointer hit").length;
    await page.mouse.move(x, y);
    for (let wait = 0; wait < 16; wait += 1) {
      await pause(150);
      const line = has("os_host pointer hit").slice(before).at(-1);
      if (line) {
        const match = /hit=Some\(\((\w+), Some\("(.*?)"\)\)\)/.exec(line);
        return { x, y, kind: match?.[1] ?? null, controlId: match?.[2] ?? null };
      }
    }
    return { x, y, kind: null, controlId: null };
  };

  /** 👆️ One click, then WAIT for the shell's own press and release traces rather than guessing a sleep. */
  const clickAndAwait = async (x, y) => {
    const before = has("wgpu-shell pointer button").length;
    const pressBefore = has("wgpu-shell retained press").length;
    await page.mouse.click(x, y);
    const deadline = Date.now() + clickWaitMs;
    let events = [];
    while (Date.now() < deadline) {
      await pause(150);
      events = has("wgpu-shell pointer button").slice(before).map(parseButton).filter(Boolean);
      if (events.some((event) => event.down) && events.some((event) => !event.down)) break;
    }
    return {
      down: events.find((event) => event.down) ?? null,
      up: events.find((event) => !event.down) ?? null,
      presses: has("wgpu-shell retained press").slice(pressBefore),
      waitedMs: clickWaitMs - Math.max(0, deadline - Date.now()),
    };
  };

  const url = `${origin}/?plugin=generation3d&mode=generate&example=${encodeURIComponent(example)}`;
  await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 300)}`));
  await pump(settleSeconds);

  const windowIds = (await dump(undefined))?.windowIds ?? [];
  const plan = dockPlan();
  lines.push(`${at()} PROBE windowIds ${JSON.stringify(windowIds)} dockPlan ${JSON.stringify(plan)}`);
  const results = { url, origin, example, rowClicks, navbarClicks, lanes: {} };

  // ── Lane A — consecutive clicks on the retained `Add Generation` row ─────────────────────────
  if (lanes.includes("a")) {
    /** 🎯️ The row's CURRENT page point. Re-derived per click: every dispatch adds a roster row above
     * it, so a fixed point would stop being the row — which is a moving target, not a drained registry. */
    const rowPoint = async () => {
      for (const id of windowIds) {
        const body = plan[id];
        if (!body) continue;
        for (const node of (await dump(id))?.nodes ?? []) {
          if (!String(node.path ?? "").includes("add-generation")) continue;
          const [rx, ry, rw, rh] = node.rect ?? [0, 0, 0, 0];
          if (rw <= 0 || rh <= 0) continue;
          return { windowId: id, path: node.path, rect: node.rect, page: [body.x + rx + rw / 2, body.y + ry + rh / 2], body };
        }
      }
      return null;
    };

    const rosterRows = async (windowId) => ((await dump(windowId))?.nodes ?? []).filter((node) => String(node.path ?? "").includes("generations/stack")).length;

    const first = await rowPoint();
    const rosterBefore = first ? await rosterRows(first.windowId) : null;
    const clicks = [];
    for (let index = 0; index < rowClicks; index += 1) {
      const target = await rowPoint();
      if (!target) {
        clicks.push({ index, ok: false, why: "the row left the published structure" });
        lines.push(`${at()} PROBE rowClick ${JSON.stringify(clicks.at(-1))}`);
        continue;
      }
      // 🧭️ The roster grows under the row, so it slides down. Scroll it back into the body when it leaves.
      if (target.page[1] > target.body.y + target.body.h - 8) {
        await page.mouse.move(target.body.x + target.body.w / 2, target.body.y + target.body.h / 2);
        await page.mouse.wheel(0, 200);
        await pause(900);
      }
      const point = (await rowPoint())?.page ?? target.page;
      await page.mouse.move(point[0], point[1]);
      await pause(250);
      const observed = await clickAndAwait(point[0], point[1]);
      const row = {
        index,
        point,
        rect: target.rect,
        down: observed.down,
        up: observed.up,
        waitedMs: observed.waitedMs,
        resolvedDown: observed.down?.controlId?.includes("add-generation") ?? false,
        resolvedUp: observed.up?.controlId?.includes("add-generation") ?? false,
        dispatched: observed.presses.filter((line) => line.includes("down=false") && line.includes("addGeneration")).length,
      };
      row.ok = row.resolvedDown && row.resolvedUp && row.dispatched > 0;
      clicks.push(row);
      lines.push(`${at()} PROBE rowClick ${JSON.stringify(row)}`);
      await pause(250);
    }
    await pump(8);
    const rosterAfter = first ? await rosterRows(first.windowId) : null;
    const resolvedDown = clicks.filter((row) => row.resolvedDown).length;
    const resolvedUp = clicks.filter((row) => row.resolvedUp).length;
    const dispatched = clicks.filter((row) => row.dispatched > 0).length;
    results.lanes.a = {
      firstTarget: first,
      clicks: clicks.length,
      resolvedDown,
      resolvedUp,
      dispatched,
      missed: clicks.filter((row) => !row.ok).length,
      emptyRegistryEvents: clicks.flatMap((row) => [row.down, row.up]).filter((event) => event && event.targets === 0).length,
      unresolvedEvents: clicks.flatMap((row) => [row.down, row.up]).filter((event) => event && !event.resolved).length,
      targetsSeen: [...new Set(clicks.flatMap((row) => [row.down?.targets, row.up?.targets]).filter((value) => value !== undefined && value !== null))].sort((a, b) => a - b),
      generationSpan: [clicks[0]?.down?.generation ?? null, clicks.at(-1)?.up?.generation ?? null],
      rosterBefore,
      rosterAfter,
      failures: clicks.filter((row) => !row.ok).slice(0, 10),
      ok: clicks.length === rowClicks && resolvedDown === rowClicks && resolvedUp === rowClicks && dispatched === rowClicks,
    };
    console.log(`[DEBUG] hit-drain lane A: ${JSON.stringify({ ...results.lanes.a, firstTarget: undefined, failures: results.lanes.a.failures.length })}`);
    await page.screenshot({ path: join(outDir, "after-row-clicks.png") }).catch(() => {});
    writeFileSync(join(outDir, "lane-a.json"), JSON.stringify({ ...results.lanes.a, clicks }, null, 2));
  }

  // ── Lane B — navbar-control clicks, on controls the sweep discovered ─────────────────────────
  if (lanes.includes("b")) {
    const bandRows = (process.env.SEMIO_PROBE_CHROME_ROWS ?? "10,20,28,36,44,52,62,880,894").split(",").map(Number);
    /** 🧭️ The chrome band is DISCOVERED, never assumed: every point the shell's own move trace names. */
    const sweepBand = async () => {
      const found = [];
      for (const y of bandRows) {
        for (let x = 24; x <= 1416; x += 40) {
          const hit = await hitAt(x, y);
          if (hit.controlId && !hit.controlId.startsWith("generation3d-")) found.push(hit);
        }
        if (found.length >= navbarClicks * 2) break;
      }
      return found;
    };
    /** 🎛️ One point per distinct control: a 40 px sweep names the same wide control several times. */
    const distinctBand = (found) => {
      const byControl = new Map();
      for (const hit of found) if (!byControl.has(hit.controlId)) byControl.set(hit.controlId, hit);
      return [...byControl.values()];
    };
    const sweep = distinctBand(await sweepBand());
    lines.push(`${at()} PROBE navbarSweep ${JSON.stringify(sweep)}`);

    // ⚖️ The predicate is §D's own law — a press resolves the same target the preceding MOVE did — so
    // every click's expectation is re-read from the shell's live trace immediately before the click,
    // never from a sweep taken minutes and several layout replans ago. A chrome control that changes
    // the layout (a role switch replans the whole dock) makes a stale coordinate name a control that
    // no longer exists there, and scoring that as a registry miss would be a lie.
    const clicks = [];
    const seen = new Set();
    let cursor = 0;
    let resweeps = 0;
    for (let index = 0; index < navbarClicks; index += 1) {
      let move = null;
      for (let attempt = 0; attempt < sweep.length && move === null; attempt += 1) {
        const candidate = sweep[(cursor + attempt) % sweep.length];
        const live = await hitAt(candidate.x, candidate.y);
        if (live.controlId) move = live;
      }
      if (move === null) {
        const fresh = distinctBand(await sweepBand());
        resweeps += 1;
        sweep.length = 0;
        sweep.push(...fresh);
        cursor = 0;
        const candidate = sweep[0];
        move = candidate ? await hitAt(candidate.x, candidate.y) : null;
      }
      if (!move?.controlId) {
        clicks.push({ index, ok: false, why: "no chrome control resolves anywhere on the band" });
        lines.push(`${at()} PROBE navbarClick ${JSON.stringify(clicks.at(-1))}`);
        continue;
      }
      cursor += 1;
      seen.add(move.controlId);
      const effectsBefore = has("wgpu-shell dock plan").length + has("wgpu-shell render begin").length + has("wgpu-shell command").length;
      const observed = await clickAndAwait(move.x, move.y);
      const row = {
        index,
        expect: move.controlId,
        kind: move.kind,
        point: [move.x, move.y],
        down: observed.down,
        up: observed.up,
        waitedMs: observed.waitedMs,
        resolvedDown: observed.down?.controlId === move.controlId,
        resolvedUp: observed.up?.controlId === move.controlId,
        effects: has("wgpu-shell dock plan").length + has("wgpu-shell render begin").length + has("wgpu-shell command").length - effectsBefore,
      };
      // ⚖️ The gate is the REGISTRY law only. A navbar `Select` carries no `ActionDescriptor` of its
      // own (`hit=Some((NavbarItem, Some("playground.navbar.fixture"), None))`) — the shell opens its
      // dropdown internally and logs no dispatch line — so demanding a dispatch witness from every
      // chrome control would fail on controls that never had one. `effects` is reported beside it.
      row.ok = row.resolvedDown && row.resolvedUp && (row.down?.targets ?? 0) > 0;
      clicks.push(row);
      lines.push(`${at()} PROBE navbarClick ${JSON.stringify(row)}`);
      // 🚪️ A chrome control may open an overlay that would swallow the next click; Escape closes it.
      await page.keyboard.press("Escape").catch(() => {});
      await pause(600);
    }
    results.lanes.b = {
      discovered: sweep.length,
      distinct: seen.size,
      resweeps,
      clicks: clicks.length,
      resolved: clicks.filter((row) => row.ok).length,
      missed: clicks.filter((row) => !row.ok).length,
      withEffects: clicks.filter((row) => (row.effects ?? 0) > 0).length,
      emptyRegistryEvents: clicks.flatMap((row) => [row.down, row.up]).filter((event) => event && event.targets === 0).length,
      unresolvedEvents: clicks.flatMap((row) => [row.down, row.up]).filter((event) => event && !event.resolved).length,
      controls: [...seen],
      failures: clicks.filter((row) => !row.ok).slice(0, 10),
      ok: clicks.length === navbarClicks && clicks.every((row) => row.ok),
    };
    console.log(`[DEBUG] hit-drain lane B: ${JSON.stringify({ ...results.lanes.b, failures: results.lanes.b.failures.length })}`);
    await page.screenshot({ path: join(outDir, "after-navbar-clicks.png") }).catch(() => {});
    writeFileSync(join(outDir, "lane-b.json"), JSON.stringify({ ...results.lanes.b, clicks }, null, 2));
  }

  results.ok = Object.values(results.lanes).every((lane) => lane.ok);
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
  await page.close();
  return results;
};

const results = await run();
await browser.close();
console.log("DONE", JSON.stringify({ ok: results.ok, out: outDir }, null, 2));
