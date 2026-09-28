/** 🔬️ S18 §14c (C12 P1): Home windowed-table DOM facts after sign-in — at rest, after wheel over the rows, after
 * PageDown/End on a focused row, after a programmatic scroll to the bottom (spacers, row range, scroller geometry, label).
 * usage: bun s18-probe-home-rows.mjs <url> [locale] */
import { USERS, boot, openSessions, signIn } from "../wp-c11/c11-lib.mjs";
const [url = "http://127.0.0.1:6540/", locale = "en-US"] = process.argv.slice(2);
const { browser, sessions } = await openSessions([url], { locale, users: [USERS[0]] });
const [A] = sessions;
const facts = () => A.page.evaluate(() => {
  const s = document.querySelector('[data-slot="table-window-scroll"]');
  const c = document.querySelector('[data-tree-window-key="framework.window.table"]');
  const rows = [...(c?.querySelectorAll("[data-tree-window-row]") ?? [])];
  const sp = [...(c?.querySelectorAll("[data-tree-window-spacer]") ?? [])].map((e) => `${e.getAttribute("data-tree-window-spacer")}:${e.getBoundingClientRect().height}`);
  const pitch = rows.length > 1 ? rows[1].getBoundingClientRect().top - rows[0].getBoundingClientRect().top : null;
  const grid = s?.parentElement;
  const label = [...document.querySelectorAll("[role='status'], [data-slot='table-window-status'], [aria-live]")].map((e) => (e.textContent ?? '').trim()).find((t) => /Rows|Zeilen/u.test(t)) ?? null;
  return { label, spaceRows: document.querySelectorAll('[data-ui-node-key^="space:"]').length, scroll: s ? { top: s.scrollTop, h: s.scrollHeight, c: s.clientHeight } : null, rows: rows.length, firstRow: rows[0]?.getAttribute("data-tree-window-row"), lastRow: rows.at(-1)?.getAttribute("data-tree-window-row"), pitch, spacers: sp,
    rowPx: getComputedStyle(document.documentElement).getPropertyValue("--tree-row-ui-spacing") || null,
    gridKey: grid?.getAttribute("data-ui-node-key"), windowBody: grid?.closest("[id^='framework.window']")?.id ?? null, panel: grid?.closest("[data-panel-key],[data-panel-id]")?.getAttribute("data-panel-key") ?? null };
});
try {
  await boot(A); await signIn(A); await A.page.waitForTimeout(8_000);
  console.log("TOP", JSON.stringify(await facts()));
  const first = A.page.locator('[data-ui-node-key^="space:"]').first();
  const box = await first.boundingBox();
  await A.page.mouse.move((box?.x ?? 100) + 200, (box?.y ?? 120) + 10);
  for (let turn = 0; turn < 8; turn += 1) { await A.page.mouse.wheel(0, 400); await A.page.waitForTimeout(400); }
  await A.page.waitForTimeout(2_000);
  console.log("WHEEL", JSON.stringify(await facts()));
  await first.focus().catch(() => {});
  for (const key of ["PageDown", "PageDown", "End"]) { await A.page.keyboard.press(key); await A.page.waitForTimeout(600); }
  await A.page.waitForTimeout(2_000);
  console.log("KEYS", JSON.stringify(await facts()), "focused", await A.page.evaluate(() => document.activeElement?.getAttribute("data-ui-node-key") ?? document.activeElement?.tagName));
  const tracked = await A.page.evaluate(async () => {
    const s = document.querySelector('[data-slot="table-window-scroll"]');
    window.__s18Scroller = s;
    const log = [];
    s?.addEventListener("scroll", () => log.push(`scroll ${Math.round(performance.now())} top=${s.scrollTop}`));
    window.__s18Log = log;
    if (s) s.scrollTop = 40;
    await new Promise((r) => setTimeout(r, 3000));
    const now = document.querySelector('[data-slot="table-window-scroll"]');
    const rows = [...document.querySelectorAll('[data-tree-window-row]')].map((e) => e.getAttribute("data-tree-window-row"));
    return { sameScroller: now === s, oldConnected: s?.isConnected, oldTop: s?.scrollTop, newTop: now?.scrollTop, h: now?.scrollHeight, c: now?.clientHeight, rows: `${rows[0]}..${rows.at(-1)} (${rows.length})`, log: log.slice(0, 12) };
  });
  console.log("TRACK", JSON.stringify(tracked));
  const walk = await A.page.evaluate(async () => {
    const out = [];
    for (const target of [0, 300, 150, 420, 90, 600, 0]) {
      const s = document.querySelector('[data-slot="table-window-scroll"]');
      s.scrollTop = target;
      await new Promise((r) => setTimeout(r, 1500));
      const now = document.querySelector('[data-slot="table-window-scroll"]');
      const rows = [...document.querySelectorAll('[data-tree-window-row]')].map((e) => Number(e.getAttribute("data-tree-window-row")));
      const first = Math.floor(now.scrollTop / 24);
      out.push(`${target}->${now.scrollTop} rows ${rows[0]}..${rows.at(-1)} visible ${first}..${first + 10} covered=${rows.includes(first) && rows.includes(Math.min(first + 10, 35))}`);
    }
    return out;
  });
  console.log("WALK", JSON.stringify(walk));
  const computed = await A.page.evaluate(async () => {
    const url = "/@fs" + encodeURI("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx");
    const m = await import(url);
    const root = document.querySelector('[data-slot="table-window-scroll"]').closest('[data-ui-node-key="framework.window.table"]') ?? document.querySelector('[data-slot="table-window-scroll"]').parentElement;
    const viewport = m.treeWindowScrollViewport(root);
    const height = m.treeWindowViewportMetrics(viewport).height;
    const containers = m.treeWindowContainersUnder(root, viewport);
    const first = m.treeWindowServedRequestsV1(containers, height, new Map());
    const shortMemory = new Map([...first.memory].map(([k, v]) => [k, { ...v, asked: v.asked }]));
    const second = m.treeWindowServedRequestsV1(containers, height, shortMemory);
    return { viewportTag: viewport?.getAttribute?.("data-slot") ?? viewport?.tagName, height, containers: containers.map((c) => ({ key: c.key, total: c.total, offset: c.offset, length: c.length, top: c.top, rows: c.rows?.length })), first: first.requests, firstMemory: [...first.memory.values()], second: second.requests, secondMemory: [...second.memory.values()] };
  }).catch((error) => ({ error: String(error).slice(0, 300) }));
  console.log("COMPUTED", JSON.stringify(computed));
  await A.page.evaluate(() => { const s = document.querySelector('[data-slot="table-window-scroll"]'); if (s) s.scrollTop = s.scrollHeight; });
  await A.page.waitForTimeout(4_000);
  console.log("BOTTOM", JSON.stringify(await facts()));
  for (const l of A.lines.filter((l) => /pageerror|error|refused/u.test(l)).slice(-15)) console.log("LINE", l.slice(0, 700));
} catch (error) {
  console.log("ERROR", String(error?.stack ?? error).slice(0, 800));
} finally {
  await browser.close();
}
