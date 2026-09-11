/** 🔬️ Headless runtime probe for the puzzle 3d React serve on 127.0.0.1:6013 — boots the shell, waits for
 * windows, drives one browser step per section of `📓️2026-09-09-user-feature-checklist.md`, and writes
 * findings + screenshots + a machine-readable verdict stream into `🗑️generated/`.
 * Ticket 26/09/02/PUZZLE-3D-END-TO-END.
 *
 * Run: `bun 🔍️browser-probe.ts [--battery] [--only=step,step] [--port=<n>] [--reload-between-groups]
 * [--example=<name>] [--settle=<seconds>]`.
 *
 * `--battery` runs EVERY registered step (smoke and fill included), ordered by blast radius:
 * `read` (nothing mutates the document) → `mutate` (reversible document edits) → `replace` (the document
 * itself is swapped: example switch, undo/redo, import). `--reload-between-groups` reboots the page
 * between those groups so a group never inherits the previous group's document.
 * `--only=a,b` runs exactly those step names in plan order and ignores every other gate.
 * Single-section flags (`--brush`, `--camera`, …) are listed in `STEP_FLAGS` below.
 *
 * Outputs: `probe-<stamp>.md` (prose timeline) and `probe-<stamp>.ndjson` (one JSON record per verdict
 * plus a final `battery PASS=n FAIL=n FAULTS=n` summary record). */
import { chromium } from "playwright";
import { appendFileSync, mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const TICKET = import.meta.dir;
const OUT = join(TICKET, "🗑️generated");
mkdirSync(OUT, { recursive: true });
const stamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19);
const battery = process.argv.includes("--battery");
const onlyArg = process.argv.find((a) => a.startsWith("--only="))?.slice(7);
const only = onlyArg ? new Set(onlyArg.split(",").map((name) => name.trim()).filter(Boolean)) : null;
const reloadBetweenGroups = process.argv.includes("--reload-between-groups");
const STEP_FLAGS = [
  "--clipboard", "--marquee", "--importexport", "--import", "--locked", "--brush", "--gumball", "--suggestions", "--undo", "--selection", "--fill", "--reserved-family", "--frame",
  "--windows", "--camera", "--projection", "--windowoptions", "--volume", "--relocate", "--engagement", "--contextmenu", "--outliner", "--catalogue", "--settings", "--keys", "--adddialog", "--locale",
];
const interact = process.argv.includes("--interact") || battery || Boolean(only) || STEP_FLAGS.some((flag) => process.argv.includes(flag));
const ndjsonPath = join(OUT, `probe-${stamp}.ndjson`);
writeFileSync(ndjsonPath, "");
/** 🧾️ Appends one machine-readable record to `probe-<stamp>.ndjson` — the join key a coordinator diffs
 * two runs on is `[section, step, verdict]`. */
const emit = (record: Record<string, unknown>) => {
  appendFileSync(ndjsonPath, `${JSON.stringify(record)}\n`);
};
const lines: string[] = [];
const log = (m: string) => {
  const row = `[${((Date.now() - t0) / 1000).toFixed(1)}s] ${m}`;
  lines.push(row);
  console.log(row);
};
const t0 = Date.now();

const consoleBuf: string[] = [];
const faults: string[] = [];
const FAULT_RE =
  /intake-budget-exhausted|fixed-capacity|section-root-mismatch|native-owner-required|terminal-fault|unreachable|shard .* (lost|terminated)|did not publish|missing field|malformed|admission failed|worker fault|\[semio-plugin panic\]|Credits \{|NodeCapacity|SemioFaultError/i;
/** 💥️ The subset of {@link FAULT_RE} that means the runtime itself broke — a guest trap, a dead worker, a
 * plugin panic or a typed fault envelope. Everything else FAULT_RE matches (capacity notices, "did not
 * publish", "malformed", a repeat of an earlier storm) is COLLATERAL: real, worth counting, but never
 * independent evidence that the step under test failed. */
const HARD_FAULT_RE = /worker fault|\bunreachable\b|\[semio-plugin panic\]|panicked at|SemioFaultError|terminal-fault|admission failed|shard .* (lost|terminated)|native-owner-required/i;
const hardFaults: string[] = [];
const collateralFaults: string[] = [];
const faultKeys = new Set<string>();
/** 💥️ Records one console line that matched {@link FAULT_RE}, classified and deduplicated by its first 60
 * characters so "one fault repeated 200×" stays distinguishable from "200 distinct faults". */
const noteFault = (text: string) => {
  if (faults.length < 200) faults.push(text);
  faultKeys.add(text.slice(0, 60));
  const bucket = HARD_FAULT_RE.test(text) ? hardFaults : collateralFaults;
  if (bucket.length < 200) bucket.push(text);
};
/** 🩹️ Transient shell banners that appear under load and say nothing about the feature under test — the
 * probe used to filter these only inside `locked-refusal`; every notice consumer now shares this. */
const COLLATERAL_NOTICE_RE = /agent disconnected|remote: detached|reconnect/i;
const isCollateralNotice = (text: string) => COLLATERAL_NOTICE_RE.test(text);

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, acceptDownloads: true });
const port = process.argv.find((a) => a.startsWith("--port="))?.slice(7) ?? "6013";
await page.context().grantPermissions(["clipboard-read", "clipboard-write"], { origin: `http://127.0.0.1:${port}` });
page.on("console", (msg) => {
  const text = msg.text().slice(0, 400);
  if (consoleBuf.length >= 4000) consoleBuf.shift();
  consoleBuf.push(`${msg.type()}: ${text}`);
  if (FAULT_RE.test(text)) noteFault(text);
});
page.on("pageerror", (err) => noteFault(`pageerror: ${String(err).slice(0, 400)}`));

log("navigating");
await page.goto(`http://127.0.0.1:${port}/?plugin=puzzle3d`, { waitUntil: "domcontentloaded", timeout: 60000 });

const snapshot = async () =>
  page.evaluate(() => {
    const q = (sel: string) => Array.from(document.querySelectorAll(sel));
    return {
      windows: q('[data-slot="window"]').map((w) => ({
        id: w.id || w.getAttribute("data-key") || "?",
        w: (w as HTMLElement).offsetWidth,
        h: (w as HTMLElement).offsetHeight,
      })),
      canvases: q("canvas").length,
      tabs: q('[data-slot="panel-tab-button"]').map((b) => b.id).slice(0, 40),
      toggles: q('[data-slot="toggle-group-item"]').map((b) => `${b.id}=${b.getAttribute("aria-pressed")}`).slice(0, 40),
      treeItems: q('[data-slot="tree-item"], [role="treeitem"]').length,
      dialogs: q('[role="dialog"]').map((d) => (d as HTMLElement).innerText.slice(0, 80)),
      body: document.body ? document.body.innerText.slice(0, 500) : "",
    };
  });

/** 🚀️ Polls until two windows with two canvases exist, dismissing the intro dialog and the welcome tour on
 * the way. Reused verbatim by `--reload-between-groups` so a regrouped run boots exactly like a cold one. */
const waitForBoot = async (label: string, polls = 60) => {
  let ready = false;
  for (let i = 0; i < polls; i++) {
    await page.waitForTimeout(3000);
    const s = await snapshot();
    if (s.dialogs.length && i % 3 === 0) {
      const skip = page.locator('[role="dialog"] button', { hasText: /skip/i }).first();
      if (await skip.count()) {
        await skip.click({ timeout: 2000 }).catch(() => {});
        log(`${label} skipped intro dialog`);
      }
    }
    if (s.windows.length >= 2 && s.canvases >= 2) {
      log(`${label} booted: windows=${JSON.stringify(s.windows)} canvases=${s.canvases} treeItems=${s.treeItems}`);
      ready = true;
      break;
    }
    if (i % 5 === 4) log(`${label} waiting… windows=${s.windows.length} canvases=${s.canvases} faults=${faults.length}`);
  }
  if (ready) {
    const skip = page.locator("button", { hasText: /^\s*(x\s*)?skip\s*$/i }).first();
    if (await skip.count()) {
      await skip.click({ timeout: 3000 }).catch(() => {});
      await page.waitForTimeout(1500);
      log(`${label} dismissed welcome tour`);
    }
  }
  return ready;
};

const booted = await waitForBoot("boot");
await page.screenshot({ path: join(OUT, `probe-${stamp}-boot.png`) }).catch(() => {});

if (booted && interact) {
  const step = async (name: string, fn: () => Promise<void>) => {
    const before = faults.length;
    try {
      await fn();
      await page.waitForTimeout(4000);
      const s = await snapshot();
      log(`step ${name}: ok windows=${s.windows.length} canvases=${s.canvases} treeItems=${s.treeItems} newFaults=${faults.slice(before).join(" | ").slice(0, 300) || "none"}`);
    } catch (e) {
      log(`step ${name}: FAILED ${String(e).slice(0, 200)}`);
    }
    await page.screenshot({ path: join(OUT, `probe-${stamp}-${name}.png`) }).catch(() => {});
  };
  const family = process.argv.includes("--reserved-family") || battery;
  const exampleArg = process.argv.find((a) => a.startsWith("--example="))?.slice(10);
  const wantUndo = process.argv.includes("--undo") || family;
  const wantExample = !!exampleArg || wantUndo;
  /** 🧱️ Blast-radius buckets, executed in this order: `read` touches no document state, `mutate` makes
   * reversible document edits, `replace` swaps the document itself (example switch, undo/redo, import). */
  type StepGroup = "read" | "mutate" | "replace";
  const GROUP_ORDER: readonly StepGroup[] = ["read", "mutate", "replace"];
  type PlanEntry = { readonly name: string; readonly section: string; readonly group: StepGroup; readonly run: () => Promise<void> };
  const plan: PlanEntry[] = [];
  /** 🗂️ Registers one step. Under `--only=` the named set is the ONLY gate; otherwise `gate` (the step's own
   * flag, OR'd with `--battery`) decides. Registration order is preserved inside each group. */
  const add = (name: string, section: string, group: StepGroup, gate: boolean, run: () => Promise<void>) => {
    if (only ? only.has(name) : gate) plan.push({ name, section, group, run });
  };
  const verdicts: string[] = [];
  let passCount = 0;
  let failCount = 0;
  let currentStep = "boot";
  let currentSection = "§0";
  const verdict = (name: string, ok: boolean, note: string, expectTag?: 41 | 42) => {
    const line = ok ? `verdict ${name} PASS` : `verdict ${name} FAIL${expectTag ? ` [expect-${expectTag}]` : ""} ${note}`;
    verdicts.push(line);
    if (ok) passCount += 1;
    else failCount += 1;
    emit({
      t: Number(((Date.now() - t0) / 1000).toFixed(1)),
      ts: new Date().toISOString(),
      section: currentSection,
      step: currentStep,
      verdict: name,
      ok,
      status: ok ? "PASS" : "FAIL",
      note,
      faults: faults.length,
      hardFaults: hardFaults.length,
      collateralFaults: collateralFaults.length,
      distinctFaults: faultKeys.size,
    });
    log(line);
  };
  verdict("boot", booted, "windows ready");
  const dismissChrome = async () => {
    await page.keyboard.press("Escape").catch(() => {});
    const collapse = page.locator("button", { hasText: /collapse/i }).first();
    if (await collapse.count()) await collapse.click({ timeout: 2000 }).catch(() => {});
    await page.keyboard.press("Escape").catch(() => {});
  };
  add("activate-perspective", "§1", "read", true, async () => {
    const c = page.locator("canvas").last();
    await c.click({ position: { x: 200, y: 200 }, timeout: 5000 });
  });
  add("pick-object", "§6", "mutate", true, async () => {
    const c = page.locator("canvas").last();
    await c.click({ position: { x: 470, y: 420 }, timeout: 5000 });
  });
  add("context-menu", "§15", "mutate", true, async () => {
    const c = page.locator("canvas").last();
    await c.click({ position: { x: 470, y: 420 }, button: "right", timeout: 5000 });
    await page.waitForTimeout(600);
    log(`context-menu raw: ${JSON.stringify(await chromeState()).slice(0, 600)}`);
    await dismissChrome();
  });
  add("example-switch", "§5", "replace", wantExample || battery, async () => {
    const sel = page.locator("select").first();
    if (await sel.count()) await sel.selectOption({ index: 1 });
    else {
      const combo = page.locator('[role="combobox"]').first();
      await combo.click({ timeout: 3000 });
      const options = page.locator('[role="option"]');
      const target = exampleArg
        ? options.filter({ hasText: new RegExp(exampleArg, "i") }).first()
        : wantUndo
          ? options.filter({ hasText: /nakagin/i }).first()
          : options.nth(1);
      await target.click({ timeout: 3000 });
    }
    const settleArg = process.argv.find((a) => a.startsWith("--settle="))?.slice(9);
    await page.waitForTimeout(settleArg ? Number(settleArg) * 1000 : 20000);
    const readExample = async () =>
      page.evaluate(() => ((document.querySelector('[role="combobox"]') as HTMLElement | null)?.innerText || "").replace(/\n/g, " ").slice(0, 80));
    let afterExample = await readExample();
    log(`example after switch: ${afterExample}`);
    if (wantUndo) verdict("example-switch", /nakagin/i.test(afterExample), `example=${afterExample}`);
    if (wantUndo && !/nakagin/i.test(afterExample)) {
      const combo = page.locator('[role="combobox"]').first();
      await combo.click({ timeout: 3000 }).catch(() => {});
      const nakagin = page.locator('[role="option"]').filter({ hasText: /nakagin/i }).first();
      if (await nakagin.count()) await nakagin.click({ timeout: 3000 }).catch(() => {});
      await page.waitForTimeout(8000);
      afterExample = await readExample();
      log(`example after nakagin retry: ${afterExample}`);
    }
    verdict("example-switch-instances", (await dumpInstances()).count > 0, `instances=${JSON.stringify(await dumpInstances())} example=${afterExample}`);
  });
  {
    const fillState = async () =>
      page.evaluate(() => {
        const q = (sel: string) => Array.from(document.querySelectorAll(sel));
        const panel = document.querySelector('[data-slot="panel"][data-anchor="bottom-middle"]') as HTMLElement | null;
        return {
          toggles: q('[data-slot="toggle-group-item"]').map((b) => `${b.id}=${b.getAttribute("aria-pressed")}`),
          sliders: q('input[type="range"], [role="slider"]').map(
            (s) => `${s.id || s.getAttribute("aria-label") || "?"}=${(s as HTMLInputElement).value || s.getAttribute("aria-valuenow")}/${(s as HTMLInputElement).max || s.getAttribute("aria-valuemax")}`,
          ),
          measures: q('[data-slot="measure"], [class*="measure"]').map((m) => (m as HTMLElement).innerText.replace(/\n/g, " ").slice(0, 60)).slice(0, 10),
          treeItems: q('[data-slot="tree-item"], [role="treeitem"]').length,
          ranges: q('input[type="range"]').length,
          params: q('[data-slot="params"], [data-slot="panel-body-stack"] [data-slot="tree"]').length,
          panel: panel ? { visible: panel.getAttribute("data-panel-visible"), tab: panel.getAttribute("data-active-tab-id"), w: panel.offsetWidth, h: panel.offsetHeight, text: panel.innerText.replace(/\n/g, " | ").slice(0, 240) } : null,
        };
      });
    const fillConsoleCensus = () => {
      const joined = consoleBuf.join("\n");
      const ticks = consoleBuf.filter((l) => /fill_build_tick|fillBuildTick/.test(l)).length;
      const arm = consoleBuf.filter((l) => /\[DEBUG\] setActiveTool|\[DEBUG\] reconcileToolTab|\[DEBUG\] fillBuildTick gate/.test(l));
      const jobs = consoleBuf.filter((l) => /\[DEBUG\] routeHostEffects spawn-job|\[DEBUG\] driveSpawnedJob|\[DEBUG\] turn effects/.test(l));
      const spawnChanged = (joined.match(/changed\s*=\s*[\s\S]{0,40}true/g) ?? []).length;
      const faults = consoleBuf.filter((l) => /FAULT_RE/.test(l)).length;
      return { ticks, jobs: jobs.length, spawnChanged, faults, jobSample: jobs.slice(-12), armSample: arm.slice(-4) };
    };
    const fillCountFromState = (s: { sliders: string[]; panel: { text: string } | null; measures: string[] }): number => {
      const panel = s.panel?.text ?? "";
      const row = /Count\s*\|\s*(\d+)/.exec(panel);
      if (row) return Number(row[1]);
      for (const sl of s.sliders) {
        const m = /=(\d+)\/(\d+)/.exec(sl);
        if (m && Number(m[1]) > 0) return Number(m[1]);
      }
      return 0;
    };
    add("tool-category", "§12", "mutate", process.argv.includes("--fill") || battery, async () => {
      const buttons = await page.locator('button[id="framework.category.tool"]').count();
      log(`tool-category buttons=${buttons}`);
      await page.locator('button[id="framework.category.tool"]').first().click({ timeout: 5000 });
      await page.waitForTimeout(1500);
      log(`fill state after category: ${JSON.stringify(await fillState()).slice(0, 500)}`);
    });
    add("fill-tab", "§12", "mutate", process.argv.includes("--fill") || battery, async () => {
      const fillButtons = await page.locator('button[id="tool.fill"]').count();
      log(`fill-tab buttons=${fillButtons}`);
      await page.locator('button[id="tool.fill"]').first().click({ timeout: 5000 });
      await page.waitForTimeout(2000);
      const s1 = await fillState();
      if (!s1.toggles.some((t) => t === "tool.fill=true")) {
        const toggle = page.locator('[data-slot="toggle-group-item"][id="tool.fill"]').first();
        if (await toggle.count()) await toggle.click({ timeout: 3000 }).catch(() => {});
        else await page.locator('button[id="tool.fill"]').first().click({ timeout: 3000 }).catch(() => {});
      }
      log(`fill state after activate: ${JSON.stringify(await fillState()).slice(0, 500)}`);
      log(`fill census after activate: ${JSON.stringify(fillConsoleCensus())}`);
      const armDeadline = Date.now() + 15000;
      while (Date.now() < armDeadline) {
        const armed = await fillState();
        if ((armed.panel?.text ?? "").includes("Cancel fill") || armed.treeItems >= 4) break;
        await page.locator('button[id="tool.fill"]').first().click({ timeout: 2000 }).catch(() => {});
        await page.waitForTimeout(1500);
      }
      log(`fill state after arm-wait: ${JSON.stringify(await fillState()).slice(0, 500)}`);
      log(`fill census after arm-wait: ${JSON.stringify(fillConsoleCensus())}`);
    });
    add("fill-abort-engagement", "§12", "mutate", process.argv.includes("--fill") || battery, async () => {
      const before = await fillState();
      const beforeArm = fillConsoleCensus();
      log(`fill abort before: ${JSON.stringify(before).slice(0, 500)}`);
      log(`fill abort census before: ${JSON.stringify(beforeArm)}`);
      await page.keyboard.press("Escape");
      await page.waitForTimeout(800);
      await page.keyboard.press("Escape");
      await page.waitForTimeout(1200);
      const after = await fillState();
      const afterArm = fillConsoleCensus();
      const emptyBounce = consoleBuf.filter((l) => /setActiveTool/.test(l) && (/requested:\s*""/.test(l) || /requested: ""/.test(l) || /toolId:\s*""/.test(l) || /next:\s*null/.test(l) || /next: null/.test(l)));
      const sat = consoleBuf.filter((l) => /setActiveTool/.test(l)).slice(-12);
      const stillFill = (after.panel?.text ?? "").includes("Fill") || after.toggles.some((row) => row === "tool.fill=true") || (after.panel?.tab ?? "") === "tool.fill";
      const cancelStill = (after.panel?.text ?? "").includes("Cancel fill");
      log(`fill abort emptyBounce=${emptyBounce.length} stillFill=${stillFill} cancelStill=${cancelStill}`);
      log(`fill abort setActiveTool tail: ${JSON.stringify(sat).slice(0, 800)}`);
      log(`fill abort after: ${JSON.stringify(after).slice(0, 500)}`);
      log(`fill abort census after: ${JSON.stringify(afterArm)}`);
    });
    add("fill-wait-ready", "§12", "mutate", process.argv.includes("--fill") || battery, async () => {
      const deadline = Date.now() + 60000;
      let last = await fillState();
      let count = fillCountFromState(last);
      while (count <= 0 && Date.now() < deadline) {
        await page.waitForTimeout(5000);
        last = await fillState();
        count = fillCountFromState(last);
        log(`fill wait poll count=${count} census=${JSON.stringify(fillConsoleCensus())} state=${JSON.stringify(last).slice(0, 500)}`);
      }
      log(`fill census after wait: ${JSON.stringify(fillConsoleCensus())}`);
      log(`fill state after wait: ${JSON.stringify(last).slice(0, 600)}`);
      log(`fill count after wait: ${count}`);
    });
    add("fill-apply-max", "§12", "mutate", process.argv.includes("--fill") || battery, async () => {
      const slider = page.locator('[role="slider"]').first();
      if (await slider.count()) {
        await slider.focus();
        await page.keyboard.press("End");
        await page.waitForTimeout(8000);
        const after = await fillState();
        log(`fill state after End: ${JSON.stringify(after).slice(0, 600)}`);
        log(`fill count after End: ${fillCountFromState(after)}`);
        log(`fill census after End: ${JSON.stringify(fillConsoleCensus())}`);
      } else log("no slider found for fill-apply");
    });
  }
  const chromeState = async () =>
    page.evaluate(() => {
      const q = (sel: string) => Array.from(document.querySelectorAll(sel));
      const combo = document.querySelector('[role="combobox"]') as HTMLElement | null;
      const notices = q('[role="status"], [role="alert"], [data-slot="notice"], [data-slot="toast"]').map((n) => (n as HTMLElement).innerText.replace(/\n/g, " ").slice(0, 160)).slice(0, 12);
      const menus = q('[role="menu"], [data-slot="context-menu"], [data-slot="dropdown-menu"]').map((m) => (m as HTMLElement).innerText.replace(/\n/g, " | ").slice(0, 200));
      const fileish = q("button, [role='menuitem']")
        .filter((b) => /import|export|copy|cut|paste|brush|lock|suggest/i.test((b as HTMLElement).innerText + " " + b.id))
        .map((b) => `${b.id || "?"}=${(b as HTMLElement).innerText.replace(/\n/g, " ").slice(0, 40)}`)
        .slice(0, 30);
      return {
        example: (combo?.innerText || "").replace(/\n/g, " ").slice(0, 80),
        treeItems: q('[data-slot="tree-item"], [role="treeitem"]').length,
        notices,
        menus,
        fileish,
        utilityIds: q("button, [data-slot='toggle-group-item']").filter((b) => /brush|transform|relocate|volume|utilit/i.test((b.id || "") + (b as HTMLElement).innerText)).map((b) => `${b.id || "?"}=${(b as HTMLElement).innerText.replace(/\n/g, " ").slice(0, 40)}`).slice(0, 24),
        suggestion: (document.body?.innerText || "").match(/suggest[^|]{0,80}/gi)?.slice(0, 8) ?? [],
        body: (document.body?.innerText || "").replace(/\n/g, " | ").slice(0, 400),
      };
    });
  const selectionState = async () =>
    page.evaluate(() => {
      const q = (sel: string) => Array.from(document.querySelectorAll(sel));
      const inspection = document.querySelector("#framework.panel.inspection, [id*='inspection']") as HTMLElement | null;
      const selected = q('[aria-selected="true"], [data-selected="true"], [data-state="selected"]').map((e) => `${e.id || e.tagName}=${(e as HTMLElement).innerText.replace(/\n/g, " ").trim().slice(0, 80)}`).slice(0, 20);
      const census = (document.body?.innerText || "").match(/\d+\s+Objects[^|]{0,40}/g) ?? [];
      const inspectorIds = q("[id*='inspector'], [id*='Inspection'], [id*='inspection']").map((e) => e.id).filter(Boolean).slice(0, 30);
      const locked = q("[id*='locked'], [id*='lock']").map((e) => `${e.id}=${(e as HTMLElement).innerText.replace(/\n/g, " ").trim().slice(0, 40)}`).slice(0, 20);
      const hosts = q("[data-status-json], [data-instances-json]").map((e) => {
        const el = e as HTMLElement;
        const status = (el.getAttribute("data-status-json") || "").slice(0, 400);
        return { surface: el.getAttribute("data-surface-id"), status };
      });
      return {
        census,
        selected,
        inspectorIds,
        locked,
        inspectionText: (inspection?.innerText || "").replace(/\n/g, " | ").slice(0, 500),
        treeItems: q('[data-slot="tree-item"], [role="treeitem"]').length,
        hosts,
      };
    });
  const leftoverLaneCensus = () => {
    const firstTurns = consoleBuf.filter((l) => /performInvocation settled/.test(l) && /interactionSelect/.test(l));
    const leftovers = consoleBuf.filter((l) => /leftover InteractionView/.test(l));
    const reserved = consoleBuf.filter((l) => /job done kind=framework.reserved|framework.reserved/.test(l) && /done|job-completed|steps=/.test(l));
    return { firstTurnSelect: firstTurns.slice(-8), leftoverTaps: leftovers.slice(-8), leftoverCount: leftovers.length, reservedDone: reserved.slice(-8) };
  };
  const waitLeftoverInteractionView = async (label: string) => {
    const before = leftoverLaneCensus().leftoverCount;
    let last = leftoverLaneCensus();
    for (let i = 0; i < 20; i++) {
      last = leftoverLaneCensus();
      if (last.leftoverCount > before) break;
      await page.waitForTimeout(400);
    }
    log(`${label} leftover-lane before=${before} after=${last.leftoverCount} ${JSON.stringify(last).slice(0, 2000)}`);
    return last;
  };
  const inspectionPopulate = async () =>
    page.evaluate(() => {
      const q = (sel: string) => Array.from(document.querySelectorAll(sel));
      const fields = q("[id*='puzzle3d-play-inspector']").map((e) => `${e.id}=${(e as HTMLElement).innerText.replace(/\n/g, " ").trim().slice(0, 80)}`).slice(0, 48);
      const bySuffix = (end: string) => {
        const el = q("[id]").find((node) => node.id === end || node.id.endsWith(end) || node.id.endsWith(`/${end}`));
        return el ? (el as HTMLElement).innerText.replace(/\n/g, " ").trim().slice(0, 80) : null;
      };
      const objectId = bySuffix("puzzle3d-play-inspector.object.id");
      const objectOrigin = bySuffix("puzzle3d-play-inspector.object.origin");
      const emptySummary = bySuffix("puzzle3d-play-inspector.empty");
      const objectLocked = bySuffix("puzzle3d-play-inspector.object.locked");
      return {
        fields,
        objectId,
        objectOrigin,
        objectKind: bySuffix("puzzle3d-play-inspector.object.kind"),
        objectLocked,
        emptySummary,
        populated: Boolean(objectId || objectOrigin),
        lockChromePresent: Boolean(objectLocked),
        emptyPresent: Boolean(emptySummary),
      };
    });
  const waitInspectionPopulated = async (label: string) => {
    let last = await inspectionPopulate();
    for (let i = 0; i < 12; i++) {
      last = await inspectionPopulate();
      if (last.populated) break;
      await page.waitForTimeout(350);
    }
    log(`${label} inspection-wait populated=${last.populated} empty=${Boolean(last.emptySummary)} id=${last.objectId} lock=${last.lockChromePresent} fields=${JSON.stringify(last.fields).slice(0, 500)}`);
    return last;
  };
  const clickForestTable = async () => {
    await page.evaluate(() => {
      const w = window as unknown as { __abDebug?: string[]; __abPtr?: EventListener };
      w.__abDebug = [];
      const tap = (ev: PointerEvent) => {
        const t = ev.target as HTMLElement | null;
        const line = `[DEBUG] native pointerdown ${t?.tagName} ${t?.id ?? ""} ${ev.clientX} ${ev.clientY}`;
        w.__abDebug!.push(line);
        console.log(line);
      };
      document.addEventListener("pointerdown", tap, true);
      w.__abPtr = tap as unknown as EventListener;
      const wrap = console.log.bind(console);
      console.log = (...args: unknown[]) => {
        const s = args.map(String).join(" ");
        if (s.includes("[DEBUG]")) w.__abDebug!.push(s);
        wrap(...args);
      };
    });
    const canvases = page.locator("canvas");
    const n = await canvases.count();
    const top = canvases.first();
    const persp = canvases.nth(Math.max(0, n - 1));
    const box = await persp.boundingBox();
    const topBox = await top.boundingBox();
    if (!box || !topBox) throw new Error("no canvas");
    const spots = [
      { x: 529, y: 415 },
      { x: 541, y: 411 },
    ];
    for (const s of spots) {
      await persp.hover({ position: s, timeout: 4000, force: true }).catch(() => {});
      await page.waitForTimeout(150);
      await persp.click({ position: s, timeout: 5000, force: true }).catch(() => {});
      await page.waitForTimeout(800);
    }
    const actions = page.locator("button", { hasText: /^actions$/i }).last();
    if (await actions.count()) await actions.click({ timeout: 3000 }).catch(() => {});
    const inspection = page.locator("button", { hasText: /^inspection$/i }).last();
    if (await inspection.count()) await inspection.click({ timeout: 3000 }).catch(() => {});
    return { box, topBox, spots };
  };
  const unfoldPerspectiveUtilities = async () => {
    const persp = page.locator("canvas").last();
    await persp.click({ position: { x: 80, y: 80 }, timeout: 4000 }).catch(() => {});
    const byId = page.locator('[id="framework.window.puzzle3dMainPerspective.utilityBar.unfold"]').first();
    const chip = page.locator("button").filter({ hasText: /^utilities$/i }).last();
    log(`utilities chips=${await page.locator("button").filter({ hasText: /utilities/i }).count()} unfoldId=${await byId.count()}`);
    if (await byId.count()) await byId.click({ timeout: 4000 }).catch(() => {});
    else if (await chip.count()) await chip.click({ timeout: 4000 }).catch(() => {});
    await page.waitForTimeout(800);
    const ids = await page.evaluate(() =>
      Array.from(document.querySelectorAll("button, [data-slot='toggle-group-item']"))
        .map((b) => `${(b as HTMLElement).id || "?"}=${(b as HTMLElement).innerText.replace(/\n/g, " ").slice(0, 40)}`)
        .filter((s) => /brush|transform|relocate|volume|utilit/i.test(s))
        .slice(0, 30),
    );
    log(`utility ids=${JSON.stringify(ids)}`);
    return ids;
  };
  const armBrushUtility = async () => {
    let last = await dumpBrushPreview();
    if (last.utility === "brush") {
      log(`arm-brush already utility=brush hover=${last.hover ?? "null"}`);
      return last;
    }
    await unfoldPerspectiveUtilities();
    const metrics = await page.evaluate(() => {
      const el = document.getElementById("brush") as HTMLButtonElement | null;
      if (!el) return { found: false };
      const r = el.getBoundingClientRect();
      const cs = getComputedStyle(el);
      return {
        found: true,
        pressed: el.getAttribute("aria-pressed") ?? el.getAttribute("data-state"),
        disabled: el.disabled,
        pe: cs.pointerEvents,
        w: Math.round(r.width),
        h: Math.round(r.height),
        x: Math.round(r.x),
        y: Math.round(r.y),
      };
    });
    log(`arm-brush #brush metrics=${JSON.stringify(metrics)}`);
    if (metrics.pressed !== "true" && metrics.pressed !== "on") {
      // ToggleGroup kind=multiple: a second click deactivates. Playwright+evaluate together
      // is two activations (measured Nakagin #41: seq 222 then 223, guest stayed select).
      const brush = page.locator("#brush").last();
      await brush.click({ force: true, timeout: 4000 });
      log("arm-brush playwright force click once");
    } else {
      log("arm-brush skipped click, already pressed");
    }
    const afterClick = await page.evaluate(() => {
      const el = document.getElementById("brush") as HTMLButtonElement | null;
      return { pressed: el?.getAttribute("aria-pressed") ?? el?.getAttribute("data-state") ?? null };
    });
    log(`arm-brush post-click ${JSON.stringify(afterClick)}`);
    const polls = 50;
    for (let i = 0; i < polls && last.utility !== "brush"; i++) {
      await page.waitForTimeout(400);
      last = await dumpBrushPreview();
      if (i % 5 === 0 || last.utility === "brush") log(`arm-brush poll ${i} utility=${last.utility ?? "null"} hover=${last.hover ?? "null"}`);
    }
    log(`arm-brush result utility=${last.utility ?? "null"} hover=${last.hover ?? "null"}`);
    return last;
  };
  const dumpForestCensus = async () => page.evaluate(() => {
    const host = document.querySelector("#puzzle3d-main-perspective");
    const world = host?.querySelector("[data-instances-json]") as HTMLElement | null;
    let instanceCount = 0;
    let vortexCount = 0;
    try {
      instanceCount = JSON.parse(world?.getAttribute("data-instances-json") || "[]").length;
    } catch {
      instanceCount = -1;
    }
    try {
      vortexCount = JSON.parse((host?.querySelector("[data-vortices-json]") as HTMLElement | null)?.getAttribute("data-vortices-json") || "[]").length;
    } catch {
      vortexCount = -1;
    }
    return { instanceCount, vortexCount };
  });
  const dumpVorticesJson = async () =>
    page.evaluate(() => {
      const host = document.querySelector("#puzzle3d-main-perspective");
      const el = (host?.querySelector("[data-vortices-json]") as HTMLElement | null) ?? (host as HTMLElement | null);
      const raw = el?.getAttribute("data-vortices-json") || "[]";
      let parsed: Array<{ id?: string; fullId?: string }> = [];
      try {
        parsed = JSON.parse(raw);
      } catch {
        parsed = [];
      }
      return { count: parsed.length, ids: parsed.slice(0, 12).map((v) => v.id ?? v.fullId ?? "?"), rawLen: raw.length };
    });
  const dumpBrushPreview = async () =>
    page.evaluate(() => {
      const nodes = Array.from(document.querySelectorAll("[data-brush-preview-json]")) as HTMLElement[];
      const host = (document.querySelector("#puzzle3d-main-perspective [data-brush-preview-json]") as HTMLElement | null)
        ?? nodes.sort((a, b) => (b.getAttribute("data-brush-preview-json") || "").length - (a.getAttribute("data-brush-preview-json") || "").length)[0]
        ?? (document.querySelector("#puzzle3d-main-perspective") as HTMLElement | null);
      const el = host;
      const raw = el?.getAttribute("data-brush-preview-json") || "";
      const interactionRaw = (host?.querySelector("[data-interaction-json]") as HTMLElement | null)?.getAttribute("data-interaction-json")
        || (host as HTMLElement | null)?.getAttribute("data-interaction-json")
        || "";
      let parsed: { targetVortexFullId?: string; objectKindId?: string } | null = null;
      let interaction: { activeUtility?: string; hoveredVortexFullId?: string; suggestionMenu?: { open?: boolean } } | null = null;
      try {
        parsed = raw ? JSON.parse(raw) : null;
      } catch {
        parsed = null;
      }
      try {
        interaction = interactionRaw ? JSON.parse(interactionRaw) : null;
      } catch {
        interaction = null;
      }
      const laneKids = Array.from(document.querySelectorAll("[data-ui-node-key]")).map((n) => {
        const key = n.getAttribute("data-ui-node-key") || "";
        if (!key.includes("brushPreview") && !key.includes("world3d.brush") && !key.includes("suggestionMenu")) return null;
        const txt = (n.textContent || "").trim();
        return { key, textLen: txt.length, textHead: txt.slice(0, 80) };
      }).filter(Boolean);
      const hasAttr = Boolean(el?.hasAttribute("data-brush-preview-json") || host?.hasAttribute("data-brush-preview-json"));
      return {
        hasAttr,
        rawLen: raw.length,
        laneKids,
        hosts: nodes.map((n) => ({ id: n.id, rawLen: (n.getAttribute("data-brush-preview-json") || "").length })),
        target: parsed?.targetVortexFullId ?? null,
        kind: parsed?.objectKindId ?? null,
        utility: interaction?.activeUtility ?? null,
        hover: interaction?.hoveredVortexFullId ?? null,
        hoverKeys: interaction ? Object.keys(interaction) : [],
        rawHoverField: interaction && "hover" in interaction ? (interaction as { hover?: unknown }).hover ?? null : null,
        menuOpen: interaction?.suggestionMenu?.open ?? false,
      };
    });
  const dumpSuggestionMenu = async () =>
    page.evaluate(() => {
      const host = document.querySelector("#puzzle3d-main-perspective");
      const el = (host?.querySelector("[data-suggestion-menu-json]") as HTMLElement | null) ?? (host as HTMLElement | null);
      const raw = el?.getAttribute("data-suggestion-menu-json") || "";
      let parsed: { open?: boolean; vortexFullId?: string; candidates?: unknown[] } | null = null;
      try {
        parsed = raw ? JSON.parse(raw) : null;
      } catch {
        parsed = null;
      }
      const menus = Array.from(document.querySelectorAll('[role="menu"], [data-slot="context-menu"]')).map((m) => (m as HTMLElement).innerText.replace(/\n/g, " | ").slice(0, 240));
      const items = Array.from(document.querySelectorAll('[role="menuitem"]')).map((b) => (b as HTMLElement).innerText.replace(/\n/g, " ").trim().slice(0, 80)).slice(0, 16);
      return { rawLen: raw.length, open: parsed?.open ?? false, vortex: parsed?.vortexFullId ?? null, candidates: parsed?.candidates?.length ?? 0, menus, items };
    });
  const dumpInstances = async () =>
    page.evaluate(() => {
      const host = document.querySelector("#puzzle3d-main-perspective");
      const el = host?.querySelector("[data-instances-json]") as HTMLElement | null;
      const raw = el?.getAttribute("data-instances-json") || "[]";
      let parsed: Array<{ id?: string }> = [];
      try {
        parsed = JSON.parse(raw);
      } catch {
        parsed = [];
      }
      return { count: parsed.length, ids: parsed.slice(0, 16).map((o) => o.id ?? "?") };
    });
  const waitBrushVortices = async () => {
    let last = { count: 0, ids: [] as string[], rawLen: 0 };
    for (let i = 0; i < 16; i++) {
      last = await dumpVorticesJson();
      log(`brush vortices poll ${i} count=${last.count} ids=${JSON.stringify(last.ids)} rawLen=${last.rawLen}`);
      if (last.count > 0) break;
      await page.waitForTimeout(400);
    }
    return last;
  };
  const waitForestCensus = async () => {
    let last = { instanceCount: 0, vortexCount: 0 };
    for (let i = 0; i < 20; i++) {
      last = await dumpForestCensus();
      log(`frame census poll ${i} instanceCount=${last.instanceCount} vortexCount=${last.vortexCount}`);
      if (last.instanceCount >= 1 && i >= 3) break;
      await page.waitForTimeout(400);
    }
    return last;
  };
  const frameForestTableAfterCensus = async () => {
    await waitForestCensus();
    await frameForestTable();
    const settled = await waitForestCensus();
    const second = await frameForestTable();
    log(`reframe after census settled instanceCount=${settled.instanceCount} vortexCount=${settled.vortexCount}`);
    return second;
  };
  const frameForestTable = async () => {
    const c = page.locator("canvas").last();
    await c.click({ position: { x: 80, y: 80 }, timeout: 4000 }).catch(() => {});
    const persp = page.locator("#puzzle3d-main-perspective");
    const dump = await page.evaluate(() => {
      const host = document.querySelector("#puzzle3d-main-perspective");
      const btn = host?.querySelector('[id^="world3d-frame-instances-"]') as HTMLButtonElement | null;
      const world = host?.querySelector("[data-instances-json]") as HTMLElement | null;
      let instanceCount = 0;
      try {
        instanceCount = JSON.parse(world?.getAttribute("data-instances-json") || "[]").length;
      } catch {
        instanceCount = -1;
      }
      let vortexCount = 0;
      try {
        vortexCount = JSON.parse((host?.querySelector("[data-vortices-json]") as HTMLElement | null)?.getAttribute("data-vortices-json") || "[]").length;
      } catch {
        vortexCount = -1;
      }
      if (!btn) return { frameBtn: 0, pe: "", w: 0, h: 0, x: 0, y: 0, instanceCount, vortexCount };
      const cs = getComputedStyle(btn);
      const r = btn.getBoundingClientRect();
      return { frameBtn: 1, pe: cs.pointerEvents, w: Math.round(r.width), h: Math.round(r.height), x: Math.round(r.x), y: Math.round(r.y), instanceCount, vortexCount };
    });
    log(`frame dump ${JSON.stringify(dump)}`);
    const frameBtn = persp.locator('[id^="world3d-frame-instances-"]').first();
    const frameCount = await frameBtn.count();
    if (frameCount) {
      await frameBtn.click({ force: true, timeout: 4000 }).catch(() => {});
      await page.evaluate(() => {
        const host = document.querySelector("#puzzle3d-main-perspective");
        (host?.querySelector('[id^="world3d-frame-instances-"]') as HTMLButtonElement | null)?.click();
      });
    }
    const named = persp.getByRole("button", { name: /^Frame$/ });
    const namedCount = await named.count();
    for (let i = 0; i < namedCount; i++) {
      await named.nth(i).click({ force: true, timeout: 2000 }).catch(() => {});
    }
    await page.waitForTimeout(700);
    const box = await c.boundingBox();
    if (!box) throw new Error("no canvas");
    const table = { x: Math.round(box.width * 0.78), y: Math.round(box.height * 0.42) };
    log(`framed forest table frameBtn=${frameCount} named=${namedCount} box=${Math.round(box.width)}x${Math.round(box.height)} table=${table.x},${table.y}`);
    return { box, table };
  };
  const hopCensus = () => {
    const keys = [
      "brush-place hop",
      "brush-place deferred",
      "suggestions-rightdown",
      "suggestions-contextmenu",
      "interactionHover dispatch",
      "import-picker hop",
      "importFixture ingress",
      "request-file-open mapped",
      "openVortexSuggestions",
      "addBrushObject",
    ];
    const counts: Record<string, number> = {};
    for (const key of keys) counts[key] = consoleBuf.filter((l) => l.includes(key)).length;
    return counts;
  };
  const dumpVortexHoverFlags = async () =>
    page.evaluate(() => {
      const host = document.querySelector("#puzzle3d-main-perspective [data-vortices-json]") as HTMLElement | null;
      let parsed: Array<{ fullId?: string; hovered?: boolean }> = [];
      try {
        parsed = JSON.parse(host?.getAttribute("data-vortices-json") || "[]");
      } catch {
        parsed = [];
      }
      return { count: parsed.length, hovered: parsed.filter((v) => v.hovered).map((v) => v.fullId) };
    });
  const vortexHits = async () => {

    const hits = await page.evaluate(() => {
      const host = document.querySelector("#puzzle3d-main-perspective [data-instances-json]") as HTMLElement | null;
      let parsed: Array<{ fullId?: string; x?: number; y?: number; z?: number; sx?: number; sy?: number; ndcZ?: number }> = [];
      try {
        parsed = JSON.parse(host?.getAttribute("data-vortex-hits") || "[]");
      } catch {
        parsed = [];
      }
      let vortices = 0;
      try {
        vortices = JSON.parse(host?.getAttribute("data-vortices-json") || "[]").length;
      } catch {
        vortices = -1;
      }
      return { vortices, hits: parsed.slice(0, 20) };
    });
    log(`vortex hits ${JSON.stringify(hits).slice(0, 800)}`);
    return hits;
  };
  const gumballHits = async () => {
    const hits = await page.evaluate(() => {
      const host = document.querySelector("#puzzle3d-main-perspective [data-instances-json]") as HTMLElement | null;
      let parsed: Array<{ kind?: string; sx?: number; sy?: number; ndcZ?: number }> = [];
      try {
        parsed = JSON.parse(host?.getAttribute("data-gumball-hits") || "[]");
      } catch {
        parsed = [];
      }
      const entered = Boolean((window as unknown as { __gumballDragEntered?: boolean }).__gumballDragEntered);
      return { hits: parsed, entered };
    });
    log(`gumball hits ${JSON.stringify(hits).slice(0, 800)}`);
    return hits;
  };
  const waitGumballHits = async () => {
    let last = { hits: [] as Array<{ kind?: string; sx?: number; sy?: number; ndcZ?: number }>, entered: false };
    for (let i = 0; i < 16; i++) {
      last = await gumballHits();
      if (last.hits.some((h) => (h.sx ?? -1) >= 8 && (h.sy ?? -1) >= 8 && (h.ndcZ ?? 2) >= -1 && (h.ndcZ ?? 2) <= 1)) break;
      await page.waitForTimeout(250);
    }
    return last;
  };
  const dragGumballMoveX = async (box: { x: number; y: number; width: number; height: number }) => {
    await page.evaluate(() => {
      (window as unknown as { __gumballDragEntered?: boolean }).__gumballDragEntered = false;
    });
    const c = page.locator("canvas").last();
    await c.hover({ position: { x: 80, y: 80 }, timeout: 4000 }).catch(() => {});
    await page.evaluate(() => {
      const canvas = document.querySelector("#puzzle3d-main-perspective canvas");
      canvas?.dispatchEvent(new PointerEvent("pointerenter", { bubbles: true }));
    });
    await page.waitForTimeout(600);
    const dump = await waitGumballHits();
    const onscreen = dump.hits.filter((h) => {
      const sx = h.sx ?? -1;
      const sy = h.sy ?? -1;
      const z = h.ndcZ ?? 2;
      return sx >= 8 && sy >= 8 && sx <= box.width - 8 && sy <= box.height - 8 && z >= -1 && z <= 1;
    });
    const handle = onscreen.find((h) => h.kind === "moveX") ?? onscreen[0];
    log(`gumball handle ${JSON.stringify(handle)} onscreen=${onscreen.length}`);
    if (!handle || handle.sx == null || handle.sy == null) return { handle: null, entered: false };
    await c.hover({ position: { x: handle.sx, y: handle.sy }, timeout: 4000 }).catch(() => {});
    await page.waitForTimeout(200);
    await page.mouse.move(box.x + handle.sx, box.y + handle.sy);
    await page.mouse.down();
    await page.mouse.move(box.x + handle.sx + 72, box.y + handle.sy + 72, { steps: 16 });
    await page.mouse.up();
    await page.waitForTimeout(800);
    const after = await gumballHits();
    return { handle, entered: after.entered };
  };
  const clickVortexHits = async (box: { x: number; y: number; width: number; height: number }) => {
    const dump = await vortexHits();
    const onscreen = dump.hits.filter((h) => {
      const sx = h.sx ?? -1;
      const sy = h.sy ?? -1;
      const z = h.ndcZ ?? 2;
      return sx >= 8 && sy >= 8 && sx <= box.width - 8 && sy <= box.height - 8 && z >= -1 && z <= 1;
    });
    log(`vortex onscreen=${onscreen.length}`);
    const c = page.locator("#puzzle3d-main-perspective canvas").last();
    for (const hit of onscreen.slice(0, 8)) {
      const sx = hit.sx ?? 0;
      const sy = hit.sy ?? 0;
      await c.hover({ position: { x: sx, y: sy }, timeout: 4000 }).catch(() => {});
      await page.waitForTimeout(120);
      await page.mouse.click(box.x + sx, box.y + sy);
      await page.waitForTimeout(200);
    }
    return onscreen;
  };
  const activateWorkspaceMenuOrdinal = async (ordinal: string) => {
    await dismissChrome();
    const c = page.locator("canvas").last();
    const box = await c.boundingBox();
    if (!box) throw new Error("no canvas");
    await page.mouse.click(box.x + 200, box.y + 160, { button: "right" });
    await page.waitForTimeout(500);
    log(`menu before ${ordinal}: ${JSON.stringify(await chromeState()).slice(0, 500)}`);
    const menuAction = ordinal === "4" ? "exportFixture" : "openImportFixture";
    const byAction = page.locator(`[data-menu-action="${menuAction}"]`).last();
    const byId = page.locator(`[id="shell-menu.action.${menuAction}"]`).last();
    const dump = await page.evaluate((action) => {
      const nodes = Array.from(document.querySelectorAll(`[data-menu-action="${action}"], [id="shell-menu.action.${action}"]`));
      return nodes.map((el) => {
        const b = el as HTMLButtonElement;
        const cs = getComputedStyle(b);
        return { id: b.id, action: b.getAttribute("data-menu-action"), disabled: b.disabled, pe: cs.pointerEvents, display: cs.display, text: (b.textContent ?? "").replace(/\s+/g, " ").trim().slice(0, 40) };
      });
    }, menuAction);
    log(`menu nodes ${menuAction}: ${JSON.stringify(dump)}`);
    if (await byAction.count()) {
      await byAction.click({ force: true, timeout: 4000 }).catch(() => {});
      await page.evaluate((action) => {
        const el = document.querySelector(`[data-menu-action="${action}"]`) as HTMLButtonElement | null;
        el?.click();
      }, menuAction);
      log(`menu data-action click ${menuAction}`);
    } else if (await byId.count()) {
      await byId.click({ force: true, timeout: 4000 }).catch(() => {});
      log(`menu id click ${menuAction}`);
    } else {
    const clicked = await page.evaluate((label) => {
      const buttons = Array.from(document.querySelectorAll("[role='menu'] button, [role='menuitem']"));
      const dump = buttons.map((b) => {
        const el = b as HTMLButtonElement;
        return {
          text: (el.textContent ?? "").replace(/\s+/g, " ").trim().slice(0, 60),
          id: el.id || "",
          disabled: el.disabled || el.getAttribute("aria-disabled") === "true",
          role: el.getAttribute("role"),
        };
      });
      const row = buttons.find((b) => new RegExp(label, "i").test(b.textContent ?? ""));
      if (!row) return `missing:${dump.map((d) => d.text).join("|").slice(0, 200)}`;
      (row as HTMLButtonElement).click();
      return `clicked:${(row.textContent ?? "").trim()} disabled=${(row as HTMLButtonElement).disabled}`;
    }, ordinal === "4" ? "export" : "import");
    log(`menu dom click ${clicked}`);
    }
    await page.waitForTimeout(400);
  };

  const historyState = async () =>
    page.evaluate(() => {
      const q = (sel: string) => Array.from(document.querySelectorAll(sel));
      const combo = document.querySelector('[role="combobox"]') as HTMLElement | null;
      const entries = q('[id^="framework.history.entry."]').map((r) => `${r.id}=${(r as HTMLElement).innerText.replace(/\n/g, " ").trim().slice(0, 80)}`);
      const sections = q('[id^="framework.history."]').map((r) => `${r.id}=${(r as HTMLElement).innerText.replace(/\n/g, " ").trim().slice(0, 60)}`).slice(0, 40);
      const tree = q('[data-slot="tree-item"], [role="treeitem"]').map((r) => `${r.id || "?"}=${(r as HTMLElement).innerText.replace(/\n/g, " ").trim().slice(0, 80)}`).slice(0, 40);
      return {
        example: (combo?.innerText || "").replace(/\n/g, " ").slice(0, 80),
        entries,
        entryCount: entries.length,
        sections,
        tree,
      };
    });
  const openHistory = async () => {
    const tab = page.locator('#framework.panel.history, [data-slot="panel-tab-button"][id="framework.panel.history"]').first();
    if (await tab.count()) await tab.click({ timeout: 5000 }).catch(() => {});
    else await page.locator("button", { hasText: /^\s*history\s*$/i }).first().click({ timeout: 5000 }).catch(() => {});
    await page.waitForTimeout(800);
    await page.locator("#framework.history.actions").first().click({ timeout: 2000 }).catch(() => {});
    await page.locator("#framework.history.undo").first().click({ timeout: 2000 }).catch(() => {});
    for (const sel of ["#framework.history.commands", "#framework.history.actions"]) {
      const loc = page.locator(sel).first();
      if (await loc.count()) await loc.click({ timeout: 2000 }).catch(() => {});
    }
    const collapsed = page.locator('[role="treeitem"][aria-expanded="false"], [data-slot="tree-item"][aria-expanded="false"]');
    const n = await collapsed.count();
    for (let i = 0; i < Math.min(n, 6); i++) await collapsed.nth(i).click({ timeout: 1500 }).catch(() => {});
    await page.waitForTimeout(600);
  };
  add("fill-history", "§12", "mutate", process.argv.includes("--fill") || battery, async () => {
    await openHistory();
    const hist = await historyState();
    log(`fill history after apply: ${JSON.stringify(hist).slice(0, 1600)}`);
    verdict("fill-history-entry", hist.entryCount > 0, `entries=${hist.entryCount}`);
  });
  const dispatchHistory = async (kind: "undo" | "redo") => {
    const before = await historyState();
    const id = kind === "undo" ? "framework.history.undo" : "framework.history.redo";
    await page.evaluate((target) => {
      const root = document.getElementById(target);
      const btn = (root?.querySelector("button") as HTMLElement | null) ?? (root as HTMLElement | null);
      btn?.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
    }, id);
    await page.waitForTimeout(2800);
    let after = await historyState();
    const changed = after.example !== before.example || JSON.stringify(after.entries) !== JSON.stringify(before.entries);
    if (!changed) {
      if (kind === "undo") await page.keyboard.press("Meta+z");
      else {
        await page.keyboard.press("Meta+Shift+z");
        await page.waitForTimeout(400);
        await page.keyboard.press("Control+Shift+z");
      }
      await page.waitForTimeout(2800);
      after = await historyState();
    }
    return after;
  };
  const chromeReplayCensus = () => {
    const chrome = consoleBuf.filter((l) => /chrome history action=/.test(l)).length;
    const replay = consoleBuf.filter((l) => /replayShellCommand dispatch/.test(l)).length;
    return { chrome, replay };
  };
  {
    add("history-open", "§20", "read", process.argv.includes("--undo") || family, async () => {
      await openHistory();
      log(`history state: ${JSON.stringify(await historyState()).slice(0, 1600)}`);
    });
    {
      add("undo-unwind", "§20", "replace", family, async () => {
        let last = await historyState();
        log(`unwind start: example=${last.example} entries=${JSON.stringify(last.entries).slice(0, 600)}`);
        for (let i = 0; i < 12; i++) {
          last = await dispatchHistory("undo");
          const live = last.entries.filter((row) => row.includes("↶"));
          const exampleLive = live.some((row) => /set active example/i.test(row));
          log(`unwind ${i}: example=${last.example} live=${live.length} exampleLive=${exampleLive} census=${JSON.stringify(chromeReplayCensus())} entries=${JSON.stringify(last.entries).slice(0, 500)}`);
          if (/concrete forest/i.test(last.example) && !/nakagin/i.test(last.example) && !exampleLive) break;
        }
        log(`unwind done: example=${last.example} census=${JSON.stringify(chromeReplayCensus())}`);
        verdict("undo-unwind", /concrete forest/i.test(last.example) && !/nakagin/i.test(last.example), `example=${last.example}`);
      });
      add("undo-redo", "§20", "replace", family, async () => {
        let last = await historyState();
        log(`redo start: example=${last.example} entries=${JSON.stringify(last.entries).slice(0, 600)}`);
        for (let i = 0; i < 12; i++) {
          last = await dispatchHistory("redo");
          log(`redo ${i}: example=${last.example} census=${JSON.stringify(chromeReplayCensus())} entries=${JSON.stringify(last.entries).slice(0, 500)}`);
          if (/nakagin/i.test(last.example)) break;
        }
        log(`redo done: example=${last.example} census=${JSON.stringify(chromeReplayCensus())}`);
        verdict("undo-redo", /nakagin/i.test(last.example), `example=${last.example}`);
      });
    }
    add("undo-once", "§20", "replace", !family && process.argv.includes("--undo"), async () => {
      log(`before undo: ${JSON.stringify(await historyState()).slice(0, 1600)}`);
      const clicked = await page.evaluate(() => {
        const clicks: string[] = [];
        const undo = document.getElementById("framework.history.undo") as HTMLElement | null;
        const run = document.getElementById("framework.history.undo.run") as HTMLElement | null;
        const entry2 = document.getElementById("framework.history.entry.2") as HTMLElement | null;
        for (const [name, el] of [
          ["run", run],
          ["undoBtn", undo?.querySelector("button") ?? null],
          ["undo", undo],
          ["entry2btn", entry2?.querySelector("button") ?? null],
          ["entry2", entry2],
        ] as const) {
          if (el) {
            el.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
            clicks.push(name);
          }
        }
        return { clicks, undoText: undo?.innerText.slice(0, 80) ?? null, entry2: entry2?.innerText.slice(0, 80) ?? null };
      });
      log(`dom clicks: ${JSON.stringify(clicked)}`);
      await page.waitForTimeout(3000);
      log(`after dom click: ${JSON.stringify(await historyState()).slice(0, 1200)}`);
      await page.locator("canvas").last().click({ force: true, timeout: 3000 }).catch(() => {});
      await page.keyboard.press("Meta+z");
      await page.waitForTimeout(2500);
      log(`after meta-z: ${JSON.stringify(await historyState()).slice(0, 1200)}`);
      await page.keyboard.press("Control+z");
      await page.waitForTimeout(2500);
      log(`after ctrl-z: ${JSON.stringify(await historyState()).slice(0, 1200)}`);
      await page.keyboard.press("Meta+z");
      await page.waitForTimeout(4000);
      log(`after meta-z-2: ${JSON.stringify(await historyState()).slice(0, 1200)}`);
    });

  }
  {
    add("selection-surfaces", "§6/§16", "mutate", process.argv.includes("--selection") || family, async () => {
      await dismissChrome();
      const framed = await frameForestTableAfterCensus();
      const inspection = page.locator("#framework.panel.inspection, button").filter({ hasText: /^inspection$/i }).first();
      if (await inspection.count()) await inspection.click({ timeout: 4000 }).catch(() => {});
      await page.waitForTimeout(600);
      log(`selection before: ${JSON.stringify(await selectionState()).slice(0, 1000)}`);
      log(`inspection before: ${JSON.stringify(await inspectionPopulate()).slice(0, 1400)}`);
      const leftoverBefore = leftoverLaneCensus().leftoverCount;
      const hit = await clickForestTable();
      log(`selection clicks box=${JSON.stringify(hit.box)} spots=${JSON.stringify(hit.spots)}`);
      const c = page.locator("canvas").last();
      await c.click({ position: { x: framed.table.x, y: framed.table.y }, timeout: 5000 }).catch(() => {});
      await waitLeftoverInteractionView("selection");
      const debug = await page.evaluate(() => (window as unknown as { __abDebug?: string[] }).__abDebug ?? []);
      log(`selection debug: ${JSON.stringify(debug)}`);
      log(`selection after: ${JSON.stringify(await selectionState()).slice(0, 1200)}`);
      log(`inspection after: ${JSON.stringify(await inspectionPopulate()).slice(0, 1800)}`);
      const inspect = await waitInspectionPopulated("selection");
      verdict("inspection-object-fields", inspect.populated && !inspect.emptyPresent, `populated=${inspect.populated} empty=${inspect.emptyPresent} id=${inspect.objectId}`, inspect.populated ? undefined : 42);
      verdict("inspection-locked-flag-row", inspect.lockChromePresent, `lockChrome=${inspect.lockChromePresent}`, inspect.lockChromePresent ? undefined : 42);
      log(`selection chrome: ${JSON.stringify(await chromeState()).slice(0, 800)}`);
      const inspectSelect = consoleBuf.filter((l) => /interactionSelect|job done kind=framework.reserved|leftover|performInvocation settled/.test(l)).slice(-24);
      log(`selection leftover-before=${leftoverBefore}`);
      log(`selection lane: ${JSON.stringify(inspectSelect).slice(0, 2200)}`);
    });
  }
  {
    add("clipboard-copy-paste", "§21", "mutate", process.argv.includes("--clipboard") || family, async () => {
      await dismissChrome();
      const framed = await frameForestTableAfterCensus();
      await clickForestTable();
      await page.locator("canvas").last().click({ position: { x: framed.table.x, y: framed.table.y }, timeout: 5000 }).catch(() => {});
      await waitLeftoverInteractionView("clipboard");
      log(`clipboard inspection: ${JSON.stringify(await waitInspectionPopulated("clipboard")).slice(0, 1200)}`);
      const debug = await page.evaluate(() => (window as unknown as { __abDebug?: string[] }).__abDebug ?? []);
      log(`clipboard pick debug: ${JSON.stringify(debug)}`);
      const before = await chromeState();
      const censusBefore = await dumpInstances();
      log(`clipboard before: ${JSON.stringify(before).slice(0, 800)}`);
      log(`clipboard census before: ${JSON.stringify(censusBefore)}`);
      const actions = page.locator("button", { hasText: /^actions$/i }).last();
      if (await actions.count()) await actions.click({ timeout: 3000 }).catch(() => {});
      await page.waitForTimeout(400);
      const copyById = page.locator("[id*='action.copy'], [data-menu-action='copy'], [id*='copySelection'], [id*='copy.execute']").first();
      const copyBtn = page.locator("button, [role='menuitem']").filter({ hasText: /^copy$/i }).first();
      log(`copyBtn=${await copyBtn.count()} copyById=${await copyById.count()}`);
      if (await copyById.count()) await copyById.click({ force: true, timeout: 3000 }).catch(() => {});
      else if (await copyBtn.count()) await copyBtn.click({ force: true, timeout: 3000 }).catch(() => {});
      else {
        const command = page.locator("button").filter({ hasText: /^command$/i }).last();
        if (await command.count()) {
          await command.click({ timeout: 3000 }).catch(() => {});
          await page.waitForTimeout(400);
          const box = page.locator("input, [role='textbox']").last();
          if (await box.count()) await box.fill("copy").catch(() => {});
          await page.keyboard.press("Enter");
          await page.waitForTimeout(600);
        }
      }
      await page.keyboard.press("Meta+c");
      await page.waitForTimeout(600);
      await page.keyboard.press("Control+c");
      await page.waitForTimeout(800);
      const afterCopy = await page.evaluate(() => (window as unknown as { __abDebug?: string[] }).__abDebug ?? []);
      log(`clipboard after-copy debug: ${JSON.stringify(afterCopy)}`);
      const copyLane = consoleBuf.filter((l) => /actionId=copy|actionId=paste|clipboard-write|unmapped effect|leftover effects|performInvocation settled/.test(l)).slice(-28);
      log(`clipboard copy-lane: ${JSON.stringify(copyLane).slice(0, 2400)}`);
      await page.keyboard.press("Meta+v");
      await page.waitForTimeout(600);
      await page.keyboard.press("Control+v");
      await page.waitForTimeout(1500);
      const pasteLane = consoleBuf.filter((l) => /actionId=paste|clipboard-write|CreateObject|leftover effects|performInvocation settled/.test(l)).slice(-16);
      log(`clipboard paste-lane: ${JSON.stringify(pasteLane).slice(0, 1600)}`);
      const exec = page.locator("#framework.window.puzzle3dMainPerspective.action.paste.execute, button", { hasText: /^execute$/i }).first();
      if (await exec.count()) await exec.click({ force: true, timeout: 4000 }).catch(() => {});
      await page.waitForTimeout(3000);
      await openHistory();
      const censusAfter = await dumpInstances();
      log(`clipboard after: ${JSON.stringify(await chromeState()).slice(0, 800)}`);
      log(`clipboard census after: ${JSON.stringify(censusAfter)} delta=${censusAfter.count - censusBefore.count}`);
      verdict("clipboard", censusAfter.count > censusBefore.count, `delta=${censusAfter.count - censusBefore.count} historyHasCopyPaste=${/Copy|Paste/.test((await historyState()).entries.join(" "))}`, censusAfter.count > censusBefore.count ? undefined : 42);
      log(`clipboard history: ${JSON.stringify(await historyState()).slice(0, 800)}`);
    });
  }
  {
    add("marquee-drag", "§6", "mutate", process.argv.includes("--marquee") || battery, async () => {
      await dismissChrome();
      const box = await page.locator("canvas").last().boundingBox();
      if (!box) throw new Error("no canvas box");
      await page.mouse.move(box.x + 40, box.y + 40);
      await page.mouse.down();
      await page.mouse.move(box.x + box.width - 40, box.y + box.height - 80, { steps: 12 });
      await page.mouse.up();
      await page.waitForTimeout(1500);
      log(`marquee after drag: ${JSON.stringify(await chromeState()).slice(0, 800)}`);
    });
    add("marquee-click", "§6", "mutate", process.argv.includes("--marquee") || battery, async () => {
      const c = page.locator("canvas").last();
      await c.click({ position: { x: 470, y: 420 }, timeout: 5000 });
      log(`marquee after click: ${JSON.stringify(await chromeState()).slice(0, 800)}`);
    });
  }
  {
    add("locked-refusal", "§8/§16", "mutate", process.argv.includes("--locked") || family, async () => {
      await dismissChrome();
      const framed = await frameForestTableAfterCensus();
      const c = page.locator("canvas").last();
      await clickForestTable();
      await c.click({ position: { x: framed.table.x, y: framed.table.y }, timeout: 5000 }).catch(() => {});
      await waitLeftoverInteractionView("locked");
      const inspection = page.locator("#framework.panel.inspection").first();
      if (await inspection.count()) await inspection.click({ timeout: 4000 }).catch(() => {});
      await page.waitForTimeout(800);
      const inspect = await waitInspectionPopulated("locked");
      log(`locked inspection: ${JSON.stringify(inspect).slice(0, 1800)}`);
      verdict("locked-flag-row", inspect.lockChromePresent, `lockChrome=${inspect.lockChromePresent}`, inspect.lockChromePresent ? undefined : 42);
      const lock = page.locator("[id$='puzzle3d-play-inspector.object.locked']").first();
      const lockCount = await lock.count();
      log(`lock controls=${lockCount}`);
      if (lockCount) {
        const toggle = lock.locator("button, [role='switch'], [role='checkbox']").first();
        if (await toggle.count()) await toggle.click({ force: true, timeout: 4000 }).catch(() => {});
        else await lock.click({ force: true, timeout: 4000 }).catch(() => {});
      }
      await unfoldPerspectiveUtilities();
      const move = page.locator("#move, button").filter({ hasText: /^move$/i }).first();
      if (await move.count()) await move.click({ timeout: 3000 }).catch(() => {});
      const box = await c.boundingBox();
      if (box) await dragGumballMoveX(box);
      await page.waitForTimeout(1500);
      const after = await chromeState();
      const notices = (after.notices ?? []).filter((n: string) => !isCollateralNotice(n));
      const notice = notices.some((n: string) => /locked/i.test(n));
      log(`locked after: ${JSON.stringify(after).slice(0, 1000)}`);
      verdict("locked-refusal-notice", notice, `notices=${JSON.stringify(notices).slice(0, 200)}`, notice ? undefined : 42);
    });
  }
  {
    add("gumball-drag", "§8", "mutate", process.argv.includes("--gumball") || family, async () => {
      await dismissChrome();
      const framed = await frameForestTableAfterCensus();
      const c = page.locator("canvas").last();
      await clickForestTable();
      await c.click({ position: { x: framed.table.x, y: framed.table.y }, timeout: 5000 }).catch(() => {});
      await waitLeftoverInteractionView("gumball");
      log(`gumball inspection: ${JSON.stringify(await inspectionPopulate()).slice(0, 1200)}`);
      await unfoldPerspectiveUtilities();
      const move = page.locator("#move, button, [data-slot='toggle-group-item']").filter({ hasText: /^move$/i }).first();
      if (await move.count()) await move.click({ timeout: 3000 }).catch(() => {});
      const poseBefore = await page.evaluate(() => document.querySelector("#puzzle3d-main-perspective [data-instances-json]")?.getAttribute("data-instances-json") ?? "");
      log(`gumball pose before len=${poseBefore.length}`);
      const box = await c.boundingBox();
      const drag = box ? await dragGumballMoveX(box) : { handle: null, entered: false };
      await page.waitForTimeout(1500);
      await openHistory();
      const poseAfter = await page.evaluate(() => document.querySelector("#puzzle3d-main-perspective [data-instances-json]")?.getAttribute("data-instances-json") ?? "");
      const enteredLogs = consoleBuf.filter((l) => /\[DEBUG\] gumball drag entered/.test(l));
      const deltaLogs = consoleBuf.filter((l) => /\[DEBUG\] gumball pose delta/.test(l));
      const entered = drag.entered || enteredLogs.length > 0;
      const sceneDelta = poseBefore !== poseAfter;
      log(`gumball after: ${JSON.stringify(await chromeState()).slice(0, 800)}`);
      log(`gumball sceneDelta=${sceneDelta} poseAfterLen=${poseAfter.length} instances=${JSON.stringify(await dumpInstances())}`);
      log(`gumball hops entered=${JSON.stringify(enteredLogs.slice(-6)).slice(0, 800)} delta=${JSON.stringify(deltaLogs.slice(-8)).slice(0, 800)}`);
      log(`gumball history: ${JSON.stringify(await historyState()).slice(0, 800)}`);
      const moveEntered = enteredLogs.some((l) => /kind: move/.test(l));
      verdict("gumball-handle-enter", entered && moveEntered, `handle=${JSON.stringify(drag.handle)} entered=${drag.entered} moveEntered=${moveEntered} taps=${enteredLogs.length}`);
      verdict("gumball-scene-delta", sceneDelta, `sceneDelta=${sceneDelta} poseLen=${poseAfter.length}`);
    });
  }
  {
    add("frame-perspective", "§0", "mutate", process.argv.includes("--brush") || process.argv.includes("--suggestions") || process.argv.includes("--frame") || battery, async () => {
      await dismissChrome();
      await frameForestTableAfterCensus();
    });
  }
  {
    add("brush-stroke", "§9", "mutate", process.argv.includes("--brush") || battery, async () => {
      await dismissChrome();
      const before = await selectionState();
      log(`brush census before: ${JSON.stringify(before).slice(0, 500)}`);
      await armBrushUtility();
      const published = await waitBrushVortices();
      log(`brush vortices after arm: ${JSON.stringify(published)}`);
      const framed = await frameForestTableAfterCensus();
      const publishedFramed = await dumpVorticesJson();
      log(`brush vortices after frame: ${JSON.stringify(publishedFramed)}`);
      await page.screenshot({ path: join(OUT, `probe-${stamp}-brush-framed.png`) }).catch(() => {});
      await page.screenshot({ path: join(OUT, `probe-${stamp}-brush-vortices.png`) }).catch(() => {});
      const c = page.locator("canvas").last();
      const box = framed.box;
      const table = framed.table;
      const dump = await vortexHits();
      const onscreen = dump.hits.filter((h) => {
        const sx = h.sx ?? -1;
        const sy = h.sy ?? -1;
        const z = h.ndcZ ?? 2;
        return sx >= 8 && sy >= 8 && sx <= box.width - 8 && sy <= box.height - 8 && z >= -1 && z <= 1;
      });
      log(`vortex onscreen=${onscreen.length}`);
      const stormHits = onscreen.length ? onscreen : [{ sx: table.x, sy: table.y, fullId: "?" }];
      const beforePreview = await dumpBrushPreview();
      const beforeInstances = await dumpInstances();
      log(`brush preview before storm: ${JSON.stringify(beforePreview)} instances=${JSON.stringify(beforeInstances)}`);
      for (let i = 0; i < 70; i++) {
        const hit = stormHits[i % stormHits.length];
        const sx = hit.sx ?? table.x;
        const sy = hit.sy ?? table.y;
        await page.mouse.move(box.x + sx, box.y + sy, { steps: 1 });
      }
      let afterStorm = await dumpBrushPreview();
      for (let i = 0; i < 12 && !afterStorm.target; i++) {
        await page.waitForTimeout(400);
        afterStorm = await dumpBrushPreview();
        log(`brush preview poll ${i} rawLen=${afterStorm.rawLen} target=${afterStorm.target} hover=${afterStorm.hover} utility=${afterStorm.utility}`);
      }
      log(`brush preview after hover-storm: ${JSON.stringify(afterStorm)}`);
      log(`brush vortex-flags after storm: ${JSON.stringify(await dumpVortexHoverFlags())}`);
      log(`brush hop-census after storm: ${JSON.stringify(hopCensus())}`);
      await page.screenshot({ path: join(OUT, `probe-${stamp}-brush-hover-storm.png`) }).catch(() => {});
      const aim = stormHits[0];
      const sx = aim.sx ?? table.x;
      const sy = aim.sy ?? table.y;
      log(`brush click at canvas ${Math.round(sx)},${Math.round(sy)} page ${Math.round(box.x + sx)},${Math.round(box.y + sy)} aim=${JSON.stringify(aim)}`);
      await c.hover({ position: { x: sx, y: sy }, timeout: 4000 }).catch(() => {});
      await page.waitForTimeout(200);
      await c.click({ position: { x: sx, y: sy }, timeout: 4000 }).catch(() => {});
      await page.waitForTimeout(2500);
      log(`brush hop-census after click: ${JSON.stringify(hopCensus())}`);
      log(`brush place hops=${JSON.stringify(consoleBuf.filter((l) => /\[DEBUG\] (brush-place|interactionHover dispatch|addBrushObject)/.test(l)).slice(-16)).slice(0, 1400)}`);
      log(`brush instances after click: ${JSON.stringify(await dumpInstances())}`);
      {
        const placed = await dumpInstances();
        const preview = await dumpBrushPreview();
        verdict("brush-preview-place", placed.count > beforeInstances.count || Boolean(preview.target || afterStorm.target), `instances=${placed.count} preview=${preview.target ?? afterStorm.target}`, 41);
      }
      log(`brush preview after click: ${JSON.stringify(await dumpBrushPreview())}`);
      log(`brush vortex-flags after click: ${JSON.stringify(await dumpVortexHoverFlags())}`);
      await openHistory();
      const hist = await historyState();
      const addBrush = [...hist.entries, ...hist.tree].filter((row) => /addBrushObject|brush object|Add Brush/i.test(row));
      log(`brush addBrushObject rows=${JSON.stringify(addBrush).slice(0, 400)}`);
      log(`brush census after: ${JSON.stringify(await selectionState()).slice(0, 800)}`);
      log(`brush history: ${JSON.stringify(hist).slice(0, 800)}`);
      log(`brush after: ${JSON.stringify(await chromeState()).slice(0, 1000)}`);
      const afterV = await dumpVorticesJson();
      log(`brush vortices after clicks: ${JSON.stringify(afterV)}`);
      await page.keyboard.press("Escape").catch(() => {});
      await page.locator('#framework.panel.history, [data-slot="panel-tab-button"][id="framework.panel.history"]').first().click({ timeout: 2000 }).catch(() => {});
      await page.keyboard.press("Escape").catch(() => {});
    });
  }
  {
    add("suggestions-open", "§13", "mutate", process.argv.includes("--suggestions") || battery, async () => {
      await dismissChrome();
      await armBrushUtility();
      const published = await waitBrushVortices();
      log(`suggestions vortices after brush: ${JSON.stringify(published)}`);
      const framed = await frameForestTableAfterCensus();
      await page.screenshot({ path: join(OUT, `probe-${stamp}-suggestions-framed.png`) }).catch(() => {});
      const c = page.locator("canvas").last();
      const box = framed.box;
      const table = framed.table;
      const dump = await vortexHits();
      const onscreen = dump.hits.filter((h) => {
        const sx = h.sx ?? -1;
        const sy = h.sy ?? -1;
        const z = h.ndcZ ?? 2;
        return sx >= 8 && sy >= 8 && sx <= box.width - 8 && sy <= box.height - 8 && z >= -1 && z <= 1;
      });
      log(`suggestions vortex onscreen=${onscreen.length}`);
      const aim = onscreen[0];
      if (aim) {
        await c.hover({ position: { x: aim.sx ?? table.x, y: aim.sy ?? table.y }, timeout: 4000 }).catch(() => {});
        await page.waitForTimeout(200);
      }
      const rx = box.x + (aim?.sx ?? table.x);
      const ry = box.y + (aim?.sy ?? table.y);
      await page.mouse.move(rx, ry);
      await page.waitForTimeout(300);
      await page.mouse.move(rx, ry);
      await page.waitForTimeout(400);
      const pre = await dumpBrushPreview();
      log(`suggestions pre-gesture preview: ${JSON.stringify(pre)}`);
      log(`suggestions vortex-flags: ${JSON.stringify(await dumpVortexHoverFlags())}`);
      log(`suggestions gesture at ${Math.round(rx)},${Math.round(ry)} aim=${JSON.stringify(aim)} utility=${pre.utility ?? "null"} hover=${pre.hover ?? "null"}`);
      if (pre.utility !== "brush") await armBrushUtility();
      await page.mouse.move(rx, ry);
      await page.waitForTimeout(300);
      await page.keyboard.down("Alt").catch(() => {});
      await page.waitForTimeout(80);
      await page.mouse.click(rx, ry, { button: "right", modifiers: ["Alt"] });
      await page.waitForTimeout(2000);
      await page.keyboard.up("Alt").catch(() => {});
      const menus = await page.locator('[role="menu"]').count();
      const suggestRows = await page.locator('[role="menuitem"], button, [data-slot="tree-item"]').filter({ hasText: /suggest|placement|candidate/i }).count();
      const hop = consoleBuf.filter((l) => /\[DEBUG\] (vortex-hover|brush-place|suggestions-)/.test(l)).slice(-20);
      log(`suggestions hops=${JSON.stringify(hop).slice(0, 1200)}`);
      log(`suggestions hop-census: ${JSON.stringify(hopCensus())}`);
      log(`suggestions menus=${menus} suggestRows=${suggestRows}`);
      verdict("suggestions", menus > 0 || suggestRows > 0 || hop.length > 0, `menus=${menus} rows=${suggestRows} hops=${hop.length}`, 41);
      log(`suggestions menu dump: ${JSON.stringify(await dumpSuggestionMenu()).slice(0, 1200)}`);
      log(`suggestions after: ${JSON.stringify(await chromeState()).slice(0, 1200)}`);
    });
  }
  {
    add("export-import", "§24", "replace", process.argv.includes("--importexport") || process.argv.includes("--import") || battery, async () => {
      log(`importexport chrome: ${JSON.stringify(await chromeState()).slice(0, 1000)}`);
      const censusBefore = await selectionState();
      log(`census before: ${JSON.stringify(censusBefore).slice(0, 600)}`);
      let dest = join(OUT, `probe-${stamp}-export.json`);
      let [download] = await Promise.all([
        page.waitForEvent("download", { timeout: 15000 }).catch(() => null),
        activateWorkspaceMenuOrdinal("4"),
      ]);
      log(`export download=${download ? download.suggestedFilename() : "none"}`);
      if (!download) {
        const actionsChip = page.locator('[id="framework.window.puzzle3dMainPerspective.actionPane.unfold"], button').filter({ hasText: /^actions$/i }).last();
        if (await actionsChip.count()) await actionsChip.click({ timeout: 3000 }).catch(() => {});
        await page.waitForTimeout(500);
        const exportAction = page.locator('[id*="exportFixture"], button').filter({ hasText: /^export$/i }).first();
        log(`action-pane export=${await page.locator('[id*="exportFixture"]').count()} textBtn=${await page.locator("button").filter({ hasText: /^export$/i }).count()}`);
        const [paneDownload] = await Promise.all([
          page.waitForEvent("download", { timeout: 8000 }).catch(() => null),
          exportAction.click({ timeout: 4000 }).catch(() => {}),
        ]);
        if (paneDownload) {
          download = paneDownload;
          log(`export pane download=${paneDownload.suggestedFilename()}`);
        }
      }
      if (download) {
        await download.saveAs(dest);
        log(`export saved ${dest}`);
      } else {
        const row = page.locator("[role='menu'] button, [role='menuitem']").filter({ hasText: /export/i }).first();
        if (await row.count()) {
          const [retry] = await Promise.all([
            page.waitForEvent("download", { timeout: 8000 }).catch(() => null),
            row.click({ timeout: 4000 }).catch(() => {}),
          ]);
          log(`export click retry=${retry ? retry.suggestedFilename() : "none"}`);
          if (retry) {
            await retry.saveAs(dest);
            log(`export saved ${dest}`);
          }
        }
      }
      verdict("export-only", Boolean(download), `download=${download ? download.suggestedFilename() : "none"} dest=${dest}`);
      await page.waitForTimeout(600);
      await dismissChrome();
      let [chooser] = await Promise.all([
        page.waitForEvent("filechooser", { timeout: 25000 }).catch(() => null),
        activateWorkspaceMenuOrdinal("5"),
      ]);
      const fileOpenFx = consoleBuf.filter((l) => /requestFileOpen|RequestFileOpen|openImportFixture|importFixture|unmapped effect|request-file-open|import-picker/.test(l)).slice(-24);
      log(`import chooser=${chooser ? "yes" : "none"} fileOpenFx=${JSON.stringify(fileOpenFx).slice(0, 1200)}`);
      if (!chooser) {
        const [late] = await Promise.all([
          page.waitForEvent("filechooser", { timeout: 8000 }).catch(() => null),
          page.waitForTimeout(200),
        ]);
        if (late) {
          chooser = late;
          log("import late chooser=yes");
        }
      }
      const fallback = join(OUT, "probe-2026-09-10T13-52-33-export.json");
      const { existsSync } = await import("node:fs");
      const feed = existsSync(dest) ? dest : fallback;
      const instancesBeforeSameFile = await dumpInstances();
      log(`import feed=${feed} instancesBefore=${JSON.stringify(instancesBeforeSameFile)}`);
      if (chooser) {
        await chooser.setFiles(feed);
        log("import setFiles fixture");
      } else {
        const row = page.locator("[role='menuitem'], button").filter({ hasText: /import/i }).first();
        if (await row.count()) await row.click({ timeout: 4000 }).catch(() => {});
        const execute = page.locator("button").filter({ hasText: /^execute$/i }).first();
        if (await execute.count()) {
          const payload = page.locator("textarea, [role='textbox']").first();
          if (await payload.count()) {
            const fs = await import("node:fs");
            if (fs.existsSync(dest)) await payload.fill(fs.readFileSync(dest, "utf8").slice(0, 200000));
          }
          await execute.click({ timeout: 4000 }).catch(() => {});
          log("import staged execute");
        }
      }
      await page.waitForTimeout(2000);
      await openHistory();
      log(`census after: ${JSON.stringify(await selectionState()).slice(0, 800)}`);
      log(`import hop-census: ${JSON.stringify(hopCensus())}`);
      log(`import picker hops=${JSON.stringify(consoleBuf.filter((l) => /\[DEBUG\] (import-picker|request-file-open mapped)/.test(l)).slice(-8)).slice(0, 1200)}`);
      {
        const sameFileAfter = await dumpInstances();
        log(`instances after import: ${JSON.stringify(sameFileAfter)}`);
        verdict("import-same-file-idempotent", sameFileAfter.count === instancesBeforeSameFile.count, `before=${instancesBeforeSameFile.count} after=${sameFileAfter.count}`);
      }
      log(`history after: ${JSON.stringify(await historyState()).slice(0, 800)}`);
      log(`same-file ingress hops=${JSON.stringify(consoleBuf.filter((l) => /\[DEBUG\] importFixture ingress/.test(l)).slice(-6)).slice(0, 1200)}`);
      log(`importexport after same-file: ${JSON.stringify(await chromeState()).slice(0, 1000)}`);
      const fs = await import("node:fs");
      const distinct = join(OUT, `probe-${stamp}-distinct.json`);
      let distinctReady = false;
      if (fs.existsSync(feed)) {
        const src = JSON.parse(fs.readFileSync(feed, "utf8")) as { objects?: Array<Record<string, unknown>>; meta?: Record<string, unknown> };
        const objects = Array.isArray(src.objects) ? src.objects : [];
        if (objects[0]) {
          const clone = JSON.parse(JSON.stringify(objects[0])) as Record<string, unknown>;
          clone.id = `probe-distinct-${stamp}`;
          clone.label = `Distinct ${String(clone.label ?? "import")}`;
          const origin = Array.isArray(clone.origin) ? clone.origin.map((n, i) => (i === 0 ? Number(n) + 4 : n)) : [4, 0, 0];
          clone.origin = origin;
          src.objects = [...objects, clone];
        } else {
          src.meta = { ...(src.meta ?? {}), probeDistinct: stamp };
        }
        fs.writeFileSync(distinct, JSON.stringify(src));
        distinctReady = true;
        log(`distinct fixture objects=${(src.objects ?? []).length} dest=${distinct}`);
      }
      const instancesBeforeDistinct = await dumpInstances();
      if (distinctReady) {
        await dismissChrome();
        let [chooser2] = await Promise.all([
          page.waitForEvent("filechooser", { timeout: 25000 }).catch(() => null),
          activateWorkspaceMenuOrdinal("5"),
        ]);
        if (!chooser2) {
          const [late2] = await Promise.all([
            page.waitForEvent("filechooser", { timeout: 8000 }).catch(() => null),
            page.waitForTimeout(200),
          ]);
          if (late2) chooser2 = late2;
        }
        log(`distinct chooser=${chooser2 ? "yes" : "none"}`);
        if (chooser2) {
          await chooser2.setFiles(distinct);
          log("distinct setFiles");
        } else {
          const row = page.locator("[role='menuitem'], button").filter({ hasText: /import/i }).first();
          if (await row.count()) await row.click({ timeout: 4000 }).catch(() => {});
          const execute = page.locator("button").filter({ hasText: /^execute$/i }).first();
          if (await execute.count()) {
            const payload = page.locator("textarea, [role='textbox']").first();
            if (await payload.count()) await payload.fill(fs.readFileSync(distinct, "utf8").slice(0, 200000));
            await execute.click({ timeout: 4000 }).catch(() => {});
            log("distinct staged execute");
          } else {
            log("distinct import path missed chooser and execute");
          }
        }
        await page.waitForTimeout(2000);
        await openHistory();
      }
      log(`distinct ingress hops=${JSON.stringify(consoleBuf.filter((l) => /\[DEBUG\] importFixture ingress/.test(l)).slice(-8)).slice(0, 1600)}`);
      {
        const afterDistinct = await dumpInstances();
        log(`distinct instances before=${JSON.stringify(instancesBeforeDistinct)} after=${JSON.stringify(afterDistinct)}`);
        verdict("import-distinct", afterDistinct.count !== instancesBeforeDistinct.count || JSON.stringify(afterDistinct.ids) !== JSON.stringify(instancesBeforeDistinct.ids), `before=${instancesBeforeDistinct.count} after=${afterDistinct.count}`, 41);
      }
      log(`distinct history after=${JSON.stringify(await historyState()).slice(0, 1200)}`);
      log(`distinct hop-census=${JSON.stringify(hopCensus())}`);
      log(`importexport after distinct: ${JSON.stringify(await chromeState()).slice(0, 1000)}`);
    });
  }
  /** 🪟️ Per-window census scoped to each window element — never the global `canvas` locator, which cannot
   * tell "two windows each painting their own view" from "one live window and one corpse". */
  const windowHostState = async () =>
    page.evaluate(
      (ids: string[]) =>
        ids.map((id) => {
          const root = (document.getElementById(id) ?? document.getElementById(`framework.window.${id}`)) as HTMLElement | null;
          const host = (root?.querySelector("[data-surface-id]") ?? null) as HTMLElement | null;
          const count = (raw: string | null | undefined) => {
            try {
              return JSON.parse(raw || "[]").length;
            } catch {
              return -1;
            }
          };
          return {
            id,
            present: Boolean(root),
            canvases: root ? root.querySelectorAll("canvas").length : 0,
            surface: host?.getAttribute("data-surface-id") ?? null,
            camera: host?.getAttribute("data-camera-json") ?? null,
            cameraAttr: Boolean(host?.hasAttribute("data-camera-json")),
            volumesAttr: Boolean(host?.hasAttribute("data-target-volumes-json")),
            instances: count(host?.getAttribute("data-instances-json")),
            vortices: count(host?.getAttribute("data-vortices-json")),
            volumes: count(host?.getAttribute("data-target-volumes-json")),
          };
        }),
      ["puzzle3d-main-top", "puzzle3d-main-perspective"],
    );
  const cameraOf = async (id: string) => (await windowHostState()).find((w) => w.id === id)?.camera ?? null;
  const targetVolumeCount = async () => (await windowHostState()).find((w) => w.id === "puzzle3d-main-perspective")?.volumes ?? -1;
  /** 🕰️ History entry count WITHOUT touching undo/redo — `openHistory()` deliberately clicks
   * `#framework.history.undo` to expand its sections, which would mutate the document from a read-only step. */
  const readHistoryEntryCount = async () => {
    const tab = page.locator('[data-slot="panel-tab-button"][id="framework.panel.history"], #framework.panel.history').first();
    if (await tab.count()) await tab.click({ timeout: 4000 }).catch(() => {});
    await page.waitForTimeout(700);
    return (await historyState()).entryCount;
  };
  /** 🗂️ Opens a panel tab by id or visible text, logging the whole tab inventory on a miss so the next reader
   * gets the real id instead of another guess. */
  const openPanel = async (match: RegExp) => {
    const tabs = await page.evaluate(() =>
      Array.from(document.querySelectorAll('[data-slot="panel-tab-button"]')).map((b) => ({ id: b.id, text: (b as HTMLElement).innerText.replace(/\n/g, " ").trim().slice(0, 40) })),
    );
    const hit = tabs.find((t) => match.test(t.id) || match.test(t.text));
    log(`openPanel ${match} hit=${JSON.stringify(hit)} tabs=${JSON.stringify(tabs).slice(0, 900)}`);
    if (!hit) return { opened: false, tabs, id: null as string | null };
    await page.locator(`[data-slot="panel-tab-button"][id="${hit.id}"]`).first().click({ force: true, timeout: 4000 }).catch(() => {});
    await page.waitForTimeout(1200);
    return { opened: true, tabs, id: hit.id };
  };
  /** 🎚️ Unfolds the perspective window's measures rail (§3/§4's entry point) and dumps every puzzle3d measure
   * id it exposes, so a missing control is reported as "absent from the rail", not as a silent skip. */
  const unfoldMeasures = async () => {
    await page.locator("canvas").last().click({ position: { x: 80, y: 80 }, timeout: 4000 }).catch(() => {});
    const unfold = page.locator('[id="framework.window.puzzle3dMainPerspective.measures.unfold"]').first();
    const found = await unfold.count();
    for (let i = 0; i < 3; i++) {
      const stillFolded = await page.locator('[id="framework.window.puzzle3dMainPerspective.measures.unfold"]').count();
      if (!stillFolded) break;
      await page.locator('[id="framework.window.puzzle3dMainPerspective.measures.unfold"]').first().click({ timeout: 4000 }).catch(() => {});
      await page.waitForTimeout(900);
      await page.evaluate(() => (document.getElementById("framework.window.puzzle3dMainPerspective.measures.unfold") as HTMLButtonElement | null)?.click());
      await page.waitForTimeout(900);
    }
    await page.waitForTimeout(1200);
    const dump = await page.evaluate(() => {
      const container = document.getElementById("framework.window.puzzle3dMainPerspective.measures");
      const measures = Array.from(document.querySelectorAll('[id^="puzzle3d-measure-"], [id^="puzzle3d-play-"], [id^="puzzle3d-voxel-"]'))
        .map((el) => `${el.id}|${el.getAttribute("data-slot") ?? el.tagName.toLowerCase()}|${el.getAttribute("aria-pressed") ?? (el as HTMLInputElement).value ?? ""}`)
        .slice(0, 90);
      return {
        measures,
        containerPresent: Boolean(container),
        containerText: (container as HTMLElement | null)?.innerText.replace(/\n/g, " | ").slice(0, 300) ?? null,
        containerIds: container ? Array.from(container.querySelectorAll("[id]")).map((el) => el.id).slice(0, 60) : [],
        railToggles: Array.from(document.querySelectorAll('[id^="framework.window.puzzle3dMainPerspective.measures"]')).map((el) => `${el.id}=${el.getAttribute("aria-expanded") ?? el.getAttribute("data-state") ?? ""}`),
        anyPuzzle3dIds: Array.from(document.querySelectorAll('[id*="puzzle3d"]')).map((el) => el.id).filter((id) => !id.startsWith("framework.window")).slice(0, 60),
      };
    });
    log(`measures rail unfoldControl=${found} ${JSON.stringify(dump)}`);
    return dump.measures;
  };
  /** 🎚️ One window-measure control's observable value — `aria-pressed` for toggles, `value` for sliders and
   * steppers, the trigger's own text for selects. */
  const readMeasure = async (id: string) =>
    page.evaluate((target: string) => {
      const el = document.getElementById(target) as HTMLElement | null;
      if (!el) return null;
      const input = el as HTMLInputElement;
      const thumb = (el.matches('[role="slider"]') ? el : el.querySelector('[role="slider"]')) as HTMLElement | null;
      const range = el.querySelector('input[type="range"]') as HTMLInputElement | null;
      return {
        tag: el.tagName.toLowerCase(),
        slot: el.getAttribute("data-slot"),
        role: el.getAttribute("role"),
        pressed: el.getAttribute("aria-pressed") ?? el.getAttribute("data-state"),
        checked: typeof input.checked === "boolean" ? input.checked : null,
        value: range?.value ?? thumb?.getAttribute("aria-valuenow") ?? (typeof input.value === "string" ? input.value : null),
        text: (el.innerText || "").replace(/\n/g, " ").trim().slice(0, 60),
      };
    }, id);
  /** 🎚️ Drives one measure the way a user would and returns before/after readings — sliders get keyboard
   * arrows (their thumb has no stable headless hit box), selects get their last option, toggles get a click. */
  const nudgeMeasure = async (id: string) => {
    const before = await readMeasure(id);
    if (!before) return { before: null, after: null };
    const loc = page.locator(`[id="${id}"]`).first();
    if (before.slot === "tree-action-checkbox" || (before.tag === "input" && before.checked !== null)) {
      await page.evaluate((target: string) => {
        const el = document.getElementById(target) as HTMLInputElement | null;
        if (!el) return;
        const label = (el.closest("label") ?? (el.id ? document.querySelector(`label[for="${CSS.escape(el.id)}"]`) : null)) as HTMLElement | null;
        (label ?? el).click();
      }, id);
    } else if (before.slot === "slider" || before.role === "slider") {
      const thumb = page.locator(`[id="${id}"] [role="slider"], [id="${id}"] input[type="range"]`).first();
      const handle = (await thumb.count()) ? thumb : loc;
      await handle.click({ force: true, timeout: 4000 }).catch(() => {});
      await handle.focus().catch(() => {});
      for (let i = 0; i < 5; i++) await page.keyboard.press("ArrowRight").catch(() => {});
    } else if (before.tag === "input") {
      await loc.focus().catch(() => {});
      for (let i = 0; i < 4; i++) await page.keyboard.press("ArrowRight").catch(() => {});
    } else if (before.role === "combobox" || before.slot === "select-trigger") {
      await loc.click({ force: true, timeout: 4000 }).catch(() => {});
      await page.waitForTimeout(500);
      const options = page.locator('[role="option"]');
      const n = await options.count();
      if (n > 1) await options.nth(n - 1).click({ timeout: 3000 }).catch(() => {});
      else await page.keyboard.press("Escape").catch(() => {});
    } else {
      await loc.click({ force: true, timeout: 4000 }).catch(() => {});
    }
    await page.waitForTimeout(1800);
    return { before, after: await readMeasure(id) };
  };
  /** 🧰️ Unfolds the utility bar and activates one utility by its literal id (`brush`, `transform`,
   * `volumeBrush`, `worldRelocate` — the `UTILITY_ID` const each `🪛️utilities` leaf declares). */
  const armUtility = async (utilityId: string) => {
    await unfoldPerspectiveUtilities();
    const loc = page.locator(`[id="${utilityId}"]`).first();
    const found = await loc.count();
    if (found) await loc.click({ force: true, timeout: 4000 }).catch(() => {});
    await page.waitForTimeout(1800);
    const active = (await dumpBrushPreview()).utility;
    log(`arm-utility ${utilityId} found=${found} active=${active ?? "null"}`);
    return { found: found > 0, active };
  };

  add("window-content", "§1", "read", process.argv.includes("--windows") || battery, async () => {
    const state = await windowHostState();
    log(`windows: ${JSON.stringify(state)}`);
    const top = state.find((w) => w.id === "puzzle3d-main-top");
    const persp = state.find((w) => w.id === "puzzle3d-main-perspective");
    verdict("window-both-present", Boolean(top?.present && persp?.present), `top=${top?.present} perspective=${persp?.present}`);
    verdict("window-one-canvas-each", top?.canvases === 1 && persp?.canvases === 1, `topCanvases=${top?.canvases} perspectiveCanvases=${persp?.canvases}`);
    verdict("window-same-document-both-views", (top?.instances ?? -1) > 0 && top?.instances === persp?.instances, `topInstances=${top?.instances} perspectiveInstances=${persp?.instances}`);
    verdict("window-distinct-camera", Boolean(top?.camera && persp?.camera && top.camera !== persp.camera), `top=${String(top?.camera).slice(0, 130)} perspective=${String(persp?.camera).slice(0, 130)}`);
    for (const w of state) {
      const loc = page.locator(`[id="${w.id}"], [id="framework.window.${w.id}"]`).first();
      if (await loc.count()) await loc.screenshot({ path: join(OUT, `probe-${stamp}-window-${w.id}.png`) }).catch(() => {});
    }
  });

  add("camera-gestures", "§2", "read", process.argv.includes("--camera") || battery, async () => {
    await dismissChrome();
    log("camera gestures: resolveWorldOrbitMouseButtonsIdle is {LEFT:null, MIDDLE:PAN, RIGHT:null} (🎨️r3f/🟦️.tsx:3255) — orbit is Alt+right-drag, pan is Shift+right-drag, zoom is the wheel");
    const canvas = page.locator('[id="puzzle3d-main-perspective"] canvas, [id="framework.window.puzzle3d-main-perspective"] canvas').first();
    const box = (await canvas.count()) ? await canvas.boundingBox() : await page.locator("canvas").last().boundingBox();
    if (!box) throw new Error("no perspective canvas box");
    const cx = box.x + box.width * 0.5;
    const cy = box.y + box.height * 0.35;
    const historyBefore = await readHistoryEntryCount();
    await dismissChrome();
    const initial = await windowHostState();
    const start = initial.find((w) => w.id === "puzzle3d-main-perspective")?.camera ?? null;
    const topBefore = initial.find((w) => w.id === "puzzle3d-main-top")?.camera ?? null;
    verdict("camera-json-attribute", Boolean(start), `data-camera-json=${String(start).slice(0, 170)}`);
    const drag = async (button: "left" | "middle" | "right", modifier: string | null, dx: number, dy: number) => {
      if (modifier) await page.keyboard.down(modifier).catch(() => {});
      await page.mouse.move(cx, cy);
      await page.mouse.down({ button });
      await page.mouse.move(cx + dx, cy + dy, { steps: 20 });
      await page.mouse.up({ button });
      if (modifier) await page.keyboard.up(modifier).catch(() => {});
      await page.waitForTimeout(1200);
      await dismissChrome();
      await page.waitForTimeout(500);
      return cameraOf("puzzle3d-main-perspective");
    };
    const afterOrbit = await drag("right", "Alt", 160, 70);
    verdict("camera-orbit", Boolean(afterOrbit) && afterOrbit !== start, `before=${String(start).slice(0, 110)} after=${String(afterOrbit).slice(0, 110)}`);
    const afterPan = await drag("right", "Shift", -130, 90);
    verdict("camera-pan", Boolean(afterPan) && afterPan !== afterOrbit, `before=${String(afterOrbit).slice(0, 110)} after=${String(afterPan).slice(0, 110)}`);
    await page.mouse.move(cx, cy);
    await page.mouse.wheel(0, -700);
    await page.waitForTimeout(1600);
    const afterZoom = await cameraOf("puzzle3d-main-perspective");
    verdict("camera-zoom", Boolean(afterZoom) && afterZoom !== afterPan, `before=${String(afterPan).slice(0, 110)} after=${String(afterZoom).slice(0, 110)}`);
    const alive = await snapshot().catch(() => null);
    verdict("camera-lane-responsive", Boolean(alive && alive.windows.length >= 2), `windows=${alive?.windows.length ?? "unreachable"}`);
    const historyAfter = await readHistoryEntryCount();
    verdict(
      "camera-emits-no-artifact-history",
      historyAfter === historyBefore,
      `historyEntries before=${historyBefore} after=${historyAfter} — checklist §2 declares setCamera emits no artifact mutations, but World3dHost's dispatchWorldCameraDebounced exists so "the shell-side command-history panel has something to show"; a non-zero delta needs the ticket to say which of the two is the contract`,
    );
    const topAfter = await cameraOf("puzzle3d-main-top");
    verdict("camera-per-window", topAfter === topBefore, `top before=${String(topBefore).slice(0, 110)} after=${String(topAfter).slice(0, 110)}`);
  });

  add("projection-options", "§3", "read", process.argv.includes("--projection") || battery, async () => {
    await dismissChrome();
    await unfoldMeasures();
    const ids = await page.evaluate(() => Array.from(document.querySelectorAll('[id^="puzzle3d-measure-projection"], [id="framework.worldOrbit.projection"]')).map((el) => el.id));
    log(`projection measure ids=${JSON.stringify(ids)}`);
    verdict("projection-measures-present", ids.length > 0, `ids=${JSON.stringify(ids).slice(0, 260)}`);
    if (!ids.length) return;
    const cameraBefore = await cameraOf("puzzle3d-main-perspective");
    const moved = await nudgeMeasure(ids[0]);
    log(`projection nudge ${ids[0]}: ${JSON.stringify(moved)}`);
    verdict("projection-control-flips", JSON.stringify(moved.before) !== JSON.stringify(moved.after), `id=${ids[0]} before=${JSON.stringify(moved.before)} after=${JSON.stringify(moved.after)}`);
    const cameraAfter = await cameraOf("puzzle3d-main-perspective");
    verdict("projection-repaints-camera", Boolean(cameraBefore) && cameraBefore !== cameraAfter, `before=${String(cameraBefore).slice(0, 100)} after=${String(cameraAfter).slice(0, 100)}`);
  });

  const WINDOW_OPTION_IDS = [
    "puzzle3d-play-grid-visible",
    "puzzle3d-play-grid-snap",
    "puzzle3d-play-grid-spacing",
    "puzzle3d-play-lod-auto",
    "puzzle3d-play-lod-value",
    "puzzle3d-play-vortex-show",
    "puzzle3d-play-vortex-direction",
    "puzzle3d-measure-sun-enabled",
  ];
  add("window-options", "§4", "read", process.argv.includes("--windowoptions") || battery, async () => {
    await dismissChrome();
    await unfoldMeasures();
    const historyBefore = await readHistoryEntryCount();
    await dismissChrome();
    await unfoldMeasures();
    let touched = 0;
    for (const id of WINDOW_OPTION_IDS) {
      let present = await readMeasure(id);
      if (!present) {
        await unfoldMeasures();
        present = await readMeasure(id);
      }
      if (!present) {
        verdict(`window-option-${id}`, false, "control absent from the measures rail even after re-unfolding it");
        continue;
      }
      const moved = await nudgeMeasure(id);
      touched += 1;
      verdict(`window-option-${id}`, JSON.stringify(moved.before) !== JSON.stringify(moved.after), `before=${JSON.stringify(moved.before)} after=${JSON.stringify(moved.after)}`);
      const alive = await snapshot().catch(() => null);
      if (!alive) {
        verdict("window-options-lane-responsive", false, `page stopped responding after ${id} — this is the checklist §4 WindowConfig hang`);
        return;
      }
    }
    verdict("window-options-lane-responsive", true, `touched=${touched} windows=${(await snapshot()).windows.length}`);
    const historyAfter = await readHistoryEntryCount();
    verdict("window-options-emit-no-history", historyAfter === historyBefore, `historyEntries before=${historyBefore} after=${historyAfter} (WindowConfig lane, not Artifact)`);
  });

  add("settings-panel", "§19", "read", process.argv.includes("--settings") || battery, async () => {
    await dismissChrome();
    const opened = await openPanel(/settings/i);
    verdict("settings-panel-opens", opened.opened, `tab=${opened.id} tabs=${JSON.stringify(opened.tabs).slice(0, 400)}`);
    if (!opened.opened) return;
    const steppers = await page.evaluate(() =>
      Array.from(document.querySelectorAll('[id^="puzzle3d-play-settings"]')).map((el) => ({ id: el.id, slot: el.getAttribute("data-slot"), value: (el as HTMLInputElement).value ?? null })),
    );
    log(`settings steppers=${JSON.stringify(steppers)}`);
    verdict("settings-steppers-present", steppers.length >= 4, `ids=${JSON.stringify(steppers.map((s) => s.id))}`);
    const targetId = "puzzle3d-play-settings.grid-spacing";
    const before = await readMeasure(targetId);
    const plus = page.locator(`[id="${targetId}"]`).locator("xpath=..").locator('[data-slot="stepper-plus"]').first();
    const plusCount = await plus.count().catch(() => 0);
    if (plusCount) await plus.click({ force: true, timeout: 4000 }).catch(() => {});
    else {
      await page.locator(`[id="${targetId}"]`).first().focus().catch(() => {});
      await page.keyboard.press("ArrowUp").catch(() => {});
    }
    await page.waitForTimeout(2500);
    const after = await readMeasure(targetId);
    verdict("settings-grid-spacing-bumps", Boolean(before) && JSON.stringify(before) !== JSON.stringify(after), `plusButtons=${plusCount} before=${JSON.stringify(before)} after=${JSON.stringify(after)}`);
    const railGrid = await readMeasure("puzzle3d-play-grid-spacing");
    verdict("settings-value-reaches-window-rail", Boolean(railGrid) && railGrid?.value === after?.value, `settings=${JSON.stringify(after)} windowRail=${JSON.stringify(railGrid)}`);
  });

  add("add-object-dialog", "§23", "read", process.argv.includes("--adddialog") || battery, async () => {
    await dismissChrome();
    const direct = page.locator('[id="shell-menu.action.openAddObjectDialog"], [data-menu-action="openAddObjectDialog"]').first();
    let trigger = direct;
    if (!(await direct.count())) {
      const byText = page.locator('button, [role="menuitem"]').filter({ hasText: /^\s*add object\s*$/i }).first();
      if (await byText.count()) trigger = byText;
      else {
        const c = page.locator("canvas").last();
        const box = await c.boundingBox();
        if (box) await page.mouse.click(box.x + 200, box.y + 160, { button: "right" });
        await page.waitForTimeout(800);
        trigger = page.locator('[data-menu-action="openAddObjectDialog"], [role="menu"] button, [role="menuitem"]').filter({ hasText: /add object/i }).first();
      }
    }
    const found = await trigger.count();
    const menuDump = await page.evaluate(() =>
      Array.from(document.querySelectorAll('[role="menu"] button, [role="menuitem"]')).map((b) => `${b.id || "?"}=${(b as HTMLElement).innerText.replace(/\n/g, " ").trim().slice(0, 34)}`).slice(0, 24),
    );
    log(`add-object trigger=${found} menu=${JSON.stringify(menuDump)}`);
    verdict("add-object-trigger-present", found > 0, `trigger=${found} menu=${JSON.stringify(menuDump).slice(0, 400)}`);
    if (!found) {
      await dismissChrome();
      return;
    }
    await trigger.click({ force: true, timeout: 4000 }).catch(() => {});
    await page.waitForTimeout(2500);
    const dialogs = await page.evaluate(() => Array.from(document.querySelectorAll('[role="dialog"]')).map((d) => (d as HTMLElement).innerText.replace(/\n/g, " | ").slice(0, 200)));
    verdict("add-object-dialog-opens", dialogs.length > 0, `dialogs=${JSON.stringify(dialogs).slice(0, 400)}`);
    const selectTrigger = page.locator('[role="dialog"] [role="combobox"], [role="dialog"] [data-slot="select-trigger"]').first();
    if (await selectTrigger.count()) {
      await selectTrigger.click({ force: true, timeout: 4000 }).catch(() => {});
      await page.waitForTimeout(800);
      const options = await page.evaluate(() => Array.from(document.querySelectorAll('[role="option"]')).map((o) => (o as HTMLElement).innerText.trim().slice(0, 40)));
      log(`add-object kind options=${JSON.stringify(options)}`);
      verdict("add-object-kind-options-are-dynamic", options.length > 1, `options=${JSON.stringify(options)} — checklist §23 predicts a single hardcoded "Object"`);
    } else verdict("add-object-kind-options-are-dynamic", false, "no kind select rendered inside the dialog");
    await page.keyboard.press("Escape").catch(() => {});
    await dismissChrome();
  });

  /** 🌍️ Reads the outliner's own section labels plus the grid/LOD rail labels — the DOM translation of
   * checklist §25's `document_json.contains("Baukomponenten")` assertion. */
  const readLocaleLabels = async () => {
    await openPanel(/document|artifact|outliner|puzzle3d-play-document/i);
    return page.evaluate(() => {
      const root = document.querySelector('[id^="puzzle3d-play-document"]') as HTMLElement | null;
      return {
        rootText: (root?.innerText || "").replace(/\n/g, " | ").slice(0, 400),
        rails: Array.from(document.querySelectorAll('[id^="puzzle3d-play-grid"], [id^="puzzle3d-play-lod"]'))
          .map((el) => `${el.id}=${(el as HTMLElement).innerText.replace(/\n/g, " ").trim().slice(0, 40)}`)
          .slice(0, 12),
      };
    });
  };
  add("locale-switch", "§25", "read", process.argv.includes("--locale") || battery, async () => {
    await dismissChrome();
    const english = await readLocaleLabels();
    log(`locale en labels=${JSON.stringify(english)}`);
    const setLanguage = async (label: RegExp) => {
      const opened = await openPanel(/settings/i);
      const trigger = page.locator('[id="framework.settings.language"]').last();
      const count = await trigger.count();
      if (!count) return { switched: false, tabs: opened.tabs };
      await trigger.click({ force: true, timeout: 4000 }).catch(() => {});
      await page.waitForTimeout(700);
      const option = page.locator('[role="option"]').filter({ hasText: label }).first();
      const optionCount = await option.count();
      if (optionCount) await option.click({ timeout: 3000 }).catch(() => {});
      else await page.keyboard.press("Escape").catch(() => {});
      await page.waitForTimeout(3000);
      return { switched: optionCount > 0, tabs: opened.tabs };
    };
    const toGerman = await setLanguage(/deutsch|german/i);
    verdict("locale-control-present", toGerman.switched, `switched=${toGerman.switched} tabs=${JSON.stringify(toGerman.tabs ?? []).slice(0, 320)}`);
    if (!toGerman.switched) return;
    const german = await readLocaleLabels();
    log(`locale de labels=${JSON.stringify(german)}`);
    verdict("locale-flips-document-labels", german.rootText.length > 0 && german.rootText !== english.rootText, `en=${english.rootText.slice(0, 170)} de=${german.rootText.slice(0, 170)}`);
    verdict("locale-de-document-section-label", /Baukomponenten/i.test(german.rootText), `de=${german.rootText.slice(0, 220)} — checklist §25 expects "Baukomponenten"`);
    verdict("locale-no-english-leak", german.rootText.length > 0 && !/\b(Objects|References|Attractions|Target Volumes)\b/.test(german.rootText), `de=${german.rootText.slice(0, 220)}`);
    const back = await setLanguage(/english|englisch/i);
    const restored = await readLocaleLabels();
    verdict("locale-switch-back-en", back.switched && restored.rootText === english.rootText, `switched=${back.switched} restored=${restored.rootText === english.rootText}`);
  });

  add("volume-brush", "§10", "mutate", process.argv.includes("--volume") || battery, async () => {
    await dismissChrome();
    const framed = await frameForestTableAfterCensus();
    const armed = await armUtility("volumeBrush");
    verdict("volume-brush-arm", armed.found && armed.active === "volumeBrush", `found=${armed.found} activeUtility=${armed.active}`);
    const before = await targetVolumeCount();
    verdict("volume-brush-target-volume-attribute", before >= 0, `data-target-volumes-json count=${before}`);
    const canvas = page.locator("canvas").last();
    await canvas.hover({ position: { x: framed.table.x, y: framed.table.y }, timeout: 4000 }).catch(() => {});
    await page.keyboard.down("Alt").catch(() => {});
    await canvas.click({ position: { x: framed.table.x, y: framed.table.y }, modifiers: ["Alt"], timeout: 4000, force: true }).catch(() => {});
    await page.keyboard.up("Alt").catch(() => {});
    await page.waitForTimeout(3000);
    const after = await targetVolumeCount();
    log(`volume-brush volumes before=${before} after=${after} instances=${JSON.stringify(await dumpInstances())}`);
    verdict("volume-brush-add-target-volume", after > before, `before=${before} after=${after}`);
    const voxel = await nudgeMeasure("puzzle3d-voxel-w");
    verdict("volume-brush-voxel-dims", Boolean(voxel.before) && JSON.stringify(voxel.before) !== JSON.stringify(voxel.after), `before=${JSON.stringify(voxel.before)} after=${JSON.stringify(voxel.after)}`);
  });

  add("relocate", "§11", "mutate", process.argv.includes("--relocate") || battery, async () => {
    await dismissChrome();
    const framed = await frameForestTableAfterCensus();
    await clickForestTable();
    await page.locator("canvas").last().click({ position: { x: framed.table.x, y: framed.table.y }, timeout: 5000 }).catch(() => {});
    await page.waitForTimeout(1200);
    const armed = await armUtility("worldRelocate");
    verdict("relocate-arm", armed.found, `found=${armed.found} activeUtility=${armed.active}`);
    const hardBefore = hardFaults.length;
    const readPose = async () => page.evaluate(() => document.querySelector("#puzzle3d-main-perspective [data-instances-json]")?.getAttribute("data-instances-json") ?? "");
    const poseBefore = await readPose();
    const box = framed.box;
    await page.mouse.move(box.x + framed.table.x, box.y + framed.table.y);
    await page.mouse.down();
    await page.mouse.move(box.x + framed.table.x - 130, box.y + framed.table.y - 70, { steps: 24 });
    await page.mouse.up();
    await page.waitForTimeout(3500);
    const poseAfter = await readPose();
    const instances = await dumpInstances();
    log(`relocate poseBeforeLen=${poseBefore.length} poseAfterLen=${poseAfter.length} instances=${JSON.stringify(instances)}`);
    verdict("relocate-pose-delta", poseAfter.length > 0 && poseBefore !== poseAfter, `beforeLen=${poseBefore.length} afterLen=${poseAfter.length} instances=${instances.count}`);
    verdict(
      "relocate-no-hard-fault",
      hardFaults.length === hardBefore,
      `newHardFaults=${hardFaults.length - hardBefore} instances=${instances.count} newest=${(hardFaults[hardFaults.length - 1] ?? "none").slice(0, 180)} — checklist §11 expects an extent fault above 62 objects`,
    );
  });

  add("engagement-bar", "§14", "mutate", process.argv.includes("--engagement") || battery, async () => {
    await dismissChrome();
    const toggle = page.locator('[id="framework.window.puzzle3dMainPerspective.engagement.toggle"]').first();
    const toggleCount = await toggle.count();
    if (toggleCount) await toggle.click({ force: true, timeout: 4000 }).catch(() => {});
    await page.waitForTimeout(1200);
    const input = page.locator('input[placeholder*="fill" i], input[placeholder*="brush" i], [role="textbox"][placeholder*="fill" i]').first();
    const inputCount = await input.count();
    const placeholder = inputCount ? await input.getAttribute("placeholder") : null;
    log(`engagement toggle=${toggleCount} input=${inputCount} placeholder=${placeholder}`);
    verdict("engagement-input-present", inputCount > 0, `toggle=${toggleCount} input=${inputCount} placeholder=${placeholder}`);
    if (!inputCount) return;
    verdict("engagement-placeholder-has-no-dead-verbs", !/clear|rectangle|lasso/i.test(placeholder ?? ""), `placeholder=${placeholder} — checklist §14: clear/rectangle/lasso were dropped but stayed advertised`);
    const submit = async (text: string) => {
      await input.fill(text).catch(() => {});
      await page.keyboard.press("Enter").catch(() => {});
      await page.waitForTimeout(3000);
    };
    await submit("brush");
    const brushPreview = await dumpBrushPreview();
    verdict("engagement-brush-verb", brushPreview.utility === "brush", `activeUtility=${brushPreview.utility}`);
    const beforeClear = JSON.stringify(await dumpInstances());
    await submit("clear");
    const afterClear = JSON.stringify(await dumpInstances());
    verdict("engagement-clear-is-a-noop", afterClear === beforeClear, `before=${beforeClear} after=${afterClear}`);
    await submit("fill 5");
    const fillPressed = await page.evaluate(() => document.getElementById("tool.fill")?.getAttribute("aria-pressed") ?? null);
    verdict("engagement-fill-verb", fillPressed === "true", `#tool.fill aria-pressed=${fillPressed}`);
    await page.keyboard.press("Escape").catch(() => {});
    await page.waitForTimeout(2000);
    const aborted = await dumpBrushPreview();
    verdict("engagement-abort", aborted.utility !== "brush", `activeUtility=${aborted.utility}`);
    await dismissChrome();
  });

  add("context-menu-rows", "§15", "mutate", process.argv.includes("--contextmenu") || battery, async () => {
    await dismissChrome();
    const framed = await frameForestTableAfterCensus();
    await clickForestTable();
    const canvas = page.locator("canvas").last();
    await canvas.click({ position: { x: framed.table.x, y: framed.table.y }, timeout: 5000 }).catch(() => {});
    await page.waitForTimeout(1500);
    await canvas.click({ position: { x: framed.table.x, y: framed.table.y }, button: "right", timeout: 5000 }).catch(() => {});
    await page.waitForTimeout(1500);
    const rows = await page.evaluate(() =>
      Array.from(document.querySelectorAll('[role="menu"] button, [role="menuitem"]')).map((el) => ({
        id: el.id || null,
        action: el.getAttribute("data-menu-action"),
        text: (el as HTMLElement).innerText.replace(/\n/g, " ").trim().slice(0, 40),
      })),
    );
    log(`context-menu rows=${JSON.stringify(rows).slice(0, 1400)}`);
    verdict("context-menu-opens", rows.length > 0, `rows=${rows.length}`);
    const ids = new Set(rows.map((r) => r.id));
    const missing = ["duplicate", "select-same-kind", "zoom", "delete", "hide-show", "lock-unlock"].filter((id) => !ids.has(id));
    verdict("context-menu-object-vocabulary", rows.length > 0 && missing.length === 0, `missing=${JSON.stringify(missing)} present=${JSON.stringify([...ids])}`);
    const zoom = rows.find((r) => r.id === "zoom");
    verdict("context-menu-zoom-row-action-is-registered", Boolean(zoom) && zoom?.action !== "zoomToSelection", `zoom row action=${zoom?.action ?? "row absent"} — checklist §15 predicts the unregistered "zoomToSelection"`);
    if (zoom) {
      const hardBefore = hardFaults.length;
      const faultsBefore = faults.length;
      const cameraBefore = await cameraOf("puzzle3d-main-perspective");
      await page.locator('[id="zoom"]').last().click({ force: true, timeout: 4000 }).catch(() => {});
      await page.waitForTimeout(3000);
      const cameraAfter = await cameraOf("puzzle3d-main-perspective");
      verdict(
        "context-menu-zoom-moves-camera",
        Boolean(cameraBefore) && cameraBefore !== cameraAfter,
        `before=${String(cameraBefore).slice(0, 100)} after=${String(cameraAfter).slice(0, 100)} newFaults=${faults.length - faultsBefore} newHardFaults=${hardFaults.length - hardBefore}`,
      );
    }
    await dismissChrome();
  });

  add("outliner-rows", "§17", "mutate", process.argv.includes("--outliner") || battery, async () => {
    await dismissChrome();
    const opened = await openPanel(/document|artifact|outliner|puzzle3d-play-document/i);
    verdict("outliner-panel-opens", opened.opened, `tab=${opened.id} tabs=${JSON.stringify(opened.tabs).slice(0, 400)}`);
    if (!opened.opened) return;
    const rowDump = async () =>
      page.evaluate(() =>
        Array.from(document.querySelectorAll('[data-slot="tree-item"], [role="treeitem"]'))
          .map((r) => ({ id: r.id || null, text: (r as HTMLElement).innerText.replace(/\n/g, " ").trim().slice(0, 80) }))
          .slice(0, 40),
      );
    const before = await rowDump();
    log(`outliner rows before=${JSON.stringify(before).slice(0, 1400)}`);
    const hideButton = page.locator('[role="treeitem"] button, [data-slot="tree-item"] button').filter({ hasText: /^(hide|verbergen)$/i }).first();
    const hideByLabel = page.locator('[role="treeitem"] [aria-label*="Hide" i], [data-slot="tree-item"] [title*="Hide" i]').first();
    const hideCount = await hideButton.count();
    const labelCount = await hideByLabel.count();
    const target = hideCount ? hideButton : hideByLabel;
    verdict("outliner-hide-control-present", hideCount + labelCount > 0, `byText=${hideCount} byLabel=${labelCount}`);
    if (!hideCount && !labelCount) return;
    await target.click({ force: true, timeout: 4000 }).catch(() => {});
    await page.waitForTimeout(3000);
    const afterHide = await rowDump();
    verdict("outliner-hide-applies", JSON.stringify(afterHide) !== JSON.stringify(before), `beforeHead=${JSON.stringify(before.slice(0, 3))} afterHead=${JSON.stringify(afterHide.slice(0, 3))}`);
    const showButton = page.locator('[role="treeitem"] button, [data-slot="tree-item"] button').filter({ hasText: /^(show|anzeigen|einblenden)$/i }).first();
    const showCount = await showButton.count();
    log(`outliner show controls=${showCount}`);
    if (!showCount) {
      verdict("outliner-show-restores", false, "no Show row action rendered after hiding");
      return;
    }
    await showButton.click({ force: true, timeout: 4000 }).catch(() => {});
    await page.waitForTimeout(3000);
    const afterShow = await rowDump();
    verdict(
      "outliner-show-restores",
      JSON.stringify(afterShow) === JSON.stringify(before),
      `restored=${JSON.stringify(afterShow) === JSON.stringify(before)} afterShowHead=${JSON.stringify(afterShow.slice(0, 3))} — checklist §17 predicts flag_args hardcodes value:true so Show is a no-op`,
    );
  });

  add("catalogue-panel", "§18", "mutate", process.argv.includes("--catalogue") || battery, async () => {
    await dismissChrome();
    const opened = await openPanel(/kinds|catalogue|catalog|bauteil/i);
    verdict("catalogue-panel-opens", opened.opened, `tab=${opened.id} tabs=${JSON.stringify(opened.tabs).slice(0, 400)}`);
    if (!opened.opened) return;
    const beforeAdd = await dumpInstances();
    const row = page.locator('[id^="puzzle3d-play-kinds"] [role="treeitem"], [id^="puzzle3d-play-kinds"] [data-slot="tree-item"]').first();
    const rowCount = await row.count();
    verdict("catalogue-kind-rows-present", rowCount > 0, `rows=${rowCount}`);
    if (rowCount) {
      await row.click({ force: true, timeout: 4000 }).catch(() => {});
      await page.waitForTimeout(3500);
      const afterAdd = await dumpInstances();
      verdict("catalogue-add-object-kind", afterAdd.count > beforeAdd.count, `before=${beforeAdd.count} after=${afterAdd.count}`);
      const sel = await selectionState();
      verdict("catalogue-add-selects-new-object", sel.selected.length > 0, `selected=${JSON.stringify(sel.selected).slice(0, 220)}`);
    }
    const beforeDrop = await dumpInstances();
    const dropped = await page.evaluate(() => {
      const source = document.querySelector('[id^="puzzle3d-play-kinds"] [role="treeitem"], [id^="puzzle3d-play-kinds"] [data-slot="tree-item"]') as HTMLElement | null;
      const canvas = document.querySelector("#puzzle3d-main-perspective canvas") as HTMLElement | null;
      const host = document.querySelector("#puzzle3d-main-perspective [data-surface-id]") as HTMLElement | null;
      if (!source || !canvas || !host) return { ran: false, payload: null as string | null };
      const payload = source.getAttribute("data-drag-payload") ?? source.id ?? "";
      const transfer = new DataTransfer();
      transfer.setData("application/x-semio-catalogue-item", payload);
      const rect = canvas.getBoundingClientRect();
      const clientX = Math.round(rect.x + rect.width / 2);
      const clientY = Math.round(rect.y + rect.height / 2);
      source.dispatchEvent(new DragEvent("dragstart", { bubbles: true, cancelable: true, dataTransfer: transfer }));
      for (const type of ["dragenter", "dragover", "drop"]) host.dispatchEvent(new DragEvent(type, { bubbles: true, cancelable: true, dataTransfer: transfer, clientX, clientY }));
      source.dispatchEvent(new DragEvent("dragend", { bubbles: true, dataTransfer: transfer }));
      return { ran: true, payload: payload.slice(0, 120) };
    });
    await page.waitForTimeout(3500);
    const afterDrop = await dumpInstances();
    log(`catalogue drag-drop ${JSON.stringify(dropped)} before=${JSON.stringify(beforeDrop)} after=${JSON.stringify(afterDrop)}`);
    verdict("catalogue-drag-drop", dropped.ran && afterDrop.count > beforeDrop.count, `ran=${dropped.ran} payload=${dropped.payload} before=${beforeDrop.count} after=${afterDrop.count}`);
  });

  add("selection-keybindings", "§22", "mutate", process.argv.includes("--keys") || battery, async () => {
    await dismissChrome();
    const framed = await frameForestTableAfterCensus();
    const canvas = page.locator("canvas").last();
    const select = async () => {
      await clickForestTable();
      await canvas.click({ position: { x: framed.table.x, y: framed.table.y }, timeout: 5000 }).catch(() => {});
      await page.waitForTimeout(1500);
    };
    await select();
    const beforeDelete = await dumpInstances();
    await page.keyboard.press("Delete").catch(() => {});
    await page.waitForTimeout(3000);
    let afterDelete = await dumpInstances();
    if (afterDelete.count === beforeDelete.count) {
      await page.keyboard.press("Backspace").catch(() => {});
      await page.waitForTimeout(3000);
      afterDelete = await dumpInstances();
    }
    verdict("delete-selection", afterDelete.count < beforeDelete.count, `before=${beforeDelete.count} after=${afterDelete.count}`);
    await select();
    const beforeDup = await dumpInstances();
    await page.keyboard.press("Meta+d").catch(() => {});
    await page.waitForTimeout(3000);
    let afterDup = await dumpInstances();
    if (afterDup.count === beforeDup.count) {
      await page.keyboard.press("Control+d").catch(() => {});
      await page.waitForTimeout(3000);
      afterDup = await dumpInstances();
    }
    verdict("duplicate-selection", afterDup.count > beforeDup.count, `before=${beforeDup.count} after=${afterDup.count}`);
    const sel = await selectionState();
    verdict("duplicate-reselects-clone", afterDup.count > beforeDup.count && sel.selected.length > 0, `selected=${JSON.stringify(sel.selected).slice(0, 220)}`);
    await select();
    const cameraBefore = await cameraOf("puzzle3d-main-perspective");
    await page.keyboard.press("f").catch(() => {});
    await page.waitForTimeout(3000);
    const cameraAfter = await cameraOf("puzzle3d-main-perspective");
    verdict("focus-selection", Boolean(cameraBefore) && cameraBefore !== cameraAfter, `before=${String(cameraBefore).slice(0, 100)} after=${String(cameraAfter).slice(0, 100)}`);
  });

  log(`plan: ${JSON.stringify(GROUP_ORDER.map((group) => ({ group, steps: plan.filter((entry) => entry.group === group).map((entry) => entry.name) })))}`);
  let ranGroup = false;
  for (const group of GROUP_ORDER) {
    const entries = plan.filter((entry) => entry.group === group);
    if (!entries.length) continue;
    if (ranGroup && reloadBetweenGroups) {
      log(`reloading page before group ${group}`);
      await page.goto(`http://127.0.0.1:${port}/?plugin=puzzle3d`, { waitUntil: "domcontentloaded", timeout: 60000 }).catch(() => {});
      const rebooted = await waitForBoot(`reboot:${group}`, 40);
      currentStep = `reboot-${group}`;
      currentSection = "§0";
      verdict(`reboot-${group}`, rebooted, `group=${group}`);
    }
    ranGroup = true;
    for (const entry of entries) {
      currentStep = entry.name;
      currentSection = entry.section;
      await step(entry.name, entry.run);
    }
  }
  currentStep = "battery";
  currentSection = "§0";
  verdict("battery-hard-faults", hardFaults.length === 0, `hard=${hardFaults.length} collateral=${collateralFaults.length} distinct=${faultKeys.size} first=${(hardFaults[0] ?? "none").slice(0, 200)}`);
  verdict("battery-faults", faults.length === 0, `raw=${faults.length} hard=${hardFaults.length} collateral=${collateralFaults.length} distinct=${faultKeys.size}`);
  const summaryLine = `battery PASS=${passCount} FAIL=${failCount} FAULTS=${faults.length}`;
  emit({
    t: Number(((Date.now() - t0) / 1000).toFixed(1)),
    ts: new Date().toISOString(),
    summary: summaryLine,
    pass: passCount,
    fail: failCount,
    faults: faults.length,
    hardFaults: hardFaults.length,
    collateralFaults: collateralFaults.length,
    distinctFaults: faultKeys.size,
    steps: plan.map((entry) => `${entry.group}:${entry.name}`),
  });
  log(summaryLine);
  log(`battery summary count=${verdicts.length}`);
  for (const row of verdicts) log(`battery ${row}`);
}

const tail = consoleBuf.slice(-1200).join("\n");
const verdictBlock = lines.filter((row) => row.includes("verdict ")).join("\n");
writeFileSync(
  join(OUT, `probe-${stamp}.md`),
  `# probe ${stamp} (interact=${interact} battery=${battery} only=${onlyArg ?? "-"} reloadBetweenGroups=${reloadBetweenGroups})\n\n## verdicts\n${verdictBlock || "(none)"}\n\n## timeline\n${lines.join("\n")}\n\n## faults (raw ${faults.length}, hard ${hardFaults.length}, collateral ${collateralFaults.length}, distinct ${faultKeys.size})\n\n### hard\n${hardFaults.slice(0, 60).join("\n") || "(none)"}\n\n### collateral\n${collateralFaults.slice(0, 60).join("\n") || "(none)"}\n\n## console tail\n\`\`\`\n${tail}\n\`\`\`\n`,
);
log(`done booted=${booted} faults=${faults.length} hard=${hardFaults.length} collateral=${collateralFaults.length} verdicts=${lines.filter((row) => row.includes("verdict ")).length} → 🗑️generated/probe-${stamp}.md + probe-${stamp}.ndjson`);
await browser.close();
process.exit(0);
