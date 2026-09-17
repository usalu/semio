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
 * Usage: cd <ticket> && SEMIO_PROBE_URL=… SEMIO_PROBE_TREE_NS=… SEMIO_PROBE_OUT=w3/cad bun 🐍️tree-window-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6020/?plugin=cad";
const ns = process.env.SEMIO_PROBE_TREE_NS ?? "cad-play-document";
const bootSeconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 120);
const panelName = process.env.SEMIO_PROBE_PANEL ?? "Artifact";
const wantedExample = process.env.SEMIO_PROBE_EXAMPLE ?? "";
const ROWS_MAX = Number(process.env.SEMIO_PROBE_ROWS_MAX ?? 128);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "w3/tree-window");
mkdirSync(outDir, { recursive: true });

const lines = [];
const steps = [];
const report = { lane: { url, ns, panelName }, startedAt: new Date().toISOString(), steps };
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
  const containers = [...(rootEl.querySelectorAll('[data-tree-window-key]') ?? [])].map((el) => {
    const rect = el.getBoundingClientRect();
    const spacers = [...el.querySelectorAll('[data-slot="tree-window-spacer"]')]
      .filter((s) => s.closest("[data-tree-window-key]") === el)
      .map((s) => ({ edge: s.getAttribute("data-tree-window-spacer"), rows: Number(s.getAttribute("data-tree-window-rows") ?? "NaN"), heightPx: Math.round(s.getBoundingClientRect().height) }));
    const ownRows = [...el.querySelectorAll(`[id^="${prefix}"]`)].filter((r) => r.closest("[data-tree-window-key]") === el);
    return {
      key: el.getAttribute("data-tree-window-key"),
      slot: el.getAttribute("data-slot"),
      total: num(el, "data-tree-window-total"),
      offset: num(el, "data-tree-window-offset"),
      length: num(el, "data-tree-window-length"),
      hidden: el.hasAttribute("hidden") || rect.height === 0,
      top: Math.round(rect.top),
      height: Math.round(rect.height),
      leading: spacers.filter((s) => s.edge === "leading").reduce((a, s) => a + s.rows, 0),
      trailing: spacers.filter((s) => s.edge === "trailing").reduce((a, s) => a + s.rows, 0),
      spacers,
      rowKeys: ownRows.map((r) => r.id.slice(prefix.length)),
    };
  }).filter((c) => c.rowKeys.length > 0 || (c.key !== null && nsKeys.has(c.key)));
  return {
    rows,
    containers,
    dotMore: rows.filter((r) => r.key.endsWith(".more")).map((r) => r.key),
    plusLabels: rows.filter((r) => /^\+\d+$/.test(r.label)).map((r) => ({ key: r.key, label: r.label })),
    bodyPlus: (rootEl.innerText ?? "").split("\n").map((l) => l.trim()).filter((l) => /^\+\d+$/.test(l)).slice(0, 8),
    viewport: viewportEl ? { found: true, scrollTop: Math.round(viewportEl.scrollTop), scrollHeight: Math.round(viewportEl.scrollHeight), clientHeight: Math.round(viewportEl.clientHeight) } : { found: false },
  };
}, namespace);

const scrollPanel = (namespace, fraction) => page.evaluate(([nsIn, f]) => {
  const anchor = document.querySelector(`[id^="panel:${nsIn}/"]`);
  const vp = anchor?.closest('[data-slot="scroll-area-viewport"]');
  if (!vp) return { ok: false };
  const max = vp.scrollHeight - vp.clientHeight;
  vp.scrollTop = Math.round(max * f);
  vp.dispatchEvent(new Event("scroll", { bubbles: false }));
  return { ok: true, scrollTop: Math.round(vp.scrollTop), max: Math.round(max) };
}, [namespace, fraction]);

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
  await page.mouse.click(box.x + Math.min(90, box.width / 2), box.y + box.height / 2);
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
    if (over.length === 0) {
      const beforeSignature = treeSignature(tree);
      const combo = page.locator('[role="combobox"]').first();
      if (await combo.count()) {
        await combo.click({ timeout: 5000 }).catch(() => {});
        await page.waitForTimeout(600);
        const options = await page.evaluate(() => [...document.querySelectorAll('[role="option"]')].map((o) => o.textContent?.trim()).filter(Boolean));
        const wanted = wantedExample && options.find((o) => o.includes(wantedExample));
        const target = wanted ?? options[options.length - 1];
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
        } else { switched = `no-option (${options.length})`; await page.keyboard.press("Escape"); }
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

  // ── (e) scroll → offset moves, new row keys arrive ───────────────────────
  await guard("e", "scrolling the panel viewport moves a container's offset (host→guest round trip)", async () => {
    let before = await readTree(ns);
    // 📏️ The design's scroll container is the panel's `📜️Scrollable` viewport. If it is not height-bounded
    // there is nothing to scroll, so shrink the browser window once before calling the round trip untested.
    let extent = before.viewport.found ? before.viewport.scrollHeight - before.viewport.clientHeight : 0;
    let resized = "not-needed";
    if (extent <= 0) {
      await page.setViewportSize({ width: 1600, height: 520 });
      await page.waitForTimeout(3000);
      before = await readTree(ns);
      extent = before.viewport.found ? before.viewport.scrollHeight - before.viewport.clientHeight : 0;
      resized = `1600x520 -> extent ${extent}`;
    }
    if (extent <= 0) {
      const chain = await scrollChain(ns);
      report.scrollChain = chain;
      await page.setViewportSize({ width: 1600, height: 1000 });
      await shot("3-no-scroll");
      return { ok: "skip", evidence: { verdict: "panel viewport is not height-bounded (scrollHeight == clientHeight); scroll round trip not exercised", resized, viewport: before.viewport, scrollableAncestors: chain.filter((c) => c.extent > 0 || /auto|scroll/.test(c.overflowY)).slice(0, 6), chain: chain.slice(0, 10) } };
    }
    const target = [...before.containers].filter((c) => c.total > c.length && !c.hidden).sort((a, b) => b.total - a.total)[0] ?? biggest(before);
    if (!target) return { ok: false, evidence: { reason: "no windowed container to scroll", resized } };
    const snap = (t) => { const c = byKey(t, target.key); return c ? { offset: c.offset, length: c.length, first: c.rowKeys[0] ?? null, last: c.rowKeys[c.rowKeys.length - 1] ?? null } : null; };
    const base = snap(before);
    const legs = [];
    for (const [name, fraction] of [["middle", 0.5], ["bottom", 1]]) {
      const scrolled = await scrollPanel(ns, fraction);
      const prev = snap(await readTree(ns));
      const waited = await waitFor((t) => { const c = byKey(t, target.key); return Boolean(c) && (c.offset !== base.offset || (c.rowKeys[0] ?? null) !== base.first); }, 5000);
      const after = snap(waited.tree);
      legs.push({ leg: name, scrolled, before: prev, after, offsetMoved: Boolean(after) && after.offset !== base.offset, keysChanged: Boolean(after) && (after.first !== base.first || after.last !== base.last), waitedMs: waited.waitedMs });
      await shot(name === "middle" ? "3-scroll-mid" : "4-scroll-bottom");
    }
    report.scroll = { container: target.key, total: target.total, resized, viewport: before.viewport, base, legs };
    await page.setViewportSize({ width: 1600, height: 1000 });
    await page.waitForTimeout(2000);
    return { ok: legs.some((l) => l.offsetMoved) && legs.some((l) => l.keysChanged), evidence: report.scroll };
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
