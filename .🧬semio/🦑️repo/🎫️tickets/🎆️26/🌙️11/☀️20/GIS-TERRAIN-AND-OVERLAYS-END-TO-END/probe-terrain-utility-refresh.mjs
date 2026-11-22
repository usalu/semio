import { chromium } from "playwright";

const variant = process.argv[2] ?? "body";
const browser = await chromium.launch({ channel: "chrome", headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader"] });
try {
  const page = await browser.newPage({ locale: "en-US", viewport: { width: 1100, height: 760 } });
  page.on("pageerror", error => console.log("[DEBUG] page", error.stack));
  page.on("console", message => {
    const value = message.text();
    if (value.startsWith("[DEBUG]") || value.includes("wasm trap") || value.includes("surface-render")) console.log(value);
  });
  await page.route("**/*", async route => {
    const url = decodeURIComponent(route.request().url());
    if (variant === "trace" && url.includes("ShellHost/🔁️contributions-refresh/") && url.includes(".ts")) {
      const response = await route.fetch();
      const body = await response.text();
      const marker = /uiRefreshLaneRef\.current\.request\(\{\s*session:/;
      if (marker.test(body)) console.log("[DEBUG] instrumented refresh queue");
      return route.fulfill({ response, body: body.replace(marker, 'uiRefreshLaneRef.current.request({ ...(console.log("[DEBUG] refresh queue", scopeArg.kind, Object.values(activeUtilityByWindowIdRef.current), new Error().stack), {}), session:') });
    }
    if (variant === "publications" && url.includes("ShellHost/⚡️host-effects/📡️operation-subscriptions/") && url.includes(".ts")) {
      const response = await route.fetch();
      const body = await response.text();
      const progress = /\(progress\) => \{\s*const \{ scope, historyPatch \}/g;
      const completion = /\(completion\) => \{\s*settleOperation/g;
      return route.fulfill({ response, body: body
        .replace(progress, '(progress) => { console.log("[DEBUG] progress publication", JSON.stringify(progress)); const { scope, historyPatch }')
        .replace(completion, '(completion) => { console.log("[DEBUG] completion publication", JSON.stringify(completion)); settleOperation') });
    }
    if (["no-measure-utility", "no-measure-roster", "scoped-measures"].includes(variant) && url.includes("PluginRuntime/🖼️surface-refresh/") && url.includes(".ts")) {
      const response = await route.fetch();
      const body = await response.text();
      const marker = "sectionViewContext(request.viewState)";
      if (body.includes(marker)) console.log("[DEBUG] instrumented measure view input");
      const change = variant === "no-measure-utility"
        ? "activeUtilityByWindowId: {}"
        : variant === "no-measure-roster"
          ? "windowInstances: []"
          : "windowId: request.viewState.windowInstances?.[0]?.id, activeWindowKindId: request.viewState.windowInstances?.[0]?.windowKindId";
      return route.fulfill({ response, body: body.replace(marker, `sectionViewContext(section.key === "measures" ? { ...request.viewState, ${change} } : request.viewState)`) });
    }
    if (!url.includes("PluginRuntime/🧩️module-loader/") || !url.includes(".ts")) return route.continue();
    const response = await route.fetch();
    const body = await response.text();
    const source = /const events = uiRefreshSurfaceEvents\(instanceId, request,/;
    if (!source.test(body)) {
      console.log("[DEBUG] module request instrumenter skipped", url);
      return route.fulfill({ response, body });
    }
    const tool = variant === "body-tools" ? "request.tools" : "undefined";
    const measure = variant === "body-measures" ? "request.measures" : "undefined";
    const alteration = variant.startsWith("body") ? `request = { ...request, panels: [], forms: [], engagements: undefined, measures: ${measure}, tools: ${tool}, catalogue: undefined, labels: undefined };` : "";
    console.log("[DEBUG] instrumented module-loader", variant);
    return route.fulfill({
      response,
      body: body.replace(source, `if (Object.values(request.viewState.activeUtilityByWindowId ?? {}).includes("terrainRaise")) {
        console.log("[DEBUG] terrain refresh request", JSON.stringify({ windows: request.windows, viewState: { utility: request.viewState.activeUtilityByWindowId, windowId: request.viewState.windowId, windowKind: request.viewState.activeWindowKindId, scopes: Object.keys(request).filter(key => request[key] != null) } }));
        ${alteration}
      }
      const events = uiRefreshSurfaceEvents(instanceId, request,`),
    });
  });
  await page.goto("http://127.0.0.1:6083/?example=demo", { waitUntil: "domcontentloaded", timeout: 30_000 });
  const viewport = page.locator(".semio-world-3d-host");
  await viewport.locator('[data-slot="terrain-feature-list"] li').first().waitFor({ state: "attached", timeout: 45_000 });
  const action = page.getByRole("button", { name: "Raise Terrain", exact: true });
  if (!(await action.isVisible())) await page.locator('[id$=".utilityBar.unfold"]').click();
  if (variant === "settled") await page.waitForTimeout(2500);
  await action.click({ timeout: 10000 });
  await page.waitForFunction(() => document.querySelector(".semio-world-3d-host")?.getAttribute("data-interaction-json")?.includes("terrainBrush"), null, { timeout: 12000 }).catch(() => {});
  const final = await viewport.getAttribute("data-interaction-json");
  console.log("[DEBUG] terrain interaction", final);
  console.log("[DEBUG] visible refusals", (await page.locator('[role="alert"]').allTextContents()).filter(value=>value.includes("trap")||value.includes("refused")));
} finally {
  await browser.close();
}
