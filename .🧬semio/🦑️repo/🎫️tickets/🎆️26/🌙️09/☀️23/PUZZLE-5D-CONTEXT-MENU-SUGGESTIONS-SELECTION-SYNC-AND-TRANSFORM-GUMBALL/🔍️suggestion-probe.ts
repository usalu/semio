/** 🔍️ Headless end-to-end probe of the puzzle 🖐️5d grip context menu on the React serve (127.0.0.1:6014):
 * boots the shell, loads Concrete Forest, hovers a grip marker of the seed part in the 3D pane, right-clicks it
 * and checks, in order, that (1) the menu offers the "suggest" row as a SUBMENU, (2) the suggestion search
 * started the moment the menu opened (`suggestionMenu.submenu` on the interaction lane before any row was
 * hovered), (3) hovering the row lists the free candidates, (4) hovering one candidate focuses exactly its trace
 * record (`data-suggestion-focus`) and leaving releases it, (5) clicking it places the part (parts 1 → 2) and
 * closes the menu, and (6) Escape on a reopened menu releases the search. Also checks the Transform utility
 * (one toggle, gumball armed for a picked part) and that a 3D pick lands in the board pane's selection.
 *
 * Run: `bun 🔍️suggestion-probe.ts [--plugin=puzzle5d|puzzle3d] [--port=6014|6013]` — writes `🗑️generated/suggestion-probe-<stamp>.{md,png}`. */
import { chromium, type Page } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const TICKET = decodeURIComponent(new URL(".", import.meta.url).pathname);
const OUT = join(TICKET, "🗑️generated");
mkdirSync(OUT, { recursive: true });
const stamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19);
const port = process.argv.find((a) => a.startsWith("--port="))?.slice(7) ?? "6014";
const plugin = process.argv.find((a) => a.startsWith("--plugin="))?.slice(9) ?? "puzzle5d";
const lines: string[] = [];
const verdicts: { step: string; ok: boolean; detail: string }[] = [];
const t0 = Date.now();
const log = (m: string) => {
  const row = `[${((Date.now() - t0) / 1000).toFixed(1)}s] ${m}`;
  lines.push(row);
  console.log(row);
};
const verdict = (step: string, ok: boolean, detail: unknown) => {
  const text = typeof detail === "string" ? detail : (JSON.stringify(detail) ?? "∅");
  verdicts.push({ step, ok, detail: text });
  log(`${ok ? "PASS" : "FAIL"} ${step} — ${text.slice(0, 600)}`);
};

type WorldVitals = { interaction: Record<string, unknown>; selection: Record<string, unknown>; guest: Record<string, unknown>; vortices: { fullId: string; objectId: string; position: number[] }[]; instances: { id: string; position: number[] }[]; camera: { position: number[]; target: number[]; up?: number[]; fov?: number }; rect: { left: number; top: number; width: number; height: number }; focus: string | null; board: string[] };

const readWorld = (page: Page) =>
  page.evaluate(() => {
    const panes = [...document.querySelectorAll("[data-instances-json]")];
    const el = panes.find((pane) => (pane.getAttribute("data-viewport-camera-json") ?? "").includes('"perspective"')) ?? panes[0];
    const board = document.querySelector("[data-board-selection-json]");
    if (!el) return null;
    const parse = (name: string, fallback: unknown) => {
      try {
        return JSON.parse(el.getAttribute(name) ?? "") as unknown;
      } catch {
        return fallback;
      }
    };
    const canvas = el.querySelector("canvas") ?? el;
    const rect = canvas.getBoundingClientRect();
    return {
      interaction: parse("data-interaction-json", {}),
      selection: parse("data-selection-json", {}),
      guest: parse("data-guest-selection-json", {}),
      vortices: parse("data-vortices-json", []),
      instances: parse("data-instances-json", []),
      camera: parse("data-viewport-camera-json", { position: [0, 0, 1], target: [0, 0, 0] }),
      rect: { left: rect.left, top: rect.top, width: rect.width, height: rect.height },
      focus: el.getAttribute("data-suggestion-focus"),
      board: (() => {
        try {
          return JSON.parse(board?.getAttribute("data-board-selection-json") ?? "[]") as string[];
        } catch {
          return [];
        }
      })(),
    };
  }).catch(() => null) as Promise<WorldVitals | null>;

/** 📐️ Perspective projection of a world point through the pane's published viewport camera (z-up). */
const project = (world: WorldVitals, point: number[]) => {
  const sub = (a: number[], b: number[]) => a.map((x, i) => x - b[i]!);
  const dot = (a: number[], b: number[]) => a.reduce((s, x, i) => s + x * b[i]!, 0);
  const cross = (a: number[], b: number[]) => [a[1]! * b[2]! - a[2]! * b[1]!, a[2]! * b[0]! - a[0]! * b[2]!, a[0]! * b[1]! - a[1]! * b[0]!];
  const norm = (a: number[]) => {
    const l = Math.hypot(...a);
    return a.map((x) => x / l);
  };
  const { camera, rect } = world;
  const f = norm(sub(camera.target, camera.position));
  const right = norm(cross(f, camera.up ?? [0, 0, 1]));
  const up = cross(right, f);
  const d = sub(point, camera.position);
  const z = dot(d, f);
  const t = Math.tan(((camera.fov ?? 50) * Math.PI) / 360);
  return { x: rect.left + ((dot(d, right) / (z * t * (rect.width / rect.height)) + 1) / 2) * rect.width, y: rect.top + ((1 - dot(d, up) / (z * t)) / 2) * rect.height };
};

const waitFor = async <T,>(read: () => Promise<T>, done: (value: T) => boolean, timeoutMs = 20000) => {
  const start = Date.now();
  let value = await read();
  while (!done(value) && Date.now() - start < timeoutMs) {
    await new Promise((resolve) => setTimeout(resolve, 250));
    value = await read();
  }
  return { value, ok: done(value), waitedMs: Date.now() - start };
};

/** 🎯️ Sweeps the pointer over `points` until the world pane's own hover paint names `id`. */
const hoverUntilPainted = async (page: Page, id: string, points: { x: number; y: number }[]) => {
  for (const point of points) {
    await page.mouse.move(point.x, point.y);
    await page.waitForTimeout(60);
    const painted = await page.evaluate(() => {
      const panes = [...document.querySelectorAll("[data-instances-json]")];
      const pane = panes.find((el) => (el.getAttribute("data-viewport-camera-json") ?? "").includes('"perspective"')) ?? panes[0];
      return pane?.getAttribute("data-hover-paint-id") ?? null;
    });
    if (painted === id) return point;
  }
  return null;
};
const grid = (rect: { left: number; top: number; width: number; height: number }, step: number) => {
  const points: { x: number; y: number }[] = [];
  for (let y = rect.top + rect.height * 0.25; y < rect.top + rect.height * 0.75; y += step) for (let x = rect.left + rect.width * 0.2; x < rect.left + rect.width * 0.8; x += step) points.push({ x, y });
  return points;
};

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const faults: string[] = [];
page.on("pageerror", (error) => faults.push(String(error)));
const consoleRing: string[] = [];
page.on("console", (message) => {
  const text = `${message.type()}: ${message.text()}`;
  if (message.type() === "error" || /toolRun|tool run|brush|dropped|refus|suggestion|fault|panic/i.test(text)) consoleRing.push(text.slice(0, 400));
});
try {
  await page.goto(`http://127.0.0.1:${port}/?plugin=${plugin}`);
  const booted = await waitFor(() => readWorld(page), (world) => (world?.instances.length ?? 0) > 0, Number(process.argv.find((a) => a.startsWith("--boot-ms="))?.slice(10) ?? "300000"));
  verdict("boot", booted.ok, { waitedMs: booted.waitedMs, instances: booted.value?.instances.length });
  if (!booted.ok) {
    await page.screenshot({ path: join(OUT, `suggestion-probe-${stamp}-boot.png`) });
    const surfaces = await page.evaluate(() => [...document.querySelectorAll("[data-surface-id]")].map((el) => `${el.getAttribute("data-surface-id")} instances=${(el.getAttribute("data-instances-json") ?? "∅").slice(0, 80)}`)).catch(() => []);
    log(`  surfaces: ${JSON.stringify(surfaces)}`);
    log(`  console: ${consoleRing.slice(-30).join("\n    ")}`);
    log(`  text: ${(await page.evaluate(() => document.body.innerText.slice(0, 600)).catch(() => "")).replace(/\n/g, " | ")}`);
  }

  await waitFor(() => page.evaluate(() => document.body.innerText.includes("Verifying shared access")), (verifying) => !verifying, 60000);
  await page.waitForTimeout(6000);
  const skipTour = page.getByText("Skip", { exact: true });
  if ((await skipTour.count()) > 0) await skipTour.first().click().catch(() => undefined);
  if ((await readWorld(page))?.instances.length !== 1) {
    await page.getByRole("combobox").first().click();
    await page.getByText("Concrete Forest", { exact: true }).first().click();
  }
  const forest = await waitFor(() => readWorld(page), (world) => world?.instances.length === 1, 30000);
  verdict("load-concrete-forest", forest.ok, { instances: forest.value?.instances.length });

  const seed = forest.value!.instances[0]!;
  if (plugin === "puzzle5d") {
    // 🧰️ Transform: ONE toggle, and a picked part arms the gumball.
    const unfold = page.locator('[id$="puzzle5d3d.utilityBar.unfold"]');
    if ((await unfold.count()) > 0) await unfold.click();
    const toggles = await page.locator('[data-slot="toggle-group-item"][data-level="pane"]').evaluateAll((els) => els.map((el) => el.getAttribute("data-toggle-value")));
    verdict("transform-is-one-utility", toggles.includes("transform") && !toggles.some((id) => id === "move" || id === "rotate" || id === "scale"), toggles);
    const setTransform = async (on: boolean) => {
      const toggle = page.locator("button#transform");
      if (((await toggle.getAttribute("data-state")) === "on") !== on) await toggle.click();
      return waitFor(() => readWorld(page), (world) => (world?.interaction["activeUtility"] === "transform") === on, 10000);
    };
    verdict("transform-arms", (await setTransform(true)).ok, "activeUtility=transform");
    const seedAt = (await hoverUntilPainted(page, seed.id, grid(forest.value!.rect, 30))) ?? project(forest.value!, seed.position);
    await page.mouse.click(seedAt.x, seedAt.y);
    const picked = await waitFor(() => readWorld(page), (world) => (world?.guest["selectedIds"] as string[] | undefined)?.includes(seed.id) === true && (plugin !== "puzzle5d" || (world?.board.includes(seed.id) ?? false)) && world?.selection["gumballActive"] === true, 15000);
    verdict("3d-pick-selects-the-part", picked.ok, picked.value?.selection["selectedIds"]);
    if (plugin === "puzzle5d") verdict("3d-pick-reaches-the-board", picked.value?.board.includes(seed.id) === true, picked.value?.board);
    verdict("transform-arms-the-gumball", picked.value?.selection["gumballActive"] === true && picked.value?.selection["transformMode"] === "transform", { gumballActive: picked.value?.selection["gumballActive"], transformMode: picked.value?.selection["transformMode"] });
    await page.screenshot({ path: join(OUT, `suggestion-probe-${stamp}-gumball.png`) });
    verdict("transform-disarms", (await setTransform(false)).ok, "activeUtility≠transform");
  } else {
    const seedAt = project(forest.value!, forest.value!.camera.target);
    let picked = { value: null as WorldVitals | null, ok: false, waitedMs: 0 };
    for (let attempt = 0; attempt < 5 && !picked.ok; attempt++) {
      await page.mouse.click(seedAt.x, seedAt.y);
      picked = await waitFor(() => readWorld(page), (world) => (world?.guest["selectedIds"] as string[] | undefined)?.includes(seed.id) === true, 3000);
    }
    verdict("3d-pick-selects-the-object", picked.ok, picked.value?.guest["selectedIds"]);
    await page.screenshot({ path: join(OUT, `suggestion-probe-${stamp}-3d-pick.png`) });
  }

  // 🐁️ Hover a grip marker of the seed part.
  const world = (await readWorld(page))!;
  let hovered: string | null = null;
  let at = { x: 0, y: 0 };
  for (const grip of world.vortices.filter((row) => row.objectId === seed.id)) {
    const centre = project(world, grip.position);
    for (let ring = 0; ring <= 8 && !hovered; ring += 2) {
      for (const [dx, dy] of [[0, 0], [ring, 0], [-ring, 0], [0, ring], [0, -ring], [ring, ring], [-ring, -ring], [ring, -ring], [-ring, ring]] as const) {
        await page.mouse.move(centre.x + dx, centre.y + dy);
        const hover = await waitFor(() => readWorld(page), (w) => w?.interaction["hoveredVortexFullId"] === grip.fullId, 400);
        if (hover.ok) {
          hovered = grip.fullId;
          at = { x: centre.x + dx, y: centre.y + dy };
          break;
        }
      }
    }
    if (hovered) break;
  }
  verdict("grip-hover-publishes-hoveredVortexFullId", hovered !== null, { hovered, at, grips: world.vortices.filter((row) => row.objectId === seed.id).length, camera: world.camera });

  if (process.argv.includes("--brush-check")) {
    const brush = page.locator("button#brush").last();
    await brush.click();
    await page.mouse.move(at.x + 30, at.y + 30);
    await page.mouse.move(at.x, at.y);
    await page.waitForTimeout(6000);
    const vitals = await page.locator("[data-instances-json]").evaluate((el) => Object.fromEntries([...el.attributes].filter((a) => a.name.startsWith("data-tool-run")).map((a) => [a.name, a.value.slice(0, 120)])));
    verdict("armed-brush-starts-a-live-run", vitals["data-tool-run-run"] !== "", vitals);
    await brush.click();
  }

  // 🖱️ Right-click the grip: the menu is the GRIP's, its suggest row is a submenu, and the search starts now.
  await page.mouse.click(at.x, at.y, { button: "right" });
  const menu = page.locator('[role="menu"]').first();
  await menu.waitFor({ timeout: 10000 });
  const rows = await menu.locator("[data-menu-action]").evaluateAll((els) => els.map((el) => ({ action: el.getAttribute("data-menu-action"), submenu: el.getAttribute("aria-haspopup") === "menu", label: el.textContent?.trim() })));
  const suggestRow = rows.find((row) => row.action === "openVortexSuggestions");
  verdict("grip-menu-offers-suggest-submenu", suggestRow?.submenu === true && !rows.some((row) => row.action === "duplicateSelection"), rows);
  const started = await waitFor(() => readWorld(page), (w) => (w?.interaction["suggestionMenu"] as { submenu?: boolean; vortexFullId?: string } | null)?.vortexFullId === hovered, 8000);
  verdict("search-starts-when-the-menu-opens", started.ok && (started.value?.interaction["suggestionMenu"] as { submenu?: boolean }).submenu === true, { waitedMs: started.waitedMs, menu: started.value?.interaction["suggestionMenu"] });
  const runVitals = () => page.locator("[data-instances-json]").first().evaluate((el) => Object.fromEntries([...el.attributes].filter((a) => a.name.startsWith("data-tool-run")).map((a) => [a.name, a.value.slice(0, 200)])));
  log(`  tool run vitals at open: ${JSON.stringify(await runVitals())}`);
  await page.waitForTimeout(5000);
  log(`  tool run vitals +5s: ${JSON.stringify(await runVitals())}`);
  log(`  console: ${consoleRing.slice(-25).join("\n    ")}`);
  log(`  tool-run panel: ${(await page.locator('[data-slot="tool-run-panel"], [data-tool-run-panel]').allInnerTexts().catch(() => [])).join(" | ").slice(0, 400)}`);
  const found = await waitFor(() => readWorld(page), (w) => ((w?.interaction["suggestionMenu"] as { candidates?: unknown[] } | null)?.candidates?.length ?? 0) > 0, 30000);
  const candidates = ((found.value?.interaction["suggestionMenu"] as { candidates?: { index: number; key: number; objectLabel: string }[] } | null)?.candidates ?? []);
  verdict("search-finds-candidates-before-the-row-is-hovered", found.ok, { waitedMs: found.waitedMs, candidates });

  // 📂️ Hovering the row opens the live list.
  await menu.locator('[data-menu-action="openVortexSuggestions"]').hover();
  const list = page.locator('[data-menu-action="acceptSuggestion"]');
  await list.first().waitFor({ timeout: 10000 }).catch(() => undefined);
  const listed = await list.count();
  verdict("row-hover-lists-the-candidates", listed === Math.min(candidates.length, 8) && listed > 0, { listed, candidates: candidates.length });
  await page.screenshot({ path: join(OUT, `suggestion-probe-${stamp}-submenu.png`) });

  // 🔦️ Hovering one candidate focuses exactly its trace record; the next one moves the focus.
  await list.nth(0).hover();
  const focus0 = await waitFor(() => readWorld(page), (w) => w?.focus === String(candidates[0]?.key), 5000);
  verdict("candidate-hover-focuses-its-record", focus0.ok, { focus: focus0.value?.focus, expected: candidates[0]?.key });
  await page.screenshot({ path: join(OUT, `suggestion-probe-${stamp}-focus-0.png`) });
  if (listed > 1) {
    await list.nth(1).hover();
    const focus1 = await waitFor(() => readWorld(page), (w) => w?.focus === String(candidates[1]?.key), 5000);
    verdict("next-candidate-moves-the-focus", focus1.ok, { focus: focus1.value?.focus, expected: candidates[1]?.key });
    await page.screenshot({ path: join(OUT, `suggestion-probe-${stamp}-focus-1.png`) });
  }

  // ✅️ Clicking places the part and closes everything.
  await list.nth(0).click();
  const placed = await waitFor(() => readWorld(page), (w) => w?.instances.length === 2 && w.interaction["suggestionMenu"] === null, 30000);
  verdict("accept-places-the-part-and-closes-the-menu", placed.ok && (await page.locator('[role="menu"]').count()) === 0, { instances: placed.value?.instances.length, menu: placed.value?.interaction["suggestionMenu"], focus: placed.value?.focus });
  const reselected = await waitFor(() => readWorld(page), (w) => ((w?.guest["selectedIds"] as string[] | undefined) ?? []).some((id) => id !== seed.id), 10000);
  verdict("accept-selects-the-placed-part", reselected.ok, { selectedIds: reselected.value?.guest["selectedIds"], waitedMs: reselected.waitedMs });
  await page.screenshot({ path: join(OUT, `suggestion-probe-${stamp}-placed.png`) });

  // 🔒️ Escape on a reopened menu releases the search.
  await page.mouse.move(at.x + 40, at.y + 40);
  await page.mouse.move(at.x, at.y);
  await waitFor(() => readWorld(page), (w) => typeof w?.interaction["hoveredVortexFullId"] === "string", 3000);
  await page.mouse.click(at.x, at.y, { button: "right" });
  await waitFor(() => readWorld(page), (w) => w?.interaction["suggestionMenu"] !== null, 8000);
  await page.keyboard.press("Escape");
  const closed = await waitFor(() => readWorld(page), (w) => w?.interaction["suggestionMenu"] === null, 8000);
  verdict("escape-releases-the-search", closed.ok, { menu: closed.value?.interaction["suggestionMenu"] });

  // ◻️ Board pane: the SAME grip menu on a board handle — submenu, live candidates, board-twin focus.
  const board = page.locator("[data-board-handle-positions-json]").first();
  if ((await board.count()) === 0) throw new Error("skip-board");
  const boardRect = await board.evaluate((el) => { const r = el.getBoundingClientRect(); return { left: r.left, top: r.top, width: r.width, height: r.height }; });
  const boardCamera = JSON.parse((await page.locator("[data-board-camera-json]").first().getAttribute("data-board-camera-json")) ?? "{}") as { readonly x?: number; readonly y?: number; readonly zoom?: number };
  const positions = JSON.parse((await board.getAttribute("data-board-handle-positions-json")) ?? "{}") as { readonly rows?: readonly [string, number, number, string, string, boolean][] };
  const seedHandle = (positions.rows ?? []).find((row) => row[3] === seed.id && row[5]);
  verdict("board-publishes-seed-handles", seedHandle !== undefined, { handles: positions.rows?.length, seedHandle });
  if (seedHandle) {
    const zoom = boardCamera.zoom || 1;
    const client = { x: boardRect.left + (seedHandle[1] - (boardCamera.x ?? 0)) * zoom + boardRect.width / 2, y: boardRect.top + (seedHandle[2] - (boardCamera.y ?? 0)) * zoom + boardRect.height / 2 };
    await page.mouse.move(client.x + 20, client.y + 20);
    await page.mouse.move(client.x, client.y);
    await page.waitForTimeout(800);
    log(`  board at handle: ${JSON.stringify(await board.evaluate((el) => Object.fromEntries([...el.attributes].filter((a) => /hover|interaction|selection/i.test(a.name)).map((a) => [a.name, a.value.slice(0, 160)]))))} rect=${JSON.stringify(boardRect)} client=${JSON.stringify(client)}`);
    await page.mouse.click(client.x, client.y, { button: "right" });
    const boardMenu = page.locator('[role="menu"]').first();
    await boardMenu.waitFor({ timeout: 10000 });
    const boardRows = await boardMenu.locator("[data-menu-action]").evaluateAll((els) => els.map((el) => ({ action: el.getAttribute("data-menu-action"), submenu: el.getAttribute("aria-haspopup") === "menu" })));
    verdict("board-grip-menu-offers-suggest-submenu", boardRows.some((row) => row.action === "openVortexSuggestions" && row.submenu), boardRows);
    const boardMenuRecord = () => page.locator("[data-board-suggestion-menu-json]").first().getAttribute("data-board-suggestion-menu-json").then((text) => (text ? (JSON.parse(text) as { submenu?: boolean; handleId?: string; candidates?: { key: string }[] }) : null)).catch(() => null);
    const listedOnBoard = await waitFor(boardMenuRecord, (record) => (record?.candidates?.length ?? 0) > 0, 30000);
    verdict("board-search-starts-and-finds-candidates", listedOnBoard.ok && listedOnBoard.value?.submenu === true, { waitedMs: listedOnBoard.waitedMs, handleId: listedOnBoard.value?.handleId, candidates: listedOnBoard.value?.candidates?.length });
    await boardMenu.locator('[data-menu-action="openVortexSuggestions"]').hover();
    const boardList = page.locator('[data-menu-action="acceptSuggestion"]');
    await boardList.first().waitFor({ timeout: 10000 }).catch(() => undefined);
    verdict("board-row-hover-lists-the-candidates", (await boardList.count()) > 0, { listed: await boardList.count() });
    for (const at of [0, 1].filter((index) => index < (listedOnBoard.value?.candidates?.length ?? 0))) {
      await boardList.nth(at).hover();
      const expectedKey = listedOnBoard.value?.candidates?.[at]?.key;
      const boardFocus = await waitFor(() => board.getAttribute("data-suggestion-focus").catch(() => null), (focus) => focus !== null && expectedKey !== undefined && focus === expectedKey, 5000);
      verdict(`board-candidate-${at}-hover-focuses-its-own-twin`, boardFocus.ok, { focus: boardFocus.value, expected: expectedKey });
    }
    await page.screenshot({ path: join(OUT, `suggestion-probe-${stamp}-board-focus.png`) });
    await page.keyboard.press("Escape");
    await page.waitForTimeout(300);
    log(`  after 1st Escape: menus=${await page.locator('[role="menu"]').count()}`);
    await page.keyboard.press("Escape");
    await page.waitForTimeout(300);
    log(`  after 2nd Escape: menus=${await page.locator('[role="menu"]').count()} focused=${await page.evaluate(() => document.activeElement?.outerHTML.slice(0, 120))}`);
    const boardClosed = await waitFor(boardMenuRecord, (record) => record === null, 8000);
    verdict("board-escape-releases-the-search", boardClosed.ok, boardClosed.value);
  }
} catch (error) {
  if (!String(error).includes("skip-board")) verdict("probe", false, String(error));
} finally {
  verdict("no-page-errors", faults.length === 0, faults.slice(0, 5));
  const pass = verdicts.filter((row) => row.ok).length;
  const summary = `PASS=${pass} FAIL=${verdicts.length - pass}`;
  log(summary);
  writeFileSync(join(OUT, `suggestion-probe-${stamp}.md`), `# 🔍️ Suggestion probe ${stamp}\n\n${summary}\n\n\`\`\`\n${lines.join("\n")}\n\`\`\`\n`);
  await browser.close();
}
