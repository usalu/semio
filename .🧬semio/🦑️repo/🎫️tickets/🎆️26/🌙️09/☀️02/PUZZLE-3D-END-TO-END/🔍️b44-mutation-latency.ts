/** ⏱️ Wave B44 — end-to-end latency of ONE mutation on a large puzzle-3d document.
 *
 * Boots the React serve on 127.0.0.1:6013, switches the example to Nakagin (180 objects), proves a
 * world selection, then dispatches ONE mutation and timestamps every hop it can observe: the guest's
 * own `[DEBUG]` turn/continuation taps, the host's `performInvocation`/`command ingress` records, the
 * `b44.*` host taps (intake, projection, World3dHost instance apply) and the DOM's own
 * `data-instances-json`. Attribution, not a verdict — the numbers land in
 * `🗑️generated/b44-<stamp>.md` and `…ndjson`.
 *
 * Run: `bun 🔍️b44-mutation-latency.ts [--port=6013] [--mutation=delete|duplicate] [--objects=nakagin|forest] [--label=before]`
 *
 * Ticket 26/09/02/PUZZLE-3D-END-TO-END. */
import { chromium } from "playwright";
import { appendFileSync, mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const TICKET = import.meta.dir;
const OUT = join(TICKET, "🗑️generated");
mkdirSync(OUT, { recursive: true });
const label = process.argv.find((a) => a.startsWith("--label="))?.slice(8) ?? "run";
const stamp = `${new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19)}-${label}`;
const port = process.argv.find((a) => a.startsWith("--port="))?.slice(7) ?? "6013";
const mutation = process.argv.find((a) => a.startsWith("--mutation="))?.slice(11) ?? "delete";
const objects = process.argv.find((a) => a.startsWith("--objects="))?.slice(10) ?? "nakagin";
const mdPath = join(OUT, `b44-${stamp}.md`);
const ndPath = join(OUT, `b44-${stamp}.ndjson`);
writeFileSync(ndPath, "");

const t0 = Date.now();
const since = () => ((Date.now() - t0) / 1000).toFixed(2);
const lines: string[] = [];
const log = (m: string) => {
  const row = `[${since()}s] ${m}`;
  lines.push(row);
  console.log(row);
};
const emit = (record: Record<string, unknown>) => appendFileSync(ndPath, `${JSON.stringify(record)}\n`);

/** 📼️ Every console line with the millisecond it arrived, so a phase boundary is a subtraction. */
const tape: { ms: number; text: string }[] = [];
const TAPE_MAX = 20_000;
const TAP_RE = /\[DEBUG\] b44\.|performInvocation|command ingress|puzzle3d\.prologue\.sync|puzzle3d\.utility\.publish|refreshUi|intake|projection|registerBrushMesh|worker fault|panic/i;

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (msg) => {
  const text = msg.text().slice(0, 400);
  if (tape.length < TAPE_MAX) tape.push({ ms: Date.now() - t0, text });
});
page.on("pageerror", (err) => {
  if (tape.length < TAPE_MAX) tape.push({ ms: Date.now() - t0, text: `pageerror: ${String(err).slice(0, 300)}` });
});

const mark = () => tape.length;
const tapeSince = (m: number) => tape.slice(m);
const interesting = (m: number) => tapeSince(m).filter((row) => TAP_RE.test(row.text));
/** 🔢️ How many of each observable tap the window since `m` carries — the honest answer to "is the
 * instrument even wired?", which a phase table of `null`s cannot give. */
const tapCensus = (m: number) => {
  const window = tapeSince(m);
  const count = (re: RegExp) => window.filter((row) => re.test(row.text)).length;
  return {
    all: window.length,
    intake: count(/b44\.intake/),
    project: count(/b44\.project/),
    world: count(/b44\.world/),
    invocations: count(/performInvocation \{/),
    settled: count(/performInvocation settled/),
    ingress: count(/command ingress lane/),
    registerBrushMesh: count(/registerBrushMesh/),
    prologue: count(/prologue\.sync/),
  };
};

const instances = async () =>
  page
    .evaluate(() => {
      const host = document.querySelector("#puzzle3d-main-perspective");
      const el = host?.querySelector("[data-instances-json]") as HTMLElement | null;
      const raw = el?.getAttribute("data-instances-json") || "[]";
      let parsed: Array<{ id?: string; position?: number[] }> = [];
      try {
        parsed = JSON.parse(raw);
      } catch {
        parsed = [];
      }
      return { bytes: raw.length, count: parsed.length, ids: parsed.map((o) => o.id ?? "?"), hash: raw.length ? `${raw.length}:${raw.slice(0, 24)}:${raw.slice(-24)}` : "-" };
    })
    .catch(() => ({ bytes: 0, count: 0, ids: [] as string[], hash: "-" }));

const selectionOf = async () =>
  page
    .evaluate(() => {
      const out: { surface: string; ids: string[] }[] = [];
      for (const el of Array.from(document.querySelectorAll("[data-selection-json]"))) {
        const raw = (el as HTMLElement).getAttribute("data-selection-json") || "{}";
        let parsed: { ids?: string[]; activeObjectId?: string } = {};
        try {
          parsed = JSON.parse(raw);
        } catch {
          parsed = {};
        }
        out.push({ surface: (el.closest("[id]") as HTMLElement | null)?.id ?? "?", ids: [...(parsed.ids ?? []), ...(parsed.activeObjectId ? [parsed.activeObjectId] : [])] });
      }
      return out;
    })
    .catch(() => [] as { surface: string; ids: string[] }[]);

const settle = async <T>(read: () => Promise<T>, done: (v: T) => boolean, budgetMs: number) => {
  const started = Date.now();
  let value = await read();
  while (!done(value) && Date.now() - started < budgetMs) {
    await page.waitForTimeout(250);
    value = await read();
  }
  return { value, waitedMs: Date.now() - started, ok: done(value) };
};

log(`goto http://127.0.0.1:${port}/ mutation=${mutation} objects=${objects}`);
await page.goto(`http://127.0.0.1:${port}/`, { waitUntil: "domcontentloaded", timeout: 120_000 });
const booted = await settle(instances, (v) => v.count > 0, 180_000);
log(`boot instances=${booted.value.count} bytes=${booted.value.bytes} waitedMs=${booted.waitedMs} taps=${JSON.stringify(tapCensus(0))}`);
emit({ phase: "boot", ...booted.value, waitedMs: booted.waitedMs });
if (!booted.ok) {
  log("boot never published instances — aborting");
  writeFileSync(mdPath, `# b44 ${stamp}\n\n${lines.join("\n")}\n`);
  await browser.close();
  process.exit(1);
}

if (objects === "nakagin") {
  const switchMark = mark();
  const picker = page.locator('[id="playground.navbar.fixture"]');
  const native = picker.locator("select").or(page.locator('select[id="playground.navbar.fixture"]')).first();
  if (await native.count().catch(() => 0)) {
    const labels = await native.locator("option").allTextContents();
    await native.selectOption({ label: labels.find((l) => /nakagin/i.test(l)) ?? "" }).catch(() => {});
  } else {
    // 🧯️ `force: true`: the navbar select trigger sits under the window-chrome strip's own hit layer, so a
    // hit-tested Playwright click on it times out while the element is neither disabled nor covered
    // (`elementFromPoint` returns the trigger itself). Measured 2026-09-12 — a plain click never opened the
    // listbox and every "the example switch does not land" reading in this wave's first two runs was that.
    await page.keyboard.press("Escape").catch(() => {});
    await picker.first().click({ timeout: 6000, force: true }).catch(() => {});
    await page.waitForTimeout(800);
    await page.locator('[role="option"]').filter({ hasText: /nakagin/i }).first().click({ timeout: 6000, force: true }).catch(() => {});
  }
  const swapStarted = Date.now();
  let swapped = await instances();
  while (swapped.count < 100 && Date.now() - swapStarted < 180_000) {
    await page.waitForTimeout(5_000);
    swapped = await instances();
    log(`switch poll +${Date.now() - swapStarted}ms instances=${swapped.count} bytes=${swapped.bytes} taps=${JSON.stringify(tapCensus(switchMark))}`);
  }
  log(`example-switch instances=${swapped.count} bytes=${swapped.bytes} waitedMs=${Date.now() - swapStarted}`);
  emit({ phase: "example-switch", ...swapped, waitedMs: Date.now() - swapStarted, taps: tapCensus(switchMark) });
}

const census = await instances();
log(`census instances=${census.count} bytes=${census.bytes}`);

// 🎯️ Selection precondition: click the pane at a projected instance, else fall back to pane fractions.
const canvasBox = await page.locator("#puzzle3d-main-perspective canvas").last().boundingBox();
log(`perspective canvas box=${JSON.stringify(canvasBox)}`);
const canvas = page.locator("#puzzle3d-main-perspective canvas").last();
let selected: string[] = [];
for (const fraction of [
  [0.5, 0.5],
  [0.5, 0.62],
  [0.45, 0.55],
  [0.55, 0.45],
  [0.5, 0.72],
  [0.42, 0.48],
]) {
  if (!canvasBox) break;
  await canvas.click({ position: { x: canvasBox.width * fraction[0], y: canvasBox.height * fraction[1] }, timeout: 6000 }).catch(() => {});
  const got = await settle(selectionOf, (v) => v.some((s) => s.ids.length > 0), 6000);
  selected = got.value.flatMap((s) => s.ids);
  log(`select at ${fraction.join(",")} ids=${JSON.stringify(selected).slice(0, 200)} waitedMs=${got.waitedMs}`);
  if (selected.length) break;
}
if (!selected.length) {
  // 🌳️ Canvas picks resolve nothing on the 180-object document (they DO on the 1-object Concrete
  // Forest — wave B44 §7). The outliner is the second route: open the Artifact panel and click an
  // ENTITY row (not the tree root, not a group header).
  const tab = page.locator('[id="framework.panel.artifact"]');
  await tab.first().click({ timeout: 6000, force: true }).catch(() => {});
  await page.waitForTimeout(2500);
  const rows = await page
    .evaluate(() =>
      Array.from(document.querySelectorAll<HTMLElement>('[role="treeitem"], [data-slot="tree-item"]'))
        .filter((element) => element.id.includes("puzzle3d-play-document") && element.id.includes("/"))
        .map((element) => element.id)
        .filter((id) => {
          const leaf = id.split("/").pop() ?? "";
          return !leaf.startsWith("puzzle3d-play-document.") && leaf !== "puzzle3d-play-document";
        }),
    )
    .catch(() => [] as string[]);
  log(`outliner entityRows=${rows.length} first=${rows[0] ?? "none"}`);
  if (rows[0]) {
    await page.locator(`[id="${rows[0].replace(/"/g, '\\"')}"]`).first().click({ timeout: 6000, force: true }).catch(() => {});
    const viaOutliner = await settle(selectionOf, (v) => v.some((s) => s.ids.length > 0), 15_000);
    selected = viaOutliner.value.flatMap((s) => s.ids);
    log(`select via outliner ids=${JSON.stringify(selected).slice(0, 200)} waitedMs=${viaOutliner.waitedMs}`);
  }
}
emit({ phase: "selection", ids: selected });
if (!selected.length) log("NO SELECTION — the mutation will be a no-op; numbers below measure the dispatch only");

const before = await instances();
const gestureMark = mark();
const gestureAt = Date.now();
log(`gesture ${mutation} before=${before.count} bytes=${before.bytes}`);
if (mutation === "duplicate") {
  await page.keyboard.press("Meta+d").catch(() => {});
  await page.keyboard.press("Control+d").catch(() => {});
} else {
  await page.keyboard.press("Delete").catch(() => {});
}
const landed = await settle(instances, (v) => v.count !== before.count || v.hash !== before.hash, 60_000);
const landedAt = Date.now();
log(`landed ok=${landed.ok} count=${landed.value.count} bytes=${landed.value.bytes} waitedMs=${landed.waitedMs}`);
emit({ phase: "mutation", mutation, ok: landed.ok, before: before.count, after: landed.value.count, waitedMs: landed.waitedMs, beforeBytes: before.bytes, afterBytes: landed.value.bytes });

/** 🔁️ A SECOND mutation right after the first, so a one-off first-publication cost is distinguishable
 * from the steady-state per-mutation price. */
const second = await (async () => {
  await page.waitForTimeout(1_500);
  if (canvasBox) await canvas.click({ position: { x: canvasBox.width * 0.5, y: canvasBox.height * 0.5 }, timeout: 6000 }).catch(() => {});
  await settle(selectionOf, (v) => v.some((s) => s.ids.length > 0), 8_000);
  const base = await instances();
  const m = mark();
  const at = Date.now();
  await page.keyboard.press(mutation === "duplicate" ? "Meta+d" : "Delete").catch(() => {});
  const got = await settle(instances, (v) => v.count !== base.count || v.hash !== base.hash, 60_000);
  log(`second ${mutation} ok=${got.ok} before=${base.count} after=${got.value.count} waitedMs=${got.waitedMs} taps=${JSON.stringify(tapCensus(m))}`);
  emit({ phase: "mutation-2", ok: got.ok, before: base.count, after: got.value.count, waitedMs: got.waitedMs, taps: tapCensus(m) });
  return { mark: m, at, rows: interesting(m) };
})();
log(`second tape (${second.rows.length} rows)`);
for (const row of second.rows) log(`  2nd +${row.ms - (second.at - t0)}ms ${row.text.slice(0, 260)}`);

const rows = interesting(gestureMark);
log(`tape rows after gesture: ${rows.length} (of ${tapeSince(gestureMark).length} console lines)`);
for (const row of rows) log(`  +${row.ms - (gestureAt - t0)}ms ${row.text.slice(0, 260)}`);

/** 📊️ Phase boundaries, each named by the first tape row that matches it. */
const firstAfter = (re: RegExp) => rows.find((row) => re.test(row.text));
const lastAfter = (re: RegExp) => [...rows].reverse().find((row) => re.test(row.text));
const rel = (row: { ms: number } | undefined) => (row ? row.ms - (gestureAt - t0) : null);
const phases = {
  gestureToIngress: rel(firstAfter(/command ingress lane|performInvocation"/)),
  guestFirstTap: rel(firstAfter(/\[DEBUG\] puzzle3d\./)),
  ingressSettled: rel(firstAfter(/command ingress settled|performInvocation settled/)),
  lastGuestTap: rel(lastAfter(/\[DEBUG\] puzzle3d\./)),
  firstIntake: rel(firstAfter(/b44\.intake/)),
  lastIntake: rel(lastAfter(/b44\.intake/)),
  firstProjection: rel(firstAfter(/b44\.project/)),
  lastProjection: rel(lastAfter(/b44\.project/)),
  firstWorldApply: rel(firstAfter(/b44\.world/)),
  lastWorldApply: rel(lastAfter(/b44\.world/)),
  domChanged: landed.ok ? landedAt - gestureAt : null,
};
log(`phases ${JSON.stringify(phases)}`);
emit({ phase: "phases", ...phases });

/** 🧮️ Tap aggregates — total intake steps / projected nodes / republished bytes for this one mutation. */
const sum = (re: RegExp, key: string) =>
  rows
    .filter((row) => re.test(row.text))
    .reduce((total, row) => {
      const found = new RegExp(`${key}=(\\d+)`).exec(row.text);
      return total + (found ? Number(found[1]) : 0);
    }, 0);
const aggregates = {
  intakeCalls: rows.filter((r) => /b44\.intake/.test(r.text)).length,
  intakeSteps: sum(/b44\.intake/, "steps"),
  intakeMs: sum(/b44\.intake/, "ms"),
  projectionCalls: rows.filter((r) => /b44\.project/.test(r.text)).length,
  projectionNodes: sum(/b44\.project/, "nodes"),
  projectionSteps: sum(/b44\.project/, "steps"),
  projectionMs: sum(/b44\.project/, "ms"),
  worldApplies: rows.filter((r) => /b44\.world/.test(r.text)).length,
  worldBytes: sum(/b44\.world/, "bytes"),
  guestTurns: rows.filter((r) => /prologue\.sync/.test(r.text)).length,
  meshAnnounces: rows.filter((r) => /registerBrushMesh/.test(r.text)).length,
};
log(`aggregates ${JSON.stringify(aggregates)}`);
emit({ phase: "aggregates", ...aggregates });

writeFileSync(
  mdPath,
  `# b44 ${stamp} (mutation=${mutation} objects=${objects} port=${port})\n\n## timeline\n${lines.join("\n")}\n\n## phases\n\`\`\`json\n${JSON.stringify(phases, null, 2)}\n\`\`\`\n\n## aggregates\n\`\`\`json\n${JSON.stringify(aggregates, null, 2)}\n\`\`\`\n\n## full tape after the gesture\n\`\`\`\n${tapeSince(gestureMark)
    .map((row) => `+${row.ms - (gestureAt - t0)}ms ${row.text}`)
    .join("\n")
    .slice(0, 200_000)}\n\`\`\`\n\n## whole-run tape\n\`\`\`\n${tape
    .map((row) => `+${row.ms}ms ${row.text}`)
    .join("\n")
    .slice(0, 600_000)}\n\`\`\`\n`,
);
log(`wrote ${mdPath}`);
await browser.close();
