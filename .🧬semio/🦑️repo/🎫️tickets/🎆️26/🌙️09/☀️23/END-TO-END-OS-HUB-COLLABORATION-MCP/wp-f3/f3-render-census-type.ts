/** 🔬️ F3 — TYPING variant of `f3-render-census.ts` (per key instead of per hover transition; phase label stays "hover"):
 * which React components ACTUALLY render per commit, idle vs. typing, with React DevTools' own semantics
 * (`didFiberRender` + "identical child pointer = bailed-out subtree, do not descend"): the first census
 * (`f3-commit-census.ts`) read the PerformedWork flag of every fiber in the current tree, but a bailed-out subtree keeps
 * its fibers from an earlier commit, flags included — it counted the whole mounted chrome on every commit.
 * usage: bun f3-render-census-type.ts <baseUrl> <pluginId> <appId> <windowSuffix> <tag> [keys/2] [idleMs] */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { loadavg } from "node:os";
import { awaitBeacon, dismissIntroduction, windowIds } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";
const [baseUrl, pluginId, appId, windowSuffix, tag, hoversArg, idleArg] = process.argv.slice(2) as [string, string, string, string, string, string?, string?];
const hovers = Number(hoversArg ?? 6);
const idleMs = Number(idleArg ?? 4_000);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
type Phase = { commits: Record<string, number>; fibers: Record<string, number>; counts: Record<string, number>; roots: Record<string, number>; samples: Record<string, number> };
try {
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
  await page.addInitScript(() => {
    const census = { phase: "", phases: {} as Record<string, Phase>, rendererNames: {} as Record<number, string> };
    Object.defineProperty(window, "__f3Render", { value: census });
    let next = 1;
    const nameOf = (type: unknown): string => typeof type === "string" ? `<${type}>` : typeof type === "function" ? ((type as { displayName?: string; name?: string }).displayName ?? (type as { name?: string }).name ?? "anon") : type && typeof type === "object" ? `${String((type as { $$typeof?: symbol }).$$typeof?.description ?? "obj")}:${nameOf((type as { type?: unknown; render?: unknown }).type ?? (type as { render?: unknown }).render)}` : String(type);
    type Fiber = { tag: number; type: unknown; flags: number; child: Fiber | null; sibling: Fiber | null; alternate: Fiber | null; return: Fiber | null; memoizedProps: unknown; memoizedState: { memoizedState: unknown; next: unknown } | null; _debugHookTypes?: string[] };
    const COMPONENT_TAGS = new Set([0, 1, 9, 11, 14, 15]);
    const SAMPLED = new Set(["FrameworkOsShellInner", "World3dHost", "WorldOrbitViewGizmo", "WorldLodGridHelper", "LodFrameRunner", "WorldAutoFit", "WorldVortexHitStamp"]);
    const same = (left: unknown, right: unknown): boolean => { try { const l = JSON.stringify(left), r = JSON.stringify(right); return l !== undefined && l.length < 200_000 && l === r; } catch { return false; } };
    const diffOf = (left: unknown, right: unknown, depth: number): string => {
      if (!left || !right || typeof left !== "object" || typeof right !== "object" || Array.isArray(left) || left instanceof Map || left instanceof Set) return same(left, right) ? "=json" : "≠";
      const keys = Object.keys(right as object).filter((key) => (left as Record<string, unknown>)[key] !== (right as Record<string, unknown>)[key]);
      return `{${keys.slice(0, 8).map((key) => `${key}${depth < 2 ? diffOf((left as Record<string, unknown>)[key], (right as Record<string, unknown>)[key], depth + 1) : same((left as Record<string, unknown>)[key], (right as Record<string, unknown>)[key]) ? "=json" : "≠"}`).join(",")}}`;
    };
    const leafDiff = (left: unknown, right: unknown, path: string, out: string[]): string[] => {
      if (out.length >= 12 || left === right) return out;
      if (!left || !right || typeof left !== "object" || typeof right !== "object") { out.push(`${path}: ${JSON.stringify(left)?.slice(0, 80)} → ${JSON.stringify(right)?.slice(0, 80)}`); return out; }
      const keys = new Set([...Object.keys(left as object), ...Object.keys(right as object)]);
      for (const key of keys) leafDiff((left as Record<string, unknown>)[key], (right as Record<string, unknown>)[key], `${path}.${key}`, out);
      return out;
    };
    const brief = (value: unknown, depth = 0): string => {
      if (value === null || typeof value !== "object") return typeof value === "function" ? "fn" : JSON.stringify(value)?.slice(0, 40) ?? String(value);
      if (Array.isArray(value)) return depth > 1 ? `[${value.length}]` : `[${value.slice(0, 3).map((item) => brief(item, depth + 1)).join(",")}${value.length > 3 ? ",…" : ""}]`;
      if (value instanceof Map || value instanceof Set) return `${value.constructor.name}(${value.size})`;
      const keys = Object.keys(value as object);
      return depth > 1 ? `{${keys.length}}` : `{${keys.slice(0, 6).map((key) => `${key}:${brief((value as Record<string, unknown>)[key], depth + 1)}`).join(",")}${keys.length > 6 ? ",…" : ""}}`;
    };
    const rendered = (fiber: Fiber): boolean => fiber.alternate === null || (COMPONENT_TAGS.has(fiber.tag) && (fiber.flags & 1) === 1);
    (window as unknown as { __REACT_DEVTOOLS_GLOBAL_HOOK__: unknown }).__REACT_DEVTOOLS_GLOBAL_HOOK__ = {
      supportsFiber: true,
      renderers: new Map(),
      inject(renderer: { rendererPackageName?: string }) { const id = next++; census.rendererNames[id] = renderer.rendererPackageName ?? `renderer${id}`; return id; },
      onScheduleFiberRoot() {},
      onCommitFiberUnmount() {},
      onPostCommitFiberRoot() {},
      checkDCE() {},
      onCommitFiberRoot(id: number, root: { current: Fiber }) {
        if (!census.phase) return;
        const phase = (census.phases[census.phase] ??= { commits: {}, fibers: {}, counts: {}, roots: {}, samples: {}, edges: {}, renderMs: {}, shellMs: 0, shellRenders: 0 });
        const renderer = census.rendererNames[id] ?? String(id);
        phase.commits[renderer] = (phase.commits[renderer] ?? 0) + 1;
        const timing = phase as unknown as { renderMs: Record<string, number>; shellMs: number; shellRenders: number };
        timing.renderMs[renderer] = (timing.renderMs[renderer] ?? 0) + ((root.current as unknown as { actualDuration?: number }).actualDuration ?? 0);
        const visit = (fiber: Fiber, parentRendered: boolean): void => {
          for (let cursor: Fiber | null = fiber; cursor; cursor = cursor.sibling) {
            const did = COMPONENT_TAGS.has(cursor.tag) && rendered(cursor);
            if (did) {
              phase.fibers[renderer] = (phase.fibers[renderer] ?? 0) + 1;
              if (nameOf(cursor.type) === "FrameworkOsShellInner") {
                const timing = phase as unknown as { shellMs: number; shellRenders: number };
                timing.shellMs += (cursor as unknown as { actualDuration?: number }).actualDuration ?? 0;
                timing.shellRenders += 1;
              }
              const key = `${renderer} ${nameOf(cursor.type)}`;
              phase.counts[key] = (phase.counts[key] ?? 0) + 1;
              const prior = cursor.alternate;
              if (prior && parentRendered) {
                const nextProps = (cursor.memoizedProps ?? {}) as Record<string, unknown>, lastProps = (prior.memoizedProps ?? {}) as Record<string, unknown>;
                const changed = Object.keys(nextProps).filter((name) => name !== "children" && nextProps[name] !== lastProps[name]);
                const childrenChanged = nextProps.children !== lastProps.children;
                let ownOwner = cursor.return;
                while (ownOwner && !COMPONENT_TAGS.has(ownOwner.tag)) ownOwner = ownOwner.return;
                const edge = `${renderer} ${nameOf(cursor.type)} ← ${ownOwner ? nameOf(ownOwner.type) : "root"} :: ${changed.length === 0 ? (childrenChanged ? "children-only" : "equal-props") : `props[${changed.slice(0, 5).join(",")}]`}`;
                (phase as unknown as { edges: Record<string, number> }).edges[edge] = ((phase as unknown as { edges: Record<string, number> }).edges[edge] ?? 0) + 1;
              }
              if (!parentRendered) {
                const previous = cursor.alternate;
                let cause = "mount";
                if (previous) {
                  const props = previous.memoizedProps !== cursor.memoizedProps ? Object.keys((cursor.memoizedProps ?? {}) as object).filter((name) => (cursor!.memoizedProps as Record<string, unknown>)[name] !== (previous.memoizedProps as Record<string, unknown>)[name]) : [];
                  const hookTypes = (cursor._debugHookTypes ?? []).filter((type) => type !== "useContext" && type !== "useDebugValue");
                  const hooks: string[] = [];
                  let a = cursor.memoizedState, b = previous.memoizedState, index = 0, stateIndex = -1;
                  while (a && b) {
                    const queue = (a as unknown as { queue?: { lastRenderedReducer?: unknown; getSnapshot?: unknown; lastRenderedState?: unknown } | null }).queue;
                    if (queue) stateIndex += 1;
                    if (a.memoizedState !== b.memoizedState && queue) {
                      const kind = queue.getSnapshot ? "uSES" : "state";
                      hooks.push(`${kind}@${stateIndex}`);
                      if (SAMPLED.has(nameOf(cursor.type))) {
                        const sample = `${nameOf(cursor.type)} ${kind}@${stateIndex} ${diffOf(b.memoizedState, a.memoizedState, 0)} ← ${brief(a.memoizedState)}`.slice(0, 700);
                        phase.samples[sample] = (phase.samples[sample] ?? 0) + 1;
                        const leftUi = (b.memoizedState as { spawnedWindow?: { spawnedWindowUiByWindowId?: unknown } })?.spawnedWindow?.spawnedWindowUiByWindowId, rightUi = (a.memoizedState as { spawnedWindow?: { spawnedWindowUiByWindowId?: unknown } })?.spawnedWindow?.spawnedWindowUiByWindowId;
                        if (leftUi && rightUi && leftUi !== rightUi) for (const path of leafDiff(leftUi, rightUi, "", [])) phase.samples[`ui-leaf ${path}`.slice(0, 400)] = (phase.samples[`ui-leaf ${path}`.slice(0, 400)] ?? 0) + 1;
                        if (leftUi && rightUi && leftUi !== rightUi) for (const [windowId, node] of Object.entries(rightUi as Record<string, { component?: { doc?: { bytes?: Record<string, number> | number[] } } }>)) {
                          const before = (leftUi as Record<string, typeof node>)[windowId]?.component?.doc?.bytes, after = node?.component?.doc?.bytes;
                          if (!before || !after || before === after) continue;
                          const la = Object.values(before), ra = Object.values(after);
                          let first = 0;
                          while (first < ra.length && la[first] === ra[first]) first += 1;
                          const text = (bytes: number[]) => bytes.slice(Math.max(0, first - 200), first + 60).map((code) => code >= 32 && code < 127 ? String.fromCharCode(code) : ".").join("");
                          const key = `ui-bytes ${windowId} len ${la.length}→${ra.length} first@${first}\n  L ${text(la)}\n  R ${text(ra)}`;
                          phase.samples[key] = (phase.samples[key] ?? 0) + 1;
                        }
                      }
                    }
                    a = a.next as typeof a; b = b.next as typeof b; index += 1;
                  }
                  cause = `props[${props.slice(0, 6).join(",")}] hooks[${hooks.slice(0, 6).join(",")}]${props.length === 0 && hooks.length === 0 ? " context" : ""}`;
                }
                let owner = cursor.return;
                while (owner && !COMPONENT_TAGS.has(owner.tag)) owner = owner.return;
                const rootKey = `${renderer} ${nameOf(cursor.type)} ← ${owner ? nameOf(owner.type) : "root"} :: ${cause}`;
                phase.roots[rootKey] = (phase.roots[rootKey] ?? 0) + 1;
              }
            }
            const previous = cursor.alternate;
            if (cursor.child && !(previous && previous.child === cursor.child)) visit(cursor.child, did || (parentRendered && !COMPONENT_TAGS.has(cursor.tag)));
          }
        };
        visit(root.current, false);
      },
    };
  });
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  await awaitBeacon(page, Date.now() + 300_000);
  await dismissIntroduction(page);
  const before = await windowIds(page);
  await page.keyboard.press("Meta+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 15_000 });
  await input.fill(/^s\.[^.]+\.([^@]+)@/u.exec(appId)![1]!);
  await page.waitForTimeout(1_500);
  for (const id of [`spawn.${pluginId}.${appId}`, `spawn.${pluginId}`]) {
    const item = page.locator(`[data-slot="command-item"][data-command-item-id="${id}"]`).first();
    if ((await item.count()) > 0) { await item.click({ force: true }); break; }
  }
  const deadline = Date.now() + 120_000;
  while (Date.now() < deadline && (await windowIds(page)).filter((id) => !before.includes(id)).length === 0) await page.waitForTimeout(500);
  await page.waitForTimeout(8_000);
  const editor = page.locator(`[id$="${windowSuffix}"] [data-slot="window-body"] .semio-text-editor-host canvas`).first();
  await editor.waitFor({ state: "visible", timeout: 60_000 });
  const box = (await editor.boundingBox())!;
  await page.mouse.click(box.x + box.width * 0.5, box.y + box.height * 0.4);
  await page.waitForTimeout(800);
  await page.keyboard.press("End");
  await page.waitForTimeout(2_000);
  const arm = (phase: string) => page.evaluate((name) => { (window as unknown as { __f3Render: { phase: string } }).__f3Render.phase = name; }, phase);
  await arm("idle");
  await page.waitForTimeout(idleMs);
  await arm("hover");
  for (let index = 0; index < hovers * 2; index += 1) {
    await page.keyboard.press(index % 2 === 0 ? "a" : "Backspace");
    await page.waitForTimeout(180);
  }
  await page.waitForTimeout(1_000);
  await arm("");
  const census = await page.evaluate(() => (window as unknown as { __f3Render: { phases: Record<string, Phase>; rendererNames: Record<number, string> } }).__f3Render);
  const transitions = hovers * 2;
  const per = (phase: Phase | undefined, divisor: number, unit: string) => phase ? {
    renderMs: Object.fromEntries(Object.entries((phase as unknown as { renderMs: Record<string, number> }).renderMs).map(([key, value]) => [key, +(value / divisor).toFixed(1)])),
    shellRenders: +((phase as unknown as { shellRenders: number }).shellRenders / divisor).toFixed(2),
    shellMs: +((phase as unknown as { shellMs: number }).shellMs / divisor).toFixed(1),
    commits: Object.fromEntries(Object.entries(phase.commits).map(([key, value]) => [key, +(value / divisor).toFixed(2)])),
    fibers: Object.fromEntries(Object.entries(phase.fibers).map(([key, value]) => [key, +(value / divisor).toFixed(1)])),
    top: Object.entries(phase.counts).sort((a, b) => b[1] - a[1]).slice(0, 40).map(([key, count]) => `${(count / divisor).toFixed(2)}/${unit} ${key}`),
    roots: Object.entries(phase.roots).sort((a, b) => b[1] - a[1]).slice(0, 40).map(([key, count]) => `${(count / divisor).toFixed(2)}/${unit} ${key}`),
    edges: Object.entries((phase as unknown as { edges: Record<string, number> }).edges).sort((a, b) => b[1] - a[1]).slice(0, 80).map(([key, count]) => `${(count / divisor).toFixed(2)}/${unit} ${key}`),
    samples: Object.entries(phase.samples).sort((a, b) => b[1] - a[1]).slice(0, 80).map(([key, count]) => `${(count / divisor).toFixed(2)}/${unit} ${key}`),
  } : null;
  const report = { tag, appId, windowSuffix, load: +loadavg()[0]!.toFixed(1), renderers: census.rendererNames, idle: { ms: idleMs, perSecond: per(census.phases.idle, idleMs / 1000, "s") }, hover: { transitions, perTransition: per(census.phases.hover, transitions, "transition") } };
  writeFileSync(`/Users/ueli/Documents/semio/.tmp-ticket/wp-f3/generated/f3-render-census-type-${pluginId}-${tag}.json`, JSON.stringify(report, null, 1));
  console.log(JSON.stringify({ load: report.load, idle: { commits: report.idle.perSecond?.commits, fibers: report.idle.perSecond?.fibers }, hover: { commits: report.hover.perTransition?.commits, fibers: report.hover.perTransition?.fibers, renderMs: report.hover.perTransition?.renderMs, shellRenders: report.hover.perTransition?.shellRenders, shellMs: report.hover.perTransition?.shellMs } }));
} finally {
  await browser.close();
}
