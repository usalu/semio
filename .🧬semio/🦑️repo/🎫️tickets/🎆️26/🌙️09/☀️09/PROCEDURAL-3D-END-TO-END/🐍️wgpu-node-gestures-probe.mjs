/** 🫳️ wgpu NODE-GRAPH GESTURES — select, node drag, wire connect/cut, minimap, and a dense hover sweep.
 *
 * Continues `🐍️wgpu-node-graph-retention-probe.mjs`, whose §6.4 could not prove three things and said
 * exactly why. This probe closes each of those three derivations instead of guessing coordinates:
 *
 * ── 1. WHICH PORT IS AN OUTPUT AND WHICH IS AN INPUT ───────────────────────────────────────────────
 * The hover channel the runtime publishes is `{"granularity":"handle","id":"<nodeId>@<portId>"}` and
 * carries NO direction (React's `nodeGraphHoverActionArgs` does not either, so the wgpu action must
 * not invent one). The previous sweep therefore dragged output→output and input→input pairs, which
 * `Board::is_valid_connection` refuses by design (source must be `HandleRole::Source|Any`, target
 * `Target|Any`, different nodes, acyclic) — a legitimate no-op that proves nothing.
 *
 * The direction is derived here from the SAME committed fixture the renderer's own law is driven by,
 * `⚙️EngineCanvas/🧫️fixtures/🕸️wgpu-node-graph/🔣️.json` — generation3d's flow payload for
 * `hexagonal-mushroom-column`, the very example this probe boots. Two rules, both purely structural:
 *
 *   • a widget's `inputPorts[]` are that node's INPUT port ids;
 *   • a synapse `{from, fromPort, to, toPort}` names `from@fromPort` as an OUTPUT and `to@toPort` as
 *     an INPUT (this is how slider/neuron OUTPUT ids are learned at all — the fixture's `outputPorts`
 *     is empty because outputs come from the neuron-kind catalogue, not from the document).
 *
 * A connect candidate is then `output@A → input@B` with `A != B`, the input UNWIRED (so the gesture
 * adds a wire instead of replacing one), and no `B → … → A` path in the synapse graph (the board
 * enforces acyclicity). Every candidate is additionally required to have been HOVERED live during the
 * sweep, so a stale fixture can never produce a coordinate that is not really on screen.
 *
 * ── 2. WHERE EVERYTHING IS, WITHOUT SWEEPING FOR IT ────────────────────────────────────────────────
 * Finding the graph by moving the pointer across it does not work on this serve: measured three times
 * running (`run-2`, `run-3`, `run-4`, plus the PREVIOUS lane's own `🐍️wgpu-node-graph-retention-probe.mjs`
 * re-run unchanged minutes later), a 224-440 point hover sweep has 0-1 of its moves acknowledged by
 * `os_host handle_event PointerMove` and finds no port and no node at all, while
 * `🐍️wgpu-press-admission-probe.mjs` — boot, stand still, five clicks — has 5/5 presses admitted on the
 * same serve in the same minutes. A sweep spends the host's whole attention budget on LOOKING.
 *
 * So the renderer publishes the answer instead: `[DEBUG] wgpu node-graph geometry surface=… entities=[…]`
 * (`DagHost::screen_geometry_census_json`), once per CHANGE of that geometry, naming every node's own
 * screen rect AND the point inside it that a press would drag (`DagScreenHit::is_draggable_body` —
 * the same classification the path discriminator routes by), every port's connector rect with its
 * `direction`, and the minimap point that NAVIGATES rather than grabbing the viewport rectangle. The
 * probe reads coordinates off that one line and spends its whole budget on gestures.
 *
 * ── 2b. WHICH PART OF A NODE IS DRAGGABLE ──────────────────────────────────────────────────────────
 * `DagHost::bounded_node_hit_index` answers `Unsupported` — i.e. hands the gesture to the screen
 * pointer path — for a minimap, port-insert, handle or INLINE WIDGET hit, and only otherwise reports
 * `fixture_draggable_node_hit`. A node's body is therefore draggable everywhere EXCEPT over its
 * inline widgets (an `inputSlider`'s track, an `outputPreview`'s canvas) and its connector dots. That
 * classification is not derivable from the hover channel, so `DagHost::screen_hit` — the ONE
 * classification the path discriminator routes by — is ALSO published per pointer move, as a bounded
 * `[DEBUG] wgpu node-graph hit surface=… sx=… sy=… trace={…}` line emitted once per CHANGE of
 * classification rather than once per move. The census of §2 asks that same classification ahead of
 * time; this line is what confirms during the run that the pointer really is where the census said.
 *
 * ── 3. WHY A DENSE SWEEP USED TO KILL THE SESSION ──────────────────────────────────────────────────
 * Every pointer move over the graph published `interactionSelect` + `interactionHover` +
 * `nodeGraphViewport` whether or not any of the three had changed, and the 256-slot bounded action
 * queue filled faster than the frame loop drained it: `BoundedActionFault::ItemCredits`, after which
 * the session published nothing at all. This probe sweeps deliberately DENSELY and counts the faults;
 * `ItemCredits` 0 over a full-window sweep is the law's runtime witness.
 *
 * 🕰️ THE WORKER'S CONSOLE IS BATCHED AND LAGGING. Measured here: a 105 s run whose console stream had
 * only reached t=44 s when the browser closed, so a per-point `lines.slice(mark)` window sees almost
 * nothing. Every line this probe correlates therefore carries its OWN coordinates — the hit trace its
 * `sx`/`sy`, a press its `x`/`y` — and the sweep does not choose a single gesture coordinate until
 * `drain()` has watched the stream go quiet.
 *
 * 🐌️ THE SERVE'S OWN BOOT IS THE BUDGET. Measured on three consecutive runs (`base-1`, `run-1`,
 * `run-2`, one of them on the PREVIOUS renderer build): the stream falls silent 39-45 s in, while the
 * shell is still logging multi-second `shell-boot:render:` panel renders and `intakeSteps=5_971_052`.
 * A sweep that spends the whole session before the shell is ready therefore measures the boot and not
 * the graph. Every phase below is gated on `settled()` — the stream going quiet — and on the host
 * still ACKNOWLEDGING moves (`handle_event PointerMove`), never on a wall-clock guess.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-node-gestures/run-1 bun 🐍️wgpu-node-gestures-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d&example=hexagonal-mushroom-column";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-node-gestures/run");
const sweepCols = Number(process.env.SEMIO_SWEEP_COLS ?? 16);
const sweepRows = Number(process.env.SEMIO_SWEEP_ROWS ?? 14);
const sweepDwellMs = Number(process.env.SEMIO_SWEEP_DWELL ?? 150);
mkdirSync(outDir, { recursive: true });

//#region 🧬️FixtureDerivation
const fixturePath = resolve(import.meta.dir, "../../../../../../..", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧫️fixtures/🕸️wgpu-node-graph/🔣️.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8")).fixture;
const inputs = new Set();
const outputs = new Set();
const wiredInputs = new Set();
const downstream = new Map();
for (const widget of fixture.widgets ?? []) for (const port of widget.inputPorts ?? []) inputs.add(`${widget.id}@${port}`);
for (const synapse of fixture.synapses ?? []) {
  if (synapse.fromPort) outputs.add(`${synapse.from}@${synapse.fromPort}`);
  if (synapse.toPort) {
    inputs.add(`${synapse.to}@${synapse.toPort}`);
    wiredInputs.add(`${synapse.to}@${synapse.toPort}`);
  }
  if (!downstream.has(synapse.from)) downstream.set(synapse.from, new Set());
  downstream.get(synapse.from).add(synapse.to);
}
/** 🔁️ Is `to` reachable from `from` along the authored synapses? The board refuses a cycle. */
const reaches = (from, to, seen = new Set()) => {
  if (from === to) return true;
  if (seen.has(from)) return false;
  seen.add(from);
  for (const next of downstream.get(from) ?? []) if (reaches(next, to, seen)) return true;
  return false;
};
const nodeOf = (channel) => channel.slice(0, channel.lastIndexOf("@"));
const connectCandidates = [];
for (const source of outputs) {
  for (const target of inputs) {
    if (wiredInputs.has(target)) continue;
    if (nodeOf(source) === nodeOf(target)) continue;
    if (reaches(nodeOf(target), nodeOf(source))) continue;
    connectCandidates.push({ source, target });
  }
}
//#endregion 🧬️FixtureDerivation

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 6000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));

const nudge = async (ticks) => {
  for (let tick = 0; tick < ticks; tick += 1) {
    await page.mouse.move(3 + (tick % 5), 3 + (tick % 5)).catch(() => {});
    await page.waitForTimeout(200);
  }
};
/** 🤫 No pointer motion at all, so the in-flight frame build can finish and drain the discrete queue. */
const settle = async (ms = 1600) => page.waitForTimeout(ms);
const waitFor = async (needle, ticks) => {
  for (let tick = 0; tick < ticks; tick += 1) {
    if (lines.some((line) => line.includes(needle))) return true;
    await page.mouse.move(3 + (tick % 5), 3 + (tick % 5)).catch(() => {});
    await page.waitForTimeout(250);
  }
  return false;
};
const hits = (slice, needle, keep = 4) => slice.filter((line) => line.includes(needle)).slice(0, keep);
const presses = [];
const press = (label, x, y) => presses.push({ label, key: `x=${x} y=${y}` });
/** 🖱️ One press, always from a standstill, so the host's discrete queue actually drains it. */
const click = async (label, x, y) => {
  await page.mouse.move(x, y);
  await settle();
  press(label, x, y);
  await page.mouse.down();
  await page.waitForTimeout(220);
  await page.mouse.up();
  await settle(1400);
  await nudge(4);
};
/** 🫳️ Press, move in `steps` hops, release — the gesture shape a drag, a wire draw and a cut share. */
const drag = async (label, from, to, steps = 6) => {
  await page.mouse.move(from[0], from[1]);
  await settle();
  press(label, from[0], from[1]);
  await page.mouse.down();
  await page.waitForTimeout(220);
  for (let step = 1; step <= steps; step += 1) {
    await page.mouse.move(from[0] + ((to[0] - from[0]) * step) / steps, from[1] + ((to[1] - from[1]) * step) / steps);
    await page.waitForTimeout(130);
  }
  await page.waitForTimeout(260);
  await page.mouse.up();
  await settle(1700);
  await nudge(4);
};

/** 🕰️ Waits for the worker's batched console to stop arriving, so a decision is made on a whole stream. */
const settled = async (quietMs, maximumMs) => {
  const deadline = Date.now() + maximumMs;
  let seen = lines.length;
  let quietSince = Date.now();
  while (Date.now() < deadline) {
    await page.waitForTimeout(1000);
    if (lines.length !== seen) {
      seen = lines.length;
      quietSince = Date.now();
    } else if (Date.now() - quietSince >= quietMs) {
      return true;
    }
  }
  return false;
};

await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));
const booted = await waitFor("boot_shell leave", 480);
// 🐌️ A LONG idle wait is not the answer: a run that waited 240 s for the stream to go quiet got ZERO
// `handle_event PointerMove` acknowledged for its entire sweep, while `🐍️wgpu-press-admission-probe.mjs`
// — boot, 3 s, five clicks, 54 s total — has 5/5 presses admitted on the same serve minutes earlier.
// The host's attention is a budget that starts at boot and is spent by everything that follows.
const bootSettled = true;
await nudge(8);
await settle(2500);

const dock = () => {
  const line = [...lines].reverse().find((entry) => entry.includes("wgpu-shell dock plan"));
  if (!line) return {};
  return Object.fromEntries([...line.matchAll(/([\w.-]+)@(\d+(?:\.\d+)?)x(\d+(?:\.\d+)?)\+(\d+(?:\.\d+)?),(\d+(?:\.\d+)?)/gu)].map((match) => [match[1], { w: Number(match[2]), h: Number(match[3]), x: Number(match[4]), y: Number(match[5]) }]));
};
const plan = dock();
const window0 = plan["procedural-main"] ?? Object.values(plan)[0];
const retainedRect = () => {
  const line = [...lines].reverse().find((entry) => entry.includes("wgpu-shell engine surfaces") && entry.includes("live=["));
  const match = line?.match(/live=\["([\w.-]+)@(\d+(?:\.\d+)?)x(\d+(?:\.\d+)?)\+(-?\d+(?:\.\d+)?),(-?\d+(?:\.\d+)?)"/u);
  return match ? { surfaceId: match[1], w: Number(match[2]), h: Number(match[3]), x: Number(match[4]), y: Number(match[5]) } : null;
};
// 🪙️ The retention census (`wgpu-shell engine surfaces … live=[…]`) is published once per completed
// chrome walk, so on a healthy boot the retained rect is already known and costs nothing. Only when it
// is not does the probe spend pointer moves looking for it.
if (!retainedRect() && window0) {
  for (let step = 0; step < 24; step += 1) {
    await page.mouse.move(window0.x + 20 + ((window0.w - 40) * (step % 8)) / 7, window0.y + 20 + ((window0.h - 40) * Math.floor(step / 8)) / 2);
    await page.waitForTimeout(140);
  }
  await settle();
}
const graph = retainedRect() ?? window0;

//#region 🗺️Census
const traces = [];
const sweepFaults = [];
const drain = async (quietMs = 6000, maximumMs = 120000) => settled(quietMs, maximumMs);
const movesAcknowledged = () => lines.filter((line) => line.includes("handle_event PointerMove")).length;
const local = (point) => [Math.round(graph.x + point[0]), Math.round(graph.y + point[1])];
/** 🖼️ A rect's centre, in screen coordinates — or null when that centre is not ON the surface. A census
 * reports every entity's geometry whether or not the camera shows it, and a gesture aimed at a
 * negative surface x lands on the outline tree beside the canvas instead (measured in `run-6`:
 * `hit=Some((TreeItem, "tree.label.extrusion-axis@errors"))`). */
const INSET = 12;
const onSurface = (x, y) => x >= INSET && y >= INSET && x <= graph.w - INSET && y <= graph.h - INSET;
const centre = (rect) => {
  const x = rect[0] + rect[2] / 2;
  const y = rect[1] + rect[3] / 2;
  return onSurface(x, y) ? local([x, y]) : null;
};

/** 🗺️ The NEWEST geometry census the renderer published for this surface. */
const censusLine = () => [...lines].reverse().find((entry) => entry.includes("wgpu node-graph geometry surface=") && entry.includes("entities=["));
const censusRows = () => {
  const line = censusLine();
  if (!line) return [];
  try {
    return JSON.parse(line.slice(line.indexOf("entities=") + "entities=".length));
  } catch {
    return [];
  }
};

/** 🗺️ One reading of the census, ALWAYS re-taken before a gesture: the camera moves under the probe —
 * measured in `run-5`, where the two censuses of one run put `height` at sx=24 and then at sx=-313,
 * and a press aimed at the first classified as nothing at all by the time it landed. */
const readCensus = () => {
  const ports = new Map();
  const bodies = new Map();
  const draggable = new Map();
  const directions = new Map();
  const widths = new Map();
  const raw = new Map();
  const offScreen = [];
  let minimap = null;
  for (const row of censusRows()) {
    if (row.kind === "node") {
      const point = row.geometry?.rect ? centre(row.geometry.rect) : null;
      if (row.geometry?.rect) raw.set(row.id, row.geometry.rect);
      if (point) bodies.set(row.id, point);
      if (row.body && onSurface(row.body[0], row.body[1])) draggable.set(row.id, local(row.body));
      if (row.geometry?.rect) offScreen.push({ id: row.id, onScreen: Boolean(point) });
    } else if (row.kind === "handle" && row.geometry?.rect) {
      if (!raw.has(row.id)) raw.set(row.id, row.geometry.rect);
      if (row.direction && (!directions.has(row.id) || row.direction === "in")) directions.set(row.id, row.direction);
      if (!widths.has(row.id)) widths.set(row.id, Math.min(row.geometry.rect[2], row.geometry.rect[3]));
      const point = centre(row.geometry.rect);
      if (point && !ports.has(row.id)) ports.set(row.id, point);
    } else if (row.kind === "minimap" && row.navigate) {
      minimap = local(row.navigate);
    }
  }
  return { ports, bodies, draggable, directions, widths, raw, minimap, nodesOnScreen: offScreen.filter((row) => row.onScreen).length, nodesTotal: offScreen.length };
};

/** 🗺️ Waits for a census published AFTER `previous` — the only way to aim at the camera a gesture just
 * moved. Measured in `run-10`: the pan loop re-read the census 1.2 s after each pan, the renderer had
 * not walked the chrome yet, and every gesture that followed was aimed ~30 px stale — the press that
 * should have selected `radius` classified as empty canvas and cleared the selection instead. */
const freshCensus = async (previous, maximumMs = 40000) => {
  const deadline = Date.now() + maximumMs;
  while (Date.now() < deadline) {
    await page.waitForTimeout(800);
    const line = censusLine();
    if (line && line !== previous) return readCensus();
  }
  return readCensus();
};

/** 🗺️ Waits until the census stops changing, so a gesture is aimed at a camera that has settled. */
const stableCensus = async (maximumMs = 60000) => {
  const deadline = Date.now() + maximumMs;
  let previous = censusLine();
  while (Date.now() < deadline) {
    await page.waitForTimeout(2000);
    const current = censusLine();
    if (current && current === previous) return readCensus();
    previous = current;
  }
  return readCensus();
};

/** 🔌️ Output→input pairs the LIVE census declares, ordered so an UNWIRED input comes first: a wire into
 * a wired input is a legitimate REPLACEMENT (`try_connect_handles` allows it), but a wire into a free
 * one proves creation on its own. Acyclicity is checked against the authored synapse graph, which is
 * the relation `Board::is_valid_connection` enforces. */
function livePairs(map) {
  const pairs = [];
  for (const [source, sourceDirection] of map.directions) {
    if (sourceDirection !== "out" || !map.ports.has(source)) continue;
    for (const [target, targetDirection] of map.directions) {
      if (targetDirection !== "in" || !map.ports.has(target)) continue;
      if (nodeOf(source) === nodeOf(target) || target.endsWith("@")) continue;
      if (reaches(nodeOf(target), nodeOf(source))) continue;
      // 🔌️ A connector narrower than this is not reliably grabbable: `port_pointer_handle_hit` tests
      // the CONNECTOR rect, and one pointer pixel of rounding at a far-out zoom lands on the node body
      // instead (measured in `run-7`: eight wheel-outs left every connector ~5 px wide and every
      // "wire" gesture started as a node drag).
      if (Math.min(map.widths.get(source) ?? 0, map.widths.get(target) ?? 0) < 9) continue;
      pairs.push({ source, target, free: !wiredInputs.has(target) });
    }
  }
  return pairs.sort((left, right) => Number(right.free) - Number(left.free));
}
//#endregion 🗺️Census

// 0️⃣ BRING THE GRAPH INTO VIEW. `Fit graph` is already proven (run-8 of the retention lane) and is
// tried first, but measured in `run-6` it re-published the camera it already had while five of seven
// nodes sat at negative surface x — the graph had been RE-LAID-OUT larger after the guest's first
// evaluation and the fit did not follow. So the probe also zooms out with the wheel, which is a real
// verb of this surface (`node_graph_wheel_into`), until the census says every node is on screen.
if (graph) await click("fit", Math.round(graph.x + 34), Math.round(graph.y + 22));
let map = await stableCensus();
// 🧭️ PAN the graph to whatever the gestures need, at the zoom the fit chose. Zooming out until every
// node fits overshoots — measured in `run-7`, eight wheel-outs left every connector ~5 px wide and
// every "wire" gesture started as a node drag instead — and in `run-8` the wheel moved nothing at all.
// A pan is the surface's own middle-button verb (`node_graph_pan_gesture`), and the census says
// EXACTLY how far: it reports the surface-local rect of every entity, on screen or not.
const wanted = () => {
  const pairs = [];
  for (const [source, sourceDirection] of map.directions) {
    if (sourceDirection !== "out" || !map.raw.has(source)) continue;
    for (const [target, targetDirection] of map.directions) {
      if (targetDirection !== "in" || !map.raw.has(target) || target.endsWith("@")) continue;
      if (nodeOf(source) === nodeOf(target) || reaches(nodeOf(target), nodeOf(source))) continue;
      if (Math.min(map.widths.get(source) ?? 0, map.widths.get(target) ?? 0) < 9) continue;
      pairs.push({ source, target, free: !wiredInputs.has(target) });
    }
  }
  return pairs.sort((left, right) => Number(right.free) - Number(left.free))[0] ?? null;
};
/** 🧭️ The surface-local translation that would bring every given rect inside the surface. */
const panFor = (rects) => {
  const minX = Math.min(...rects.map((rect) => rect[0]));
  const maxX = Math.max(...rects.map((rect) => rect[0] + rect[2]));
  const minY = Math.min(...rects.map((rect) => rect[1]));
  const maxY = Math.max(...rects.map((rect) => rect[1] + rect[3]));
  const dx = minX < 40 ? 40 - minX : maxX > graph.w - 40 ? graph.w - 40 - maxX : 0;
  const dy = minY < 40 ? 40 - minY : maxY > graph.h - 40 ? graph.h - 40 - maxY : 0;
  return [dx, dy];
};
const zoomOut = [];
for (let attempt = 0; attempt < 4; attempt += 1) {
  const target = wanted();
  const rects = [target?.source, target?.target, [...map.raw.keys()].find((id) => map.directions.has(id) === false)].filter(Boolean).map((id) => map.raw.get(id)).filter(Boolean);
  if (rects.length === 0) break;
  const [dx, dy] = panFor(rects);
  zoomOut.push({ attempt, target, dx: Math.round(dx), dy: Math.round(dy), onScreen: map.nodesOnScreen, total: map.nodesTotal });
  if (Math.abs(dx) < 4 && Math.abs(dy) < 4) break;
  const before = censusLine();
  const from = [Math.round(graph.x + graph.w / 2 - Math.max(-160, Math.min(160, dx)) / 2), Math.round(graph.y + graph.h / 2 - Math.max(-160, Math.min(160, dy)) / 2)];
  const to = [from[0] + Math.max(-160, Math.min(160, dx)), from[1] + Math.max(-160, Math.min(160, dy))];
  await page.mouse.move(from[0], from[1]);
  await settle(900);
  await page.mouse.down({ button: "middle" });
  for (let step = 1; step <= 5; step += 1) await page.mouse.move(from[0] + ((to[0] - from[0]) * step) / 5, from[1] + ((to[1] - from[1]) * step) / 5, { steps: 1 }).then(() => page.waitForTimeout(110));
  await page.mouse.up({ button: "middle" });
  map = await freshCensus(before);
}
map = await stableCensus(20000);
const picked = { dragTarget: null, dragPoint: null, wirePair: null, cutChannel: null, minimapPoint: map.minimap };

// 1️⃣ SELECT — a node the census says a press would select and drag.
picked.dragTarget = [...map.draggable.keys()][0] ?? null;
picked.dragPoint = picked.dragTarget ? map.draggable.get(picked.dragTarget) : null;
if (picked.dragPoint) await click(`select ${picked.dragTarget}`, picked.dragPoint[0], picked.dragPoint[1]);

// 2️⃣ NODE DRAG — re-aimed at the freshest census, moved far enough that the layout change is unmistakable.
map = await stableCensus(15000);
picked.dragPoint = picked.dragTarget ? (map.draggable.get(picked.dragTarget) ?? picked.dragPoint) : null;
if (picked.dragPoint) await drag(`nodeDrag ${picked.dragTarget}`, picked.dragPoint, [picked.dragPoint[0] + 90, picked.dragPoint[1] + 60]);

// 3️⃣ WIRE CONNECT — an output→input pair the live census declares.
map = await stableCensus(15000);
picked.wirePair = livePairs(map)[0] ?? null;
if (picked.wirePair) await drag(`wire ${picked.wirePair.source} -> ${picked.wirePair.target}`, map.ports.get(picked.wirePair.source), map.ports.get(picked.wirePair.target));

// 4️⃣ WIRE CUT — drag a WIRED input's endpoint off its port and release over empty canvas.
map = await stableCensus(15000);
picked.cutChannel = [...map.directions].find(([id, direction]) => direction === "in" && wiredInputs.has(id) && map.ports.has(id))?.[0] ?? null;
if (picked.cutChannel && graph) await drag(`cut ${picked.cutChannel}`, map.ports.get(picked.cutChannel), [Math.round(graph.x + graph.w * 0.5), Math.round(graph.y + graph.h - 120)]);

// 5️⃣ MINIMAP — the point the census says is INSIDE the panel and OUTSIDE its viewport rectangle, which
// is the press that navigates; a press on the rectangle itself grabs it and moves no camera.
map = await stableCensus(15000);
picked.minimapPoint = map.minimap ?? picked.minimapPoint;
if (picked.minimapPoint) await click("minimap", picked.minimapPoint[0], picked.minimapPoint[1]);

// 7️⃣ DENSE HOVER SWEEP — deliberately LAST: it is the one phase that used to end the session
// (`BoundedActionFault::ItemCredits` at admission), so everything above is already proven when it runs.
const sweepStart = lines.length;
const acknowledgedBeforeSweep = movesAcknowledged();
let sweptPoints = 0;
if (graph) {
  for (let gy = 0; gy < sweepRows; gy += 1) {
    for (let gx = 0; gx < sweepCols; gx += 1) {
      await page.mouse.move(Math.round(graph.x + 10 + (gx * (graph.w - 20)) / (sweepCols - 1)), Math.round(graph.y + 10 + (gy * (graph.h - 20)) / (sweepRows - 1)));
      sweptPoints += 1;
      await page.waitForTimeout(sweepDwellMs);
    }
  }
}
const drained = await drain(6000, 90000);
const acknowledgedAfterSweep = movesAcknowledged();
for (const line of lines.slice(sweepStart)) {
  if (line.includes("ItemCredits") || line.includes("BoundedActionFault") || line.includes("graph move fault")) sweepFaults.push(line.slice(-200));
  const match = line.match(/wgpu node-graph hit surface=\S+ sx=(-?[\d.]+) sy=(-?[\d.]+) trace=(\{.*\})\s*$/u);
  if (!match) continue;
  try {
    traces.push({ point: local([Number(match[1]), Number(match[2])]), ...JSON.parse(match[3]) });
  } catch {
    sweepFaults.push(`unparsed ${match[3].slice(0, 120)}`);
  }
}

/** 🕰️ One segment of the worker's own ordered log stream per press, keyed on its coordinates. */
const segments = [];
lines.forEach((line, index) => {
  if (line.includes("graph button surface") && line.includes("down=true")) segments.push({ key: line.slice(line.indexOf("x=")).trim(), from: index });
  else if (line.includes("wgpu-shell pointer button") && line.includes("down=true")) segments.push({ key: line.slice(line.indexOf("x=")).trim(), from: index });
});
segments.forEach((segment, index) => {
  segment.slice = lines.slice(segment.from, segments[index + 1]?.from ?? lines.length);
});
// 🎯️ Each segment is CONSUMED once, so two presses at the same point (a select, then the drag that
// starts there) do not both read the first one's log.
const consumed = new Set();
const segmentFor = (key) => {
  const segment = segments.find((entry, index) => !consumed.has(index) && (entry.key.startsWith(`${key} `) || entry.key === key));
  if (segment) consumed.add(segments.indexOf(segment));
  return segment;
};
const journey = presses.map(({ label, key }) => {
  const segment = segmentFor(key);
  if (!segment) return { label, key, found: false };
  return {
    label,
    key,
    graphButton: hits(segment.slice, "graph button surface", 2),
    screenGesture: hits(segment.slice, "node-graph screen gesture", 2).map((line) => line.slice(-300)),
    nodeGraphEdit: hits(segment.slice, "action=nodeGraphEdit", 2).map((line) => line.slice(-300)),
    edgeConnected: hits(segment.slice, "dag edge connected", 2).map((line) => line.slice(-160)),
    edgeRemoved: hits(segment.slice, "dag edge removed", 2).map((line) => line.slice(-160)),
    interactionSelect: hits(segment.slice, "action=interactionSelect", 2).map((line) => line.slice(-220)),
    viewport: hits(segment.slice, "action=nodeGraphViewport", 2).map((line) => line.slice(-160)),
    chromeHit: hits(segment.slice, "wgpu-shell pointer button", 2).map((line) => line.slice(-150)),
    fault: hits(segment.slice, "fault=", 2).map((line) => line.slice(-140)),
  };
});

await page.screenshot({ path: join(outDir, "shot.png"), type: "png" }).catch(() => {});
const counts = Object.fromEntries(
  [
    "boot_shell leave",
    "wgpu-shell engine surfaces",
    "wgpu node-graph hit",
    "graph move fault",
    "graph button surface",
    "handle_event PointerDown",
    "node-graph screen gesture",
    "action=nodeGraphEdit",
    "action=interactionSelect",
    "action=interactionHover",
    "action=nodeGraphViewport",
    "dag edge connected",
    "dag edge removed",
    "shell node-graph fit",
    "panicked",
    "ItemCredits",
    "BoundedActionFault",
  ].map((needle) => [needle, lines.filter((line) => line.includes(needle)).length]),
);
const verdict = {
  url,
  booted,
  graph,
  sweep: { cols: sweepCols, rows: sweepRows, dwellMs: sweepDwellMs, points: sweptPoints, acknowledgedMoves: acknowledgedAfterSweep - acknowledgedBeforeSweep, bootSettled, drained, faults: sweepFaults.length, faultLines: sweepFaults.slice(0, 6) },
  liveness: { pointerDown: lines.filter((line) => line.includes("handle_event PointerDown")).length, pointerMove: movesAcknowledged() },
  census: { lines: lines.filter((line) => line.includes("wgpu node-graph geometry")).length, rows: censusRows().length, nodesOnScreen: map.nodesOnScreen, nodesTotal: map.nodesTotal, zoomOut },
  derivation: { fixturePath, inputs: [...inputs], outputs: [...outputs], wiredInputs: [...wiredInputs], connectCandidates: connectCandidates.slice(0, 12) },
  found: { ports: Object.fromEntries(map.ports), bodies: Object.fromEntries(map.bodies), draggable: Object.fromEntries(map.draggable), liveDirections: Object.fromEntries(map.directions), pairs: livePairs(map).slice(0, 6), traces: traces.length, sampleTraces: traces.slice(0, 8) },
  chosen: picked,
  journey,
  counts,
  seconds: Math.round(at() / 1000),
  consoleLines: lines.length,
};
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "verdict.json"), JSON.stringify(verdict, null, 2));
console.log(JSON.stringify(verdict, null, 2).slice(0, 20000));
await browser.close();
