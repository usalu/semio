/** 🔬️ F3 — which React components render on a hover transition: a minimal `__REACT_DEVTOOLS_GLOBAL_HOOK__` installed before
 * React loads receives every committed root of every renderer (react-dom and the R3F reconciler); per commit it walks the
 * fiber tree and counts fibers that performed work (PerformedWork flag) by renderer + component/host type. Then the same
 * hover run as f3-hover.ts. usage: bun f3-commit-census.ts <baseUrl> <pluginId> <appId> <windowSuffix> [n] */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { awaitBeacon, dismissIntroduction, windowIds } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";
const [baseUrl, pluginId, appId, windowSuffix, nArg] = process.argv.slice(2) as [string, string, string, string, string?];
const hovers = Number(nArg ?? 6);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
try {
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
  await page.addInitScript(() => {
    const census = { armed: false, commits: 0, byRenderer: {} as Record<string, number>, counts: {} as Record<string, number>, roots: {} as Record<string, number>, hookDiffs: {} as Record<string, number>, rendererNames: {} as Record<number, string> };
    Object.defineProperty(window, "__f3Census", { value: census });
    let next = 1;
    const nameOf = (type: unknown): string => typeof type === "string" ? `<${type}>` : typeof type === "function" ? ((type as { displayName?: string; name?: string }).displayName ?? (type as { name?: string }).name ?? "anon") : type && typeof type === "object" ? `${String((type as { $$typeof?: symbol }).$$typeof?.description ?? "obj")}:${nameOf((type as { type?: unknown; render?: unknown }).type ?? (type as { render?: unknown }).render)}` : String(type);
    (window as unknown as { __REACT_DEVTOOLS_GLOBAL_HOOK__: unknown }).__REACT_DEVTOOLS_GLOBAL_HOOK__ = {
      supportsFiber: true,
      renderers: new Map(),
      inject(renderer: { rendererPackageName?: string }) { const id = next++; census.rendererNames[id] = renderer.rendererPackageName ?? `renderer${id}`; return id; },
      onScheduleFiberRoot() {},
      onCommitFiberUnmount() {},
      onPostCommitFiberRoot() {},
      checkDCE() {},
      onCommitFiberRoot(id: number, root: { current: { child: unknown } }) {
        if (!census.armed) return;
        census.commits += 1;
        const renderer = census.rendererNames[id] ?? String(id);
        const stack: unknown[] = [root.current];
        while (stack.length) {
          const fiber = stack.pop() as { type: unknown; flags: number; child: unknown; sibling: unknown; alternate: unknown; memoizedProps: unknown } | null;
          if (!fiber) continue;
          if ((fiber.flags & 1) && fiber.type) {
            const key = `${renderer} ${nameOf(fiber.type)}`;
            census.counts[key] = (census.counts[key] ?? 0) + 1;
            census.byRenderer[renderer] = (census.byRenderer[renderer] ?? 0) + 1;
            let parent = (fiber as unknown as { return: { flags: number; type: unknown; return: unknown } | null }).return;
            while (parent && typeof parent.type !== "function" && !(parent.type && typeof parent.type === "object" && ((parent.type as { render?: unknown }).render || (parent.type as { type?: unknown }).type))) parent = parent.return as typeof parent;
            if (typeof fiber.type === "function" && (!parent || !(parent.flags & 1))) {
              const alternate = fiber.alternate as { memoizedProps: unknown; memoizedState: unknown } | null;
              let cause = "mount";
              if (alternate) {
                const changedProps = alternate.memoizedProps !== fiber.memoizedProps ? Object.keys((fiber.memoizedProps ?? {}) as object).filter((name) => (fiber.memoizedProps as Record<string, unknown>)[name] !== (alternate.memoizedProps as Record<string, unknown>)[name]) : [];
                const hookTypes = ((fiber as unknown as { _debugHookTypes?: string[] })._debugHookTypes ?? []).filter((type) => type !== "useContext" && type !== "useDebugValue");
                const changedHooks: string[] = [];
                let a = (fiber as unknown as { memoizedState: { memoizedState: unknown; next: unknown } | null }).memoizedState, b = alternate.memoizedState as { memoizedState: unknown; next: unknown } | null, index = 0;
                const brief = (value: unknown, depth = 0): string => {
                  if (value === null || typeof value !== "object") return typeof value === "function" ? "fn" : String(value).slice(0, 40);
                  if (Array.isArray(value)) return depth > 1 ? `[${value.length}]` : `[${value.slice(0, 3).map((item) => brief(item, depth + 1)).join(",")}${value.length > 3 ? ",…" : ""}]`;
                  if (value instanceof Map || value instanceof Set) return `${value.constructor.name}(${value.size})`;
                  const keys = Object.keys(value as object);
                  return depth > 1 ? `{${keys.length}}` : `{${keys.slice(0, 8).map((key) => `${key}:${brief((value as Record<string, unknown>)[key], depth + 1)}`).join(",")}${keys.length > 8 ? ",…" : ""}}`;
                };
                const differing = (left: unknown, right: unknown): string => {
                  if (!left || !right || typeof left !== "object" || typeof right !== "object" || Array.isArray(left)) return "";
                  return Object.keys(right as object).filter((key) => (left as Record<string, unknown>)[key] !== (right as Record<string, unknown>)[key]).slice(0, 10).join(",");
                };
                while (a && b) {
                  if (a.memoizedState !== b.memoizedState && ["useState", "useReducer", "useSyncExternalStore", "useContext"].includes(hookTypes[index] ?? "")) {
                    changedHooks.push(`${hookTypes[index]}#${index}`);
                    if (nameOf(fiber.type) === "FrameworkOsShellInner" || nameOf(fiber.type) === "World3dHost") {
                      const sampleKey = `${nameOf(fiber.type)} ${hookTypes[index]}#${index}`;
                      const samples = ((census as unknown as { samples: Record<string, string[]> }).samples ??= {});
                      const list = (samples[sampleKey] ??= []);
                      const top = differing(b.memoizedState, a.memoizedState);
                      const deeper = top.split(",").filter(Boolean).map((key) => {
                        const left = (b.memoizedState as Record<string, unknown>)[key], right = (a.memoizedState as Record<string, unknown>)[key];
                        const inner = differing(left, right);
                        const third = inner.split(",").filter(Boolean).slice(0, 4).map((sub) => `${sub}{${differing((left as Record<string, unknown>)[sub], (right as Record<string, unknown>)[sub])}}=${brief((right as Record<string, unknown>)[sub], 1).slice(0, 160)}`);
                        return `${key}.[${third.join(" | ")}]`;
                      });
                      const deepKey = `${sampleKey} ${deeper.join(" ; ").slice(0, 60)}`;
                      const deepList = (samples[deepKey] ??= []);
                      if (deepList.length < 2) deepList.push(deeper.join(" ; ").slice(0, 900));
                    }
                  }
                  a = a.next as typeof a; b = b.next as typeof b; index += 1;
                }
                cause = `props[${changedProps.slice(0, 6).join(",")}] hooks[${changedHooks.slice(0, 6).join(",")}]`;
              }
              const rootKey = `${renderer} ${nameOf(fiber.type)} ← ${parent ? nameOf(parent.type) : "root"} :: ${cause}`;
              census.roots[rootKey] = (census.roots[rootKey] ?? 0) + 1;
            }
          }
          if (fiber.child) stack.push(fiber.child);
          if (fiber.sibling) stack.push(fiber.sibling);
        }
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
  const host = page.locator(`[id$="${windowSuffix}"] .semio-world-3d-host`).first();
  const box = (await host.boundingBox())!;
  let target: { x: number; y: number } | null = null;
  for (let gy = 0.3; gy <= 0.8 && !target; gy += 0.05) for (let gx = 0.2; gx <= 0.8 && !target; gx += 0.05) {
    const x = box.x + box.width * gx, y = box.y + box.height * gy;
    await page.mouse.move(x, y);
    await page.waitForTimeout(120);
    if (await page.evaluate((suffix) => document.querySelector(`[id$="${suffix}"] .semio-world-3d-host`)!.getAttribute("data-hover-paint-id"), windowSuffix)) target = { x, y };
  }
  if (!target) throw new Error("no hoverable point found");
  const empty = { x: box.x + 8, y: box.y + box.height - 60 };
  await page.mouse.move(empty.x, empty.y);
  await page.waitForTimeout(2_000);
  await page.evaluate(() => { (window as unknown as { __f3Census: { armed: boolean } }).__f3Census.armed = true; });
  for (let index = 0; index < hovers; index += 1) {
    await page.mouse.move(target.x, target.y);
    await page.waitForTimeout(500);
    await page.mouse.move(empty.x, empty.y);
    await page.waitForTimeout(500);
  }
  const census = await page.evaluate(() => { const state = (window as unknown as { __f3Census: { armed: boolean; commits: number; byRenderer: Record<string, number>; counts: Record<string, number>; roots: Record<string, number>; rendererNames: Record<number, string> } }).__f3Census; state.armed = false; return state; });
  const transitions = hovers * 2;
  const top = Object.entries(census.counts).sort((a, b) => b[1] - a[1]).slice(0, 60).map(([key, count]) => `${(count / transitions).toFixed(1)}/transition ${key}`);
  const roots = Object.entries(census.roots).sort((a, b) => b[1] - a[1]).slice(0, 40).map(([key, count]) => `${(count / transitions).toFixed(2)}/transition ${key}`);
  const report = { samples: (census as unknown as { samples?: Record<string, string[]> }).samples ?? {}, roots, appId, transitions, commits: census.commits, renderers: census.rendererNames, perTransitionByRenderer: Object.fromEntries(Object.entries(census.byRenderer).map(([key, value]) => [key, +(value / transitions).toFixed(1)])), top };
  writeFileSync(`/Users/ueli/Documents/semio/.tmp-ticket/wp-f3/generated/f3-commit-census-${pluginId}.json`, JSON.stringify(report, null, 1));
  console.log(JSON.stringify(report, null, 1));
} finally {
  await browser.close();
}
