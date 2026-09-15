/** 🧾️ Flow SURFACE cost probe for the procedural 3d REACT door — what a boot and a click cost the
 * plugin, as opposed to what a gesture costs the renderer (`🐍️flow-scroll-render-perf-probe.mjs`).
 *
 * Three readings, one boot each:
 *
 * 1. 🪞 **Surface attaches.** `node-graph surface ready` per page. A board that attaches twice pays
 *    every per-attach cost twice; the line is the host's own, emitted once per successful
 *    `attachCanvas`.
 * 2. 🛰️ **Click hops.** Three clicks after the shell is quiet — empty canvas, a node, the SAME node
 *    again — each counted by `semio.hop.invoke` `actionId`. A plain click that changed nothing owes
 *    the plugin nothing; a click that moved the selection owes exactly one `interactionSelect`.
 * 3. 📦️ **Flow payload bytes.** The `[DEBUG] flow payload <feature> bytes=<n> ref=<0|1>` line
 *    `sendFlowPayloadOnce` emits per delivery: how many bytes each app-static payload actually moved
 *    across the ABI for this page, and how many crossings were content-addressed references.
 *
 * 🧭️ Quiescence, the page clock and the hop-span reading are all taken from
 * `🐍️flow-scroll-render-perf-probe.mjs` — same technique, same reason (a boot tail is not a gesture).
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6022/?plugin=generation3d SEMIO_PROBE_OUT=flow-surface/before bun 🐍️flow-surface-followup-probe.mjs
 *
 * @see `🐍️flow-scroll-render-perf-probe.mjs` — the hop-span and quiescence technique this reuses
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6022/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "flow-surface/cost");
const bootWaitS = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 90);
const attachWatchMs = Number(process.env.SEMIO_PROBE_ATTACH_WATCH_MS ?? 30_000);
const clickSettleMs = Number(process.env.SEMIO_PROBE_CLICK_SETTLE_MS ?? 1500);
const gate = process.env.SEMIO_PROBE_GATE === "1";
const examples = (process.env.SEMIO_PROBE_EXAMPLES ?? "hexagonal-mushroom-column,sphere-cut-with-torus").split(",").map((entry) => entry.trim()).filter(Boolean);
mkdirSync(outDir, { recursive: true });

const MEASURE_PREFIX = "semio.hop.";

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });

let lines = [];
let pageErrors = [];
const t0 = Date.now();
/** 🖨️ Playwright hands `console.log("a %s b", x)` back with its specifiers UNSUBSTITUTED and the
 * arguments trailing, so every parser downstream would read `%s` as the value. Substituted here once,
 * on ingest. Arguments are taken as whitespace-separated, which is what every line read below carries.
 * (memory: a probe that reads the raw text reads the format string, not the reading) */
const substituteConsoleFormat = (text) => {
  const specifier = /%[sdifoOc]/g;
  let end = -1;
  let count = 0;
  for (let match = specifier.exec(text); match !== null; match = specifier.exec(text)) {
    end = match.index + 2;
    count += 1;
  }
  if (count === 0) return text;
  const args = text.slice(end).trim().split(/\s+/);
  if (args.length < count) return text;
  let index = 0;
  return text.slice(0, end).replace(/%[sdifoOc]/g, () => args[index++]) + (args.length > count ? ` ${args.slice(count).join(" ")}` : "");
};
page.on("console", (message) => lines.push(`${Date.now() - t0} ${message.type()} ${substituteConsoleFormat(message.text()).slice(0, 600)}`));
page.on("pageerror", (error) => pageErrors.push(String(error).slice(0, 400)));

//#region 📏️Reads
const readSpans = () =>
  page.evaluate(
    (prefix) =>
      performance
        .getEntriesByType("measure")
        .filter((entry) => entry.name.startsWith(prefix))
        .map((entry) => ({ stage: entry.name.slice(prefix.length), startMs: entry.startTime, detail: entry.detail ?? null })),
    MEASURE_PREFIX,
  );

const pageNow = () => page.evaluate(() => performance.now());

const flowSurfaces = () =>
  page.evaluate(() => {
    const registry = window.__semioFlowGraphProbe ?? {};
    return Object.entries(registry).map(([surfaceId, entry]) => {
      const rect = entry.rect?.() ?? null;
      let nodeIds = [];
      try { nodeIds = [...(entry.nodeIds?.() ?? [])]; } catch { nodeIds = []; }
      return { surfaceId, rect, widgets: nodeIds };
    });
  });

/** 🎯️ A board point the pointer can actually land a node on, found by sweeping a grid and watching
 * for the surface's OWN hover publication.
 *
 * The board paints itself — one canvas, no per-node DOM — and the host's `entity("node", id)`
 * resolver answers `null` on this stage, so geometry cannot be read out of it. A hover that reaches
 * the plugin is the honest detector: it is the same span the click measurement counts. */
const findNodePoint = async (rect) => {
  const hovers = () =>
    page.evaluate(
      (prefix) => performance.getEntriesByType("measure").filter((entry) => entry.name === `${prefix}invoke` && entry.detail?.actionId === "interactionHover").length,
      MEASURE_PREFIX,
    );
  const columns = 14;
  const rows = 12;
  let empty = null;
  let swept = 0;
  for (let row = 1; row < rows; row += 1) {
    for (let column = 1; column < columns; column += 1) {
      const x = Math.round(rect.x + (rect.width * column) / columns);
      const y = Math.round(rect.y + (rect.height * row) / rows);
      const before = await hovers();
      await page.mouse.move(x, y);
      await page.waitForTimeout(260);
      swept += 1;
      if ((await hovers()) > before) return { x, y, swept, empty };
      // 🕳️ A swept point that published NO hover is empty board — the honest place to click for "a
      // plain click that changed nothing", rather than a corner that may sit under another element.
      if (empty === null) empty = { x, y };
    }
  }
  return null;
};

/** 🤫️ The shell stops dispatching on its own before a click is driven — same definition and same
 * spans as the scroll probe: the `semio.hop.invoke` count holding still. */
const settleShell = async (quietMs = 2000, maximumMs = 40_000) => {
  const invocations = () => page.evaluate((prefix) => performance.getEntriesByType("measure").filter((entry) => entry.name === `${prefix}invoke`).length, MEASURE_PREFIX);
  const startedAt = Date.now();
  let previous = await invocations();
  let quietSince = Date.now();
  while (Date.now() - startedAt < maximumMs) {
    await page.waitForTimeout(250);
    const current = await invocations();
    if (current !== previous) {
      previous = current;
      quietSince = Date.now();
      continue;
    }
    if (Date.now() - quietSince >= quietMs) return true;
  }
  return false;
};

const invocationsByAction = (spans, fromMs, toMs) => {
  const counts = {};
  for (const span of spans) {
    if (span.stage !== "invoke" || span.startMs < fromMs || span.startMs > toMs) continue;
    const action = String(span.detail?.actionId ?? "?");
    counts[action] = (counts[action] ?? 0) + 1;
  }
  return counts;
};
const total = (counts) => Object.values(counts).reduce((sum, value) => sum + value, 0);
//#endregion 📏️Reads

//#region 🖱️Clicks
/** 🧾️ Consultations of the interaction-publication ledger so far. Every one of them dispatched BOTH
 * lanes under the rule this replaces, so `2 × consultations` is the click's before-number. */
const consultations = () => lines.filter((line) => line.includes("flow interaction publish")).length;

const clickAt = async (label, x, y) => {
  const quiet = await settleShell();
  const from = await pageNow();
  await page.mouse.move(Math.round(x), Math.round(y));
  await page.waitForTimeout(160);
  const afterHover = await pageNow();
  const consultedBeforeClick = consultations();
  await page.mouse.down();
  await page.waitForTimeout(30);
  await page.mouse.up();
  await page.waitForTimeout(clickSettleMs);
  const spans = await readSpans();
  const to = await pageNow();
  const hoverCounts = invocationsByAction(spans, from, afterHover);
  const clickCounts = invocationsByAction(spans, afterHover, to);
  const consultedByClick = consultations() - consultedBeforeClick;
  return {
    label,
    quiet,
    at: { x: Math.round(x), y: Math.round(y) },
    hover: hoverCounts,
    hoverTotal: total(hoverCounts),
    click: clickCounts,
    clickTotal: total(clickCounts),
    ledgerConsultations: consultedByClick,
    hopsUnderTheOldRule: consultedByClick * 2,
  };
};
//#endregion 🖱️Clicks

//#region 📦️PayloadCensus
/** 📦️ The `[DEBUG] flow payload …` deliveries of this page, from the host's own line.
 *
 * `bodyBytes` is what the SAME crossings would have cost without content addressing — the body is
 * what `sendFlowPayloadOnce` used to put on the wire verbatim — so before and after are read off one
 * boot instead of two, with the session-level dedupe identical in both columns. */
const payloadCensus = (consoleLines) => {
  const census = {};
  for (const line of consoleLines) {
    const match = /\[DEBUG\] flow payload (\S+) bytes=(\d+) body=(\d+) parts=(\d+) named=(\d+)/.exec(line);
    if (!match) continue;
    const entry = (census[match[1]] ??= { crossings: 0, wireBytes: 0, bodyBytes: 0, parts: 0, namedParts: 0 });
    entry.crossings += 1;
    entry.wireBytes += Number(match[2]);
    entry.bodyBytes += Number(match[3]);
    entry.parts += Number(match[4]);
    entry.namedParts += Number(match[5]);
  }
  return census;
};
const countLines = (consoleLines, needle) => consoleLines.filter((line) => line.includes(needle)).length;
/** 🪞 Which surfaces reported ready, and how often each did — two `surface ready` lines for ONE
 * surfaceId is a re-attach; two different surfaceIds are two boards. */
const surfacesReady = (consoleLines) => {
  const counts = {};
  for (const line of consoleLines) {
    const match = /node-graph surface ready surface=(\S+)/.exec(line);
    if (match) counts[match[1]] = (counts[match[1]] ?? 0) + 1;
  }
  return counts;
};
//#endregion 📦️PayloadCensus

//#region ▶️Run
const rows = [];
for (const example of examples) {
  lines = [];
  pageErrors = [];
  await page.goto(`${url}&example=${example}`, { waitUntil: "domcontentloaded" });
  let surfaces = [];
  for (let second = 0; second < bootWaitS; second += 1) {
    await page.waitForTimeout(1000);
    surfaces = (await flowSurfaces()).filter((surface) => surface.rect && surface.rect.width > 80 && surface.widgets.length >= 3);
    if (surfaces.length > 0) break;
  }
  if (surfaces.length === 0) {
    rows.push({ example, error: "no flow canvas", pageErrors: pageErrors.length });
    writeFileSync(join(outDir, `console-${example}.txt`), lines.join("\n"));
    console.log(`[DEBUG] flow-surface ${example}: no flow canvas after ${bootWaitS}s, ${lines.length} console lines, ${pageErrors.length} page errors`);
    for (const error of pageErrors.slice(0, 5)) console.log(`[DEBUG]   pageerror ${error}`);
    continue;
  }
  const board = surfaces[0];
  // 🕰️ The second attach was reported at ~14 s into a boot, so the window is watched well past it
  // before the attach count is read.
  await page.waitForTimeout(attachWatchMs);
  const bootLines = [...lines];
  const surfaceReady = countLines(bootLines, "node-graph surface ready");
  const attachCalled = countLines(bootLines, "node-graph attach called");
  const hostMount = countLines(bootLines, "node-graph host mount");
  const hostUnmount = countLines(bootLines, "node-graph host unmount");
  const sessionReady = countLines(bootLines, "node-graph session ready");
  const census = payloadCensus(bootLines);
  const readyBySurface = surfacesReady(bootLines);
  // 🧾️ Consultations of the publication ledger — under the rule this replaces, EVERY one of them
  // dispatched both lanes, so `2 × consultations` is the measured before-number for a gesture.
  const publicationConsultations = countLines(bootLines, "flow interaction publish");

  const nodePoint = await findNodePoint(board.rect);
  // 🖱️ The plan, in the order a user would produce it: pick a node (one hop is due), pick the same
  // node again (nothing changed), clear on empty board (one hop is due), click the same empty board
  // again (nothing changed). The node is clicked FIRST, while the sweep's own reading of where it is
  // still holds.
  const clicks = [];
  const empty = nodePoint?.empty ?? { x: board.rect.x + 12, y: board.rect.y + board.rect.height - 12 };
  if (nodePoint) {
    clicks.push(await clickAt("node-first", nodePoint.x, nodePoint.y));
    clicks.push(await clickAt("node-again", nodePoint.x, nodePoint.y));
  }
  clicks.push(await clickAt("empty-canvas", empty.x, empty.y));
  clicks.push(await clickAt("empty-again", empty.x, empty.y));
  rows.push({
    example,
    surfaceId: board.surfaceId,
    widgets: board.widgets.length,
    nodePoint,
    surfaceReady,
    readyBySurface,
    reattachedSurfaces: Object.entries(readyBySurface).filter(([, count]) => count > 1).map(([surface, count]) => `${surface}×${count}`),
    attachCalled,
    hostMount,
    hostUnmount,
    sessionReady,
    payloads: census,
    payloadWireBytesTotal: Object.values(census).reduce((sum, entry) => sum + entry.wireBytes, 0),
    payloadBodyBytesTotal: Object.values(census).reduce((sum, entry) => sum + entry.bodyBytes, 0),
    clicks,
    pageErrors: pageErrors.length,
  });
  writeFileSync(join(outDir, "surface.json"), JSON.stringify(rows, null, 2));
  writeFileSync(join(outDir, `console-${example}.txt`), bootLines.join("\n"));
  const row = rows.at(-1);
  console.log(
    `[DEBUG] flow-surface ${example}: surfaceReady=${row.surfaceReady} attachCalled=${row.attachCalled} mount/unmount=${row.hostMount}/${row.hostUnmount} ` +
      `payloadBytes=${row.payloadWireBytesTotal} of ${row.payloadBodyBytesTotal} clicks=${row.clicks.map((click) => `${click.label}:${click.clickTotal}`).join(" ")} pageerrors=${row.pageErrors}`,
  );
  for (const click of row.clicks) console.log(`[DEBUG]   ${click.label} hover=${JSON.stringify(click.hover)} click=${JSON.stringify(click.click)} consultations=${click.ledgerConsultations} oldRuleHops=${click.hopsUnderTheOldRule}`);
  for (const [feature, entry] of Object.entries(row.payloads)) console.log(`[DEBUG]   payload ${feature} crossings=${entry.crossings} wireBytes=${entry.wireBytes} bodyBytes=${entry.bodyBytes} parts=${entry.parts} named=${entry.namedParts}`);
}
//#endregion ▶️Run

//#region 📝️Report
const measured = rows.filter((row) => !row.error);
const verdicts = measured.map((row) => {
  const byLabel = Object.fromEntries(row.clicks.map((click) => [click.label, click]));
  return {
    example: row.example,
    attachesOnce: row.reattachedSurfaces.length === 0,
    firstNodeClickCostsOne: (byLabel["node-first"]?.clickTotal ?? 0) === 1,
    repeatNodeClickCostsNothing: (byLabel["node-again"]?.clickTotal ?? 0) === 0,
    // 🕳️ A click on empty board only owes a hop when it actually cleared something — whether the
    // BOARD clears on an empty pick is its own rule, so the law here is "no hop unless the mark
    // moved", not "a hop".
    everyOtherClickCostsNothing: row.clicks.filter((click) => click.label !== "node-first").every((click) => click.clickTotal === 0),
    hopsUnderTheOldRule: row.clicks.reduce((sum, click) => sum + click.hopsUnderTheOldRule, 0),
    hopsNow: row.clicks.reduce((sum, click) => sum + click.clickTotal, 0),
    noPageErrors: row.pageErrors === 0,
    attachedSurfaces: row.reattachedSurfaces,
    payloadWireBytes: row.payloadWireBytesTotal,
    payloadBodyBytes: row.payloadBodyBytesTotal,
  };
});
const green = verdicts.every((verdict) => verdict.attachesOnce && verdict.firstNodeClickCostsOne && verdict.repeatNodeClickCostsNothing && verdict.everyOtherClickCostsNothing && verdict.hopsNow < verdict.hopsUnderTheOldRule && verdict.noPageErrors);

const table = [
  "| example | surface ready | re-attached | attach called | mount/unmount | payload wire B | payload body B | node click | node again | clear click | empty again | hops now / old rule | page errors |",
  "|---|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|",
  ...measured.map((row) => {
    const byLabel = Object.fromEntries(row.clicks.map((click) => [click.label, click]));
    return `| ${row.example} | ${row.surfaceReady} | ${row.reattachedSurfaces.join(" ") || "—"} | ${row.attachCalled} | ${row.hostMount}/${row.hostUnmount} | ${row.payloadWireBytesTotal} | ${row.payloadBodyBytesTotal} | ${byLabel["node-first"]?.clickTotal ?? "-"} | ${byLabel["node-again"]?.clickTotal ?? "-"} | ${byLabel["empty-canvas"]?.clickTotal ?? "-"} | ${byLabel["empty-again"]?.clickTotal ?? "-"} | ${row.clicks.reduce((sum, click) => sum + click.clickTotal, 0)} / ${row.clicks.reduce((sum, click) => sum + click.hopsUnderTheOldRule, 0)} | ${row.pageErrors} |`;
  }),
].join("\n");

writeFileSync(join(outDir, "surface.json"), JSON.stringify(rows, null, 2));
writeFileSync(join(outDir, "surface.md"), `${table}\n\n${JSON.stringify(verdicts, null, 2)}\n`);
console.log(`\n${table}\n`);
console.log(`[DEBUG] flow-surface gate ${green ? "GREEN" : "RED"} (1 attach per document, 1 hop for the click that moved the selection, 0 for every click that moved nothing)`);
console.log(`[DEBUG] flow-surface wrote ${join(outDir, "surface.json")}`);
await browser.close();
if (gate && !green) process.exit(1);
//#endregion 📝️Report
