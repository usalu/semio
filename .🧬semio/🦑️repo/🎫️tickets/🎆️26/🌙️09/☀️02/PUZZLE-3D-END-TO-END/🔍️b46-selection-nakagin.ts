/** 🎯️ Wave B46 — why NO world selection can be established on the 180-object Nakagin document.
 *
 * Boots the React serve, switches the example to Nakagin, then answers two questions separately with
 * host-side taps that need no guest round trip:
 *
 * 1. **canvas pick** — sweeps a grid of canvas points and reports, per point, whether R3F's own raycast
 *    reached an instance (`b46.pick.instance`), whether the pointerup AABB fallback resolved one
 *    (`b46.pick.fallback … hit=`), or whether the click was treated as background
 *    (`b46.pick.empty`). The first point that reaches an instance is then waited out to a settled
 *    `data-selection-json`, with the guest's own `interaction selection lost reason=…` verdict quoted.
 * 2. **outliner** — opens `framework.panel.artifact` and reports the published node census for
 *    `puzzle.3d.play.document` (`b46.panel`) beside the tree rows the DOM actually carries, so
 *    "the guest published nothing" and "the host rendered nothing" are distinguishable.
 *
 * Run: `bun 🔍️b46-selection-nakagin.ts [--port=6013] [--label=before] [--grid=5x4] [--example=nakagin|forest]`.
 * Output: `🗑️generated/b46-<stamp>-<label>.md` / `.ndjson`. Ticket 26/09/02/PUZZLE-3D-END-TO-END. */
import { chromium } from "playwright";
import { appendFileSync, mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const TICKET = import.meta.dir;
const OUT = join(TICKET, "🗑️generated");
mkdirSync(OUT, { recursive: true });
const label = process.argv.find((a) => a.startsWith("--label="))?.slice(8) ?? "run";
const stamp = `${new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19)}-${label}`;
const port = process.argv.find((a) => a.startsWith("--port="))?.slice(7) ?? "6013";
const example = process.argv.find((a) => a.startsWith("--example="))?.slice(10) ?? "nakagin";
const grid = (process.argv.find((a) => a.startsWith("--grid="))?.slice(7) ?? "5x4").split("x").map(Number);
/** ⏳️ How long one pick may take to reach the GUEST's own selection lane — 40 s on the 180-object
 * document is one queued command, and the queue behind a pick is deeper than that. */
const guestSettleMs = Number(process.argv.find((a) => a.startsWith("--guest-settle="))?.slice(15) ?? "40") * 1000;
const mdPath = join(OUT, `b46-${stamp}.md`);
const ndPath = join(OUT, `b46-${stamp}.ndjson`);
writeFileSync(ndPath, "");

const t0 = Date.now();
const lines: string[] = [];
const log = (m: string) => {
  const row = `[${((Date.now() - t0) / 1000).toFixed(2)}s] ${m}`;
  lines.push(row);
  console.log(row);
};
const emit = (record: Record<string, unknown>) => appendFileSync(ndPath, `${JSON.stringify(record)}\n`);

const tape: { ms: number; text: string }[] = [];
const TAP = /b46\.|interaction selection lost|command ingress|performInvocation settled|refreshUi dropped|ui\.fixed-capacity|fixed UI|panel:puzzle3d-play-document|panic|worker fault/i;
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (msg) => tape.push({ ms: Date.now() - t0, text: msg.text().slice(0, 500) }));
page.on("pageerror", (error) => tape.push({ ms: Date.now() - t0, text: `pageerror: ${String(error).slice(0, 300)}` }));
const mark = () => tape.length;
const since = (m: number) => tape.slice(m).filter((row) => TAP.test(row.text)).map((row) => `+${row.ms}ms ${row.text}`);

const instances = async () =>
  page
    .evaluate(() => {
      const element = document.querySelector("#puzzle3d-main-perspective [data-instances-json]") as HTMLElement | null;
      const raw = element?.getAttribute("data-instances-json") || "[]";
      let parsed: { id?: string; position?: number[]; disabled?: boolean; meshId?: string }[] = [];
      try {
        parsed = JSON.parse(raw);
      } catch {
        parsed = [];
      }
      return { bytes: raw.length, count: parsed.length, disabled: parsed.filter((o) => o.disabled).length, sample: parsed.slice(0, 2), meshIds: [...new Set(parsed.map((o) => o.meshId ?? "?"))].slice(0, 6) };
    })
    .catch(() => ({ bytes: 0, count: 0, disabled: 0, sample: [] as unknown[], meshIds: [] as string[] }));

const selection = async () =>
  page
    .evaluate(() => {
      const read = (attribute: string) =>
        Array.from(document.querySelectorAll(`[${attribute}]`)).map((element) => ({
          surface: (element.closest("[id]") as HTMLElement | null)?.id ?? "?",
          raw: (element.getAttribute(attribute) || "").slice(0, 2000),
        }));
      return { selection: read("data-selection-json"), guest: read("data-guest-selection-json") };
    })
    .catch(() => ({ selection: [] as { surface: string; raw: string }[], guest: [] as { surface: string; raw: string }[] }));

/** 🔦️ `worldSurfaceSelectionDomV1` publishes `selectedIds`, NOT `ids` — wave B44's own probe read
 * `ids`/`activeObjectId` and therefore read `[]` on every pane of every document, which is the whole of
 * its §6.2 "no world selection can be established". `pane` chooses the merged pane attribute
 * (`data-selection-json`, what the pane PAINTS) or the guest's own lane (`data-guest-selection-json`). */
const selectedIds = async (pane: "painted" | "guest" = "painted") => {
  const state = await selection();
  return (pane === "painted" ? state.selection : state.guest).flatMap((row) => {
    try {
      const parsed = JSON.parse(row.raw || "{}") as { selectedIds?: string[]; activeObjectId?: string };
      return [...(parsed.selectedIds ?? []), ...(parsed.activeObjectId ? [parsed.activeObjectId] : [])];
    } catch {
      return [] as string[];
    }
  });
};

const settle = async <T>(read: () => Promise<T>, done: (value: T) => boolean, budgetMs: number) => {
  const started = Date.now();
  let value = await read();
  while (!done(value) && Date.now() - started < budgetMs) {
    await page.waitForTimeout(300);
    value = await read();
  }
  return { value, waitedMs: Date.now() - started, ok: done(value) };
};

log(`goto http://127.0.0.1:${port}/ example=${example} grid=${grid.join("x")}`);
await page.goto(`http://127.0.0.1:${port}/`, { waitUntil: "domcontentloaded", timeout: 120_000 });
const booted = await settle(instances, (value) => value.count > 0, 180_000);
log(`boot instances=${booted.value.count} bytes=${booted.value.bytes} waitedMs=${booted.waitedMs}`);
if (!booted.ok) {
  writeFileSync(mdPath, `# b46 ${stamp}\n\n${lines.join("\n")}\n`);
  await browser.close();
  process.exit(1);
}

if (example === "nakagin") {
  const switchMark = mark();
  const picker = page.locator('[id="playground.navbar.fixture"]');
  const native = picker.locator("select").or(page.locator('select[id="playground.navbar.fixture"]')).first();
  if (await native.count().catch(() => 0)) {
    const labels = await native.locator("option").allTextContents();
    await native.selectOption({ label: labels.find((text) => /nakagin/i.test(text)) ?? "" }).catch(() => {});
  } else {
    // 🧯️ `force: true` — wave B44 §1.0: a hit-tested click on the navbar select trigger times out.
    await page.keyboard.press("Escape").catch(() => {});
    await picker.first().click({ timeout: 6000, force: true }).catch(() => {});
    await page.waitForTimeout(800);
    await page.locator('[role="option"]').filter({ hasText: /nakagin/i }).first().click({ timeout: 6000, force: true }).catch(() => {});
  }
  const swapped = await settle(instances, (value) => value.count >= 100, 180_000);
  log(`example-switch instances=${swapped.value.count} bytes=${swapped.value.bytes} waitedMs=${swapped.waitedMs} disabled=${swapped.value.disabled} meshIds=${JSON.stringify(swapped.value.meshIds)}`);
  log(`example-switch sample=${JSON.stringify(swapped.value.sample).slice(0, 400)}`);
  emit({ phase: "example-switch", ...swapped.value, waitedMs: swapped.waitedMs, tail: since(switchMark).slice(-12) });
}

// ⏳️ The shell is still registering brush meshes and mounting panels for ~20 s after the first
// instance lane lands; a click inside that window reaches no handler at all (measured: six picks with
// NO `b46.pick.*` tap of any kind, and `panels=[]`). Settle before measuring anything.
const settleSeconds = Number(process.argv.find((a) => a.startsWith("--settle="))?.slice(9) ?? "25");
await page.waitForTimeout(settleSeconds * 1000);
const census = await instances();
log(`census instances=${census.count} bytes=${census.bytes} disabled=${census.disabled} settledFor=${settleSeconds}s`);

//#region 🔖️CanvasPick
const canvas = page.locator("#puzzle3d-main-perspective canvas").last();
const box = await canvas.boundingBox();
log(`perspective canvas box=${JSON.stringify(box)}`);
const hops: { at: string; kind: string; tail: string[] }[] = [];
let landed: { at: string; id: string } | null = null;
if (box) {
  for (let row = 1; row <= (grid[1] ?? 4); row += 1) {
    for (let column = 1; column <= (grid[0] ?? 5); column += 1) {
      if (landed) break;
      const fraction = { x: column / ((grid[0] ?? 5) + 1), y: row / ((grid[1] ?? 4) + 1) };
      const pointMark = mark();
      await canvas.click({ position: { x: box.width * fraction.x, y: box.height * fraction.y }, timeout: 6000, force: true }).catch(() => {});
      await page.waitForTimeout(900);
      const tail = since(pointMark);
      const instanceHit = tail.find((text) => text.includes("b46.pick.instance"));
      const fallback = tail.find((text) => text.includes("b46.pick.fallback"));
      const kind = instanceHit ? "instance" : fallback && !fallback.includes("hit=none") ? "fallback" : tail.some((text) => text.includes("b46.pick.empty")) ? "empty" : "silent";
      const at = `${fraction.x.toFixed(2)},${fraction.y.toFixed(2)}`;
      log(`pick ${at} kind=${kind} ${JSON.stringify(tail.slice(0, 4))}`);
      hops.push({ at, kind, tail: tail.slice(0, 6) });
      emit({ phase: "pick", at, kind, tail: tail.slice(0, 6) });
      if (kind === "instance" || kind === "fallback") {
        const settled = await settle(() => selectedIds("guest"), (ids) => ids.length > 0, guestSettleMs);
        const state = await selection();
        log(`pick ${at} SETTLED guestIds=${JSON.stringify(settled.value)} paintedIds=${JSON.stringify(await selectedIds())} waitedMs=${settled.waitedMs}`);
        log(`pick ${at} selection-json=${JSON.stringify(state.selection).slice(0, 600)}`);
        log(`pick ${at} guest-selection-json=${JSON.stringify(state.guest).slice(0, 400)}`);
        log(`pick ${at} verdicts=${JSON.stringify(since(pointMark).filter((text) => /interaction selection lost|command ingress|performInvocation settled/.test(text)).slice(-10))}`);
        // 🧮️ What the guest actually spent the wait on: one line per action id, so "the pick never
        // settled" and "the pick settled but no refresh carried it" are different readings.
        const window = tape.slice(pointMark).map((row) => row.text);
        const census = (pattern: RegExp) => window.filter((text) => pattern.test(text)).length;
        log(
          `pick ${at} census lines=${window.length} interactionSelectIngress=${census(/actionId":"interactionSelect"/)} interactionSelectSettled=${census(/performInvocation settled.*interactionSelect/)}` +
            ` registerBrushMesh=${census(/actionId":"registerBrushMesh"/)} settledAny=${census(/performInvocation settled/)} refreshUi=${census(/refreshUi/)}`,
        );
        log(`pick ${at} selectTail=${JSON.stringify(window.filter((text) => /interactionSelect/.test(text)).slice(0, 8)).slice(0, 900)}`);
        emit({ phase: "pick-settled", at, ids: settled.value, waitedMs: settled.waitedMs, selection: state.selection, guest: state.guest, lines: window.length });
        if (settled.value.length) landed = { at, id: settled.value[0]! };
      }
    }
  }
}
log(`pick census ${JSON.stringify(hops.reduce<Record<string, number>>((totals, hop) => ({ ...totals, [hop.kind]: (totals[hop.kind] ?? 0) + 1 }), {}))} landed=${landed ? `${landed.at}:${landed.id}` : "none"}`);
//#endregion 🔖️CanvasPick

//#region 🔖️Outliner
const panelMark = mark();
await page.locator('[id="framework.panel.artifact"]').first().click({ timeout: 6000, force: true }).catch(() => {});
await page.waitForTimeout(4000);
const panel = await page
  .evaluate(() => {
    const rows = Array.from(document.querySelectorAll<HTMLElement>('[role="treeitem"], [data-slot="tree-item"]')).map((element) => ({ id: element.id, text: element.innerText.replace(/\n/g, " ").trim().slice(0, 48) }));
    const panels = Array.from(document.querySelectorAll<HTMLElement>('[data-slot="panel"]')).map((element) => ({ anchor: element.getAttribute("data-anchor"), tab: element.getAttribute("data-active-tab-id"), visible: element.getAttribute("data-panel-visible"), h: element.offsetHeight }));
    const prefixed = Array.from(document.querySelectorAll<HTMLElement>('[id*="puzzle3d-play-document"]')).map((element) => element.id).slice(0, 60);
    return { rows, panels, prefixed };
  })
  .catch(() => ({ rows: [] as { id: string; text: string }[], panels: [] as unknown[], prefixed: [] as string[] }));
const entityRow = (id: string) => {
  const leaf = id.split("/").pop() ?? "";
  return id.includes("puzzle3d-play-document") && id.includes("/") && !leaf.startsWith("puzzle3d-play-document.") && leaf !== "puzzle3d-play-document";
};
const entityRows = panel.rows.filter((row) => entityRow(row.id));
log(`outliner panels=${JSON.stringify(panel.panels)}`);
log(`outliner treeRows=${panel.rows.length} entityRows=${entityRows.length} first=${JSON.stringify(entityRows[0] ?? "none")}`);
log(`outliner ids=${JSON.stringify(panel.prefixed).slice(0, 1200)}`);
log(`outliner taps=${JSON.stringify(since(panelMark).filter((text) => /b46\.panel|refreshUi dropped|fixed UI|ui\.fixed-capacity/.test(text)).slice(-12))}`);
emit({ phase: "outliner", treeRows: panel.rows.length, entityRows: entityRows.length, ids: panel.prefixed, panels: panel.panels, taps: since(panelMark).slice(-20) });

if (entityRows[0]) {
  const rowMark = mark();
  await page.locator(`[id="${entityRows[0].id.replace(/"/g, '\\"')}"]`).first().click({ timeout: 6000, force: true }).catch(() => {});
  const settled = await settle(() => selectedIds("guest"), (ids) => ids.length > 0, guestSettleMs);
  const state = await selection();
  log(`outliner select row=${entityRows[0].id} guestIds=${JSON.stringify(settled.value)} paintedIds=${JSON.stringify(await selectedIds())} waitedMs=${settled.waitedMs}`);
  log(`outliner select selection-json=${JSON.stringify(state.selection).slice(0, 600)}`);
  log(`outliner select verdicts=${JSON.stringify(since(rowMark).filter((text) => /interaction selection lost|command ingress|performInvocation settled/.test(text)).slice(-10))}`);
  emit({ phase: "outliner-select", row: entityRows[0].id, ids: settled.value, waitedMs: settled.waitedMs, selection: state.selection });
}
//#endregion 🔖️Outliner

await page.screenshot({ path: join(OUT, `b46-${stamp}.png`), fullPage: false }).catch(() => {});
writeFileSync(mdPath, `# b46 ${stamp}\n\n## Timeline\n\n\`\`\`\n${lines.join("\n")}\n\`\`\`\n\n## Pick hops\n\n${hops.map((hop) => `- ${hop.at} → **${hop.kind}**\n${hop.tail.map((text) => `  - \`${text}\``).join("\n")}`).join("\n")}\n`);
log(`wrote ${mdPath}`);
await browser.close();
