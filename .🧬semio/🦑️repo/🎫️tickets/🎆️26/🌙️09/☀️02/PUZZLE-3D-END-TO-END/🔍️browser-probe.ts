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
 * plus a final `battery PASS=n FAIL=n FAULTS=n first-hard-fault-at=<s> guest-death-faults=n` summary
 * record). Every group closes with a `guest-alive-<group>` verdict read off `data-plugin-recovery` and the
 * world surfaces' `data-instances-json`, so a battery that measured a corpse says so at the group boundary
 * rather than as a run of unexplained late FAILs. */
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
/** ☠️ The guest-death family B13 decoded out of `shard 0 worker fault [handler/turn] actor=puzzle#1`
 * (`📓️2026-09-11-wave-B13-export-history-locale.md` §5): the reactor-close trap itself, the
 * `registerBrushMesh` retry storm that precedes it, and the `Agent disconnected` banner the shell prints
 * once the handle is gone. Every one of these means the actor behind the world surfaces is dead, so every
 * later step measures a corpse — they are HARD, never collateral, wherever they appear in the console. */
const GUEST_DEATH_RE = /reactor-close-authority|native close terminal unavailable|actor-activation\.revoked|Agent disconnected/i;
const FAULT_RE =
  /intake-budget-exhausted|fixed-capacity|section-root-mismatch|native-owner-required|terminal-fault|unreachable|shard .* (lost|terminated)|did not publish|missing field|malformed|admission failed|worker fault|\[semio-plugin panic\]|Credits \{|NodeCapacity|SemioFaultError|reactor-close-authority|native close terminal unavailable|actor-activation\.revoked|Agent disconnected/i;
/** 💥️ The subset of {@link FAULT_RE} that means the runtime itself broke — a guest trap, a dead worker, a
 * plugin panic or a typed fault envelope. Everything else FAULT_RE matches (capacity notices, "did not
 * publish", "malformed", a repeat of an earlier storm) is COLLATERAL: real, worth counting, but never
 * independent evidence that the step under test failed. */
const HARD_FAULT_RE =
  /worker fault|\bunreachable\b|\[semio-plugin panic\]|panicked at|SemioFaultError|terminal-fault|admission failed|shard .* (lost|terminated)|native-owner-required|reactor-close-authority|native close terminal unavailable|actor-activation\.revoked|Agent disconnected/i;
const hardFaults: string[] = [];
const collateralFaults: string[] = [];
const guestDeathFaults: string[] = [];
const faultKeys = new Set<string>();
/** ⏱️ Seconds into the run at which the FIRST hard fault landed, `null` while none has. The summary prints
 * it so "the battery measured a dead guest from step three onward" is visible without reading the log. */
let firstHardFaultAt: number | null = null;
/** 💥️ Records one console line that matched {@link FAULT_RE}, classified and deduplicated by its first 60
 * characters so "one fault repeated 200×" stays distinguishable from "200 distinct faults". */
const noteFault = (text: string) => {
  if (faults.length < 200) faults.push(text);
  faultKeys.add(text.slice(0, 60));
  const hard = HARD_FAULT_RE.test(text);
  const bucket = hard ? hardFaults : collateralFaults;
  if (bucket.length < 200) bucket.push(text);
  if (hard && firstHardFaultAt === null) firstHardFaultAt = Number(((Date.now() - t0) / 1000).toFixed(1));
  if (GUEST_DEATH_RE.test(text) && guestDeathFaults.length < 200) guestDeathFaults.push(text);
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

/** 🌊️ The Playwright errors a `page.evaluate` raises when the page navigated out from under it —
 * `--reload-between-groups` crashed battery #46b on exactly this, because the reload and the next
 * `snapshot()` raced at "reloading page before group mutate". */
const NAVIGATION_RACE_RE = /Execution context was destroyed|Most likely the page has been closed|frame was detached|Cannot find context with specified id|navigation/i;
/** 🧭️ Runs one `page.evaluate` through a navigation-safe retry: on a navigation race it waits for the new
 * document's `domcontentloaded` and evaluates once more, and only then falls back. Every boot, snapshot and
 * poll helper reads the page through this, so a reload can never take the run down with it. */
const evalSafe = async <T>(body: () => T, fallback: T): Promise<T> => {
  for (let attempt = 0; attempt < 2; attempt++) {
    try {
      return await page.evaluate(body);
    } catch (error) {
      if (!NAVIGATION_RACE_RE.test(String(error))) throw error;
      log(`evaluate raced a navigation (attempt ${attempt + 1}) — awaiting domcontentloaded`);
      await page.waitForLoadState("domcontentloaded").catch(() => {});
      await page.waitForTimeout(750);
    }
  }
  return fallback;
};
/** 🔢️ Navigation-safe `locator.count()` — a reload mid-poll rejects the same way `page.evaluate` does. */
const countSafe = async (locator: ReturnType<typeof page.locator>): Promise<number> => {
  try {
    return await locator.count();
  } catch (error) {
    if (!NAVIGATION_RACE_RE.test(String(error))) throw error;
    await page.waitForLoadState("domcontentloaded").catch(() => {});
    return 0;
  }
};
/** 🔁️ The one navigation this probe performs: a cold load of the puzzle3d serve, awaited to
 * `domcontentloaded` on BOTH the goto and the settled load state before any evaluate runs against it. */
const gotoShell = async (label: string) => {
  await page.goto(`http://127.0.0.1:${port}/?plugin=puzzle3d`, { waitUntil: "domcontentloaded", timeout: 60000 }).catch((error) => log(`${label} goto: ${String(error).slice(0, 160)}`));
  await page.waitForLoadState("domcontentloaded").catch(() => {});
};

log("navigating");
await gotoShell("boot");

const EMPTY_SNAPSHOT = { windows: [] as { id: string; w: number; h: number }[], canvases: 0, tabs: [] as (string | null)[], toggles: [] as string[], treeItems: 0, dialogs: [] as string[], body: "" };
const snapshot = async () =>
  evalSafe(() => {
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
  }, EMPTY_SNAPSHOT);

/** 🚀️ Polls until two windows with two canvases exist, dismissing the intro dialog and the welcome tour on
 * the way. Reused verbatim by `--reload-between-groups` so a regrouped run boots exactly like a cold one. */
const waitForBoot = async (label: string, polls = 60) => {
  let ready = false;
  for (let i = 0; i < polls; i++) {
    await page.waitForTimeout(3000);
    const s = await snapshot();
    if (s.dialogs.length && i % 3 === 0) {
      const skip = page.locator('[role="dialog"] button', { hasText: /skip/i }).first();
      if (await countSafe(skip)) {
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
    if (await countSafe(skip)) {
      await skip.click({ timeout: 3000 }).catch(() => {});
      await page.waitForTimeout(1500);
      log(`${label} dismissed welcome tour`);
    }
  }
  return ready;
};

/** 🫀️ The cheapest DOM proof that the guest actor behind the world surfaces is still alive, read straight
 * off the two attributes the host already publishes: `ChromePanels`' `data-plugin-recovery` (the
 * "This program crashed." card the shell swaps a dead program's body for) and `World3dHost`'s
 * `data-instances-json` / `data-status-json` (`🌐️World3dHost/🟦️.tsx` — the scene payload stops arriving the
 * moment the actor traps). No round trip to the guest, so it costs nothing when the guest is already gone. */
const guestVitals = async () =>
  evalSafe(
    () => {
      const q = (sel: string) => Array.from(document.querySelectorAll(sel));
      return {
        recovery: q("[data-plugin-recovery]").map((el) => el.getAttribute("data-plugin-recovery") ?? "?"),
        surfaces: q("[data-instances-json]").map((el) => ({
          surface: el.getAttribute("data-surface-id") ?? "?",
          instances: (() => {
            try {
              return (JSON.parse(el.getAttribute("data-instances-json") || "[]") as unknown[]).length;
            } catch {
              return -1;
            }
          })(),
          status: (el.getAttribute("data-status-json") || "").slice(0, 80),
        })),
        canvases: q("canvas").length,
      };
    },
    { recovery: ["evaluate-unavailable"], surfaces: [] as { surface: string; instances: number; status: string }[], canvases: 0 },
  );

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
  // 🧬️ The reserved-verb family — `--only=` counts as asking for it. `family`/`wantUndo` used to read
  // argv ALONE, so `--only=example-switch,history-open,undo-unwind,undo-redo` registered the undo steps
  // (`--only` overrides every gate) while leaving `wantUndo` FALSE: `example-switch` then picked
  // `options.nth(1)` instead of Nakagin, emitted no `example-switch` verdict, `undo-unwind` passed
  // vacuously against the document it never left, and `undo-redo` demanded a Nakagin that was never
  // loaded. A lane and its flag must select the same behaviour or a lane's greens mean nothing.
  const family = process.argv.includes("--reserved-family") || battery || (only?.has("undo-unwind") ?? false) || (only?.has("undo-redo") ?? false) || (only?.has("undo-once") ?? false);
  const exampleArg = process.argv.find((a) => a.startsWith("--example="))?.slice(10);
  const wantUndo = process.argv.includes("--undo") || family;
  const wantExample = !!exampleArg || wantUndo || (only?.has("example-switch") ?? false);
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
      guestDeathFaults: guestDeathFaults.length,
      firstHardFaultAt,
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
    // 🎨️ THE example picker, by its own id (`NavbarExampleSelect id="playground.navbar.fixture"`,
    // `🏛️ShellHost/🟦️.tsx`) — never `select`/`[role="combobox"]` `.first()`. The history panel the
    // `history-open` step opens right before this one carries its own command-filter select, and a bare
    // `page.locator("select").first()` reached THAT: battery #48's lane changed the history filter,
    // never dispatched `setActiveExample`, and still scored `undo-unwind` green on a document that had
    // never moved.
    const picker = page.locator('[id="playground.navbar.fixture"]');
    const native = picker.locator("select").or(page.locator('select[id="playground.navbar.fixture"]')).first();
    const wanted = exampleArg ? new RegExp(exampleArg, "i") : /nakagin/i;
    if (await countSafe(native)) {
      await native.selectOption({ label: (await native.locator("option").allTextContents()).find((label) => wanted.test(label)) ?? "" }).catch(() => {});
    } else {
      const combo = (await countSafe(picker)) ? picker.first() : page.locator('[role="combobox"]').first();
      await combo.click({ timeout: 3000 }).catch(() => {});
      await page.waitForTimeout(400);
      const options = page.locator('[role="option"]');
      log(`example options=${JSON.stringify(await options.allTextContents()).slice(0, 300)} pickerPresent=${await countSafe(picker)}`);
      const target = options.filter({ hasText: wanted }).first();
      await ((await countSafe(target)) ? target : options.nth(1)).click({ timeout: 3000 }).catch(() => {});
    }
    const settleArg = process.argv.find((a) => a.startsWith("--settle="))?.slice(9);
    await page.waitForTimeout(settleArg ? Number(settleArg) * 1000 : 20000);
    const readExample = async () =>
      evalSafe(() => {
        const picker = (document.getElementById("playground.navbar.fixture") ?? document.querySelector('[role="combobox"]')) as HTMLElement | null;
        return (picker?.innerText || "").replace(/\n/g, " ").slice(0, 80);
      }, "");
    let afterExample = await readExample();
    log(`example after switch: ${afterExample}`);
    if (wantUndo) verdict("example-switch", /nakagin/i.test(afterExample), `example=${afterExample}`);
    if (wantUndo && !/nakagin/i.test(afterExample)) {
      const combo = ((await countSafe(page.locator('[id="playground.navbar.fixture"]'))) ? page.locator('[id="playground.navbar.fixture"]') : page.locator('[role="combobox"]')).first();
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
      // 🎨️ THE example picker by its own id (`NavbarExampleSelect id="playground.navbar.fixture"`), with
      // the first combobox only as a fallback: the history panel carries a command-filter combobox of
      // its own, so whichever of the two the DOM ordered first decided what a verdict called "the
      // example".
      const combo = (document.getElementById("playground.navbar.fixture") ?? document.querySelector('[role="combobox"]')) as HTMLElement | null;
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
    evalSafe(
      () => {
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
      },
      { count: 0, ids: [] as string[] },
    );
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
  /** 🗂️ The Actions pane of one window instance: its root text, the file-category action rows it
   * carries and whether the pane is folded. `export-only`'s old route (right-click the canvas, then
   * `[data-menu-action]`) can never work — `World3dHost.onContextMenu` calls `preventDefault()`
   * synchronously, which is exactly how an inner surface CLAIMS the right-click, so `ShellHost`'s
   * fallback menu (the only thing that ever renders `shell-menu.action.exportFixture`) is suppressed
   * by design, and the guest's own viewport menu carries no file verbs. */
  const actionPaneState = async () =>
    evalSafe(
      () => {
        const root = document.getElementById("framework.window.puzzle3dMainPerspective.engagement");
        const rows = Array.from(document.querySelectorAll<HTMLElement>('[id^="action."]')).map((row) => `${row.id}|${row.innerText.replace(/\s+/g, " ").trim().slice(0, 28)}`);
        return { rootPresent: Boolean(root), folded: root?.getAttribute("data-folded") ?? null, rootText: root?.innerText.replace(/\n/g, " | ").slice(0, 200) ?? null, rows: rows.slice(0, 24) };
      },
      { rootPresent: false, folded: null as string | null, rootText: null as string | null, rows: [] as string[] },
    );
  /** 📤️ Drives one window action through the route a USER has: unfold the window's Actions pane
   * (`framework.window.<window>.engagement.toggle`), then press the action's own row (`action.<id>`,
   * `windowActionPaneSections` in `🛠️ShellHelpers/🟦️.tsx`). `exportFixture` and `openImportFixture` are
   * both `ActionKind::Shell` with no declared args, so their row EXECUTES on one press — no staged
   * form, no confirmation. Never folds an already-open pane: the toggle is pressed only while the row
   * is absent, which is what the old fallback got wrong (it clicked the last button whose text was
   * exactly "actions" and closed the pane it needed). */
  const activateWindowFileAction = async (actionId: "exportFixture" | "openImportFixture") => {
    await page.keyboard.press("Escape").catch(() => {});
    const row = page.locator(`[id="action.${actionId}"]`);
    const toggle = page.locator('[id="framework.window.puzzle3dMainPerspective.engagement.toggle"]').first();
    if ((await countSafe(row)) === 0 && (await countSafe(toggle)) > 0) {
      await toggle.click({ force: true, timeout: 4000 }).catch(() => {});
      for (let attempt = 0; attempt < 10 && (await countSafe(row)) === 0; attempt++) await page.waitForTimeout(600);
    }
    log(`action-pane ${actionId} rows=${await countSafe(row)} pane=${JSON.stringify(await actionPaneState()).slice(0, 700)}`);
    await row.first().click({ force: true, timeout: 6000 }).catch((error) => log(`action-pane ${actionId} click failed ${String(error).slice(0, 120)}`));
  };

  const historyState = async () =>
    page.evaluate(() => {
      const q = (sel: string) => Array.from(document.querySelectorAll(sel));
      // 🎨️ THE example picker by its own id (`NavbarExampleSelect id="playground.navbar.fixture"`), with
      // the first combobox only as a fallback: the history panel carries a command-filter combobox of
      // its own, so whichever of the two the DOM ordered first decided what a verdict called "the
      // example".
      const combo = (document.getElementById("playground.navbar.fixture") ?? document.querySelector('[role="combobox"]')) as HTMLElement | null;
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
      const dest = join(OUT, `probe-${stamp}-export.json`);
      // 🏷️ The example the navbar picker currently names, as the export FILENAME it should produce —
      // the example ids are exactly their labels lowercased and dash-joined
      // (`PUZZLE3D_EXAMPLE_CONCRETE_FOREST = "concrete-forest"`,
      // `PUZZLE3D_EXAMPLE_NAKAGIN = "nakagin-capsule-tower"`), so this stays a pure read of the chrome.
      const activeExample = String((await chromeState()).example ?? "").replace(/\s+/g, " ").trim();
      const expectedExportName = `${activeExample.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "")}.json`;
      const [download] = await Promise.all([
        page.waitForEvent("download", { timeout: 20000 }).catch(() => null),
        activateWindowFileAction("exportFixture"),
      ]);
      log(`export download=${download ? download.suggestedFilename() : "none"} example=${activeExample} expected=${expectedExportName}`);
      if (download) {
        await download.saveAs(dest);
        log(`export saved ${dest}`);
      }
      verdict("export-only", Boolean(download), `download=${download ? download.suggestedFilename() : "none"} dest=${dest}`);
      verdict(
        "export-names-the-example",
        Boolean(download) && download?.suggestedFilename() === expectedExportName,
        `download=${download ? download.suggestedFilename() : "none"} expected=${expectedExportName} — exporting Concrete Forest and Nakagin must not both land as one constant name`,
      );
      await page.waitForTimeout(600);
      let [chooser] = await Promise.all([
        page.waitForEvent("filechooser", { timeout: 25000 }).catch(() => null),
        activateWindowFileAction("openImportFixture"),
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
        log("import missed the file chooser — `openImportFixture` never reached `requestFileOpen`");
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
          const cloneId = `probe-distinct-${stamp}`;
          clone.id = cloneId;
          clone.label = `Distinct ${String(clone.label ?? "import")}`;
          const origin = Array.isArray(clone.origin) ? clone.origin.map((n, i) => (i === 0 ? Number(n) + 4 : n)) : [4, 0, 0];
          clone.origin = origin;
          // 🌀️ Re-key the clone's vortices onto its OWN id. A clone that keeps `seed-left-001:v0…` carries
          // vortex ids the document already holds, which is not a second object the app can attract
          // against — a "distinct" payload has to be distinct all the way down, not only at the object id.
          if (Array.isArray(clone.vortices)) {
            clone.vortices = (clone.vortices as Record<string, unknown>[]).map((vortex, index) => ({ ...vortex, id: `${cloneId}:v${index}` }));
          }
          src.objects = [...objects, clone];
        } else {
          src.meta = { ...(src.meta ?? {}), probeDistinct: stamp };
        }
        fs.writeFileSync(distinct, JSON.stringify(src));
        distinctReady = true;
        log(`distinct fixture objects=${(src.objects ?? []).length} dest=${distinct}`);
      }
      const instancesBeforeDistinct = await dumpInstances();
      const historyBeforeDistinct = (await historyState()).entryCount;
      if (distinctReady) {
        let [chooser2] = await Promise.all([
          page.waitForEvent("filechooser", { timeout: 25000 }).catch(() => null),
          activateWindowFileAction("openImportFixture"),
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
          log("distinct import missed the file chooser — `openImportFixture` never reached `requestFileOpen`");
        }
        // 🕰️ POLLED: the ingress, the guest fold and the world republication are three round trips, so a
        // single fixed sample cannot tell "the fold was an identity no-op" from "it had not landed yet".
        for (let attempt = 0; attempt < 10 && (await dumpInstances()).count === instancesBeforeDistinct.count; attempt++) await page.waitForTimeout(1000);
        await openHistory();
      }
      const distinctIngress = consoleBuf.filter((l) => /\[DEBUG\] puzzle3d\.import\./.test(l)).slice(-8);
      log(`distinct ingress hops=${JSON.stringify(distinctIngress).slice(0, 1600)}`);
      {
        const afterDistinct = await dumpInstances();
        log(`distinct instances before=${JSON.stringify(instancesBeforeDistinct)} after=${JSON.stringify(afterDistinct)}`);
        verdict(
          "import-distinct",
          afterDistinct.count !== instancesBeforeDistinct.count || JSON.stringify(afterDistinct.ids) !== JSON.stringify(instancesBeforeDistinct.ids),
          `before=${instancesBeforeDistinct.count} after=${afterDistinct.count} guestTaps=${JSON.stringify(distinctIngress).slice(0, 500)}`,
          41,
        );
        const historyAfterDistinct = await historyState();
        verdict(
          "import-distinct-records-history",
          historyAfterDistinct.entryCount > historyBeforeDistinct,
          `before=${historyBeforeDistinct} after=${historyAfterDistinct.entryCount} entries=${JSON.stringify(historyAfterDistinct.entries.slice(-4)).slice(0, 400)} — a distinct exported file must record one undoable row`,
        );
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
  /** 📷️ Waits for ONE pane's published pose to leave `previous`, instead of guessing a fixed settle time.
   * A camera gesture is trailing-debounced host-side (`CAMERA_SYNC_DEBOUNCE_MS`) and only then makes the
   * `setCamera` round trip, so the old fixed 1.2 s/1.6 s waits sampled a pose that had not moved yet and
   * read a false negative (wave B12 §4.1). Returns the last reading either way, so a real "never moved"
   * still fails. */
  const cameraSettled = async (id: string, previous: string | null, budgetMs = 5000) => {
    const deadline = Date.now() + budgetMs;
    let latest = await cameraOf(id);
    while (latest === previous && Date.now() < deadline) {
      await page.waitForTimeout(250);
      latest = await cameraOf(id);
    }
    log(`camera settle ${id} moved=${latest !== previous} waitedMs=${budgetMs - Math.max(0, deadline - Date.now())}`);
    return latest;
  };
  /** 🛰️ Orbits the Perspective pane's camera AWAY from wherever it currently stands, and returns the
   * settled pose. The precondition every focus/zoom assertion needs: `focusSelection` frames the
   * selection (`Puzzle3dFocusSelectionWork`), and framing a document the camera is ALREADY framing
   * publishes a bit-identical pose — which reads exactly like "the camera write was dropped". Waves
   * B11/B20 measured both `focus-selection` and `context-menu-zoom-moves-camera` on a freshly-framed
   * pane, so both scored a true no-op as a defect. Orbit is Alt + right-drag
   * (`resolveWorldOrbitMouseButtonsIdle`, see `camera-gestures`). */
  const orbitPerspectiveAway = async () => {
    const canvas = page.locator('[id="puzzle3d-main-perspective"] canvas, [id="framework.window.puzzle3d-main-perspective"] canvas').first();
    const box = (await countSafe(canvas)) ? await canvas.boundingBox() : await page.locator("canvas").last().boundingBox();
    if (!box) return null;
    const before = await cameraOf("puzzle3d-main-perspective");
    const cx = box.x + box.width * 0.5;
    const cy = box.y + box.height * 0.35;
    await page.keyboard.down("Alt").catch(() => {});
    await page.mouse.move(cx, cy);
    await page.mouse.down({ button: "right" });
    await page.mouse.move(cx + 180, cy + 90, { steps: 20 });
    await page.mouse.up({ button: "right" });
    await page.keyboard.up("Alt").catch(() => {});
    const after = await cameraSettled("puzzle3d-main-perspective", before);
    await page.keyboard.press("Escape").catch(() => {});
    log(`orbit-away moved=${after !== before} before=${String(before).slice(0, 100)} after=${String(after).slice(0, 100)}`);
    return after;
  };
  /** 📷️ Whether a published pose is a real one — `Puzzle3dCamera::default()` is all zeros, i.e. a camera
   * standing exactly where it looks, which is no view direction at all. */
  const cameraIsPosed = (raw: string | null) => {
    if (!raw) return false;
    try {
      const camera = JSON.parse(raw) as { position?: number[]; target?: number[] };
      const position = camera.position ?? [];
      const target = camera.target ?? [];
      return position.length === 3 && position.some((axis, index) => Math.abs(axis - (target[index] ?? 0)) > 1e-6);
    } catch {
      return false;
    }
  };
  /** 🪪️ Resolves an authored ui node id to the DOM id it actually carries. `uiNodeDomId` namespaces every
   * node as `${surface}/${key}` (26/09/09/PROCEDURAL-3D-END-TO-END, `f39d4b0db3`), so an authored
   * `puzzle3d-play-settings.grid-spacing.control` lands as `panel:<bodyKey>/puzzle3d-play-settings.grid-spacing.control`.
   * Matching on the authored suffix keeps this probe independent of the surface prefix. */
  const domIdForAuthoredId = async (authored: string) =>
    page.evaluate((key: string) => {
      const exact = document.getElementById(key);
      if (exact) return key;
      const hit = Array.from(document.querySelectorAll<HTMLElement>("[id]")).find((element) => element.id === key || element.id.endsWith(`/${key}`));
      return hit?.id ?? null;
    }, authored);
  const targetVolumeCount = async () => (await windowHostState()).find((w) => w.id === "puzzle3d-main-perspective")?.volumes ?? -1;
  /** 🕰️ History entry count WITHOUT touching undo/redo — `openHistory()` deliberately clicks
   * `#framework.history.undo` to expand its sections, which would mutate the document from a read-only step. */
  const readHistoryEntryIds = async () => {
    const tab = page.locator('[data-slot="panel-tab-button"][id="framework.panel.history"], #framework.panel.history').first();
    if (await tab.count()) await tab.click({ timeout: 4000 }).catch(() => {});
    await page.waitForTimeout(900);
    return page.evaluate(() => Array.from(document.querySelectorAll('[id^="framework.history.entry."]')).map((r) => r.id));
  };
  /** 🕰️ Entry ids present after an action that were not present before — paging-robust, unlike a raw count
   * (the 21:28 coordination entry found the history panel's own row paging faked a 4→106 "regression"). */
  const newHistoryEntries = (before: string[], after: string[]) => after.filter((id) => !before.includes(id));
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
      await loc.click({ force: true, timeout: 4000 }).catch(() => {});
      await page.waitForTimeout(900);
      if ((await readMeasure(id))?.checked === before.checked) {
        await loc.focus().catch(() => {});
        await page.keyboard.press("Space").catch(() => {});
        await page.waitForTimeout(900);
      }
      if ((await readMeasure(id))?.checked === before.checked) {
        const wrapper = await page.evaluate((target: string) => {
          const el = document.getElementById(target) as HTMLInputElement | null;
          const label = el?.closest('[data-slot="tree-action-checkbox-wrapper"]') as HTMLElement | null;
          if (!el) return null;
          (label ?? el).click();
          return { wrapper: Boolean(label), disabled: el.disabled };
        }, id);
        log(`nudge ${id} checkbox fell through to a synthetic wrapper click ${JSON.stringify(wrapper)}`);
      }
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
      const texts = await options.allInnerTexts().catch(() => [] as string[]);
      const different = texts.findIndex((text) => text.replace(/\n/g, " ").trim() !== (before.text ?? "").trim());
      log(`nudge ${id} select options=${JSON.stringify(texts)} current=${before.text} picking=${different}`);
      if (n > 0 && different >= 0) await options.nth(different).click({ timeout: 3000 }).catch(() => {});
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
    verdict(
      "window-distinct-camera",
      cameraIsPosed(top?.camera ?? null) && cameraIsPosed(persp?.camera ?? null) && top?.camera !== persp?.camera,
      `top=${String(top?.camera).slice(0, 130)} perspective=${String(persp?.camera).slice(0, 130)} — both must be REAL guest-published poses (position !== target); all-zero defaults are two identical non-poses, not two framings (wave B12 §4.1)`,
    );
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
    const historyBefore = await readHistoryEntryIds();
    await dismissChrome();
    const initial = await windowHostState();
    const start = initial.find((w) => w.id === "puzzle3d-main-perspective")?.camera ?? null;
    const topBefore = initial.find((w) => w.id === "puzzle3d-main-top")?.camera ?? null;
    verdict("camera-json-attribute", Boolean(start), `data-camera-json=${String(start).slice(0, 170)}`);
    const drag = async (button: "left" | "middle" | "right", modifier: string | null, dx: number, dy: number, previous: string | null) => {
      if (modifier) await page.keyboard.down(modifier).catch(() => {});
      await page.mouse.move(cx, cy);
      await page.mouse.down({ button });
      await page.mouse.move(cx + dx, cy + dy, { steps: 20 });
      await page.mouse.up({ button });
      if (modifier) await page.keyboard.up(modifier).catch(() => {});
      const settled = await cameraSettled("puzzle3d-main-perspective", previous);
      await dismissChrome();
      return settled;
    };
    const afterOrbit = await drag("right", "Alt", 160, 70, start);
    verdict("camera-orbit", Boolean(afterOrbit) && afterOrbit !== start, `before=${String(start).slice(0, 110)} after=${String(afterOrbit).slice(0, 110)}`);
    const afterPan = await drag("right", "Shift", -130, 90, afterOrbit);
    verdict("camera-pan", Boolean(afterPan) && afterPan !== afterOrbit, `before=${String(afterOrbit).slice(0, 110)} after=${String(afterPan).slice(0, 110)}`);
    await page.mouse.move(cx, cy);
    await page.mouse.wheel(0, -700);
    const afterZoom = await cameraSettled("puzzle3d-main-perspective", afterPan);
    verdict("camera-zoom", Boolean(afterZoom) && afterZoom !== afterPan, `before=${String(afterPan).slice(0, 110)} after=${String(afterZoom).slice(0, 110)}`);
    const alive = await snapshot().catch(() => null);
    verdict("camera-lane-responsive", Boolean(alive && alive.windows.length >= 2), `windows=${alive?.windows.length ?? "unreachable"}`);
    const historyAfter = await readHistoryEntryIds();
    verdict(
      "camera-emits-no-artifact-history",
      newHistoryEntries(historyBefore, historyAfter).length === 0,
      `newEntries=${JSON.stringify(newHistoryEntries(historyBefore, historyAfter))} before=${historyBefore.length} after=${historyAfter.length} — checklist §2 declares setCamera emits no artifact mutations, but World3dHost's dispatchWorldCameraDebounced exists so "the shell-side command-history panel has something to show"; a non-zero delta needs the ticket to say which of the two is the contract`,
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
    const historyBefore = await readHistoryEntryIds();
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
    const historyAfter = await readHistoryEntryIds();
    verdict("window-options-emit-no-history", newHistoryEntries(historyBefore, historyAfter).length === 0, `newEntries=${JSON.stringify(newHistoryEntries(historyBefore, historyAfter))} before=${historyBefore.length} after=${historyAfter.length} (WindowConfig lane, not Artifact)`);
  });

  add("settings-panel", "§19", "read", process.argv.includes("--settings") || battery, async () => {
    await dismissChrome();
    const opened = await openPanel(/settings/i);
    verdict("settings-panel-opens", opened.opened, `tab=${opened.id} tabs=${JSON.stringify(opened.tabs).slice(0, 400)}`);
    if (!opened.opened) return;
    // 🪪️ Authored ids are namespaced `${surface}/${key}` by `uiNodeDomId`, so every §19 locator matches on
    // the AUTHORED suffix, never on a bare `^=` prefix (wave B12 §4.2 read the live ids off :6013).
    const steppers = await page.evaluate(() =>
      Array.from(document.querySelectorAll<HTMLElement>('[id*="puzzle3d-play-settings."]'))
        .filter((el) => el.id.endsWith(".control"))
        .map((el) => ({ id: el.id, slot: el.getAttribute("data-slot"), value: (el as HTMLInputElement).value ?? null })),
    );
    const settingsIds = await page.evaluate(() => Array.from(document.querySelectorAll<HTMLElement>('[id*="puzzle3d-play-settings"]')).map((el) => el.id));
    log(`settings steppers=${JSON.stringify(steppers)} allSettingsIds=${JSON.stringify(settingsIds)}`);
    verdict("settings-steppers-present", steppers.length >= 4, `stepper controls=${JSON.stringify(steppers.map((s) => s.id))} allSettingsIds=${JSON.stringify(settingsIds).slice(0, 500)} openedTab=${opened.id}`);
    const targetId = (await domIdForAuthoredId("puzzle3d-play-settings.grid-spacing.control")) ?? (await domIdForAuthoredId("puzzle3d-play-settings.grid-spacing"));
    if (!targetId) {
      verdict("settings-grid-spacing-bumps", false, "no grid-spacing stepper is addressable in the Settings panel at any surface prefix");
      verdict("settings-value-reaches-window-rail", false, "no grid-spacing stepper to bump");
      return;
    }
    const before = await readMeasure(targetId);
    // 🪜️ `Stepper`'s +/− is a press-and-hold driven from mousedown/mouseup, never a synthesized `click`.
    const plus = page.locator(`[id="${targetId}"]`).locator("xpath=..").locator('[data-slot="stepper-plus"]').first();
    const plusCount = await plus.count().catch(() => 0);
    if (plusCount) {
      const box = await plus.boundingBox().catch(() => null);
      if (box) {
        await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
        await page.mouse.down();
        await page.waitForTimeout(120);
        await page.mouse.up();
      } else await plus.click({ force: true, timeout: 4000 }).catch(() => {});
    } else {
      await page.locator(`[id="${targetId}"]`).first().focus().catch(() => {});
      await page.keyboard.press("ArrowUp").catch(() => {});
    }
    await page.waitForTimeout(2500);
    const after = await readMeasure(targetId);
    verdict("settings-grid-spacing-bumps", Boolean(before) && JSON.stringify(before) !== JSON.stringify(after), `targetId=${targetId} plusButtons=${plusCount} before=${JSON.stringify(before)} after=${JSON.stringify(after)}`);
    await unfoldMeasures();
    const railGrid = await readMeasure("puzzle3d-play-grid-spacing");
    verdict("settings-value-reaches-window-rail", Boolean(railGrid) && railGrid?.value === after?.value, `settings=${JSON.stringify(after)} windowRail=${JSON.stringify(railGrid)}`);
  });

  add("add-object-dialog", "§23", "read", process.argv.includes("--adddialog") || battery, async () => {
    await dismissChrome();
    const instancesBefore = await dumpInstances();
    const found = await page.evaluate(() => document.querySelectorAll('[id="shell-menu.action.openAddObjectDialog"]').length);
    log(`add-object trigger=${found}`);
    verdict("add-object-trigger-present", found > 0, `trigger=${found}`);
    if (!found) {
      await dismissChrome();
      return;
    }
    await page.evaluate(() => {
      const buttons = Array.from(document.querySelectorAll('[id="shell-menu.action.openAddObjectDialog"]'));
      (buttons.at(-1) as HTMLButtonElement | undefined)?.click();
    });
    await page.locator('[data-slot="dialog-content"] #objectKind, [data-slot="dialog-title"]').first().waitFor({ state: "visible", timeout: 5000 }).catch(() => {});
    await page.waitForTimeout(400);
    const dialogs = await page.evaluate(() =>
      Array.from(document.querySelectorAll('[data-slot="dialog-content"], [role="dialog"]')).map(
        (d) => `${d.id || d.getAttribute("data-slot")}=${(d as HTMLElement).innerText.replace(/\n/g, " | ").slice(0, 160)}`,
      ),
    );
    const realDialog = dialogs.some((row) => /add object|objekt hinzufügen|choose the kind/i.test(row));
    verdict("add-object-dialog-opens", realDialog, `dialogSurfaces=${JSON.stringify(dialogs).slice(0, 500)}`);
    const selectTrigger = page.locator('[data-slot="dialog-content"] #objectKind, [data-slot="dialog-content"] [data-slot="select-trigger"]').first();
    if (await selectTrigger.count()) {
      await selectTrigger.click({ force: true, timeout: 4000 }).catch(() => {});
      await page.waitForTimeout(800);
      const options = await page.evaluate(() => Array.from(document.querySelectorAll('[role="option"]')).map((o) => (o as HTMLElement).innerText.trim().slice(0, 40)));
      log(`add-object kind options=${JSON.stringify(options)}`);
      verdict("add-object-kind-options-are-dynamic", options.length > 1, `options=${JSON.stringify(options)} — checklist §23 predicts a single hardcoded "Object"`);
      const option = page.locator('[role="option"]').nth(options.length > 1 ? 1 : 0);
      if (await option.count()) await option.click({ force: true, timeout: 4000 }).catch(() => {});
    } else verdict("add-object-kind-options-are-dynamic", false, "no kind select rendered inside the dialog");
    const submit = page.locator('[data-slot="dialog-content"] #ui.dialog.submit, [data-slot="dialog-content"] button').filter({ hasText: /^(add|hinzufügen)/i }).first();
    if (await submit.count()) await submit.click({ force: true, timeout: 4000 }).catch(() => {});
    else await page.evaluate(() => (document.getElementById("ui.dialog.submit") as HTMLButtonElement | null)?.click());
    await page.waitForTimeout(3500);
    const instancesAfter = await dumpInstances();
    log(`add-object instances before=${JSON.stringify(instancesBefore)} after=${JSON.stringify(instancesAfter)}`);
    verdict("add-object-instance-count-increases", instancesAfter.count > instancesBefore.count, `before=${instancesBefore.count} after=${instancesAfter.count}`);
    await page.keyboard.press("Escape").catch(() => {});
    await dismissChrome();
  });

  /** 🌍️ Reads the outliner's own section labels plus the grid/LOD rail labels — the DOM translation of
   * checklist §25's `document_json.contains("Baukomponenten")` assertion. */
  const readLocaleLabels = async () => {
    await openPanel(/document|artifact|outliner|puzzle3d-play-document/i);
    const read = async () =>
      page.evaluate(() => {
        const root = document.querySelector('[id^="puzzle3d-play-document"]') as HTMLElement | null;
        return {
          rootText: (root?.innerText || "").replace(/\n/g, " | ").slice(0, 400),
          rails: Array.from(document.querySelectorAll('[id^="puzzle3d-play-grid"], [id^="puzzle3d-play-lod"]'))
            .map((el) => `${el.id}=${(el as HTMLElement).innerText.replace(/\n/g, " ").trim().slice(0, 40)}`)
            .slice(0, 12),
        };
      });
    let last = await read();
    for (let i = 0; i < 8 && !last.rootText; i++) {
      await page.waitForTimeout(1200);
      last = await read();
    }
    return last;
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
    // 💬️ The typed engagement field is NOT inside the top-left Actions pane: `Window` renders it in the
    // top-middle `search` Pane (`framework.window.<window>.search`), as the `Search` element's own
    // `Input` — whose id is the guest's `engagement.input.id`, or `ui.windowSearch.action` when the
    // guest leaves it unnamed (`windowEngagementToSearchSpec` → `Search`). Wave B10 merged the two
    // FOLD states (`searchExpanded = searchVisible && !actionsFolded`), not the two panes, so the old
    // `[id^="…engagement"] input` scope could never match the field it was measuring, and the blind
    // toggle click below FOLDED an already-open pane on every run that started with one.
    const inputSelector =
      '[id="framework.window.puzzle3dMainPerspective.search"] input, [id="framework.window.puzzle3dMainPerspective.search"] [role="textbox"], [id="framework.window.puzzle3dMainPerspective.search"] textarea, [id="ui.windowSearch.action"], input[placeholder*="fill" i], input[placeholder*="brush" i], [role="textbox"][placeholder*="fill" i]';
    const foldState = async () =>
      evalSafe(
        () => {
          const el = document.getElementById("framework.window.puzzle3dMainPerspective.engagement");
          const search = document.getElementById("framework.window.puzzle3dMainPerspective.search");
          const btn = document.getElementById("framework.window.puzzle3dMainPerspective.engagement.toggle");
          const rect = btn?.getBoundingClientRect();
          const top = rect ? document.elementFromPoint(rect.x + rect.width / 2, rect.y + rect.height / 2) : null;
          return {
            folded: el?.getAttribute("data-folded") ?? null,
            searchFolded: search?.getAttribute("data-folded") ?? null,
            toggle: btn ? `${btn.tagName}|${btn.getAttribute("aria-pressed") ?? ""}|${btn.getAttribute("disabled") ?? ""}` : "absent",
            rect: rect ? `${Math.round(rect.x)},${Math.round(rect.y)} ${Math.round(rect.width)}x${Math.round(rect.height)}` : "none",
            covered: top ? !(btn?.contains(top) ?? false) : true,
            top: top ? `${top.tagName}#${top.id || "-"}[${top.getAttribute("data-slot") ?? "-"}]` : "none",
          };
        },
        { folded: null as string | null, searchFolded: null as string | null, toggle: "eval-failed", rect: "none", covered: true, top: "none" },
      );
    for (let attempt = 0; attempt < 6 && (await countSafe(page.locator(inputSelector))) === 0; attempt++) {
      log(`[DEBUG] engagement fold attempt=${attempt} before=${JSON.stringify(await foldState())}`);
      if (toggleCount) await toggle.click({ force: true, timeout: 4000 }).catch((error) => log(`[DEBUG] engagement toggle click failed ${String(error).slice(0, 120)}`));
      await page.waitForTimeout(1200);
      log(`[DEBUG] engagement fold attempt=${attempt} after=${JSON.stringify(await foldState())}`);
    }
    const pane = await page.evaluate(() => {
      const root = document.getElementById("framework.window.puzzle3dMainPerspective.engagement");
      const search = document.getElementById("framework.window.puzzle3dMainPerspective.search");
      return {
        rootPresent: Boolean(root),
        searchPresent: Boolean(search),
        rootText: (root as HTMLElement | null)?.innerText.replace(/\n/g, " | ").slice(0, 240) ?? null,
        fields: Array.from(document.querySelectorAll('input, textarea, [role="textbox"], [contenteditable="true"]'))
          .map((el) => `${el.id || "?"}|${el.getAttribute("type") ?? el.getAttribute("role") ?? el.tagName.toLowerCase()}|${el.getAttribute("placeholder") ?? ""}`)
          .slice(0, 24),
      };
    });
    log(`engagement pane=${JSON.stringify(pane)}`);
    const input = page.locator(inputSelector).first();
    const inputCount = await input.count();
    const placeholder = inputCount ? await input.getAttribute("placeholder") : null;
    log(`engagement toggle=${toggleCount} input=${inputCount} placeholder=${placeholder}`);
    verdict("engagement-input-present", inputCount > 0, `toggle=${toggleCount} input=${inputCount} placeholder=${placeholder} pane=${JSON.stringify(pane).slice(0, 400)}`);
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

  /** 🧾️ The host-side selection truth for every world surface. `data-interaction-json` is
   * `mergeWorldInteractionWithLeftoverV1(scene.interactionJson, leftoverWorldSelectionOverlayV1())`
   * (`🌐️World3dHost/🟦️.tsx`), so it carries BOTH the guest-published selection and the host's leftover
   * overlay — the one attribute that separates "the row click never dispatched" from "the guest answered
   * with an empty InteractionView". */
  const worldInteraction = async () =>
    evalSafe(
      () =>
        Array.from(document.querySelectorAll<HTMLElement>("[data-interaction-json]")).map((element) => {
          let selectedIds: string[] = [];
          let hovered: string | null = null;
          try {
            // 🔦️ `data-selection-json` (wave B20) is the pane's PAINTED selection — the guest's
            // `selectionJson` lane with the host leftover overlay merged over it. The interaction
            // record carries neither `selectedIds` nor a hover target (it is the utility/brush/fill
            // record), so reading it alone reported an empty selection on a pane that was painting one.
            const painted = JSON.parse(element.getAttribute("data-selection-json") || "{}") as { selectedIds?: string[]; hoverTarget?: { id?: string } | null; hoveredVortexFullId?: string | null };
            const parsed = JSON.parse(element.getAttribute("data-interaction-json") || "{}") as { selectedIds?: string[]; hoverTarget?: { id?: string } | null; hoveredVortexFullId?: string | null };
            selectedIds = painted.selectedIds ?? parsed.selectedIds ?? [];
            hovered = painted.hoverTarget?.id ?? painted.hoveredVortexFullId ?? parsed.hoverTarget?.id ?? parsed.hoveredVortexFullId ?? null;
          } catch {
            selectedIds = ["parse-failed"];
          }
          let selectedInstances: string[] = [];
          try {
            selectedInstances = (JSON.parse(element.getAttribute("data-instances-json") || "[]") as { id?: string; selected?: boolean }[]).filter((instance) => instance.selected).map((instance) => instance.id ?? "?");
          } catch {
            selectedInstances = ["parse-failed"];
          }
          return { surface: element.getAttribute("data-surface-id") ?? "?", window: element.getAttribute("data-window-instance-id") ?? "?", selectedIds: selectedIds.slice(0, 8), hovered, selectedInstances: selectedInstances.slice(0, 8) };
        }),
      [] as { surface: string; window: string; selectedIds: string[]; hovered: string | null; selectedInstances: string[] }[],
    );
  /** 🪜️ `ShellHost.applyLeftoverInteractionView` prints `[DEBUG] leftover InteractionView` the moment it
   * turns a guest response's `output` into the leftover overlay. Its presence after a row click pins the
   * failure to the RENDER hop; its absence pins it to the DISPATCH hop, which is the whole point of
   * measuring it instead of guessing. */
  const leftoverViewTail = (since: number) => consoleBuf.slice(since).filter((row) => row.includes("leftover InteractionView")).slice(-4);
  /** 🔖️ Hands {@link clickTreeRowPoint}'s evaluate the row and part to measure — a page global rather than an
   * argument so the helper body stays closure-free and {@link evalSafe} can retry it verbatim. */
  const markTreeRowTarget = async (id: string, part: "label" | "background") => {
    await page
      .evaluate(([rowId, rowPart]) => {
        (globalThis as unknown as { __semioProbeRow?: { id: string; part: string } }).__semioProbeRow = { id: rowId, part: rowPart };
      }, [id, part] as const)
      .catch(() => {});
  };
  /** 🖱️ Clicks one DOM point of a tree row by its viewport rect, so "the label text" and "the row
   * background" are two separately measurable targets rather than one `force: true` guess at the centre.
   *
   * 🚫️ The BACKGROUND point is searched, never assumed: an outliner row carries a fold chevron on the left
   * and its `Hide`/`Lock` row actions on the right, all of which `stopPropagation`. Clicking the row's
   * right edge therefore LOCKED the object (measured live: the next selection was refused, and the row read
   * "Unlock" afterwards). The search scans the row at mid-height and returns the first x that lies inside
   * no descendant `button` — and, for `background`, outside `[data-slot="tree-label"]` as well. */
  const clickTreeRowPoint = async (rowId: string, part: "label" | "background") => {
    await markTreeRowTarget(rowId, part);
    const spot = await evalSafe(
      () => {
        const marker = (globalThis as unknown as { __semioProbeRow?: { id: string; part: string } }).__semioProbeRow;
        const row = marker ? document.getElementById(marker.id) : null;
        if (!row) return null;
        const rowRect = row.getBoundingClientRect();
        const label = row.querySelector<HTMLElement>('[data-slot="tree-label"]');
        const labelRect = label?.getBoundingClientRect() ?? null;
        const kind = row.getAttribute("data-tree-row-kind");
        const describe = (x: number, y: number) => {
          const hit = document.elementFromPoint(x, y) as HTMLElement | null;
          return hit ? `${hit.tagName.toLowerCase()}#${hit.id || "-"}[${hit.getAttribute("data-slot") ?? "-"}]${hit.closest(`#${CSS.escape(row.id)}`) === row ? "" : "@outside"}` : "nothing";
        };
        const shape = { kind, has: Boolean(labelRect), labelWidth: Math.round(labelRect?.width ?? 0), rowWidth: Math.round(rowRect.width) };
        const y = rowRect.y + rowRect.height / 2;
        if (marker?.part === "label" && labelRect && labelRect.width > 2) {
          const labelY = labelRect.y + labelRect.height / 2;
          const labelX = labelRect.x + labelRect.width / 2;
          return { x: labelX, y: labelY, part: "label", hit: describe(labelX, labelY), ...shape };
        }
        const blockers = Array.from(row.querySelectorAll("button, [role='button'], input, a")).map((element) => element.getBoundingClientRect());
        const free = (x: number) =>
          !blockers.some((rect) => x >= rect.x - 3 && x <= rect.x + rect.width + 3) && !(labelRect && x >= labelRect.x - 3 && x <= labelRect.x + labelRect.width + 3) && document.elementFromPoint(x, y)?.closest(`#${CSS.escape(row.id)}`) === row;
        for (let x = rowRect.x + 4; x < rowRect.x + rowRect.width - 4; x += 3) if (free(x)) return { x, y, part: "background", hit: describe(x, y), ...shape };
        return { x: rowRect.x + rowRect.width / 2, y, part: "background-fallback", hit: describe(rowRect.x + rowRect.width / 2, y), ...shape };
      },
      null as { x: number; y: number; part: string; hit: string; kind: string | null; has: boolean; labelWidth: number; rowWidth: number } | null,
    );
    if (!spot) return null;
    await page.mouse.click(spot.x, spot.y).catch(() => {});
    return spot;
  };
  /** 🎯️ Selects the first object row through the outliner — checklist §6 says this `interactionSelect` path
   * was never part of the viewport render/hover gap, so it isolates "the menu is wrong" from "nothing was
   * selected" when a context-menu or keybinding step reads an empty selection.
   *
   * 🪜️ Clicks the row BACKGROUND first and its label second, reporting each separately: `Tree`'s group-row
   * branches historically wired `onClick` only to `[data-slot="tree-label"]` while the leaf branch wired it
   * to the whole `role="treeitem"` shell, so an object row (which nests its vortices, hence a group row)
   * answered a centre click with nothing at all. */
  const selectViaOutliner = async () => {
    await openPanel(/document|artifact|outliner|puzzle3d-play-document/i);
    // 🌳️ A tree ROW, not the first node whose id happens to start with the outliner's surface prefix —
    // that one is the section wrapper, which owns no selection and swallowed the click. `treeitem`/
    // `tree-item` is what the interpreter marks a selectable row with.
    const rows = await page.evaluate(() =>
      Array.from(document.querySelectorAll<HTMLElement>('[role="treeitem"], [data-slot="tree-item"]'))
        .filter((element) => element.id.includes("puzzle3d-play-document"))
        .map((element) => ({ id: element.id, text: element.innerText.replace(/\n/g, " ").trim().slice(0, 40) })),
    );
    const fallback = await page.evaluate(() => Array.from(document.querySelectorAll<HTMLElement>('[id*="puzzle3d-play-document"]')).map((element) => element.id).slice(0, 40));
    // 🌳️ An ENTITY row, not the tree root and not a group header: the root (`puzzle3d-play-document`) and
    // the three group headers (`…/puzzle3d-play-document.objects|.references|.target-volumes`) are
    // `treeitem`s too, and selecting one of them selects nothing (measured live on :6013, 16:27).
    const entityRow = (id: string) => {
      const leaf = id.split("/").pop() ?? "";
      return id.includes("/") && !leaf.startsWith("puzzle3d-play-document.") && leaf !== "puzzle3d-play-document";
    };
    const targetId = rows.find((row) => entityRow(row.id))?.id ?? fallback.find(entityRow) ?? rows[0]?.id ?? fallback[0];
    log(`selectViaOutliner treeRows=${JSON.stringify(rows).slice(0, 400)} target=${targetId ?? "none"} outlinerIds=${JSON.stringify(fallback).slice(0, 400)}`);
    const attempt = async (part: "label" | "background") => {
      if (!targetId) return { part, spot: null, interaction: [] as Awaited<ReturnType<typeof worldInteraction>>, leftover: [] as string[], ariaSelected: [] as string[] };
      const consoleMark = consoleBuf.length;
      const spot = await clickTreeRowPoint(targetId, part);
      await page.waitForTimeout(2500);
      const interaction = await worldInteraction();
      const leftover = leftoverViewTail(consoleMark);
      const state = await selectionState();
      const ariaSelected = state.selected.filter((row) => row.startsWith("panel:") || row.startsWith("puzzle3d-play-"));
      log(`selectViaOutliner ${part} spot=${JSON.stringify(spot)} interaction=${JSON.stringify(interaction).slice(0, 400)} leftoverLines=${JSON.stringify(leftover).slice(0, 400)} ariaSelected=${JSON.stringify(ariaSelected).slice(0, 200)}`);
      return { part, spot, interaction, leftover, ariaSelected };
    };
    const landed = (result: Awaited<ReturnType<typeof attempt>>) =>
      result.interaction.some((surface) => surface.selectedIds.length > 0 || surface.selectedInstances.length > 0) || result.ariaSelected.length > 0 || result.leftover.some((row) => !row.includes('"selectedIds":[]'));
    // 🎯️ BACKGROUND first, deliberately: it is the strictly harder target (measured live, the object row's
    // label span is 12 px of a 160 px row, which is why a centre click reached only the background), so a
    // background landing proves the label one too and a background-only failure names the defect precisely.
    const byBackground = await attempt("background");
    const byLabel = landed(byBackground) ? null : await attempt("label");
    const winner = landed(byBackground) ? byBackground : byLabel && landed(byLabel) ? byLabel : byBackground;
    const sel = await selectionState();
    const entities = sel.selected.filter((row) => row.startsWith("panel:") || row.startsWith("puzzle3d-play-"));
    const selectedIds = winner.interaction.flatMap((surface) => surface.selectedIds);
    // 🕹️ The world surface's own `data-status-json` is the app-side half of the evidence: if a row click
    // never reaches the framework selection domain, the rail says "0 selected" there too, and the verdict
    // can distinguish "the probe read the wrong DOM attribute" from "nothing was selected".
    const status = sel.hosts.map((host) => `${host.surface}=${host.status.slice(0, 120)}`);
    const hop = `rowKind=${winner.spot?.kind ?? "?"} labelWidth=${winner.spot?.labelWidth ?? 0}/${winner.spot?.rowWidth ?? 0} landedVia=${landed(byBackground) ? "background" : byLabel && landed(byLabel) ? "label" : "neither"} dispatchObserved=${byBackground.leftover.length + (byLabel?.leftover.length ?? 0) > 0} selectedIds=${JSON.stringify(selectedIds)}`;
    log(`selectViaOutliner clicked=${targetId ?? "none"} entitySelected=${JSON.stringify(entities)} allAriaSelected=${JSON.stringify(sel.selected).slice(0, 300)} surfaceStatus=${JSON.stringify(status).slice(0, 400)} ${hop}`);
    return {
      rows: rows.length || fallback.length,
      selected: entities.length + selectedIds.length + winner.interaction.reduce((total, surface) => total + surface.selectedInstances.length, 0),
      raw: sel.selected,
      clicked: targetId ?? null,
      status,
      hop,
      selectedIds,
    };
  };

  /** 🎯️ Projects one rendered instance's world position onto the Perspective pane's canvas, out of the two
   * attributes the host already publishes there — `data-instances-json` (`WorldInstanceRecord.position`) and
   * `data-viewport-camera-json` (`world3dCameraDomJson`: position/target/up/fov/projection). Replaces the
   * `0.78 × 0.42` guess the context-menu step used to right-click, which lands on whatever the framing put
   * there. Returns viewport pixels, or `null` when the instance sits behind the camera or the pane has not
   * published a scene yet. */
  const projectInstance = async (wantedId: string | null) => {
    await page.evaluate((id) => {
      (globalThis as unknown as { __semioProbePick?: string | null }).__semioProbePick = id;
    }, wantedId).catch(() => {});
    return evalSafe(
      () => {
        const wanted = (globalThis as unknown as { __semioProbePick?: string | null }).__semioProbePick ?? null;
        const host = document.querySelector("#puzzle3d-main-perspective");
        const surface = host?.querySelector("[data-instances-json]") as HTMLElement | null;
        const canvas = host?.querySelector("canvas") as HTMLCanvasElement | null;
        if (!surface || !canvas) return null;
        let instances: { id?: string; position?: number[]; x?: number; y?: number; z?: number }[] = [];
        try {
          instances = JSON.parse(surface.getAttribute("data-instances-json") || "[]");
        } catch {
          return null;
        }
        const chosen = (wanted ? instances.find((instance) => Boolean(instance.id) && (instance.id === wanted || wanted.endsWith(`/${instance.id}`))) : undefined) ?? instances[0];
        if (!chosen) return null;
        const point = chosen.position ?? [chosen.x ?? 0, chosen.y ?? 0, chosen.z ?? 0];
        let camera: { position?: number[]; target?: number[]; up?: number[]; fov?: number | null; projection?: string | null } = {};
        try {
          camera = JSON.parse(surface.getAttribute("data-viewport-camera-json") || surface.getAttribute("data-camera-json") || "{}");
        } catch {
          camera = {};
        }
        const eye = camera.position ?? [4, -4, 3];
        const target = camera.target ?? [0, 0, 0];
        const up = camera.up ?? [0, 0, 1];
        const sub = (a: number[], b: number[]) => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
        const cross = (a: number[], b: number[]) => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
        const dot = (a: number[], b: number[]) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
        const norm = (a: number[]) => {
          const length = Math.hypot(a[0], a[1], a[2]) || 1;
          return [a[0] / length, a[1] / length, a[2] / length];
        };
        const forward = norm(sub(target, eye));
        const right = norm(cross(forward, up));
        const trueUp = cross(right, forward);
        const relative = sub(point, eye);
        const depth = dot(relative, forward);
        if (!(depth > 0.0001)) return null;
        const rect = canvas.getBoundingClientRect();
        const aspect = rect.width / Math.max(1, rect.height);
        const tanHalf = Math.tan((((camera.fov ?? 45) as number) * Math.PI) / 360);
        const ndcX = dot(relative, right) / (depth * tanHalf * aspect);
        const ndcY = dot(relative, trueUp) / (depth * tanHalf);
        return {
          id: chosen.id ?? "?",
          x: rect.x + (ndcX * 0.5 + 0.5) * rect.width,
          y: rect.y + (0.5 - ndcY * 0.5) * rect.height,
          rect: { x: Math.round(rect.x), y: Math.round(rect.y), w: Math.round(rect.width), h: Math.round(rect.height) },
          instances: instances.length,
          fov: camera.fov ?? 45,
          projection: camera.projection ?? null,
        };
      },
      null as { id: string; x: number; y: number; rect: { x: number; y: number; w: number; h: number }; instances: number; fov: number; projection: string | null } | null,
    );
  };
  /** 🖱️ Hovers the projected point and widens into a small spiral until the pane reports a hovered or
   * selected instance, so the context-menu step opens over a real object instead of empty space. Hover is
   * read back off `data-interaction-json`/`data-instances-json`, never assumed. */
  const landOnInstance = async (wantedId: string | null) => {
    const projected = await projectInstance(wantedId);
    if (!projected) return null;
    const offsets = [0, 10, -10, 20, -20, 34, -34, 52, -52];
    for (const dy of offsets) {
      for (const dx of offsets) {
        const x = projected.x + dx;
        const y = projected.y + dy;
        if (x < projected.rect.x || y < projected.rect.y || x > projected.rect.x + projected.rect.w || y > projected.rect.y + projected.rect.h) continue;
        await page.mouse.move(x, y).catch(() => {});
        await page.waitForTimeout(180);
        const state = await worldInteraction();
        if (state.some((surface) => Boolean(surface.hovered) || surface.selectedInstances.length > 0)) {
          log(`landOnInstance hit id=${projected.id} at=${Math.round(x)},${Math.round(y)} offset=${dx},${dy} projected=${Math.round(projected.x)},${Math.round(projected.y)} instances=${projected.instances} fov=${projected.fov} projection=${projected.projection}`);
          return { ...projected, x, y, hovered: true };
        }
      }
    }
    log(`landOnInstance no hover id=${projected.id} projected=${Math.round(projected.x)},${Math.round(projected.y)} rect=${JSON.stringify(projected.rect)} instances=${projected.instances} fov=${projected.fov} projection=${projected.projection}`);
    return { ...projected, hovered: false };
  };

  add("context-menu-rows", "§15", "mutate", process.argv.includes("--contextmenu") || battery, async () => {
    await dismissChrome();
    const framed = await frameForestTableAfterCensus();
    const picked = await selectViaOutliner();
    verdict(
      "context-menu-selection-precondition",
      picked.rows > 0 && picked.selected > 0,
      `outlinerRows=${picked.rows} clicked=${picked.clicked ?? "none"} selected=${picked.selected} ${picked.hop} surfaceStatus=${JSON.stringify(picked.status).slice(0, 300)}`,
    );
    // 🛰️ Precondition for `context-menu-zoom-moves-camera`, established BEFORE the menu opens (an
    // Alt+right-drag on the canvas would dismiss an open menu): the row zooms to the SELECTION, and a
    // pane already framing it republishes a bit-identical pose, which reads exactly like a dropped
    // camera write. Orbit away so "the camera moved" is a question this step can actually answer.
    await orbitPerspectiveAway();
    const canvas = page.locator("canvas").last();
    const box = await canvas.boundingBox();
    // 🎯️ The projected position of a real instance when the pane publishes one; the old `0.78 × 0.42`
    // fraction only as the last resort, so `context-menu-opens` measures the plugin's object rows rather
    // than whatever empty space the framing left at that fraction.
    const landing = await landOnInstance(picked.selectedIds[0] ?? picked.clicked);
    const point = landing ? { x: landing.x, y: landing.y } : { x: (box?.x ?? 0) + framed.table.x, y: (box?.y ?? 0) + framed.table.y };
    log(`context-menu point=${Math.round(point.x)},${Math.round(point.y)} via=${landing ? (landing.hovered ? "projected-hover" : "projected-blind") : "table-fraction"}`);
    // 🖱️ A left click on empty canvas is a pick that CLEARS the selection, which is the very precondition
    // the step just established through the outliner. Pick first only when the outliner landed nothing.
    if (picked.selected === 0) {
      await page.mouse.click(point.x, point.y).catch(() => {});
      await page.waitForTimeout(1500);
    }
    const consoleMark = consoleBuf.length;
    await page.mouse.click(point.x, point.y, { button: "right" }).catch(() => {});
    await page.waitForTimeout(1500);
    const menuRows = async () =>
      evalSafe(
        () =>
          Array.from(document.querySelectorAll('[role="menu"] button, [role="menuitem"], [data-slot="context-menu"] button, [data-slot="context-menu-item"]')).map((el) => ({
            id: el.id || null,
            action: el.getAttribute("data-menu-action"),
            text: (el as HTMLElement).innerText.replace(/\n/g, " ").trim().slice(0, 40),
          })),
        [] as { id: string | null; action: string | null; text: string }[],
      );
    // ⏳️ `World3dHost`'s `onContextMenu` AWAITS `openSurfaceContextMenu`, a round trip to the guest, before
    // it calls `setContextMenu` — so the menu appears whenever that reply does, not on a fixed delay. One
    // fixed 1.5 s wait measured rows=5 and rows=0 on two otherwise identical runs; poll instead.
    let topRows = await menuRows();
    let polls = 0;
    for (; polls < 30 && topRows.length === 0; polls++) {
      await page.waitForTimeout(1000);
      topRows = await menuRows();
    }
    log(`context-menu opened after ${topRows.length ? `${polls} polls` : "30 polls with nothing"} rows=${topRows.length}`);
    // 🗂️ `hide-show` and `lock-unlock` are authored INSIDE the `menu.group.hand` submenu — the guest's own
    // law asserts that grouping (`✏️editor/🧪️tests/🔬️unit/🦀️.rs`, "hide/lock rows should be grouped under
    // hand") — so a vocabulary read of the top level alone reports them missing when they are merely folded.
    const groups = topRows.filter((row) => row.id?.startsWith("menu.group."));
    const merged = new Map(topRows.filter((row) => row.id).map((row) => [row.id as string, row]));
    for (const group of groups) {
      await page.locator(`[id="${group.id}"]`).first().hover({ timeout: 2500 }).catch(() => {});
      await page.waitForTimeout(900);
      for (const row of await menuRows()) if (row.id) merged.set(row.id, row);
    }
    const rows = [...merged.values()];
    log(`context-menu rows=${JSON.stringify(rows).slice(0, 1400)} groupsExpanded=${JSON.stringify(groups.map((group) => group.id))}`);
    log(`context-menu selectionAtRightClick=${JSON.stringify(await worldInteraction()).slice(0, 300)} chrome=${JSON.stringify((await chromeState()).menus).slice(0, 400)}`);
    log(`context-menu console tail=${JSON.stringify(consoleBuf.slice(consoleMark).filter((row) => /context.?menu/i.test(row)).slice(-8)).slice(0, 800)}`);
    log(`[DEBUG] context-menu console ingress=${JSON.stringify(consoleBuf.slice(consoleMark).filter((row) => /world3d-contextmenu/i.test(row)).slice(0, 8)).slice(0, 1400)}`);
    log(
      `[DEBUG] context-menu dom=${JSON.stringify(
        await evalSafe(
          () => ({
            menus: document.querySelectorAll('[role="menu"]').length,
            items: document.querySelectorAll('[role="menuitem"]').length,
            slots: Array.from(document.querySelectorAll("[data-slot]")).map((el) => el.getAttribute("data-slot") ?? "").filter((slot) => /menu/i.test(slot)).slice(0, 20),
            poppers: document.querySelectorAll("[data-radix-popper-content-wrapper]").length,
            states: Array.from(document.querySelectorAll("[data-state]")).filter((el) => /menu/i.test(el.getAttribute("data-slot") ?? el.className.toString())).map((el) => `${el.getAttribute("data-slot") ?? el.tagName}=${el.getAttribute("data-state")}`).slice(0, 20),
          }),
          { menus: -1, items: -1, slots: [] as string[], poppers: -1, states: [] as string[] },
        ),
      ).slice(0, 1200)}`,
    );
    log(`[DEBUG] context-menu console menuish=${JSON.stringify(consoleBuf.slice(consoleMark).filter((row) => /menu|dropped|surface|refus|Error|error/i.test(row)).slice(0, 14)).slice(0, 2200)}`);
    verdict("context-menu-opens", rows.length > 0, `rows=${rows.length}`);
    const ids = new Set(rows.map((r) => r.id));
    const missing = ["duplicate", "select-same-kind", "zoom", "delete", "hide-show", "lock-unlock"].filter((id) => !ids.has(id));
    verdict("context-menu-object-vocabulary", rows.length > 0 && missing.length === 0, `missing=${JSON.stringify(missing)} present=${JSON.stringify([...ids])}`);
    const zoom = rows.find((r) => r.id === "zoom");
    verdict("context-menu-zoom-row-action-is-registered", Boolean(zoom) && zoom?.action !== "zoomToSelection", `zoom row action=${zoom?.action ?? "row absent"} — checklist §15 predicts the unregistered "zoomToSelection"`);
    if (zoom) {
      const hardBefore = hardFaults.length;
      const faultsBefore = faults.length;
      // 🪟️ BOTH panes, not just the one the menu opened over: `focusSelection` carries no `windowId`
      // of its own, so the shell addresses it to `activeWindowIdRef` — if the OTHER pane moves, the
      // camera write landed on the wrong window instance, which reads identically to "dropped" when
      // only one pane is sampled (wave B20 defect 2).
      const panesBefore = await windowHostState();
      const cameraBefore = panesBefore.find((pane) => pane.id === "puzzle3d-main-perspective")?.camera ?? null;
      const zoomMark = consoleBuf.length;
      await page.locator('[id="zoom"]').last().click({ force: true, timeout: 4000 }).catch(() => {});
      await page.waitForTimeout(3000);
      const panesAfter = await windowHostState();
      const cameraAfter = panesAfter.find((pane) => pane.id === "puzzle3d-main-perspective")?.camera ?? null;
      const movedPanes = panesAfter.filter((pane) => pane.camera !== (panesBefore.find((other) => other.id === pane.id)?.camera ?? null)).map((pane) => pane.id);
      log(`zoom panes before=${JSON.stringify(panesBefore.map((pane) => ({ id: pane.id, surface: pane.surface, camera: String(pane.camera).slice(0, 90) })))}`);
      log(`zoom panes after=${JSON.stringify(panesAfter.map((pane) => ({ id: pane.id, surface: pane.surface, camera: String(pane.camera).slice(0, 90) })))} moved=${JSON.stringify(movedPanes)}`);
      log(`zoom console tail=${JSON.stringify(consoleBuf.slice(zoomMark).filter((row) => /focus|camera|window-required|windowConfig/i.test(row)).slice(-10)).slice(0, 1000)}`);
      verdict(
        "context-menu-zoom-moves-camera",
        Boolean(cameraBefore) && cameraBefore !== cameraAfter,
        `before=${String(cameraBefore).slice(0, 100)} after=${String(cameraAfter).slice(0, 100)} movedPanes=${JSON.stringify(movedPanes)} newFaults=${faults.length - faultsBefore} newHardFaults=${hardFaults.length - hardBefore}`,
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
    const controls = await page.evaluate(() =>
      Array.from(document.querySelectorAll('[id^="panel:puzzle3d-play-document/"] *'))
        .filter((el) => el.matches('button, [role="button"], [data-slot^="tree-action"], a'))
        .map((el) => ({ tag: el.tagName.toLowerCase(), id: el.id || null, slot: el.getAttribute("data-slot"), label: el.getAttribute("aria-label") ?? el.getAttribute("title"), text: (el as HTMLElement).innerText.replace(/\n/g, " ").trim().slice(0, 24) }))
        .slice(0, 30),
    );
    log(`outliner row controls=${JSON.stringify(controls)}`);
    const hideButton = page
      .locator('[id^="panel:puzzle3d-play-document/"] button, [id^="panel:puzzle3d-play-document/"] [role="button"], [id^="panel:puzzle3d-play-document/"] [data-slot^="tree-action"]')
      .filter({ hasText: /hide|verbergen/i })
      .first();
    const hideByLabel = page.locator('[id^="panel:puzzle3d-play-document/"] [aria-label*="Hide" i], [id^="panel:puzzle3d-play-document/"] [title*="Hide" i]').first();
    const hideCount = await hideButton.count();
    const labelCount = await hideByLabel.count();
    const target = hideCount ? hideButton : hideByLabel;
    verdict("outliner-hide-control-present", hideCount + labelCount > 0, `byText=${hideCount} byLabel=${labelCount} controls=${JSON.stringify(controls).slice(0, 400)}`);
    if (!hideCount && !labelCount) return;
    // 🙈️ Three independent observables per click, so a red says WHICH hop broke: what was actually
    // clicked (an icon-only row action is easy to miss and easy to mis-hit), the object's own
    // `scale` inside `data-instances-json` — `world_instances_geometry_json` emits `[0,0,0]` for a
    // hidden object, so the WORLD lane answers even when the row text does not — and the dispatch
    // console tail (wave B20 defect 3).
    const clicked = await target.evaluate((el) => ({ tag: el.tagName.toLowerCase(), slot: el.getAttribute("data-slot"), label: el.getAttribute("aria-label") ?? el.getAttribute("title"), text: (el as HTMLElement).innerText.replace(/\n/g, " ").trim().slice(0, 40), row: el.closest('[role="treeitem"]')?.id ?? null })).catch(() => null);
    const hiddenScales = async () =>
      evalSafe(
        () =>
          Array.from(document.querySelectorAll<HTMLElement>("[data-instances-json]")).map((element) => {
            try {
              const rows = JSON.parse(element.getAttribute("data-instances-json") || "[]") as { id?: string; scale?: number[] }[];
              return { surface: element.getAttribute("data-surface-id") ?? "?", hidden: rows.filter((row) => (row.scale ?? [1, 1, 1]).every((axis) => axis === 0)).map((row) => row.id ?? "?") };
            } catch {
              return { surface: element.getAttribute("data-surface-id") ?? "?", hidden: ["parse-failed"] };
            }
          }),
        [] as { surface: string; hidden: string[] }[],
      );
    const hiddenBefore = await hiddenScales();
    const hideMark = consoleBuf.length;
    await target.click({ force: true, timeout: 4000 }).catch(() => {});
    // 🕰️ POLLED, not a fixed wait: the document mutation, its history patch and the `refreshUi` that
    // repaints the panel are three separate round trips, so a single 3 s sample cannot tell "never
    // repainted" from "had not repainted yet".
    let afterHide = await rowDump();
    for (let attempt = 0; attempt < 8 && JSON.stringify(afterHide) === JSON.stringify(before); attempt++) {
      await page.waitForTimeout(1000);
      afterHide = await rowDump();
    }
    const hiddenAfter = await hiddenScales();
    log(`outliner hide clicked=${JSON.stringify(clicked)} worldHiddenBefore=${JSON.stringify(hiddenBefore)} worldHiddenAfter=${JSON.stringify(hiddenAfter)}`);
    log(`outliner hide console tail=${JSON.stringify(consoleBuf.slice(hideMark).filter((row) => /selectionFlag|performInvocation|command ingress|ui-refresh|refreshUi|unchanged|dirty|scope|panel:|puzzle\.3d\.play\.document/i.test(row)).slice(-28)).slice(0, 4000)}`);
    const hideApplied = JSON.stringify(afterHide) !== JSON.stringify(before);
    verdict("outliner-hide-applies", hideApplied, `beforeHead=${JSON.stringify(before.slice(0, 3))} afterHead=${JSON.stringify(afterHide.slice(0, 3))} worldHidden=${JSON.stringify(hiddenAfter)} clicked=${JSON.stringify(clicked)}`);
    if (!hideApplied) {
      verdict("outliner-show-restores", false, "not reachable — the Hide row action itself never changed the row, so Show has nothing to restore");
      return;
    }
    const showButton = page
      .locator('[id^="panel:puzzle3d-play-document/"] button, [id^="panel:puzzle3d-play-document/"] [role="button"], [id^="panel:puzzle3d-play-document/"] [data-slot^="tree-action"]')
      .filter({ hasText: /show|anzeigen|einblenden/i })
      .first();
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
    const rowDump = await page.evaluate(() =>
      Array.from(document.querySelectorAll('[id^="puzzle3d-play-kinds"] [role="treeitem"], [id^="puzzle3d-play-kinds"] [data-slot="tree-item"]'))
        .map((el) => ({
          id: el.id || null,
          draggable: (el as HTMLElement).draggable,
          // 🖱️ `data-activatable` is the row's own declared activation reaching the DOM (`🌳️Tree/🟦️.tsx`,
          // `TreeItemProps.activatable`) — the one observable that separates "the guest never authored
          // the binding", "the binding was lost on the way in" and "the click was swallowed".
          activatable: el.getAttribute("data-activatable"),
          rowKind: el.getAttribute("data-tree-row-kind"),
          text: (el as HTMLElement).innerText.replace(/\n/g, " ").trim().slice(0, 50),
        }))
        .slice(0, 20),
    );
    log(`catalogue rows=${JSON.stringify(rowDump)}`);
    verdict("catalogue-kind-rows-present", rowCount > 0, `rows=${rowCount} dump=${JSON.stringify(rowDump).slice(0, 400)}`);
    if (rowCount) {
      // 🖱️ The row is EXPANDABLE (its rim-vortex templates are its children), so the click has to land
      // on the row shell rather than on the fold chevron, which stops propagation — press the row's own
      // label, and fall back to the shell. Both observables are recorded: whether the click reached the
      // action channel at all (`performInvocation actionId:"addObjectKind"`), and whether the guest
      // answered (the ingress lane, a refusal notice, a fault).
      const addMark = consoleBuf.length;
      const label = row.locator('[data-slot="tree-label"]').first();
      // 🎯️ What is actually ON TOP of the row's label, before pressing it. `click({force:true})` skips
      // the "receives pointer events" check, so an overlay covering the panel eats the press and the
      // step reads exactly like "the row is not wired" — this separates the two for good.
      const hit = await evalSafe(() => {
        const target = document.querySelector('[id^="puzzle3d-play-kinds"] [role="treeitem"] [data-slot="tree-label"]') as HTMLElement | null;
        if (!target) return null;
        const box = target.getBoundingClientRect();
        const top = document.elementFromPoint(box.x + box.width / 2, box.y + box.height / 2) as HTMLElement | null;
        return {
          label: `${target.tagName.toLowerCase()}|${target.getAttribute("data-slot")}`,
          top: top ? `${top.tagName.toLowerCase()}|${top.id || "?"}|${top.getAttribute("data-slot") ?? "?"}` : null,
          covered: Boolean(top) && !target.contains(top) && top !== target,
          activatableRow: top?.closest("[data-activatable]")?.getAttribute("data-activatable") ?? null,
          // 🧭️ The covering element's own ancestry names the overlay that swallows the press — an
          // anonymous `div` says nothing on its own, its chain says which chrome it belongs to.
          topChain: (() => {
            const chain: string[] = [];
            for (let node = top; node && chain.length < 8; node = node.parentElement) {
              chain.push(`${node.tagName.toLowerCase()}#${node.id || "-"}[${node.getAttribute("data-slot") ?? "-"}]{${(node.className || "").toString().slice(0, 60)}}`);
            }
            return chain;
          })(),
        };
      }, null as { label: string; top: string | null; covered: boolean; activatableRow: string | null; topChain: string[] } | null);
      log(`catalogue add hit-test=${JSON.stringify(hit)}`);
      await ((await countSafe(label)) ? label : row).click({ force: true, timeout: 4000 }).catch(() => {});
      let afterAdd = await dumpInstances();
      for (let attempt = 0; attempt < 8 && afterAdd.count === beforeAdd.count; attempt++) {
        await page.waitForTimeout(1000);
        afterAdd = await dumpInstances();
      }
      log(`catalogue add console tail=${JSON.stringify(consoleBuf.slice(addMark).filter((line) => /addObjectKind|performInvocation|command ingress|dropped action|refus|fault|notice|window-required|puzzle3d-add-kind/i.test(line)).slice(-20)).slice(0, 3000)}`);
      verdict("catalogue-add-object-kind", afterAdd.count > beforeAdd.count, `before=${beforeAdd.count} after=${afterAdd.count} ids=${JSON.stringify(afterAdd.ids).slice(0, 200)} hitTest=${JSON.stringify(hit).slice(0, 600)}`);
      const sel = await selectionState();
      verdict("catalogue-add-selects-new-object", afterAdd.count > beforeAdd.count && sel.selected.some((row) => afterAdd.ids.some((objectId) => row.includes(objectId))), `added=${afterAdd.count - beforeAdd.count} selected=${JSON.stringify(sel.selected).slice(0, 220)} ids=${JSON.stringify(afterAdd.ids)}`);
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
    // 🛰️ Two preconditions, both measured, both named in the verdict: something must BE selected (an
    // empty-selection focus frames the whole document — wave B11) and the camera must not already be
    // standing where the focus would put it (framing a framed pane republishes a bit-identical pose,
    // which is indistinguishable from a dropped write — wave B20). Orbit away, then press `f`.
    const focusSelected = (await worldInteraction()).flatMap((surface) => [...surface.selectedIds, ...surface.selectedInstances]);
    await orbitPerspectiveAway();
    const cameraBefore = await cameraOf("puzzle3d-main-perspective");
    await page.keyboard.press("f").catch(() => {});
    const cameraAfter = await cameraSettled("puzzle3d-main-perspective", cameraBefore, 8000);
    verdict(
      "focus-selection",
      Boolean(cameraBefore) && cameraBefore !== cameraAfter,
      `selected=${JSON.stringify(focusSelected).slice(0, 120)} before=${String(cameraBefore).slice(0, 100)} after=${String(cameraAfter).slice(0, 100)}`,
    );
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
  });

  log(`plan: ${JSON.stringify(GROUP_ORDER.map((group) => ({ group, steps: plan.filter((entry) => entry.group === group).map((entry) => entry.name) })))}`);
  let ranGroup = false;
  for (const group of GROUP_ORDER) {
    const entries = plan.filter((entry) => entry.group === group);
    if (!entries.length) continue;
    if (ranGroup && reloadBetweenGroups) {
      log(`reloading page before group ${group}`);
      await gotoShell(`reboot:${group}`);
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
    currentStep = `guest-alive-${group}`;
    currentSection = "§0";
    const vitals = await guestVitals();
    const deaths = guestDeathFaults.length;
    verdict(
      `guest-alive-${group}`,
      vitals.recovery.length === 0 && deaths === 0 && vitals.canvases >= 2 && vitals.surfaces.some((surface) => surface.instances > 0),
      `recovery=${JSON.stringify(vitals.recovery)} canvases=${vitals.canvases} surfaces=${JSON.stringify(vitals.surfaces).slice(0, 240)} guestDeathFaults=${deaths} first=${(guestDeathFaults[0] ?? "none").slice(0, 160)} firstHardFaultAt=${firstHardFaultAt ?? "none"}`,
    );
  }
  currentStep = "battery";
  currentSection = "§0";
  verdict("battery-hard-faults", hardFaults.length === 0, `hard=${hardFaults.length} collateral=${collateralFaults.length} distinct=${faultKeys.size} first=${(hardFaults[0] ?? "none").slice(0, 200)}`);
  verdict("battery-faults", faults.length === 0, `raw=${faults.length} hard=${hardFaults.length} collateral=${collateralFaults.length} distinct=${faultKeys.size}`);
  const summaryLine = `battery PASS=${passCount} FAIL=${failCount} FAULTS=${faults.length} first-hard-fault-at=${firstHardFaultAt ?? "none"} guest-death-faults=${guestDeathFaults.length}`;
  emit({
    t: Number(((Date.now() - t0) / 1000).toFixed(1)),
    ts: new Date().toISOString(),
    summary: summaryLine,
    pass: passCount,
    fail: failCount,
    firstHardFaultAt,
    guestDeathFaults: guestDeathFaults.length,
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
  `# probe ${stamp} (interact=${interact} battery=${battery} only=${onlyArg ?? "-"} reloadBetweenGroups=${reloadBetweenGroups})\n\n## verdicts\n${verdictBlock || "(none)"}\n\n## timeline\n${lines.join("\n")}\n\n## faults (raw ${faults.length}, hard ${hardFaults.length}, collateral ${collateralFaults.length}, distinct ${faultKeys.size}, first-hard-fault-at ${firstHardFaultAt ?? "none"}, guest-death ${guestDeathFaults.length})\n\n### guest death\n${guestDeathFaults.slice(0, 20).join("\n") || "(none)"}\n\n### hard\n${hardFaults.slice(0, 60).join("\n") || "(none)"}\n\n### collateral\n${collateralFaults.slice(0, 60).join("\n") || "(none)"}\n\n## console tail\n\`\`\`\n${tail}\n\`\`\`\n`,
);
log(
  `done booted=${booted} faults=${faults.length} hard=${hardFaults.length} collateral=${collateralFaults.length} first-hard-fault-at=${firstHardFaultAt ?? "none"} guest-death-faults=${guestDeathFaults.length} verdicts=${lines.filter((row) => row.includes("verdict ")).length} → 🗑️generated/probe-${stamp}.md + probe-${stamp}.ndjson`,
);
await browser.close();
process.exit(0);
