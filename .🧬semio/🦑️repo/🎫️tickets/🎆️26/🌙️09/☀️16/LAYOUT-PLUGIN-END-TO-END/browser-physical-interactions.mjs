import assert from "node:assert/strict";
import sharp from "sharp";

const center = ([x, y, w, h]) => [x + w / 2, y + h / 2];
const sleep = (page, milliseconds) => page.waitForTimeout(milliseconds);
const closeEnough = (a, b, slack) => Math.abs(a - b) <= slack;
const readChrome = page => page.evaluate(async () => JSON.parse(await window.semioWgpuIntrospection.dumpChrome()));

async function until(page, label, observe, accepts, timeout = 20000) {
  const end = Date.now() + timeout;
  let value;
  do {
    value = await observe();
    if (accepts(value)) return value;
    await sleep(page, 125);
  } while (Date.now() < end);
  assert.fail(`${label} did not appear: ${JSON.stringify(value).slice(0, 1600)}`);
}

async function sceneBounds(page, owner) {
  const document = await page.evaluate(async id => JSON.parse(await window.semioWgpuIntrospection.dumpStructure(id)), owner);
  assert.equal(document.windowId, owner);
  const scenes = document.nodes.filter(node => node.kind === "componentScene" && node.visible && node.path.includes("blueprint"));
  assert.equal(scenes.length, 1, "one visible Blueprint scene");
  return scenes[0].rect;
}

async function rgb(bytes) {
  return sharp(bytes).removeAlpha().raw().toBuffer({ resolveWithObject: true });
}

/** 🎯️ Reads the actual X-arrow pixels painted when the retained renderer arms the manipulator. */
export function visibleHorizontalHandle(before, after, rect) {
  const { width, height, channels } = after.info;
  assert.deepEqual([before.info.width, before.info.height, before.info.channels], [width, height, channels]);
  const [left, top, w, h] = rect;
  const x0 = Math.max(0, Math.ceil(left));
  const y0 = Math.max(0, Math.ceil(top));
  const x1 = Math.min(width, Math.floor(left + w));
  const y1 = Math.min(height, Math.floor(top + h));
  let arrow = null;
  for (let y = y0; y < y1; y++) {
    let count = 0;
    for (let x = x0; x < x1; x++) {
      const at = (y * width + x) * channels;
      const red = after.data[at], green = after.data[at + 1], blue = after.data[at + 2];
      const changed = Math.max(Math.abs(red - before.data[at]), Math.abs(green - before.data[at + 1]), Math.abs(blue - before.data[at + 2])) >= 32;
      const newRed = changed && red > green + 32 && red > blue + 32;
      count = newRed ? count + 1 : 0;
      if (count >= 40 && (arrow === null || count > arrow.count)) arrow = { x: x - 9, y, count };
    }
  }
  return arrow;
}

/** 🖼️ Counts rendered changes inside the measured canvas region. */
export function changedPaintPixels(before, after, rect, threshold) {
  const { width, height, channels } = after.info;
  assert.deepEqual([before.info.width, before.info.height, before.info.channels], [width, height, channels]);
  const [left, top, w, h] = rect;
  let changed = 0;
  for (let y = Math.max(0, Math.ceil(top)); y < Math.min(height, Math.floor(top + h)); y++) {
    for (let x = Math.max(0, Math.ceil(left)); x < Math.min(width, Math.floor(left + w)); x++) {
      const at = (y * width + x) * channels;
      if (Math.max(
        Math.abs(after.data[at] - before.data[at]),
        Math.abs(after.data[at + 1] - before.data[at + 1]),
        Math.abs(after.data[at + 2] - before.data[at + 2]),
      ) >= threshold) changed++;
    }
  }
  return changed;
}

/** 🟦️ Reads the distinct chart colors from the exact linked PNG. */
export async function linkedProxyColors(bytes) {
  const { data, info: { channels } } = await rgb(bytes);
  assert.equal(channels, 3);
  const distinct = new Map();
  for (let offset = 0; offset < data.length; offset += channels) {
    const color = [...data.subarray(offset, offset + channels)];
    if (Math.max(...color) - Math.min(...color) >= 48) distinct.set(color.join(","), color);
  }
  assert.equal(distinct.size, 2, "the Linked Asset has two distinct chart colors");
  return [...distinct.values()];
}

export function countLinkedProxyColors(paint, rect, colors, tolerance, before) {
  const { width, height, channels } = paint.info;
  const [left, top, w, h] = rect;
  const counts = colors.map(() => 0);
  const matches = (data, at, color) => color.every((value, channel) => Math.abs(value - data[at + channel]) <= tolerance);
  if (before) assert.deepEqual([before.info.width, before.info.height, before.info.channels], [width, height, channels]);
  for (let y = Math.max(0, Math.ceil(top)); y < Math.min(height, Math.floor(top + h)); y++) {
    for (let x = Math.max(0, Math.ceil(left)); x < Math.min(width, Math.floor(left + w)); x++) {
      const at = (y * width + x) * channels;
      for (const [index, color] of colors.entries()) {
        if (matches(paint.data, at, color) && (!before || !matches(before.data, at, color))) counts[index]++;
      }
    }
  }
  return counts;
}

export async function observeLinkedProxy({ page, owner, bytes, tolerance, minimumColorPixels }) {
  const palette = await linkedProxyColors(bytes);
  const rect = await sceneBounds(page, owner);
  const capture = async () => rgb(await page.screenshot());
  const enough = counts => counts.every(count => count >= minimumColorPixels);
  const before = await until(page, "original linked chart is visible in Blueprint", async () => {
    const paint = await capture();
    return { paint, counts: countLinkedProxyColors(paint, rect, palette, tolerance) };
  }, ({ counts }) => enough(counts));
  return async ([x, y]) => {
    assert.deepEqual(await sceneBounds(page, owner), rect, "placement keeps the measured Blueprint viewport");
    const [left, top, width, height] = rect;
    const target = [x, y, left + width - x, top + height - y];
    const placed = await until(page, "placed linked chart colors are visible in Blueprint", async () =>
      countLinkedProxyColors(await capture(), target, palette, tolerance, before.paint), enough);
    return { palette, original: before.counts, placed };
  };
}

const scopeActions = (chrome, cursor, verb) => chrome.actions.filter(row => row.seq > cursor && row.action === verb);
function actionPayload(row) {
  assert.equal(typeof row.args, "string", "renderer publishes native arguments");
  return JSON.parse(row.args);
}

/** 📍️ Locates the active horizontal handle by comparing two frames of the same live Blueprint. */
export async function locateHorizontalGumball(page, owner, setTransform) {
  await setTransform(false);
  const rect = await sceneBounds(page, owner);
  const inactive = await rgb(await page.screenshot());
  await setTransform(true);
  return until(page, "painted X-gumball handle", async () => visibleHorizontalHandle(inactive, await rgb(await page.screenshot()), rect), Boolean);
}

/** 🖱️ Requires live Blueprint paint to appear and clear after pointer moves on an unselected frame. */
export async function verifyFrameHover({ page, owner, frameId, locateHorizontal, setTransform, readSelection, spec }) {
  const handle = await locateHorizontal();
  await setTransform(false);
  const [left, top, width, height] = await sceneBounds(page, owner);
  const pivot = [handle.x - spec.axisLength, handle.y];
  const neutral = [left + spec.neutralInset, top + spec.neutralInset];
  assert.ok(Math.hypot(pivot[0] - neutral[0], pivot[1] - neutral[1]) > spec.cropRadius * 1.5, "neutral pointer position stays outside the measured frame");
  const cropLeft = Math.max(left, pivot[0] - spec.cropRadius);
  const cropTop = Math.max(top, pivot[1] - spec.cropRadius);
  const crop = [cropLeft, cropTop, Math.min(left + width, pivot[0] + spec.cropRadius) - cropLeft, Math.min(top + height, pivot[1] + spec.cropRadius) - cropTop];
  const capture = async () => rgb(await page.screenshot());
  const move = async point => {
    const cursor = (await readChrome(page)).actions.at(-1)?.seq ?? 0;
    const started = performance.now();
    await page.mouse.move(...point);
    const chrome = await until(page, "live Blueprint pointer move", () => readChrome(page), value => scopeActions(value, cursor, "canvasPointerMove").length > 0);
    return { seq: scopeActions(chrome, cursor, "canvasPointerMove").at(-1).seq, started };
  };

  await page.mouse.click(neutral[0], neutral[1] + 2);
  await until(page, "unselected Layout frame", readSelection, selected => selected.length === 0);
  const { seq: beforeHover } = await move(neutral);
  let before = await capture();
  before = await until(page, "settled unselected canvas paint", async () => {
    const next = await capture();
    const change = changedPaintPixels(before, next, crop, spec.pixelThreshold);
    before = next;
    return { next, change };
  }, value => value.change <= spec.maximumRestoredDifference).then(value => value.next);

  const enter = await move(pivot);
  const changed = await until(page, "hover stroke painted", async () => changedPaintPixels(before, await capture(), crop, spec.pixelThreshold), value => value >= spec.minimumChangedPixels);
  const enterPaintMs = performance.now() - enter.started;
  assert.deepEqual(await readSelection(), [], "entering the frame keeps selection empty");
  const leave = await move(neutral);
  const restored = await until(page, "hover stroke cleared", async () => changedPaintPixels(before, await capture(), crop, spec.pixelThreshold), value => value <= spec.maximumRestoredDifference);
  const leavePaintMs = performance.now() - leave.started;
  assert.deepEqual(await readSelection(), [], "leaving the frame keeps selection empty");
  assert.equal(scopeActions(await readChrome(page), beforeHover, "canvasPointerDown").length, 0, "hover transition sends no pointer click");
  return { valid: true, frameId, pivot, neutral, enter: enter.seq, leave: leave.seq, changed, restored, enterPaintMs, leavePaintMs };
}

/** 🕹️ Drives a physical X-gumball drag and checks the committed inspector geometry and history. */
export async function verifyGumball({ page, frameId, locateHorizontal, openGeometry, closeGeometry, spec }) {
  const before = await openGeometry();
  await closeGeometry();
  const handle = await locateHorizontal();
  const chrome = await readChrome(page);
  assert.equal(chrome.armed, true);
  const cursor = chrome.actions.at(-1)?.seq ?? 0;
  await page.mouse.move(handle.x, handle.y);
  await page.mouse.down();
  try {
    await page.mouse.move(handle.x + spec.dragCssPixels, handle.y, { steps: 8 });
    await until(page, "streamed gumball translate", () => readChrome(page), value => scopeActions(value, cursor, "translateSelection").some(row => actionPayload(row).phase === "stream"));
  } finally { await page.mouse.up(); }
  const ledger = await until(page, "committed gumball translate", () => readChrome(page), value => scopeActions(value, cursor, "translateSelection").some(row => actionPayload(row).phase === "commit"));
  const translations = scopeActions(ledger, cursor, "translateSelection");
  const movements = translations.map(row => {
    assert.equal(row.origin, "gesture");
    const payload = actionPayload(row);
    assert.deepEqual(payload.ids, [frameId]);
    assert.ok(Number.isFinite(payload.dx));
    assert.equal(payload.dy, 0);
    return payload;
  });
  assert.equal(movements.at(-1).phase, "commit");
  assert.ok(movements.slice(0, -1).every(row => row.phase === "stream"));
  const dx = movements.reduce((sum, row) => sum + row.dx, 0);
  assert.ok(dx > 0, "X drag moves the selected frame right");
  const expected = position => closeEnough(position.x, before.x + dx, spec.epsilon) && closeEnough(position.y, before.y, spec.epsilon);
  const committed = await until(page, "committed Layout geometry", openGeometry, expected);
  await closeGeometry();
  const modifier = process.platform === "darwin" ? "Meta" : "Control";
  await page.keyboard.press(modifier + "+z");
  const undone = await until(page, "gumball undo restores frame geometry", openGeometry, value => closeEnough(value.x, before.x, spec.epsilon) && closeEnough(value.y, before.y, spec.epsilon));
  await closeGeometry();
  await page.keyboard.press(modifier + "+Shift+z");
  const redone = await until(page, "gumball redo restores commit", openGeometry, expected);
  return { valid: true, frameId, handle, dx, before, committed, undone, redone, movements };
}

/** 🛍️ Drags one real retained Catalogue transfer handle onto the mounted Blueprint. */
export async function dragStoredLayoutLink(page, owner, rowKey) {
  const chrome = await until(page, "live linked-asset transfer handle", () => readChrome(page), value => value.hits.some(hit => hit.controlId === "tree.drag.transfer." + rowKey));
  const handle = chrome.hits.find(hit => hit.controlId === "tree.drag.transfer." + rowKey);
  const [x, y, w, h] = await sceneBounds(page, owner);
  const from = center(handle.rect);
  const to = [x + w * 0.62, y + h * 0.6];
  const cursor = chrome.actions.at(-1)?.seq ?? 0;
  await page.mouse.move(...from);
  await page.mouse.down();
  try {
    await sleep(page, 650);
    await page.mouse.move(...to, { steps: 12 });
    await sleep(page, 200);
  } finally { await page.mouse.up(); }
  const ledger = await until(page, "native Layout canvas drop", () => readChrome(page), value => scopeActions(value, cursor, "canvasDrop").length > 0);
  const [action] = scopeActions(ledger, cursor, "canvasDrop");
  const payload = actionPayload(action);
  const source = typeof payload.dragData === "string" ? JSON.parse(payload.dragData) : payload.dragData;
  assert.equal(source.kind, "image");
  assert.ok(source.sourceDocumentId && source.sourceLinkId);
  return { from, to, source, controller: action.controllerId };
}
