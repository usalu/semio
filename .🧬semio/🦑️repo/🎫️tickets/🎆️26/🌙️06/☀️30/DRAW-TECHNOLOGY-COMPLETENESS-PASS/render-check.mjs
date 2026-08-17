import { chromium } from "playwright";

const url = "http://127.0.0.1:6064/";
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
const logs = [];
page.on("console", (msg) => logs.push(msg.text()));

try {
  await page.goto(url, { waitUntil: "networkidle", timeout: 90_000 });
  await page.waitForTimeout(3000);
  const fixtureSelect = page.locator("#playground\\.navbar\\.fixture");
  if (await fixtureSelect.count()) {
    await fixtureSelect.selectOption("semio");
    await page.waitForTimeout(6000);
  }

  const canvasPaths = await page.locator(".bg-neutral-950 svg path").evaluateAll((nodes) =>
    nodes.map((node) => {
      const path = node;
      const box = path.getBBox();
      return {
        fill: path.getAttribute("fill"),
        stroke: path.getAttribute("stroke"),
        dLen: path.getAttribute("d")?.length ?? 0,
        x: box.x,
        y: box.y,
        width: box.width,
        height: box.height,
      };
    }),
  );

  const filled = canvasPaths.filter((row) => row.fill && row.fill !== "none");
  const stroked = canvasPaths.filter((row) => row.stroke && row.stroke !== "none");
  const maxFilledX = Math.max(0, ...filled.map((row) => row.x + row.width));
  const maxStrokeX = Math.max(0, ...stroked.map((row) => row.x + row.width));
  const maxFilledY = Math.max(0, ...filled.map((row) => row.y + row.height));
  const maxStrokeY = Math.max(0, ...stroked.map((row) => row.y + row.height));
  const alignmentDelta = Math.max(Math.abs(maxFilledX - maxStrokeX), Math.abs(maxFilledY - maxStrokeY));
  const hugeStrokePaths = stroked.filter((row) => row.dLen > 5000).length;
  const authoredFills = new Set(["rgba(250,149,0,1)", "rgba(255,52,79,1)", "rgba(52,209,191,1)"]);
  const unexpectedFilledPaths = filled.filter((row) => !authoredFills.has(row.fill)).length;
  const ok = canvasPaths.length === 5 && filled.length === 5 && stroked.length === 5 && unexpectedFilledPaths === 0 && alignmentDelta < 2 && hugeStrokePaths === 0;

  console.log(
    JSON.stringify(
      {
        canvasPathCount: canvasPaths.length,
        filledPathCount: filled.length,
        strokedPathCount: stroked.length,
        hugeStrokePaths,
        unexpectedFilledPaths,
        maxFilledX,
        maxStrokeX,
        maxFilledY,
        maxStrokeY,
        alignmentDelta,
        ok,
      },
      null,
      2,
    ),
  );

  process.exitCode = ok ? 0 : 1;
} finally {
  await browser.close();
}
