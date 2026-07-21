import { chromium } from "playwright";

const url = "http://127.0.0.1:6023/";
const browser = await chromium.launch({
  executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
  headless: true,
});
const page = await browser.newPage();
page.on("console", (msg) => {
  const t = msg.text();
  if (/instances|object|mesh|error|Error|DEBUG|example|fixture/i.test(t)) console.log(`[console.${msg.type()}] ${t}`);
});
page.on("pageerror", (e) => console.log(`[pageerror] ${e}`));
page.on("crash", () => console.log("[CRASH]"));

await page.addInitScript(() => {
  const orig = console.log;
  // Patch fetch to log setActiveExample-ish traffic is hard; instead monkeypatch JSON.parse of large payloads later.
  (window as any).__capture = { responses: [] as string[] };
});

await page.route("**/*", async (route) => {
  await route.continue();
});

await page.goto(url, { waitUntil: "networkidle", timeout: 180_000 });
await page.waitForTimeout(8000);

// Walk React fiber from canvas to find props with instancesJson
const sceneProbe = await page.evaluate(() => {
  const canvas = document.querySelector("canvas");
  if (!canvas) return { error: "no canvas" };
  const key = Object.keys(canvas).find((k) => k.startsWith("__reactFiber$") || k.startsWith("__reactInternalInstance$"));
  // Also search all elements for fiber with instancesJson
  const found: any[] = [];
  const visit = (fiber: any, depth: number) => {
    if (!fiber || depth > 80 || found.length > 5) return;
    const props = fiber.memoizedProps || fiber.pendingProps;
    if (props && typeof props === "object") {
      if (typeof props.instancesJson === "string") {
        found.push({
          instancesJson: props.instancesJson.slice(0, 500),
          instancesLen: props.instancesJson.length,
          vorticesJson: typeof props.vorticesJson === "string" ? props.vorticesJson.slice(0, 300) : null,
          vorticesLen: typeof props.vorticesJson === "string" ? props.vorticesJson.length : null,
          cameraJson: typeof props.cameraJson === "string" ? props.cameraJson : null,
          meshes: Array.isArray(props.meshes) ? props.meshes.length : props.meshes,
        });
      }
      if (props.scene && typeof props.scene === "object" && typeof props.scene.instancesJson === "string") {
        found.push({
          via: "scene",
          instancesJson: props.scene.instancesJson.slice(0, 500),
          instancesLen: props.scene.instancesJson.length,
          vorticesLen: typeof props.scene.vorticesJson === "string" ? props.scene.vorticesJson.length : null,
          cameraJson: props.scene.cameraJson,
        });
      }
    }
    visit(fiber.child, depth + 1);
    visit(fiber.sibling, depth + 1);
  };
  const rootKey = Object.keys(document.getElementById("root") || {}).find((k) => k.startsWith("__reactContainer$") || k.startsWith("__reactFiber$"));
  const rootEl = document.getElementById("root") as any;
  if (rootEl && rootKey) {
    const fiber = rootEl[rootKey];
    visit(fiber.stateNode ? fiber : fiber, 0);
    // react 19 container
    if (fiber?.current) visit(fiber.current, 0);
  }
  // fallback: scan all nodes
  if (found.length === 0) {
    document.querySelectorAll("*").forEach((el) => {
      const k = Object.keys(el).find((x) => x.startsWith("__reactFiber$"));
      if (k) visit((el as any)[k], 0);
    });
  }
  return { foundCount: found.length, found: found.slice(0, 3) };
});
console.log(`[DEBUG] sceneProbe=${JSON.stringify(sceneProbe, null, 2)}`);

// Switch to Nakagin via navbar select if possible
await page.getByRole("button", { name: "Überspringen" }).click().catch(() => {});
await page.waitForTimeout(500);

// Use keyboard/accessibility: find combobox
const selectInfo = await page.evaluate(() => {
  const triggers = [...document.querySelectorAll("button, [role='combobox']")].map((el) => ({
    role: el.getAttribute("role"),
    text: (el.textContent || "").trim().slice(0, 80),
    aria: el.getAttribute("aria-label"),
  }));
  return triggers.filter((t) => /Beispiel|Abbau|Nakagin|Example|concrete/i.test(t.text + (t.aria || ""))).slice(0, 20);
});
console.log(`[DEBUG] selectInfo=${JSON.stringify(selectInfo, null, 2)}`);

await browser.close();
