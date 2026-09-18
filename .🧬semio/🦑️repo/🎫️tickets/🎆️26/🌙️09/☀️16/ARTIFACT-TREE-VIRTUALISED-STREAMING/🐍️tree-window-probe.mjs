/** 🪟️ Generic virtualised-tree-window browser probe (ticket 26/09/16 ARTIFACT-TREE-VIRTUALISED-STREAMING, W3).
 *
 * Boots any react dev lane, opens its Artifact panel and verifies the §6 host↔guest window contract
 * straight off the DOM: no `.more`/`+N` paging leftovers, the four `data-tree-window-*` mirrors against
 * the `[data-slot="tree-window-spacer"]` pitch, the scroll→`reportWindows`→`refreshUi` round trip (the
 * container's `offset` MUST move), collapse/re-expand, expansion surviving a body refresh, and a pick.
 *
 * Every step lands as one PASS/FAIL row with its evidence in `report.json`; `console.txt` keeps the raw
 * console feed. Nothing here is app-specific — a lane is described entirely by env:
 *   SEMIO_PROBE_URL      e.g. http://127.0.0.1:6020/?plugin=cad
 *   SEMIO_PROBE_TREE_NS  the panel tree's node-key namespace, e.g. cad-play-document
 *   SEMIO_PROBE_OUT      out dir under 🗑️generated, e.g. w3/cad
 *   SEMIO_PROBE_SECONDS  boot budget in seconds (default 120)
 *   SEMIO_PROBE_PANEL    panel tab button label (default "Artifact")
 *   SEMIO_PROBE_EXAMPLE  optional example-picker option text for step (d)
 *   SEMIO_PROBE_SCROLL_HEIGHT  browser height the streaming steps shrink to (default 520)
 *   SEMIO_PROBE_BODY_BUDGET    override for `TREE_WINDOW_BODY_NODE_BUDGET` (default: read off 🌳️Tree/🟦️.tsx)
 *   SEMIO_REPO           repo root the budget constant is read from (default /Users/ueli/Documents/semio)
 * Usage: cd <ticket> && SEMIO_PROBE_URL=… SEMIO_PROBE_TREE_NS=… SEMIO_PROBE_OUT=w3/cad bun 🐍️tree-window-probe.mjs
 *
 * 📐️ Step (e) is the `📓️f1-host-scroll-streaming.md` §7 contract, one sub-step per row of that table:
 * e1 the observed scroller really scrolls · e2 the `data-tree-window-*` mirror · e2b PATH identity ·
 * e3 the materialised band's row indices · e4 spacer + extent arithmetic · e5 scroll streams new rows ·
 * e6 scroll position stable across the refresh · e7 no oscillation once settled · e8 body node budget ·
 * e9 lazy expand (a container closed by its HEADER state, never by row count) · e10 nesting ·
 * e11 no duplicate path · e13 no view-context rejection · e14 nothing open stays empty ·
 * e12 no `.more` / `+N` survivor.
 * `SEMIO_PROBE_SCROLL_HEIGHT` is applied unconditionally, and an explicit `SEMIO_PROBE_EXAMPLE` always
 * switches documents even when the boot document already streams.
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6020/?plugin=cad";
const ns = process.env.SEMIO_PROBE_TREE_NS ?? "cad-play-document";
const bootSeconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 120);
const panelName = process.env.SEMIO_PROBE_PANEL ?? "Artifact";
const wantedExample = process.env.SEMIO_PROBE_EXAMPLE ?? "";
const ROWS_MAX = Number(process.env.SEMIO_PROBE_ROWS_MAX ?? 128);
const scrollHeight = Number(process.env.SEMIO_PROBE_SCROLL_HEIGHT ?? 520);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "w3/tree-window");
mkdirSync(outDir, { recursive: true });

// 🧮️ The whole-body node budget is ONE literal in the host tree element (`📓️f1…md` §10.1 keeps it on one
// line so the Rust parity law can grep it). Read it — never hard-code it, it moved 111 → 103 on 2026-09-17.
const repoRoot = process.env.SEMIO_REPO ?? "/Users/ueli/Documents/semio";
const treeElementPath = join(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx");
const readBodyBudget = () => {
  if (process.env.SEMIO_PROBE_BODY_BUDGET) return { budget: Number(process.env.SEMIO_PROBE_BODY_BUDGET), source: "env" };
  try {
    const match = readFileSync(treeElementPath, "utf8").match(/export const TREE_WINDOW_BODY_NODE_BUDGET\s*=\s*(\d+)/);
    if (match) return { budget: Number(match[1]), source: `${treeElementPath} (${match[0]})` };
  } catch (error) { return { budget: 0, source: `unreadable: ${String(error).slice(0, 120)}` }; }
  return { budget: 0, source: "not found" };
};
const bodyBudget = readBodyBudget();
// 🔑️ The window-path separator, READ off the same one line as the budget (`TREE_WINDOW_PATH_SEPARATOR`,
// Rust ↔ TS parity law). It moved from the invisible U+001F to the printable `␟` U+241F on 2026-09-17,
// because a raw control character is not a legal view-context identifier (📓️f1…md §7b) — never hard-code it.
const readPathSeparator = () => {
  if (process.env.SEMIO_PROBE_PATH_SEP) return { sep: process.env.SEMIO_PROBE_PATH_SEP, source: "env" };
  try {
    const found = readFileSync(treeElementPath, "utf8").match(/export const TREE_WINDOW_PATH_SEPARATOR\s*=\s*"([^"]*)"/);
    if (found) {
      // 🧯️ The literal may be written as an escape (`\u001f`) or as the character itself.
      const sep = found[1].replace(/\\u\{?([0-9a-fA-F]{1,6})\}?/g, (_, hex) => String.fromCodePoint(Number.parseInt(hex, 16)));
      if (sep) return { sep, source: `${treeElementPath} (U+${sep.codePointAt(0).toString(16).toUpperCase().padStart(4, "0")})` };
    }
  } catch (error) { return { sep: "\u241f", source: `unreadable, fell back to U+241F: ${String(error).slice(0, 120)}` }; }
  return { sep: "\u241f", source: "not found, fell back to U+241F" };
};
const pathSeparator = readPathSeparator();
const PATH_SEP = pathSeparator.sep;
// 🖨️ Print a path with its separator made visible, whichever code point it currently is.
const printPath = (path) => String(path ?? "").replaceAll(PATH_SEP, "␟");

const lines = [];
const steps = [];
const report = { lane: { url, ns, panelName, bodyBudget: bodyBudget.budget, bodyBudgetSource: bodyBudget.source, pathSeparator: `U+${PATH_SEP.codePointAt(0).toString(16).toUpperCase().padStart(4, "0")}`, pathSeparatorSource: pathSeparator.source }, startedAt: new Date().toISOString(), steps };
const t0 = Date.now();
const flush = () => {
  report.summary = { pass: steps.filter((s) => s.status === "PASS").length, fail: steps.filter((s) => s.status === "FAIL").length, skip: steps.filter((s) => s.status === "SKIP").length };
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
};
const note = (key, value) => { lines.push(`${Date.now() - t0} probe ${key} ${JSON.stringify(value).slice(0, 2000)}`); flush(); };
const record = (id, title, ok, evidence) => {
  const entry = { id, title, status: ok === "skip" ? "SKIP" : ok ? "PASS" : "FAIL", evidence };
  steps.push(entry);
  lines.push(`${Date.now() - t0} STEP ${id} ${entry.status} ${JSON.stringify(evidence).slice(0, 2000)}`);
  console.log(`[DEBUG] ${id} ${entry.status} ${JSON.stringify(evidence).slice(0, 600)}`);
  flush();
  return entry;
};
const guard = async (id, title, fn) => {
  try { const { ok, evidence } = await fn(); return record(id, title, ok, evidence); }
  catch (error) { return record(id, title, false, { threw: String(error).slice(0, 400) }); }
};

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
await page.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
page.on("console", (msg) => lines.push(`${Date.now() - t0} ${msg.type()} ${msg.text().slice(0, msg.type() === "error" ? 6000 : 1500)}`));
page.on("pageerror", (err) => lines.push(`${Date.now() - t0} pageerror ${String(err).slice(0, 2000)}`));
// 🌐️ "Failed to load resource" never names the URL in the console feed, so record the response itself.
// `/favicon.ico` is a dev-serve asset every lane lacks — it is not a product fault.
page.on("response", (res) => { if (res.status() >= 400 && !/favicon\.ico/.test(res.url())) lines.push(`${Date.now() - t0} httpfault ${res.status()} ${res.url().slice(0, 300)}`); });
const shot = (name) => page.screenshot({ path: join(outDir, `${name}.png`), type: "png" }).catch(() => {});
// ⚠️ `[DEBUG]` is this repo's temporary-log prefix (AGENTS.md) and the guests print plenty of it through
// `console.error`; only a pageerror or a NON-`[DEBUG]` console error is a fault.
// `[DBGARCH]` is a peer's temporary archive-lane prefix on the same console.error lane as `[DEBUG]`.
const DEBUG_PREFIXES = ["[DEBUG]", "[DBGARCH]"];
const isFault = (line) => /^\d+ (pageerror|httpfault) /.test(line)
  || (/^\d+ error /.test(line) && !DEBUG_PREFIXES.some((p) => line.includes(p)) && !line.includes("Failed to load resource"));
const faultsSince = (from) => lines.slice(from).filter(isFault).map((l) => l.slice(0, 400));

// 🔎️ ONE in-page reader: rows, windowed containers, spacers and the scroll viewport of ONE panel tree.
const readTree = (namespace) => page.evaluate((nsIn) => {
  const prefix = `panel:${nsIn}/`;
  const all = [...document.querySelectorAll(`[id^="${prefix}"]`)];
  const anchor = all[0] ?? null;
  const viewportEl = anchor?.closest('[data-slot="scroll-area-viewport"]') ?? null;
  // 📜️ Mirror of the host's `treeWindowScrollViewport` (🗣️Interpreter/🟦️.tsx:1519, F1): the nearest ancestor
  // that REALLY scrolls wins, `[data-slot="scroll-area"]` is the fallback, the page is the last resort.
  // W3 §5's defect was the probe (and the host) measuring `scroll-area-viewport`, which never scrolls.
  const isScroller = (el) => {
    if (el.getAttribute && el.getAttribute("data-slot") === "scroll-area") return true;
    const overflowY = getComputedStyle(el).overflowY;
    return overflowY === "auto" || overflowY === "scroll" || overflowY === "overlay";
  };
  const scrollerChain = [];
  for (let el = anchor; el; el = el.parentElement) if (isScroller(el)) scrollerChain.push(el);
  const scrollerEl = scrollerChain.find((c) => c.scrollHeight - c.clientHeight > 1)
    ?? scrollerChain.find((c) => c.getAttribute("data-slot") === "scroll-area")
    ?? scrollerChain[0]
    ?? document.scrollingElement;
  const metrics = (el) => (el
    ? { found: true, slot: el.getAttribute ? el.getAttribute("data-slot") : null, scrollTop: Math.round(el.scrollTop), scrollHeight: Math.round(el.scrollHeight), clientHeight: Math.round(el.clientHeight), extent: Math.round(el.scrollHeight - el.clientHeight), top: Math.round(el.getBoundingClientRect().top) }
    : { found: false });
  const rootEl = viewportEl ?? anchor?.closest('[data-slot="panel-body"]') ?? document.body;
  const scoped = rootEl === document.body ? all : all.filter((el) => rootEl.contains(el));
  const text = (el) => (el.textContent ?? "").trim().replace(/\s+/g, " ").slice(0, 80);
  const rows = scoped.map((el) => {
    const rect = el.getBoundingClientRect();
    return {
      id: el.id,
      key: el.id.slice(prefix.length),
      label: text(el),
      role: el.getAttribute("role"),
      expanded: el.getAttribute("aria-expanded"),
      selected: el.getAttribute("aria-selected"),
      // 👁️ A row inside a COLLAPSED container is still in the DOM with a zero box — never clickable.
      visible: rect.height > 0 && rect.width > 0,
    };
  });
  const num = (el, name) => Number(el.getAttribute(name) ?? "NaN");
  const nsKeys = new Set(rows.map((r) => r.key));
  // 🔑️ A window is addressed by its PATH (`data-tree-window-path`): the enclosing windowed containers' node
  // keys, outermost first, then its own, joined by U+001F. `data-tree-window-key` stays the authored node key
  // — the pick target id — and is legitimately shared by containers under different parents.
  const containerEls = [...(rootEl.querySelectorAll("[data-tree-window-path]") ?? [])];
  const scrollerRect = scrollerEl && scrollerEl.getBoundingClientRect ? scrollerEl.getBoundingClientRect() : null;
  const containers = containerEls.map((el) => {
    const rect = el.getBoundingClientRect();
    const spacers = [...el.querySelectorAll('[data-slot="tree-window-spacer"]')]
      .filter((s) => s.closest("[data-tree-window-path]") === el)
      .map((s) => ({ edge: s.getAttribute("data-tree-window-spacer"), rows: Number(s.getAttribute("data-tree-window-rows") ?? "NaN"), heightPx: Math.round(s.getBoundingClientRect().height) }));
    const ownRows = [...el.querySelectorAll(`[id^="${prefix}"]`)].filter((r) => r.closest("[data-tree-window-path]") === el);
    // 📐️ The MATERIALISED BAND, read exactly the way the host reads it (`treeWindowRowsUnder`,
    // 🗣️Interpreter/🟦️.tsx:1601): `[data-tree-window-row]` owned via `closest('[data-tree-window-key]')`,
    // sorted by TOP — a nested open group is many rows tall, so index order is not pixel order.
    const bandEls = [...el.querySelectorAll("[data-tree-window-row]")].filter((r) => r.closest("[data-tree-window-key]") === el);
    const band = bandEls.map((r) => {
      const rr = r.getBoundingClientRect();
      const owned = r.id && r.id.startsWith(prefix) ? r.id.slice(prefix.length) : (r.querySelector(`[id^="${prefix}"]`)?.id ?? "").slice(prefix.length) || null;
      return { index: Number(r.getAttribute("data-tree-window-row")), top: Math.round(rr.top), height: Math.round(rr.height), key: owned };
    }).sort((a, b) => a.top - b.top || a.index - b.index);
    // 🪆️ Nesting: the enclosing windowed container, and the ones directly inside this one.
    const parentEl = el.parentElement ? el.parentElement.closest("[data-tree-window-path]") : null;
    const nested = containerEls
      .filter((n) => n !== el && (n.parentElement ? n.parentElement.closest("[data-tree-window-path]") : null) === el)
      .map((n) => ({ path: n.getAttribute("data-tree-window-path"), key: n.getAttribute("data-tree-window-key"), height: Math.round(n.getBoundingClientRect().height) }));
    const key = el.getAttribute("data-tree-window-key");
    // 🪆️ Which entry of the PARENT's window this container hangs off — the parent must be showing that row
    // for this nested window to be on screen at all.
    // 🧯️ The owner row is a SIBLING, not an ancestor: `TreeItem` renders an expandable row as a fragment of
    // [header row, branch content], so the header carrying `data-tree-window-row` sits BEFORE this content
    // element inside the same parent container. `closest()` walks upwards and can never reach it.
    const ownerRowEl = (() => {
      const parent = el.parentElement ? el.parentElement.closest("[data-tree-window-path]") : null;
      if (!parent) return null;
      const own = [...parent.querySelectorAll("[data-tree-window-row]")].filter((r) => r.closest("[data-tree-window-path]") === parent);
      const top = el.getBoundingClientRect().top;
      let found = null;
      for (const row of own) if (row.getBoundingClientRect().top <= top) found = row;
      return found;
    })();
    const ownerRowIndex = ownerRowEl ? Number(ownerRowEl.getAttribute("data-tree-window-row")) : null;
    // 🌀️ `border-loading` is the ring a container wears while its rows are in flight (🌀️status-border-presentation).
    const ownerRow = key ? document.getElementById(`${prefix}${key}`) : null;
    const ring = Boolean(ownerRow && (String(ownerRow.className).includes("border-loading") || ownerRow.querySelector(".border-loading") || (ownerRow.parentElement && ownerRow.parentElement.querySelector(":scope > .border-loading"))));
    // 🗂️ OPEN vs CLOSED is what the HEADER says (`aria-expanded` / `data-state`), never the row count: an open
    // container whose window has not been answered yet also shows zero rows (📓️f1…md §7b, the cad e9 defect).
    const headerState = ownerRow ? ownerRow.getAttribute("data-state") : null;
    const headerExpanded = ownerRow
      ? (ownerRow.getAttribute("aria-expanded") ?? (headerState === "open" ? "true" : headerState === "closed" ? "false" : null))
      : null;
    const closed = headerExpanded === "false" || headerState === "closed";
    // 👁️ Does this container's own box intersect the scroller's visible box? (e14)
    const inViewport = Boolean(scrollerRect) && rect.height > 0 && rect.bottom > scrollerRect.top + 1 && rect.top < scrollerRect.bottom - 1;
    return {
      key,
      path: el.getAttribute("data-tree-window-path"),
      parentPath: parentEl ? parentEl.getAttribute("data-tree-window-path") : null,
      slot: el.getAttribute("data-slot"),
      total: num(el, "data-tree-window-total"),
      offset: num(el, "data-tree-window-offset"),
      length: num(el, "data-tree-window-length"),
      hidden: el.hasAttribute("hidden") || rect.height === 0,
      top: Math.round(rect.top),
      height: Math.round(rect.height),
      // 📍️ The container's own top in the SCROLLER's content space — what a scrollTop must be measured against.
      topInScroller: scrollerRect ? Math.round(rect.top - scrollerRect.top + scrollerEl.scrollTop) : null,
      leading: spacers.filter((s) => s.edge === "leading").reduce((a, s) => a + s.rows, 0),
      trailing: spacers.filter((s) => s.edge === "trailing").reduce((a, s) => a + s.rows, 0),
      spacers,
      rowKeys: ownRows.map((r) => r.id.slice(prefix.length)),
      band,
      nested,
      ownerRowIndex,
      ring,
      headerExpanded,
      headerState,
      closed,
      inViewport,
    };
  }).filter((c) => c.rowKeys.length > 0 || c.band.length > 0 || (c.key !== null && nsKeys.has(c.key)));
  // 📏️ The row pitch, straight off a spacer (`rows × treeRowHeightPx`) — measured, never assumed.
  const pitches = containers.flatMap((c) => c.spacers.filter((s) => s.rows > 0 && s.heightPx > 0).map((s) => s.heightPx / s.rows)).sort((a, b) => a - b);
  const bandHeights = containers.flatMap((c) => c.band.map((r) => r.height)).filter((h) => h > 0).sort((a, b) => a - b);
  const pitch = pitches.length ? Math.round(pitches[Math.floor(pitches.length / 2)]) : (bandHeights.length ? bandHeights[0] : 0);
  // 👁️ Rows really painted inside the scroller's visible box — a viewport that shows only spacers after the
  // window settled is the streaming failure this probe exists to catch.
  const visibleBand = scrollerRect
    ? [...rootEl.querySelectorAll("[data-tree-window-row]")].filter((r) => { const rr = r.getBoundingClientRect(); return rr.height > 0 && rr.bottom > scrollerRect.top + 1 && rr.top < scrollerRect.bottom - 1; }).length
    : 0;
  const paths = containers.map((c) => c.path).filter(Boolean);
  return {
    rows,
    containers,
    dotMore: rows.filter((r) => r.key.endsWith(".more")).map((r) => r.key),
    plusLabels: rows.filter((r) => /^\+\d+$/.test(r.label)).map((r) => ({ key: r.key, label: r.label })),
    bodyPlus: (rootEl.innerText ?? "").split("\n").map((l) => l.trim()).filter((l) => /^\+\d+$/.test(l)).slice(0, 8),
    duplicatePaths: paths.filter((p, i) => paths.indexOf(p) !== i),
    pitch,
    visibleBand,
    // 🪟️ `viewport` is the element the host observes AND the probe scrolls; `innerViewport` is the
    // never-scrolling `scroll-area-viewport` W3 §5 caught the host binding to.
    viewport: metrics(scrollerEl),
    innerViewport: metrics(viewportEl),
  };
}, namespace);

// 📜️ Scroll the element the HOST observes (same resolution as `readTree`), by fraction of its extent or to
// an absolute pixel offset. `px === null` means "use the fraction".
const scrollScroller = (namespace, fraction, px = null) => page.evaluate(([nsIn, f, target]) => {
  const anchor = document.querySelector(`[id^="panel:${nsIn}/"]`);
  if (!anchor) return { ok: false, reason: "no tree" };
  const isScroller = (el) => {
    if (el.getAttribute && el.getAttribute("data-slot") === "scroll-area") return true;
    const overflowY = getComputedStyle(el).overflowY;
    return overflowY === "auto" || overflowY === "scroll" || overflowY === "overlay";
  };
  const chain = [];
  for (let el = anchor; el; el = el.parentElement) if (isScroller(el)) chain.push(el);
  const vp = chain.find((c) => c.scrollHeight - c.clientHeight > 1) ?? chain.find((c) => c.getAttribute("data-slot") === "scroll-area") ?? chain[0] ?? document.scrollingElement;
  if (!vp) return { ok: false, reason: "no scroller" };
  const max = vp.scrollHeight - vp.clientHeight;
  vp.scrollTop = target === null ? Math.round(max * f) : Math.max(0, Math.min(max, Math.round(target)));
  vp.dispatchEvent(new Event("scroll", { bubbles: false }));
  return { ok: true, slot: vp.getAttribute ? vp.getAttribute("data-slot") : null, scrollTop: Math.round(vp.scrollTop), asked: target, max: Math.round(max), clientHeight: Math.round(vp.clientHeight) };
}, [namespace, fraction, px]);
const scrollPanel = (namespace, fraction) => scrollScroller(namespace, fraction, null);

// 🧭️ When nothing scrolls, name the chain: `useTreeWindowObserver` picks
// `closest('[data-slot="scroll-area-viewport"]')`, else the nearest `overflow-y: auto|scroll` ancestor.
const scrollChain = (namespace) => page.evaluate((nsIn) => {
  let el = document.querySelector(`[id^="panel:${nsIn}/"]`);
  const chain = [];
  while (el && chain.length < 18) {
    const style = getComputedStyle(el);
    chain.push({
      tag: el.tagName.toLowerCase(), slot: el.getAttribute("data-slot"), id: el.id || null,
      overflowY: style.overflowY, clientHeight: el.clientHeight, scrollHeight: el.scrollHeight,
      extent: el.scrollHeight - el.clientHeight,
      maxHeight: style.maxHeight, height: style.height, flex: style.flex, minHeight: style.minHeight,
    });
    el = el.parentElement;
  }
  return chain;
}, namespace);

const clickRowKey = async (key) => {
  const row = page.locator(`[id="panel:${ns}/${key}"]`).first();
  await row.waitFor({ state: "attached", timeout: 8000 });
  await row.evaluate((el) => el.scrollIntoView({ block: "center" }));
  await page.waitForTimeout(250);
  const box = await row.boundingBox();
  if (!box) throw new Error(`row ${key} has no box`);
  // 🪆️ A row that owns an open nested window is MANY rows tall; its own label line is the FIRST pitch, so
  // clicking the box's vertical centre would land inside the child window instead.
  await page.mouse.click(box.x + Math.min(90, box.width / 2), box.y + Math.min(12, box.height / 2));
  await page.waitForTimeout(2200);
};
// ⏱️ Poll a predicate over the tree for at most `ms` — the host→guest window request is a partial
// `refreshUi` round trip, so the DOM only moves once the guest answers.
const waitFor = async (predicate, ms = 5000) => {
  const deadline = Date.now() + ms;
  let last = await readTree(ns);
  while (Date.now() < deadline) {
    if (predicate(last)) return { hit: true, tree: last, waitedMs: ms - (deadline - Date.now()) };
    await page.waitForTimeout(400);
    last = await readTree(ns);
  }
  return { hit: predicate(last), tree: last, waitedMs: ms };
};
// 🗂️ The panel rail button TOGGLES, so a blind second click closes the panel the probe just opened.
// Only ever click it while the tree is absent, and re-try on a budget: after a whole-document replace
// the body is legitimately empty until the guest republishes it.
const ensurePanel = async (budgetMs) => {
  const deadline = Date.now() + budgetMs;
  let clicks = 0;
  for (;;) {
    const tree = await readTree(ns);
    if (tree.rows.length > 0) return { ok: true, clicks, waitedMs: budgetMs - (deadline - Date.now()), rows: tree.rows.length };
    if (Date.now() >= deadline) return { ok: false, clicks, waitedMs: budgetMs, rows: 0 };
    const tab = page.getByRole("button", { name: panelName, exact: true }).first();
    if (await tab.count()) { await tab.click({ timeout: 8000 }).catch(() => {}); clicks += 1; }
    await page.waitForTimeout(6000);
  }
};
// 🧊️ A whole-document replace lands in several refreshes; comparing a tree read mid-flight against one
// read after it is how a probe invents failures. Wait until the shape stops moving.
const treeSignature = (t) => `${t.rows.length}|${t.containers.map((c) => `${c.key}:${c.total}:${c.offset}:${c.length}`).join(",")}`;
const settleTree = async (budgetMs, quietMs = 8000) => {
  const deadline = Date.now() + budgetMs;
  let signature = treeSignature(await readTree(ns));
  let quietSince = Date.now();
  for (;;) {
    await page.waitForTimeout(1500);
    const next = treeSignature(await readTree(ns));
    if (next !== signature) { signature = next; quietSince = Date.now(); }
    else if (Date.now() - quietSince >= quietMs) return { settled: true, waitedMs: budgetMs - (deadline - Date.now()) };
    if (Date.now() >= deadline) return { settled: false, waitedMs: budgetMs };
  }
};
const byKey = (tree, key) => tree.containers.find((c) => c.key === key) ?? null;
// 🔑️ Identity is the PATH (F2 §C1): the same `-key` legitimately appears under two parents.
const byPath = (tree, path) => tree.containers.find((c) => c.path === path) ?? null;
const biggest = (tree) => [...tree.containers].filter((c) => c.total > 0).sort((a, b) => b.total - a.total)[0] ?? null;

try {
  // ── (a) boot + Artifact panel ────────────────────────────────────────────
  await page.goto(url, { waitUntil: "domcontentloaded" });
  await guard("a", "boot + Artifact panel visible", async () => {
    let state = null;
    for (let i = 0; i < bootSeconds; i++) {
      await page.waitForTimeout(1000);
      state = await page.evaluate(() => ({
        ready: document.documentElement.getAttribute("data-semio-os-ready"),
        error: document.documentElement.getAttribute("data-semio-os-error"),
        surfaces: [...document.querySelectorAll("[data-surface-id]")].map((el) => el.getAttribute("data-surface-id")).slice(0, 12),
      }));
      if (state.error) break;
      if (state.ready && state.surfaces.length >= 1 && i > 6) break;
    }
    const opened = await ensurePanel(60000);
    const quiet = await settleTree(60000);
    const tree = await readTree(ns);
    report.boot = { state, opened, settled: quiet, rows: tree.rows.length, containers: tree.containers.length, viewport: tree.viewport };
    await shot("1-boot");
    return { ok: Boolean(state?.ready) && !state?.error && tree.rows.length > 0, evidence: report.boot };
  });

  // ── (b) no paging leftovers ──────────────────────────────────────────────
  await guard("b", "no `.more` row key and no `+N` row label in the panel", async () => {
    const tree = await readTree(ns);
    return { ok: tree.dotMore.length === 0 && tree.plusLabels.length === 0 && tree.bodyPlus.length === 0, evidence: { dotMore: tree.dotMore, plusLabels: tree.plusLabels, bodyPlusText: tree.bodyPlus, rowsScanned: tree.rows.length } };
  });

  // ── (c) every windowed container's spacer arithmetic ─────────────────────
  await guard("c", "spacers mirror total/offset/length and rows <= 128", async () => {
    const tree = await readTree(ns);
    const table = tree.containers.map((c) => ({
      key: c.key, slot: c.slot, total: c.total, offset: c.offset, length: c.length, hidden: c.hidden,
      leadingRows: c.leading, trailingRows: c.trailing, spacers: c.spacers,
      leadingOk: c.leading === c.offset,
      trailingOk: c.trailing === Math.max(0, c.total - c.offset - c.length),
      lengthOk: c.length <= ROWS_MAX,
      sumOk: c.offset + c.length <= c.total,
    }));
    report.containers = table;
    const bad = table.filter((c) => !(c.leadingOk && c.trailingOk && c.lengthOk && c.sumOk));
    return { ok: table.length > 0 && bad.length === 0, evidence: { containers: table.length, bad, sample: table.slice(0, 12) } };
  });

  // ── (d) a container larger than its window ───────────────────────────────
  // 🪟️ A container only EXERCISES streaming when it is open and shows a strict slice of its own total;
  // a collapsed section with `total > 0, length 0` is the §6 closed case, not a streamed window.
  const streamingOf = (tree) => tree.containers.filter((c) => c.total > c.length && c.length > 0 && !c.hidden);
  const oversized = await guard("d", "at least one OPEN container has total > length (streaming exercised)", async () => {
    let tree = await readTree(ns);
    let over = streamingOf(tree);
    let switched = "not-needed";
    // 🎚️ An EXPLICIT `SEMIO_PROBE_EXAMPLE` always switches, even when the boot document already streams —
    // the lane is named by env, and "the default happened to stream" must not silently probe another document.
    if (over.length === 0 || wantedExample) {
      const beforeSignature = treeSignature(tree);
      const combo = page.locator('[role="combobox"]').first();
      if (await combo.count()) {
        await combo.click({ timeout: 5000 }).catch(() => {});
        await page.waitForTimeout(600);
        const options = await page.evaluate(() => [...document.querySelectorAll('[role="option"]')].map((o) => o.textContent?.trim()).filter(Boolean));
        const wanted = wantedExample ? options.find((o) => o.includes(wantedExample)) : undefined;
        // 🧯️ A named example that is not on the menu is a LANE error, never "take the last option instead".
        const target = wantedExample ? wanted : options[options.length - 1];
        if (wantedExample && !wanted) { await page.keyboard.press("Escape"); switched = `wanted "${wantedExample}" not among ${JSON.stringify(options.slice(0, 8))}`; }
        if (target) {
          const opt = page.locator('[role="option"]').filter({ hasText: target }).first();
          switched = await opt.click({ timeout: 5000 }).then(() => `picked:${target}`).catch((e) => String(e).slice(0, 100));
          // ⏳️ A whole-document replace re-solves; the Artifact panel is EMPTY until the guest republishes
          // its body, so wait for rows to come back before calling the tree small.
          const budget = Number(process.env.SEMIO_PROBE_EXAMPLE_SECONDS ?? 150) * 1000;
          // 🔁️ The new document arrives tens of seconds after the click; a settle that starts immediately
          // just certifies the OLD body as quiet. Wait for the tree to CHANGE, then for it to stop moving.
          const changed = await waitFor((t) => treeSignature(t) !== beforeSignature, budget);
          const back = await ensurePanel(budget);
          const quiet = await settleTree(budget);
          switched = `${switched} changed=${changed.hit} after ${Math.round(changed.waitedMs / 1000)}s, rowsBack=${back.rows} (${back.clicks} tab clicks), settled=${quiet.settled} after ${Math.round(quiet.waitedMs / 1000)}s`;
        } else if (!wantedExample) { switched = `no-option (${options.length})`; await page.keyboard.press("Escape"); }
      } else switched = "no-example-picker";
      tree = await readTree(ns);
      over = streamingOf(tree);
    }
    report.oversized = over.map((c) => ({ key: c.key, total: c.total, offset: c.offset, length: c.length }));
    if (switched !== "not-needed") report.containersAfterExample = tree.containers.map((c) => ({
      key: c.key, total: c.total, offset: c.offset, length: c.length, hidden: c.hidden, leadingRows: c.leading, trailingRows: c.trailing,
      leadingOk: c.leading === c.offset, trailingOk: c.trailing === Math.max(0, c.total - c.offset - c.length), lengthOk: c.length <= ROWS_MAX,
    }));
    const totals = tree.containers.map((c) => `${c.key}=${c.length}/${c.total}${c.hidden ? " (closed)" : ""}`).slice(0, 24);
    const closedWithTotal = tree.containers.filter((c) => c.total > 0 && c.length === 0).map((c) => `${c.key}=0/${c.total}`);
    await shot("2-panel");
    return {
      ok: over.length > 0 ? true : "skip",
      evidence: over.length > 0
        ? { switched, streaming: report.oversized.slice(0, 8), totals }
        : { switched, verdict: "document fits, streaming not exercised", closedWithTotal: closedWithTotal.slice(0, 12), totals },
    };
  });

  // ── (e) the F1 §7 DOM/scroll contract, one sub-step per row ──────────────
  // 🪟️ A container only EXERCISES streaming when it is OPEN and shows a strict slice of its own total, so
  // every streaming sub-step SKIPs (never silently passes) when the document fits the viewport.
  const streamState = { scrollable: false, target: null, rest: null, after: null };
  const streamingOpen = (tree) => tree.containers.filter((c) => c.total > c.length && c.length > 0 && !c.hidden);

  await guard("e1", "the element the host observes really scrolls, and it is not scroll-area-viewport", async () => {
    // 📏️ ALWAYS apply `SEMIO_PROBE_SCROLL_HEIGHT`: the lane picks the viewport, and a document that happens
    // to scroll at 1000 px must still be measured at the height the lane asked for (the w5 fem3d round-1 run
    // reported `clientHeight 946` because the shrink was conditional).
    await page.setViewportSize({ width: 1600, height: scrollHeight });
    await page.waitForTimeout(3500);
    await settleTree(20000, 2500);
    let tree = await readTree(ns);
    let resized = `1600x${scrollHeight} -> scroller clientHeight ${tree.viewport.clientHeight}, extent ${tree.viewport.extent}`;
    report.scroller = { viewport: tree.viewport, innerViewport: tree.innerViewport, resized, pitch: tree.pitch };
    if (!tree.viewport.found || tree.viewport.extent <= 1) {
      const chain = await scrollChain(ns);
      report.scrollChain = chain;
      await shot("3-no-scroll");
      return { ok: "skip", evidence: { verdict: `nothing in the panel's chain scrolls even at 1600x${scrollHeight}`, ...report.scroller, scrollableAncestors: chain.filter((c) => c.extent > 0 || /auto|scroll/.test(c.overflowY)).slice(0, 6) } };
    }
    streamState.scrollable = true;
    // 🧯️ W3 §5's defect: the unbounded inner `scroll-area-viewport` must never be the observed element.
    const innerIsScroller = tree.viewport.slot === "scroll-area-viewport";
    const innerStill = !tree.innerViewport.found || tree.innerViewport.extent <= 1;
    return { ok: !innerIsScroller && innerStill, evidence: { ...report.scroller, innerIsScroller, innerStill } };
  });

  await guard("e2", "every windowed container mirrors key/path/total/offset/length with offset+length <= total", async () => {
    const tree = await readTree(ns);
    const bad = tree.containers.filter((c) => !c.path || !Number.isFinite(c.total) || !Number.isFinite(c.offset) || !Number.isFinite(c.length) || c.offset + c.length > c.total || c.length > ROWS_MAX);
    report.windowMirror = tree.containers.map((c) => ({ key: c.key, path: c.path, slot: c.slot, total: c.total, offset: c.offset, length: c.length, hidden: c.hidden, ownerRowIndex: c.ownerRowIndex }));
    return { ok: tree.containers.length > 0 && bad.length === 0, evidence: { containers: tree.containers.length, bad: bad.map((c) => ({ path: c.path, total: c.total, offset: c.offset, length: c.length })), sample: report.windowMirror.slice(0, 12) } };
  });

  await guard("e2b", "a window's PATH is its key at the top and <parent path>U+001F<key> below", async () => {
    const tree = await readTree(ns);
    const bad = tree.containers.filter((c) => c.path !== (c.parentPath ? `${c.parentPath}${PATH_SEP}${c.key}` : c.key));
    const nested = tree.containers.filter((c) => c.parentPath);
    // 🔑️ A repeated `-key` under different parents is LEGAL and must not be reported as a duplicate.
    const keys = tree.containers.map((c) => c.key);
    const repeatedKeys = [...new Set(keys.filter((k, i) => keys.indexOf(k) !== i))];
    return { ok: bad.length === 0, evidence: { containers: tree.containers.length, nested: nested.map((c) => ({ path: printPath(c.path), parent: c.parentPath })).slice(0, 10), repeatedKeysLegal: repeatedKeys.slice(0, 8), bad: bad.map((c) => ({ key: c.key, path: printPath(c.path), parentPath: c.parentPath })) } };
  });

  await guard("e3", "each container's own rows carry data-tree-window-row = offset … offset+length-1 in top order", async () => {
    const tree = await readTree(ns);
    const open = tree.containers.filter((c) => !c.hidden && c.length > 0);
    const table = open.map((c) => ({
      path: printPath(c.path), offset: c.offset, length: c.length,
      indices: c.band.map((r) => r.index),
      expected: Array.from({ length: c.length }, (_, i) => c.offset + i),
    })).map((r) => ({ ...r, ok: JSON.stringify(r.indices) === JSON.stringify(r.expected) }));
    report.bands = table;
    const bad = table.filter((r) => !r.ok);
    return { ok: open.length > 0 && bad.length === 0, evidence: { open: open.length, bad: bad.slice(0, 6), sample: table.slice(0, 8) } };
  });

  await guard("e4", "spacers are exactly offset / total-offset-length, zero-row spacers absent, extent = total x pitch + nested", async () => {
    const tree = await readTree(ns);
    const pitch = tree.pitch;
    const table = tree.containers.filter((c) => !c.hidden).map((c) => {
      const nestedExtent = c.nested.reduce((a, n) => a + n.height, 0);
      const expectedHeight = c.total * pitch + nestedExtent;
      return {
        path: printPath(c.path), total: c.total, offset: c.offset, length: c.length,
        leading: c.leading, trailing: c.trailing, height: c.height, expectedHeight, nestedExtent,
        leadingOk: c.leading === c.offset,
        trailingOk: c.trailing === Math.max(0, c.total - c.offset - c.length),
        zeroSpacer: c.spacers.filter((s) => s.rows === 0).length,
        pitchOk: c.spacers.every((s) => s.rows <= 0 || Math.abs(s.heightPx - s.rows * pitch) <= 1),
        heightOk: pitch > 0 && Math.abs(c.height - expectedHeight) <= 2 + 2 * c.nested.length,
      };
    });
    report.spacerArithmetic = { pitch, table };
    const bad = table.filter((r) => !(r.leadingOk && r.trailingOk && r.pitchOk && r.zeroSpacer === 0 && r.heightOk));
    return { ok: pitch > 0 && table.length > 0 && bad.length === 0, evidence: { pitch, containers: table.length, bad: bad.slice(0, 6), sample: table.slice(0, 8) } };
  });

  await guard("e5", "scrolling streams: the offset moves and the new band covers the row under scrollTop", async () => {
    if (!streamState.scrollable) return { ok: "skip", evidence: { verdict: "e1: the panel does not scroll" } };
    const before = await readTree(ns);
    const pitch = before.pitch;
    const candidates = streamingOpen(before).sort((a, b) => b.total - a.total);
    if (!candidates.length || pitch <= 0) {
      await shot("3-scroll-fits");
      return { ok: "skip", evidence: { verdict: "no OPEN container with total > length at this viewport — the document fits, streaming not exercised", pitch, totals: before.containers.map((c) => `${c.key}=${c.length}/${c.total}`).slice(0, 20) } };
    }
    const target = candidates[0];
    streamState.target = target.path;
    // 📍️ Aim WELL past the current window: the first row after it, plus a whole window again.
    const wantIndex = Math.min(target.total - 1, target.offset + target.length + Math.max(4, target.length));
    const scrolled = await scrollScroller(ns, 0, (target.topInScroller ?? 0) + wantIndex * pitch);
    const rest = (await readTree(ns)).viewport;
    streamState.rest = rest;
    const moved = await waitFor((t) => { const c = byPath(t, target.path); return Boolean(c) && c.offset !== target.offset; }, 6000);
    await page.waitForTimeout(1200);
    const after = await readTree(ns);
    streamState.after = after;
    const c = byPath(after, target.path);
    // 🔎️ The row a reader is looking at, in the container's own space — read off the container's real top in
    // the scroller, the way `treeWindowRowIndexAt` does, not off `scrollTop / rowHeight` of the whole body.
    const indexAtScroll = c ? Math.max(0, Math.min(c.total - 1, Math.floor((after.viewport.scrollTop - (c.topInScroller ?? 0)) / pitch))) : null;
    const bandTop = c && c.band.length ? c.band[0].index : null;
    report.stream = {
      container: printPath(target.path), total: target.total, pitch, wantIndex, scrolled,
      before: { offset: target.offset, length: target.length },
      after: c ? { offset: c.offset, length: c.length, bandTop, ring: c.ring } : "container-gone",
      indexAtScroll, movedAfterMs: moved.waitedMs, visibleBand: after.visibleBand, viewport: after.viewport,
    };
    await shot("4-scroll-streamed");
    const offsetMoved = Boolean(c) && c.offset !== target.offset;
    const covers = Boolean(c) && indexAtScroll !== null && c.offset <= indexAtScroll && indexAtScroll < c.offset + c.length;
    // 👁️ A viewport showing only spacers (or a ring that never clears) after the window settled is the failure.
    const materialised = after.visibleBand > 0;
    return { ok: offsetMoved && covers && materialised, evidence: { ...report.stream, offsetMoved, covers, materialised } };
  });

  await guard("e6", "the scroll position is unchanged (+-1px) across the streaming refresh", async () => {
    if (!streamState.rest || !streamState.after) return { ok: "skip", evidence: { verdict: "e5 did not run" } };
    const rest = streamState.rest;
    const after = streamState.after.viewport;
    const topDelta = Math.abs(after.scrollTop - rest.scrollTop);
    const heightDelta = Math.abs(after.scrollHeight - rest.scrollHeight);
    return { ok: topDelta <= 1 && heightDelta <= 1, evidence: { rest, after, topDelta, heightDelta } };
  });

  await guard("e7", "once settled the window stops moving — no oscillation for 3s at a held scroll position", async () => {
    if (!streamState.target) return { ok: "skip", evidence: { verdict: "e5 did not stream" } };
    const samples = [];
    for (let i = 0; i < 6; i++) {
      const t = await readTree(ns);
      samples.push({ ms: i * 500, scrollTop: t.viewport.scrollTop, windows: t.containers.map((c) => `${c.path}:${c.offset}:${c.length}`).join(",") });
      await page.waitForTimeout(500);
    }
    const distinct = [...new Set(samples.map((s) => s.windows))];
    const scrollDistinct = [...new Set(samples.map((s) => s.scrollTop))];
    report.stability = { distinctWindowStates: distinct.length, distinctScrollTops: scrollDistinct.length, samples: samples.map((s) => ({ ms: s.ms, scrollTop: s.scrollTop, hash: s.windows.length })) };
    return { ok: distinct.length === 1 && scrollDistinct.length === 1, evidence: { ...report.stability, states: distinct.slice(0, 3).map((s) => s.slice(0, 300)) } };
  });

  await guard("e8", "the body's windows fit TREE_WINDOW_BODY_NODE_BUDGET (1 + rows per container)", async () => {
    const tree = await readTree(ns);
    const charged = tree.containers.filter((c) => c.total > 0).map((c) => ({ path: printPath(c.path), cost: 1 + c.length }));
    const cost = charged.reduce((a, c) => a + c.cost, 0);
    // 🧯️ The retained-surface ceiling speaks through the console, not the DOM.
    const overflow = lines.filter((l) => /nodes:\s*\d+.*max_nodes|fixed-capacity|ui\.tree-window/i.test(l)).map((l) => l.slice(0, 300));
    report.budget = { budget: bodyBudget.budget, source: bodyBudget.source, cost, containersCharged: charged.length, charged: charged.slice(0, 20), overflow: overflow.slice(0, 6) };
    return { ok: bodyBudget.budget > 0 && cost <= bodyBudget.budget && overflow.length === 0, evidence: report.budget };
  });

  await guard("e9", "lazy expand: a CLOSED container (aria-expanded=false) shows zero rows and loads them on expand", async () => {
    const addressable = (tree, c) => tree.rows.some((r) => r.key === c.key && r.visible);
    let tree = await readTree(ns);
    // 🧯️ §7b: "closed" is `aria-expanded="false"` / `data-state="closed"` on the header — NEVER `rows === 0`.
    // An OPEN container whose window has not been answered yet also shows zero rows, and clicking it FOLDS it.
    let target = tree.containers.filter((c) => c.total > 0 && c.closed && addressable(tree, c)).sort((a, b) => b.total - a.total)[0] ?? null;
    let prepared = "already-closed";
    const closeEvidence = {};
    if (!target) {
      // 🗂️ Nothing ships closed on this lane: close the biggest OPEN, materialised container by its header,
      // assert it really emptied (total kept), and expand it again — the same lazy path.
      const open = tree.containers.filter((c) => c.total > 0 && !c.closed && c.length > 0 && addressable(tree, c)).sort((a, b) => b.total - a.total)[0];
      if (!open) return { ok: "skip", evidence: { verdict: "no addressable windowed container to close and re-expand", containers: tree.containers.map((c) => `${printPath(c.path)}=${c.length}/${c.total} closed=${c.closed}`).slice(0, 12) } };
      await clickRowKey(open.key);
      const folded = await waitFor((t) => { const c = byPath(t, open.path); return !c || c.closed || c.length === 0 || c.hidden; }, 8000);
      tree = folded.tree;
      target = byPath(tree, open.path);
      prepared = `closed-first:${open.key}`;
      closeEvidence.closedByProbe = target ? { closed: target.closed, headerExpanded: target.headerExpanded, rows: target.length, total: target.total, totalKept: target.total === open.total } : "container-gone";
      if (!target || !target.closed) return { ok: false, evidence: { verdict: "clicking the header did not close the container", prepared, ...closeEvidence } };
    }
    const closedRows = target.length;
    const closedBand = target.band.length;
    const closedTotal = target.total;
    await clickRowKey(target.key);
    const opened = await waitFor((t) => { const c = byPath(t, target.path); return Boolean(c) && c.length > 0 && !c.hidden && !c.closed; }, 15000);
    const c = byPath(opened.tree, target.path);
    const ring = opened.tree.containers.filter((x) => x.ring).map((x) => printPath(x.path));
    report.lazyExpand = { container: printPath(target.path), prepared, ...closeEvidence, total: closedTotal, closedRows, closedBand, headerWhenClosed: target.headerExpanded, openedRows: c ? c.length : null, openedBand: c ? c.band.length : null, openedHeader: c ? c.headerExpanded : null, waitedMs: opened.waitedMs, ringsAfter: ring.slice(0, 6) };
    await shot("9-lazy-expand");
    // 🔁️ Restore the authored state for the steps that follow: re-close only what shipped closed.
    if (prepared === "already-closed" && c && c.length > 0) await clickRowKey(target.key).catch(() => {});
    return { ok: closedRows === 0 && closedBand === 0 && Boolean(c) && c.length > 0 && c.total === closedTotal, evidence: report.lazyExpand };
  });

  await guard("e10", "a nested window's parent still shows the row that owns it", async () => {
    const tree = await readTree(ns);
    const nested = tree.containers.filter((c) => c.parentPath && !c.hidden);
    if (!nested.length) return { ok: "skip", evidence: { verdict: "this document has no nested windowed container on screen", containers: tree.containers.map((c) => printPath(c.path)).slice(0, 16) } };
    const table = nested.map((c) => {
      const parent = byPath(tree, c.parentPath);
      return {
        path: printPath(c.path), ownerRowIndex: c.ownerRowIndex,
        parent: parent ? { path: printPath(parent.path), offset: parent.offset, length: parent.length } : null,
        covered: Boolean(parent) && Number.isFinite(c.ownerRowIndex) && parent.offset <= c.ownerRowIndex && c.ownerRowIndex < parent.offset + parent.length,
        pathOk: c.path === `${c.parentPath}${PATH_SEP}${c.key}`,
      };
    });
    report.nesting = table;
    const bad = table.filter((r) => !(r.covered && r.pathOk));
    return { ok: bad.length === 0, evidence: { nested: table.length, bad: bad.slice(0, 6), sample: table.slice(0, 8) } };
  });

  await guard("e11", "no duplicate data-tree-window-path in the body and no [tree-window] duplicate key fault", async () => {
    const tree = await readTree(ns);
    const dupConsole = lines.filter((l) => /\[tree-window\]\s*duplicate/i.test(l)).map((l) => l.slice(0, 300));
    return { ok: tree.duplicatePaths.length === 0 && dupConsole.length === 0, evidence: { duplicatePaths: tree.duplicatePaths.slice(0, 8).map((p) => printPath(p)), dupConsole: dupConsole.slice(0, 4), containers: tree.containers.length } };
  });

  await guard("e13", "no view-context rejection: the paths the host sends are legal identifiers", async () => {
    // 🔑️ §7b: the U+001F separator made every `treeWindows` view state an illegal identifier, so EVERY
    // refresh was rejected — which showed up as unrelated-looking pick and lazy-expand failures.
    const rejects = lines.filter((l) => /view context:?\s*invalid identifier|tree window refresh failed|unsendable path|invalid identifier/i.test(l)).map((l) => l.slice(0, 300));
    report.viewContext = { rejects: rejects.slice(0, 10), rejectCount: rejects.length, separator: `U+${PATH_SEP.codePointAt(0).toString(16).toUpperCase().padStart(4, "0")}` };
    return { ok: rejects.length === 0, evidence: report.viewContext };
  });

  await guard("e14", "nothing open stays empty: a visible open container has rows, and one scrolled into view gets its window", async () => {
    await settleTree(20000, 3000);
    const tree = await readTree(ns);
    // (i) everything OPEN and on screen must have materialised something by now.
    const emptyOnScreen = tree.containers.filter((c) => !c.closed && c.total > 0 && c.length === 0 && c.inViewport && !c.hidden)
      .map((c) => ({ path: printPath(c.path), total: c.total, ring: c.ring, header: c.headerExpanded }));
    // (ii) an OPEN container below the fold: scroll it to the top of the viewport and it must fill in.
    const below = tree.containers.filter((c) => !c.closed && c.total > 0 && !c.inViewport && !c.hidden && c.topInScroller !== null)
      .sort((a, b) => (b.total - b.length) - (a.total - a.length))[0] ?? null;
    let scrolledIn = "no OPEN container below the fold";
    let filled = true;
    if (below && streamState.scrollable) {
      const before = { rows: below.length, total: below.total };
      const scrolled = await scrollScroller(ns, 0, below.topInScroller);
      const got = await waitFor((t) => { const c = byPath(t, below.path); return Boolean(c) && c.inViewport && c.length > 0; }, 8000);
      const after = byPath(got.tree, below.path);
      filled = Boolean(after) && after.length > 0;
      scrolledIn = { path: printPath(below.path), before, scrolled, after: after ? { rows: after.length, offset: after.offset, total: after.total, inViewport: after.inViewport, ring: after.ring } : "container-gone", waitedMs: got.waitedMs };
      await shot("10-scrolled-in");
    }
    report.nothingOpenStaysEmpty = { emptyOnScreen, scrolledIn };
    return { ok: emptyOnScreen.length === 0 && filled, evidence: report.nothingOpenStaysEmpty };
  });

  await guard("e12", "still no `.more` key and no `+N` label after streaming", async () => {
    const tree = await readTree(ns);
    await page.setViewportSize({ width: 1600, height: 1000 });
    await page.waitForTimeout(2500);
    return { ok: tree.dotMore.length === 0 && tree.plusLabels.length === 0 && tree.bodyPlus.length === 0, evidence: { dotMore: tree.dotMore, plusLabels: tree.plusLabels, bodyPlusText: tree.bodyPlus, rowsScanned: tree.rows.length } };
  });

  // ── (f) collapse then re-expand ──────────────────────────────────────────
  await guard("f", "collapse empties a container (total kept) and re-expand restores it", async () => {
    await scrollPanel(ns, 0);
    await page.waitForTimeout(1200);
    const before = await readTree(ns);
    const target = [...before.containers].filter((c) => c.total > 0 && !c.hidden && before.rows.some((r) => r.key === c.key)).sort((a, b) => b.total - a.total)[0];
    if (!target) return { ok: false, evidence: { reason: "no windowed container whose owner row is addressable", containers: before.containers.map((c) => c.key) } };
    await clickRowKey(target.key);
    const closed = await waitFor((t) => { const c = byKey(t, target.key); return !c || c.length === 0 || c.hidden; }, 5000);
    const closedC = byKey(closed.tree, target.key);
    const closedRow = closed.tree.rows.find((r) => r.key === target.key);
    await shot("5-collapse");
    await clickRowKey(target.key);
    const back = await waitFor((t) => { const c = byKey(t, target.key); return Boolean(c) && c.length > 0; }, 5000);
    const backC = byKey(back.tree, target.key);
    await shot("6-reexpand");
    report.collapse = {
      container: target.key, before: { total: target.total, length: target.length },
      closed: closedC ? { total: closedC.total, length: closedC.length, hidden: closedC.hidden } : "container-gone",
      closedRowExpanded: closedRow?.expanded ?? null,
      reopened: backC ? { total: backC.total, length: backC.length } : "container-gone",
    };
    const closedOk = !closedC || closedC.length === 0 || closedC.hidden;
    const totalKept = !closedC || closedC.total === target.total;
    const reopenedOk = Boolean(backC) && backC.length > 0;
    return { ok: closedOk && totalKept && reopenedOk, evidence: report.collapse };
  });

  // ── (g) expansion state survives a body refresh ──────────────────────────
  await guard("g", "a host-closed container stays closed across a body refresh", async () => {
    const before = await readTree(ns);
    const target = [...before.containers].filter((c) => c.total > 0 && !c.hidden && before.rows.some((r) => r.key === c.key)).sort((a, b) => b.total - a.total)[0];
    if (!target) return { ok: false, evidence: { reason: "no addressable windowed container" } };
    await clickRowKey(target.key);
    await waitFor((t) => { const c = byKey(t, target.key); return !c || c.length === 0 || c.hidden; }, 5000);
    const leaf = (await readTree(ns)).rows.find((r) => r.key !== target.key && r.expanded === null && r.visible && !r.key.startsWith(`${target.key}.`));
    let refreshed = "no-leaf-row";
    if (leaf) { await clickRowKey(leaf.key); refreshed = `picked:${leaf.key}`; }
    await page.waitForTimeout(2500);
    const after = await readTree(ns);
    const c = byKey(after, target.key);
    const row = after.rows.find((r) => r.key === target.key);
    report.refreshSurvival = { container: target.key, refreshed, afterContainer: c ? { total: c.total, length: c.length, hidden: c.hidden } : "container-gone", rowExpanded: row?.expanded ?? null };
    await shot("7-refresh");
    const stillClosed = (!c || c.length === 0 || c.hidden) && row?.expanded !== "true";
    // 🔁️ restore, so a later run of this probe starts from the authored state
    if (stillClosed) await clickRowKey(target.key).catch(() => {});
    return { ok: stillClosed, evidence: report.refreshSurvival };
  });

  // ── (h) a pick row selects, with no console error ────────────────────────
  await guard("h", "clicking a leaf row selects it without a console error", async () => {
    const from = lines.length;
    const tree = await readTree(ns);
    const owned = new Set(tree.containers.flatMap((c) => c.rowKeys));
    const leaf = tree.rows.find((r) => r.expanded === null && r.visible && owned.has(r.key)) ?? tree.rows.find((r) => r.expanded === null && r.visible);
    if (!leaf) return { ok: false, evidence: { reason: "no visible leaf row", rows: tree.rows.slice(0, 10) } };
    await clickRowKey(leaf.key);
    const after = await readTree(ns);
    const row = after.rows.find((r) => r.key === leaf.key);
    const faults = faultsSince(from);
    report.pick = { row: leaf.key, label: leaf.label, selected: row?.selected ?? null, selectedRows: after.rows.filter((r) => r.selected === "true").map((r) => r.key).slice(0, 8), faults: faults.slice(0, 6), faultCount: faults.length };
    await shot("8-pick");
    return { ok: (row?.selected === "true" || report.pick.selectedRows.length > 0) && faults.length === 0, evidence: report.pick };
  });

  // ── (i) console findings ─────────────────────────────────────────────────
  await guard("i", "console has no pageerror / non-[DEBUG] error and no dropped-refresh diagnostics", async () => {
    const faults = lines.filter(isFault).map((l) => l.slice(0, 500));
    const pageErrors = faults.filter((l) => l.includes(" pageerror "));
    const httpFaults = faults.filter((l) => l.includes(" httpfault "));
    report.httpFaults = httpFaults.slice(0, 20);
    const debugErrors = lines.filter((l) => /^\d+ error /.test(l) && DEBUG_PREFIXES.some((p) => l.includes(p))).length;
    const diagnostics = lines.filter((l) => /tree-window|fixed-capacity|refreshUi dropped|dropped action|surface-render|trapped|panicked|UiText|admission/i.test(l)).map((l) => l.slice(0, 500));
    report.console = { faults: faults.slice(0, 60), faultCount: faults.length, pageErrorCount: pageErrors.length, debugErrorCount: debugErrors, diagnostics: diagnostics.slice(0, 60), diagnosticCount: diagnostics.length };
    return { ok: faults.length === 0, evidence: { faultCount: faults.length, pageErrorCount: pageErrors.length, httpFaultCount: httpFaults.length, debugErrorCount: debugErrors, firstFaults: faults.slice(0, 6), diagnosticCount: diagnostics.length, firstDiagnostics: diagnostics.slice(0, 6) } };
  });

  void oversized;
} catch (error) {
  record("fatal", "probe crashed", false, { error: String(error).slice(0, 600) });
  await shot("error");
}

flush();
const pass = steps.filter((s) => s.status === "PASS").length;
const fail = steps.filter((s) => s.status === "FAIL").length;
const skip = steps.filter((s) => s.status === "SKIP").length;
for (const s of steps) console.log(`${s.status} ${s.id} ${s.title}`);
console.log(`tree-window-probe PASS=${pass} FAIL=${fail}${skip ? ` SKIP=${skip}` : ""}`);
console.log(`out ${outDir}`);
await browser.close();
