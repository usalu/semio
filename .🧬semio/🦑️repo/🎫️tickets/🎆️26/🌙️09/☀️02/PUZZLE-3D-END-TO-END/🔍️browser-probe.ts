/** 🔬️ Headless runtime probe for the puzzle 3d React serve on 127.0.0.1:6013 — boots the shell,
 * waits for windows, optionally runs interaction steps, and writes findings + screenshots into
 * `🗑️generated/`. Ticket 26/09/02/PUZZLE-3D-END-TO-END. Run: `bun 🔍️browser-probe.ts [--interact]`. */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const TICKET = import.meta.dir;
const OUT = join(TICKET, "🗑️generated");
mkdirSync(OUT, { recursive: true });
const stamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19);
const interact = process.argv.includes("--interact");
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

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, acceptDownloads: true });
const port = process.argv.find((a) => a.startsWith("--port="))?.slice(7) ?? "6013";
await page.context().grantPermissions(["clipboard-read", "clipboard-write"], { origin: `http://127.0.0.1:${port}` });
page.on("console", (msg) => {
  const text = msg.text().slice(0, 400);
  if (consoleBuf.length < 4000) consoleBuf.push(`${msg.type()}: ${text}`);
  if (FAULT_RE.test(text) && faults.length < 200) faults.push(text);
});
page.on("pageerror", (err) => faults.push(`pageerror: ${String(err).slice(0, 400)}`));

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

let booted = false;
for (let i = 0; i < 60; i++) {
  await page.waitForTimeout(3000);
  const s = await snapshot();
  if (s.dialogs.length && i % 3 === 0) {
    const skip = page.locator('[role="dialog"] button', { hasText: /skip/i }).first();
    if (await skip.count()) {
      await skip.click({ timeout: 2000 }).catch(() => {});
      log("skipped intro dialog");
    }
  }
  if (s.windows.length >= 2 && s.canvases >= 2) {
    log(`booted: windows=${JSON.stringify(s.windows)} canvases=${s.canvases} treeItems=${s.treeItems}`);
    booted = true;
    break;
  }
  if (i % 5 === 4) log(`waiting… windows=${s.windows.length} canvases=${s.canvases} faults=${faults.length}`);
}
await page.screenshot({ path: join(OUT, `probe-${stamp}-boot.png`) }).catch(() => {});

if (booted) {
  const skip = page.locator("button", { hasText: /^\s*(x\s*)?skip\s*$/i }).first();
  if (await skip.count()) {
    await skip.click({ timeout: 3000 }).catch(() => {});
    await page.waitForTimeout(1500);
    log("dismissed welcome tour");
  }
}

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
  const extraFlags = ["--clipboard", "--marquee", "--importexport", "--locked", "--brush", "--gumball", "--suggestions", "--undo", "--selection"].some((f) => process.argv.includes(f));
  const exampleArg = process.argv.find((a) => a.startsWith("--example="))?.slice(10);
  const wantUndo = process.argv.includes("--undo");
  const wantExample = !!exampleArg || wantUndo || !extraFlags;
  const dismissChrome = async () => {
    await page.keyboard.press("Escape").catch(() => {});
    const collapse = page.locator("button", { hasText: /collapse/i }).first();
    if (await collapse.count()) await collapse.click({ timeout: 2000 }).catch(() => {});
    await page.keyboard.press("Escape").catch(() => {});
  };
  if (!extraFlags) {
  await step("activate-perspective", async () => {
    const c = page.locator("canvas").last();
    await c.click({ position: { x: 200, y: 200 }, timeout: 5000 });
  });
  await step("pick-object", async () => {
    const c = page.locator("canvas").last();
    await c.click({ position: { x: 470, y: 420 }, timeout: 5000 });
  });
  await step("context-menu", async () => {
    const c = page.locator("canvas").last();
    await c.click({ position: { x: 470, y: 420 }, button: "right", timeout: 5000 });
  });
  }
  if (wantExample) {
  await step("example-switch", async () => {
    const sel = page.locator("select").first();
    if (await sel.count()) await sel.selectOption({ index: 1 });
    else {
      const combo = page.locator('[role="combobox"]').first();
      await combo.click({ timeout: 3000 });
      const options = page.locator('[role="option"]');
      const target = exampleArg
        ? options.filter({ hasText: new RegExp(exampleArg, "i") }).first()
        : process.argv.includes("--undo")
          ? options.filter({ hasText: /nakagin/i }).first()
          : options.nth(1);
      await target.click({ timeout: 3000 });
    }
    const settleArg = process.argv.find((a) => a.startsWith("--settle="))?.slice(9);
    await page.waitForTimeout(settleArg ? Number(settleArg) * 1000 : 20000);
  });
  }
  if (process.argv.includes("--fill")) {
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
    await step("tool-category", async () => {
      const buttons = await page.locator('button[id="framework.category.tool"]').count();
      log(`tool-category buttons=${buttons}`);
      await page.locator('button[id="framework.category.tool"]').first().click({ timeout: 5000 });
      await page.waitForTimeout(1500);
      log(`fill state after category: ${JSON.stringify(await fillState()).slice(0, 500)}`);
    });
    await step("fill-tab", async () => {
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
    await step("fill-wait-ready", async () => {
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
    await step("fill-apply-max", async () => {
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
      await persp.hover({ position: s, timeout: 4000 });
      await page.waitForTimeout(150);
      await persp.click({ position: s, timeout: 5000 });
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
      if (!btn) return { frameBtn: 0, pe: "", w: 0, h: 0, x: 0, y: 0, instanceCount };
      const cs = getComputedStyle(btn);
      const r = btn.getBoundingClientRect();
      return { frameBtn: 1, pe: cs.pointerEvents, w: Math.round(r.width), h: Math.round(r.height), x: Math.round(r.x), y: Math.round(r.y), instanceCount };
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
    const menuAction = ordinal === "4" ? "exportFixture" : "importFixture";
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
  if (process.argv.includes("--fill")) {
    await step("fill-history", async () => {
      await openHistory();
      log(`fill history after apply: ${JSON.stringify(await historyState()).slice(0, 1600)}`);
    });
  }
  if (process.argv.includes("--undo")) {
    await step("history-open", async () => {
      await openHistory();
      log(`history state: ${JSON.stringify(await historyState()).slice(0, 1600)}`);
    });
    await step("undo-once", async () => {
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
  if (process.argv.includes("--selection")) {
    await step("selection-surfaces", async () => {
      await dismissChrome();
      const inspection = page.locator("#framework.panel.inspection, button").filter({ hasText: /^inspection$/i }).first();
      if (await inspection.count()) await inspection.click({ timeout: 4000 }).catch(() => {});
      await page.waitForTimeout(600);
      log(`selection before: ${JSON.stringify(await selectionState()).slice(0, 1000)}`);
      const hit = await clickForestTable();
      log(`selection clicks box=${JSON.stringify(hit.box)} spots=${JSON.stringify(hit.spots)}`);
      await page.waitForTimeout(1500);
      const debug = await page.evaluate(() => (window as unknown as { __abDebug?: string[] }).__abDebug ?? []);
      log(`selection debug: ${JSON.stringify(debug)}`);
      log(`selection after: ${JSON.stringify(await selectionState()).slice(0, 1200)}`);
      log(`selection chrome: ${JSON.stringify(await chromeState()).slice(0, 800)}`);
    });
  }
  if (process.argv.includes("--clipboard")) {
    await step("clipboard-copy-paste", async () => {
      await dismissChrome();
      await clickForestTable();
      await page.waitForTimeout(800);
      const debug = await page.evaluate(() => (window as unknown as { __abDebug?: string[] }).__abDebug ?? []);
      log(`clipboard pick debug: ${JSON.stringify(debug)}`);
      const before = await chromeState();
      log(`clipboard before: ${JSON.stringify(before).slice(0, 800)}`);
      const copyBtn = page.locator("button, [role='menuitem']").filter({ hasText: /copy/i }).first();
      log(`copyBtn=${await copyBtn.count()}`);
      if (await copyBtn.count()) await copyBtn.click({ force: true, timeout: 3000 }).catch(() => {});
      await page.keyboard.press("Meta+c");
      await page.waitForTimeout(600);
      await page.keyboard.press("Control+c");
      await page.waitForTimeout(800);
      const afterCopy = await page.evaluate(() => (window as unknown as { __abDebug?: string[] }).__abDebug ?? []);
      log(`clipboard after-copy debug: ${JSON.stringify(afterCopy)}`);
      await page.keyboard.press("Meta+v");
      await page.waitForTimeout(600);
      await page.keyboard.press("Control+v");
      await page.waitForTimeout(1500);
      const exec = page.locator("#framework.window.puzzle3dMainPerspective.action.paste.execute, button", { hasText: /^execute$/i }).first();
      if (await exec.count()) await exec.click({ force: true, timeout: 4000 }).catch(() => {});
      await page.waitForTimeout(3000);
      await openHistory();
      log(`clipboard after: ${JSON.stringify(await chromeState()).slice(0, 800)}`);
      log(`clipboard history: ${JSON.stringify(await historyState()).slice(0, 800)}`);
    });
  }
  if (process.argv.includes("--marquee")) {
    await step("marquee-drag", async () => {
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
    await step("marquee-click", async () => {
      const c = page.locator("canvas").last();
      await c.click({ position: { x: 470, y: 420 }, timeout: 5000 });
      log(`marquee after click: ${JSON.stringify(await chromeState()).slice(0, 800)}`);
    });
  }
  if (process.argv.includes("--importexport")) {
    await step("export-import", async () => {
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
      await page.waitForTimeout(600);
      await dismissChrome();
      const [chooser] = await Promise.all([
        page.waitForEvent("filechooser", { timeout: 10000 }).catch(() => null),
        activateWorkspaceMenuOrdinal("5"),
      ]);
      log(`import chooser=${chooser ? "yes" : "none"}`);
      if (chooser) {
        await chooser.setFiles(dest);
        log("import setFiles export json");
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
      log(`history after: ${JSON.stringify(await historyState()).slice(0, 800)}`);
      log(`importexport after: ${JSON.stringify(await chromeState()).slice(0, 1000)}`);
    });
  }
  if (process.argv.includes("--locked")) {
    await step("locked-refusal", async () => {
      await dismissChrome();
      const c = page.locator("canvas").last();
      await c.click({ position: { x: 470, y: 420 }, timeout: 5000 });
      await page.waitForTimeout(800);
      const inspection = page.locator("#framework.panel.inspection").first();
      if (await inspection.count()) await inspection.click({ timeout: 4000 }).catch(() => {});
      await page.waitForTimeout(800);
      const lock = page.locator("button, [role='switch'], [role='checkbox']").filter({ hasText: /lock/i }).first();
      const lockCount = await page.locator("button, [role='switch'], [role='checkbox'], [id*='locked']").filter({ hasText: /lock/i }).count();
      const lockIds = await page.evaluate(() => Array.from(document.querySelectorAll("[id*='locked'], [id*='lock']")).map((e) => e.id).slice(0, 20));
      log(`lock controls=${lockCount} ids=${JSON.stringify(lockIds)}`);
      if (await lock.count()) await lock.click({ force: true, timeout: 4000 }).catch(() => {});
      else {
        const byId = page.locator("[id*='object.locked'], [id*='locked']").first();
        if (await byId.count()) await byId.click({ force: true, timeout: 4000 }).catch(() => {});
      }
      const box = await c.boundingBox();
      if (box) {
        await page.mouse.move(box.x + box.width * 0.55, box.y + box.height * 0.45);
        await page.mouse.down();
        await page.mouse.move(box.x + box.width * 0.7, box.y + box.height * 0.45, { steps: 8 });
        await page.mouse.up();
      }
      await page.waitForTimeout(2000);
      log(`locked after: ${JSON.stringify(await chromeState()).slice(0, 1000)}`);
    });
  }
  if (process.argv.includes("--brush") || process.argv.includes("--suggestions") || process.argv.includes("--frame")) {
    await step("frame-perspective", async () => {
      await dismissChrome();
      await frameForestTable();
    });
  }
  if (process.argv.includes("--brush")) {
    await step("brush-stroke", async () => {
      await dismissChrome();
      const before = await selectionState();
      log(`brush census before: ${JSON.stringify(before).slice(0, 500)}`);
      await unfoldPerspectiveUtilities();
      const brush = page.locator("#brush, button, [data-slot='toggle-group-item']").filter({ hasText: /^brush$/i }).first();
      const brushById = page.locator("#brush").first();
      log(`brush buttons=${await page.locator("button, [data-slot='toggle-group-item']").filter({ hasText: /brush/i }).count()} id=${await brushById.count()}`);
      if (await brushById.count()) await brushById.click({ timeout: 4000 }).catch(() => {});
      else if (await brush.count()) await brush.click({ timeout: 4000 }).catch(() => {});
      await page.waitForTimeout(600);
      const framed = await frameForestTable();
      await page.screenshot({ path: join(OUT, `probe-${stamp}-brush-framed.png`) }).catch(() => {});
      const c = page.locator("canvas").last();
      const box = framed.box;
      const table = framed.table;
      await clickVortexHits(box);
      const markerSpots = [
        [0, 0], [-40, 10], [-80, 20], [-120, 30], [30, -20], [50, 10],
        [-60, -15], [-100, 5], [20, 35], [-140, 15], [-30, 40], [70, -8],
      ];
      for (const [dx, dy] of markerSpots) {
        await c.hover({ position: { x: table.x + dx, y: table.y + dy }, timeout: 4000 }).catch(() => {});
        await page.waitForTimeout(80);
        await page.mouse.click(box.x + table.x + dx, box.y + table.y + dy);
        await page.waitForTimeout(160);
      }
      await page.waitForTimeout(2500);
      await openHistory();
      log(`brush census after: ${JSON.stringify(await selectionState()).slice(0, 800)}`);
      log(`brush history: ${JSON.stringify(await historyState()).slice(0, 800)}`);
      log(`brush after: ${JSON.stringify(await chromeState()).slice(0, 1000)}`);
    });
  }
  if (process.argv.includes("--gumball")) {
    await step("gumball-drag", async () => {
      await dismissChrome();
      const c = page.locator("canvas").last();
      await c.click({ position: { x: 470, y: 420 }, timeout: 5000 });
      await page.waitForTimeout(800);
      const box = await c.boundingBox();
      if (box) {
        await page.mouse.move(box.x + box.width * 0.52, box.y + box.height * 0.48);
        await page.mouse.down();
        await page.mouse.move(box.x + box.width * 0.62, box.y + box.height * 0.48, { steps: 8 });
        await page.mouse.up();
      }
      await page.waitForTimeout(2000);
      await openHistory();
      log(`gumball after: ${JSON.stringify(await chromeState()).slice(0, 800)}`);
      log(`gumball history: ${JSON.stringify(await historyState()).slice(0, 800)}`);
    });
  }
  if (process.argv.includes("--suggestions")) {
    await step("suggestions-open", async () => {
      await dismissChrome();
      const framed = await frameForestTable();
      await page.screenshot({ path: join(OUT, `probe-${stamp}-suggestions-framed.png`) }).catch(() => {});
      const c = page.locator("canvas").last();
      const box = framed.box;
      const table = framed.table;
      const onscreen = await clickVortexHits(box);
      if (onscreen[0]) {
        await c.hover({ position: { x: onscreen[0].sx ?? table.x, y: onscreen[0].sy ?? table.y }, timeout: 4000 }).catch(() => {});
        await page.waitForTimeout(200);
      }
      const spots = [
        { x: table.x, y: table.y },
        { x: table.x - 40, y: table.y + 10 },
        { x: table.x - 80, y: table.y + 20 },
        { x: table.x - 120, y: table.y + 25 },
        { x: table.x + 30, y: table.y - 15 },
        { x: table.x - 60, y: table.y - 10 },
        { x: table.x + 20, y: table.y + 30 },
        { x: table.x - 100, y: table.y + 5 },
      ];
      for (const s of spots) {
        await c.hover({ position: s, timeout: 4000 }).catch(() => {});
        await page.waitForTimeout(120);
      }
      await page.keyboard.down("Alt");
      await page.mouse.click(box.x + table.x, box.y + table.y, { button: "right" });
      await page.keyboard.up("Alt");
      await page.waitForTimeout(1500);
      const menus = await page.locator('[role="menu"]').count();
      const suggestRows = await page.locator('[role="menuitem"], button, [data-slot="tree-item"]').filter({ hasText: /suggest/i }).count();
      log(`suggestions menus=${menus} suggestRows=${suggestRows}`);
      log(`suggestions after: ${JSON.stringify(await chromeState()).slice(0, 1200)}`);
    });
  }
}

const tail = consoleBuf.slice(-120).join("\n");
writeFileSync(
  join(OUT, `probe-${stamp}.md`),
  `# probe ${stamp} (interact=${interact})\n\n## timeline\n${lines.join("\n")}\n\n## faults (${faults.length})\n${faults.slice(0, 100).join("\n")}\n\n## console tail\n\`\`\`\n${tail}\n\`\`\`\n`,
);
log(`done booted=${booted} faults=${faults.length} → 🗑️generated/probe-${stamp}.md`);
await browser.close();
process.exit(0);
